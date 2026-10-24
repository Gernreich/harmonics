# Harmonics

**[← All projects](https://gernreich.github.io/)**

Early programs by the author about whole-number ratios: how often each musical
interval turns up among pairs of numbers, which pairs share a factor, and the
Mandelbrot set. They were written in BASIC, translated into Fortran, and
turned into C, some with the `f2c` translator (version of 22 July 1992) and
some by hand. Several do not run as written; the table says what each was
meant to do. They are the starting point of
[coprime-intervals](https://github.com/Gernreich/coprime-intervals).

## What is here

| Folder | Contents |
|---|---|
| `scans/` | the printed listings, scanned, with the scanner's metadata removed |
| `original/` | exact transcriptions of the scans, mistakes included |

Each program comes in up to four forms. The scan and the transcription share a
name:

| Form | Scan | Transcription |
|---|---|---|
| BASIC original | `harmon1-bas.pdf` | `harmon1.bas` |
| Fortran translation | `harmon1-f.pdf` | `harmon1.f` |
| C from `f2c` | `harmon1-f2c-c.pdf` | `harmon1-f2c.c` |
| C by hand, marked "Fixed" | `harmon1-fixed-c.pdf` | `harmon1-fixed.c` |

| Program | What it does | Forms |
|---|---|---|
| Harmon1 | For every pair n ≥ m, works out (2n+m)/(n+m), which always lies between 3/2 and 2, and counts how often it lands on each of 11 intervals: 3/2, 5/3, 7/4, 8/5, 9/5, 11/7, 11/6, 12/7, 13/8, 14/9, 13/7. Prints running totals. | BASIC, Fortran, f2c, hand C |
| Harmon2 | Walks along sin(x) from a start to an end and lists where it turns over. | BASIC, Fortran, f2c |
| Harmon3 | Harmon1's count, started again from zero each time the limit doubles. | BASIC, Fortran, f2c, hand C |
| Harmon4 | Lists every distinct ratio (2X+Y)/(X+Y) of at least 3/2 and counts how often each occurs. | BASIC |
| Harmon5 | Draws one row per number with a dot at every multiple of each of its prime factors, so the dark gaps are the coprime pairs. | BASIC, Fortran |
| Pattern | Prints every point (X, Y) where X and Y are both multiples of some A from 2 to 199: every pair with a common factor. | BASIC, Fortran, f2c, hand C |
| Program1 | The Mandelbrot set on an 81 × 81 grid over −2 to 2. | BASIC, Fortran, f2c, hand C |

## Transcription notes

The transcriptions copy the listings character for character, including lines
that will not run. Where a listing is odd, this is what the scan shows:

- `harmon1-f2c.c` starts at the constant table, with no `f2c` banner and no
  `#include "f2c.h"`, and its function is named `MAIN()` although its closing
  comment says `MAIN__`.
- `harmon3-fixed.c` ends at `o = 0;`: there is no `p = 0;`, no closing braces,
  and no jump back to `L30`. The page may have been the first of two.
- In both hand-written C Harmon programs the tests read
  `fabs(e-(3.0/2.0)<0.0001)`, with the comparison inside `fabs`.
- `harmon4.bas` line 90 reads `IF N= THEN 110`.
- `harmon5.bas` has no line 300.
- `pattern-f2c-c.pdf` is struck through in pink on the page.
- The BASIC and Fortran Program1 listings were scanned together as one
  two-page file and are split here. The BASIC page's handwritten title began as "Harmon 4.Bas" and was
  crossed out to read "Program 1.Bas".
