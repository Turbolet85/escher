@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: A virtual node reads its id, declared role and name, `enabled` as not declared disabled, `invalid` and `focused`, `clickable` and `highlighted` false, no `checked`, `value`, `selected` or `pressed`, and no children.
source: ctr-ub7uhcef «A virtual node reads its id, declared role and name, enabled as not declared disabled, selected as its tree node reads it, invalid and focused, clickable false, highlighted false, no checked, value»
replace-with: A virtual node reads its id, declared role and name, `enabled` as not declared disabled, `selected` as its tree node reads it, `invalid` and `focused`, `clickable` and `highlighted` false, no `checked`, `value` or `pressed`, and no children.
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=15 class=omission
leaf-text: disabled where declared; a hidden, collapsed or inert canvas
source: ctr-7r44w5kv «A canvas's virtual node takes selected as its declaration states it, none where it states none; a canvas's current node is conveyed by selected alone, the tree's focus staying on the canvas and no active»
replace-with: disabled where declared, selected as declared and none where none is declared — a canvas's current node conveyed by selected alone, the tree's focus staying on the canvas and no active descendant naming a virtual node; a hidden, collapsed or inert canvas
