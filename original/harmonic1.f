      A=.5
      B=2
      DO 210 C=2 , 1025
      DO 200 D=2 , C
      E=((2*C)+D-3)/(C+D-2)
      IF( E.EQ.3/2 ) F=F+1
      IF( E.EQ.5/3 ) G=G+1
      IF( E.EQ.7/4 ) H=H+1
      IF( E.EQ.8/5 ) I=I+1
      IF( E.EQ.9/5 ) J=J+1
      IF( E.EQ.11/7 ) K=K+1
      IF( E.EQ.11/6 ) L=L+1
      IF( E.EQ.12/7 ) M=M+1
      IF( E.EQ.13/8 ) N=N+1
      IF( E.EQ.14/9 ) O=O+1
      IF( E.EQ.13/7 ) P=P+1
      IF( B.EQ.C .AND. B.EQ.D ) GOTO 220
      IF( B.EQ.C .AND. B.EQ.D ) A=A*2
      IF( B.EQ.C .AND. B.EQ.D ) B=B+A
  200 CONTINUE
  210 CONTINUE
  220 WRITE(*,*) F,G,H,I,J,K,L,M,N,O,P
  230 END
