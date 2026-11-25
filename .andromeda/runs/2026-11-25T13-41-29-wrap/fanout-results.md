# Fan-out results — 2026-11-25-virtual-nodes-in-every-read (wrap 2026-11-25T13-41-29-wrap)

architecture: add 3 · set 8 · retire 0 · none 1 · ops-architecture.md
security-plan: add 2 · set 3 · retire 0 · none 2 · ops-security-plan.md
design-system: add 2 · set 4 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 4 · set 2 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 5 · set 5 · retire 2 · none 1 · ops-test-plan.md
obs-plan: add 2 · set 0 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: add 2 · set 2 · retire 0 · none 2 · ops-a11y-plan.md

Expected amendments not raised as operations (check 5), each read and found still true: security-plan decision/stable-id-exits, decision/graph-answer-crossing, decision/actions-answer-crossing, decision/coverage-answer-crossing, decision/kept-check-graph-answer, decision/kept-check-actions-answer, decision/kept-check-layout-answer, decision/cold-agent-screen-to-provider — each carries «ids» whatever their form, a virtual id among them; architecture contract/answer-json/layout-placing (already lists `app`), contract/driver-session/layout (resolves as the run path does — layout's run path now takes a virtual id; the virtual answer stands in the add contract/driver-session/layout-virtual-node), contract/actions-verb/target-verbs (its virtual-node case stands in the add contract/actions-verb/virtual-node); test-plan contract/handler-coverage-limits/canvas-handler and decision/handler-coverage-canvas-uncounted (a handler inside a canvas is still not counted; the virtual click's count is architecture contract/handler-coverage/canvas's); a11y-plan decision/keyed-wire-form (the limit's one home is architecture decision/keyed-wire-form's `except`); obs-plan decision/declared-node-read-logs-nothing (the new reads' no-span fact stands in obs-plan's add decision/virtual-node-reads-open-no-span).

## ops-architecture.md
contract/driver-verbs/click-virtual-node · apply — check 5: the plan's expected amendment, `click` and `layout` take a virtual id (F16 A) · word: delegate the escher overseer, 2026-11-25
contract/refusal-causes/not-found · apply — check 5: expected amendment, the same two verbs (F16 A) · word: delegate the escher overseer, 2026-11-25
contract/ui-graph/graph · apply — check 5: expected amendment, the virtual-node exception dropped (F13 A, F14 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2: «A — id во всех чтениях записи (Recommended)»
contract/ui-graph/owner · apply — check 5: expected amendment, a select's or a canvas's (F14 A) · word: delegate the escher overseer, 2026-11-25
contract/handler-coverage/canvas · reject — check 4: the value drops «handlers inside a canvas are not counted», still true; main's ops-main-architecture.md sets the row keeping it
contract/dioxus-bridge/virtual-node-id · apply — check 1: the reads the id is served in grew (F13 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2: «A — id во всех чтениях записи (Recommended)»
contract/stable-element-id/virtual-node-id · apply — check 1: the reads the id is served in grew (F13 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2: «A — id во всех чтениях записи (Recommended)»
contract/driver-session/canvas · apply — check 1: canvas still refuses a virtual id while layout takes one, the row's «as Session::layout does» false (F16 A) · word: delegate the escher overseer, 2026-11-25
contract/driver-session/layout-virtual-node · apply — check 5: expected amendment, `app` produced for a virtual id (F16 A, F17 A) · word: delegate the escher overseer, 2026-11-25
decision/virtual-id-verb-table · apply — check 1: the one table implement chose, a decision future verbs keep (report Deviations, self) · word: default, implement's own choice
contract/actions-verb/virtual-node · apply — check 5: expected amendment, a virtual node's four not-found (F18 A) · word: delegate the escher overseer, 2026-11-25

## ops-main-architecture.md
contract/handler-coverage/canvas · apply — check 5: expected amendment restated for virtual nodes, its true clause kept (F19 A) · word: delegate the escher overseer, 2026-11-25
decision/keyed-wire-form · reject — check 7: not_master type 4 — the rule already reads false for any id holding a slash; the cell only retells how that falls for one kind of node, which the code holds

## ops-security-plan.md
decision/virtual-node-crossing · escalate → Q1 — check 1: a boundary widening (the never-routine class), F13 A the founder's own answer: the answer card · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I5: «Подтвердить все 3 (Recommended)», on his F13 «A — id во всех чтениях записи (Recommended)» (inputs#I2)
contract/actions-answer/verbs · apply — check 5: expected amendment, a virtual node's four not-found (F18 A) · word: delegate the escher overseer, 2026-11-25
contract/stable-element-id/virtual-node-form · escalate → Q1 — check 1: the reads the reserved id is served in, riding the widening F13 A: the answer card · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I5: «Подтвердить все 3 (Recommended)», on his F13 «A — id во всех чтениях записи (Recommended)» (inputs#I2)
decision/virtual-id-verbs · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
decision/virtual-node-keyed-clickable-false · reject — check 2: the fact's home is architecture decision/keyed-wire-form (keyed); clickable false stands in a11y-plan contract/snapshot/clickable's element reading and a residual row

## ops-main-security-plan.2.md
decision/virtual-node-crossing · escalate → Q1 — check 1: the crossing row's guard names, renamed with the widening's guards: the answer card · resolved: apply (Q1) — relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I5: «Подтвердить все 3 (Recommended)», on his F13 «A — id во всех чтениях записи (Recommended)» (inputs#I2)

## ops-design-system.md
contract/actions-line/nodes · apply — check 5: expected amendment, «A virtual node is never listed» dropped (F13 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2: «A — id во всех чтениях записи (Recommended)»
contract/actions-line/content · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing (no declared name crosses)
contract/ui-graph-line/content · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing
contract/coverage-line/content · reject — check 2: the fact's home is architecture contract/handler-coverage/canvas
decision/virtual-id-verbs-click-and-layout · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
contract/escher-session-commands/layout-answer-virtual-node · reject — check 2: the fact's home is architecture contract/driver-session/layout-virtual-node

## ops-layout-templates.md
contract/escher-session-actions/answer · reject — check 2: the fact's home is architecture contract/actions-verb/virtual-node
contract/escher-session-lint/answer · reject — check 2: the fact's home is test-plan decision/reserved-graph-id-form
decision/virtual-id-verbs · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
contract/escher-session-layout/virtual-node · reject — check 2: the fact's home is architecture contract/driver-session/layout-virtual-node
contract/escher-session-graph/virtual-node · reject — check 2: the fact's home is architecture contract/ui-graph/graph and contract/ui-graph/owner
contract/escher-session-coverage/virtual-node-click · reject — check 2: the fact's home is architecture contract/handler-coverage/canvas

## ops-test-plan.md
contract/refusal-cause/not-found · apply — check 5: expected amendment, every verb but `click` and `layout` (F16 A) · word: delegate the escher overseer, 2026-11-25
contract/driver-refusal-causes/not-found · apply — check 5: expected amendment (F16 A) · word: delegate the escher overseer, 2026-11-25
contract/driver-session/stale-or-not-found · apply — check 5: expected amendment (F16 A) · word: delegate the escher overseer, 2026-11-25
contract/handler-coverage-limits/canvas-handler · reject — check 4: «a handler inside a canvas is not counted» still holds; the virtual click's count is architecture contract/handler-coverage/canvas's
contract/element-id/virtual-form · apply — check 5: expected amendment, served in the new reads (F13 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2: «A — id во всех чтениях записи (Recommended)»
decision/reserved-graph-id-form · reject — check 5: the plan's expected amendment narrows the row in place (lint answer, and the stand's graph and actions while it declares none); main's ops-main-test-plan.md sets it
decision/handler-coverage-canvas-uncounted · reject — check 4: the rule (a handler inside a canvas is not counted) still holds
decision/lint-holds-no-virtual-node · reject — check 5: the heir of a rejected retire; the narrowed decision/reserved-graph-id-form holds the lint's half
decision/handler-coverage-virtual-click-counts-canvas · reject — check 2: the fact's home is architecture contract/handler-coverage/canvas
contract/session-run/layout-virtual-node · reject — check 2: the fact's home is architecture contract/driver-session/layout-virtual-node
contract/driver-verbs/actions-virtual-node · reject — check 2: the fact's home is architecture contract/actions-verb/virtual-node
contract/exploration/virtual-nodes · reject — check 2: the fact's home is architecture contract/driver-explore/expansion (one child per actionable node, a listed virtual node among them) and security-plan decision/virtual-node-crossing (kept checks)

## ops-main-test-plan.md
decision/reserved-graph-id-form · apply — check 5: expected amendment, narrowed (F13 A, F20 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2: «A — id во всех чтениях записи (Recommended)»

## ops-main-test-plan.2.md
decision/reserved-graph-id-form · apply — check 5: the narrowed rule's guards (F20 A) · word: delegate the escher overseer, 2026-11-25

## ops-obs-plan.md
decision/virtual-node-answers-carry-id-alone · reject — check 2: the fact's home is security-plan decision/virtual-node-crossing
decision/virtual-node-reads-open-no-span · reject — check 7: not_master type 4 — the basis reports only that no span, field or log site was added, which the code holds, and the standing layout, canvas and exploration no-span rows bind the rest

## ops-a11y-plan.md
decision/ui-graph-one-node-per-snapshot-node · reject — check 4: the value says handler coverage holds virtual nodes, which the measurement contradicts (neither coverage line holds `///`, report Outcome (coverage)); main's ops-main-a11y-plan.md sets the row without it
contract/node-id/reserved-separator · apply — check 5: expected amendment, the reads it is served in (F13 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2: «A — id во всех чтениях записи (Recommended)»
decision/virtual-id-verbs · reject — check 2: the fact's home is architecture contract/driver-verbs/click-virtual-node
contract/layout-answer/virtual-node · reject — check 2: the fact's home is architecture contract/driver-session/layout-virtual-node

## ops-main-a11y-plan.md
decision/ui-graph-one-node-per-snapshot-node · apply — check 5: expected amendment, the virtual-node exception dropped (F13 A, F14 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-25, inputs#I2: «A — id во всех чтениях записи (Recommended)»
