#!/usr/bin/env python3
"""What the 1992 Harmonic and Pattern listings set out to compute,
worked out again with exact integer arithmetic. The listings themselves are
kept unchanged in original/; this program does not run them.

    python3 harmonics.py           # writes results/
    python3 harmonics.py --check   # exits 1 if any file in results/ would change

Standard library only. Pictures are PNGs with no ancillary chunks at all
(no text, no gamma, no colour profile, no pHYs).

  results/harmonic1.txt    interval counts at C = 2, 3, 5, 9, ... 32769
  results/harmonic2.txt    turning points of sin(x) for 0 to 20 in 200 steps
  results/harmonic3.txt    the same counts recounted from scratch each pass
  results/harmonic4.txt    every distinct ratio (2X+Y)/(X+Y) >= 3/2 and its tally
  results/harmonic5.png    one row per number, a dot at each multiple of its primes
  results/pattern.txt    every (X, Y) that share a factor A, as Pattern prints it
  results/pattern.png    those points drawn
  results/consonance.txt   the counts' rating beside four rules for consonance
"""
import math
import struct
import sys
import zlib
from fractions import Fraction
from math import gcd
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"

# The 11 intervals of Harmonic1 and Harmonic3, in the order the listings test
# them and print them (counters F to P).
INTERVALS = [(3, 2), (5, 3), (7, 4), (8, 5), (9, 5), (11, 7), (11, 6),
             (12, 7), (13, 8), (14, 9), (13, 7)]


def cents(r):
    return 1200 * math.log2(r)


def png(width, height, rows, scale=1):
    """Greyscale PNG from rows of 0..255 values, each pixel drawn scale x scale."""
    raw = bytearray()
    for row in rows:
        line = bytes(v for v in row for _ in range(scale))
        for _ in range(scale):
            raw += b"\x00" + line

    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF))

    ihdr = struct.pack(">IIBBBBB", width * scale, height * scale, 8, 0, 0, 0, 0)
    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr)
            + chunk(b"IDAT", zlib.compress(bytes(raw), 9)) + chunk(b"IEND", b""))


def interval_header():
    ratios = "".join(f"{p}/{q}".rjust(7) for p, q in INTERVALS)
    cent = "".join(f"{cents(p / q):7.0f}" for p, q in INTERVALS)
    return [f"{'ratio':>7}{ratios}", f"{'cents':>7}{cent}"]


def count_pairs(c_from, c_to, counts):
    """Add to counts every pair 2 <= D <= C, c_from <= C <= c_to, whose
    (2C+D-3)/(C+D-2) is one of the intervals."""
    for c in range(c_from, c_to + 1):
        for d in range(2, c + 1):
            num, den = 2 * c + d - 3, c + d - 2
            g = gcd(num, den)
            key = (num // g, den // g)
            if key in counts:
                counts[key] += 1


def harmonic1():
    """Harmonic1 prints running totals each time C = D = B, with B running
    2, 3, 5, 9, ... 2^k + 1 up to 32769. Writing n = C-1, m = D-1 the ratio is
    1 + n/(n+m), which equals p/q exactly when (n, m) is a multiple of
    (p-q, 2q-p), so the total up to C is floor((C-1)/(p-q)). The pairs are
    counted one by one up to C = 1025 to confirm that; beyond it the formula
    gives the totals, since counting 5 x 10^8 pairs one by one adds nothing."""
    checkpoints = [2 ** k + 1 for k in range(16)]
    counts = {pq: 0 for pq in INTERVALS}
    out = ["Harmonic1: how many pairs 2 <= D <= C give (2C+D-3)/(C+D-2) = p/q,",
           "totals printed each time C reaches 2, 3, 5, 9, ... 32769.",
           "Counted pair by pair up to C = 1025; above that from the formula",
           "floor((C-1)/(p-q)), which the counting confirms exactly.", ""]
    out += interval_header()
    out.append("      C")
    done = 1
    for c in checkpoints:
        formula = [(c - 1) // (p - q) for p, q in INTERVALS]
        if c <= 1025:
            count_pairs(done + 1, c, counts)
            done = c
            assert [counts[pq] for pq in INTERVALS] == formula, c
        out.append(f"{c:7d}" + "".join(f"{v:7d}" for v in formula))
    return "\n".join(out) + "\n"


def harmonic3():
    """Harmonic3 prints B, recounts every pair from C = 2 up to B, prints the
    totals, zeroes them and doubles the step: B = 2, 3, 5, 9, ... for ever.
    Stopped here at B = 1025."""
    out = ["Harmonic3: B, then the interval counts for every pair 2 <= D <= C <= B,",
           "recounted from scratch each pass. The listing never stops; this",
           "stops at B = 1025.", ""]
    out += interval_header()
    out.append("      B")
    a, b = 0.5, 1.5
    while True:
        b += a
        if b > 1025:
            break
        counts = {pq: 0 for pq in INTERVALS}
        count_pairs(2, int(b), counts)
        out.append(f"{int(b):7d}" + "".join(f"{counts[pq]:7d}" for pq in INTERVALS))
        a *= 2
    return "\n".join(out) + "\n"


def harmonic2(start=0.0, end=20.0, steps=200):
    """Harmonic2 reads a start, an end and a number of steps, walks along
    sin(x) and records each sample where the curve turns over."""
    d = (end - start) / steps
    xs = [start + i * d for i in range(steps + 1)]
    ys = [math.sin(x) for x in xs]
    out = [f"Harmonic2: turning points of sin(x) from {start:g} to {end:g} in {steps} steps",
           f"(step {d:g}). Each is the sample where the curve turns over, with the",
           "true turning point (k + 1/2)pi beside it.", "",
           "        x     sin(x)   kind     true x"]
    for i in range(1, steps):
        rise_in, rise_out = ys[i] - ys[i - 1], ys[i + 1] - ys[i]
        if rise_in * rise_out < 0:
            kind = "peak" if rise_in > 0 else "trough"
            k = round(xs[i] / math.pi - 0.5)
            out.append(f"{xs[i]:9.4f}  {ys[i]:9.5f}   {kind:<6} {(k + 0.5) * math.pi:9.4f}")
    return "\n".join(out) + "\n"


def harmonic4():
    """Harmonic4 lists every distinct M = (2X+Y)/(X+Y) >= 3/2 for X, Y from
    1 to 20, then tallies how often each occurs. The listing tallies over
    X, Y from 1 to 10, where most of the ratios never occur; tallied here
    over the same 1 to 20 grid they came from, every ratio counts at least 1."""
    tally = {}
    for x in range(1, 21):
        for y in range(1, 21):
            m = Fraction(2 * x + y, x + y)
            if m >= Fraction(3, 2):
                tally[m] = tally.get(m, 0) + 1
    assert min(tally.values()) >= 1
    out = ["Harmonic4: every distinct ratio (2X+Y)/(X+Y) of at least 3/2 for X, Y",
           "from 1 to 20, smallest first, with how often it occurs in that grid.",
           f"{len(tally)} ratios.", "",
           "  ratio    cents  count"]
    for m in sorted(tally):
        out.append(f"{str(m):>7} {cents(m):8.1f} {tally[m]:6d}")
    return "\n".join(out) + "\n"


def prime_factors(n):
    ps, p = [], 2
    while p * p <= n:
        if n % p == 0:
            ps.append(p)
            while n % p == 0:
                n //= p
        p += 1
    if n > 1:
        ps.append(n)
    return ps


def harmonic5():
    """Harmonic5 gives screen row Y to the number Y, splits it into primes, and
    for each prime p plots S = 0, p, 2p, ... up to 159. Rows 0 and 1 have no
    prime factors and stay blank. A dot at (S, Y) means S and Y share a
    factor, so the dark gaps are the coprime pairs."""
    size = 160
    rows = []
    for y in range(size):
        ps = prime_factors(y) if y > 1 else []
        row = [255 if any(s % p == 0 for p in ps) else 0 for s in range(size)]
        assert row == [255 if y > 1 and gcd(s, y) > 1 else 0 for s in range(size)]
        rows.append(row)
    return png(size, size, rows, scale=3)


def pattern_points():
    for a in range(2, 200):
        for x in range(a, 200, a):
            for y in range(a, 200, a):
                yield a, x, y


def pattern_txt():
    """Pattern prints X and Y for every point where both are multiples of A,
    for A from 2 to 199. A point with several common factors is printed once
    for each."""
    lines = ["# Pattern: X Y for every X, Y up to 199 that are both multiples of A,",
             "# A = 2 to 199, in the order the listing prints them."]
    lines += [f"{x} {y}" for _, x, y in pattern_points()]
    return "\n".join(lines) + "\n"


def pattern_png():
    size = 200
    lit = [[0] * size for _ in range(size)]
    for _, x, y in pattern_points():
        lit[y][x] = 255
    return png(size, size, lit, scale=2)


NAMES = {(3, 2): "perfect 5th", (5, 3): "major 6th", (7, 4): "harmonic 7th",
         (8, 5): "minor 6th", (9, 5): "minor 7th"}


def gradus(n):
    """Euler's gradus suavitatis of n: 1 plus the sum of (prime - 1) over the
    prime factors of n, with repeats."""
    s, p = 1, 2
    while n > 1:
        while n % p == 0:
            s += p - 1
            n //= p
        p += 1
    return s


def ranks(scores):
    """Dense ranks, smallest score first: equal scores share a rank."""
    order = sorted(set(scores))
    return [order.index(s) + 1 for s in scores]


def spearman(xs, ys):
    """Rank correlation, ties given their average rank."""
    def avg(v):
        return [sum(1 for u in v if u < x) + (sum(1 for u in v if u == x) + 1) / 2 for x in v]
    a, b = avg(xs), avg(ys)
    ma, mb = sum(a) / len(a), sum(b) / len(b)
    cov = sum((x - ma) * (y - mb) for x, y in zip(a, b))
    return cov / math.sqrt(sum((x - ma) ** 2 for x in a) * sum((y - mb) ** 2 for y in b))


def consonance():
    """How well the counts rate consonance. The count up to C is
    floor((C-1)/(p-q)), so the programs rank intervals by p-q alone (smaller
    first). That rank is set beside the four rules coprime-intervals compares:
    p^2+q^2, p+q, Tenney's p*q and Euler's gradus."""
    window = [(p, q) for q in range(1, 40) for p in range(q, 2 * q)
              if gcd(p, q) == 1 and 2 * p >= 3 * q]
    upto5 = [pq for pq in window if pq[0] - pq[1] <= 5]
    assert sorted(INTERVALS) == sorted(upto5 + [(13, 7)])

    rules = [("p-q", lambda p, q: p - q), ("p2+q2", lambda p, q: p * p + q * q),
             ("p+q", lambda p, q: p + q), ("p*q", lambda p, q: p * q),
             ("Euler", lambda p, q: gradus(p * q))]
    order = sorted(INTERVALS, key=lambda pq: (pq[0] - pq[1], pq[0] * pq[0] + pq[1] * pq[1]))
    table = {name: ranks([f(p, q) for p, q in order]) for name, f in rules}
    out = ["The consonance rating the counting programs produce, beside four rules for",
           "how simple a ratio p/q is. 1 = most consonant; equal scores share a rank.",
           "The count is how many pairs up to C = 32769 land on the interval:",
           "floor(32768/(p-q)), so the programs' rank is the rank of p-q.", "",
           "ratio  interval        count   programs  p2+q2  p+q  Tenney p*q  Euler",
           "                                  (p-q)"]
    for i, (p, q) in enumerate(order):
        out.append(f"{p}/{q}".rjust(5) + f"  {NAMES.get((p, q), ''):<14}"
                   f"{32768 // (p - q):6d}" + f"{table['p-q'][i]:9d}" + f"{table['p2+q2'][i]:8d}"
                   + f"{table['p+q'][i]:5d}" + f"{table['p*q'][i]:12d}" + f"{table['Euler'][i]:7d}")
    out += ["", "Rank correlation (Spearman) of the programs' rating with each rule:"]
    for name, label in [("p2+q2", "p^2 + q^2"), ("p+q", "p + q (Pythagorean)"),
                        ("p*q", "p * q (Tenney)"), ("Euler", "Euler's gradus")]:
        out.append(f"  {label:<22}{spearman(table['p-q'], table[name]):6.2f}")

    tally = {}
    for x in range(1, 21):
        for y in range(1, 21):
            m = Fraction(2 * x + y, x + y)
            if m >= Fraction(3, 2):
                tally[m] = tally.get(m, 0) + 1
    ms = list(tally)
    rho = spearman([-tally[m] for m in ms], [m.numerator ** 2 + m.denominator ** 2 for m in ms])
    out += ["", f"Harmonic4 over all {len(ms)} ratios from the 20 x 20 grid: its counts",
            f"floor(20/(p-q)) take only {len(set(tally.values()))} values, "
            f"{sum(1 for v in tally.values() if v == 1)} ratios tie at 1, "
            f"and the rank correlation", f"with p^2 + q^2 is {rho:.2f}."]
    return "\n".join(out) + "\n"


# (name, title, what it does, forms on paper, result shown on the page,
#  how far it got toward a consonance rating, what the listings do as written)
PROGRAMS = [
    ("harmonic1", "Harmonic1",
     "For every pair 2 &le; D &le; C, works out (2C+D&minus;3)/(C+D&minus;2), which always "
     "lies between 3/2 and 2, and counts how often it lands exactly on each of 11 "
     "intervals. Prints the totals each time C reaches 2, 3, 5, 9, &hellip; 32769. "
     "The totals turn out to be &lfloor;(C&minus;1)/(p&minus;q)&rfloor;, so intervals with "
     "the same p&minus;q always tie: 7/4 with 8/5, 9/5 with 11/7, and 11/6, 12/7, 13/8 "
     "and 14/9 together.",
     ["bas", "f", "f2c-c", "fixed-c"], "txt",
     "The first rating: more pairs means more consonant. Its top five, the fifth, "
     "the major sixth, the harmonic seventh and minor sixth (tied), and the minor "
     "seventh, come in the order p&sup2;+q&sup2;, p+q and Tenney&rsquo;s p&middot;q "
     "give. Below that it goes astray, because it sees only p&minus;q.",
     "The BASIC should print the counts (not run here), stopping with "
     "&ldquo;RETURN without GOSUB&rdquo; once the loops end; reaching 32769 would take "
     "days. The Fortran and its f2c translation divide 3/2 as whole numbers, which "
     "gives 1, so every count is 0, and they stop after the first line. The C by hand "
     "puts the comparison inside <code>fabs</code>, never declares <code>fabs</code>, "
     "starts its counters at garbage and never stops."),
    ("harmonic2", "Harmonic2",
     "Reads a start, an end and a number of steps, walks along sin(x), and lists "
     "where the curve turns over. Shown for 0 to 20 in 200 steps.",
     ["bas", "f", "f2c-c"], "txt",
     "No rating. It looks at a single sine wave, not at two notes together.",
     "Records nothing useful in any form: once the curve is rising it never checks "
     "for a turn, and when falling it writes every point to the same slot. The "
     "Fortran and f2c versions also write outside the array (W(0), w[&minus;1])."),
    ("harmonic3", "Harmonic3",
     "Harmonic1&rsquo;s count, started again from zero each time the limit doubles. "
     "The listing never stops; the result stops at 1025.",
     ["bas", "f", "f2c-c", "fixed-c"], "txt",
     "The same rating as Harmonic1. Recounting each pass shows it settles early: "
     "from B = 17 on, the order of the 11 intervals never changes.",
     "The BASIC should work, and runs for ever as designed. The Fortran and f2c "
     "versions make one pass of zeros (3/2 divided as whole numbers again). The C by "
     "hand has Harmonic1&rsquo;s <code>fabs</code> mistakes and its listing stops "
     "partway."),
    ("harmonic4", "Harmonic4",
     "Lists every distinct ratio (2X+Y)/(X+Y) of at least 3/2 for X and Y up to 20, "
     "then counts how often each occurs. The listing counts only up to 10, where "
     "most of the ratios never occur; counted over the same grid up to 20, every "
     "ratio occurs at least once.",
     ["bas"], "txt",
     "The widest attempt: it rates every ratio the grid makes, 128 of them, rather "
     "than 11 chosen ones. But the counts are &lfloor;20/(p&minus;q)&rfloor; again, "
     "with only 8 different values: 96 ratios tie at 1, and the major seventh 15/8 "
     "ties with 20/13.",
     "Does not run: its 210 entries overflow a 100-place array, line 90 reads "
     "<code>IF N= THEN 110</code>, it tallies only the first two ratios, and its "
     "print loop opens with <code>FOR X</code> but closes with <code>NEXT O</code>."),
    ("harmonic5", "Harmonic5",
     "Gives each screen row to a number, splits it into primes, and puts a dot at "
     "every multiple of each prime along the row. A dot means the column and row "
     "share a factor, so the dark gaps are the coprime pairs.",
     ["bas", "f"], "png",
     "No rating, but a different idea: the coprime pairs are the ratios in lowest "
     "terms, each interval exactly once. This is the map coprime-intervals later "
     "rates by height.",
     "Draws nothing: the lines that store a new prime can never be reached. The "
     "Fortran does not compile."),
    ("pattern", "Pattern",
     "Prints every point (X, Y) up to 199 where X and Y are both multiples of some "
     "A from 2 to 199: every pair with a common factor, 24,471 lines in all.",
     ["bas", "f", "f2c-c", "fixed-c"], "png",
     "No rating: the same map as Harmonic5, from the other side, listing the pairs "
     "that are not in lowest terms.",
     "The BASIC and the C by hand work. The Fortran and its f2c translation stop "
     "after A = 2, for want of a jump back."),
]

# form: (label, suffix of its transcription in original/)
FORM_NAMES = {"bas": ("BASIC", ".bas"), "f": ("Fortran", ".f"),
              "f2c-c": ("C from f2c", "-f2c.c"), "fixed-c": ("C by hand", "-fixed.c")}


def page(results):
    from html import escape
    sections = []
    for name, title, about, forms, shown, rating, written in PROGRAMS:
        links = " &middot; ".join(
            f'{FORM_NAMES[f][0]}: <a href="scans/{name}-{f}.pdf">scan</a>, '
            f'<a href="original/{name}{FORM_NAMES[f][1]}">text</a>'
            for f in forms)
        if shown == "png":
            body = f'<img src="results/{name}.png" alt="{title} result">'
            if f"{name}.txt" in results:
                body += f'<p><a href="results/{name}.txt">The full listing of points</a>.</p>'
        else:
            body = f"<pre>{escape(results[name + '.txt'].decode())}</pre>"
        sections.append(f'<section id="{name}">\n<h2>{title}</h2>\n<p>{about}</p>\n'
                        f'<p><strong>Toward a consonance rating.</strong> {rating}</p>\n'
                        f'<p><strong>As written.</strong> {written}</p>\n'
                        f'<p class="forms">{links}</p>\n{body}\n</section>')
    rated = f"<pre>{escape(results['consonance.txt'].decode())}</pre>"
    return (PAGE.replace("{consonance}", rated)
            .replace("{sections}", "\n\n".join(sections)).encode())


PAGE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Harmonics</title>
<style>
:root { --bg: #fbfaf7; --fg: #1d1d1b; --muted: #5d5b55; --rule: #dedbd3;
        --link: #2a5d9f; --code: #f1efe9; }
@media (prefers-color-scheme: dark) {
  :root { --bg: #161615; --fg: #e8e6e1; --muted: #a29f97; --rule: #34332f;
          --link: #8ab4f0; --code: #20201e; }
}
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--fg);
       font: 17px/1.55 Georgia, "Times New Roman", serif; }
main { max-width: 52rem; margin: 0 auto; padding: 2rem 16px 4rem; }
h1 { font-size: 2.2rem; margin: 0 0 .3rem; }
h2 { font-size: 1.4rem; margin: 2.6rem 0 .4rem; padding-top: 1rem;
     border-top: 1px solid var(--rule); }
a { color: var(--link); }
.back { margin: 0 0 1.5rem; font-family: system-ui, sans-serif; font-size: .95rem; }
.forms { color: var(--muted); font-family: system-ui, sans-serif; font-size: .9rem; }
pre { background: var(--code); padding: .9rem 1rem; overflow-x: auto;
      font: 13px/1.4 ui-monospace, Menlo, Consolas, monospace; border-radius: 4px; }
img { display: block; max-width: 100%; height: auto; image-rendering: pixelated;
      border: 1px solid var(--rule); }
</style>
</head>
<body>
<main>
<h1>Harmonics</h1>
<p class="back"><a href="https://gernreich.github.io/">&larr; All projects</a></p>
<p>In 1992 the author set out to find a consonance rating for musical
intervals: a number, worked out from the ratio p/q alone, that says how smooth
the interval sounds. The idea was to count how often each interval turns up
among pairs of whole numbers, and to call the more frequent ones the more
consonant. These are the programs, written in BASIC on a second-hand IBM PCjr
and then translated into Fortran and C.</p>
<p>Several do not run as written, so each result below is what the program set
out to compute, worked out again by <a href="harmonics.py">harmonics.py</a>
with exact arithmetic. The listings are kept exactly as printed: the scans, and
a transcription of each. They are the starting point of
<a href="https://gernreich.github.io/coprime-intervals/">Coprime pairs and musical intervals</a>,
which takes up the same question.</p>

<h2>Did they find a consonance rating?</h2>
<p>Partly. The counting programs (Harmonic1, Harmonic3 and Harmonic4) do
produce a rating, and it gets the top of the order right: fifth, major sixth,
then the harmonic seventh and minor sixth, then the minor seventh. But the
count for p/q works out to &lfloor;(C&minus;1)/(p&minus;q)&rfloor;, so the
rating depends only on p&minus;q. Three things follow.</p>
<ul>
<li><strong>Ties that should not be.</strong> 9/5 ties with 11/7, which the
other rules put at least two places lower, and 11/6, 12/7, 13/8 and 14/9 all tie.
13/7 comes last, where p&sup2;+q&sup2;, p+q and Tenney put it ahead of 13/8
and 14/9.</li>
<li><strong>Half the octave is missing.</strong> Counting only pairs with
D &le; C keeps every ratio between 3/2 and 2. The fourth (4/3), the major third
(5/4) and the minor third (6/5), three of the six classical consonances after
the octave, never appear.</li>
<li><strong>The rating comes from the cap, not just the interval.</strong> A
pair lands on p/q when it is a multiple of (p&minus;q, 2q&minus;p), and capping
C caps only the first number, so only p&minus;q counts. Capping the sum of the
two instead would rank by q alone.</li>
</ul>
<p>The 11 intervals the programs test are exactly the ratios between 3/2 and 2
with p&minus;q from 1 to 5, plus 13/7. Set beside four rules for consonance,
the rating&rsquo;s rank correlation is 0.90 with three of them and 0.85 with
Euler&rsquo;s:</p>
{consonance}
<p>The two programs that draw (Harmonic5 and Pattern) rate nothing, but they
hold the idea that worked later: the coprime pairs are the ratios in lowest
terms, each interval once, and coprime-intervals rates them by height
1/(p&sup2;+q&sup2;), which very nearly orders the consonances as Helmholtz
listed them.</p>

{sections}
</main>
</body>
</html>
"""


def outputs():
    results = {
        "harmonic1.txt": harmonic1().encode(),
        "harmonic2.txt": harmonic2().encode(),
        "harmonic3.txt": harmonic3().encode(),
        "harmonic4.txt": harmonic4().encode(),
        "harmonic5.png": harmonic5(),
        "pattern.txt": pattern_txt().encode(),
        "pattern.png": pattern_png(),
        "consonance.txt": consonance().encode(),
    }
    files = {f"results/{name}": data for name, data in results.items()}
    files["index.html"] = page(results)
    return files


def main():
    check = "--check" in sys.argv[1:]
    changed = []
    RESULTS.mkdir(exist_ok=True)
    for name, data in outputs().items():
        path = HERE / name
        if path.exists() and path.read_bytes() == data:
            continue
        changed.append(name)
        if not check:
            path.write_bytes(data)
    if check:
        for name in changed:
            print(f"{name} is out of date; run python3 harmonics.py")
        sys.exit(1 if changed else 0)
    for name in changed:
        print(f"wrote {name}")


if __name__ == "__main__":
    main()
