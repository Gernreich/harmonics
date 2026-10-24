    main()
{
    int a, x, y;

    a = 2;
    x = a;
    y = a;
L40:
    printf("%d %d %d\n",x,y,a);
    y += a;
    if (y > 199) {
        goto L80;
    }
    goto L40;
L80:
    x += a;
    y = a;
    if (x > 199) {
        goto L120;
    }
    goto L40;
L120:
    a += 1;
    y = a;
    x = a;
    if (a > 199) {
        goto L160;
    }
    goto L40;
L160:
    printf(" ");
    }
