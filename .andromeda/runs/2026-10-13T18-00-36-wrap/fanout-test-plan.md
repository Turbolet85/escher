proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-test-harness"
    change: "Append to the unit-test count: 22 since 2026-10-13-stand-contrast-harness, 5 more in `contrast.rs` — the WCAG 2.x ratio table (5 rows), the large-text threshold table (5 rows), a button reading its own text and background, a translucent ancestor composited over the page, an element with no text reading `None` (packages/blitz-test-harness/src/contrast.rs:200-308) — as measured at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md."
    sidecar: "§1 blitz-test-harness: unit tests 17 → 22, the five `contrast.rs` tests of 2026-10-13-stand-contrast-harness."
    rationale: "The report's Counts bullet states blitz-test-harness unit tests 17 → 22 (gate `cargo test -p blitz-test-harness --locked`, 22 passed), and Coverage of new surfaces names the five `contrast.rs` tests (ratio table 5 rows, threshold table 5 rows, three fixture tests; Deviations, Step 5). The §1 bullet still ends at «17 since 2026-10-12-headless-screenshot»."
    basis: "packages/blitz-test-harness/src/contrast.rs:200-308"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "Add the new stand check: since 2026-10-13-stand-contrast-harness `stand_contrast` 2 — every stand control reads its recorded text/background pair (18 rows, one per `controls(task)` row over the four lean tasks at boot, through `Harness::text_contrast` from computed styles; verdicts pass 9 · fail 5 · exempt-inactive 3 · no-text 1, exempt exactly when the snapshot reads `enabled == Some(false)`; the record identical in both layout modes) and the painted background is the measured background (the pixel inside the left padding at the box's vertical middle within 1 per channel on all 17 measured rows, both modes) — run by the workspace `test` leg and per package, not by the `a11y` leg, which stays three files; it gates nothing — SC 1.4.3 assertion is owned by the route entry «Stand a11y assertions» (tests/blitz-tests/tests/stand_contrast.rs:41-129; tests/blitz-tests/tests/stand_contrast.rs:194-225; tests/blitz-tests/tests/stand_contrast.rs:227-277) — as measured at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md."
    sidecar: "§1 tests/blitz-tests: `stand_contrast` (2 tests, 18-row record, painted witness) added by 2026-10-13-stand-contrast-harness."
    rationale: "Changes → Files lists the new target `tests/blitz-tests/tests/stand_contrast.rs`; Harness / gate surface names it a new stand check target outside the `a11y` leg; the measured record and the Outcome's acceptance rows state its two tests and what they assert. §1's tests/blitz-tests bullet names no contrast check."
    basis: "tests/blitz-tests/tests/stand_contrast.rs:194-225"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "`tests/common/mod.rs`, read through `mod common;` by 18 `stand_*.rs` checks (17 until 2026-10-13-stand-contrast-harness added `stand_contrast.rs`)."
    sidecar: "§2 Directory pattern: `mod common;` readers 17 → 18 (`stand_contrast`)."
    rationale: "Report Counts / qualifiers moved: `mod common;` readers 17 → 18, basis `grep -l 'mod common;' tests/blitz-tests/tests/*.rs | wc -l` reading 18 (evidence/counts.md); it names test-plan.md:66 as stating 17."
    basis: "test-plan.md:66"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)"
    change: "Add `stand_contrast` (since 2026-10-13-stand-contrast-harness) to the named readers of `mod common;` and make the total «18 in all»."
    sidecar: "§3 stand-check helpers: `stand_contrast` named as a `mod common;` reader, 17 → 18 in all."
    rationale: "Same retired claim as the §2 Directory pattern (17 readers): the report states test-plan.md:100 lists the readers by name, «… do, 17 in all», and `stand_contrast` is not in the list."
    basis: "test-plan.md:100"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "The tests/ count moves to 119 `.rs` files, 118 test targets and the unbuilt `all.rs` (119 and 118 since 2026-10-13-stand-contrast-harness added `stand_contrast.rs`, its own auto-discovered target with no manifest line; 118 and 117 since 2026-10-13-upstream-sync-polish-and-ship …)."
    sidecar: "§2 Directory pattern: blitz-tests files/targets 118/117 → 119/118 (`stand_contrast.rs`, auto-discovered)."
    rationale: "Changes → Files adds the new file tests/blitz-tests/tests/stand_contrast.rs; Decisions & corrections states tests/blitz-tests/Cargo.toml stays untouched, so the file is auto-discovered with no manifest line; Harness / gate surface calls it a new stand check target. The report does not state the total — the +1 is derived from the new file; the orchestrator may re-read the count."
    basis: "test-plan.md:66"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: "Append: re-counted at 2026-10-13-stand-contrast-harness, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 141 · failed 0 · ignored 7 over 37 files — +2 passed, `stand_contrast`, a new file picked up by its `stand_` prefix with no script change — and `run all` was not re-read — as measured at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md."
    sidecar: "§3 agent-run Proof: `run stand` 139 · 0 · 7 over 36 files → 141 · 0 · 7 over 37 (`stand_contrast`)."
    rationale: "Report Counts: `run stand` 36 files → 37; passed 139 → 141; failed 0; ignored 7 (gate `bash scripts/agent-run.sh run stand`, `run.end` ts 2026-10-13T17:34:01Z); Harness / gate surface: picked up with no script change. The Proof bullet's last re-count is 139 · 0 · 7 over 36 files."
    basis: "test-plan.md:116"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline → `cargo test --workspace` row"
    change: "Append: re-counted at 2026-10-13-stand-contrast-harness: 180 result lines, 900 passed · 0 failed · 12 ignored (+7 passed: one new result line, blitz-tests' `stand_contrast` 2, and blitz-test-harness +5 in its lib line, `contrast.rs`, to 22), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`, 2026-10-13T17:35:53Z; escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md)."
    sidecar: "§9 Local baseline: workspace test leg 179 · 893 · 0 · 12 → 180 · 900 · 0 · 12."
    rationale: "Report Counts: workspace test leg 179 → 180 result lines; 893 → 900 passed; failed 0; ignored 12, read from that run's `fast` leg; the +7 are the five `contrast` unit tests and the two `stand_contrast` tests. The report names test-plan.md:330 as stating 179 · 893."
    basis: "test-plan.md:330"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → blitz-test-harness"
    change: "Append: since 2026-10-13-stand-contrast-harness `contrast.rs` pins the contrast reader — `contrast_ratio` as the WCAG 2.x ratio over a stated table of 5 rows (21.0 both ways, 1.0, 4.48 and 4.39 within 0.005), `required_ratio` over a stated table of 5 rows (3.0 for large text — at least 24 px, or 14 pt at weight 700 — else 4.5), each row count asserted, and three fixture tests: a button reads its own text and background, a translucent ancestor is composited over the page, an element with no text reads `None` (packages/blitz-test-harness/src/contrast.rs:200-308)."
    sidecar: "§4 blitz-test-harness: the five `contrast.rs` unit tests described."
    rationale: "Report Symbols / APIs and Coverage of new surfaces (tests unit: ratio table 5 rows, threshold table 5 rows), Deviations Step 5 (two stated tables with row counts asserted plus three fixture tests), Outcome (unit table reads 21.0 both ways, 1.0, 4.48 and 4.39 within 0.005). §4's blitz-test-harness bullet covers settle.rs and screenshot.rs only."
    basis: "packages/blitz-test-harness/src/contrast.rs:200-308"
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy → Test levels observed → Harness-driven tests"
    change: "Add: and since 2026-10-13-stand-contrast-harness a contrast reader — `Harness::text_contrast(node_id) -> Option<TextContrast>`, reading and changing nothing (no mutate, input, pump or settle; every node by `get_node`): the text-bearing element's computed `color`, font size and weight over the background composited source-over from its ancestors' `background-color`s onto the root-else-body canvas over opaque white, `None` with no text or where an element on the walk has `opacity` below 1, a filter or a background image; `contrast_ratio` the WCAG 2.x ratio over the `color` crate's relative luminance (packages/blitz-test-harness/src/contrast.rs:13-23; packages/blitz-test-harness/src/contrast.rs:56-91)."
    sidecar: "§2 Harness-driven tests: `Harness::text_contrast` / `contrast_ratio` added to the harness's surface."
    rationale: "Report Symbols / APIs: the new public reader `Harness::text_contrast` and `contrast_ratio` / `TextContrast`, re-exported at the crate root (lib.rs:28); Harness / gate surface: «A new harness reader, `Harness::text_contrast`». The Expected amendments list test-plan.md:47 (pattern `arch-harness`) as the master's own harness-surface entry the reader joins."
    basis: "test-plan.md:47"
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (`Harness`) → Core"
    change: "Add: since 2026-10-13-stand-contrast-harness `text_contrast(&self, NodeId) -> Option<TextContrast>` in the private module `contrast`, the crate's sixth, with `TextContrast { text, background, ratio, font_size_px, font_weight }` (`is_large_text`, `required_ratio` 3.0 / 4.5) and `contrast_ratio` re-exported at the root; it reads computed styles only, takes the crate's one new dependency `color` (workspace 0.3, no package entering the lock), opens no span and prints or logs nothing (packages/blitz-test-harness/src/contrast.rs:25-54; packages/blitz-test-harness/src/contrast.rs:56-91)."
    sidecar: "§3 Harness Core: `text_contrast` and the `contrast` module (sixth) listed."
    rationale: "Same retired claim as the §2 Harness-driven tests entry — the harness's listed surface omitting the contrast reader; report Symbols / APIs, Crates / modules (private module `contrast`, its sixth, lib.rs:21), Dependencies (`color = { workspace = true }`, Cargo.toml:27; no `[[package]]` enters) and obs-plan §4 entry (census 0, no `tracing`)."
    basis: "packages/blitz-test-harness/src/contrast.rs:56-91"
    dependent-of: D-tests-framework
note: "D-tests-obs-harness holds — no agent-run verb, status shape or log format changed (report Harness / gate surface). The report attributes `139 · 0 · 7` to test-plan.md:330; in the body that re-count sits in §3's agent-run Proof at test-plan.md:116 (a line the report states as an obs-census row), while :330 holds the 179 · 893 workspace row — the proposals target each where it stands."
