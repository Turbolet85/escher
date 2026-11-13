@leaf_group group=test-plan-2
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=13 class=omission
leaf-text: inspection (`query`, `layout_rect`, `text_content`, `focused`, `hit`, `dom_string`)
source: ctr-523awgmf «contract/harness-inspect/focused · Harness inspection helpers · focused — With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click»
replace-with: inspection (`query`, `layout_rect`, `text_content`, `focused` — with no node focused answering the root html element, never none, so a check that a click clears focus asserts the root element —, `hit`, `dom_string`)
