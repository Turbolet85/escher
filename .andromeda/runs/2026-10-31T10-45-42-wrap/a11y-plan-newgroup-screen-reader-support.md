# New groups — a11y-plan · screen-reader-support

## Verdicts
- fanout-a11y-plan--cascade-g5.md 2 · contradiction · lines 11–11 · «each with its select's client rect as its `bounds`» · the standing line gives every `select`'s options, `ListBox` for `multiple` included, their select's client rect, the proposal gives a `multiple` select's rows and groups their own
- fanout-a11y-plan--cascade-g5.md 3 · distinct · no group names `evt.values()` or `multiple_select_values`; line 11's `select_value` is the snapshot's reading, not the event's
- fanout-a11y-plan--cascade-g5.md 5 · distinct · no group names `cells-grid` or its `aria_label`; line 38 counts Cells' keys and click reactors only
- fanout-a11y-plan--cascade-g5.md 7 · contradiction · lines 52–52 · «the focused DOM node is reported as tree focus» · the standing line reports the focused DOM node unconditionally, the proposal only where it is among the built nodes, else the window · its own line rewritten
- cascade-compare-a11y-plan.md 4 · distinct · lines 25–26 state `toggled` on a checkable role and the snapshot's native `checked`, but no group states the tree's `toggled` read from `checkbox_input_checked()` on a native control
