at infinite temperature. In particular, we will employ discrete-time (blocked) cellular automaton circuits

- (CAC) to model the dynamics of these quantities in order to ascertain whether DFL can arise in this inherently classical setup. A further advantage of using CAC is that it allows accessing large system sizes and long evolution times. Cellular automaton circuits.—Previous works have utilized CAC to probe the hydrodynamic behav-

ior of (classical) systems with kinetic constraints and different kinds of unconventional conservation laws such as dipole-moment conservation [56–64]. The method is schematically depicted in Fig. 1a. Analogously to the quantum Hamiltonian in Eq. (1), we distinguish between matter sites (straight purple lines) of index j and gauge links (curly green lines) between sites, lying on a chain of L sites. The (classical) discrete spin sj,j+1 representing the discretized local electric field on the link between sites j and j + 1 takes on values sj,j+1 ∈ {−S, . . . , S}, while the matter occupation on site j is given by σ ∈ {−1, +1}, denoted in Fig. 1a as ◦ and

j

- , respectively. Consequently, the state of the system

at any time-step t is given by a string configuration C(t) = (σ , s , σ , . . . , s , σ , s ), explicitly as-

1 1,2 2 L−1,L L L,1

suming PBC. The dynamics is governed by local gates Pj,j+1 acting on pairs of neighboring sites j and j + 1 and their intermediate link, akin to the 3-local terms in the Hamiltonian (1). Using an analogous formulation to that in Ref. [64], we define the action of a local gate by two integers α and β, such that upon applying Pj,j

\+

= {α, −β}

\+1

or its inverse Pj,j

−

= {−α, β}, the matter and gauge

\+1

degrees of freedom are updated as σ → σ ± α, σ →

j j j+1

σ ± α and sj,j → sj,j ∓ β, respectively. These

j+1 +1 +1

updates are randomly applied among those for which |σ ± α| = 1, |σ ± α| = 1, and |s ± β| ≤ S,

j j+1 j,j+1

with symmetric transition rates such that detailed balance is satisfied for the uniformly random ensemble over string configurations C. As a result, a local gate Pj,j+1 simply implements a random permutation between two allowed strings C → Pj,j+1C. A (discrete) time-step is then given by a sequence of such non-overlapping gates acting on neighboring sites and the links in between as shown in Fig. 1a. The full time evolution follows from the application of several such layers of gates. In the following, we consider the case α = 2β = 2, to appropriately model the tunneling term of Hamiltonian (1). Furthermore, we define the "classical" Gauss's law as G = (−1)j s + s + (σ + 1)/2 at site j and its

j j−1,j j,j+1 j

two neighboring links; cf. Eq. (2). One can then see that evaluating Gj on a string C as given by Gj(C), agrees with G (P C) for all i, j = 1, . . . , L. This is triv-

j i,i+1

ially true for i 6= j − 1, j. In the remainder cases, one finds G (P C) = G (C) ± (β − α/2), which is equal

j i,i+1 j

to Gj(C) for α = 2β. Hence, the previously described CAC leaves the values of Gj invariant for all j and at all

3 S = 1/2 S = 1 1.0 1 0

i

(0)

0.8

σ0

|  | 1.0 |  |
| --- | --- | --- |
| (a) |  | (b) |
| L = 4 |  | L = 4 |
| L = 8 |  | L = 8 |
|  | 0.8 |  |
| L = 200 |  | L = 200 |
| M1(L = 8) |  | M (L = 6) |
|  | 0.6 0.4 |  |

M1 M1

)

2

(t

hσ0

0.6 0 100 101 102 100 101 102 t t (c)

i

0.4

0(∞) σ0(0)

0.2

hσ

L = 4 L = 200 L = 4 (MS) L = 6 L = 400 L = 6 (MS) 0.0 1 2 3 S

FIG. 2. (Color online). **Localized dynamics of the matter autocorrelator.**

Modeling of the dynamics of the matter autocorrelator for (a) S = 1/2 and (b) S = 1 for using cellular automaton circuits. The black horizontal line demarcates the corresponding Mazur bound (4). (c) Infinite-time value of the matter autocorrelator, showing a monotonic decrease with S, and convergence to the thermodynamic limit for system sizes L & 200 sites. Diamonds represent the finite-size Mazur's bounds obtained from Eq. (4).

times, leading to reducible classical dynamics whose sectors correspond to the gauge superselection sectors of the spin-S U(1) QLM (1). This motivates us to investigate whether DFL, well-established for the spin-S U(1) QLM, can also arise in this purely classical setting, in which case we can adjudge whether quantum interference effects are necessary for DFL to arise. Localized dynamics from CAC.—We now focus on the dynamics as calculated through CAC. We prepare an initial string C(0) with a domain-wall structure in the matter degrees of freedom, i.e., σj(0) = ±1 on the left

- (right) half of the chain, and a uniform random configuration of the electric field variables sj,j+1. We then let

it evolve and average the observables of interest A over P N such different preparations hAi ≡ A(C(t))/N.

{C(t)}

The imbalance is then computed as follows

L

X 1

I(t) = hσ (0)σ (t)i. (3)

j j

L

j=1

The corresponding dynamics is shown in Fig. 1b for a system of L = 500 sites and for N = 8000 for various values of S. We see that the imbalance settles into a plateau of finite value at intermediate times that persists for all in-
