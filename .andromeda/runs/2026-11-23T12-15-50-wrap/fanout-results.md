# Fan-out results — 2026-11-23-canvas-placement-and-text (wrap 2026-11-23T12-15-50-wrap)

architecture: add 3 · set 1 · retire 0 · none 0 · ops-architecture.md
security-plan: add 4 · set 0 · retire 0 · none 1 · ops-security-plan.md
design-system: add 2 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: none · ops-layout-templates.md (reason: the chunk adds no user-facing surface or region, no verb, argument, field, refusal or element id; every touched row still reads true)
test-plan: add 4 · set 0 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 0 · set 0 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: no proposer

Expected amendments floor (check 5): architecture §Established Decisions → custom-widget-content-box-paint · §Cross-cutting Patterns → forwarded-event-coordinates set (sub-document-event-forwarding still true: «adjusted to their own space») · §Standard Contracts → widget-fonts and font-ctx · security-plan §Input Validation → custom-widget-content-box-frame and custom-widget-document-font-context · §Dependency Security → cookie-advisory-lock-pin and main's blitz-tests-parley-dev-dependency · test-plan §2 → the three witnesses · obs-plan §6 → the proposer's `@row_none` type 7 (no log site, span or field added; contract/command-span/field-count stays true), no row owed · `v040-09` notes: P7.3 · residual `res-evkq636g`: P5.
No boundary widening (check 1): the three engine changes add no carrier, verb, argument, field, input class or crossing (the report's Symbols / APIs, last line); the font-context handle is in-process app code's, the plan's P5-approved expected amendments name each change and say no escalate pattern is met, so each applies on that recorded direction with the founder's word.

ops-architecture.md
decision/forwarded-event-coordinates · reject — check 7: not_master type 4 — the set lengthens a still-true rule with the helper's mechanism, stating no reason; the widget's content-box pointer frame stands ratified in security-plan decision/custom-widget-content-box-frame
decision/custom-widget-content-box-paint · apply — check 1: the plan's expected amendment names this decision (content-box paint size × scale, the scene at the content origin inside the element's transform, unclipped, #1578; guard stand_canvas_placement) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I5: F1 «A+C: отрисовка и события указателя (Recommended)»
contract/nodes-widgets/widget-fonts · apply — check 1: the plan's expected amendment names the defaulted Widget::fonts contract (fct-wvvcibgm) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I5: F2 «A: контекст шрифтов документа (Recommended)»
contract/base-document/font-ctx · apply — check 1: the plan's expected amendment names the BaseDocument · font_ctx contract (fct-wvvcibgm) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I5: F2 «A: контекст шрифтов документа (Recommended)»

ops-security-plan.md
decision/custom-widget-content-box-frame · apply — check 1: the plan's expected amendment (Input Validation, beside custom-widget-initial-attribute-replay) names paint size, placement and pointer frame at the content box · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I5: F1 «A+C: отрисовка и события указателя (Recommended)»
decision/custom-widget-document-font-context · apply — check 1: the plan's expected amendment (Input Validation) names app code holding a shared, mutable handle to the document's fonts · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I5: F2 «A: контекст шрифтов документа (Recommended)»
decision/cookie-advisory-lock-pin · reject — check 7: not_master type 5 — an inventory of one crate's locked version the lockfile holds; the apply rested on the operator's directive (inputs#I11), a delegate's word, PROVISIONAL; the reviewed pin is recorded as a security-plan sidecar entry (Dependency Security) instead
decision/lock-only-patch-bump-by-hand-edit · reject — check 2: the fact's home is residual res-evkq636g (the syn edges kept at 3.0.6 by edit, its second instance), amended at P5 on the operator's directive point 4 (inputs#I11), never a master rule of its own

ops-design-system.md
decision/custom-widget-text-from-document-fonts · reject — check 2: the fact's home is architecture contract/nodes-widgets/widget-fonts (the handoff) with security-plan decision/custom-widget-document-font-context; design-system holds no rule of it
decision/custom-widget-content-box · reject — check 2: the fact's home is architecture decision/custom-widget-content-box-paint and decision/forwarded-event-coordinates

ops-test-plan.md
decision/app-widget-paints-at-content-box · reject — check 7: not_master type 4 — a coverage line the code and its stand check hold; the witness is named by the architecture row's guard
decision/app-widget-pointer-frame-content-box · reject — check 7: not_master type 4 — a coverage line the code and its stand check hold; named by the security-plan row's guard
decision/canvas-text-from-document-fonts · reject — check 7: not_master type 4 — a coverage line the code and its stand check hold; named by the security-plan row's guard
decision/shared-build-dir-stale-fresh · reject — check 3: a dev-host build hazard the session measured (fct-wrcglkpd), outside the chunk's intent and no test-strategy decision; its home is curation (P3)

ops-main-security-plan.md
decision/blitz-tests-parley-dev-dependency · reject — check 7: not_master type 5 — an inventory of one dev-dependency the manifest and lockfile hold; recorded in the same security-plan sidecar entry

ops-main-architecture.2.md
decision/custom-widget-content-box-paint · apply — check 5: the plan's expected amendment has the body read the bare ratified (fct-wvvcibgm) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I5: F1 «A+C: отрисовка и события указателя (Recommended)»
