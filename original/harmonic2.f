      DIMENSION W(100),X(100)
      READ(*,*) A,B,C
      D=(B-A)/C
      E=SIN(A)
      A=A+D
      F=SIN(A)
   60 IF( F.GT.E ) GOTO 80
      IF( F.LT.E ) GOTO 130
   80 IF( F.GT.E ) GOTO 100
      W(N)=A
      X(N)=SIN(A)
      N=N+1
      GOTO 60
  100 E=F
      A=A+D
      F=SIN(A)
      IF( A.GT.B ) GOTO 180
      GOTO 100
  130 IF( F.LT.E ) GOTO 150
      W(N)=A
      X(N)=SIN(A)
  150 E=F
      A=A+D
      F=SIN(A)
      IF( A.GT.B ) GOTO 180
      GOTO 130
  180 DO 200 R=0 , N
      WRITE(*,*) W(R),X(R)
  200 CONTINUE
      END
