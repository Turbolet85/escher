@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=74
last: - 2026-11-19: A check written over a path just measured pins
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: but pins the measured value as a literal;
source: dec-a7xrcpzo «decision/expected-value-pinned-as-literal · Expected value pinned as a literal — A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal.»
replace-with: but pins the measured value as a literal (a pixel-row pin that font-embolden moves is pinned as a stated band with its reason in a comment where the test crate cannot see the feature, its wholly-inside assertions kept exact);
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=37 class=omission
leaf-text: `contrast_ratio` is the WCAG 2.x ratio.
source: ctr-wcvnmydf «highlight_contrast(NodeId) -> Option<TextContrast> is a public read-only reader answering only for the highlighted row of an open select list, with the list's highlight text and background through the one»
replace-with: `highlight_contrast(NodeId)` returns `Option<TextContrast>` for the highlighted row of an open select list alone — the list's highlight text and background through the one contrast formula, the option's own font size and weight — `None` for any other node, reading and changing nothing; `contrast_ratio` is the WCAG 2.x ratio.
