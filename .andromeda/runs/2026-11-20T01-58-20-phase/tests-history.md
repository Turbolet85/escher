@history_item n=1 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (bridge tests, the shared stand module) — moved the stand checks' restated tables and helpers into one module, `tests/blitz-tests/tests/common/mod.rs`, with no test and no target: three tables (`controls`, `INPUT_NAMES`, `rendered`), six helpers and six reading checks. `stand_element_ids` kept its own `boot`, and §2/§4 changed to "one test-target file per behaviour plus one shared module that is no target". This is the direct precedent for this chunk: a test-only consolidation that added no check and no target and is the origin of decision/shared-stand-tables-in-one-module and its per-file exceptions. Trap: a sed or grep address that ends at a function's name also selects every item whose name starts with it.

@history_item n=2 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (session checks, two shared modules) — introduced `tests/session_common/mod.rs` beside `common`. It was kept separate because the stand module was held unchanged, which is the "why" of decision/session-checks-share-one-module. `mod common;` readers went from six to eleven. Traps: an `#[ignore]` host child blocks a bare `--ignored` run, and a child's inherited libtest lines are counted as tests. Helpers moved into these modules must keep those host children intact.

@history_item n=3 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** (a third shared module) — created `tests/blitz-tests/tests/fixtures/mod.rs` with `boot(app, incremental)`, `hold(label, app, incremental)`, the shared pages and `HeldNet`, which replaced three file-private network copies. It set two rules. A page with one reader, or whose ids differ, stays in its file. The module holds no print, `tracing` call, env read, file, socket, listener or `blitz_net`. It is the origin of decision/shared-fixtures-in-one-module and decision/shared-fixture-pages, the exact rules this chunk's helpers must keep. `run stand` runs the `stand_*` readers and there is no `run fixtures` selection.

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`BlitzTestsTargetsTest`) — recorded the operator's 2026-10-10 one-target-per-file decision, with upstream's `tests/all.rs` declared and held at `test = false`, pinned by `.github/scripts/test_blitz_tests_targets.py`. The shared modules and `all.rs` were listed as no built target. This is the decision the scope's boundary cites: no file is merged, split or renamed and no `[[test]]` target moves (rows decision/one-test-target-per-file, decision/blitz-tests-single-test-target, decision/blitz-tests-manifest-difference).

@history_item n=5 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** (a textarea's value read on the fixture) — put the new check inside `stand_snapshot_state` because the code audit runs against a fixed test set holding that file, and a new file would sit outside the next audit's set. The `common` tables were deliberately not given the textarea because they hold stand controls only. This matters for this chunk: audit-driven test edits keep the audit's file set and keep the shared tables to stand expectations.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (six `stand_act_*` files) — set the rule that a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads. It also set the stand's time step, `stand::timer_step`. Shared act/read helpers this chunk introduces must keep assertion messages to that form, which is the scope's "no id, name, value or typed text newly written into an assertion message" boundary.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — set two rules: every table a driver test reads is stated in the test with its row count asserted, and an assertion over a call carries a row index only. These are the origins of decision/table-tests-count-their-rows, decision/call-assertions-carry-row-index and decision/assertions-print-no-argument-value. A helper that moves a table or an assertion out of a test must not move the table's statement or count out of the test, and must not print an argument value.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `session_common` gained `scroll`, `refused`, `refused_unchanged` and `Acted.in_view`, and its readers went from 11 to 14. These are the existing refusal-reading helpers that the first family's clones (refusal, diff, `caused` reads) may already partly duplicate or could fold into.

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (harness and session contracts gain the shot) — `session_common` gained `screenshot()`, `screenshot_of(id)` and `shot(session, call)`, with `act` reading `None` for a shot. Readers went from 18 to 20; six of the driver-action checks read `mod common;`. This records which shot helpers already exist in the shared module.

@history_item n=10 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — moved `tab_order` and `focused` from per-file copies into `common` (19 readers), taking `tab_order` off the per-file list. It is an earlier precedent of folding repeated helpers into `common` without changing a check.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `replay`, `kept` and a unix-only `mode` to `session_common` (24 readers). It set the session-checks rule that a recording check is `cfg(unix)`-gated item by item beside an off-unix check. A helper folded into `session_common` that touches recording must keep the gating item by item, or it breaks the windows build.

@history_item n=12 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (test counts, the third flow script) — removed `session_common`'s `replay(session, check)` in favour of `Check::replay` and added `shot_prefix` (readers 24 → 25). This is a precedent of shared-helper churn: a helper retired once its callers moved to the library, with the reader count re-measured.

@history_item n=13 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — moved escher-driver's cli and recording tests into `#[path]` files and gave seven_guis' log checks a shared fragment, `common::recorded_needles`. It found that moving test code moved a crate's coverage reading, and recorded "a later test split should forecast a reading change, not none". This chunk's helper moves sit in `tests/`, outside the floors' crates, but the lesson is to forecast what a move changes rather than assume nothing.

@history_item n=14 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — moved unit tests out of line with their test paths unchanged and counted them under their new files. This is a precedent for code moves that keep every test name and path, as this chunk's scope requires.

@history_item n=15 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (the 0.2.0 record) — moved the `many_items` page and `ITEMS` unchanged from `stand_snapshot_parts` into the shared pages, because a second reader (`stand_act_spans`) appeared. The same chunk put ten named targets into `stand-repeat` as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`. This is a precedent for moving a fixture to the shared module on its second reader. The stand-repeat named list is one the scope holds unchanged.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the shared-module readers and the blitz-tests targets) — set `session_common` to 32 readers, `fixtures` to 31 and blitz-tests to 156 files / 155 targets. Trap: an unanchored `mod fixtures;` count reads one more than the true figure, because each shared module's doc comment names the line. Reader counts must be taken anchored at line start.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the stand-repeat leg, seventeen named words and 51 files) — brought `stand-repeat`'s named `--test` words to seventeen, pinned by `test_stand_repeat_leg_runs_three_rounds`, with one `--test` per `stand_*.rs`. The scope requires the stand-repeat leg's named targets to stay unchanged, so no file this chunk touches may leave or join that list.

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the stand checks' shared modules gain the graph's drives and readers) — `common` gained `GRAPH_DRIVES` and its `Drive` enum, and `session_common` gained `graph()` and `graphed(session, call)`. This is part of the current shared-module inventory that new helpers sit beside. The sibling entry put `common` at 22 `stand_*` readers and `session_common` at 33.

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (blitz-tests' shared modules read by stand_actions) — set the counts to `common` 23 readers, `session_common` 34, `fixtures` 32 (24 `stand_*`) and blitz-tests 160 files / 159 targets. It also added `ACTIONABLE_AT_BOOT` to the shared module. In `stand_mcp_tools`, the `graph` test's body was made a shared helper, `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone`, which the `actions` test also runs. That is an in-file precedent for folding a repeated check body into one helper, "one statement of the contract".

@history_item n=20 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the blitz-tests shared modules and targets) — the latest reader counts before this chunk: `common` 24 `stand_*` readers, `session_common` 35, blitz-tests 161 files / 160 targets and the unbuilt `all.rs`. A sibling entry added `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES` and made `GRAPH_DRIVES` read by two checks. These are the baseline figures this chunk's census moves against, re-read at HEAD.

@history_item n=21 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the two table tests' private helpers) — split two table tests under the complexity ceiling using private `#[track_caller]` helpers alone. Every assertion and message was kept, each helper's message names the row and index only, and the controls read red then green. This is the closest precedent for this chunk's own rule, "a helper moves setup and reading code, never an assertion's subject or bound", from the same v030/v040 "Codebase health" capability.

@history_item n=22 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host complexity ceiling) — recorded the code audit's dev-host-only cognitive-complexity ceiling of 15 (rust-code-analysis-cli 0.0.25). Analyser fact: `cognitive.sum` adds nested closures and nested `fn`s into their parent, so only module-level sibling items lower a reading. This is the same audit recipe family as this chunk's jscpd duplication reading. Extracted helpers should be module-level items.

@history_item n=23 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (2. Test Strategy) — `+ decision/count-and-rename-sweeps`: a missed numeric pin and a missed renamed witness set a convention for count changes and renames. This chunk's scope forbids renaming any test function, and a moved helper must not change a function name another check pins.

@history_item n=24 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (2. Test Strategy) — `~ decision/count-and-rename-sweeps` gained an exception: a test name holding a count word that another check pins as a witness string is kept, and the name is searched before any rename. `~ decision/cross-os-readings-by-answer-digest` changed six served tasks to seven. Both bear on this chunk: witness-pinned test names must stay exactly as they are.

@history_item n=25 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — `+ decision/requirement-witness-inside-stand-repeat` makes every 0.3.0 requirement name a witness inside stand-repeat, in a table pinned by function name. `~ decision/typed-password-never-returned` widened the mask rule to every answer carrier and added the guard `stand_masks_on_new_answers`. Moving reading code into helpers must keep every pinned function name and every mask assertion in its check.

@history_item n=26 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (5. Integration Test Strategy) — `+ decision/mcp-result-decoded-whole`: a check comparing an MCP tool result decodes it whole through the one shared session-check helper (guard `stand_act_errors`), after the Windows red of CI#43940101161. `~ decision/root-boundary-err-on-own-session`: caused now carries the error text and chain. Both name `stand_act_errors`, a first-family clone file. Any fold of its MCP read must route through that same shared helper.

@history_item n=27 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (5. Integration Test Strategy) — `+ decision/root-boundary-err-on-own-session`: a handler Err the root boundary takes is checked on a session of its own. `~ contract/driver-session/acting-verbs` adds `caused`. The first-family clone (`stand_act_error_text` / `stand_act_errors` / `stand_order_served`) reads `caused`, and a shared helper must keep the root-Err case on its own session.

@history_item n=28 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (2. Test Strategy) — `+ decision/engine-items-asserted-on-tracing-build`: engine caused items are asserted only on a tracing build, read through the engine's own constant, and the check runs on both builds (guard `stand_act_error_text`, the first family's source file). A folded `caused` reader must keep that build-conditional assertion inside the check.

@history_item n=29 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ decision/stand-boots-offline` and `~ decision/stand-load-page-fixed-answer` each gained a named exception: the error-text check's failing page, whose `FailingNet` stand-in fails its fetch. That boot belongs to `stand_act_error_text`. A shared boot must keep every other stand boot offline and leave `FailingNet` where its single reader is.

@history_item n=30 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. / 5.) — added `+ contract/stand-boot/boot-order` (`stand::boot_order`, the seventh served task) and `+ decision/caused-owner-pinned-as-graph-reads` (guard `stand_order_served`, a first-family clone file). The owner of a caused item is pinned as the UI graph reads it, not as asserted. A shared caused-reading helper must keep that pin.

@history_item n=31 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (3. Test Harness Contract) — added `+ contract/harness-construction/from-component-alone`, `+ contract/harness-construction/vdom-alone`, `+ contract/stand-boot/boot-alone` and `+ decision/component-alone-props-as-maker`. A sibling entry added `+ decision/component-alone-root-context-before-build`. These are boot surfaces a shared boot helper would wrap: the props maker and the root context provided before the build bind every caller.

@history_item n=32 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (2. / 8.) — added `+ decision/pinned-viewport-read-from-document`, which reads the pinned viewport from the document and never from the root's bounds, and `+ decision/timer-context-read-by-delivered-ticks`. Boot-condition checks a helper might absorb must keep those readings.

@history_item n=33 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages' boots and checks) — added `stand::boot_cells` and `stand::boot_load` with `LoadNet`; `net_provider: None` is kept for every caller but `boot_load`. The served-page checks declare `mod session_common;` and no `mod fixtures;`. These are existing boot contracts (rows contract/stand-boot/boot-cells and contract/stand-boot/boot-load) and the per-file module declarations a consolidation must respect.

@history_item n=34 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand's tables and checks re-pinned) — re-pinned the shared tables in `common` (`controls`, `ENABLED_TAB_ORDER`) and `session_common::change` to the flight booker's select. This shows that the shared tables are stated expectation updated in one place, which is the rule behind decision/shared-stand-tables-in-one-module.

@history_item n=35 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' grid as one Tab stop in the checks) — `tests/common/mod.rs` gained `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`. A sibling entry put `mod fixtures;` readers at 28, with `run {file}` covering the non-stand readers. Both are further shared-table growth and reader-census precedents.

@history_item n=36 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb through the command line and MCP) — `stand_mcp_tools`' `graph` and `actions` tests share one helper as "one statement of the contract". This in-file fold kept both test names and their assertions. See also n=19.

@history_item n=37 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (§1, §3, §4, §5, §6, §8) — `advance` is accepted on every session and moves the clock by exactly `ms`, and `timer_step` returns nothing and carries its remainder. The harness's `VirtualClock` is never moved by `screenshot`, `text_contrast` or `settle`. This is the origin of the picked advance, clock and timer rows, ratified by the founder in 2026-10-18-engine-debts (Epoch 2 close). An act helper that wraps `advance` must keep those readings.

@history_item n=38 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the clock checks counted) — `into_config` puts the harness's clock in `DocumentConfig.clock`, and a wrapped document keeps its own. This is the origin of decision/wrapped-document-keeps-own-clock. `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep exception.

@history_item n=39 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — added `stand_load_wait` and `LateNet`, and gave "no sleep and no wall-clock read" its second exception (`stand_load_wait`'s `Instant::now()` and `LateNet`'s 200 ms sleep). `session_common` readers went from 26 to 27. Shared helpers must not introduce a sleep or a clock read, which is decision/no-wall-time-in-session-checks.

@history_item n=40 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a fetching `net_provider` is wrapped in the load counter that settle reads, and `session_common` gained `act` semantics via `Session::act`. Traps: a delivered tick needs a pass of the check's own only when it follows the input helper, and the changed set is non-empty on a freshly booted harness. A shared boot-then-act helper must keep the order of the check's own pass and drain.

@history_item n=41 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ contract/harness-construction/from-vdom`: `from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context where none is held, and a builder-given context stands. A shared boot that builds through `from_vdom` inherits this, and any root context it adds changes it.

@history_item n=42 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — established the stand boot (`seven_guis::stand::{boot, boot_timer}`, `stand::options(incremental)`) as the stand checks' drive surface, with checks asserting unconditionally under bundled DejaVu Sans and system fonts off, and `TimerTicks` as the time stand-in. This is the origin of the picked rows decision/stand-checks-bundled-font, decision/stand-pinned-real-font and decision/stand-timer-ticks, which shared boots must keep.

@history_item n=43 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — added `+ decision/cross-os-readings-by-answer-digest` and set `decision/stand-checks-bundled-font`'s guard to `stand_bundled_font_families`. Moving boot or read code must leave the committed digest table's readings unchanged.

@history_item n=44 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the same check file read green per package and red in the workspace leg. A green per-package run is not the workspace leg's green, and a check reading a sink's capture counts lines by target. This is the origin of decision/per-package-green-not-workspace-green: helper moves must be verified under the workspace build too.

@history_item n=45 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. A shared span-capture helper must not share one recorder across tests.

@history_item n=46 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (three engine checks) — each of the bounds reader's two bodies (writing-mode on in the workspace build only) holds a check of its own. This is the origin of decision/bounds-reader-bodies-each-checked. The same stand check runs over a different reader under each runner.

@history_item n=47 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the zero-failure flakiness budget on the stand checks (the `stand-repeat` leg, three rounds, no retry) and added the coverage floors. This is the origin of decision/stand-checks-zero-failure-budget and decision/zero-failure-stand-repeat, which this chunk's moved checks must still pass.

@history_item n=48 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — `~ decision/local-pre-push-gate`: a green fast leg ends with the target sweep on a dev host with CI unset. It also added `+ decision/target-sweep-safe-around-builds`. This is the current form of the local gate this chunk runs.

@history_item n=49 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` reads fmt → clippy → test → ci-scripts → embolden, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. This is the leg order of decision/local-pre-push-gate. The clippy leg is where the scope's no-new-dead-code-warning boundary is read.

@history_item n=50 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Integration Test Strategy) — `~ decision/layout-tests-in-both-layout-modes` gained the except clause that upstream targets taken in a sync that run one layout mode stand as named exceptions. Stand checks keep both modes (decision/stand-checks-in-both-layout-modes). A helper must not drop a mode loop.

@history_item n=51 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** (3. Test Harness Contract) — added `+ contract/agent-run-selection/session`, and changed `~ decision/agent-run-holds-no-process` (except now names the session selection's `cli_flow` flows) and `~ decision/test-data-is-code`. A sibling entry restated `overview/session-lifecycle`. These rows bound which checks may start host processes; shared helpers stay in-process.

@history_item n=52 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the session lifecycle contract gains the recording) — session-checks: a check that records is `cfg(unix)`-gated item by item beside an off-unix `Unsupported` check the windows job alone runs. A shared helper used by unix-only and all-OS checks must follow that gating so windows compiles with no dead-code warning.
