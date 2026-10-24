      A=.5
      B=1.5
      B=B+A
      WRITE(*,*) B
      DO 200 C=2 , B
      DO 190 D=2 , C
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
  190 CONTINUE
  200 CONTINUE
      WRITE(*,*) F,G,H,I,J,K,L,M,N,O,P
      F=0
      G=0
      H=0
      I=0
      J=0
      K=0
      L=0
      M=0
      N=0
      O=0
      P=0
      A=A*2
  210 END
