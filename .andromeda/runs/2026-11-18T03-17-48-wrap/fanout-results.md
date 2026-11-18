architecture: add 1 · set 2 · retire 0 · none 0 · ops-architecture.md
security-plan: add 0 · set 0 · retire 0 · none 2 · ops-security-plan.md
design-system: add 1 · set 0 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 0 · set 0 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 0 · set 1 · retire 0 · none 1 · ops-test-plan.md
obs-plan: no proposer
a11y-plan: add 0 · set 3 · retire 0 · none 0 · ops-a11y-plan.md

ops-architecture.md
contract/ci-leg-legs/a11y · reject — check 7: not_master type 5 — the count is the leg script's inventory; main's shorter set in ops-main-architecture.md
decision/removed-node-state-cleared · reject — check 7: not_master type 4 — the clause retells one mechanism's target; the standing rule is not false
decision/hidden-focused-control-keeps-focus · reject — check 2: the fact's home is a11y-plan decision/focus-fixup-reads-inert-alone

ops-design-system.md
decision/unshown-focused-control-keeps-focus · reject — check 2: the fact's home is a11y-plan decision/focus-fixup-reads-inert-alone

ops-test-plan.md
decision/a11y-leg-membership · reject — check 7: not_master type 5 — the file list is the leg script's inventory; main's shorter set in ops-main-test-plan.md

ops-a11y-plan.md
decision/removed-focused-node-drops-focus · reject — check 7: not_master type 5 — the Cells clause is a stand inventory; «to the body» is false, main's shorter set in ops-main-a11y-plan.md
decision/focus-fixup-reads-inert-alone · reject — check 5: the set drops «in every document» and «takes keys», which the row's guards still pin; main's own set in ops-main-a11y-plan.md carries the expected amendment
contract/cells-grid/editor-enter · reject — check 4: the measured editor was opened by a click; the promise holds for an editor opened by the keyboard (`cells.rs:511-516` returns focus to the grid when `keyed_open`), so the measurement does not contradict it

ops-main-a11y-plan.md
decision/focus-fixup-reads-inert-alone · apply — check 5: the plan's expected amendment, the limit read as a named SC 2.4.3 gap with the tree's focus on the window, every standing clause kept
decision/removed-focused-node-drops-focus · apply — check 4: `get_focussed_node_id` falls back to the root element once `focus_node_id` is cleared, the tree's focus the window (control-2 holds); the standing «to the body» is false

ops-main-architecture.md
contract/ci-leg-legs/a11y · apply — check 7: the shorter set fit's not_master owes — «fourteen» false since the leg runs eighteen (`ci-leg.sh:41`)

ops-main-test-plan.md
decision/a11y-leg-membership · apply — check 7: the shorter set fit's not_master owes — the list no longer the leg's whole membership
