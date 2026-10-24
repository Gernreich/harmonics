      A=2
      X=A
      Y=A
   40 WRITE(*,*) X,Y
      Y=Y+A
      IF( Y.GT.199 ) GOTO 80
      GOTO 40
   80 X=X+A
      Y=A
      IF( X.GT.199 ) GOTO 120
      GOTO 40
  120 A=A+1
      Y=A
      X=A
      IF( A.GT.199 ) GOTO 160
  160 STOP
      END
