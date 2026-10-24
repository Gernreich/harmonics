/* PROGRAM1.f -- translated by f2c (version of 22 July 1992  22:54:52).
   You must link the resulting object file with the libraries:
	-lF77 -lI77 -lm -lc   (in that order)
*/

#include "f2c.h"

/* Table of constant values */

static integer c__4 = 4;
static integer c__1 = 1;

/* Main program */ MAIN__()
{
    /* System generated locals */
    real r__1, r__2;

    /* Builtin functions */
    integer s_wsle(), do_lio(), e_wsle();

    /* Local variables */
    static real a, b, c, d, e, f, g, h, z, aa, bb;

    /* Fortran I/O blocks */
    static cilist io___12 = { 0, 6, 0, 0, 0 };


    a = (float)-2.;
    b = (float)-2.;
L30:
    c = a;
    d = b;
    z = (float)0.;
L40:
    z += 1;
/* Computing 2nd power */
    r__1 = c;
/* Computing 2nd power */
    r__2 = d;
    e = r__1 * r__1 - r__2 * r__2;
    f = c * 2 * d;
    g = e + a;
    h = f + b;
    c = g;
    d = h;
    if (c > (float)2. || d > (float)2.) {
	goto L90;
    }
    if (z < (float)100.) {
	goto L40;
    }
    aa = a * 159 + 318;
    bb = b * 98 + 196;
    s_wsle(&io___12);
    do_lio(&c__4, &c__1, (char *)&aa, (ftnlen)sizeof(real));
    do_lio(&c__4, &c__1, (char *)&bb, (ftnlen)sizeof(real));
    e_wsle();
L90:
    b += (float).05;
    if (b > (float)2.) {
	goto L120;
    }
    goto L30;
L120:
    b = (float)-2.;
    a += (float).05;
    if (a > (float)2.) {
	goto L100;
    }
    goto L30;
L100:
    ;
} /* MAIN__ */
