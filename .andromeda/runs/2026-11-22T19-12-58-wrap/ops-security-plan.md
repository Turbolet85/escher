@row_add n=1 row=decision/syn-edges-held-for-bindgen area=dependency-security fact=F1 detector=none
name: bindgen and cssparser-macros syn edges held in the lock
rule: The lockfile holds the syn edges of bindgen and cssparser-macros at 3.0.6, against cargo's own re-resolution, until bindgen and prettyplease agree on syn; a url update that moves them breaks the locked build.
why: bindgen 0.73.2 fails to build against syn 2.0.119, the version a url update re-resolves those edges to.
basis: fct-qtdiehht, fct-x33srsoo, fct-3jl3bfeu
because: The chunk's lock keeps both syn edges at 3.0.6 where cargo would move them, a standing hold on the lock that future dependency updates must respect, and no row of the section says it.
@row_add n=2 row=decision/lock-move-proven-by-locked-build area=dependency-security fact=F2 detector=none
name: A lock move proven by the locked build
rule: A lockfile move is proven by the locked workspace build as well as by the audit leg; the audit leg alone proves no build.
why: A lock the audit leg passed did not compile.
basis: fct-nejzrmgf, fct-s6vdi6yn
because: The chunk found that a lock move checked by the audit leg alone did not build, a rule for every later lock move that no standing row of the section states.
