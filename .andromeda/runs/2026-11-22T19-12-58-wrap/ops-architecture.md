@row_add n=1 row=decision/fork-additions-in-own-functions area=stack-and-technologies fact=F1 detector=none
name: Fork additions in functions of escher's own
rule: A fork addition to an upstream function over the cognitive complexity ceiling goes into a function of escher's own, with one call in upstream's body.
why: ratified, beside the cognitive complexity ceiling
basis: fct-s2gq4ixt, fct-ysuprmlb
because: The chunk moved escher's additions out of three upstream bodies into functions of its own on this rule, which now binds every later fork addition and stands in no row.
@row_add n=2 row=decision/lock-syn-edges-held area=stack-and-technologies fact=F2 detector=D-arch-decisions
name: syn edges of bindgen and cssparser-macros held in the lock
rule: The workspace lock holds the syn edges of bindgen 0.73.2 and cssparser-macros 0.7.1 at syn 3.0.6, a state a url update does not produce, as an exception to a lock that otherwise stays what cargo resolves.
why: bindgen 0.73.2 fails to compile against syn 2.0.119 beside prettyplease 0.3.0
basis: fct-qtdiehht, fct-x33srsoo
because: The chunk's lock keeps both syn edges against cargo's own re-resolution, so a later lock move must keep them and no row says so.
