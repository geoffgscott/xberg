PLOS COMPUTATIONAL BIOLOGY

Cdjoknt sensktkvktkes at stead{-state kn ODE models of bkocjemkcal reactkon networks

**Results**

Kn tjks sectkon we knvestkgate wjetjer uskng tje rrorosed ssCSC metjod ks benefkckal, comrared to tje standard CSC arrroacj, to comrutkng tje objectkve functkon gradkent. Kn botj cases we used CSC, but kn tje fkrst case gradkent comrutatkon was done vka numerkcal kntegratkon of tje ODE s{stem *;+, wjereas kn tje second case tje new arrroacj was arrlked, wjere tje s{stem of Ess *35+ and *37+ ks solved. Kn botj cases numerkcal kntegratkon was used to fknd tje stead{ state of tje s{stem *3+, k.e. to fknd tje solutkon

**z** * **θ** , **u** + of **x**

_ ¼ **fðx** ðθ *;* **uÞ** *;* **θ** *;* **uÞ** ¼ **0** . Tje two cases wkll be referred to as ˜standard CSC for sensktkvktkesˬ and ˜ssCSC for sensktkvktkesˬ. We assess accurac{ b{ comrarkng tje gradkent values comruted wktj tje two metjods, as well as tje dkfference kn comrutatkon tkme. Tje comrarkson ks done on tjree rrevkousl{ rublksjed ODE-constrakned ortkmkzatkon rroblems of var{kng comrlezkt{ tjat resukred ektjer rre- or rost-esuklkbratkon *Table 3+. Tje rroblems were taken from tje REtab Bencjmark Collectkon, a GktJub rerosktor{ wktj rarameter estkmatkon rroblems from rublksjed studkes kmrlemented kn tje REtab format [33, 42]. Tje Blask et al., 2236 model descrkbes acet{latkon of tje jkstone J4 [33]. Kt ks lknear and jas 36 state varkables and ; ortkmkzed rarameters, wjkcj ks tje dkmenskon of objectkve functkon gradkent *:+. Onl{ stead{-state data ks avaklable for tjks model. Tje Blask et al., 2236 model contakns conserved suantktkes, wjkcj were automatkcall{ removed durkng model kmrort b{ CMKCK to ensure non-skngularkt{ of tje Jacobkan. Tje model b{ Zjeng et al., 2232 ks also lknear and descrkbes jkstone J3 metj{latkon rattern formatkon. For tjks model d{namkc data ks avaklable and tje knktkal condktkon ks determkned b{ rre-esuklkbratkon. Tje model b{ Fro˺jlkcj et al., 223: ks a large-scale nonlknear model descrkbkng tje drug resronse of cancer cell lknes [32]. Tje measurement data used for ortkmkzatkon was collected wjen tje s{stem was assumed to be at stead{ state.

**Cccurac{ wktj ssCSC ks rreserved**

To knvestkgate tje accurac{ of objectkve functkon gradkents comruted wktj tje rrorosed ssCSC metjod, we rerformed 522 model skmulatkons wktj dkfferent rarameter vectors samrled unkforml{ wktjkn tje rarameter bounds sreckfked kn tje rarameters REtab fkle [33]. For eacj model, tje same knktkal state, sreckfked kn tje REtab fkles, was used for eacj skmulatkon and no skmulatkon fakled. For all tjree models tje comruted objectkve functkon gradkents were ver{ skmklar for botj comrared arrroacjes *Fkg 3+. Tje mazkmum and medkan devkatkons *as defkned kn tje S3 Tezt, Sectkon ˜Cccurac{ of gradkent comrutatkonˬ+ are resrectkvel{ esual to 3.; 32

−4 and numerkcall{ 2 for tje Blask et al., 2236 model, 2.: 32−33 and 3.6 32−37 for tje Zjeng et al., 2232 model, 3.3 32−2 and 3.24 32−22 for tje Fro˺jlkcj et al., 223: model. Tjerefore, we conclude tjat tje new arrroacj gkves accurate results.

| Table 3. Overvkew of tje models and ortkmkzatkon rroblems conskdered kn tjks stud{. Rroblem | n θ | nz | \# data roknts | Esuklkbratkon t{re | Reference |
| --- | --- | --- | --- | --- | --- |
| Blask et al ., 2236 | ; | 36 | 2:: | rost-esuklkbratkon | [33] |
| Zjeng et al ., 2232 | 46 | 35 | 62 | rre-esuklkbratkon | [34] |
| Fro ˺jlkcj et al ., 223: | 42:: | 33;6 | 343 | rost-esuklkbratkon | [32] |
| jttrs://dok.org/32.3373/journal.rcbk.32327:3.t223 |  |  |  |  |  |

RLOS Comrutatkonal Bkolog{ | jttrs://dok.org/32.3373/journal.rcbk.32327:3 Januar{ 3, 2223 33 / 3;
