# Answer card — 2026-11-24-canvas-lint, Q1

## The answer it rests on
- **F1 A**, the founder's own answer, given by question dialog on the evening of 2026-11-24 and relayed verbatim through the overseers: «A — слово truncated в lint (Recommended)». A canvas finding reuses the existing `truncated` word inside the `lint` answer; no new defect word, no new reason key, the defect set stays six; the canvas element reads `truncated`, its reason `text` the label's extent and `visible` the part of it inside its region. Record: inputs#I2 (`../additional/escher-overseer/relays/canvas-lint-founder-2026-11-24.md`); its weight at this wrap: inputs#I5 point 2.

## The escalated operations read as lying wholly inside it (7)
- ops-architecture.md · contract/lint-verb/truncated · standard-contracts · fit: fits type 3 — a canvas element reads `truncated` once per label whose ink box is not inside its region.
- ops-architecture.md · contract/lint-verb/finding · standard-contracts · fit: fits type 3 — one finding per defect read, a canvas element's `truncated` once per label in paint order.
- ops-security-plan.md · decision/canvas-label-lint-crossing · threat-model-summary · fit: fits type 1 — a canvas label crosses the lint's carriers only as numbers (its ink box and visible region inside `truncated`), never its text, a glyph id or font bytes.
- ops-security-plan.md · decision/lint-findings-closed-content · input-validation · fit: fits type 1 — a finding holds ids, closed words and numbers only, never page text or a canvas label's text.
- ops-layout-templates.md · decision/circle-drawer-circles-painted-on-canvas · surface · fit: fits type 1 — a founder-ratified row's «reach no … lint» re-stated: the lint reads label ink boxes.
- ops-test-plan.md · contract/lint-findings/order · fit: fits type 3 — a canvas element's `truncated` findings come once per label, in paint order.
- ops-a11y-plan.md · decision/circle-drawer-canvas-pixels-only · fit: fits type 1 — a founder-ratified row's «reach no … lint» re-stated: the labels leave as lint `truncated` boxes, no label text among them.

None of the escalated operations reads `not_master`.

## Whose confirm
The founder is away (inputs#I5 point 4). The operator's confirm is a delegate's: each line it resolves carries `delegate the escher overseer, 2026-11-25`, and its rows land PROVISIONAL for the founder's batch at the epoch boundary.

## The answer
Given 2026-11-25 by question dialog in this session, from the escher overseer acting as the founder's delegate: «Confirm all 7 (Recommended)», with the note «delegate escher-overseer: the founder is away; record it PROVISIONAL as the confirmer's.» — a delegate's confirm, PROVISIONAL; the ratification stays the founder's F1 A, named as his in each owning sidecar.
