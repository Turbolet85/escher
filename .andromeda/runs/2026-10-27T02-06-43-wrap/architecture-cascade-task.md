# Task — architecture · 2026-10-26-codebase-health

## The session
The chunk (v030-01) factored three functions under the cognitive-complexity ceiling of 15, private items only: escher-driver's lint findings walk (`findings` now calls new private `readings` and `placement` per entry), the MCP listing test (three new private `#[track_caller]` helpers `assert_described`, `assert_schema`, `assert_property`) and the 0.2.0 requirement-sweep test (new private `assert_0_2_0_row`, `assert_0_2_0_witness` and the predicate `inside_stand_repeat`, which now holds the `BLITZ_TESTS` and `SEVEN_GUIS` path-prefix constants).
No public symbol, signature, verb, cause, field, carrier, socket, port, env var, crate, module, dependency, CI step or leg changed; nothing under `packages/blitz-dom/` changed; every message and test name kept.
Measured: the audit's population over complexity 15 moved 88 → 85 (the three rows gone: `findings` 16 → 7, the listing test 18 → 3, the sweep test 18 → 3; each new helper 1–4; `clear_interaction_state_for_removed_node` 17.0 and `get_text_selection_ranges` 32.0 standing), read by `rust-code-analysis-cli` 0.0.25 on the dev host, no CI leg.
Measured: escher-driver coverage 4918/5019 → 4930/5031 = 97.99 % (floor 97 % holding); every other count re-read unchanged; CI#40434241601 green 17/17 on `3c04042ed`, its job tallies equal the base run's.
Disproved by measurement: the sweep test's leg check reads `ci-leg.sh` whole by `contains`, so a target dropped from `stand-repeat` alone while the `a11y` leg still names it leaves the test green; stand-repeat membership is held jointly with `test_stand_repeat_leg_runs_three_rounds` in `.github/scripts/test_ci_workflows.py`. The property predates the chunk; its owner is placed at P5.
Line citations into the three edited files moved (the lint's inline tests `:526-604` → `:542-620`, the MCP tests' `env!("CARGO_MANIFEST_DIR")` `:633` → `:650`, the sweep test `:1015-1090` → `:1015-1109`).

## Reached files
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/mcp_tests.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- escher-0.3.0/chunks/2026-10-26-codebase-health/scope.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/research.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/plan.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/inputs/
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/complexity.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/counts.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/row-controls.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/operator-pass.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- .andromeda/runs/2026-10-27T01-14-55-implement/

## Touches
- existing-scopes (line 115 region): the citation `tests/blitz-tests/tests/stand_requirement_sweep.rs:1015-1090` (the test to the file's end) now `:1015-1109`, the test plus its three new private helpers; its other spans `:231-233` · `:542-837` · `:839-863` unmoved.
- existing-scopes (line 115 region): its claim that every other 0.2.0 witness is found inside `stand-repeat` by the `stand_` prefix or by `ci-leg.sh`'s named `--test` list is contradicted — the leg check reads `ci-leg.sh` whole by `contains`, so a target named by the `a11y` leg (`ci-leg.sh:39`) and dropped from `stand-repeat` alone (`ci-leg.sh:57`) still reads found; membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`.
- existing-scopes (line 114 region): the sweep-file spans `:92-218` · `:245-1013` re-read, unmoved (above the first hunk at `:1017`).
- contract-ci-contracts (standard-contracts lines 271–280, line 273): the claim that the stand-repeat list's names are read from the script's own text by the sweep test's leg check alone is contradicted the same way — stand-repeat membership is the joint hold of that leg check and the CI-scripts test `test_stand_repeat_leg_runs_three_rounds`.
- contract-driver-session (standard-contracts lines 185–190, line 188): its `packages/escher-driver/src/lint.rs` spans `:190-230` · `:176-187` · `:95-139` · `:141-174` · `:16-27` re-read, unmoved (all above the first hunk at `:273`); `pub fn lint` stays at `:221`, `Session::lint` and every reader untouched.
- contract-driver-session (line 196): its citations are `tests/blitz-tests/tests/stand_lint.rs` spans only — that file was not edited, so they are unmoved.
- contract-driver-session: `Session::lint`'s findings, their document order then `Defect::ALL` order, and its `fonts-unavailable` refusal re-read unchanged (`stand_lint` 7 green under both builds; boot `[0, 1, 0, 0]`; the twelfth-Create `clipped` row) — the per-entry push order inside the walk is private and kept.
- contract-driver-session: the lint's `ordered` moved `:299` → `:315`, `overlaps` `:334` → `:350`, the inline `mod tests` `:527` → `:543` (file 604 → 620 lines), bodies unchanged — any architecture citation to those lines moves.
- contract-nodes-and-widgets (standard-contracts line 76 region): its `readings` is blitz-dom `node/select.rs`'s reader, a different function from the new driver-private helper of the same name — re-read, unmoved.
- standard-contracts line 150 (contract-driver-session group 143–179): a `factored`-pattern hit, re-read, no change.
- MCP listing contract: the listing test still asserts the twelve tools in `VERBS` order with every description, schema and property check; `mcp_tests.rs` 16 `#[test]` (15 on unix); its every line after base `:480` moved +17 (file 786 → 803).
- occupied-resources res-filesystem / res-environment-variables (lines 30 · 32): the `CARGO_MANIFEST_DIR` readers cited there re-read, unmoved; the MCP tests' own `env!("CARGO_MANIFEST_DIR")` moved `:633` → `:650`, `env!("CARGO_PKG_VERSION")` `:247` unmoved, `env!` count 2.
- contract-wpt-runner-outputs (standard-contracts line 264): its `CARGO_MANIFEST_DIR` reader is another one, unmoved.
- conv-tests: unit tests still at the end of the source file in `#[cfg(test)] mod tests` or a `#[path]` child file; the lint's inline module moved, the MCP tests stay a `#[path]` child; blitz-tests still 147 `.rs` files and 146 targets, one target per file.
- conv-feature-gating: escher-driver still builds its spans by hand — `info_span!` 2, no `#[instrument]`, no event added; the silence census over the three edited files reads 0, the MCP test file's first reading in a census.
- Counts re-read unchanged: escher-driver 100 unit tests and seventeen private modules; `stand_requirement_sweep` 3 tests; `stand_lint` 7; stand checks 205 · 0 · 7; a11y leg 14 · 85 · 0 · 0; stand-repeat 204 · 864 · 0 · 21; workspace 208 · 1117 · 0 · 12; CI scripts 108.
- contract-ci-contracts / inherited-defaults code quality: `coverage_floor.py`'s per-crate floors hold — escher-driver 4930/5031 = 97.99 % against 97 %, dioxus-native-dom 85.60 % and blitz-test-harness 94.44 % unchanged; any architecture text carrying escher-driver's coverage reading moves from 4918/5019.
- inherited-defaults code quality / conventions formatting and lints: the gate set (fmt, clippy, rustdoc, cargo-deny, coverage floors, stand-repeat budget) unchanged; a cognitive-complexity ceiling of 15 now stands as a measured dev-host reading beside them (see new facts).
- stack-and-technologies: a dev-host analyser, `rust-code-analysis-cli` 0.0.25, carries the complexity reading; no lockfile resolves it, no manifest or `Cargo.lock` change.
- crosscut-invalidation-and-state-integrity: `clear_interaction_state_for_removed_node` reads complexity 17.0, over the ceiling and deliberately left untouched (the engine function out of the chunk's scope).
- established-decisions dec-driver-session: no carrier, exit, verb, cause or field added — the boundary rule re-read, unmoved.

## New facts
- F1 · conventions · Codebase health holds a cognitive-complexity ceiling of 15 per function, test functions counted, read on the dev host by `rust-code-analysis-cli` 0.0.25 (no CI leg); 85 functions read over it at `3c04042ed`, the engine's `clear_interaction_state_for_removed_node` (17.0) and blitz-dom's `get_text_selection_ranges` (32.0) among them, left untouched.
- F2 · conventions · A table-asserting test keeps its outer assertions and row walk in the test function and moves each row's checks into private `#[track_caller]` helpers whose messages name the row and index only, so the test stays under the complexity ceiling with its name, target and messages unchanged.
- F3 · conventions · rust-code-analysis's `cognitive.sum` aggregates nested spaces, so a closure or nested `fn` keeps its branches counted in its parent; only a module-level sibling item lowers a function's reading.
- F4 · stack-and-technologies · `rust-code-analysis-cli` 0.0.25 is a dev-host tool in `~/.cargo/bin` (installed 2026-10-03), resolved by no lockfile and run by no CI leg.
- F5 · standard-contracts · The 0.2.0 sweep test's leg check reads `ci-leg.sh` whole by `contains`, so it cannot see a target dropped from `stand-repeat` alone while another leg names it; stand-repeat membership is held jointly by it and `test_stand_repeat_leg_runs_three_rounds` in `.github/scripts/test_ci_workflows.py`, a reach gap predating 0.3.0 whose owner is placed at this wrap's P5.
- F6 · existing-scopes · The sweep test's target predicate is a private helper of its own, `inside_stand_repeat`, holding the two path-prefix constants `BLITZ_TESTS` and `SEVEN_GUIS`.

## Untouched
- blitz-dom's engine contracts and decisions — the select model and its crate-private rules, the range input, keyboard activation, the clock, hit testing, scrolling, `text_extent`, `visible_region`, the accessibility builder: no file under `packages/blitz-dom/` changed.
- The driver's schema, wire and carriers — twelve verbs, eight causes, thirteen node keys, the `help` self-description, the `v3` socket wire, the command line's exits, the MCP server's revisions and bounds: no table, carrier or public item changed.
- The `command` and `tool_call` spans' fields and escher-telemetry's sink and allowlist: no span, field or event added.
- The session recording, kept checks, `Check::replay` and the two committed Counter checks under `tests/blitz-tests/checks/`: byte-unchanged.
- Occupied resources — ports, the one Unix-domain socket listener, outbound hosts, filesystem areas, env vars, names, CI infrastructure: nothing added or retired.
- Dependency pinning, inherited framework versions, `deny.toml` and the stack's crate versions: no manifest, lock or deny change.
- The CI workflow, the `ci-leg.sh` legs and their membership lists, `apt-install.sh` and the coverage floors' values: no step or leg edited.
- The seven_guis stand — its six served tasks, boots, `LoadNet`, the lint at boot reading `[0, 1, 0, 0]`: unchanged.
- The Dioxus DOM bridge — stable ids, snapshot, its text form, parts and diff, `unkeyed_actionable`, `is_masked_input`: unchanged.
- Project intent, the upstream-sync merge base, the front page and `docs/capabilities.md`: unchanged.
- Infrastructure patterns' directory tree and build system: no crate, directory or test target added.
- Design philosophy: unchanged.
