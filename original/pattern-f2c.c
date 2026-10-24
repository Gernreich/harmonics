/* patt.f -- translated by f2c (version of 22 July 1992  22:54:52).
   You must link the resulting object file with the libraries:
	-lF77 -lI77 -lm -lc   (in that order)
*/

#include "f2c.h"

/* Table of constant values */

static integer c__4 = 4;
static integer c__1 = 1;

/* Main program */ MAIN__()
{
    /* Builtin functions */
    integer s_wsle(), do_lio(), e_wsle();
    /* Subroutine */ int s_stop();

    /* Local variables */
    static real a, x, y;

    /* Fortran I/O blocks */
    static cilist io___4 = { 0, 6, 0, 0, 0 };


    a = (float)2.;
    x = a;
    y = a;
L40:
    s_wsle(&io___4);
    do_lio(&c__4, &c__1, (char *)&x, (ftnlen)sizeof(real));
    do_lio(&c__4, &c__1, (char *)&y, (ftnlen)sizeof(real));
    e_wsle();
    y += a;
    if (y > (float)199.) {
	goto L80;
    }
    goto L40;
L80:
    x += a;
    y = a;
    if (x > (float)199.) {
	goto L120;
    }
    goto L40;
L120:
    a += 1;
    y = a;
    x = a;
    if (a > (float)199.) {
	goto L160;
    }
L160:
    s_stop("", 0L);
} /* MAIN__ */
