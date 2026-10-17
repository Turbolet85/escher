# Code Audit — escher · Epoch 1 — Foundation · 2026-10-17T12:14:26Z
mode trend · HEAD 143b6013229d785f4613895e16c1c82b1c1a5be2 · baseline af1a00ad5b346697ef8b20004e4f1f769ce772f2 (Epoch 6 — Polish & ship, escher-0.1.0) · span 1

Overshoot: this run 0 commits (HEAD is the boundary, the 2026-10-16-fast-failing-mutants complete flip); the baseline record's 0 commits.
Label: «Epoch 1 — Foundation» repeats the ledger's first record's label (escher-0.1.0's Epoch 1, d4113768). This record is escher-0.2.0's Epoch 1, a different boundary, recorded as `trend` against 0.1.0's Epoch 6 at span 1 on the operator's choice at the epoch dialogue — a label join across the two proposal documents needs the sha beside it.
No ancestry break; no trend-break (every tool token equal to the baseline's; cargo-nextest 0.9.146 is new — Informational).

## Proposals
None. No threshold fired at this boundary.

## Informational
- **mutation-drop · not comparable** — seven_guis: 100.0 at unit (Epoch 6 — Polish & ship) · 100.0 at files [examples/seven_guis/src/tasks/timer.rs] (Epoch 1 — Foundation) — no delta
- **mutation-drop · not comparable** — escher-driver: 100.0 at unit (Epoch 6 — Polish & ship) · 100.0 at files [packages/escher-driver/src/mcp.rs, packages/escher-driver/src/mcp/framing.rs, packages/escher-driver/src/mcp/listing.rs, packages/escher-driver/src/mcp/result.rs, packages/escher-driver/src/schema.rs, packages/escher-driver/src/schema/spec.rs] (Epoch 1 — Foundation) — no delta
- **mutation counts** — seven_guis: complete 19/19 · caught 18 · missed 0 · not measured 0 · timeout 0 · unviable 1
- **mutation counts** — escher-driver: complete 102/102 · caught 84 · missed 0 · not measured 0 · timeout 0 · unviable 18
- **mutation counts** — blitz-dom/range-keyboard: complete 135/135 · caught 129 · missed 0 · not measured 0 · timeout 0 · unviable 6
- **top-N entrants** — complexity.top (fn + file): 1 new — resolve_color_stops · packages/blitz-paint/src/gradient.rs:325 · 56.0
- **top-N entrants** — sizes.top (file): 1 new — packages/blitz-dom/src/layout/inline.rs · 916
- **top-N entrants** — hotspots (file): 7 new — packages/escher-driver/src/mcp_tests.rs · 30.0; packages/escher-driver/src/mcp.rs · 16.0; packages/escher-driver/src/mcp/framing.rs · 13.0; packages/escher-driver/src/schema_tests.rs · 13.0; packages/blitz-dom/src/node/range.rs · 10.0; packages/escher-driver/src/mcp/listing.rs · 6.0; packages/escher-driver/src/mcp/result.rs · 6.0
- **churn** — churn 1.5 % over 1 churned file(s) (22.59 % / 29 at the baseline) — informational, no threshold
- **tool added** — cargo-nextest 0.9.146: new in this record — the mutation runner, which was plain `cargo test` at the baseline; a scored unit's counts are read under a different runner
- **mutation survivors** — 0 (counts.missed summed over scoped units).
- **not measured on this host (x86_64-unknown-linux-gnu)** — 0.
  - Project union verdict: none registered — test-plan §10 names cargo-mutants' host-run tier and no per-leg union verdict.

## Below threshold — no action
- new-cycle — graph.cycles 0 → 0
- duplication-up — duplication.pct 3.686 → 3.679 (-0.007 pt; fires at ≥ +0.5 pt AND ≥ +15 %)
- complexity-creep — complexity.over_ceiling 83 → 82 (-1; fires at ≥ +3 AND ≥ +25 %)
- dead-growth — dead.zero_ref_candidates 119 → 119 (+0; fires at ≥ +5)
- coverage-drop — coverage.line 67.07 → 66.37 (-0.70 pt; fires at ≤ −2 pt)
- mutation-drop — blitz-dom/range-keyboard: 100.0 → 100.0 at equal scope files [packages/blitz-dom/src/events/keyboard.rs, packages/blitz-dom/src/node/range.rs] (Epoch 6 — Polish & ship → Epoch 1 — Foundation); fires at ≤ −10 pt
- monotonic — duplication.pct: 3.646 → 3.686 → 3.679 (Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship → Epoch 1 — Foundation)
- monotonic — complexity.over_ceiling: 80 → 83 → 82 (Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship → Epoch 1 — Foundation)
- monotonic — dead.zero_ref_candidates: 122 → 119 → 119 (Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship → Epoch 1 — Foundation)
- monotonic — sizes.file_max: 2756 → 2793 → 2573 (Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship → Epoch 1 — Foundation)
- monotonic — sizes.over_800: 15 → 16 → 14 (Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship → Epoch 1 — Foundation)
- monotonic — coverage.line: 65.08 → 67.07 → 66.37 (Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship → Epoch 1 — Foundation)
- count-under-ratio — duplication.clones 371 → 371 (no rise; pct 3.686 → 3.679) — not applicable
- count-under-ratio — duplication.duplicated_lines 4167 → 4167 (no rise; pct 3.686 → 3.679) — not applicable
- top-N entrants — duplication.top (pair + fragment lines): no new entrant
- top-N entrants — graph.fan_in_top (symbol): no new entrant
- top-N entrants — dead.top (symbol): no new entrant
- sizes — sizes.file_p50 137 → 137
- sizes — sizes.file_p90 605 → 600
- sizes — sizes.file_max 2793 → 2573
- sizes — sizes.over_800 16 → 14
- totals (population) — totals.loc 92380 → 92513 — a population, never a movement
- totals (population) — totals.files 385 → 393 — a population, never a movement
- graph — cross_unit_edges 97 → 97
- dead — unused_deps 26 → 26 (lists equal)

## Skips
- mutation:dioxus-native-dom — declined
- mutation:seven_guis (the unit beyond timer.rs: 239 listed whole) — declined
- mutation:escher-driver (the unit beyond its touched mcp/schema files: 525 listed whole) — declined
- mutation:blitz-dom (its touched document.rs 500 + events/pointer.rs 265 beyond range.rs + keyboard.rs; 5617 whole) — declined

Ledger: 7 parseable records after this run's append.
