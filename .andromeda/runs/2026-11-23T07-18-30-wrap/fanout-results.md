# Fan-out results — 2026-11-23-canvas-in-a-headless-shot (wrap 2026-11-23T07-18-30-wrap)

architecture: add 2 · set 0 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 0 · retire 0 · none 0 · ops-security-plan.md
design-system: add 1 · set 0 · retire 0 · none 1 · ops-design-system.md
layout-templates: no proposer
test-plan: add 4 · set 0 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 2 · set 0 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 1 · set 0 · retire 0 · none 0 · ops-a11y-plan.md

No fix was made to any file.

## Dispositions

ops-architecture.md
decision/canvas-crosses-as-pixels · escalate → Q1 — check 1: a boundary widening (rul-izni57qv, never routine) — a canvas's app-painted content as a new content class in a shot on every ratified carrier; the plan's expected amendment (architecture §Established Decisions, beside select-list-crossing) on the founder's F1 A, inputs#I4 — the answer card · resolved: apply (Q1) — operator, 2026-11-23: «Confirm both (Recommended)» · word: relayed verbatim by the overseer, 2026-11-23, inputs#I4: F1 «Ратифицировать для всех каналов (Recommended)»
decision/harness-paints-custom-widgets · apply — check 1: the plan's expected amendment names this change itself (architecture §Established Decisions, blitz-test-harness names blitz-paint's custom-widget, guard stand_canvas_shot); check 5 carried by the report's Crates / modules

ops-security-plan.md
decision/canvas-content-crosses-as-shot-pixels · escalate → Q1 — check 1: a boundary widening (rul-izni57qv, never routine) — the founder's F1 A, inputs#I4; the plan's expected amendment (security-plan §Threat Model Summary, beside open-list-shot-pixels) — the answer card · resolved: apply (Q1) — operator, 2026-11-23: «Confirm both (Recommended)» · word: relayed verbatim by the overseer, 2026-11-23, inputs#I4: F1 «Ратифицировать для всех каналов (Recommended)»
decision/harness-blitz-paint-custom-widget · apply — check 1: the plan's expected amendment names this change itself (security-plan §Dependency Security: the harness gains blitz-paint's custom-widget feature, no package); check 5 carried by the report's Dependencies

ops-design-system.md
contract/screenshot/custom-widget · reject — check 2: the fact's home is security-plan decision/canvas-content-crosses-as-shot-pixels (the crossing) and test-plan decision/app-widget-shows-in-shots (the witness); one fact twice

ops-test-plan.md
decision/app-widget-shows-in-shots · apply — check 1: the plan's expected amendment names this change itself (test-plan §2 Test Strategy, stand_canvas_shot the witness a widget paints in a harness and a driver shot per package and in the workspace build); check 5 carried by the report's Files and gates
decision/harness-turns-on-custom-widget-paint · reject — check 2: the fact's home is architecture decision/harness-paints-custom-widgets; one fact twice
decision/canvas-crosses-only-as-shot-pixels · reject — check 2: the fact's home is security-plan decision/canvas-content-crosses-as-shot-pixels; one fact twice
decision/stand-failure-messages-hold-no-content · reject — check 3: the report's deviation is one check's failure message printing the pixel it read (stand_canvas_shot); a convention for every stand check's failure message is not a fact the report carries

ops-obs-plan.md
decision/canvas-content-crosses-as-pixels · reject — check 2: the fact's home is security-plan decision/canvas-content-crosses-as-shot-pixels; one fact twice
decision/pixel-check-failures-name-mode-row-pixel · reject — check 3: the report's deviation is one check's failure message; a contract for every pixel-reading stand check is not a fact the report carries

ops-a11y-plan.md
decision/canvas-content-leaves-only-as-pixels · reject — check 2: the fact's home is security-plan decision/canvas-content-crosses-as-shot-pixels; one fact twice
