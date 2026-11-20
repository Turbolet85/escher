@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/services/blitz-test-harness.md line=15 class=omission
leaf-text: it opens no span and prints or logs nothing.
source: ctr-wcvnmydf «highlight_contrast(NodeId) -> Option<TextContrast> is a public read-only reader answering only for the highlighted row of an open select list, with the list's highlight text and background through»
replace-with: it opens no span and prints or logs nothing. `Harness::highlight_contrast(&self, NodeId) -> Option<TextContrast>`, a public read-only reader, answers only for the highlighted row of an open select list, with the list's highlight text and background through the one `contrast_ratio` and the option's own font size and weight, and `None` for any other node; like `text_contrast` it changes nothing (no mutation, input, pump or settle).
@leaf_edit n=2 leaf=.claude/docs/services/blitz-test-harness.md line=16 class=stale
leaf-text: reads its `contrast` defect through `Harness::text_contrast`
source: dec-6q3k2ofk «Session lint's contrast reads through the harness's contrast readers, with no second formula.»
replace-with: reads its `contrast` defect through the harness's contrast readers — `Harness::text_contrast`, and `Harness::highlight_contrast` for an open list's highlighted row, read at its painted pair, the engine's highlight colours — with no second formula
