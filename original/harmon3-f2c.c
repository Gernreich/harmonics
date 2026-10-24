/* HARMON3.f -- translated by f2c (version of 22 July 1992  22:54:52).
   You must link the resulting object file with the libraries:
	-lF77 -lI77 -lm -lc   (in that order)
*/

#include "f2c.h"

/* Table of constant values */

static integer c__4 = 4;
static integer c__1 = 1;
static integer c__3 = 3;

/* Main program */ MAIN__()
{
    /* System generated locals */
    real r__1, r__2;

    /* Builtin functions */
    integer s_wsle(), do_lio(), e_wsle();

    /* Local variables */
    static real a, b, c, d, e, f, g, h;
    static integer i, j, k, l, m, n;
    static real o, p;

    /* Fortran I/O blocks */
    static cilist io___3 = { 0, 6, 0, 0, 0 };
    static cilist io___18 = { 0, 6, 0, 0, 0 };


    a = (float).5;
    b = (float)1.5;
    b += a;
    s_wsle(&io___3);
    do_lio(&c__4, &c__1, (char *)&b, (ftnlen)sizeof(real));
    e_wsle();
    r__1 = b;
    for (c = (float)2.; c <= r__1; c += (float)1.) {
	r__2 = c;
	for (d = (float)2.; d <= r__2; d += (float)1.) {
	    e = (c * 2 + d - 3) / (c + d - 2);
	    if (e == (float)1.) {
		f += 1;
	    }
	    if (e == (float)1.) {
		g += 1;
	    }
	    if (e == (float)1.) {
		h += 1;
	    }
	    if (e == (float)1.) {
		++i;
	    }
	    if (e == (float)1.) {
		++j;
	    }
	    if (e == (float)1.) {
		++k;
	    }
	    if (e == (float)1.) {
		++l;
	    }
	    if (e == (float)1.) {
		++m;
	    }
	    if (e == (float)1.) {
		++n;
	    }
	    if (e == (float)1.) {
		o += 1;
	    }
	    if (e == (float)1.) {
		p += 1;
	    }
/* L190: */
	}
/* L200: */
    }
    s_wsle(&io___18);
    do_lio(&c__4, &c__1, (char *)&f, (ftnlen)sizeof(real));
    do_lio(&c__4, &c__1, (char *)&g, (ftnlen)sizeof(real));
    do_lio(&c__4, &c__1, (char *)&h, (ftnlen)sizeof(real));
    do_lio(&c__3, &c__1, (char *)&i, (ftnlen)sizeof(integer));
    do_lio(&c__3, &c__1, (char *)&j, (ftnlen)sizeof(integer));
    do_lio(&c__3, &c__1, (char *)&k, (ftnlen)sizeof(integer));
    do_lio(&c__3, &c__1, (char *)&l, (ftnlen)sizeof(integer));
    do_lio(&c__3, &c__1, (char *)&m, (ftnlen)sizeof(integer));
    do_lio(&c__3, &c__1, (char *)&n, (ftnlen)sizeof(integer));
    do_lio(&c__4, &c__1, (char *)&o, (ftnlen)sizeof(real));
    do_lio(&c__4, &c__1, (char *)&p, (ftnlen)sizeof(real));
    e_wsle();
    f = (float)0.;
    g = (float)0.;
    h = (float)0.;
    i = 0;
    j = 0;
    k = 0;
    l = 0;
    m = 0;
    n = 0;
    o = (float)0.;
    p = (float)0.;
    a *= 2;
/* L210: */
} /* MAIN__ */
