# Code Audit — escher · Epoch 6 — Polish & ship · 2026-11-19T06:14:57Z
mode trend · version escher-0.3.0 · HEAD bc766124059bc619b94019fe200681196553acbc · baseline 0d93539f19d1662658302f59480fdeb334587eff (Epoch 5 — Errors and explanation) · span 1
overshoot: this run's 0 commits past the boundary (0 at it) · the baseline's 0 (at its boundary)
ledger: 15 records after this append (tx 2303) · no tool-version change against the baseline (no trend-break) · ancestry OK
mutation tier: PARTIAL — 3 units read complete (2 scored), escher-driver stopped mid-unit at 89/96, blitz-dom postponed and not measured this boundary, 4 units baseline-test-failure (see Informational and Skips)

## Proposals

### M1 — monotonic · duplication.pct — +0.01pt then +0.08pt
**Movement:** 3.80 → 3.81 → 3.89 (Epoch 4 → Epoch 5 → Epoch 6, escher-0.3.0)
**Evidence:** this boundary clones 527 → 543 (+16), duplicated lines 5815 → 6027 (+212), jscpd's population (`total_lines`)
152784 → 155116 (+1.5 %). By path split: test–test pairs 291 → 307, lines 3344 → 3555 (+211); src–src pairs 223 → 223,
lines 2276 → 2277 (+1); mixed 13 → 13, lines 195 → 195. Two top-10 entrants, both test–test:
`tests/blitz-tests/tests/stand_mcp_masked.rs:30` ↔ `tests/blitz-tests/tests/stand_snapshot_parts.rs:183` (39 lines; the
standing pair re-anchored from :27/:180, 42 lines) and `tests/blitz-tests/tests/stand_act_error_text.rs:804` ↔
`tests/blitz-tests/tests/stand_order_served.rs:252` (29 lines, the epoch's new order-page test). Each step is below the
single-epoch `duplication-up` threshold (+0.5pt AND +15 %); the rule that fires is the two-step direction.
**Suspected shape:** the whole growth is in test files. The epoch added `stand_order_served.rs` (+359),
`stand_masks_on_new_answers.rs` (+269) and `stand_requirement_sweep.rs` (+649), and these repeat the stand's boot-and-act
scaffolding that the older stand tests already carry.
**Proposal:** extract the stand checks' repeated boot / act / answer-read sequences into `tests/blitz-tests/tests/common`
helpers, so new stand tests reuse them instead of copying them. The `stand_act_error_text.rs:804` family, which now pairs
with both `stand_act_errors.rs:348` and `stand_order_served.rs:252`, is the obvious first candidate.

### M2 — monotonic · sizes.file_max — +2 then +26
**Movement:** 3339 → 3341 → 3367 code lines (Epoch 4 → Epoch 5 → Epoch 6), always `packages/blitz-dom/src/document.rs`
**Evidence:** `git diff --numstat 0d93539f1..HEAD -- packages/blitz-dom/src/document.rs` reads +32 / −2. The file is the
largest of 510, ahead of `mutator.rs` (2556), and a hotspot entrant this boundary: 8th, score 32 = commits × its max
cognitive.
**Suspected shape:** `BaseDocument`'s one file keeps taking each epoch's engine edits, a few lines at a time, with no
split behind them.
**Proposal:** split `document.rs` along its existing concerns (e.g. focus, the changed set, select-list state and the
geometry readers) into child modules of `document`. `escher-driver`'s `json.rs` → `json/*.rs` split this epoch is the
in-repo pattern: 1861 → 296 lines.

### M3 — monotonic · sizes.over_800 — +2 then +1
**Movement:** 22 → 24 → 25 files over 800 code lines (Epoch 4 → Epoch 5 → Epoch 6)
**Evidence:** this boundary, entered: `packages/escher-driver/src/json_tests.rs` (new, 1021) and
`packages/blitz-paint/src/render/background.rs` (791 → 807); left: `packages/escher-driver/src/json.rs`
(1861 → 296, the epoch's split). All 25 at HEAD: document.rs 3367 · mutator.rs 2556 · node/node.rs 1742 ·
stand_requirement_sweep.rs 1582 · stylo.rs 1285 · blitz-paint render.rs 1189 · select_popup.rs 1079 · stand_keyboard.rs 1063 ·
events/pointer.rs 1057 · json_tests.rs 1021 · stand_mcp_tools.rs 991 · stand_act_error_text.rs 974 · layout/inline/layout.rs 956 ·
node/element.rs 948 · stylo_taffy convert.rs 917 · blitz-vibey-script tests/dom.rs 914 · dioxus-native-dom snapshot.rs 913 ·
seven_guis tests/cli_commands.rs 868 · layout/text_transform.rs 845 · layout/construct.rs 839 · cssom.rs 829 ·
select_interaction.rs 828 · escher-driver command.rs 826 · stylo_taffy wrapper.rs 819 · render/background.rs 807.
**Suspected shape:** the json split moved its 1066 test lines into one new file rather than spreading them, so the split
traded one file over 800 for another. Separately, `background.rs` crossed by +16.
**Proposal:** give `json_tests.rs` the same per-answer split the source got (one test child per `json/{answer}.rs`), and
add `background.rs` to the watch list beside `document.rs` (M2).

## Informational
- top-N entrant · sizes.top: `packages/escher-driver/src/json_tests.rs` (1021), 10th; `json.rs` left the top-10 by the split.
- top-N entrant · duplication.top: the two pairs named in M1.
- top-N entrant · graph.fan_in_top: `escher-driver session/Session#` at 161 distinct callers. `blitz-traits shell/ColorScheme#` (159) left the top-20. Every other row moved by +1 to +8, e.g. `escher-driver crate/` 171 → 179 and `Harness#doc.` 207 → 213.
- top-N entrants · hotspots, 8 of 10 new (window 0d93539f1..HEAD, 6 commits): `blitz-paint/src/text.rs` 54 · `blitz-dom/src/layout/construct.rs` 43 · `blitz-paint/src/render.rs` 40 · `blitz-paint/src/render/border.rs` 34 · `wpt/runner/src/main.rs` 34 · `blitz-dom/src/document.rs` 32 · `blitz-dom/src/resolve.rs` 27 · `blitz-paint/src/render/background.rs` 25. The epoch's engine edits concentrated in blitz-paint's render path.
- mutation · browser under the 20-mutant floor: 50.0 (Epoch 4, scope in-diff, denominator 6) → 0.0 (Epoch 6, scope in-diff, denominator 4). Never a drop. All four mutants survive, at one site: `apps/browser/src/capture.rs:86:5`, `replace render_scene -> (u32, u32) with (0, 0)` · `(0, 1)` · `(1, 1)` · `(1, 0)`. So no browser test the form runs tells the four returned sizes apart.
- mutation · blitz-paint 100.0 (Epoch 4, in-diff, denominator 23) → 100.0 (Epoch 6, in-diff, denominator 42): equal scope, no movement; 42 caught, 1 unviable of 43.
- mutation · escher-driver STOPPED mid-unit on the operator's word (overseer1, the founder's word, 2026-11-19 05:42:54Z): `exited-incomplete 89/96`. The before-stop copy reads 88 caught, 1 unviable, 0 missed; no score is recorded, by the letter. The live dir's 5 extra `Failure` rows are builds interrupted by the stop, not results. The resume and its cap (4407 s) are in the record's `commands.mutation:escher-driver`.
- mutation · blitz-dom NOT MEASURED this boundary: postponed on the founder's word («Да блиц дом оставляем на потом», relayed 2026-11-19 05:14Z), not declined. Its in-diff listing is 1161. The confirmed scope is 61: a departure from the letter that excludes the two `FnValue` sites `scrolling.rs:537 scroll_state` (×1000) and `:12 node_scroll_range` (×100), both combinatorial tuple returns. `postponed` is outside the letter's state and reason sets and is recorded as such.
- unscored-run · seven_guis: run of 6 (0.2.0 Epoch 2, 0.2.0 Epoch 5, 0.3.0 Epochs 3, 4, 5, 6, every one `unviable-dominant`); this boundary 13 mutants, 6 caught, 7 unviable (4 E0277, 3 E0599 on `Default::default()` / `Harness::new|from|from_iter` replacements in `stand.rs` and `tasks/order.rs`). Last score 100.0 at 0.2.0 Epoch 1, scope `files`.
- unscored-run · escher-driver: run of 2 (Epoch 5 `declined`, Epoch 6 `exited-incomplete`); last score 100.0 at 0.3.0 Epoch 4, scope in-diff.
- unscored-run · blitz-dom: run of 2 (Epoch 5 `declined`, Epoch 6 `postponed`); last score 100.0 at 0.3.0 Epoch 4, scope in-diff.
- unscored-run · wpt: run of 2 (Epoch 5 `declined`, Epoch 6 `baseline-test-failure`); last score 0.0 at 0.2.0 Epoch 5, scope in-diff.
- unscored-run · dioxus-native: run of 2 (Epoch 5 `declined`, Epoch 6 `baseline-test-failure`); never scored. It holds no test.
- mutation · wpt baseline-test-failure is an ENVIRONMENT fault of this run's form, not a missing test. It has 28 tests, and 3 failed in the unmutated copy: `tempfile::tempdir()` at `wpt/runner/src/main.rs:827` panicked `NotFound` under the relative `TMPDIR=target/mutants-tmp`, resolved from the test's own working directory. The next run's forms should give `TMPDIR` as an absolute path.
- mutation · wgpu_texture, dioxus-native, wasm_hello hold no tests: nextest exits 4 `no tests to run` in the unmutated tree; none tested.
- trend-break, once · rdme is out of the tier by `architecture:mutation-tier-excludes-demo-shells`. Its last listing (0.3.0 Epoch 4) read `baseline-test-failure`; it is touched this epoch (`apps/readme/src/main.rs`) and is recorded `declined` by the row.

## Below threshold — no action
- duplication.pct 3.81 → 3.89 (+0.08pt, +2.1 %; `duplication-up` needs +0.5pt AND +15 %). It fires as M1's monotonic.
- complexity.over_ceiling 89 → 88 (−1); p50/p90 unchanged (cyclomatic 1/5, cognitive 0/3); max unchanged (`synthesize_presentational_hints_for_legacy_attributes`, 186); no complexity top-10 entrant.
- dead.zero_ref_candidates 113 → 114 (+1; `dead-growth` needs +5); top-10 unchanged; unused deps unchanged (26).
- coverage.line 74.4 → 73.9 (−0.5pt; `coverage-drop` needs −2pts); not monotonic (73.1 → 74.4 → 73.9).
- graph.cycles 0 → 0; cross_unit_edges 99 → 99; fan_out per unit unchanged.
- sizes.file_p50 153 → 153; file_p90 612 → 606.
- churn 29.6 % → 0.6 % (26 churned adds of 4283; 2 files touched by 2+ commits, 6 commits).
- count-under-ratio: not applicable, the ratio itself rose (M1).
- monotonic, not fired: complexity.over_ceiling (86 → 89 → 88), dead.zero_ref_candidates (113 → 113 → 114), coverage.line (73.1 → 74.4 → 73.9). All six tracked scalars were evaluable this run.

## Skips
- mutation:seven_guis — unviable-dominant ({"mutants": 13, "caught": 6, "missed": 0, "not_measured": 0, "timeout": 0, "unviable": 7})
- mutation:wpt — baseline-test-failure (3 of 28 tests red under the relative TMPDIR; see Informational)
- mutation:wgpu_texture — baseline-test-failure (no tests)
- mutation:dioxus-native — baseline-test-failure (no tests)
- mutation:wasm_hello — baseline-test-failure (no tests)
- mutation:blitz-dom — postponed (outside the letter's reason set; not measured this boundary; 1161 listed, 61 the confirmed cut)
- mutation:escher-driver — no skip entry: `exited-incomplete 89/96`, stopped mid-unit, resume in the record
- mutation:rdme — declined, out of the tier by `architecture:mutation-tier-excludes-demo-shells`
- mutation:blitz-examples · blitz-tests · counter · todomvc · transparent — diff-empty (touched files with no mutable expression in their diff, manifests or tests only)

## Left in place
processes under the mutation tier's scratch dir (target/mutants-tmp, read at the exit self-check; the dir itself is gone):
none under target/mutants-tmp — processes under the scratch dirs: 0
