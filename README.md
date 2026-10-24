# Harmonics

**[← All projects](https://gernreich.github.io/)**

In 1992 the author set out to find a consonance rating for musical intervals:
a number, worked out from the ratio p/q alone, that says how smooth the
interval sounds. The idea was to count how often each interval turns up among
pairs of whole numbers, and to call the more frequent ones the more consonant.
These are the programs, written in BASIC on a second-hand IBM PCjr in its
Cartridge BASIC, then translated into Fortran and run through the `f2c`
translator (version of 22 July 1992) into C the same year or soon after, with
some C written by hand. They are the starting point of
[coprime-intervals](https://github.com/Gernreich/coprime-intervals), which
takes up the same question.

**[See the results](https://gernreich.github.io/harmonics/)**: what each
program set out to compute, how far it got, and its scan and transcription.

## Did they find a consonance rating?

Partly. The counting programs (Harmonic1, Harmonic3 and Harmonic4) do produce
a rating, and it gets the top of the order right: fifth, major sixth, then the
harmonic seventh and minor sixth, then the minor seventh. But the count for
p/q works out to ⌊(C−1)/(p−q)⌋, so the rating depends only on p−q. Three
things follow.

- **Ties that should not be.** 9/5 ties with 11/7, which the other rules put
  at least two places lower, and 11/6, 12/7, 13/8 and 14/9 all tie. 13/7 comes
  last, where p²+q², p+q and Tenney put it ahead of 13/8 and 14/9.
- **Half the octave is missing.** Counting only pairs with D ≤ C keeps every
  ratio between 3/2 and 2. The fourth (4/3), the major third (5/4) and the
  minor third (6/5), three of the six classical consonances after the octave,
  never appear.
- **The rating comes from the cap, not just the interval.** Writing n = C−1
  and m = D−1, the ratio is 1 + n/(n+m), which equals p/q exactly when (n, m)
  is a multiple of (p−q, 2q−p). Capping C caps only n, so only p−q counts.
  Capping n + m instead would rank by q alone.

The 11 intervals the programs test are exactly the ratios between 3/2 and 2
with p−q from 1 to 5, plus 13/7. Beside the four rules for consonance that
coprime-intervals compares (from `results/consonance.txt`; 1 = most consonant,
equal scores share a rank):

| Ratio | Interval | Count to C = 32769 | Programs (p−q) | p²+q² | p+q | Tenney p·q | Euler |
|---|---|---|---|---|---|---|---|
| 3/2 | perfect 5th | 32768 | 1 | 1 | 1 | 1 | 1 |
| 5/3 | major 6th | 16384 | 2 | 2 | 2 | 2 | 2 |
| 7/4 | harmonic 7th | 10922 | 3 | 3 | 3 | 3 | 4 |
| 8/5 | minor 6th | 10922 | 3 | 4 | 4 | 4 | 3 |
| 9/5 | minor 7th | 8192 | 4 | 5 | 5 | 5 | 4 |
| 11/7 | | 8192 | 4 | 7 | 7 | 7 | 9 |
| 11/6 | | 6553 | 5 | 6 | 6 | 6 | 7 |
| 12/7 | | 6553 | 5 | 8 | 8 | 8 | 5 |
| 13/8 | | 6553 | 5 | 10 | 10 | 10 | 8 |
| 14/9 | | 6553 | 5 | 11 | 11 | 11 | 6 |
| 13/7 | | 5461 | 6 | 9 | 9 | 9 | 10 |

The rating's rank correlation is 0.90 with p²+q², p+q and Tenney, and 0.85
with Euler's gradus. Harmonic4, rating all 128 ratios its 20 × 20 grid makes,
does worse: its counts ⌊20/(p−q)⌋ take only 8 values, 96 ratios tie at 1, and
its rank correlation with p²+q² is 0.74.

The two programs that draw (Harmonic5 and Pattern) rate nothing, but they
hold the idea that worked later: the coprime pairs are the ratios in lowest
terms, each interval once, and coprime-intervals rates them by height
1/(p²+q²), which very nearly orders the consonances as Helmholtz listed them.

## Each program

| Program | What it does | Toward a consonance rating | As written |
|---|---|---|---|
| Harmonic1 | For every pair 2 ≤ D ≤ C, works out (2C+D−3)/(C+D−2), always between 3/2 and 2, and counts how often it lands on each of 11 intervals. Prints totals each time C reaches 2, 3, 5, 9, … 32769. | The first rating, as above: right at the top, astray below, blind below 3/2. | BASIC should print the counts (not run here), then stop with "RETURN without GOSUB"; reaching 32769 would take days. Fortran and f2c divide 3/2 as whole numbers (= 1), so every count is 0, and stop after one line. The C by hand puts the comparison inside `fabs`, never declares `fabs`, starts its counters at garbage and never stops. |
| Harmonic2 | Walks along sin(x) from a start to an end and lists where it turns over. | No rating: it looks at one sine wave, not two notes together. | Records nothing useful: rising, it never checks for a turn; falling, it writes every point to one slot. Fortran and f2c also write outside the array. |
| Harmonic3 | Harmonic1's count, started again from zero each time the limit doubles. | The same rating. From B = 17 on, the order of the 11 never changes. | BASIC should work, and runs for ever as designed. Fortran and f2c make one pass of zeros. The C by hand has Harmonic1's `fabs` mistakes and its listing stops partway. |
| Harmonic4 | Lists every distinct ratio (2X+Y)/(X+Y) of at least 3/2 and counts how often each occurs. | The widest attempt, 128 ratios rather than 11 chosen ones, but the same p−q rating with only 8 levels; the major seventh 15/8 ties with 20/13. | Does not run: 210 entries overflow a 100-place array, line 90 has no value after `N=`, it tallies only the first two ratios, and `FOR X` closes with `NEXT O`. Here it is tallied over the 20 × 20 grid, so every ratio counts at least 1; the listing used 10 × 10, where 96 never occur. |
| Harmonic5 | Draws one row per number with a dot at every multiple of each of its prime factors, so the dark gaps are the coprime pairs. | No rating, but the map of ratios in lowest terms that coprime-intervals later rates. | Draws nothing: the lines that store a new prime can never be reached. The Fortran does not compile. |
| Pattern | Prints every point (X, Y) up to 199 where X and Y are both multiples of some A from 2 to 199. | No rating: the same map from the other side, the pairs not in lowest terms. | BASIC and the C by hand work. Fortran and f2c stop after A = 2. |

## What is here

| Path | Contents |
|---|---|
| `scans/` | the printed listings, scanned, with the scanner's metadata removed |
| `original/` | exact transcriptions of the scans, mistakes included |
| `harmonics.py` | works out what each program set out to compute, with exact arithmetic, and writes `results/` and `index.html` |
| `results/` | the tables, point lists and pictures it writes, and `consonance.txt` |

The listings are kept as printed and are not run. Where they would not run, or
would give wrong answers, `harmonics.py` computes what they were meant to:

```
python3 harmonics.py           # writes results/ and index.html
python3 harmonics.py --check   # exits 1 if any of them would change
```

It needs only the Python standard library. Its pictures are PNGs with no
metadata. It counts Harmonic1's pairs one by one up to C = 1025 and checks
⌊(C−1)/(p−q)⌋ against every total.

Each program comes in up to four forms. The scan and the transcription share a
name:

| Form | Scan | Transcription |
|---|---|---|
| BASIC original | `harmonic1-bas.pdf` | `harmonic1.bas` |
| Fortran translation | `harmonic1-f.pdf` | `harmonic1.f` |
| C from `f2c` | `harmonic1-f2c-c.pdf` | `harmonic1-f2c.c` |
| C by hand, marked "Fixed" | `harmonic1-fixed-c.pdf` | `harmonic1-fixed.c` |

Harmonic1, Harmonic3 and Pattern come in all four; Harmonic2 in BASIC,
Fortran and f2c; Harmonic5 in BASIC and Fortran; Harmonic4 in BASIC only.

## Transcription notes

The transcriptions copy the listings character for character, including lines
that will not run. Where a listing is odd, this is what the scan shows:

- On paper the programs are titled Harmon1 to Harmon5, and the `f2c` headers
  name the Fortran files `HARMON2.f` and `HARMON3.f`. Here they are called
  Harmonic1 to Harmonic5; the listings themselves are unchanged.
- `harmonic1-f2c.c` starts at the constant table, with no `f2c` banner and no
  `#include "f2c.h"`, and its function is named `MAIN()` although its closing
  comment says `MAIN__`.
- `harmonic3-fixed.c` ends at `o = 0;`: there is no `p = 0;`, no closing braces,
  and no jump back to `L30`. The page may have been the first of two.
- In both hand-written C Harmonic programs the tests read
  `fabs(e-(3.0/2.0)<0.0001)`, with the comparison inside `fabs`.
- `harmonic4.bas` line 90 reads `IF N= THEN 110`.
- `harmonic5.bas` has no line 300.
- `pattern-f2c-c.pdf` is struck through in pink on the page.
