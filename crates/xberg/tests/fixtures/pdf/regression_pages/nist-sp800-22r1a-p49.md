**2.11.3 Test Statistic and Reference Distribution**

∇ψ *2m*

- (obs) and∇ *2* ψ *2m*
- (obs) : A measure of how well the observed frequencies of *m* -bit patterns match the expected frequencies of the *m* -bit patterns.

The reference distribution for the test statistic is the χ *2* distribution.

**2.11.4 Test Description**

- (1) Form an augmented sequence ε′: Extend the sequence by appending the first *m-1* bits to the end of the sequence for distinct values of *n* .

For example, given *n = 10* and ε *= 0011011101* . If *m = 3* , then ε *´* = *001101110100* . If *m = 2* , then ε *´* = *00110111010* . If *m = 1* , then ε *´* = the original sequence *0011011101* .

- (2) Determine the frequency of all possible overlapping *m* -bit blocks, all possible overlapping ( *m-1)* bit blocks and all possible overlapping *(m-2)* -bit blocks. Let *v* denote the frequency of the *m*i *1 ...im*

bit pattern *i1…i ; m* let *v* denote the frequency of the ( *m-1)* -bit pattern *i …i1 m-1;* and let

*i1 ...im* − *1*

*v* denote the frequency of the ( *m-2)* -bit pattern *i1…im-2.*

*i1 ...im* − *2*

For the example in this section, when *m = 3* , then ( *m-1* ) = 2, and ( *m-2* ) = 1. The frequency of all 3-bit blocks is: *v = 0, v = 1, v = 1, v = 2, v = 1, v = 2 v = 2, v = 0.* The

*000 001 010 011 100 101 , 110 111*

frequency of all possible ( *m-1* )-bit blocks is: *v = 1, v = 3, v = 3 v = 3.* The frequency of all

*00 01 10 , 11*

( *m-2* )-bit blocks is: *v0 = 4, v1 = 6.*

*2m* 

- (3) Compute: ψ *2*

= ∑  *vi m*

*1...i*

*n i1 ...im*  *2 n*  *2m* −  = ∑ *2* − *n*

*m*

*vi1 ...im 2m*  *n*

*i1 ...im 2*

*2m* − *1* ψ = ∑  *v m* − *1 n i1 ...im* − *1 2*  *n*  −  = *i1 ...im* − *1*  *2m* − *1*  *1 2m* − ∑ *2* − *n vi1 ...im* − *1 n i1 ...im* − *1 2 2m* − *2* ψ = ∑  *m* − *2 n i1 ...im* − *2*  *n*  *vi1 ...im* − − *2*  *2m* − *2*  *2 2m* − *2*  = ∑ *2* − *n vi ...im*

*1* − *2*

*n i1 ...im* − *2* For the example in this section, *23* ψ *23 = (0* + *1* + *1* + *4* + *1* + *4* + *4* + *1)* − *10* = *12.8* − *10* = *2.8 10*

- (4) Compute: ∇ψ

=ψ −ψ 2

2

ψ *22 =* 10 (1+ 9 + 9 + 9) −10 = 11.2 −10 = 1.2 2 ψ

*21 =* (16 + 36) −10 = 10.4 −10 = 0.4 10 *2 2 2*

, and

*m m m* − *1 2 2 2 2* ∇ *2* ψ *2* + .

=ψ − ψ ψ *m m m* − *1 m* − *2*

| For the example in ∇ ψ 2 = ψ 2 − ψ 2 = Ψ 2 − Ψ 2 = 2.8 − 1.2 = 1.6 | this | section, |  |  |
| --- | --- | --- | --- | --- |
| m | m | m − 1 | 3 | 2 |
