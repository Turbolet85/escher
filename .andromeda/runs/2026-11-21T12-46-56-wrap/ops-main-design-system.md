@row_retire n=1 row=decision/boxless-focused-select-keeps-focus seen=1 detector=none
why: A focused select whose container lost its box no longer keeps focus: it gives focus up after the next layout and takes a key only once re-focused, a rule a11y-plan's decision/focus-fixup-gives-up-unshown-focus states.
basis: fct-mfew6peh
because: The post-layout focus fixup makes the row's rule false; its fact's home is a11y-plan, so the row retires with no heir in this master.
