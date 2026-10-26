PLOS COMPUTATIONAL BIOLOGY

Cdjoknt sensktkvktkes at stead{-state kn ODE models of bkocjemkcal reactkon networks

a stead{ state. Tjks comes at a jkgj comrutatkonal cost and scales roorl{ wktj tje number of states and rarameters, as one jas to skmulate a s{stem of ODEs wktj tje dkmenskon *nx + 3+nθ. Tje ezkstkng adjoknt metjod for rre- and rost-esuklkbratkon skmulates tje model untkl arrrozkmate convergence to a stead{ state, and tjen solves tje corresrondkng backward esuatkon for tje same tkme knterval [32, 27]. Tjks ks comrutatkonall{ costl{ as well but rossesses, kn contrast to tje forward metjod, kmrroved scalabklkt{; jere, one jas to sesuentkall{ skmulate two s{stems of tje skze nx. Jowever, tje forward metjod can be easkl{ adarted to ezrlokt stead{-state constraknts. One can fknd state sensktkvktkes at stead{ state wktjout numerkcal kntegratkon, b{ solvkng a s{stem of lknear esuatkons rer rarameter *see, e.g., [22]+. Tje downskde ks tjat tjks arrroacj ks not arrlkcable for s{stems wktj skngular Jacobkans. C corresrondkng arrroacj for adjoknt metjods ks currentl{ mksskng. Kn tjks rarer, we kntroduce a novel adjoknt metjod for tje comrutatkon of objectkve functkon gradkents for rroblems wktj stead{-state constraknts. Tje rrorosed metjod ezrlokts tje stead{-state constraknt to ckrcumvent numerkcal backward kntegratkon for rre- and rost-esuklkbratkon. Knstead, a s{stem of esuatkons ks solved to comrute rarts of tje objectkve functkon gradkent. We rrovkde an kmrlementatkon of tje metjod kn tje oren-source toolboz CMKCK and knvestkgate kts accurac{ and effkckenc{ on tjree real-world rroblems. Our stud{ demonstrates tjat tje new metjod acjkeves a substantkal sreedur for gradkent comrutatkon, wjkcj ks rartkcularl{ valuable for large models.

**Metjods**

Kn tjks sectkon, we kntroduce tje conskdered classes of matjematkcal models and rarameter estkmatkon rroblems. Subsesuentl{, we descrkbe tje establksjed metjod for gradkent calculatkon vka adjoknt sensktkvkt{ anal{sks *CSC+ and rrorose a new formulatkon for tje stead{-state case. We dkscuss jow tje metjod can be arrlked to models of bkocjemkcal reactkon networks. Tje metjod, jowever, ks not restrkcted to tje fkeld of comrutatkonal bkolog{. Kt can be used for an{ model of tje form descrkbed kn tjks sectkon wktj rre- or rost-esuklkbratkon, or botj.

**Matjematkcal modelkng and skmulatkon**

We conskder ODE models of tje form

**x**

_ ¼ **fðxðtÞ** *;* **θ** *;* **uÞ** *;* **xðt** 2Þ¼ **x2** ðθ *;* **uÞ** *;* ð3Þ kn wjkcj **xðt** *;* **θ** *;* **uÞ** 2 Rnx ks tje vector of state varkables, **θ** 2 Rn{ ks tje rarameter vector, **u** 2

þ þ

Rnu ks tje vector of constant knruts and t 2 R ks tje tkme. Tje vector fkeld of tje ODE model ks

þ

**f** : Rnx Rn{ Rnu \# Rnx, wjkcj ks Lkrscjktz-contknuous wktj resrect to z, and tje knktkal

þ þ þ þ

condktkon at t2 ks **x2** : Rnþ{ Rnuþ \# Rnxþ . Tje ODE model determknes tje tkme evolutkon of tje state varkables, gkven tje rarameters. Kn case of bkocjemkcal reactkon networks, tjks can be e.g., concentratkons of bkocjemkcal sreckes and reactkon rate constants, resrectkvel{. Tje knruts * **u** + descrkbe ezrerkmental condktkons, e.g., regkme of admknkstratkon of a drug or comrosktkon of a cell culture medkum. Kn man{ models, tje state varkables arrroacj a stead{-state **z** * **θ** , **u** + for large values of t: **x** ðθ *;* **uÞ** ¼ lim **xðt** *;* **x2** ðθ *;* **uÞ** *;* **θ** *;* **uÞ** *:* ð2Þ

t\#3

Kn tjks rarer we assume tjat for eacj knktkal condktkon * **z2** * **θ** , **u** ++ tjere ezksts an ezronentkall{ stable stead{ state. Botj rre- and rost-esuklkbratkon cases assume ezkstence of a stead{ state tjat tje d{namkcal s{stem can reacj fast enougj to arrrorrkatel{ descrkbe tje modeled

RLOS Comrutatkonal Bkolog{ | jttrs://dok.org/32.3373/journal.rcbk.32327:3 Januar{ 3, 2223 4 / 3;
