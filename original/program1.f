      A = -2
      B = -2
   30 C = A
      D = B
      Z = 0
   40 Z = Z + 1
      E = (C ** 2) - (D ** 2)
      F = 2 * C * D
      G = E + A
      H = F + B
      C = G
      D = H
      IF( C .GT. 2 .OR. D .GT. 2 ) GOTO 90
      IF( Z .LT. 100 ) GOTO 40
      AA = (159 * A) + 318
      BB = (98 * B) + 196
      WRITE(*,*) AA,BB
   90 b=b+.05
      IF( B.GT.2 ) GOTO 120
      goto 30
  120 b=-2
      a=a+.05
      IF( A.GT.2 ) GOTO 100
      goto 30
  100 END
