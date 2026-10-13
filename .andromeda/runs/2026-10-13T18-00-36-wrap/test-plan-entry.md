
## 2026-10-13-stand-contrast-harness — the contrast reader, `stand_contrast` and the counts it moved
**Section:** §1 Test Scope Summary (blitz-test-harness · tests/blitz-tests) · §2 Test Strategy (Harness-driven tests · Directory pattern) · §3 Test Harness Contract (Harness Core · Crate-local stand-check helpers · Agent-run Proof) · §4 Unit Test Strategy (blitz-test-harness) · §9 CI Integration (Local baseline)
**Change:**
- §1 and §4 blitz-test-harness: unit tests 17 → 22, five in `contrast.rs` — the WCAG 2.x ratio table (5 rows: 21.0 both ways, 1.0, 4.48, 4.39 within 0.005), the threshold table (5 rows: 3.0 for large text — 24 px, or 14 pt at weight 700 — else 4.5), each row count asserted, and three fixtures (a button's own pair, a translucent ancestor composited, no text reads `None`).
- §1 tests/blitz-tests: `stand_contrast` 2 — 18 `controls(task)` rows at boot read through `Harness::text_contrast` (pass 9 · fail 5 · exempt-inactive 3 · no-text 1, exempt exactly where `enabled == Some(false)`, identical in both modes) and the painted background within 1 per channel on the 17 measured rows; run by the workspace `test` leg and per package, not the `a11y` leg (three files); it gates nothing — SC 1.4.3's assertion is "Stand a11y assertions"'s.
- §2 Harness-driven tests and §3 Harness Core: `text_contrast` in the crate's sixth module `contrast`, `TextContrast` and `contrast_ratio` re-exported; read-only, computed styles only, `color` its one new dependency, no span, no print or log.
- §2 Directory pattern: `mod common;` readers 17 → 18; 118 → 119 `.rs` files and 117 → 118 targets (the new file auto-discovered, no manifest line). §3 stand-check helpers: `stand_contrast` named, 17 → 18 in all.
- §3 Agent-run Proof: `run stand` 139 · 0 · 7 over 36 files → 141 · 0 · 7 over 37, no script change; `run all` not re-read.
- §9 Local baseline: 179 · 893 · 0 · 12 → 180 result lines · 900 passed · 0 failed · 12 ignored (+7: `stand_contrast` 2 in one new line, the harness +5 in its lib line).
**Why:** every figure read at this chunk's gate runs, the old ones kept as dated history; the file and target count read on the tree at this wrap.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/
