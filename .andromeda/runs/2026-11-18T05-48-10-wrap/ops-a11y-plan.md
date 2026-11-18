@row_set n=1 row=decision/capability-statement-reach-by-hand cell=why seen=1 detector=none
value: The capability statement's pin reads only the a11y leg's file count among the section's claims; no check reads the rest.
basis: fct-ng2zkuhn, fct-rmsocrly
because: The pin now reads the a11y leg's file count stated in the section, so the why that it reads none of the section's claims is false.
@row_add n=1 row=decision/leg-counts-stated-in-capability-statement area=ci-integration fact=F1 detector=none
name: Leg counts held to the capability statement
rule: The a11y leg's file count and the stand-repeat leg's named-target count are stated in words in the capability statement, and a change of either leg's membership reddens the ci-scripts leg until the statement re-states the count.
guard: test_the_leg_counts_are_stated_in_words
basis: fct-rmsocrly, fct-yn5njf23
because: The ci-scripts leg gained a pin coupling the a11y and stand-repeat legs' membership to the capability statement, a gate no row of the area states.
