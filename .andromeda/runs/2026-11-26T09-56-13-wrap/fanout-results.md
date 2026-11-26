# Fan-out results — 2026-11-26-canvas-keyboard-path wrap

a11y-plan: add 9 · set 2 · retire 0 · none 2 · ops-a11y-plan.md (the re-spawn's file; the first, `.raw-ops-a11y-plan.md`, failed SET_IN_READ on its retire of absence/keyboard-virtual-nodes — the retire is main's own, ops-main-a11y-plan.md)
architecture: add 5 · set 4 · retire 0 · none 0 · ops-architecture.md
design-system: add 6 · set 1 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 8 · set 0 · retire 0 · none 3 · ops-layout-templates.md
obs-plan: add 1 · set 0 · retire 0 · none 0 · ops-obs-plan.md
security-plan: add 4 · set 1 · retire 0 · none 1 · ops-security-plan.md
test-plan: add 2 · set 2 · retire 0 · none 2 · ops-test-plan.md

Fact homes (check 2), read across the seven files together:
- the engine's Tab pass-through on a focused custom-widget element → architecture decision/widget-tab-pass-through; its engine-level check → test-plan decision/widget-tab-pass-through-engine-checked (a test-strategy fact of its own).
- Circle Drawer canvas's Tab stop and keys → a11y-plan contract/circle-drawer-canvas/{tab-stop,arrows,enter,keys-with-dialog-open}.
- a click that selects the topmost circle, else adds → layout-templates decision/circle-drawer-click-selects-or-adds.
- the open dialog disabling the other circles → layout-templates decision/circle-drawer-dialog-disables-other-circles (main's own, provisional).
- a virtual node's declared `selected` in the tree and snapshot → architecture contract/snapshot/virtual-node, contract/nodes-widgets/virtual-nodes, contract/accessibility-tree/virtual-node-selected; its closed-value crossing given no row (fit: not_master type 4, the code holds it).
- the later node covering by its declared bounding box → architecture decision/covered-by-later-node-region.

Words: the founder's F1 «B1: выделение полем» — relayed verbatim by the escher overseer, 2026-11-25, inputs#I2; F1 A, F2 A, F3 B, F4 B, the click that focuses the canvas, the canvas keeping 700 × 400 and the engine's Tab pass-through — delegate escher overseer, 2026-11-26 (inputs#I4, inputs#I7, inputs#I9 item 3), PROVISIONAL.

ops-a11y-plan.md
decision/stated-tab-order-walked · apply — check 5: the plan's expected amendment, Circle Drawer's order joins the walk (CIRCLE_DRAWER_TAB_ORDER measured equal) · word: delegate escher overseer, 2026-11-26
contract/driver-click-refusals/covered · apply — check 5: the plan's expected amendment, the later-node clause; the fixed meaning inside a ratified carrier (rul-fxlxu7mw) · word: delegate escher overseer, 2026-11-26
decision/focused-widget-passes-tab · reject — check 2: the fact's home is architecture decision/widget-tab-pass-through
contract/circle-drawer-canvas/tab-stop · apply — check 1: rul-ipb3atbm, this chunk's canvas keyboard path, beside the Cells grid's Tab-stop contract · word: delegate escher overseer, 2026-11-26
contract/circle-drawer-canvas/arrows · apply — check 1: rul-ipb3atbm, this chunk's canvas arrows · word: delegate escher overseer, 2026-11-26
contract/circle-drawer-canvas/enter · apply — check 1: rul-ipb3atbm, this chunk's canvas Enter · word: delegate escher overseer, 2026-11-26
contract/circle-drawer-canvas/keys-with-dialog-open · apply — check 1: rul-ipb3atbm, the canvas keys inert with the dialog open · word: delegate escher overseer, 2026-11-26
contract/circle-drawer-canvas/click · reject — check 2: the fact's home is layout-templates decision/circle-drawer-click-selects-or-adds
contract/virtual-node-declaration/selected · reject — check 2: the fact's home is architecture contract/snapshot/virtual-node
decision/current-circle-read-from-selected · apply — check 1: the recorded direction inputs#I9 item 6 names this limit, and the P5 card's row 2 names it beside the CRUD list's; provisional on F1 A · word: delegate escher overseer, 2026-11-26
decision/covered-by-declared-region · reject — check 2: the fact's home is architecture decision/covered-by-later-node-region

ops-main-a11y-plan.md
absence/keyboard-virtual-nodes · apply — check 5: the plan's expected amendment retires it; the keyboard route shipped (stand_keyboard, stand_circle_nodes)
decision/circle-drawer-canvas-pixels-only · apply — check 5: the plan's expected amendment, the canvas a Tab stop with its keys · word: delegate escher overseer, 2026-11-26

ops-architecture.md
contract/nodes-widgets/virtual-nodes · apply — check 5: the plan's expected amendment, the `selected` field (rul-cepujoy6, a closed field) · word: relayed verbatim by the escher overseer, 2026-11-25, inputs#I2: the founder's F1 «B1: выделение полем»
contract/snapshot/virtual-node · apply — check 5: the plan's expected amendment, `selected` read from the tree node · word: relayed verbatim by the escher overseer, 2026-11-25, inputs#I2: the founder's F1 «B1: выделение полем»
contract/refusal-causes/covered · apply — check 5: the plan's expected amendment, the later-node clause (rul-fxlxu7mw) · word: delegate escher overseer, 2026-11-26
contract/node-keys/selected · apply — check 1: rul-cepujoy6, a closed value inside the ratified snapshot carrier; the key's source restated · word: relayed verbatim by the escher overseer, 2026-11-25, inputs#I2: the founder's F1 «B1: выделение полем»
decision/widget-tab-pass-through · apply — check 1: no match — rul-47a6zcoz's precondition (the founder's own word) fails and nothing new crosses a boundary (no input class, carrier, verb or field added), so no widening; the recorded direction inputs#I9 item 3 settles it as a delegate's word landing PROVISIONAL; also check 6, the plan's Tab-stop premise disproved · word: delegate escher overseer, 2026-11-26
decision/widget-focused-on-pointer-down · reject — check 7: not_master type 4 — a measurement of how the engine's pointer-down handling already behaves, the code holds it; check 6's disposition routed to P3 curation (the report's Decisions, control C6)
decision/circle-drawer-canvas-single-tab-stop · reject — check 2: the fact's home is a11y-plan contract/circle-drawer-canvas/tab-stop
contract/accessibility-tree/virtual-node-selected · apply — check 1: rul-ipb3atbm, the tree writes a declared `selected` and keeps its focus on the canvas · word: delegate escher overseer, 2026-11-26
decision/covered-by-later-node-region · apply — check 1: the recorded direction inputs#I9 item 6 names this limit; provisional on F3 B · word: delegate escher overseer, 2026-11-26

ops-design-system.md
decision/circle-drawer-canvas-colours-in-code · apply — check 5: the plan's expected amendment, SELECTED_FILL
decision/later-virtual-node-covers-earlier · reject — check 2: the fact's home is architecture decision/covered-by-later-node-region
decision/circle-drawer-canvas-keyboard · reject — check 2: the fact's home is a11y-plan contract/circle-drawer-canvas/tab-stop
decision/circle-drawer-click-selects-topmost · reject — check 2: the fact's home is layout-templates decision/circle-drawer-click-selects-or-adds
decision/widget-never-traps-tab · reject — check 2: the fact's home is architecture decision/widget-tab-pass-through
contract/virtual-node-accessibility/selected · reject — check 2: the fact's home is architecture contract/snapshot/virtual-node
decision/circle-drawer-dialog-disables-other-circles · reject — check 2: the fact's home is layout-templates decision/circle-drawer-dialog-disables-other-circles

ops-layout-templates.md
decision/circle-drawer-canvas-keyboard-region · reject — check 2: the fact's home is a11y-plan contract/circle-drawer-canvas/tab-stop
decision/circle-drawer-click-selects-or-adds · apply — check 1: rul-ipb3atbm, the click that selects is the founder's own B1 · word: relayed verbatim by the escher overseer, 2026-11-25, inputs#I2: the founder's F1 «B1: выделение полем»
decision/circle-drawer-dialog-disables-other-circles · reject — written with no provisional mark though it rests on F4 B alone; restated as main's own in ops-main-layout-templates.md
decision/circle-drawer-canvas-keeps-size · reject — written with no provisional mark though the canvas size is the delegate's word (inputs#I9 item 3); restated as main's own in ops-main-layout-templates.md
decision/circle-drawer-canvas-selection-attributes · reject — written with no provisional mark though the selection's attributes rest on F4 B; restated as main's own in ops-main-layout-templates.md
decision/widget-tab-pass-through · reject — check 2: the fact's home is architecture decision/widget-tab-pass-through (its `area=new` asks nothing)
contract/escher-session-refusals/covered · reject — check 2: the fact's home is architecture contract/refusal-causes/covered
contract/escher-session-snapshot/virtual-node-selected · reject — check 2: the fact's home is architecture contract/snapshot/virtual-node

ops-main-layout-templates.md
decision/circle-drawer-dialog-disables-other-circles · apply — check 1: rul-ipb3atbm, the open dialog disables the other circles · word: delegate escher overseer, 2026-11-26
decision/circle-drawer-canvas-keeps-size · apply — check 6: the plan's steps 7–8 Close premise disproved; the canvas keeping 700 × 400 is the delegate's (inputs#I9 item 3) · word: delegate escher overseer, 2026-11-26
decision/circle-drawer-canvas-selection-attributes · reject — check 7: not_master type 4 — the app's hand-off of selection to its own widget is internal plumbing, the code holds it; the plan's expected amendment for decision/circle-drawer-circles-painted-on-canvas ends not carried on that verdict

ops-obs-plan.md
contract/snapshot-node/virtual-node-selected · reject — check 2: one fact with security-plan decision/virtual-node-selected-closed, whose add fit read not_master type 4 (the code holds it); the snapshot's selected stands in architecture contract/snapshot/virtual-node

ops-security-plan.md
contract/refusal-causes/covered · apply — check 5: the plan's expected amendment, the later-node clause (rul-fxlxu7mw) · word: delegate escher overseer, 2026-11-26
decision/custom-widget-tab-pass-through · reject — check 2: the fact's home is architecture decision/widget-tab-pass-through
decision/virtual-node-selected-closed · reject — check 7: not_master type 4 — the declaration's build retold, the closed crossing of control states stands in its own row; the code holds it
decision/circle-drawer-canvas-keyboard · reject — check 2: the fact's home is a11y-plan contract/circle-drawer-canvas/tab-stop
decision/covered-region-bounding-box · reject — check 2: the fact's home is architecture decision/covered-by-later-node-region

ops-test-plan.md
contract/refusal-cause/covered · apply — check 5: the plan's expected amendment, the later-node clause · word: delegate escher overseer, 2026-11-26
decision/circles-apart-written-through-attribute · apply — check 5: the plan's expected amendment, with its measured limit
decision/widget-tab-pass-through-engine-checked · apply — check 1: rul-ipb3atbm, the engine-level check and its red control C7 · word: delegate escher overseer, 2026-11-26
decision/canvas-keyboard-focus-readings-agree · apply — check 1: rul-ipb3atbm, the three focus readings the stand checks assert

ops-main-security-plan.md
contract/diff-node/selected · apply — check 1: rul-cepujoy6, the diff node key restated for a virtual node's declared selected (the cascade old-wording sweep, sel-aria-only) · word: relayed verbatim by the escher overseer, 2026-11-25, inputs#I2: the founder's F1 «B1: выделение полем»
