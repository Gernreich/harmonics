static integer c__4 = 4;
static integer c__1 = 1;
static integer c__3 = 3;

    MAIN()
{
    /* System generated locals */
    real r__1;

    /* Builtin functions */
    integer s_wsle(), do_lio(), e_wsle();

    /* Local variables */
    static real a, b, c, d, e, f, g, h;
    static integer i, j, k, l, m, n;
    static real o, p;

    /* Fortran I/O blocks */
    static cilist io___17 = { 0, 6, 0, 0, 0 };


    a = (float).5;
    b = (float)2.;
    for (c = (float)2.; c <= (float)1025.; c += (float)1.) {
	r__1 = c;
	for (d = (float)2.; d <= r__1; d += (float)1.) {
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
	    if (b == c && b == d) {
		goto L220;
	    }
	    if (b == c && b == d) {
		a *= 2;
	    }
	    if (b == c && b == d) {
		b += a;
	    }
/* L200: */
	}
/* L210: */
    }
L220:
    s_wsle(&io___17);
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
/* L230: */
} /* MAIN__ */
