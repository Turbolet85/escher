# tests extract

## Relevance
relevant — the merge re-runs every standing proof, adds two upstream test targets to blitz-tests, and its entry names the per-OS tally re-read, a test-plan reading.

## Constraints
- The fork keeps one test target per file in `tests/blitz-tests`. Upstream's `accessibility_description.rs` and `accessibility_label.rs` each land as an auto-discovered target with no manifest line. `tests/all.rs` stays the manifest's one `[[test]]` table at `test = false` and gains only upstream's `mod` lines. `[package]` sets no `autotests = false`. The plan's last recorded count is 124 `.rs` files and 123 test targets, with 0 `Running tests/all.rs` lines read at each run (per test-plan §2 Test directory + naming conventions).
- `BlitzTestsTargetsTest` (`.github/scripts/test_blitz_tests_targets.py`) pins the manifest's shape, not a count: no `autotests = false`, and exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false`. The scope says this pin "moves by two", and that inference does not match what the plan says the test checks. Whether the test or the `ci-scripts` count (100 tests) moves at all is research's question (per test-plan §4 What unit tests cover, blitz-tests targets; test-plan §1 Coverage scope, CI Python scripts).
- The two runners build different engine features, so both must be read green:
  - The workspace build (`cargo test --workspace --locked`, CI's `test` leg) compiles blitz-dom's `tracing` call sites in, and it resolves 16 blitz-dom features against the per-package build's 10, `writing-mode` included.
  - A green `-p blitz-tests` run (what `agent-run.sh run` uses) is therefore not the workspace leg's green (per test-plan §9 Engine features by runner).
- The per-OS tallies are re-read from the six test-bearing job logs of the merge's green CI run. Each delta against the recorded baseline is explained (per test-plan §9 Per-OS test tallies):
  - linux 185 · 951 · 0 · 12
  - macos and coverage 166 · 948 · 0 · 12
  - windows 166 · 926 · 0 · 9
  - ios and android built only
  - The structure the baseline records: linux − macos is the 19 `Doc-tests` lines, and macos − windows is the unix-gated binaries.
- The stand checks have a zero-failure budget. The `stand-repeat` leg runs the 40 `stand_*.rs` files and seven_guis' seven process checks three rounds, and its first red command ends the leg (per test-plan §10 Flakiness budget; test-plan §9 Legs).
- The `coverage` leg ends on three per-crate line floors: `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 % and `packages/blitz-test-harness` 90 %. Upstream's delta changes dioxus-native-dom's line base: it deletes about 550 lines of `convert_events.rs` and adds `keyboard_compat.rs`. Whether that crate still clears its floor is measured on the merged tree, not assumed (per test-plan §10 Coverage floors; test-plan §9 Coverage).
- Pipeline checks run in both layout modes, and the differential oracle compares incremental against non-incremental layout and paint after each step (per test-plan §2 Differential oracle). The sink captures must stay at 0 needles under both builds at `RUST_LOG=trace`: `host_log` and `mcp_log` (the real `escher-session` binary) and `telemetry_drop` (per test-plan §5 Session host ↔ what it writes).

## Patterns to follow
- The last upstream sync's count shape: each new upstream file is its own auto-discovered target with no manifest line, `all.rs` gains the matching `mod` lines and stays unbuilt, and files and targets are counted before and after. That is the shape recorded from `escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/evidence/counts.md` (per test-plan §2 Test directory + naming conventions).
- Prove by name that each upstream file ran with `bash scripts/agent-run.sh run {name}`, and run the whole package with `run all`. Two cautions (per test-plan §3 Agent-run contract; test-plan §9 Legs):
  - `run stand` and the `stand-repeat` leg select by the `stand_` prefix, so neither picks up upstream's two `accessibility_*` files.
  - An empty run is never a pass.
- Re-prove the resolved `accessibility.rs` conflict with the standing tables:
  - `accessibility_states` (4 tests): `aria-selected` reads `true` and `false` only (9 rows), `aria-pressed` reads its three values only (10 rows), `aria-invalid` reads its values and an unrecognized value as true (11 rows), and the row counts are asserted.
  - `accessibility_names`: names from `aria-label` and from `<label>` association.
  - These two run under the workspace `test` leg and per package. They are not in the `a11y` leg (per test-plan §1 Coverage scope, tests/blitz-tests).
- The keyboard-types 0.8 migration in the stand checks names keys through the harness. `blitz-test-harness` re-exports `keyboard_types::Key` and `keyboard_types::Modifiers`, the two types `press_with` takes, so a dependent crate needs no keyboard-types dependency of its own (per test-plan §3 blitz-test-harness (`Harness`), Core).
- Upstream's new tests follow the direct-construction integration style: `HtmlDocument` plus `DocumentConfig`, alongside the `Harness` wrapper style (per test-plan §2 Test levels observed, Integration tests (tests/blitz-tests)).

## Anti-patterns to avoid
- No retry, no `--ignored` and no env var to turn a stand round green; the first red command ends the leg (per test-plan §9 Legs, stand-repeat).
- Never count a sink capture's lines in total. Count by target: the workspace build adds engine lines such as `blitz_dom::document` that the per-package build does not (per test-plan §9 Engine features by runner).
- Never run a bare `--ignored` over blitz-tests to read ignored tests. It waits on the `#[ignore]` session-host children until their idle expiry (per test-plan §2 Test levels observed, Process-lifecycle checks).

## Contract bindings
- tests ↔ obs: the "sink allowlist re-checked" reading rests on test-plan's standing captures, `host_log`, `mcp_log` and `telemetry_drop` (per test-plan §5 Session host ↔ what it writes). Their record classes and line format are set by test-plan §3 Stand log format, which binds to obs-plan §3's allowlist.
- tests ↔ security: the `audit` leg (`cargo deny --locked --workspace --all-features check advisories licenses`) runs in the same ci.yml as the test legs (per test-plan §9 Legs).
- tests ↔ a11y: the `a11y` leg runs six named files: `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` (per test-plan §9 Legs). They re-prove focus order, Enter and Space activation, and accessibility-tree identity after the key migration. The plan does not say whether upstream's two new `accessibility_*` files join that leg.
- tests ↔ session lifecycle: the process checks (`stand_session_lifecycle`, `host_binary`, `cli_commands`, `mcp_server`) fall under test-plan §3 Session lifecycle and must stay green unchanged. The chunk changes no part of that contract.

## Acceptance criteria contributions
- (tests) Every `.rs` file under `tests/blitz-tests/tests/` other than `all.rs` runs as its own target. Files and targets each move from the recorded 124 and 123 by exactly upstream's additions, the run shows 0 `Running tests/all.rs` lines, and `accessibility_description` and `accessibility_label` appear by name with their tests passing (per test-plan §2 Test directory + naming conventions).
- (tests) On the merged tree these pass, each read as the legs run them: `bash .github/scripts/ci-leg.sh fast` (whose `test` leg is the workspace build), `doc`, `audit`, `a11y` and `stand-repeat` (3 of 3 rounds green), plus `cargo test -p blitz-tests --locked` for the per-package build (per test-plan §9 Legs; test-plan §9 Engine features by runner).
- (tests) The six test-bearing job logs of the merge's green fork CI run are re-read into per-OS tallies, and every count that moved against linux 185 · 951 · 0 · 12, macos and coverage 166 · 948 · 0 · 12 and windows 166 · 926 · 0 · 9 is explained by name (per test-plan §9 Per-OS test tallies).
- (tests) The `coverage` leg on the merged tree ends `coverage floor: 3 of 3 crates at or above their floors` (per test-plan §10 Coverage floors).
