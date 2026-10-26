attractive choice for simulation when applicable. Correspondingly, exciting opportunities open up when new interesting physical models are found which can be simulated using this method [46–48]. Recently, there has been intense experimental development to study the physics of confinement and quantum spin liquids [49–55] using tools of quantum simulation and computation. The microscopic models used to capture the physics involve fermions interacting with

- (Abelian) gauge fields. In this Letter, we develop meron cluster algorithms for a class of experimentally relevant models [56, 57], enabling a robust elucidation of their phase diagrams. We also introduce new classes of Z2 and U(1) multi-flavored gauge-fermion theories, which might be realized in cold-atom setups and also be further studied using Monte Carlo techniques. Notably, the U(1) family of these models seems to be one of the few families that falls outside the class of models known to be simulable by auxiliary-field methods, as are [23, 58, 59]. Moreover, the worldline nature of the method makes it easily employed to study the corresponding phases in these theories in higher spatial dimensions, and the resulting physically-relevant configurations are promising inputs for machine learning algorithms.

Models.– We start with the half-filled t-V model–a spinless fermionic Hamiltonian involving only the most local interactions, X t 1 1 H = − (c

†c + c†c ) + V (n − )(ny − ) (1) y y x x

2 x 2 2

⟨xy⟩

Here ⟨xy⟩ are the nearest neighbor site pairs, c†, c are the creation and annihilation operators respectively, and the repulsive interaction V is given in terms of the number operator n = c

†c. It is simulable by meron clusters for

V ≥ 2t [45, 48]. In this Letter, we extend the meron cluster method to physically interesting Hamiltonians involving gauge fields, which are lower-dimensional versions of quantum electrodynamics (QED) [60, 61]. The Z2- and U(1)-gauge symmetric families are given by:

f

XY

N (g) (g) (g),des

HN = − H + H (2) ⟨xy⟩,f ⟨xy⟩,f

f ⟨xy⟩ f=1

The label g ∈ {U(1), Z2} is the gauge symmetry, with HZ

2

= t c† s1 cy,f + c† sxy,f

1 cx,f ⟨xy⟩,f x,f xy,f y,f

(3)

U(1)

H = t c† s+ cy,f + c† s− cx,f .

⟨xy⟩,f x,f xy,f y,f xy,f

The hopping of spinless fermions between the nearest neighbors ⟨xy⟩ are now governed by the presence of gauge fields, represented by spin-1/2 operators, sxyk , on the bond. Fig. 1 illustrates the model degrees of freedom.

(g),des

Then H is a designer term [62] that makes the mod⟨ xy⟩,f

2

fermion hole spin-1/2 up spin-1/2 down FIG. 1: Example fermion occupations and bond variables for the theories on a square spatial lattice with Nf = 1. els simulable by the meron algorithm, 1 1 t des HZ2, = −2t nx,f − ny,f − + ⟨xy⟩,f 2 2 2 1 1 U(1),des

- H = −t nx,f − ny,f − (4)

⟨xy⟩,f

2 2 t − ts3 (ny,f − nx,f) +

xy,f

4 These terms serve a similar role as the V = 2t term in the meron cluster algorithm applied to the t-V model [45], and similarly an additional particle-hole symmetric

V ≥ 0 term can be added to the models here in a signproblem-free way. In the Z2 gauge theory, the gauge field s

1 = σ1/2 couples to fermions, and the local Z2 symmetry is manifest via the commutation [Qx, HZ2] = 0, P

Q

nx,f

where Q = (−1) f s3 s3 , αˆ are the

x f,αˆ x,x+ˆα,f x−α,x,fˆ

unit vectors in a d-dimensional square lattice. For the U(1) theory, the unitary operator VU , which commutes Q

- (1) Gx

with HU(1) is given by VU = eiθx , with Gx =

- (1) x

h i P P n − s

3

− s3 + ((−1)x − 1)/2 .

f x,f αˆ x,x+ˆα,f x−α,x,fˆ

In the terminology of gauge fields, our microscopic models are quantum link models [63], which realize the continuous gauge invariance using finite-dimensional quantum degrees of freedom. The identification with usual gauge field operators is U = s+ , U † = s− , E = sxy,f

3 . xy,f xy,f xy,f xy,f

We note that a straightforward application of the meron idea necessitates the introduction of an equivalent flavor index for gauge links as fermion flavors. Naively, the total Gauss law can be expressed through a product (Z2) or sum (U(1)) of the Gauss law of individual flavors degrees of freedom, and the resulting theories have

⊗Nf f

Z2 and U(1)⊗N gauge symmetry. However, flavored gauge-interactions can also be turned on in the U(1) model (as explained in the Supplementary Material), X

U(1) U(1)

H →H + J s3 s3 ,

Nf =2 Nf =2 xy,1 xy,2

(5)

⟨xy⟩

or through a Hubbard-U interaction for both Z2 and U(1)-symmetric models [48]. These additions directly
