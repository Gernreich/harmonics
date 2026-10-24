1 CLS
5 KEY OFF
6 SCREEN 2
10 CLS
20 A = -2
25 B = -2
30 C = A: D = B: Z = 0
40 Z = Z + 1
50 E = (C ^ 2) - (D ^ 2): F = 2 * C * D: G = E + A: H = F + B: C = G: D = H
60 IF C > 2 OR D > 2 THEN GOTO 90
70 IF Z < 100 GOTO 40
75 AA = (159 * A) + 318
76 BB = (98 * B) + 196
80 PSET (AA, BB)
90 b=b+.05
100 if b>2 then goto 120
110 goto 30
120 b=-2
130 a=a+.05
140 if a>2 then end
150 goto 30
