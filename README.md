# Harmonics

**[← All projects](https://gernreich.github.io/)**

Early programs by the author about whole-number ratios: how often each musical
interval turns up among pairs of numbers, and which pairs share a factor. They
were written in BASIC, translated into Fortran, and turned into C, some with
the `f2c` translator (version of 22 July 1992) and some by hand.

The BASIC programs were written in 1992 on a second-hand IBM PCjr in its
Cartridge BASIC. The Fortran was translated from them and run through `f2c`
the same year or soon after. Several do not run as written; the table says
what each was meant to do. They are the starting point of [coprime-intervals](https://github.com/Gernreich/coprime-intervals).

**[See the results](https://gernreich.github.io/harmonics/)**: each program,
what it set out to compute, and its scan and transcription.

## What is here

| Path | Contents |
|---|---|
| `scans/` | the printed listings, scanned, with the scanner's metadata removed |
| `original/` | exact transcriptions of the scans, mistakes included |
| `harmonics.py` | works out what each program set out to compute, with exact arithmetic, and writes `results/` and `index.html` |
| `results/` | the tables, point lists and pictures it writes |

The listings are kept as printed and are not run. Where they would not run, or
would give wrong answers, `harmonics.py` computes what they were meant to:

```
python3 harmonics.py           # writes results/ and index.html
python3 harmonics.py --check   # exits 1 if any of them would change
```

It needs only the Python standard library. Its pictures are PNGs with no
metadata.

## A result the counts were heading for

Harmonic1 and Harmonic3 count the pairs 2 ≤ D ≤ C whose (2C+D−3)/(C+D−2) equals
an interval p/q. Writing n = C−1 and m = D−1, the ratio is 1 + n/(n+m), which
equals p/q exactly when (n, m) is a multiple of (p−q, 2q−p). So the count up
to C is ⌊(C−1)/(p−q)⌋, and intervals with the same p−q always tie: 7/4 with
8/5, 9/5 with 11/7, and 11/6, 12/7, 13/8 and 14/9 together. `harmonics.py`
counts pair by pair up to C = 1025 and checks the formula against every
total.

Each program comes in up to four forms. The scan and the transcription share a
name:

| Form | Scan | Transcription |
|---|---|---|
| BASIC original | `harmonic1-bas.pdf` | `harmonic1.bas` |
| Fortran translation | `harmonic1-f.pdf` | `harmonic1.f` |
| C from `f2c` | `harmonic1-f2c-c.pdf` | `harmonic1-f2c.c` |
| C by hand, marked "Fixed" | `harmonic1-fixed-c.pdf` | `harmonic1-fixed.c` |

| Program | What it does | Forms |
|---|---|---|
| Harmonic1 | For every pair n ≥ m, works out (2n+m)/(n+m), which always lies between 3/2 and 2, and counts how often it lands on each of 11 intervals: 3/2, 5/3, 7/4, 8/5, 9/5, 11/7, 11/6, 12/7, 13/8, 14/9, 13/7. Prints running totals. | BASIC, Fortran, f2c, hand C |
| Harmonic2 | Walks along sin(x) from a start to an end and lists where it turns over. | BASIC, Fortran, f2c |
| Harmonic3 | Harmonic1's count, started again from zero each time the limit doubles. | BASIC, Fortran, f2c, hand C |
| Harmonic4 | Lists every distinct ratio (2X+Y)/(X+Y) of at least 3/2 and counts how often each occurs. | BASIC |
| Harmonic5 | Draws one row per number with a dot at every multiple of each of its prime factors, so the dark gaps are the coprime pairs. | BASIC, Fortran |
| Pattern | Prints every point (X, Y) where X and Y are both multiples of some A from 2 to 199: every pair with a common factor. | BASIC, Fortran, f2c, hand C |

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
