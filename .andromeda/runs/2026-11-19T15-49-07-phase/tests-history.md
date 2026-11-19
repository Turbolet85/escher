@history_item n=1 marker=2026-10-05-as-built-baseline
**2026-10-05-as-built-baseline** — (the local baseline reading) set the §9 local baseline that decision/local-baseline still names: dev host, workspace default features, `--locked`, cold after a clean and warm on re-run. It is the reference the post-merge fast leg and tallies are read against.

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (CI on the build branch, legs through ci-leg.sh, upstream-only WPT) set the rows this sync must keep green: CI triggers on build/**, every linux leg runs `bash .github/scripts/ci-leg.sh {leg}`, the fast/slow split, a native matrix that tests windows and macos and only builds ios and android, and WPT plus its PR-results posting guarded to `DioxusLabs/blitz` with the fork running WPT on the host. Rows decision/ci-trigger-branches, ci-fast-slow-split, ci-legs-host-reproducible, native-platform-ci-matrix, wpt-run-on-host-on-fork and wpt-upstream-only. Upstream workflow files the merge carries must keep the repository guard.

@history_item n=3 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (local baseline re-measured under line-tables-only debuginfo) moved the dev profile to `debug = "line-tables-only"`, which is the baseline's stated condition (decision/local-baseline). A merge that touches the root `Cargo.toml` `[profile.dev]` stanza must keep it.

@history_item n=4 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the audit, a11y and coverage legs, put rust-cache on every compiling job but coverage, and set up the shared `workspace-test` cache (decision/shared-test-cache, ci-rust-cache). Trap: `cargo llvm-cov report` without `--workspace` exits 0 with an empty table. The scope's audit leg and lock re-read run through this leg.

@history_item n=5 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — (workspace count 430 → 431; citation re-point) is the first upstream-merge precedent for the test plan. The workspace tally is re-counted after the merge and each +N is attributed to a named upstream test. Every `file:line` citation into merged upstream files is re-pointed by a measured line map. A second entry (root-manifest citations re-pointed after the line-61 insert) shows that root `Cargo.toml` citations go stale when a manifest line is inserted.

@history_item n=6 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — (bridge tests, the shared stand module) put the bridge tests in the `cfg(test)` child module `dioxus_document::bridge_tests`, because `DioxusEventHandler`'s fields are private and it has no constructor. It also set the layout of one test target per file plus shared modules that are no target (rows decision/bridge-tests-child-module, bridge-tests-without-widening, shared-test-modules-not-targets). A Dioxus 0.7 → 0.8 take touches exactly this module.

@history_item n=7 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — wrote the changed-set contract checks in blitz-dom `document.rs` (`changed_set_tests`) and `stand_diff`. The scope requires every merge resolution to keep "the changed set's contract", so these are the witnesses a conflicting `document.rs` or `mutator.rs` hunk must keep green.

@history_item n=8 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — defined harness `settle` beside `pump` (quiet pass, `NotSettled { busy: Loads }`, `SETTLE_PASS_LIMIT` 64, no clock read) and wrapped a fetching `net_provider` in the load counter (rows contract/harness-settle/*, decision/harness-counts-fetching-providers-only, network-stand-ins-counted). Trap: the changed set is non-empty on a freshly booted harness. Upstream net or provider changes must keep the counter wrapping.

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — added `host_log` and `telemetry_drop`, the three-outcome target allowlist and the drained pipes (rows decision/telemetry-target-allowlist, sink-drops-third-party-targets, drain-host-pipes-while-running, no-ids-or-names-in-stand-logs). Traps: a check that must fail red on a leaking sink cannot use `start` on a piped host, and a needle list read from the stand can outgrow a hand-kept one. The scope re-checks the sink allowlist against every new tracing/log call site the merge brings, with the capture checks held at 0 needles.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — set the rule that a check reading a sink's capture counts lines by target, never in total, and that a green per-package run is not the workspace leg's green, because `cargo test --workspace` compiles blitz-dom's `tracing` call sites in (rows decision/sink-capture-counted-by-target, per-package-green-not-workspace-green). Upstream logging added by the merge shows up under one runner only.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (the CI-scripts leg runs 78 tests; `tests/all.rs` declared and never built, pinned by `BlitzTestsTargetsTest`) is the origin of the one-target-per-file rule against upstream's single `all` target. The fork keeps upstream's `[[test]] all` at `test = false` with `autotests` on, as a two-line difference, and `.github/scripts/test_blitz_tests_targets.py` turns a sync red if it takes upstream's two lines (rows decision/one-test-target-per-behaviour, blitz-tests-one-target-per-file, one-test-target-per-file, blitz-tests-single-test-target, blitz-tests-manifest-difference). The scope names this rule for upstream's new and edited test files.

@history_item n=12 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (the test inventory gains upstream's tests; two arrivals with no test) is the precedent for recording upstream's tests by area and stating an untested arrival as an absence, not a failed gate. It also took dev-dependency feature additions in blitz-tests (`autofocus`, `text-transform-icu`).

@history_item n=13 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (readings on the merged tree: 158 lines / 719 passed, a second engine feature split by runner, a 1100 s cold CI run) set three things: a moved number is restated where the plan states it, the workspace build resolves blitz-dom with `writing-mode` and per-package does not (two bodies of the bounds reader), and a merge's CI run is cold. Per-OS tallies of a merge run belong to a route entry. Rows decision/per-package-green-not-workspace-green and bounds-reader-bodies-each-checked.

@history_item n=14 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — (both entries) fixed the scrolled-box bounds and the hit walk in the engine and added `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box` and `scrolled_box_absolute_position`. Each of the two bodies of the bounds reader holds a check, and each check reads red only under its own build (rows decision/bounds-reader-bodies-each-checked, scrolled-box-reads-where-it-stands, contract/harness-input/clipped-click). The scope lists "the hit stops and the scrolled-box bounds" as intent the merge resolutions must keep. Stylo, taffy and parley moves land on these readers.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set up the harness screenshot through the CPU renderer composited over white, with frames compared whole and a no-font shot refused, never skipped (rows decision/headless-screenshot, frames-compared-whole, shot-refused-without-fonts, contract/harness-screenshot/*). The anyrender, parley and skrifa pins the merge carries feed these frames. The scope says a moved reading is re-measured and recorded, never loosened.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (the merge's tests: five upstream targets, the bridge's six unit tests, the CI-scripts and workspace tallies) is a sync precedent. Each upstream check became its own auto-discovered target, `all.rs` gained `mod` lines and stayed unbuilt (no `all` executable), upstream's 28-name `is_bool_attr` replaced the fork's 27-name list, and a re-render now drops nodes (#1185). Every figure was re-read on the merged tree and the old ones kept as history.

@history_item n=17 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the two merged ARIA checks; 127 files and 126 targets) took upstream's `accessibility_label` and `accessibility_description` as their own targets, outside the a11y leg. It also found the pre-merge file count one short, so the directory count is measured rather than carried.

@history_item n=18 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the tallies after the merge: workspace, per-OS, agent-run) re-read the workspace, per-OS and `run all` tallies on the merge's final HEAD and explained every moved count by name. This chunk follows the same method.

@history_item n=19 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the coverage leg excludes `keyboard_compat.rs` by filename) added `--ignore-filename-regex 'keyboard_compat\.rs$'` to both llvm-cov lines, excluding upstream's temporary keyboard-types 0.7/0.8 bridge until escher takes its retirement (rows decision/keyboard-compat-excluded-from-coverage, keyboard-compat-coverage-exclusion, per-crate-coverage-floors). Upstream retires the file in `286874cc1` (Dioxus 0.8.0-alpha.3). If this merge takes the Dioxus 0.8 move, the exclusion lifts and the dioxus-native-dom floor reading must be re-read.

@history_item n=20 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the harness re-exports `NamedKey`; test-that named as the assertion crate) moved the harness re-exports to keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` (row decision/harness-reexports-key-types) and recorded the test-that 0.5.2 → 0.6.0 bump as reviewed by hand. Dependency bumps carried by a take are reviewed by hand like this one.

@history_item n=21 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — counted upstream's cursor test and re-read the tallies on the merge's CI run, naming the skipped reading. Its rule for later syncs: a wrap that does not re-read the tallies leaves the next sync to attribute the gap.

@history_item n=22 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (both entries, later ratified) made the harness time a `VirtualClock` that documents read, and made `advance` move it by exactly `ms` on every session. It also added `clock.rs` units, `virtual_clock_engine` and `stand_virtual_clock`, and put the overlay scrollbar fade on the document clock (rows decision/one-virtual-clock, harness-virtual-clock, advance-*, scrollbar-fade-on-document-clock, contract/harness-core/*). The scope lists "the clock invariant" as intent the merge resolutions must keep.

@history_item n=23 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (test counts, the third flow script and the shot's same-host limit) measured that a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577). So no check compared byte for byte across OSes holds a `screenshot` step, and committed checks stay byte-equal to the binary's recording (rows decision/no-screenshot-in-cross-os-checks, cross-os-checks-hold-no-shot, checks-byte-equal-to-recording, recorded-checks-kept-fresh). Moved render or text pins can change the committed checks and the cross-OS digests.

@history_item n=24 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — took upstream's `focus_within` and `focus_redraw` as their own targets outside the a11y leg and `run stand`, with `all.rs` gaining `mod` lines. The per-OS deltas were attributed per target, including chunks whose tallies had not been written.

@history_item n=25 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`hit_test_dropped_nodes`, the counts it moves, and the gaps the take leaves) took upstream's check, read red with the engine hunks reversed and green with them. It stated the take's gaps (for example `PARALLEL_TRAVERSAL_LOCK` asserted by no check) and found the directory count stale.

@history_item n=26 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (the `keyboard_compat.rs` exclusion re-read) rewrote the exclusion as lasting "until escher takes upstream's retirement of it" and named `286874cc1` as the retiring commit. The exclusion's lifetime depends on whether this merge keeps the Dioxus 0.8 move.

@history_item n=27 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — (`stand-repeat` runs ten named targets beside the `stand_` files) set stand-repeat as the `stand_*.rs` glob plus literal `--test` words for named targets, pinned by `test_stand_repeat_leg_runs_three_rounds` (rows decision/stand-repeat-three-rounds, stand-checks-zero-failure-budget, zero-failure-stand-repeat). Upstream targets do not join this leg by themselves.

@history_item n=28 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — set the a11y leg to fourteen files, each keeping its legs, and added the CI log tally rule: strip ANSI written as both ESC and the literal `^[` (rows decision/a11y-leg-membership, joining-file-keeps-its-legs, ci-log-ansi-strip). The scope names the a11y leg as a post-merge proof.

@history_item n=29 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the dev-host complexity ceiling) recorded the cognitive-complexity ceiling of 15 as read on the dev host alone, with engine functions over it left as they are (row decision/complexity-ceiling-host-only).

@history_item n=30 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (upstream's `pseudo_element_flow_classification` target and the legs that reach it) is the record form for an upstream target: it runs in the workspace `test` leg and `run all` with no script line naming it, outside stand-repeat, `run stand` and a11y, with its `mod` line in the unbuilt `all.rs`. New upstream targets in this take get the same legs record.

@history_item n=31 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the per-OS and local workspace tallies re-read after the take) re-read the tallies on the take's CI run and attributed +1 line and +8 passed by name across the new target and tests added inside existing result lines.

@history_item n=32 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the flow-classification file an exception to the both-layout-modes rule) is the first both-modes exception for an upstream file, with `incremental_oracle` as the layout-identity witness (rows decision/incremental-differential-oracle, layout-tests-in-both-layout-modes). It was later converted to both modes.

@history_item n=33 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the script tests the take added; the script tests' shared page and files on disk) brought upstream's `file:`-load narrowing tests (`remote_document_cannot_load_file_urls`, `file_document_can_load_sibling_file_urls`). Their `temp_dir_with_files` added a test-only `TMPDIR` read. The scope lists "the `file:` refusal" as intent the merge must keep, and the sink/env-read re-check covers the test-only env reads.

@history_item n=34 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (citations re-pointed past the take's added lines) re-pointed citations into `mutator.rs`, `element.rs`, `tests/dom.rs`, `dioxus_document.rs` and `all.rs` after the merge. Three ranges stretched over tests added inside them, and the counts they back had to move. This merge will move the same files.

@history_item n=35 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the `keyboard_compat` coverage exclusion re-read and its owner moved) read `286874cc1` as left out, kept both regex lines (gate grep reading 2), and moved the exclusion's owner to the next upstream sync. This is the re-read this chunk owes again.

@history_item n=36 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (`:open` recorded as untested) recorded upstream's `:open` (`flush_open_state`) as a gap. It was later closed by `open_state` (row decision/open-pseudo-class-follows-attribute), and conflicts in `element.rs` must keep that behaviour.

@history_item n=37 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (dioxus-native-dom's IME test and an upstream test absent) took upstream's `input_focused_on_mount_enables_ime` and recorded `task_polled_during_render_can_access_document` as absent because `286874cc1` was not taken. This is the start of the buffered-render and Dioxus 0.8 thread this merge reaches.

@history_item n=38 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the agent-run Proof re-counted after the take) re-counted `run all` (+3, the new target picked up with no script change) and kept `run stand` unmoved. The Proof series takes the same re-count after this merge.

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`pseudo_element_flow_classification` runs both layout modes) converted upstream's file to run `for incremental in [false, true]`, ending its exception. This is an edit to upstream test code, set against the later rule that taken upstream targets are kept as written.

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the free-disk step of `Stand checks repeated`, and the red before it) recorded a disk-exhaustion red on stand-repeat and the pinned free-disk step, unproven as the cure. This leads to decision/free-disk-on-test-and-stand-repeat. A merge build with a large dependency move is the case it guards.

@history_item n=41 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (every cargo leg `--locked` but `fmt` too) recorded that every cargo leg is `--locked` except `fmt` and `examples/wasm_hello` (decision/workspace-tests-in-fast-ci-leg). The merged `Cargo.lock` must satisfy `--locked` on every leg.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the `select` model's limits; `node/select.rs` holds eight tests; a named select's entries read by `form.rs`) built the `multiple` and `optgroup` select model, `select_interaction` 18 and form.rs entries (row decision/chosen-select-row-colours). The scope lists "the select model and its list" as intent the merge must keep.

@history_item n=43 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the macOS-only red and `blitz-paint/font-embolden`'s local witness) recorded that a paint change under `font-embolden` read red only on macOS, and that cargo stops at the first failing target. This led to the `embolden` local witness (row decision/embolden-local-witness). Paint changes from upstream need that leg.

@history_item n=44 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`select_popup`, `select_popup_painted`, `stand_act_popup`, `node/select_list.rs`) built the engine-painted select list and its checks, with stand-repeat naming seventeen words (row decision/select-list-painted-by-paint-scene). These are the select list's witnesses that merge resolutions in blitz-dom and blitz-paint must keep green.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed load ends the wait) found that upstream's failed-stylesheet fix (`f6edb9701`) stopped at the harness's `CountedHandler`, because `NetHandler::error`'s do-nothing default compiled silently through the wrapper (rows decision/failed-load-ends-settle-wait, contract/harness-settle/failed-load). Its trap for later syncs: when a take extends a trait, sweep that trait's implementers.

@history_item n=46 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `embolden` step of `fast`) made `embolden` the last step of the fast leg, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (rows decision/local-pre-push-gate, embolden-local-witness). The scope's post-merge proof includes the embolden witness.

@history_item n=47 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the key-press panic retired and its check) fixed in the engine a panic a sync surfaced (`deallocate_anonymous_block`) and added `key_press_after_container_lost_its_box` with its legs record (row decision/key-after-dropped-anonymous-block). An upstream change to anonymous-block handling must keep this check green.

@history_item n=48 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the matrix job's free-disk step gone; the CI scripts' count) removed the matrix free-disk step, pinned by `test_only_stand_repeat_frees_disk`. The ci-scripts leg read 110 tests.

@history_item n=49 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the workspace tally and `run all`; line citations re-pointed after the take) is the most recent sync precedent for tallies. Per-OS readings were taken on the take's CI run, with each job's +2 lines and +63 passed attributed to upstream's twelve commits, the new check and the harness tests, and citations re-pointed across ci-leg.sh, ci.yml, document.rs, mutator.rs, stylo.rs, node.rs, render.rs, settle.rs, contrast.rs and dioxus_document.rs.

@history_item n=50 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (blitz-net's unit tests and test-only loopback binds) recorded upstream tests that bind `TcpListener` on `127.0.0.1:0`. This sets how a test-only listener from upstream is stated, which matters for the CDP listener question in this take.

@history_item n=51 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `keyboard_compat.rs` exclusion through this take; the coverage readings) recorded the take of `e42ee1f25` (thirty-six of 57 taken, twenty-one left out over merge base `23b9b26a9`, the same merge base as now), with `286874cc1` left out and the exclusion standing. Coverage readings that a take moved were inferred from counts, not read from the lcov.

@history_item n=52 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (blitz-dom's unit tests from the take; `url_search_params`; `render_blocking_stylesheet`) recorded upstream's restyle tests (`check_incremental_restyle`), the script class tests and the stylesheet-unblock tests by commit. This is the area-by-area inventory form this merge's 212 commits will need.

@history_item n=53 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (blending and the inline box's paint skip read on no frame; upstream's commits with no test here) recorded upstream behaviours that have no test of their own, with owners, and recorded `element_removed_before_its_mounted_event_is_flushed` as left out because it reads 1 mounted event on Dioxus 0.7 (row decision/upstream-mounted-flush-test-left-out).

@history_item n=54 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`text_contrast` past a blend) made `text_contrast` read `None` past `mix-blend-mode` after upstream made blending paint, with one formula kept for lint (rows decision/one-contrast-formula, contract/harness-contrast/blend-mode). Upstream paint changes can require a harness reader change.

@history_item n=55 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the target, its legs and its pages; the MountedData hazard; listeners and a held signal's readers) added `dioxus_runtime_probe`, which measures Dioxus 0.7.10 internals: listener attributes over-report, and the vdom walk's dynamic attributes read current listeners (row decision/listeners-read-from-vdom-walk). Its pins are written against dioxus-core 0.7.10 scheduling, so the scope's Dioxus 0.7.3 → 0.8.0-alpha.3 move must re-measure every pin, never assume them.

@history_item n=56 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the red Dependency audit and its green re-read) recorded an audit that went red on new advisories (RUSTSEC-2026-0433, -0429) with `Cargo.lock` unchanged. The fixes were a semver bump of pulldown-cmark and an ignore by ID in `deny.toml`. The scope requires the post-merge `Cargo.lock` to be read through the audit and every `deny.toml` ignore reason re-read.

@history_item n=57 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (dioxus-native-dom's unit tests and the mutation buffer; the fix's witness on Dioxus 0.7; the hazard closed) ported upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge without its Dioxus 0.8 move. The probe pins `…_spawned_acts_*` are the discriminating witness, and upstream's `task_polled_during_render_can_access_document` is kept as written but is insensitive on 0.7 (rows decision/buffered-render-on-dioxus-07-bridge, buffered-render-unborrowed-document, upstream-buffered-render-test-kept-as-written). The scope requires the merge to keep "the buffered render". If the 0.8 move is taken, the port's crate-private `mutation_buffer.rs` meets upstream's own.

@history_item n=58 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (upstream's mounted-event test stays out; the buffered render's limit) keeps `element_removed_before_its_mounted_event_is_flushed` out until the Dioxus 0.8 move, owned by residual `res-6ssktw6x` (absorbed into v030-03) with CARRY `cry-mjwr45xr`. Its return comes with exactly the move this merge carries, and the scope names the Dioxus 0.8 residuals for re-measure.

@history_item n=59 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the per-OS CI tallies re-read, and attributing a tally move) set the rule that a job log's `Running` and `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name (row decision/tally-move-attributed-by-test-name).

@history_item n=60 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (dioxus-native-dom's coverage reading and mutation_buffer.rs) counted `mutation_buffer.rs` inside dioxus-native-dom's 83 % floor with no exclusion. A Dioxus-side restructuring in the merge moves this crate's floor reading.

@history_item n=61 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (dioxus-native-dom's unit scope and its one test-only shim; the stand's state through the fork's registry) added the graph and `signal_registry`, registered stand signals through `use_graph_signal` with no dioxus-core change, and added the one `cfg(test)` shim `DioxusDocument::dispatch_to_vdom` (rows decision/bridge-tests-without-widening, graph-signal-registration, unrun-declared-write-reads-unconfirmed). These depend on Dioxus 0.7 `ReactiveContext` and subscriber APIs that the 0.8 move can change. The scope requires dioxus-native-dom's ids, snapshot, graph and reached record to keep escher's behaviour.

@history_item n=62 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the reached record's unit tests) added `reached::tests` on the bridge's reached record (rows decision/handler-coverage-*). The scope names the reached record among the dioxus-native-dom surfaces a resolution must keep.

@history_item n=63 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the test-plan master's text into rows, each verified against the code at edd0cfaea7f0. Every picked row's handle dates from this conversion, so later amendments change those handles.

@history_item n=64 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (4. Unit Test Strategy, both entries) widened the SHA-pin rule to every workflow and moved its guard to `WorkflowPinTest` (rows decision/ci-actions-pinned-to-sha, actions-pinned-to-commit-sha). The scope requires every CI workflow edit, including upstream workflow files the merge brings, to stay 40-hex pinned under `WorkflowPinTest`.

@history_item n=65 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (5. Integration Test Strategy) gave decision/layout-tests-in-both-layout-modes its named exception: upstream integration targets taken in a sync are kept as written, and those running one layout mode stand as named exceptions. This is the rule for upstream's new test files in this take.

@history_item n=66 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — (10. Quality Gates) added the exception to decision/complexity-ceiling-host-only: the engine's click handler is held below the ceiling, upstream's arms included, at the price of a hand re-apply when an upstream sync edits one of its arms. This is a known conflict cost for this merge.

@history_item n=67 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — (3. Test Harness Contract) added contract/harness-inspect/focused: with no node focused, the harness reads the root `html` element. Upstream focus changes must keep that reading.

@history_item n=68 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (the `file:` refusal rows; the stamping constructor; the real provider inside a stand-in) widened decision/file-urls-only-for-file-documents to engine sub-resource loads, guarded by `local_files_for_local_documents`. It added decision/blitz-dom-requests-through-stamping-constructor, a crate-level clippy disallowed-methods config on blitz-dom that upstream lacks and that the entry names a new merge surface, so any upstream request construction fails the clippy leg. It also added decision/real-provider-inside-stand-in. The scope names "the stamped sub-resource request and the `file:` refusal" as intent the merge must keep.

@history_item n=69 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — (3. Test Harness Contract) made `Harness::from_vdom` provide `SeededRandom::from_seed(DEFAULT_SEED)` as a root context (contract/harness-construction/from-vdom). A Dioxus 0.8 root-context API change touches this.

@history_item n=70 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — (2. Test Strategy) added decision/cross-os-readings-by-answer-digest, which compares the served tasks' answer digests on every CI OS to one committed table, and gave the bundled-font stand decision its guard `stand_bundled_font_families`. The scope's "screenshot and cross-OS readings" proof means the parley, skrifa and Stylo moves may shift these digests: re-measure and record, never loosen.

@history_item n=71 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — (5. Integration Test Strategy) extended the `file:` rule to the browser app's favicon and link gates, while user loads stay ungated. The upstream browser app the merge edits must keep these gates.

@history_item n=72 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — (5. Integration Test Strategy) is the latest sync. Its take added SVG image and feImage href fetches as sub-resource requests under the `file:` refusal, with scope ratified as measured. This is the precedent for a take widening the `file:` surface.

@history_item n=73 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — (9. CI Integration) replaced decision/only-stand-repeat-frees-disk with decision/free-disk-on-test-and-stand-repeat: the default-features test job now frees disk too. This is the latest sync's CI change, and this merge's workflow edits must keep it.

@history_item n=74 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — (1. Test Scope Summary, 2. Test Strategy) pinned the capability statement to the driver schema and to the CI leg script's a11y file count and stand-repeat named-target count (rows decision/capability-statement-pinned, capabilities-doc-pinned-to-schema, capability-check-names-held-to-schema). Any change to a11y or stand-repeat membership made while resolving the merge reddens ci-scripts until `docs/capabilities.md` is re-stated.

@history_item n=75 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — (9. CI Integration, both entries) made a green fast leg on the dev host end with the target sweep, which refuses while a build holds the cargo lock (rows decision/local-pre-push-gate, target-sweep-safe-around-builds, build-directory-held-by-sweep). It measured that Cargo mints a unit hash per resolution × lockfile × toolchain epoch, so a merged `Cargo.lock` moves every hash. The scope's disk reading (`target/` at 244G) and the trial worktree under `target/tmp/` sit with this sweep.
