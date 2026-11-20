@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — (headless stand surface) registered `seven_guis::stand`: the pinned 800x600 · scale 1.0 · Light boot, `font_ctx()` with the bundled DejaVu Sans, `options(incremental)` offline with `net_provider: None`, a fresh `VirtualDom` per boot, no telemetry, env read or `blitz_net`. This is the boot the stand checks repeat, and the one a shared boot helper would wrap. The helper must keep every one of these pins as it reads (decision/stand-pinned-environment, decision/stand-offline).

@history_item n=2 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — registered `scripts/agent-run.sh run {stand|all|name}`, which selects the stand checks by target name. A shared helper module must not rename or merge any `stand_*` file, because `run stand` and `stand-repeat` both pick targets by that prefix.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — registered the re-executing tests: a test binary runs itself again with `--ignored --exact {child} --nocapture`. Later chunks added more of these (`stand_act_spans`, `stand_screenshot_repeat`, `stand_session_lifecycle`). Trap for this chunk: if setup code moves into a helper and a child test's name or path changes, the parent's `--exact` selector matches nothing and still exits 0.

@history_item n=4 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the precedent for this chunk. It created `tests/blitz-tests/tests/common/mod.rs`, read through `mod common;`, to state the stand checks' repeated tables and helpers once, and it closed a code-audit finding without changing any behaviour. Its standing rules: a module in a subdirectory of `tests/` is no test target, so each target file still covers one behavior; a table typed by dev-dependencies is shared through such a subdirectory module, because the test crate's `[lib]` cannot see dev-dependencies. Kept as it was: per-file repetition elsewhere in `tests/blitz-tests`, since only the stand checks share a module. That keeps this chunk's fold to the stand checks.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (the driver session registered) added the second shared module, `tests/session_common/mod.rs`, for the checks that hold a `Session`, beside `common`. This is the split between the two modules that the scope asks P3 to read: which helpers each family already includes.

@history_item n=6 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — Conventions → Tests was restated: the checks that hold a `Session` (the session checks and `stand_settle`) share `session_common`. This fixed the module's membership rule by session-holding, not by file prefix. A session-setup helper this chunk adds belongs in `session_common`.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — fixed the rule that a refusal holds nothing a call supplied, the eight closed causes and the `Fault` set. A shared refusal-reading helper must assert on the closed cause and fault words alone and write no id, name or typed text into a test name or message (scope boundary; contract/driver-session/refusal-content).

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (blitz-tests keeps one test target per file) recorded decision/blitz-tests-target-per-file. `tests/` sets no `autotests`; upstream's `tests/all.rs` is the one `[[test]]` table, held at `test = false`; the arrangement is pinned by `BlitzTestsTargetsTest` in `test_blitz_tests_targets.py`. The operator's reasons, which bind this chunk: under upstream's single binary, `--test {name}` names no target, `mod common;` and `mod session_common;` do not resolve from a module, the re-executing checks' `--exact` names would match nothing, and the telemetry checks need a process each. Shared helpers must stay in non-target subdirectory modules, and no file may be merged or split.

@history_item n=9 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added the `stand-repeat` leg: three rounds of every `stand_*.rs` target plus the seven_guis process checks, the first red ending the leg, no retry. It also added the clippy leg's second line, upstream's blitz-dom clippy, run only when the workspace line is green. This chunk's helpers must keep stand-repeat at zero failures over three rounds and add no dead-code warning under `-D warnings` in any target that includes a shared module.

@history_item n=10 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — precedent for a test-only move. Unit-test modules were moved whole into `#[path]` children with their test paths unchanged, and seven_guis' shared test module took the log checks' readers (`Needles`, `stamped_lines`, …) for `host_log` and `mcp_log`. It shows a shared fragment for two targets, written as detail under the standing records.

@history_item n=11 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — added the third shared module, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`. It states once the fixture boot and hold, the fixture pages that more than one check reads, and `HeldNet`. A page with one reader stays in its check's file (decision/single-reader-page-stays-local). This is the rule for what a shared fixture module may take, and the counts of the module's readers are registered in Existing Scopes.

@history_item n=12 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — committed the two stand checks `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, read by `stand_replay` and kept fresh by seven_guis `recording`. They are the one exception to owner-only. A refactor of the replay or recording checks' setup must leave both files and their freshness check byte-stable.

@history_item n=13 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`Parallel` style traversals serialised) [Style threading] now states that no headless document (harness, stand or driver) sets `style_threading`. A shared boot helper must keep that default (decision/headless-documents-default-threading, decision/headless-documents-sequential-style).

@history_item n=14 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` gained a named `--test` list after the `stand_*.rs` glob. The list is pinned by `test_stand_repeat_leg_runs_three_rounds` and read from the script's text by `stand_requirement_sweep`'s leg check. The scope keeps these named targets unchanged, so no target may be renamed.

@history_item n=15 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg names its targets literally on one `cargo test` line (`stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, `stand_act_select`, …). The pin asserts each target word. That is a second named-target list a rename or merge would break, beside stand-repeat's.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the headless stand serves Cells and a load page) added `boot_cells`, `boot_load` and `LoadNet`, the one boot that is not offline, with `the_stand_options_stay_offline` guarding every other caller. A shared stand-boot helper has to keep the load page's boot separate and keep the offline guard green (decision/stand-offline except, decision/stand-load-net-answers-after-delay).

@history_item n=17 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — three entries:
- Precedent for factoring test code under the code audit. Two table-asserting tests moved their per-row assertions into private `#[track_caller]` helpers whose messages name the row and index only, with no test name, target, message or count changed.
- `stand-repeat`'s membership is held jointly by `test_stand_repeat_leg_runs_three_rounds` and the sweep's whole-script `contains` reading. A target kept in the `a11y` leg alone still reads as inside, a reach gap left to its carried owner.
- The dev-host cognitive-complexity reading: `rust-code-analysis-cli`, ceiling 15, never in CI (decision/cognitive-complexity-ceiling), the audit tool beside this chunk's jscpd reading.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (`::before` / `::after` in flow classification) wrote the first exception to the both-layout-modes convention, for an upstream-taken target that asserts one mode. This is the history behind the picked rows decision/both-layout-modes and decision/scenarios-both-layout-modes: a helper that moves setup must keep each check's `for incremental in [false, true]` loop as it reads.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — two entries:
- (upstream's both-modes exception ended) The rule briefly held no exception.
- (stand-repeat's named targets) The named list grew to fifteen while `stand_requirement_sweep`'s `STAND_REPEAT_NAMED` still holds the first ten. The blitz-tests counts entry also moved the shared tables (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`) and `HeldNet`'s readers.

Both are registered readings that a fold of shared tables could move.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list's checks and the counts they moved) `stand-repeat`'s named targets outside the prefix went to seventeen, `select_popup` and `select_popup_painted` among them, pinned by `test_stand_repeat_leg_runs_three_rounds`. The declarer counts of `session_common` (32) and `fixtures` (31) were re-measured by `git grep`. This shows how each chunk restates the shared modules' declarer counts.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the key-press panic's fix) records the standing trap behind decision/tests-assert-fixture-first and decision/fixture-precondition-first: a probe can read green because its input never reaches the path, so a test asserts its fixture first. Shared setup helpers must keep each check's precondition assertion ahead of the assertion under test.

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (`stand_graph` and the blitz-tests shared modules) `common/mod.rs` took `Drive` and `GRAPH_DRIVES`, and `session_common` took the `graph` builder and the `graphed` reader. This is the per-verb builder-and-reader pattern in `session_common` that act and read helpers follow.

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (stand_actions and the stand's shared modules) `session_common` took the `actions` builder and `actioned` reader. A recorded deviation folded `stand_mcp_tools`' `graph` and `actions` tool calls through one shared helper, a direct precedent for collapsing repeated call-and-read code into one statement. Reader counts at that point: common 23, session_common 34, fixtures 32 (24 `stand_*`).

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (stand_handler_coverage, the handler tables and the shared modules' readers) the last registered reader counts: `mod common;` 24, `mod session_common;` 35, with `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES` generated in part (`grid_cells()`). This is the baseline the scope rows restate if this chunk changes which checks declare a shared module.

@history_item n=25 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Conventions: 1 row) `~ decision/both-layout-modes` took its exception: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode, and escher's own targets run both. All stand checks are escher's own, so every helper-folded check keeps both modes.

@history_item n=26 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — `~ decision/stand-typography-dejavu-sans` (guard `stand_bundled_font_families`) and `+ decision/stand-readings-same-on-every-ci-os` (guard `stand_cross_os_readings`). The stand boot's one-font fallback and the cross-OS digest equality are now guarded rows. A shared boot or read helper must leave both guards green on linux, windows and macOS.

@history_item n=27 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — two entries:
- (Standard Contracts) Added `+ contract/driver-session/reach` and `+ contract/headless-stand/start-in`, the in-process boot of a named fixture state that the stand checks use for setup, refused before boot outside the task's closed table.
- (Conventions) Added `+ decision/std-result-under-dioxus-prelude`, from a red Android run. Any file, a moved helper module included, that globs the Dioxus prelude must spell std's two-parameter `Result` explicitly.

@history_item n=28 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — two entries:
- (Standard Contracts) `+ contract/acting-verb-results/caused` and `+ decision/capture-dispatcher-pair`. This is the `caused` reading that the first candidate family (`stand_act_error_text` · `stand_act_errors` · `stand_order_served`) repeats.
- (Conventions) `+ decision/capture-before-own-recorder`, from the operator's CI fix: a stand check that installs its own recorder opens an empty capture first. A shared setup helper must keep that order, or call sites cache interest from a thread with no subscriber.

@history_item n=29 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `~ contract/acting-verb-results/caused` (one item per record: message cut at 1024, location, boundary and chain, fields, 16 kept), `+ caused_past`, `+ contract/telemetry-capture/bounds`, and the `no-echo-of-typed-input` exception. These are the readings the cloned `caused` readers assert. A shared reader moves the reading code, never the bound or subject asserted.

@history_item n=30 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ decision/no-echo-of-typed-input` and `~ contract/acting-verb-results/caused`: a caused item's `error` field crosses as written, scrubbed by the answer's own hidden field names. These are the current promises the error-text family's helpers read against.

@history_item n=31 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — `~ decision/masked-input-values` gained the guard `stand_masks_on_new_answers`. A stand check is the named guard of a masking decision, so folding its setup must keep its function name and its assertions intact.

@history_item n=32 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement`: the capability pin now reads the `a11y` leg's file count and `stand-repeat`'s named-target count from `ci-leg.sh`. A third reader holds those target lists fixed, which this chunk must leave unchanged.

@history_item n=33 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/stand-readings-same-on-every-ci-os` (seven served tasks), `~ contract/headless-stand/boot` (the order page's offline boot) and `~ contract/escher-session-cli/usage`. These added `stand_order_served`, one of the first candidate family's three clones, and the order page whose authored `caused` text it reads.
