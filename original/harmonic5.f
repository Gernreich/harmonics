      B=0
      DO 270 X=1 , B
      IF( N/P(X).EQ.INT(X/(P(X))) ) GOTO 190
      GOTO 270
      B=B+1
      P(B)=N
      C=C+1
      T(C)=N
      GOTO 280
  190 DO 210 M=1 , C
      IF( T(M).EQ.P(X) ) GOTO 240
  210 CONTINUE
      C=C+1
      T(C)=P(X)
  240 N=N/P(X)
      IF( N.EQ.1 ) GOTO 280
      IF( N/P(X).EQ.INT(N/P(X)) ) GOTO 240
  270 CONTINUE
  280 DO 350 N=1 , C
  290 WRITE(*,*) S,Y
      S=S+T(M)
      IF( S.GT.159 ) GOTO 340
      GOTO 290
  340 S=0
  350 CONTINUE
      Y=Y+1
      IF( Y.GT.159 ) END
      DO 400 M=1 , C
      T(M)=0
  400 CONTINUE
      C=0
      Z=Z+1
      N=Z
  410 END
