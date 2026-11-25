# Fan-out results — 2026-11-25-virtual-nodes-as-elements (wrap .andromeda/runs/2026-11-25T09-46-09-wrap)

architecture: add 7 · set 8 · retire 1 · none 0 · ops-architecture.md
security-plan: add 4 · set 5 · retire 0 · none 1 · ops-security-plan.md (the re-spawn propose-r1's; the first draft, `.raw-ops-security-plan.md`, failed SET_IN_READ on its set of absence/virtual-nodes-on-carriers, which the re-spawn dropped — main's own retire takes that row)
design-system: add 6 · set 1 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 3 · set 0 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 5 · set 7 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 4 · set 1 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 7 · set 4 · retire 0 · none 1 · ops-a11y-plan.md

Words: F3 A, F11 A, F8 A (with F2 A) — the founder's own, relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2 (authority set by inputs#I6 item 2). F7 A, F9 A, F10 B, F12 A — delegate escher overseer, 2026-11-25, inputs#I2; the V2 ruling — delegate escher overseer, 2026-11-25, inputs#I5 (inputs#I6 item 3).

ops-architecture.md
contract/accessibility-tree/virtual-nodes · apply — check 1: rul-ipb3atbm, the tree's new children are this chunk's (Changes, Symbols / APIs); the fact's home for the tree contract, its twins in a11y-plan, test-plan, design-system, layout-templates and security-plan rejected by check 2
decision/virtual-node-tree-id · reject — check 1: rests on a delegate's answer (F7 A) yet carries no provisional mark; re-raised by main in ops-main-architecture.md with `state: provisional`
contract/snapshot/virtual-node · apply — check 1: rul-ipb3atbm, the snapshot's reading of a virtual node is this chunk's; the fact's home, its twins elsewhere rejected by check 2 · word: delegate escher overseer, 2026-11-25, inputs#I2 (F12 A, `clickable` false)
contract/driver-verbs/click-virtual-node · apply — check 1: rul-ipb3atbm, `click` resolving a virtual id and the other id verbs reading it `not-found` are this chunk's; an id argument of the existing kind, no new input class · word: delegate escher overseer, 2026-11-25, inputs#I2 (F10 B)
decision/virtual-node-crossing · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing
decision/declaration-change-marks-nothing · apply — check 5: the plan's Expected amendment drops «yet» from decision/virtual-node-declaration-marks-nothing; heir of its retire · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2 (F11 A «A — без сигнала и без пометки (Recommended)»)
contract/stable-element-id/virtual-node-form-served · reject — check 2: the same fact stands under contract/stable-element-id/virtual-node-id, which main sets shorter in ops-main-architecture.md (one thing under one key)
decision/virtual-node-declaration-marks-nothing · apply — check 5: the plan's Expected amendment; retired for its heir decision/declaration-change-marks-nothing · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2 (F11 A)
contract/ui-graph/graph · apply — check 5: the plan's Expected amendment (`ui-graph/graph` excepting virtual nodes) · word: delegate escher overseer, 2026-11-25, inputs#I2 (F9 A)
contract/refusal-causes/covered · apply — check 5: the plan's Expected amendment (`covered` cause) · word: delegate escher overseer, 2026-11-25, inputs#I2 (F12 A)
contract/snapshot/bounds · apply — check 5: the plan's Expected amendment (snapshot bounds); the unrotated, unscaled reading of a transformed canvas is the stated limit
contract/snapshot/membership · apply — check 5: the plan's Expected amendment (snapshot membership)
contract/refusal-causes/not-found · apply — check 5: the plan's Expected amendment (`not-found` cause) · word: delegate escher overseer, 2026-11-25, inputs#I2 (F10 B)
contract/dioxus-bridge/virtual-node-id · escalate → Q1 — check 1: rul-izni57qv, a boundary widening: the reserved form now served on the tree, snapshot and diff (F3 A) · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»
contract/nodes-widgets/virtual-nodes · apply — check 5: the plan's Expected amendment (`nodes-widgets/virtual-nodes` gaining `disabled`) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2 (F8 A with F2 A «A — на трейте Widget движка (Recommended)»)
decision/canvas-placement-at-click-centre · apply — check 1: rul-ipb3atbm, a click now also lands at a declared region, this chunk's · word: delegate escher overseer, 2026-11-25, inputs#I2 (F10 B)

ops-main-architecture.md
decision/virtual-node-tree-id · reject — check 7: not_master type 4 — the text retells how one function packs a virtual node's tree id; the code holds it (an add: nothing follows)
contract/stable-element-id/virtual-node-id · escalate → Q1 — check 1: rul-izni57qv, a boundary widening: the reserved form now served (F3 A) · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»

ops-security-plan.md
contract/stable-element-id/virtual-node-form · escalate → Q1 — check 1: rul-izni57qv, the reserved form served on the tree, snapshot and diff (F3 A) · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»
contract/stable-element-id/no-collision · escalate → Q1 — check 1: rul-izni57qv, a served id now holds `//` (F3 A) · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»
decision/author-id-on-accessibility-nodes · escalate → Q1 — check 1: rul-izni57qv, the tree's author id now carries the virtual form (F3 A) · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»
decision/target-refusal-order · apply — check 1: rul-ipb3atbm, a click on a virtual node takes the one target check in its order, this chunk's · word: delegate escher overseer, 2026-11-25, inputs#I2 (F10 B)
contract/refusal-causes/covered · apply — check 1: rul-ipb3atbm, the security-plan's own covered-meaning row restated as the chunk changed it · word: delegate escher overseer, 2026-11-25, inputs#I2 (F12 A)
decision/virtual-node-crossing · escalate → Q1 — check 1: rul-izni57qv, the crossing itself (F3 A); the fact's home, its twins in architecture, a11y-plan, test-plan, obs-plan, design-system and layout-templates rejected by check 2 · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»
decision/click-only-verb-taking-virtual-id · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
decision/virtual-node-snapshot-readings · reject — check 2: the fact's home is architecture contract/snapshot/virtual-node
decision/virtual-nodes-only-from-shown-canvas · reject — check 2: the fact's home is architecture contract/accessibility-tree/virtual-nodes

ops-main-security-plan.md
absence/virtual-nodes-on-carriers · escalate → Q1 — check 1: rul-izni57qv, the absence ends because the crossing widened (F3 A); a retire brings no text · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»

ops-design-system.md
contract/actions-line/nodes · apply — check 1: rul-ipb3atbm, the actions answer holds no virtual node, this chunk's · word: delegate escher overseer, 2026-11-25, inputs#I2 (F9 A)
contract/canvas-virtual-nodes/accessibility-tree · reject — check 2: the fact's home is architecture contract/accessibility-tree/virtual-nodes
contract/canvas-virtual-nodes/snapshot · reject — check 2: the fact's home is architecture contract/snapshot/virtual-node
contract/canvas-virtual-nodes/id-verbs · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
contract/canvas-virtual-nodes/covered · reject — check 2: the fact's home is architecture contract/refusal-causes/covered
contract/canvas-virtual-nodes/carriers · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing
contract/canvas-virtual-nodes/disabled · reject — check 2: the fact's home is architecture contract/nodes-widgets/virtual-nodes

ops-layout-templates.md
contract/escher-session-id-verbs/virtual-node-id · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
contract/escher-session-answers/virtual-nodes · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing
contract/accessibility-tree/canvas-virtual-nodes · reject — check 2: the fact's home is architecture contract/accessibility-tree/virtual-nodes

ops-test-plan.md
decision/reserved-graph-id-form · reject — check 7: not_master type 4 — the lint's by-construction clause lengthens the rule with what the code holds; escalated (Q1), not asked; the row's «No served id holds» is false, so main writes the shorter set in ops-main-test-plan.md (research's disproved lint premise: disposed — the code holds the by-construction fact, held by `the_graph_actions_and_lint_hold_no_virtual_node`; check 6)
contract/element-id/virtual-form · escalate → Q1 — check 1: rul-izni57qv, the reserved form now served (F3 A) · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»
contract/driver-session/stale-or-not-found · apply — check 1: rul-ipb3atbm, a virtual id read `not-found` on every verb but `click`, this chunk's · word: delegate escher overseer, 2026-11-25, inputs#I2 (F10 B)
contract/driver-session/refusal-order · apply — check 1: rul-ipb3atbm, a virtual click refused in the one order · word: delegate escher overseer, 2026-11-25, inputs#I2 (F10 B)
contract/driver-refusal-causes/not-found · apply — check 1: rul-ipb3atbm, the `not-found` meaning reworded by this chunk · word: delegate escher overseer, 2026-11-25, inputs#I2 (F10 B)
contract/refusal-cause/not-found · apply — check 1: rul-ipb3atbm, the same, in the refusal-cause contract · word: delegate escher overseer, 2026-11-25, inputs#I2 (F10 B)
contract/refusal-cause/covered · apply — check 1: rul-ipb3atbm, the `covered` meaning reworded by this chunk · word: delegate escher overseer, 2026-11-25, inputs#I2 (F12 A)
contract/session-run/virtual-node-click · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
contract/snapshot-node/virtual-node · reject — check 2: the fact's home is architecture contract/snapshot/virtual-node
contract/accessibility-tree/virtual-nodes · reject — check 2: the fact's home is architecture contract/accessibility-tree/virtual-nodes
decision/virtual-node-crosses-mcp-and-kept-check · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing
decision/inert-guard-dropped · apply — check 6: disposes the plan acceptance «every control V1–V8 reads red», unmet for V2, on the operator's ruling; provisional · word: delegate escher overseer, 2026-11-25, inputs#I5 (the V2 ruling)

ops-main-test-plan.md
decision/reserved-graph-id-form · escalate → Q1 — check 1: rul-izni57qv, the rule narrows to the graph, actions and lint answers because the form is now served (F3 A) · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»

ops-obs-plan.md
decision/declared-node-read-logs-nothing · apply — check 5: the plan's Expected amendment (obs-plan §6 widened); no log site, span or field added, `stand_act_spans` the guard
decision/virtual-node-id-and-name-crossing · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing; its no-log clause stands in obs-plan decision/declared-node-read-logs-nothing
contract/snapshot-node/virtual-node · reject — check 2: the fact's home is architecture contract/snapshot/virtual-node
decision/click-alone-takes-virtual-id · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
decision/virtual-nodes-off-graph-actions-lint · reject — check 2: the fact's home is test-plan decision/reserved-graph-id-form

ops-a11y-plan.md
decision/ui-graph-one-node-per-snapshot-node · apply — check 5: the plan's Expected amendment (a11y-plan §7 graph exception) · word: delegate escher overseer, 2026-11-25, inputs#I2 (F9 A)
contract/node-id/reserved-separator · escalate → Q1 — check 1: rul-izni57qv, «no served id holds ///» ends because the form is served (F3 A) — the founder's «one a11y-plan row» · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I7: «Подтвердить все 10 (Recommended)»
contract/snapshot/enabled · apply — check 5: the plan's Expected amendment (`enabled` from the declared state) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2 (F8 A with F2 A)
contract/driver-click-refusals/covered · apply — check 1: rul-ipb3atbm, the a11y-plan's own covered row restated as the chunk changed it · word: delegate escher overseer, 2026-11-25, inputs#I2 (F12 A)
contract/accessibility-tree/virtual-nodes · reject — check 2: the fact's home is architecture contract/accessibility-tree/virtual-nodes
contract/snapshot/virtual-node · reject — check 2: the fact's home is architecture contract/snapshot/virtual-node
contract/driver-verbs/virtual-id · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
decision/virtual-node-crosses-carriers · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing
decision/transformed-canvas-virtual-bounds · reject — check 2: the fact's home is architecture contract/snapshot/bounds, set by ops-architecture.md
decision/lint-holds-no-virtual-node · reject — check 2: the fact's home is test-plan decision/reserved-graph-id-form
decision/declaration-change-reaches-adapter-late · reject — check 2: the fact's home is architecture decision/declaration-change-marks-nothing
