@history_item n=1 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — (a shared stand-check module, an out-of-line unit-test module, the id walk's helpers) The first audit-driven split: the id walk and three stand checks were brought under the complexity ceiling by private helpers, `bridge_tests` moved to a `#[path]` child file, and the stand tables and helpers stated once in `tests/blitz-tests/tests/common/mod.rs`, with no behaviour change. This is the precedent and the origin of decision/shared-test-modules, decision/split-test-module-as-child and decision/unit-tests-at-file-end. It also records the rule that a table typed by dev-dependencies is shared through a subdirectory module, because the test crate's `[lib]` cannot see dev-dependencies.

@history_item n=2 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (blitz-tests keeps one test target per file) This entry set decision/blitz-tests-target-per-file and decision/blitz-tests-single-test-target: one target per file directly under `tests/`, upstream's `tests/all.rs` held at `test = false`, pinned by `BlitzTestsTargetsTest` (`.github/scripts/test_blitz_tests_targets.py`). It also records why: `mod common;` does not resolve from a module, and `--exact` re-exec children would match nothing. Any split of a test file in this chunk must stay inside that layout, which is why the scope defers the seven oversize test targets.

@history_item n=3 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — (the range and button arms as private named functions) This was a complexity split by extracting private named functions with signatures and behaviour kept. The upstream-owned parents were deliberately left above the ceiling "at their upstream floor" so that later sync diffs stay out of arms escher does not own. It is the precedent for this chunk's class 2: upstream-authored functions over 15 are left as they are.

@history_item n=4 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — (five out-of-line test modules; mcp's and schema's child modules) This was the file-split precedent in escher-driver. Tests moved whole into `#[path]` children (`mcp_tests.rs`, `schema_tests.rs`) with test paths unchanged. `mcp` gained private children (framing, listing, result) widened to `pub(super)` and no further, and `schema`'s shape types went into a private child `spec`, re-exported. The private module count, the public items and the behaviour were all unchanged. This is the template for splitting `packages/escher-driver/src/command.rs` and `packages/escher-driver/src/json_tests.rs` (decision/moved-unit-tests-keep-paths, decision/restricted-crate-internals).

@history_item n=5 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — (eight `#[path]` test children) This added escher-driver's `cli::tests` (`cli_tests.rs`) and the recording test children, each moved whole with test paths unchanged, plus seven_guis' shared log-check readers. `cli.rs`, whose `read` this chunk must extract, already holds its tests out of line. A further split must keep that child shape and the unix/off-unix `cfg` gates on the recording children.

@history_item n=6 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — (the third blitz-tests shared module) This added `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`, beside `common` and `session_common`. It records the rule that a page with one reader stays in its check's file. Helpers extracted from the stand checks in scope (`stand_graph`, `stand_actions`, `stand_accessibility_ids`) belong in one of these three non-target modules (decision/stand-check-shared-helpers).

@history_item n=7 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the two table-asserting tests' row helpers; the sweep test's target predicate) This was the previous codebase-health chunk. It brought the MCP listing test and the requirement-sweep test under the ceiling by keeping the outer assertions and row walk in the test function and asserting each row through private `#[track_caller]` helpers whose messages name the row and index only. That is the origin of decision/track-caller-row-helpers and the shape to reuse for the five over-ceiling stand and probe test functions. No public symbol, test name, target or message changed.

@history_item n=8 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (`stand-repeat`'s membership held jointly) It measured that `stand_requirement_sweep`'s leg check reads `ci-leg.sh` whole by `contains`. So `stand-repeat`'s membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`. This is a trap for any test-target move: a target dropped from `stand-repeat` alone can still read as inside it.

@history_item n=9 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the dev-host cognitive-complexity reading) It recorded `rust-code-analysis-cli` 0.0.25 as the dev-host-only reading against a ceiling of 15, never in CI. Its Kept clause matters: the ceiling was stated as no project-wide convention, and over-ceiling functions, upstream's among them, stood untouched. The rows migration later landed decision/cognitive-complexity-ceiling. This is the bar this chunk applies and the reason the upstream functions go to the founder rather than being split.

@history_item n=10 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — (the quality gates: legs, the new job) This set the `coverage` leg's `coverage_floor.py` floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the `stand-repeat` zero-failure, three-round, no-retry budget (decision/per-crate-coverage-floors, decision/stand-repeat-zero-failure-budget). Moves and extractions in `escher-driver` and `dioxus-native-dom` must keep both crates at their floors, and every stand check must stay green three rounds running.

@history_item n=11 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the coverage exclusion) Both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs` by `--ignore-filename-regex 'keyboard_compat\.rs$'`, with the floors unchanged (decision/coverage-excludes-keyboard-compat). The exclusion is filename-based, so a split of `snapshot.rs` must not produce a file that matches it or that drops dioxus-native-dom's measured coverage.

@history_item n=12 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — (a real rustdoc gate) The `doc` leg runs `cargo doc --workspace --no-deps --locked` under `-D warnings` over every library crate (decision/rustdoc-warnings-are-errors, decision/rustdoc-warnings-gate). Items moved into child modules keep their intra-doc links resolvable, or the leg goes red.

@history_item n=13 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — (the driver's command and refusal schema) This created the private `command`, `refusal` and `schema` modules and the one verb table, with `validate` answering in a fixed order (verb, the call's arguments as passed, then the schema's arguments). It also recorded that `Command`, `Call` and `ArgValue` print ids and typed text under `Debug`, so nothing fields them. A split of `packages/escher-driver/src/command.rs` (826 lines) must keep the validation order, the single table and the no-fielding rule (contract/validation-order/first-failure, decision/validated-commands-only).

@history_item n=14 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (nine verbs in two levels, the written forms, `command_line`) This entry introduced `SessionCommand` and `validate_session` in `command`, the hand-written `to_json` forms, and `command_line` with its reader rules in a fixed order, four endings (0 accepted, 1 refused, 2 usage, 3 session error), and no ending writing back caller input. `read` in `packages/escher-driver/src/cli.rs` (cognitive complexity 25) is that reader. An extraction must keep the rule order and the endings exactly (contract/cli-exit-statuses/*, contract/command-line/*).

@history_item n=15 marker=2026-10-12-self-description
**2026-10-12-self-description** — (`help`, `describe`, the tables) This added the `help` ending to the command line's reader (no session reached, status 0, an argument refused) and had the MCP server answer `help` itself. Both are arms an extraction of `cli.rs` `read` or `mcp.rs` `run` must keep (contract/command-line/help, contract/mcp-role/help, decision/one-self-description-for-cli-and-mcp).

@history_item n=16 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (the kept check: a recording directory) The reader admits one `--record <dir>` on `serve` and `mcp`, a stray one being usage after `validate`. It also moved `call_of` into `command.rs`. Both are pieces of `cli.rs` `read` and `command.rs` that the split must carry unchanged.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the escher-session coverage role) The reader takes the first word `coverage` after `mcp` and before the verb table, in one shape (`coverage <app> <check>...`, no `--session` or `--record`), with endings 2, 1, 0 and 3. This ratified boundary widening is one of the branches that make `cli.rs` `read` complex, and its order and endings are contract (contract/cli-coverage-role/*).

@history_item n=18 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the MCP server and its `mcp` role) `serve_mcp` speaks newline-delimited JSON-RPC 2.0, with the 65,536-byte request-line bound, the `id` echo, the fixed protocol errors with no `data`, notifications running nothing and idle expiry checked on arrival. `run` in `packages/escher-driver/src/mcp.rs` (complexity 18) holds this loop, and an extraction must keep each bound and answer (contract/mcp-server/*).

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (`serde_json` a reader only; the `tool_call` span) `serde_json` reads MCP requests and writes nothing, the written forms stay hand-written in `json`, and one `tool_call` span per `tools/call` parents `command`. A split of `json_tests.rs` per `json/{answer}.rs`, or an extraction from `mcp.rs` `run`, keeps the hand-written writer and the span's nesting (decision/serde-json-reader-only, decision/hand-written-answers, decision/tool-call-span-tool-and-ending).

@history_item n=20 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the driver's command span) escher-driver builds its spans by hand with `info_span!` and no `#[instrument]`, and fields only the closed set. That is the origin of decision/driver-hand-built-spans, which still binds when span-opening code moves into a helper.

@history_item n=21 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (fifteen verbs; the layout answer's written form) This added the layout answer's JSON shape and its closed word sets (contract/answer-json/layout*), the latest answer form among those that `json_tests.rs` (1,021 lines) tests. Splitting that file into one test child per answer must keep every such case and its test path.

@history_item n=22 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — (snapshot model registered on the Dioxus DOM bridge) This created dioxus-native-dom's crate-private `snapshot` module under `accessibility`, with the rule that the snapshot derives no role or name of its own. `packages/dioxus-native-dom/src/snapshot.rs` (913 lines) is a split target, so any child module must stay crate-private and under the same gate with the same re-exports.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the module records of three crates) dioxus-native-dom's snapshot module re-exports `closed_select_of` and `is_multiple_select`, called by the snapshot's builder, `lint.rs` and the driver. A split of `snapshot.rs` must keep these public re-exports and their callers (contract/driver-forms/*, contract/dioxus-native-dom/closed-select-of).

@history_item n=24 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — (Cross-cutting Patterns: 2 rows) This amended decision/accessibility-feature-gating's closed lists of gated and ungated dioxus-native-dom modules, later extended by 2026-11-16-errors-with-the-action with `boundary`. A new child module carved from `snapshot.rs` sits under the gated `snapshot` and adds no top-level module to either list.

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (`stand_graph` and the blitz-tests shared modules) This added `stand_graph` (6 tests, both modes, in `stand-repeat` by prefix) with `common`'s `Drive` and `GRAPH_DRIVES` and `session_common`'s `graph`/`graphed`. The two over-ceiling functions `every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form` and `every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run` come from here. Extractions keep the test names and assertions, and helpers go to the shared modules or stay private.

@history_item n=26 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (stand_actions and the stand's shared modules) This added `stand_actions` (8 tests, both modes) and `common`'s `ACTIONABLE_AT_BOOT`. `each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs` (complexity 21) is one of its tests. An extraction keeps the table-driven reading and the counts it asserts.

@history_item n=27 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the dioxus_runtime_probe target) The probe is one file and one target with no `stand_` prefix, run only by the workspace `test` leg and `run all`. It reads Dioxus 0.7.10's public API alone, with no print, env read or subscriber. `visit` at `tests/blitz-tests/tests/dioxus_runtime_probe.rs:67` (complexity 22) is its vdom walk, and an extraction must keep the probe on public API with no symbol added to any crate.

@history_item n=28 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the buffered render's witness on Dioxus 0.7) The probe's two `mounted_data_in_a_task_a_key_handler_spawned_acts_*` tests, through `spawned_focus_acts`, are the discriminating witness of the buffered render (guard on decision/bridge-buffered-render). Restructuring the probe file must not weaken them.

@history_item n=29 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — (the a11y leg at six files) This put `stand_accessibility_ids` among the `a11y` leg's named targets, pinned by `LegScriptTest`. `every_interactive_role_element_reads_a_name` (complexity 16) lives in that target, so the target and test names must stay as the leg and its pin name them.

@history_item n=30 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — (Standard Contracts: 1 row) contract/ci-leg-legs/a11y now states named targets with no count, the count being the leg script's inventory. Any target rename would break the leg and its pin.

@history_item n=31 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — (`stand-repeat`'s named list) `stand-repeat` names blitz-tests targets as literal `--test` words after the `stand_*` glob, pinned by `test_stand_repeat_leg_runs_three_rounds` and read by `stand_requirement_sweep`. Later entries grew the list (2026-10-31-engine-debts to fifteen, 2026-10-31-select-popup to seventeen). Test-file splits touch this list, which is why the scope defers the oversize test targets.

@history_item n=32 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — (Project Intent: 1 row) decision/capability-statement now holds the a11y leg's file count and `stand-repeat`'s named-target count to the CI leg runner through the pin. A change to either list reddens `.github/scripts/test_capability_statement.py`.

@history_item n=33 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (upstream's both-modes exception ended) The both-layout-modes rule held no exception after this chunk. 2026-11-13-upstream-sync-determinism-and-exploration then re-added an exception for upstream-taken targets kept as taken (decision/both-layout-modes). Extracted test helpers in escher's own targets keep running `for incremental in [false, true]`.

@history_item n=34 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (Conventions: 1 row) decision/unit-tests-at-file-end's exception dropped its count ("Eight unit-test modules…") for a general clause because the explore module gained a `#[path]` test child. New `#[path]` children from this chunk's splits fall under the exception with no count to maintain.

@history_item n=35 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — (Conventions: 1 row) This set decision/std-result-under-dioxus-prelude: a file that glob-imports the dioxus_native prelude spells a two-parameter result as `std::result::Result`, a failure only the Android CI job sees. Code moved into or out of a prelude-globbing file must keep that spelling.

@history_item n=36 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — (Stack and Technologies: 1 row) decision/local-pre-push-gate: the `fast` leg (fmt, clippy, test, ci-scripts, embolden, stopping at the first red) is the local pre-push gate, ending on a dev host with the target sweep, which never reddens it. This is the gate each move must pass before push. 2026-11-01-upstream-sync-ui-graph made `embolden` the last leg of `fast`.

@history_item n=37 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — (Stack and Technologies: 1 row) This added decision/mutation-tier-excludes-demo-shells: `rdme` and `blitz` stay outside the mutation tier while churn, sizes and the other collectors keep them. The code audit reads its extra exclusions only from an architecture decision row, so the size and complexity readings this chunk acts on still include both shells.
