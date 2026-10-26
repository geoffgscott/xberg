And considering (3.22), we have the following estimate s

(ℓ+1)

η2(WH) 1 ¯

a

ui,h¯ − E ui,h¯ ≤ λi,h 1 + 1 +

i,h (ℓ+1) 2 (ℓ) a,h

λ1 δ λδ λ 1 λ The following ∥·∥ -error estimate holds b ! 1 ui,h¯

(ℓ+1)

− Ei,h ui,h¯ ≤

b

From (3.23) and (3.24), the proof is completed.

the following error estimates ui,h¯ − E

(ℓ+1)

ui,h¯ ≤

i,h a,h (ℓ+1)

ui,h¯ − Ei,h ui,h¯ ≤

b

1 +

(ℓ+1) a

λδ1 λ

Corollary 3.1. Under the conditions of Theorem 3.2, the eigenfunction approximation u

ℓ

¯ γ(λi,h) ui,h¯ − E ! 1 1 +

a (ℓ+1)

λδ1 λ where s η

2(WH) 1 ¯ ¯

a

γ(λ ) = λ 1 + 1 +

i,h i,h (ℓ+1) 2

λ1 δ λδ λ 1

(ℓ)

The error estimate for the eigenvalue approximation λi,h

Remark 3.2.

4 Numerical experiments
! η

2(WH) ui,h¯ a (ℓ)

− Ei,hui,h¯ .(3.23)

a,h

(ℓ+1)

η (W ) u¯ − Ei,h ui,h¯ . (3.24)

H i,h a,h

(1) ui,h¯ ,
i,h a,h (ℓ+1)

η (WH) ui,h¯ − Ei,h

! η

2(WH).

(ℓ) a λ

(ℓ+1)

has

i,h

(3.25) ui,h¯ , (3.26)

a,h

(3.27)

can be deduced from Theorem 3.2 and

In this section, numerical experiments are presented to validate our theoretical results. Here, we are concerned with the Laplace eigenvalue problem (2.1), where the computing domain is set to be the unit square Ω = (0, 1) × (0, 1). WH and the CR element spaces defined on the coarse mesh T fine mesh Th is obtained from the coarse mesh T and Vh are chosen as the conforming linear element

H

and the fine mesh Th, respectively. The

H

by the regular refinement of uniform triangular mesh.

Since the coarse space WH is the conforming linear finite element space defined on the coarse mesh TH, together with the theories of the error estimates of CR element and conforming linear element [8, 11], it is known that the following estimate holds when the mesh size h < H, η (W ) ≤ sup ∥T f − T f∥ + inf ∥T f − vH∥

a H h a,h a v ∈W f∈L2(Ω),∥f∥b=1 H H

≤ CH, where the constant C depends on shape of the meshes T Based on Theorems 3.1 and 3.2, the convergence results can be concluded with the following inequalities

(ℓ+1)

ui,h¯ − Fk,h ui,h¯

a,h 2ℓ

≤ C CH ui,h¯

16

and Th.

H

(1)

− Fk,hui,h¯

a,h

, i = 1 ,

, k, (4.1)