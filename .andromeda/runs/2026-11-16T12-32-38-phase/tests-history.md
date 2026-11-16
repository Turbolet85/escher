@history_item n=1 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — (the two pinned engine readings read fixed) the founder-ruled fix of the bounds reader and the hit walk: a scrolled box reads the same `bounds` before and after its content scrolls, a raw click outside the padding box of a box that clips it no longer lands (`contract/harness-input/clipped-click`), `stand_act_obstructed` / `stand_act_scroll` restated. These are exactly the readings `0980cce5f`'s scroll-shifted `physical_unrounded_geometry` and `hit_inner` would undo. Trap: a check that leaned on the old hit holds no searchable word, so only the stand gates find it.

@history_item n=2 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — (three engine checks, the first conformance reading) added `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box`, `scrolled_box_absolute_position`, each in both layout modes and green under both the per-package and the workspace build, because the two builds compile different bodies of the bounds reader. Each body's mutation control reads red under its own build only (`decision/bounds-reader-bodies-each-checked`). It also recorded the first `css/cssom-view` WPT reading on the dev host, report-only (`decision/fork-wpt-reading-report-only`). These are the regression checks the take has to keep green under both runners.

@history_item n=3 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (readings on the merged tree) the workspace build resolves blitz-dom with `writing-mode` on and the per-package build without it, so `physical_unrounded_geometry` compiles a different body under each runner. The same stand check runs over a different reader in each build. This is why the take's `writing_mode.rs` conflict resolution must be verified under both `cargo test --workspace` and `-p blitz-tests`.

@history_item n=4 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — set `decision/per-package-green-not-workspace-green`: one check file read green per package and red in the workspace leg, because `tracing` (and later `writing-mode`) is on only in the workspace build. A per-package green on the take is no evidence for the `test` leg.

@history_item n=5 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (`BlitzTestsTargetsTest`) upstream's `tests/all.rs` is kept declared with `test = false`, one target per file (`decision/one-test-target-per-behaviour`, `decision/blitz-tests-one-target-per-file`), and `BlitzTestsTargetsTest` turns a sync red if it takes upstream's two manifest lines. Upstream's `inline_box_scrollable_overflow.rs` must land as its own auto-discovered target, with `all.rs` gaining only its `mod` line.

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (the test inventory gains upstream's tests) recorded `layout/writing_mode.rs` as holding no unit test and upstream arrivals with no test as absences, not failed gates. This is the precedent for how the take's untested arrivals (the `is_scroll_container` seam, the `ScrollableOverflow` measure) are inventoried.

@history_item n=7 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — an upstream merge brought wpt/runner's `attr_test.rs` unit test (`subtest_names_include_nonempty_root_titles`), and file:line citations into merged upstream files were re-pointed by a measured line map. The take touches `wpt/runner/src/test_runners/attr_test.rs`, so that unit test and its citations are in play.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — introduced the harness's `scroll_into_view` and its engine check `scroll_into_view_nested` (7), plus the refusal order `disabled` · `off-screen` · `covered`. It noted that `stand_act_scroll` reads the list's `bounds` before scrolling because a scrolled box's own bounds read shifted then. The take's seam edit at `scrolling.rs` (`scroll_state` as a `(current, min, max)` triple) changes this helper's reader.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`scroll_into_view` on a row of an open list) `contract/harness-input/scroll-into-view` gained the clause that a row of an open select list moves the list's `scroll_y` alone. The take's rewrite of `scroll_into_view` over the `(current, min, max)` triple must keep this branch and the least-movement order.

@history_item n=10 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — grew `scrolled_box_client_rect` to 7 and `hit_clipped_at_scrolling_box` to 13 cases. It recorded `Node::absolute_position`'s fix and the transformed box's client rect as residual-owned, and measured `covered`'s limits. These are the case tables the reconciled readings must still pass.

@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (§1 coverage) `hit_clipped_at_scrolling_box` now reads a `contain: paint` box stopping the hit, and `scrolled_box_absolute_position` reads a scrolled box where it stands. This retired «what stays measured and not fixed» and added `transformed_client_rect` and `stand_act_transformed`. It is the hit-stop and `absolute_position` reading the scope keeps against upstream.

@history_item n=12 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (§2–§10 counts and the harness readings) `layout_rect`, `layout_rect_of` and `center_of` read a scrolled box where it stands, where they were its position less its own scroll offset. This is the origin of `decision/scrolled-box-reads-where-it-stands`, the harness reading `0980cce5f` must not shift back.

@history_item n=13 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified by the founder) ratified `scrolled_box_absolute_position`'s reading and `password_caret_click`'s caret by the mask (`decision/password-caret-placed-by-mask`). Both now stand on the founder's word, so the take cannot quietly move them.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a non-invertible transform takes no hit) `hit_non_invertible_transform` pins `Node::hit_inner` answering `None` for a whole subtree under a non-invertible transform. The take's `hit_inner` conflict (`matches_self`'s content test against `scrollable_overflow_rect`) must keep it.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`visible_region` through transforms) `visible_region_transformed` (6) and `stand_act_transformed` 3 read each clipping box through its transforms. That is a scroll/clip reading adjacent to the take's overflow-rect change, and both targets are named `--test` words of stand-repeat.

@history_item n=16 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`hit_test_dropped_nodes`) the precedent for an upstream hit-test fix arriving with its own check. The check is its own target in the `test` leg and `run all`, outside `run stand` and `a11y`, read red with the engine hunks reversed. Every count moved by that check alone. The same pattern applies to `ebacd68bd`'s `hit_testing_before_the_first_resolve`.

@history_item n=17 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (upstream's `pseudo_element_flow_classification` target and the legs that reach it) the precedent record for an upstream test arriving by a take. It is auto-discovered, with an `all.rs` `mod` line, the `test` leg and `run all` only, none of stand-repeat's named words or the a11y files, and the directory count bumped. `inline_box_scrollable_overflow` follows the same path.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the flow-classification file an exception to the both-layout-modes rule) first recorded an upstream file asserting neither layout-mode pair as a named exception, with `incremental_oracle` as the layout-identity witness. This bears on whether `inline_box_scrollable_overflow` is kept as written or looped.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`pseudo_element_flow_classification` runs both layout modes) a later chunk edited that upstream file to run `for incremental in [false, true]`, ending its exception. This is the alternative precedent for the new upstream test.

@history_item n=20 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (5. Integration Test Strategy: `~ decision/layout-tests-in-both-layout-modes`) added the except: upstream targets taken in a sync are kept as written, and those running one layout mode stand as named exceptions. This is the current rule for `inline_box_scrollable_overflow` and `hit_testing_before_the_first_resolve`.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the key-press panic retired and its check) set `decision/key-after-dropped-anonymous-block`, guarded by `key_press_after_container_lost_its_box`, in both modes. `select_popup` now presses the key in every row. The take's `is_scroll_container` seam exists because upstream's overflow read panicked in `select_popup`'s inert/disabled/hidden/removed case, so these rows guard the seam.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`select_popup`, the list's engine behaviour) the 25-test target holding `a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes`, the test that measured upstream's «layout_style() called on a node without computed styles» panic under the workspace features. It runs in the `test` leg, `run all` and stand-repeat.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`select_popup_painted`, the list on frames) a scrolled list's thumb is painted and takes no hit. The take touches `node/scrollbar.rs` and the scroll range, so this frame check and the `embolden` witness of the same target are in reach.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (`advance` on every session) `advanced_ms` is exactly `ms`, `timer_step` carries its remainder, and the harness's time is a `VirtualClock`. The rows are `decision/advance-moves-exactly-the-ms-asked`, `decision/timer-step-carries-remainder` and `decision/one-virtual-clock`. The follow-up entry moved the scrollbar fade onto the document clock (`decision/scrollbar-fade-on-document-clock`, guarded by `stand_virtual_clock`). `node/scrollbar.rs` is in the take's touches.

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (the clock checks counted) the fade reads the document's clock, so a shot after a scroll repeats until `advance` moves it, read by `stand_virtual_clock`. The take must leave scrollbar paint on that clock.

@history_item n=26 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — (`stand-repeat` runs ten named targets) stand-repeat names `scrolled_box_absolute_position` and `hit_clipped_at_scrolling_box` (among others) as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`. The take's reconciled readings are therefore repeated three rounds with zero failures (`decision/stand-repeat-three-rounds`).

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the stand-repeat leg, seventeen named words and 51 files) stand-repeat now names seventeen targets, `select_popup` and `select_popup_painted` among them. Any new upstream test joins none of them unless the script list and its pin change.

@history_item n=28 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the three per-crate coverage floors and the zero-failure stand-repeat budget (`decision/per-crate-coverage-floors`, `decision/stand-checks-zero-failure-budget`, `decision/zero-failure-stand-repeat`). The trial did not run `coverage`, so the floors are unread for this take.

@history_item n=29 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the coverage leg excludes `keyboard_compat.rs`) the filename exclusion, with floors unchanged (`decision/keyboard-compat-excluded-from-coverage`, `decision/keyboard-compat-coverage-exclusion`). Each take re-reads whether `286874cc1` entered. This take's seven picks leave it out.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `keyboard_compat.rs` exclusion through this take) the per-take re-read sentence pattern: the take leaves `286874cc1` out, so the exclusion stands. The next entry (2026-11-07-mounteddata-buffered-render, residual owners) moved its owner to `res-6ssktw6x`, absorbed into `v030-03`.

@history_item n=31 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the dev-host complexity ceiling) the ceiling of 15 is read on the dev host alone, and engine functions over it are left as they are (`decision/complexity-ceiling-host-only`).

@history_item n=32 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — (10. Quality Gates: `~ decision/complexity-ceiling-host-only`) the engine's click handler is held below the ceiling with upstream's arms, at the price of a hand re-apply when an upstream sync edits one of its arms. The take touches `events/pointer.rs`, so any pick editing a click arm owes that re-apply.

@history_item n=33 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed load ends the wait) trap: an upstream fix stopped silently at a wrapper (`CountedHandler`) because a trait's default method compiled through it, and no gate read it. A later take that extends a trait sweeps that trait's implementers. This applies to Taffy's `cache_get(&self)` and the `physical_layout` fourth argument.

@history_item n=34 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (upstream's commits with no test here) the precedent for recording upstream commits that bring no test: the workspace is their witness, with `incremental_oracle` among its checks (`decision/incremental-differential-oracle`). The Taffy bumps `35cdfc859` … `b4952e15f` bring no test of their own.

@history_item n=35 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (blending and the inline box's paint skip read on no frame) recorded `Node::is_inline_with_only_block_children` and the inline-box paint skip as unread on any frame, owned by a CARRY on «Upstream sync ahead of determinism and exploration». It is adjacent to upstream's `inline_box_scrollable_overflow` and the take's `layout/inline/layout.rs` edit.

@history_item n=36 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (blitz-dom's unit tests from the take) mutator.rs holds 52 `#[test]` functions and blitz-dom 83. `ebacd68bd`'s `hit_testing_before_the_first_resolve` moves the mutator.rs count and the blitz-dom total.

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (line citations re-pointed after the take) citations into `node/node.rs`, `mutator.rs`, `document.rs` and `scrolling.rs`-adjacent files move with a take. The take edits `node/node.rs`, `mutator.rs`, `document.rs` and `scrolling.rs`, so a citation sweep is owed.

@history_item n=38 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (citations re-pointed past the take's added lines) trap: a range that stretches over tests the take added inside it still holds its claim but can leave a count false, as with blitz-vibey-script's 27. Re-read the counts that stretched ranges back.

@history_item n=39 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the workspace tally and `run all`) the per-take tally pattern: per-OS on the final CI run, each move named by target, the local baseline re-counted, and `run all` re-read. The trial measured `test` 1501 → 1510 (+9), which this record must attribute.

@history_item n=40 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — (the tallies re-read on CI#39104364530) rule: a wrap that does not re-read the tallies leaves the next to attribute the gap. The scope's cross-OS digests and per-OS tallies are linux-only in the trial.

@history_item n=41 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the per-OS CI tallies re-read) a tally move is attributed by grepping the test's own name, never by the preceding `Running` line (`decision/tally-move-attributed-by-test-name`).

@history_item n=42 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — fixed the a11y leg at fourteen files (`decision/a11y-leg-membership`) and set the per-job tally ANSI strip `(\x1b|\^\[)\[[0-9;]*m` (`decision/ci-log-ansi-strip`). The trial read the a11y tally unmoved.

@history_item n=43 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — (2. Test Strategy: `+ decision/cross-os-readings-by-answer-digest`) `stand_cross_os_readings` compares answer digests of the six served tasks to one committed table on every CI OS. The scope's open question: the take's geometry change can move these digests, and windows/macOS read only in CI.

@history_item n=44 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (test counts, the third flow script and the shot's same-host limit) a shot's PNG differs between linux x86_64 and macOS aarch64, so a byte-compared cross-OS check holds no shot. The rows are `decision/no-screenshot-step-in-committed-checks`, `decision/cross-os-checks-hold-no-shot` and `decision/no-shot-in-cross-os-byte-checks`. The committed Counter checks are kept fresh byte for byte (`decision/recorded-checks-kept-fresh`).

@history_item n=45 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (the kept answer, the replay and the committed checks as tests) committed checks are replayed by `stand_replay`, red at the first differing kept answer (`decision/recorded-checks-replayed-as-tests`, `contract/check-replay/stop`). A bounds change from the take would turn them red.

@history_item n=46 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — set CI on build/**, linux legs through `ci-leg.sh`, the matrix testing windows/macos and building ios/android, and upstream-only WPT with the fork running WPT on the host. The rows are `decision/ci-trigger-branches`, `decision/native-platform-ci-matrix`, `decision/ci-legs-host-reproducible`, `decision/wpt-run-on-host-on-fork` and `decision/workspace-tests-in-fast-ci-leg`. WPT is unrun in the trial.

@history_item n=47 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `audit` leg (cargo-deny advisories) beside `a11y` and `coverage` (`decision/dependency-audit-advisories-and-licenses`). The Taffy rev move changes `Cargo.lock`'s git source that the audit reads.

@history_item n=48 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the red Dependency audit and its green re-read) trap: the audit went red on an unchanged `Cargo.lock` from newly published advisories, and the fix was a semver update plus an ID ignore. A red audit on the take's lockfile move is not necessarily the take's.

@history_item n=49 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (every cargo leg `--locked` but `fmt` too) `decision/cargo-legs-locked` excepts `fmt` and `examples/wasm_hello`. The take's `Cargo.lock` taffy line must resolve under `--locked` in every other leg.

@history_item n=50 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the free-disk step of `Stand checks repeated`) a stand-repeat red came from runner disk (linker bus errors, no test ran), and the free-disk step was written but not claimed as the cure. A stand-repeat red on the take may be infrastructure.

@history_item n=51 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `embolden` step of `fast`) `fast` ends with `embolden` (`decision/local-pre-push-gate`), the local witness of `select_popup_painted` under `blitz-paint/font-embolden`. A macOS-only paint red was the earlier trap (2026-10-31-select-popup, the macOS-only red).

@history_item n=52 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (9. CI Integration: `+ decision/blitz-dom-requests-through-stamping-constructor`) blitz-dom's clippy config makes the clippy leg fail on any other request construction, a merge surface absent upstream. Picks touching blitz-dom must pass it.

@history_item n=53 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (the merge's tests) the precedent for upstream targets arriving each as their own target, with `all.rs` gaining `mod` lines and staying unbuilt, the executables counted with no `all`, and the directory and workspace tallies re-read on the merged tree.

@history_item n=54 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, and the stand boots through `stand::options(incremental)`. The rows are `decision/stand-checks-bundled-font`, `decision/stand-checks-bundled-font-no-skip`, `decision/stand-pinned-real-font` and `contract/stand-boot/*`. Layout readings under the take are measured on that font.

@history_item n=55 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the served pages' boots and checks) set `stand::boot_cells`, `stand::boot_load` and `LoadNet` (`contract/stand-boot/boot-cells`, `contract/stand-boot/boot-load`). Cells' scrolled grid is a served screen whose scroll range the take's reversed-origin change touches.

@history_item n=56 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — measured the agent-run 5-command contract (`boot`, `run {selection}`, exit grammar, `run stand` empty-run) that `contract/agent-run-verbs/boot`, `contract/agent-run-selection/name` and `contract/agent-run-selection/stand` state.

@history_item n=57 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — measured the test-data bootstrap (fixtures are code compiled by `boot`) and ruled no `run fixtures` selection (`contract/agent-run-selections/all`).

@history_item n=58 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — (3. Test Harness Contract: 4 rows; and `~ overview/session-lifecycle`) `run` gained the `session` selection (`-p seven_guis --test cli_flow`), `contract/agent-run-verbs/run` restated, and the session-lifecycle overview rewritten. These are the current texts of the picked agent-run rows.

@history_item n=59 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — (3. Test Harness Contract: `+ contract/harness-inspect/focused`) with nothing focused, the harness's focused reading answers the root `html` element.

@history_item n=60 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `settle` reads no clock, advances no time, waits on no load and leaves the changed set alone, and pump and the input helpers never settle (`contract/harness-settle/side-effects`, `contract/harness-input/input-helpers`). Trap: the changed set is non-empty on a freshly booted harness.

@history_item n=61 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — acting steps wait out a load in flight up to `LOAD_WAIT`, settling every 10 ms (`decision/load-wait-bound`, guarded by `stand_load_wait`).

@history_item n=62 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a snapshot over `SNAPSHOT_TEXT_BUDGET` is read in parts (`decision/snapshot-text-budget`). Bounds text on Cells' screen moves with any geometry change.

@history_item n=63 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set the headless shot (`decision/headless-screenshot`), frames compared whole (`decision/frames-compared-whole`), and a no-font shot refused, never skipped (`decision/shot-refused-without-fonts`, `contract/driver-refusals/fonts-unavailable`).

@history_item n=64 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`covered`'s closed-option clause and the restated help) `covered`'s fixed meaning gained the closed-select option clause (`contract/refusal-cause/covered`). A hit-path change must keep `covered` and `off-screen` readings as stated.

@history_item n=65 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (stand_actions holds the actions read) `stand_actions` holds each served screen's causes, including `crud-person-14` `off-screen` until a `scroll` and the options `covered`. The rows are `contract/driver-verbs/actions` and `decision/actions-answer-holds-no-value`. These causes ride the scroll and hit readings the take touches.

@history_item n=66 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (5. Integration Test Strategy: 3 rows) added `contract/session-layout/changes-nothing` and `decision/layout-reason-words-each-read` (every placing word read on a fixture in both modes, guarded by `layout_reason`). The layout reader's readings move with any Taffy bump.

@history_item n=67 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — (3. Test Harness Contract: 2 rows; 5. Integration Test Strategy: 1 row) the acting verbs gained `caused` (`contract/driver-session/acting-verbs`), an exploration path's kept check holds its panicked step (`contract/session-recording/exploration-checks`), and `decision/root-boundary-err-on-own-session` was added.

@history_item n=68 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (5. Integration Test Strategy: `+ decision/exhaustion-asserted-on-finite-page`; 3. Test Harness Contract: `+ contract/session-recording/exploration-checks`) exhaustion is asserted on a finite fixture page only. An exploration of the Counter asserts exhausted false.

@history_item n=69 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — (5. Integration Test Strategy: `+ decision/start-in-state-reads-as-by-hand-path`) a fixture state reads as its by-hand path in both layout modes.

@history_item n=70 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph verb held in session and over MCP in process; the stand's state reaches the UI graph; the unconfirmed flag's stated limit) set `decision/graph-answer-holds-no-value`, `decision/reserved-graph-id-form`, `decision/graph-signal-registration` and `decision/unrun-declared-write-reads-unconfirmed`, guarded by `stand_graph`.

@history_item n=71 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` reads no id or name on the sink-installing host at `trace` (`decision/no-ids-or-names-in-stand-logs`, `decision/session-logs-no-screen-content`).

@history_item n=72 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — rules for driver checks: a failure message names a layout mode, task and index, never an id or screen reading. It also pinned `advanced_ms` (later restated by 2026-10-18-one-virtual-clock).

@history_item n=73 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (the session lifecycle contract gains the recording) the recording directory is `0700` and its files `0600`, and a directory open to its group is refused (`decision/owner-only-session-files`), with recording fields `contract/session-recording/record-fields`.

@history_item n=74 marker=2026-10-12-self-description
**2026-10-12-self-description** — each cause served with its remedy over MCP (`decision/mcp-causes-served-with-remedy`). The causes served through MCP are now seven (2026-10-18-one-virtual-clock).

@history_item n=75 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — after an accepted click, the clicked button reads `focused`, and the harness's `scroll_into_view` is exercised through the driver only. It has no check of its own, beyond `scroll_into_view_nested`.

@history_item n=76 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — (the masters as rows) the master's text became the rows the picked handles name, each verified against the code at edd0cfaea7f0. A row's guard is the check a chunk must keep green.
