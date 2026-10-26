- B. **Optimal Control Design for Hyperbolic PDE model The first step to derive the state feedback control policy is to write the PDE in its equivalent state space formulation on a Hilbert space H [23]. For a linear hyperbolic PDE of the form ∂x ∂x = V + Mx + B0 u (15) ∂t ∂z y = C0 x (16)**

**this results in x ˙( t ) = Ax ( t ) + Bu ( t ) (17) y ( t ) = Cx ( t ) (18) where A is the linear operator defined as d. A = V**

- **+ M**
- **I (19) dz on the domain dx D ( A ) = { x ∈ H : ∈ H and x (0) = 0 } (20) dz**

**n**

**with V ∈** Rn **× being a symmetric matrix and M, B0, and C being real continuous space-varying matrix functions.**

**0 In addition, x is assumed to be absolutely continuous. It is proven in [23] that if V \< 0 then A generates an exponentially stable C0-semigroup. The linear-quadratic optimal control problem on an infinite time interval finds a control input uopt such that the functional J(x , u) is minimized. The cost functional is given 0 as Z ∞ J(x , u) = hCx(t), QCx(t)i + hu(t), Ru(t)i dt (21) 0 0 The solution of (21) can be found by finding the non-negative self-adjoint operator P which solves [A∗P + PA + C∗QC − PBR−1B∗P]x = 0 (22)**

1

**When (A, B) is exponentially stabilizable and (Q** 2 **C, A) is exponentially detectable (22) has a unique, non-negative solution P and (21) is minimized by the unique control input uopt given by u (t) = K0 x(t) (23)**

**opt where the optimal feedback is K = −R −1B∗P (24) 0 The proof that (A, B) is exponentially stabilizable and**

1

**(P** 2 **C, A) is exponentially detectable given V \< 0 is shown in [23]. Furthermore, P := Φ(z)I (25) where Φ(z) is the solution of the matrix Riccati differential equation given by dΦ**

**V = M ∗Φ + ΦM + C∗Q C − ΦB R−1B0 ∗Φ 0 0 0 0 0 dz (26) Φ(L) = 0 where Q is a positive matrix and R is a self-adjoint positive 0 0 matrix. Thus, (24) becomes K = −R −1(z)B∗(z)Φ(z)I (27) 0 0 0**

- C. **LQ Control Deisgn for LWR Model with VSL In order to find Φ(z), we take (14) and transform it into ∂∆ρ ρ0 ∂∆ρ**

**= −b 0Umax 1 − 2 − ∂t ρmax ∂z ρ0 ∂∆b ρ U0 max 1 − (28) ρmax ∂z so that it is now in the form of (15). It follows that ρ0 V = −b0 Umax 1 − 2 ρmax M = 0 (29) ρ0 B0 = −ρ U0 max 1 − ρmax C0 = I The condition of V \< 0 is met if ρ \< 1ρ in (29). This 0 max 2 means that ρ must always fall in the free flow regime, which 0 is a valid assumption since the equilibrium profile should never fall in the congested regime to avoid traffic jams. The control input for the LWR model with VSL is the change of the speed limit over the stretch of highway. Using (26) and**

- (29) **the matrix Riccati equation becomes**

**dΦ V = Q − ΦB R0 −1B0∗Φ 0 0 dz (30) Φ(L) = 0 As in [23], R = 1. The values for V and B0 will be 0 inserted into the equation at the end of the derivation for Φ for simplicity. Using separation of variables, (30) becomes Z Z Φ z V dΦ = dz = z + C (31) Q0 − B02Φ 2 0 0 The left hand side of (31) can be solved by a combination of integration by partial fractions and substitution. After applying both methods, (31) becomes V p p √ (ln |B Φ + Q | − ln |B Φ − Q |) = z + C 0 0 0 0**

### 2B0 Q0

- (32) **Then, √ √ B0Φ + Q0 2B0 Q0 ln √ = (z + C) B0Φ − Q V √**

**0 √ B0Φ + Q 2B0 Q0 √ 0 = exp (z + C) B0Φ − Q0 V √ p p**

### 2B0 Q0

**B Φ+ Q = B Φ− Q exp (z+C) 0 0 0 0 V (33) The solution to (33) can be either √ p p**

### 2B0 Q0

**B Φ + Q = (B Φ − Q ) exp (z + C) 0 0 0 0 V (34) or √ p p**

### 2B0 Q0

**B Φ + Q = −(B Φ − Q ) exp (z + C) 0 0 0 0 V (35)**
