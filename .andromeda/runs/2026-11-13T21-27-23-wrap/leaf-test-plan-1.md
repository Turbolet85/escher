@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/commands.md lines=83
last: - Fonts missing in headless tests → `system-fonts` feature / 
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/tests
@leaf_edit n=1 leaf=.claude/docs/services/blitz-test-harness.md line=26 class=omission
leaf-text: The `query` helper panics with "invalid selector" on a selector that fails to parse.
source: ctr-523awgmf «contract/harness-inspect/focused · Harness inspection helpers · focused — With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.»
replace-with: The `query` helper panics with "invalid selector" on a selector that fails to parse. With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.
