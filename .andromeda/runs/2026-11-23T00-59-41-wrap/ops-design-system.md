@row_add n=1 row=decision/ime-composition-underline-engine-painted area=typography fact=F1 detector=none
name: IME composition underline painted by the engine
rule: Text being composed through an IME (preedit text) is painted with an underline in every document that has a preedit, until the text is committed; the underline is an engine decoration, not page CSS.
why: Taken from upstream, ratified as upstream's behaviour.
guard: ime_preedit_underline
basis: fct-yxmdkbm5 · fct-rmpinexc · fct-vtub7frn
because: The merge brings upstream's composition underline (E5), taken on the founder's ruling, and no typography row states that composing text is underlined by the engine.
@row_add n=2 row=decision/float-clear-sides-follow-text-direction area=typography fact=F2 detector=none
name: Float and clear sides follow the text direction
rule: The left and right sides of float and clear are resolved from the text direction, so inline-start and inline-end swap in right-to-left text.
why: Taken from upstream, ratified as upstream's behaviour.
basis: fct-yxmdkbm5 · fct-vtub7frn
because: The merge brings upstream's logical float and clear sides (E2), taken on the founder's ruling, and no row states how float and clear sides relate to the text direction.
