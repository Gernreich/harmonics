/* HARMON2.f -- translated by f2c (version of 22 July 1992  22:54:52).
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
    real r__1;

    /* Builtin functions */
    integer s_rsle(), do_lio(), e_rsle();
    double sin();
    integer s_wsle(), e_wsle();

    /* Local variables */
    static real a, b, c, d, e, f;
    static integer n;
    static real r, w[100], x[100];

    /* Fortran I/O blocks */
    static cilist io___1 = { 0, 5, 0, 0, 0 };
    static cilist io___12 = { 0, 6, 0, 0, 0 };


    s_rsle(&io___1);
    do_lio(&c__4, &c__1, (char *)&a, (ftnlen)sizeof(real));
    do_lio(&c__4, &c__1, (char *)&b, (ftnlen)sizeof(real));
    do_lio(&c__4, &c__1, (char *)&c, (ftnlen)sizeof(real));
    e_rsle();
    d = (b - a) / c;
    e = sin(a);
    a += d;
    f = sin(a);
L60:
    if (f > e) {
	goto L80;
    }
    if (f < e) {
	goto L130;
    }
L80:
    if (f > e) {
	goto L100;
    }
    w[n - 1] = a;
    x[n - 1] = sin(a);
    ++n;
    goto L60;
L100:
    e = f;
    a += d;
    f = sin(a);
    if (a > b) {
	goto L180;
    }
    goto L100;
L130:
    if (f < e) {
	goto L150;
    }
    w[n - 1] = a;
    x[n - 1] = sin(a);
L150:
    e = f;
    a += d;
    f = sin(a);
    if (a > b) {
	goto L180;
    }
    goto L130;
L180:
    r__1 = (real) n;
    for (r = (float)0.; r <= r__1; r += (float)1.) {
	s_wsle(&io___12);
	do_lio(&c__4, &c__1, (char *)&w[(integer) r - 1], (ftnlen)sizeof(real)
		);
	do_lio(&c__4, &c__1, (char *)&x[(integer) r - 1], (ftnlen)sizeof(real)
		);
	e_wsle();
/* L200: */
    }
} /* MAIN__ */
