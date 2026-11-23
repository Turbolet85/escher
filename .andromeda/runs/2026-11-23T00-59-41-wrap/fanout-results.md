# Fan-out results — 2026-11-22-upstream-sync-ahead-of-the-canvas

architecture: add 4 · set 1 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 0 · retire 0 · none 1 · ops-security-plan.md
design-system: add 2 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: none · ops-layout-templates.md — reason: a clean merge of upstream engine fixes, no user-facing surface or region; the three picked rows stand
test-plan: add 0 · set 0 · retire 0 · none 2 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md — reason: no log site, span, field, telemetry dependency or CI artifact; both WPT repository guards kept
a11y-plan: add 1 · set 0 · retire 0 · none 0 · ops-a11y-plan.md

Main's own: ops-main-architecture.md — add 1 (E6, the plan's expected architecture amendment no proposer raised — check 5).

## Dispositions

ops-architecture.md
contract/layout/stylo-taffy-exports · apply — check 5: the plan's expected architecture §Standard Contracts amendment names this change itself (`convert::float` / `convert::clear` take an `ltr: bool`, E2); a public signature of the bridge, no boundary widening; the report's Symbols / APIs bullet carries it · word: relayed verbatim by the overseer, 2026-11-22, inputs#I2
decision/ime-preedit-composition-underline · reject — check 2: the fact's home is security-plan decision/ime-composition-underline (the plan's expected amendment puts E5 in security-plan §Untrusted input), itself read not_master type 4 by fit — the code holds it
decision/logical-float-clear-by-direction · escalate → Q1 — check 1: rul-72xhfjoz (boundary widening by upstream merge, escalate) on the founder's take E2, an upstream behaviour change for every document; his own answer to the P4 fork (inputs#I2) covers it — the answer card · resolved: apply (Q1) — operator, 2026-11-23: Confirm all three (Recommended) · word: relayed verbatim by the overseer, 2026-11-22, inputs#I2
decision/block-containment-through-display-contents · escalate → Q1 — check 1: rul-72xhfjoz on the founder's takes E3 and E4, upstream behaviour changes for every document; his own answer (inputs#I2) covers it — the answer card · resolved: apply (Q1) — operator, 2026-11-23: Confirm all three (Recommended) · word: relayed verbatim by the overseer, 2026-11-22, inputs#I2
contract/page-script-dom-api/domtokenlist-replace · reject — check 2: the fact's home is security-plan decision/dom-token-list-replace (the plan's expected amendment puts E1 in security-plan §Untrusted input), itself read not_master type 4 by fit — the code holds it

ops-main-architecture.md
decision/textarea-text-laid-out-once-width-known · escalate → Q1 — check 5 raise (the plan's expected architecture amendment, E6) under check 1: rul-72xhfjoz on the founder's take E6; his own answer (inputs#I2) covers it — the answer card · resolved: apply (Q1) — operator, 2026-11-23: Confirm all three (Recommended) · word: relayed verbatim by the overseer, 2026-11-22, inputs#I2

ops-security-plan.md
decision/dom-token-list-replace · reject — check 7: not_master type 4 — how one script interface is built, taken from upstream as written, no reason of its own beyond the take; the code holds it; escalated (Q1), not asked · the operator left it out, 2026-11-23: Leave them out (Recommended)
decision/ime-composition-underline · reject — check 7: not_master type 4 — how the painter draws one decoration, taken from upstream as written, no reason of its own beyond the take; the code holds it; escalated (Q1), not asked · the operator left it out, 2026-11-23: Leave them out (Recommended)

ops-design-system.md
decision/ime-composition-underline-engine-painted · reject — check 2: the fact's home is security-plan decision/ime-composition-underline
decision/float-clear-sides-follow-text-direction · reject — check 2: the fact's home is architecture decision/logical-float-clear-by-direction

ops-a11y-plan.md
decision/ime-composition-underlined · reject — check 2: the fact's home is security-plan decision/ime-composition-underline
