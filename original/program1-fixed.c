main()
{

    float a, b, c, d, e, f, g, h, z, aa, bb;

    a = -2.0;
    b = -2.0;
L30:
    c = a;
    d = b;
    z = 0.0;
L40:
    z += 1.0;
    e = c * c - d * d;
    f = c * 2 * d;
    g = e + a;
    h = f + b;
    c = g;
    d = h;
    if (c > 2.0 || d > 2.0) {
        goto L90;
    }
    if (z < 100.0) {
        goto L40;
    }
    aa = a * 159.0 + 318.0;
    bb = b * 98.0 + 196.0;
    printf("%0.0f %0.0f\n",aa,bb);
    L90:
    b += 0.05;
    if (b > 2.0) {
        goto L120;
    }
    goto L30;
L120:
    b = -2.0;
    a += 0.05;
    if (a > 2.0) {
        goto L100;
    }
    goto L30;
L100:
    ;
} /* MAIN__ */
