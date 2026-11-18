@row_set n=1 row=decision/capability-statement-pinned cell=rule seen=1 fact=F1 detector=none
value: The capability statement names every driver verb and refusal cause as a code span and states in words the schema's verb, cause, session-error, node-key and exit-status counts and the a11y and stand-repeat legs' counts; the README states the count of driver commands an MCP client gets as tools.
basis: fct-rmsocrly, fct-yn5njf23, fct-ng2zkuhn
because: The pin now also holds the a11y leg's file count and stand-repeat's named-target count against the CI leg script, so a change of either leg's membership reddens ci-scripts until the page re-states it, and the row's pinned set must name both.
@row_set n=2 row=decision/capabilities-doc-pinned-to-schema cell=rule seen=1 detector=none
value: The capability statement's verb and cause names and its schema counts, and the README's count of MCP tools, are held equal to the driver schema's tables, and its a11y leg file count and stand-repeat named-target count to the CI leg script.
basis: fct-rmsocrly, fct-ng2zkuhn
because: Two counts the pin holds are now read from the CI leg script rather than the schema, so a rule naming the schema's tables alone is incomplete.
@row_set n=3 row=decision/capability-check-names-held-to-schema cell=rule seen=1 detector=none
value: Only the claim section's names and counts are pinned, the names and schema counts to the schema's tables and the a11y leg's file count and stand-repeat's named-target count to the CI leg script, so a check renamed or moved changes that section by hand.
basis: fct-rmsocrly, fct-ng2zkuhn
because: The claim map's a11y and stand-repeat counts are now held to the CI leg script, so the row's statement that only the schema's tables hold the section is false.
@row_set n=4 row=decision/capability-statement-from-schema cell=rule seen=1 detector=none
value: The capability statement names every verb and every refusal cause and states the schema's counts in words read from the schema's tables, none hand-copied.
basis: fct-rmsocrly, fct-tesg7ok2
because: The statement now also states leg counts read from the CI leg script and requirement witness counts stated by hand, so the row's claim that all its counts are read from the schema's tables is false.
@row_set n=5 row=decision/requirement-witness-inside-stand-repeat cell=rule seen=1 fact=F3 detector=none
value: Every release requirement names its witness test inside the stand-repeat leg, in a table pinned by each test's function name, so a renamed witness reads red; the capability statement's counts of those witnesses are stated by hand and read by no check.
basis: fct-tesg7ok2, fct-aud5o5p4, fct-og2uwyki
because: The page's WITNESSES_020 count went stale against its own table with no check reading it, and the capability statement now states both witness counts by hand, so the row must say no check holds them.
@row_set n=6 row=decision/mutation-controlled-tests cell=except seen=2 fact=F2 detector=none
value: A control of a Python unittest check is read by hand from its FAIL and OK lines, the control tool reading only nextest, libtest and pytest result lines.
basis: fct-gwfbjor2, fct-6csf5zjb
because: The control of the new ci-scripts test read INDETERMINATE under the control tool and was read by hand, so the controlled-test convention carries this named exception for unittest checks.
@row_none n=1 fact=F4 type=7
what: An absence inside one check's reach, not a whole capability: the capability statement's limits section states that no check asserts caused carries an app's text as written, its scrub pinned by stand_act_error_text.
