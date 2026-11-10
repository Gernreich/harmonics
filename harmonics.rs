//! What the 1992 Harmonic and Pattern listings set out to compute, worked out
//! again with exact integer arithmetic. The listings themselves are kept
//! unchanged in original/; this program does not run them.
//!
//!     target/release/harmonics           # writes results/
//!     target/release/harmonics --check   # exits 1 if any file in results/ would change
//!
//! No dependencies but the system's zlib, the one Python's zlib module uses,
//! so the PNGs come out byte for byte as they did from harmonics.py (its Rust
//! port, 2026-11-09). Pictures are PNGs with no ancillary chunks at all (no
//! text, no gamma, no colour profile, no pHYs).
//!
//!   results/harmonic1.txt    interval counts at C = 2, 3, 5, 9, ... 32769
//!   results/harmonic2.txt    turning points of sin(x) for 0 to 20 in 200 steps
//!   results/harmonic3.txt    the same counts recounted from scratch each pass
//!   results/harmonic4.txt    every distinct ratio (2X+Y)/(X+Y) >= 3/2 and its tally
//!   results/harmonic5.png    one row per number, a dot at each multiple of its primes
//!   results/pattern.txt      every (X, Y) that share a factor A, as Pattern prints it
//!   results/pattern.png      those points drawn
//!   results/consonance.txt   the counts' rating beside four rules for consonance

mod harmonics_page;

use harmonics_page::{PAGE, PROGRAMS};
use std::collections::{BTreeMap, HashMap};
use std::os::raw::c_ulong;
use std::path::Path;

/// The 11 intervals of Harmonic1 and Harmonic3, in the order the listings
/// test them and print them (counters F to P).
const INTERVALS: [(i64, i64); 11] = [(3, 2), (5, 3), (7, 4), (8, 5), (9, 5), (11, 7), (11, 6), (12, 7), (13, 8), (14, 9), (13, 7)];

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn cents(r: f64) -> f64 {
    1200.0 * r.log2()
}

#[link(name = "z")]
extern "C" {
    fn compress2(dest: *mut u8, dest_len: *mut c_ulong, source: *const u8, source_len: c_ulong, level: i32) -> i32;
    fn compressBound(source_len: c_ulong) -> c_ulong;
    fn crc32(crc: c_ulong, buf: *const u8, len: u32) -> c_ulong;
}

/// zlib.compress(data, 9), by the system's zlib, as Python's is.
fn zcompress(data: &[u8]) -> Vec<u8> {
    unsafe {
        let mut n = compressBound(data.len() as _);
        let mut out = vec![0u8; n as usize];
        assert_eq!(compress2(out.as_mut_ptr(), &mut n, data.as_ptr(), data.len() as _, 9), 0, "zlib: compress2 failed");
        out.truncate(n as usize);
        out
    }
}

/// Greyscale PNG from rows of 0..255 values, each pixel drawn scale x scale.
fn png(width: u32, height: u32, rows: &[Vec<u8>], scale: usize) -> Vec<u8> {
    let mut raw = vec![];
    for row in rows {
        let line: Vec<u8> = row.iter().flat_map(|&v| std::iter::repeat(v).take(scale)).collect();
        for _ in 0..scale {
            raw.push(0u8);
            raw.extend_from_slice(&line);
        }
    }
    let chunk = |kind: &[u8], data: &[u8]| -> Vec<u8> {
        let mut c = (data.len() as u32).to_be_bytes().to_vec();
        c.extend_from_slice(kind);
        c.extend_from_slice(data);
        let crc = unsafe { crc32(0, c[4..].as_ptr(), (c.len() - 4) as u32) } as u32;
        c.extend_from_slice(&crc.to_be_bytes());
        c
    };
    let mut ihdr = (width * scale as u32).to_be_bytes().to_vec();
    ihdr.extend_from_slice(&(height * scale as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 0, 0, 0, 0]);
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    out.extend(chunk(b"IHDR", &ihdr));
    out.extend(chunk(b"IDAT", &zcompress(&raw)));
    out.extend(chunk(b"IEND", b""));
    out
}

/// Python's format(x, "g"): six significant figures, trailing zeros dropped.
fn py_g(x: f64) -> String {
    if x == 0.0 {
        return "0".into();
    }
    let e: String = format!("{x:.5e}");
    let (m, exp) = e.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    if (-4..6).contains(&exp) {
        let s = format!("{x:.*}", (5 - exp) as usize);
        if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s }
    } else {
        let m = if m.contains('.') { m.trim_end_matches('0').trim_end_matches('.') } else { m };
        format!("{m}e{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs())
    }
}

fn interval_header() -> Vec<String> {
    let ratios: String = INTERVALS.iter().map(|(p, q)| format!("{:>7}", format!("{p}/{q}"))).collect();
    let cent: String = INTERVALS.iter().map(|&(p, q)| format!("{:7.0}", cents(p as f64 / q as f64))).collect();
    vec![format!("{:>7}{ratios}", "ratio"), format!("{:>7}{cent}", "cents")]
}

/// Add to counts every pair 2 <= D <= C, c_from <= C <= c_to, whose
/// (2C+D-3)/(C+D-2) is one of the intervals.
fn count_pairs(c_from: i64, c_to: i64, counts: &mut HashMap<(i64, i64), i64>) {
    for c in c_from..=c_to {
        for d in 2..=c {
            let (num, den) = (2 * c + d - 3, c + d - 2);
            let g = gcd(num, den);
            if let Some(v) = counts.get_mut(&(num / g, den / g)) {
                *v += 1;
            }
        }
    }
}

fn fresh() -> HashMap<(i64, i64), i64> {
    INTERVALS.iter().map(|&pq| (pq, 0)).collect()
}

/// Harmonic1 prints running totals each time C = D = B, with B running 2, 3,
/// 5, 9, ... 2^k + 1 up to 32769. Writing n = C-1, m = D-1 the ratio is
/// 1 + n/(n+m), which equals p/q exactly when (n, m) is a multiple of
/// (p-q, 2q-p), so the total up to C is floor((C-1)/(p-q)). The pairs are
/// counted one by one up to C = 1025 to confirm that; beyond it the formula
/// gives the totals, since counting 5 x 10^8 pairs one by one adds nothing.
fn harmonic1() -> String {
    let mut counts = fresh();
    let mut out: Vec<String> = [
        "Harmonic1: how many pairs 2 <= D <= C give (2C+D-3)/(C+D-2) = p/q,",
        "totals printed each time C reaches 2, 3, 5, 9, ... 32769.",
        "Counted pair by pair up to C = 1025; above that from the formula",
        "floor((C-1)/(p-q)), which the counting confirms exactly.",
        "",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    out.extend(interval_header());
    out.push("      C".into());
    let mut done = 1;
    for k in 0..16 {
        let c = (1i64 << k) + 1;
        let formula: Vec<i64> = INTERVALS.iter().map(|(p, q)| (c - 1) / (p - q)).collect();
        if c <= 1025 {
            count_pairs(done + 1, c, &mut counts);
            done = c;
            assert_eq!(INTERVALS.iter().map(|pq| counts[pq]).collect::<Vec<_>>(), formula, "{c}");
        }
        out.push(format!("{c:7}") + &formula.iter().map(|v| format!("{v:7}")).collect::<String>());
    }
    out.join("\n") + "\n"
}

/// Harmonic3 prints B, recounts every pair from C = 2 up to B, prints the
/// totals, zeroes them and doubles the step: B = 2, 3, 5, 9, ... for ever.
/// Stopped here at B = 1025.
fn harmonic3() -> String {
    let mut out: Vec<String> = [
        "Harmonic3: B, then the interval counts for every pair 2 <= D <= C <= B,",
        "recounted from scratch each pass. The listing never stops; this",
        "stops at B = 1025.",
        "",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    out.extend(interval_header());
    out.push("      B".into());
    let (mut a, mut b) = (0.5f64, 1.5f64);
    loop {
        b += a;
        if b > 1025.0 {
            break;
        }
        let mut counts = fresh();
        count_pairs(2, b as i64, &mut counts);
        out.push(format!("{:7}", b as i64) + &INTERVALS.iter().map(|pq| format!("{:7}", counts[pq])).collect::<String>());
        a *= 2.0;
    }
    out.join("\n") + "\n"
}

/// Harmonic2 reads a start, an end and a number of steps, walks along sin(x)
/// and records each sample where the curve turns over.
fn harmonic2(start: f64, end: f64, steps: usize) -> String {
    let d = (end - start) / steps as f64;
    let xs: Vec<f64> = (0..=steps).map(|i| start + i as f64 * d).collect();
    let ys: Vec<f64> = xs.iter().map(|x| x.sin()).collect();
    let mut out = vec![
        format!("Harmonic2: turning points of sin(x) from {} to {} in {steps} steps", py_g(start), py_g(end)),
        format!("(step {}). Each is the sample where the curve turns over, with the", py_g(d)),
        "true turning point (k + 1/2)pi beside it.".into(),
        "".into(),
        "        x     sin(x)   kind     true x".into(),
    ];
    let pi = std::f64::consts::PI;
    for i in 1..steps {
        let (rise_in, rise_out) = (ys[i] - ys[i - 1], ys[i + 1] - ys[i]);
        if rise_in * rise_out < 0.0 {
            let kind = if rise_in > 0.0 { "peak" } else { "trough" };
            let k = (xs[i] / pi - 0.5).round_ties_even();
            out.push(format!("{:9.4}  {:9.5}   {kind:<6} {:9.4}", xs[i], ys[i], (k + 0.5) * pi));
        }
    }
    out.join("\n") + "\n"
}

/// A ratio p/q in lowest terms, ordered by value.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Frac(i64, i64);

impl Frac {
    fn new(p: i64, q: i64) -> Frac {
        let g = gcd(p, q);
        Frac(p / g, q / g)
    }
    fn f(&self) -> f64 {
        self.0 as f64 / self.1 as f64
    }
}
impl Ord for Frac {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        (self.0 * o.1).cmp(&(o.0 * self.1))
    }
}
impl PartialOrd for Frac {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl std::fmt::Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if self.1 == 1 { format!("{}", self.0) } else { format!("{}/{}", self.0, self.1) };
        f.pad(&s)
    }
}

/// Every distinct (2X+Y)/(X+Y) >= 3/2 for X, Y from 1 to 20, with its tally,
/// in the order Python's dict first met them.
fn tally_grid() -> Vec<(Frac, i64)> {
    let mut order: Vec<Frac> = vec![];
    let mut tally: HashMap<Frac, i64> = HashMap::new();
    for x in 1..=20 {
        for y in 1..=20 {
            let m = Frac::new(2 * x + y, x + y);
            if m >= Frac(3, 2) {
                let t = tally.entry(m).or_insert(0);
                if *t == 0 {
                    order.push(m);
                }
                *t += 1;
            }
        }
    }
    order.into_iter().map(|m| (m, tally[&m])).collect()
}

/// Harmonic4 lists every distinct M = (2X+Y)/(X+Y) >= 3/2 for X, Y from 1 to
/// 20, then tallies how often each occurs. The listing tallies over X, Y from
/// 1 to 10, where most of the ratios never occur; tallied here over the same
/// 1 to 20 grid they came from, every ratio counts at least 1.
fn harmonic4() -> String {
    let tally: BTreeMap<Frac, i64> = tally_grid().into_iter().collect();
    assert!(tally.values().all(|&v| v >= 1));
    let mut out = vec![
        "Harmonic4: every distinct ratio (2X+Y)/(X+Y) of at least 3/2 for X, Y".to_string(),
        "from 1 to 20, smallest first, with how often it occurs in that grid.".into(),
        format!("{} ratios.", tally.len()),
        "".into(),
        "  ratio    cents  count".into(),
    ];
    for (m, t) in &tally {
        out.push(format!("{m:>7} {:8.1} {t:6}", cents(m.f())));
    }
    out.join("\n") + "\n"
}

fn prime_factors(mut n: i64) -> Vec<i64> {
    let (mut ps, mut p) = (vec![], 2);
    while p * p <= n {
        if n % p == 0 {
            ps.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        ps.push(n);
    }
    ps
}

/// Harmonic5 gives screen row Y to the number Y, splits it into primes, and
/// for each prime p plots S = 0, p, 2p, ... up to 159. Rows 0 and 1 have no
/// prime factors and stay blank. A dot at (S, Y) means S and Y share a
/// factor, so the dark gaps are the coprime pairs.
fn harmonic5() -> Vec<u8> {
    let size = 160i64;
    let mut rows = vec![];
    for y in 0..size {
        let ps = if y > 1 { prime_factors(y) } else { vec![] };
        let row: Vec<u8> = (0..size).map(|s| if ps.iter().any(|p| s % p == 0) { 255 } else { 0 }).collect();
        assert_eq!(row, (0..size).map(|s| if y > 1 && gcd(s, y) > 1 { 255 } else { 0 }).collect::<Vec<u8>>());
        rows.push(row);
    }
    png(size as u32, size as u32, &rows, 3)
}

fn pattern_points() -> Vec<(usize, usize)> {
    let mut v = vec![];
    for a in 2..200 {
        for x in (a..200).step_by(a) {
            for y in (a..200).step_by(a) {
                v.push((x, y));
            }
        }
    }
    v
}

/// Pattern prints X and Y for every point where both are multiples of A, for
/// A from 2 to 199. A point with several common factors is printed once for
/// each.
fn pattern_txt() -> String {
    let mut lines = vec![
        "# Pattern: X Y for every X, Y up to 199 that are both multiples of A,".to_string(),
        "# A = 2 to 199, in the order the listing prints them.".into(),
    ];
    lines.extend(pattern_points().iter().map(|(x, y)| format!("{x} {y}")));
    lines.join("\n") + "\n"
}

fn pattern_png() -> Vec<u8> {
    let size = 200;
    let mut lit = vec![vec![0u8; size]; size];
    for (x, y) in pattern_points() {
        lit[y][x] = 255;
    }
    png(size as u32, size as u32, &lit, 2)
}

fn name_of(pq: (i64, i64)) -> &'static str {
    match pq {
        (3, 2) => "perfect 5th",
        (5, 3) => "major 6th",
        (7, 4) => "harmonic 7th",
        (8, 5) => "minor 6th",
        (9, 5) => "minor 7th",
        _ => "",
    }
}

/// Euler's gradus suavitatis of n: 1 plus the sum of (prime - 1) over the
/// prime factors of n, with repeats.
fn gradus(mut n: i64) -> i64 {
    let (mut s, mut p) = (1, 2);
    while n > 1 {
        while n % p == 0 {
            s += p - 1;
            n /= p;
        }
        p += 1;
    }
    s
}

/// Dense ranks, smallest score first: equal scores share a rank.
fn ranks(scores: &[i64]) -> Vec<i64> {
    let mut order: Vec<i64> = scores.to_vec();
    order.sort();
    order.dedup();
    scores.iter().map(|s| order.iter().position(|o| o == s).unwrap() as i64 + 1).collect()
}

/// Rank correlation, ties given their average rank.
fn spearman(xs: &[i64], ys: &[i64]) -> f64 {
    let avg = |v: &[i64]| -> Vec<f64> {
        v.iter().map(|&x| v.iter().filter(|&&u| u < x).count() as f64 + (v.iter().filter(|&&u| u == x).count() as f64 + 1.0) / 2.0).collect()
    };
    let (a, b) = (avg(xs), avg(ys));
    let sum = |v: &[f64]| v.iter().fold(0.0, |s, x| s + x);
    let (ma, mb) = (sum(&a) / a.len() as f64, sum(&b) / b.len() as f64);
    let cov = a.iter().zip(&b).fold(0.0, |s, (x, y)| s + (x - ma) * (y - mb));
    let va = a.iter().fold(0.0, |s, x| s + (x - ma).powi(2));
    let vb = b.iter().fold(0.0, |s, y| s + (y - mb).powi(2));
    cov / (va * vb).sqrt()
}

/// How well the counts rate consonance. The count up to C is
/// floor((C-1)/(p-q)), so the programs rank intervals by p-q alone (smaller
/// first). That rank is set beside the four rules coprime-intervals compares:
/// p^2+q^2, p+q, Tenney's p*q and Euler's gradus.
fn consonance() -> String {
    let mut window = vec![];
    for q in 1..40i64 {
        for p in q..2 * q {
            if gcd(p, q) == 1 && 2 * p >= 3 * q {
                window.push((p, q));
            }
        }
    }
    let mut upto5: Vec<(i64, i64)> = window.into_iter().filter(|pq| pq.0 - pq.1 <= 5).collect();
    upto5.push((13, 7));
    upto5.sort();
    let mut iv = INTERVALS.to_vec();
    iv.sort();
    assert_eq!(iv, upto5);

    let rules: [(&str, fn(i64, i64) -> i64); 5] = [
        ("p-q", |p, q| p - q),
        ("p2+q2", |p, q| p * p + q * q),
        ("p+q", |p, q| p + q),
        ("p*q", |p, q| p * q),
        ("Euler", |p, q| gradus(p * q)),
    ];
    let mut order = INTERVALS.to_vec();
    order.sort_by_key(|&(p, q)| (p - q, p * p + q * q));
    let table: HashMap<&str, Vec<i64>> = rules.iter().map(|(name, f)| (*name, ranks(&order.iter().map(|&(p, q)| f(p, q)).collect::<Vec<_>>()))).collect();
    let mut out: Vec<String> = [
        "The consonance rating the counting programs produce, beside four rules for",
        "how simple a ratio p/q is. 1 = most consonant; equal scores share a rank.",
        "The count is how many pairs up to C = 32769 land on the interval:",
        "floor(32768/(p-q)), so the programs' rank is the rank of p-q.",
        "",
        "ratio  interval        count   programs  p2+q2  p+q  Tenney p*q  Euler",
        "                                  (p-q)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    for (i, &(p, q)) in order.iter().enumerate() {
        out.push(format!(
            "{:>5}  {:<14}{:6}{:9}{:8}{:5}{:12}{:7}",
            format!("{p}/{q}"),
            name_of((p, q)),
            32768 / (p - q),
            table["p-q"][i],
            table["p2+q2"][i],
            table["p+q"][i],
            table["p*q"][i],
            table["Euler"][i]
        ));
    }
    out.push("".into());
    out.push("Rank correlation (Spearman) of the programs' rating with each rule:".into());
    for (name, label) in [("p2+q2", "p^2 + q^2"), ("p+q", "p + q (Pythagorean)"), ("p*q", "p * q (Tenney)"), ("Euler", "Euler's gradus")] {
        out.push(format!("  {label:<22}{:6.2}", spearman(&table["p-q"], &table[name])));
    }

    let ms = tally_grid();
    let rho = spearman(&ms.iter().map(|(_, t)| -t).collect::<Vec<_>>(), &ms.iter().map(|(m, _)| m.0 * m.0 + m.1 * m.1).collect::<Vec<_>>());
    let mut values: Vec<i64> = ms.iter().map(|(_, t)| *t).collect();
    values.sort();
    values.dedup();
    out.push("".into());
    out.push(format!("Harmonic4 over all {} ratios from the 20 x 20 grid: its counts", ms.len()));
    out.push(format!("floor(20/(p-q)) take only {} values, {} ratios tie at 1, and the rank correlation", values.len(), ms.iter().filter(|(_, t)| *t == 1).count()));
    out.push(format!("with p^2 + q^2 is {rho:.2}."));
    out.join("\n") + "\n"
}

/// form: (label, suffix of its transcription in original/)
fn form_name(f: &str) -> (&'static str, &'static str) {
    match f {
        "bas" => ("BASIC", ".bas"),
        "f" => ("Fortran", ".f"),
        "f2c-c" => ("C from f2c", "-f2c.c"),
        "fixed-c" => ("C by hand", "-fixed.c"),
        _ => unreachable!(),
    }
}

/// Python's html.escape, quotes included.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#x27;")
}

fn page(results: &BTreeMap<String, Vec<u8>>) -> String {
    let text = |name: &str| String::from_utf8(results[name].clone()).unwrap();
    let mut sections = vec![];
    for (name, title, about, forms, shown, rating, written) in PROGRAMS {
        let links: Vec<String> = forms
            .iter()
            .map(|f| format!("{}: <a href=\"scans/{name}-{f}.pdf\">scan</a>, <a href=\"original/{name}{}\">text</a>", form_name(f).0, form_name(f).1))
            .collect();
        let links = links.join(" &middot; ");
        let body = if shown == "png" {
            let mut b = format!("<img src=\"results/{name}.png\" alt=\"{title} result\">");
            if results.contains_key(&format!("{name}.txt")) {
                b += &format!("<p><a href=\"results/{name}.txt\">The full listing of points</a>.</p>");
            }
            b
        } else {
            format!("<pre>{}</pre>", escape(&text(&format!("{name}.txt"))))
        };
        sections.push(format!(
            "<section id=\"{name}\">\n<h2>{title}</h2>\n<p>{about}</p>\n<p><strong>Toward a consonance rating.</strong> {rating}</p>\n<p><strong>As written.</strong> {written}</p>\n<p class=\"forms\">{links}</p>\n{body}\n</section>"
        ));
    }
    let rated = format!("<pre>{}</pre>", escape(&text("consonance.txt")));
    PAGE.replace("{consonance}", &rated).replace("{sections}", &sections.join("\n\n"))
}

/// Every link but an in-page #anchor opens in a new tab. A link written
/// inside a script string (href=\"...\") gets its quotes escaped the same way.
fn new_tab(html: &str) -> String {
    let b = html.as_bytes();
    let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        // <a\b[^>]*>
        if b[i] == b'<' && i + 1 < b.len() && b[i + 1] == b'a' && (i + 2 >= b.len() || !word(b[i + 2])) {
            if let Some(end) = html[i..].find('>') {
                let tag = &html[i..i + end + 1];
                out += &fix(tag);
                i += end + 1;
                continue;
            }
        }
        let ch = html[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

/// One <a ...> tag: href\s*=\s*(\\?)["']?([^"'\\\s>]*), unless it is an
/// anchor, a query, or already has a target.
fn fix(tag: &str) -> String {
    let b = tag.as_bytes();
    let space = |c: u8| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == 0x0b || c == 0x0c;
    let mut found = None;
    for (s, _) in tag.match_indices("href") {
        let mut j = s + 4;
        while j < b.len() && space(b[j]) {
            j += 1;
        }
        if j >= b.len() || b[j] != b'=' {
            continue;
        }
        j += 1;
        while j < b.len() && space(b[j]) {
            j += 1;
        }
        let esc = j < b.len() && b[j] == b'\\';
        if esc {
            j += 1;
        }
        if j < b.len() && (b[j] == b'"' || b[j] == b'\'') {
            j += 1;
        }
        let k = (j..b.len()).find(|&k| matches!(b[k], b'"' | b'\'' | b'\\' | b'>') || space(b[k])).unwrap_or(b.len());
        found = Some((esc, &tag[j..k]));
        break;
    }
    let has_target = tag.match_indices("target").any(|(s, _)| {
        let before = s == 0 || !(b[s - 1].is_ascii_alphanumeric() || b[s - 1] == b'_');
        let mut j = s + 6;
        while j < b.len() && space(b[j]) {
            j += 1;
        }
        before && j < b.len() && b[j] == b'='
    });
    match found {
        Some((esc, h)) if !h.starts_with('#') && !h.starts_with('?') && !has_target => {
            let q = if esc { "\\\"" } else { "\"" };
            format!("{} target={q}_blank{q} rel={q}noopener{q}>", &tag[..tag.len() - 1])
        }
        _ => tag.to_string(),
    }
}

fn outputs() -> Vec<(String, Vec<u8>)> {
    let mut results: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let list: Vec<(&str, Vec<u8>)> = vec![
        ("harmonic1.txt", harmonic1().into_bytes()),
        ("harmonic2.txt", harmonic2(0.0, 20.0, 200).into_bytes()),
        ("harmonic3.txt", harmonic3().into_bytes()),
        ("harmonic4.txt", harmonic4().into_bytes()),
        ("harmonic5.png", harmonic5()),
        ("pattern.txt", pattern_txt().into_bytes()),
        ("pattern.png", pattern_png()),
        ("consonance.txt", consonance().into_bytes()),
    ];
    for (n, d) in &list {
        results.insert(n.to_string(), d.clone());
    }
    let mut files: Vec<(String, Vec<u8>)> = list.into_iter().map(|(n, d)| (format!("results/{n}"), d)).collect();
    files.push(("index.html".into(), new_tab(&page(&results)).into_bytes()));
    files
}

fn main() {
    let check = std::env::args().skip(1).any(|a| a == "--check");
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut changed = vec![];
    std::fs::create_dir_all(here.join("results")).unwrap();
    for (name, data) in outputs() {
        let path = here.join(&name);
        if std::fs::read(&path).is_ok_and(|d| d == data) {
            continue;
        }
        if !check {
            std::fs::write(&path, &data).unwrap();
        }
        changed.push(name);
    }
    if check {
        for name in &changed {
            println!("{name} is out of date; run target/release/harmonics");
        }
        std::process::exit(if changed.is_empty() { 0 } else { 1 });
    }
    for name in &changed {
        println!("wrote {name}");
    }
}
