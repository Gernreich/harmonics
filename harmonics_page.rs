// The page's words: what each program does, and the page around them. Taken
// from harmonics.py when it became harmonics.rs (2026-11-09).

/// (name, title, what it does, forms on paper, result shown on the page,
///  how far it got toward a consonance rating, what the listings do as written)
pub const PROGRAMS: [(&str, &str, &str, &[&str], &str, &str, &str); 6] = [
    (r#"harmonic1"#, r#"Harmonic1"#,
     r#"For every pair 2 &le; D &le; C, works out (2C+D&minus;3)/(C+D&minus;2), which always lies between 3/2 and 2, and counts how often it lands exactly on each of 11 intervals. Prints the totals each time C reaches 2, 3, 5, 9, &hellip; 32769. The totals turn out to be &lfloor;(C&minus;1)/(p&minus;q)&rfloor;, so intervals with the same p&minus;q always tie: 7/4 with 8/5, 9/5 with 11/7, and 11/6, 12/7, 13/8 and 14/9 together."#,
     &[r#"bas"#, r#"f"#, r#"f2c-c"#, r#"fixed-c"#], r#"txt"#,
     r#"The first rating: more pairs means more consonant. Its top five, the fifth, the major sixth, the harmonic seventh and minor sixth (tied), and the minor seventh, come in the order p&sup2;+q&sup2;, p+q and Tenney&rsquo;s p&middot;q give. Below that it goes astray, because it sees only p&minus;q."#,
     r#"The BASIC should print the counts (not run here), stopping with &ldquo;RETURN without GOSUB&rdquo; once the loops end; reaching 32769 would take days. The Fortran and its f2c translation divide 3/2 as whole numbers, which gives 1, so every count is 0, and they stop after the first line. The C by hand puts the comparison inside <code>fabs</code>, never declares <code>fabs</code>, starts its counters at garbage and never stops."#),
    (r#"harmonic2"#, r#"Harmonic2"#,
     r#"Reads a start, an end and a number of steps, walks along sin(x), and lists where the curve turns over. Shown for 0 to 20 in 200 steps."#,
     &[r#"bas"#, r#"f"#, r#"f2c-c"#], r#"txt"#,
     r#"No rating. It looks at a single sine wave, not at two notes together."#,
     r#"Records nothing useful in any form: once the curve is rising it never checks for a turn, and when falling it writes every point to the same slot. The Fortran and f2c versions also write outside the array (W(0), w[&minus;1])."#),
    (r#"harmonic3"#, r#"Harmonic3"#,
     r#"Harmonic1&rsquo;s count, started again from zero each time the limit doubles. The listing never stops; the result stops at 1025."#,
     &[r#"bas"#, r#"f"#, r#"f2c-c"#, r#"fixed-c"#], r#"txt"#,
     r#"The same rating as Harmonic1. Recounting each pass shows it settles early: from B = 17 on, the order of the 11 intervals never changes."#,
     r#"The BASIC should work, and runs for ever as designed. The Fortran and f2c versions make one pass of zeros (3/2 divided as whole numbers again). The C by hand has Harmonic1&rsquo;s <code>fabs</code> mistakes and its listing stops partway."#),
    (r#"harmonic4"#, r#"Harmonic4"#,
     r#"Lists every distinct ratio (2X+Y)/(X+Y) of at least 3/2 for X and Y up to 20, then counts how often each occurs. The listing counts only up to 10, where most of the ratios never occur; counted over the same grid up to 20, every ratio occurs at least once."#,
     &[r#"bas"#], r#"txt"#,
     r#"The widest attempt: it rates every ratio the grid makes, 128 of them, rather than 11 chosen ones. But the counts are &lfloor;20/(p&minus;q)&rfloor; again, with only 8 different values: 96 ratios tie at 1, and the major seventh 15/8 ties with 20/13."#,
     r#"Does not run: its 210 entries overflow a 100-place array, line 90 reads <code>IF N= THEN 110</code>, it tallies only the first two ratios, and its print loop opens with <code>FOR X</code> but closes with <code>NEXT O</code>."#),
    (r#"harmonic5"#, r#"Harmonic5"#,
     r#"Gives each screen row to a number, splits it into primes, and puts a dot at every multiple of each prime along the row. A dot means the column and row share a factor, so the dark gaps are the coprime pairs."#,
     &[r#"bas"#, r#"f"#], r#"png"#,
     r#"No rating, but a different idea: the coprime pairs are the ratios in lowest terms, each interval exactly once. This is the map coprime-intervals later rates by height."#,
     r#"Draws nothing: the lines that store a new prime can never be reached. The Fortran does not compile."#),
    (r#"pattern"#, r#"Pattern"#,
     r#"Prints every point (X, Y) up to 199 where X and Y are both multiples of some A from 2 to 199: every pair with a common factor, 24,471 lines in all."#,
     &[r#"bas"#, r#"f"#, r#"f2c-c"#, r#"fixed-c"#], r#"png"#,
     r#"No rating: the same map as Harmonic5, from the other side, listing the pairs that are not in lowest terms."#,
     r#"The BASIC and the C by hand work. The Fortran and its f2c translation stop after A = 2, for want of a jump back."#),
];

pub const PAGE: &str = r#"<!doctype html>
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
out to compute, worked out again by <a href="harmonics.rs">harmonics.rs</a>
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
<p class="credit" style="margin-top:2em;font-size:0.85em;opacity:0.75">The original programs are Michael Gernreich's; <a href="https://www.anthropic.com/claude">Claude</a> (Anthropic), working in Claude Code, co-wrote the transcriptions, the code and the documentation that present them. Commits Claude made carry a <code>Co-Authored-By: Claude</code> line.</p>
</main>
</body>
</html>
"#;
