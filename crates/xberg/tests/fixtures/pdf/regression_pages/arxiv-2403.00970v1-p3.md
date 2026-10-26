**a continuously differentiable function** ***N*** **(** ***ζ*** **) : [0** ***,*** **∞) → This approach significantly reduces the complexity of (−∞** ***,*** **∞), it is defined via its positive and negative the gain selection process, focusing on only two key truncated forms,** ***N***

**+(** ***ζ*** **) = max{0** ***,N*** **(** ***ζ*** **)} and** ***N*** **−(** ***ζ*** **) = parameters. The time-varying gain** ***κ*** **∆(** ***t*** **) is adaptively max{0** ***,*** **−** ***N*** **(** ***ζ*** **)}, respectively. The function satisfies the updated by condition that** ***κ*** **ˆ ∆(** ***t*** **) = −** ***αψ*** **(** ***t*** **)** ***T φ*** **(** ***x*** **)** ***,*** **(10)** ***N*** **(** ***ζ*** **) =** ***N*** **+(** ***ζ*** **) −** ***N*** **−(** ***ζ*** **)** ***,*** **(4) and the adaptive law for** ***ψ*** **ˆ(** ***t*** **) is given by**

**with the properties that for any** ***ζ*** **, ˆ˙ ˆ** ***ψ*** **(** ***t*** **) = −Γ (** ***αkΨ*** **(** ***t*** **)k2** ***φ*** **(** ***x*** **) +** ***σψ*** **(** ***t*** **))** ***.*** **(11) ïZ Z ò**

***v v***

**1 lim sup** ***N***

**+(** ***ζ*** **)** ***dζ*** **−** ***N*** **−(** ***ζ*** **)** ***dζ*** **= ∞** ***,*** **(5) with** ***α*** **and** ***φ*** **are positive control constants, and Γ = Γ** ***T \> v*** **→∞**

***v***

**0 0**

- 0\. **ïZ Z ò**

***v v***

**1 In this work, we choose** ***N*** **(** ***ζ*** **) =** ***ζ*** **2 cos(** ***ζ*** **) with the lim sup** ***N***

**−(** ***ζ*** **)** ***dζ*** **−** ***N*** **+(** ***ζ*** **)** ***dζ*** **= ∞** ***.*** **(6)** ***v*** **→∞**

***v*** **property of** ***N*** **(0) = 0 as our Nussbaum function. The**

**0 0**

**updating law for** ***ζ*** **is given by A function that satisfies these conditions is utilized for managing the uncertainty in control direction, offering** ***ζ*** **˙(** ***t*** **) = Ψ (** ***t*** **)**

***T*** **(** ***k*** **∆ +** ***κ*** **∆(** ***t*** **))Ψ (** ***t*** **)** ***.*** **(12) a robust approach to control system design. The above control framework, enhanced by adaptive**

*Lemma 2:* **[16], [17] Let** ***V*** **(** ***t*** **) and** ***ζ*** **(** ***t*** **) be smooth laws based on the low complexity Nussbaum function functions defined over the interval [0** ***,t*** **) with** ***V*** **(** ***t*** **)** ***\>*** **0**

***f***

**PID control, provides us with an effective approach for all** ***t*** **∈ [0** ***,t*** **). Given** ***N*** **(** ***ζ*** **(** ***t*** **)) as a Nussbaum-type**

***f***

**to robotic manipulator control, as formalized in the function, if for any** ***t*** **∈ [0** ***,tf*** **), the following condition forthcoming theorem. is met: Z** *Theorem 1:* **Consider the robot manipulator system**

***t t***

**˙ described by (1) with the Properties 1-3, satisfying to**

***V*** **(** ***t*** **)** ***\< c*** **+** ***e*** **−** ***c*** **1**

**(** ***gN*** **(** ***ζ*** **(** ***τ*** **)) + 1)** ***ζ*** **(** ***τ*** **)** ***ec*** **1** ***τdτ,*** **(7)**

**0 0**

**Assumptions 1 and 2. If the Nussbaum function-based where** ***c \>*** **0 and** ***c \>*** **0 are positive constants, and** ***g*** **PID-like control law (9), with the adaptive updating laws**

**0 1**

**is a control parameter within closed intervals** ***L*** **= [** ***l ,l*** **] (11), and (12), and and Lemmas 1, and 2 is implemented,**

**− +**

**excluding zero (0** \< ***L*** **), then** ***V*** **(** ***t*** **),** ***ζ*** **(** ***t*** **), and the integral then, with the design parameters are properly chosen, R** ***t*** **˙ the closed-loop system remains stable, and all signals** ***gN*** **(** ***ζ*** **(** ***τ*** **))** ***ζ*** **(** ***τ*** **)** ***ec***

**1** ***τdτ*** **are guaranteed to be bounded on 0**

**[0** ***,tf*** **). within the system are uniformly bounded. Furthermore, the joint position signal** ***q*** **(** ***t*** **) closely tracks the desired**

- III. **Nussbaum Function-based Control Design and**

**trajectory** ***q*** **(** ***t*** **), with the tracking error converging to a**

***d***

**Stability Analysis small neighborhood around zero. We propose a Nussbaum function based PID-like con-** Proof: **Consider the Lyapunov candidate as trol input for robot manipulator control, delineated as: 1 1 Z ˜ ˜**

***t***

***V*** **(** ***t*** **) = Ψ (** ***t*** **)** ***T M*** **(** ***q*** **(** ***t*** **))Ψ (** ***t*** **) +** ***ψ*** **(** ***t*** **)** ***T*** **Γ −1** ***ψ*** **(** ***t*** **)** ***,*** **(13)**

**2 2** ***u*** **(** ***t*** **) =(** ***kπ*** **+** ***κπ*** **(** ***t*** **))** ***KN*** **(** ***ζ*** **)** ***e*** **(** ***t*** **) + (** ***kι*** **+** ***κι*** **(** ***t*** **))** ***KN*** **(** ***ζ*** **)** ***e*** **(** ***ρ*** **)** ***dρ***

**0**

**where** ***ψ*** **˜(** ***t*** **) =** ***ψ*** **ˆ(** ***t*** **) −** ***ψ***

**∗ with** ***ψ*** **∗(** ***t*** **) is the ideal constant** ***d*** **weight vector of neural network approximation. + (** ***k*** **+** ***κ*** **(** ***t*** **))** ***K*** **(** ***ζ*** **)** ***e*** **(** ***t*** **)** ***,*** **∆ ∆** ***N***

***dt*** **The time derivative of the Lyapunov function** ***V*** **(** ***t*** **) can (8) be bounded as with** ***K*** **(** ***ζ*** **) = −** ***N*** **(** ***ζ*** **) is the related Nussbaum function**

***N***

**1 ˙ gain. In (8) we employed constant gains** ***k*** **,** ***k*** **, and** ***k*** **˙ ˙ ˙ ˜ ˆ**

***π ι*** **∆** ***V*** **(** ***t*** **) = Ψ (** ***t*** **)** ***T M*** **(** ***t*** **)Ψ (** ***t*** **) + Ψ (** ***t*** **)** ***T M*** **(** ***t*** **)Ψ (** ***t*** **) +** ***ψ*** **(** ***t*** **)** ***T*** **Γ −1** ***ψ*** **(** ***t*** **)** ***.*** **2 alongside their time-varying analogs** ***κ*** **(** ***t*** **),** ***κ*** **(** ***t*** **), and** ***π ι***

- (14) ***κ*** **(** ***t*** **). This strategy enhances traditional PID controls**

**∆**

**Considering dynamics (1) and the definition by integrating adaptive gains that adjust in real-time of the generalized error Ψ (** ***t*** **), one can obtain to the system's state. To further simplify parameter** ***M*** **(** ***t*** **)Ψ˙ (** ***t*** **) =** ***C*** **(** ***t*** **)** ***q*** **˙(** ***t*** **) +** ***G*** **(** ***t*** **) −** ***τ*** **(** ***t*** **) +** ***M*** **(** ***t*** **)** ***e*** **¯(·), where tuning, we establish the relationships** ***k*** **=**

***kπ*** **=** ***kι*** **and ∆**

***e*** **¯(·) =** ***q*** **¨ (** ***t*** **)+2*****γe*** **˙(** ***t*** **)+** ***γ*** **2** ***e*** **(** ***t*** **). Then, utilizing the application**

**2*****γ γ*** **2** ***d κ*** **(** ***t*** **)** ***κ*** **(** ***t*** **)**

***κ*** **(** ***t*** **) = of Young's inequality, and considering Property 2,**

***π*** **=** ***ι*** **, ensuring the quadratic expression ∆ 2*****γ γ*** **2**

**we obtain Ψ (** ***t*** **)** ***T C*** **(** ***t*** **)** ***q*** **˙(** ***t*** **) ≤** ***αkΨ*** **(** ***t*** **)k2** ***c2*** **kq˙(** ***t*** **)k4 + 1 ,**

**4*****α***

***S2*** **+ 2*****γS*** **+** ***γ*** **2 is Hurwitz polynomial, where** ***S*** **is the**

**1**

**Ψ (** ***t*** **)** ***T G*** **(** ***t*** **) ≤** ***αkΨ*** **(** ***t*** **)k2** ***g2*** **+ , Ψ (** ***t*** **)** ***T M*** **(** ***t*** **)** ***e*** **¯(·) ≤ Laplace operator. Thus, the control design primarily**

**4*****α*** **2 1 1**

***αkΨ*** **(** ***t*** **)k2** ***m+*** **ke¯(·)k2 + , and Ψ (** ***t*** **) ˙**

**depends on selecting** ***k*** **and** ***κ*** **(** ***t*** **). The PID-like control** ***T M*** **(** ***t*** **)Ψ (** ***t*** **) = ∆ ∆ 4*****α*** **2**

**input is then refined to: Ψ (** ***t*** **)** ***T C*** **(** ***t*** **)Ψ (** ***t*** **) ≤** ***αkΨ*** **(** ***t*** **)k2** ***c2*** **kq˙(** ***t*** **)k2kΨ (** ***t*** **)k2 + 1 , where**

**4*****α***

**Ç Z å** ***α \>*** **0 is a design parameter. Accordingly, taking into**

***t***

***d u*** **(** ***t*** **) =(** ***k*** **account equation (14) and applying the aforementioned ∆ +** ***κ*** **∆(** ***t*** **))** ***KN*** **(** ***ζ*** **) 2*****γe*** **(** ***t*** **) +** ***γ***

**2**

***e*** **(** ***ρ*** **)** ***dρ*** **+** ***e*** **(** ***t*** **)** ***dt***

**0**

**inequalities, one has, = − (** ***k*** **+** ***κ*** **(** ***t*** **))** ***N*** **(** ***ζ*** **)Ψ** ***.***

**∆ ∆**

**1 ˙** ***V*** **˙ (** ***t*** **) ≤** ***αkΨ*** **(** ***t*** **)k2Λ(** ***t*** **)−Ψ (** ***t*** **)**

***T κu*** **(** ***t*** **)+ +Γ −1** ***ψ*** **˜(** ***t*** **) (9)** ***T ψ*** **ˆ(** ***t*** **)** ***,*** **(15)** ***α***
