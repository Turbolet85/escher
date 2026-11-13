@history_item n=1 marker=2026-10-05-as-built-baseline
**2026-10-05-as-built-baseline** (the local baseline reading) — set the §9 "Local baseline" (dev profile, workspace default features, `--locked`, no opt-level rewrite, cold/warm). It is what decision/local-baseline records, and the take's re-counted workspace tally is stated against it.

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (CI on the build branch, legs through ci-leg.sh, upstream-only WPT) — set the CI triggers (main, v0.*, build/**), the linux legs through `.github/scripts/ci-leg.sh`, the fast/slow `needs` split, the matrix that tests windows/macos and only builds ios/android, and WPT plus its result posting guarded to `DioxusLabs/blitz`. These are the rows decision/ci-trigger-branches, ci-fast-slow-split, ci-legs-host-reproducible, native-platform-ci-matrix, wpt-run-on-host-on-fork and wpt-upstream-only. A take that edits `wpt.yml` (`f2065fd48`, `946f7c461`) has to keep the repository guard.

@history_item n=3 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `audit` (cargo deny), `a11y` and `coverage` legs, and CiWorkflowTest's SHA-pin, toolchain and `permissions` invariants. Trap: `cargo llvm-cov report` without `--workspace` exits 0 with an empty table. This chunk must turn the audit and a11y legs green. It also widens the SHA-pin check past ci.yml (`cry-zexngiys`), which extends these pins (decision/ci-actions-pinned-to-sha, actions-pinned-to-commit-sha, ci-read-only-permissions).

@history_item n=4 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off; only the pre-existing font tests skip. These are rows decision/stand-checks-bundled-font(-no-skip), font-assertions-on-system-fonts and stand-pinned-real-font. If the take brings Parley 0.13 or a default-font fallback (`3a146c669`), stand text numbers may move, and they get re-measured, never skipped.

@history_item n=5 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (workspace count 430 → 431; citation re-point) — the first upstream merge. It re-counted the workspace tally after the take, attributed the +1 to an upstream test, and re-pointed 19 citations into merged upstream files by a measured line map. This is the precedent for every later sync's test-plan wrap.

@history_item n=6 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot unit tests' default config carries the no-op HTML parser, so parsed markup is pinned in blitz-tests (row decision/parsed-markup-pinned-with-real-parser). Trap: the editor's Backspace arm is compiled out on macOS, so a deleting key is green on linux and red on macOS. Any upstream test the take brings that uses keys reads differently per OS.

@history_item n=7 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — put the bridge tests in the `cfg(test)` child module `dioxus_document::bridge_tests`, because `DioxusEventHandler`'s fields are private and it has no constructor. It also set the layout rule of one target per behaviour plus shared modules that are no target. These are rows decision/bridge-tests-child-module, bridge-tests-without-widening, unit-tests-in-source-file, shared-test-modules-not-targets and one-test-target-per-behaviour. `dioxus_document.rs` is a merge surface of the take.

@history_item n=8 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — recorded that `stand_session_quiet` proves silence only for a host with no sink (row decision/log-silence-read-on-a-sink-host). Traps: an `#[ignore]` host child blocks a bare `--ignored` run, and a sink-installing host at `trace` overruns an undrained pipe (rows decision/drain-host-pipes-while-running, sink-host-output-drained).

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` and `telemetry_drop` cover the sink-installing host: needles read from the stand, both streams drained, and third-party targets dropped under a naming `RUST_LOG`. Rows decision/telemetry-target-allowlist, sink-drops-third-party-targets and host-stderr-no-screen-content. The scope's "sink allowlist re-checked" (0 needles, `packages/escher-telemetry/src/format.rs`) re-runs exactly these checks against every new tracing/log call site the take brings.

@history_item n=10 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `settle` reads no clock, advances no time and neither reads nor drains the changed set. A fetching `net_provider` is wrapped in the load counter that settle reads. Trap: the changed set is non-empty on a freshly booted harness, so a check drains it first. Rows contract/harness-settle/side-effects, decision/harness-counts-fetching-providers-only and settle-never-advances-time. These are guarded behaviours a conflict resolution in the changed-set code must keep.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — added §9 "Engine features by runner": the workspace build compiles blitz-dom with `tracing`, and the per-package build does not. A check reading a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green. Rows decision/per-package-green-not-workspace-green and sink-capture-counted-by-target. Upstream's new tracing call sites land under this split.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the stand log's second record class, the closed-span line (rows decision/closed-spans-logged-as-lines, contract/stand-log/span-line). It is part of the log format the sink re-check holds.

@history_item n=13 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`tests/all.rs` declared and never built, pinned by `BlitzTestsTargetsTest`) — the fork keeps one target per file and carries upstream's `all` table at `test = false`, a two-line difference from upstream. `.github/scripts/test_blitz_tests_targets.py` turns a sync red if it takes upstream's lines (rows decision/blitz-tests-single-test-target, blitz-tests-manifest-difference, one-test-target-per-file, blitz-tests-one-target-per-file). The scope names this rule for upstream's new test files.

@history_item n=14 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (the test inventory gains upstream's tests; two arrivals with no test) — the precedent for recording upstream's arrived tests in the inventory and stating untested arrivals as absences, not failed gates (the plan is tier 0).

@history_item n=15 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (readings on the merged tree; a second engine feature split by runner) — the workspace and per-package builds resolve blitz-dom with different features (`writing-mode` on in the first only), which selects the body of the bounds reader. It also recorded that the per-OS tallies after a merge are stated nowhere until a route entry attributes them. This chunk owns re-reading those tallies.

@history_item n=16 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (three engine checks, the re-measured counts) — `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box` and `scrolled_box_absolute_position` each hold a check in both bounds-reader bodies, with mutation controls red under their own build only. Rows decision/bounds-reader-bodies-each-checked and scrolled-box-reads-where-it-stands. The scope requires that the scrolled-box bounds and hit stops survive the conflict resolution.

@history_item n=17 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — a check that holds a session over the socket is `cfg(unix)`-gated, while the MCP stdio checks run on windows (row decision/socket-checks-unix-gated). Trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This bears on reading new upstream tracing sites.

@history_item n=18 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the counts moved and the new checks of the screenshot and the mask) — shots are painted with no display and compared whole (PNG equal to the raster, byte-identical), a font-less shot is refused, and the password mask has its frame checks. Rows decision/frames-compared-whole, shot-refused-without-fonts, typed-password-never-leaves and headless-screenshot. Paint moves the take brings (vello 0.12, text-shadow, masks) may move pixels, and a moved number is re-measured, never loosened.

@history_item n=19 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the merge's tests: five upstream targets) — each upstream check became its own target, with `all.rs` gaining `mod` lines and staying unbuilt. The bridge's falsy clear moved to upstream's `is_bool_attr`, and counts were re-read on the merged tree. This is the precedent for upstream test files in this take.

@history_item n=20 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %), the zero-failure stand-repeat budget, and the first attributed per-OS tallies (rows decision/per-crate-coverage-floors, stand-checks-zero-failure-budget, zero-failure-stand-repeat). The take has to keep every floor and the three-round repeat green.

@history_item n=21 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the two merged ARIA checks; bump leaves the inventory) — merged upstream checks become their own targets, run by the workspace leg and not by the a11y leg unless added. Measuring the tree corrected the stale file and target counts.

@history_item n=22 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the tallies after the merge: workspace, per-OS, agent-run) — re-read the per-OS tallies on the final HEAD's green run and named every moved count. This is the method this chunk's "per-OS tallies re-read" follows.

@history_item n=23 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the coverage leg excludes `keyboard_compat.rs` by filename) — both llvm-cov lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`, upstream's temporary keyboard-types bridge, with the floors unchanged (rows decision/keyboard-compat-excluded-from-coverage, keyboard-compat-coverage-exclusion). The exclusion lifts only if this take carries `286874cc1` (Dioxus 0.8).

@history_item n=24 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the harness re-exports `NamedKey`; test-that named) — the harness re-exports keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey`, and blitz-tests carries test-that 0.6.0 (row decision/harness-reexports-key-types). If a take bumps dev-dependencies, they are reviewed by hand.

@history_item n=25 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — re-read the tallies on the sync's green run and named the skipped earlier reading. Rule for later syncs: a wrap that does not re-read the tallies leaves the next wrap to attribute the gap.

@history_item n=26 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (`advance` on every session, eight causes, the harness's clock) — `advance` moves the session's clock by exactly `ms`, `timer_step` carries its remainder, and the harness's time is a `VirtualClock` that screenshot, text_contrast and settle never move. Rows decision/advance-moves-exactly-the-ms-asked, harness-virtual-clock, timer-step-carries-remainder and one-virtual-clock. The "clock invariant" the scope requires every resolution to keep.

@history_item n=27 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the clock checks counted) — `virtual_clock_engine` and `stand_virtual_clock` hold the clock. A wrapped document keeps its own clock (row decision/wrapped-document-keeps-own-clock), and the scrollbar fade reads the document's clock (row decision/scrollbar-fade-on-document-clock). Upstream scroll/animation changes must keep that.

@history_item n=28 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§1 coverage: the five new checks, the fixes' checks restated) — the password caret is placed by the mask bullets, a `contain: paint` box stops the hit, and a scrolled or transformed box reads where it stands (`password_caret_click`, `inert_subtree`, `transformed_client_rect`). Rows decision/password-caret-placed-by-mask and scrolled-box-reads-where-it-stands. These are engine surfaces escher changed, which the take must keep.

@history_item n=29 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder) — ratified the clock and caret marks. The rows' "ratified" provenance comes from here.

@history_item n=30 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (test counts and the shot's same-host limit) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no screenshot step (rows decision/no-shot-in-cross-os-byte-checks, shot-png-deterministic-per-host). The committed checks under `tests/blitz-tests/checks/` must still replay after the take.

@history_item n=31 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `HeldNet` replaces the file-private network stand-ins. It has no `is_noop` override, so the harness counts its loads (row decision/network-stand-ins-counted). Upstream's net or NetHandler changes reach these stand-ins.

@history_item n=32 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — a sync's tallies: upstream's targets each their own, `all.rs` gaining `mod` lines, per-OS re-read on the green run with the moves of earlier unwritten chunks attributed per target from the job logs. Precedent for this take's tally re-read.

@history_item n=33 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`hit_test_dropped_nodes`, the counts it moves, and the gaps the take leaves) — the take of «Десять коммитов» (cherry-pick, pins unchanged) brought an upstream check, which was read red with its engine hunks reversed. The gaps the take left (`PARALLEL_TRAVERSAL_LOCK` etc.) were stated, and a stale target count was corrected. This is the shape precedent the scope names for the founder's P4 choice.

@history_item n=34 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the `keyboard_compat.rs` exclusion re-read) — the exclusion now lasts "until escher takes upstream's retirement of it" (`286874cc1`, Dioxus 0.8.0-alpha.3), owned by the left-out delta's residual. Each sync re-reads it.

@history_item n=35 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` runs ten named targets beside the `stand_` files) — non-`stand_` witnesses enter the leg by a fixed list of `--test` words pinned by `test_stand_repeat_leg_runs_three_rounds`. Upstream targets the take brings join no such list unless named.

@history_item n=36 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg names fourteen `--test` words (row decision/a11y-leg-membership; a file joining keeps its legs, decision/joining-file-keeps-its-legs). A per-job tally strips ANSI codes written both as ESC and as a literal `^[` (row decision/ci-log-ansi-strip), which is the method for this chunk's tally re-read. The a11y leg must be green after the take.

@history_item n=37 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages' boots and checks, `LoadNet` and the host-sink part read) — every stand boot but `boot_load` is offline, and `LoadNet` answers a fixed sheet after `LOAD_ANSWER_DELAY` (rows decision/stand-boots-offline, stand-load-page-fixed-answer, contract/stand-boot/boot-load). `host_log` reads the paged Cells part at `trace` with 0 needles, which is part of the sink re-check.

@history_item n=38 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (upstream's `pseudo_element_flow_classification` target and the legs that reach it) — the «Одиннадцать коммитов» take. An upstream target with no `stand_` prefix reaches the workspace `test` leg and `run all`, but not stand-repeat, `run stand` or a11y. The directory counts were re-read, which is the template for recording each new upstream target.

@history_item n=39 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the per-OS and local workspace tallies re-read after the take) — +1 line and +8 passed on each test job, every move named by the take's tests. This is the precedent for "each moved count explained".

@history_item n=40 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the flow-classification file an exception to the both-layout-modes rule) — upstream's file asserted neither mode pair, so it was first recorded as an exception and later made to loop. Upstream's new tests in this take may need the same (row decision/layout-tests-in-both-layout-modes).

@history_item n=41 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script tests the take added) — a `file:` URL is loaded only for a `file:` document, `FetchError` otherwise (row decision/file-urls-only-for-file-documents), and CSS property accessors were added. Script-engine surfaces from upstream are read for what they carry.

@history_item n=42 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations re-pointed past the take's added lines) — cited ranges in merged files (`mutator.rs`, `node/element.rs`, `tests/dom.rs`, `dioxus_document.rs`, `all.rs`) were re-pointed, and ranges stretched over new tests had their counts corrected. This take's wrap will need the same citation sweep.

@history_item n=43 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `keyboard_compat` coverage exclusion re-read and its owner moved) — `286874cc1` was left out again, the gate entry read 2, and the owner moved to the left-out delta's residual.

@history_item n=44 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (dioxus-native-dom's IME test and an upstream test absent; `:open` recorded as untested; the redraw route with no check) — the precedent for stating what the take brings untested, and upstream tests dropped with their commit, as gaps rather than fixes.

@history_item n=45 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the agent-run Proof re-counted after the take) — `run all` picks up a new upstream target with no script change, and `run stand` stays unmoved. Re-counted at every take.

@history_item n=46 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand-repeat` names fifteen targets) — five targets added to the named `--test` list and pinned by `test_stand_repeat_leg_runs_three_rounds`, among them `hit_non_invertible_transform`. The stand-repeat rows rest on this list.

@history_item n=47 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (every cargo leg `--locked` but `fmt` too) — row decision/cargo-legs-locked's exceptions are `fmt` and `examples/wasm_hello`.

@history_item n=48 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`pseudo_element_flow_classification` runs both layout modes) — the fork edited upstream's test file to loop `for incremental in [false, true]`. That is a non-additive edit to upstream test code, and a merge surface for this take.

@history_item n=49 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the free-disk step of `Stand checks repeated`, and the red before it) — the step's `jlumbroso/free-disk-space@…` pin, unproven as the cure, and linker bus errors at 5 MB free. It bears on the take's stand-repeat run and on the action pins.

@history_item n=50 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (CI tallies re-read at the final HEAD; the a11y leg reads 14 · 112) — a per-OS and a11y tally line with each move named. These are earlier readings the take's re-read extends.

@history_item n=51 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — `hit_non_invertible_transform`: `Node::hit_inner` answers `None` under a non-invertible transform. `0980cce5f`'s `hit_inner` hunk stays a textual conflict with this answer (`cry-lurql7hp`). The take also brings `563d3db73` "Resolve transforms through anonymous block boxes", which this check guards.

@history_item n=52 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (what `select_interaction` pins, the chosen-row pair) — the chosen select row reads `#c8c8c8` on `#000000`, Chromium's inactive pair, 12.56:1 (row decision/chosen-select-row-colours). The select model and its list are listed conflict surfaces.

@history_item n=53 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`change` on a checkbox and a radio, from the engine) — the engine fires `change`, and the script runtime synthesizes none (row decision/change-event-from-engine-only). Upstream script or event changes must keep it.

@history_item n=54 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the macOS-only red and `blitz-paint/font-embolden`'s local witness) — no local gate built the font-embolden feature, so a macOS-only red slipped through. The witness `select_popup_painted` under the feature reproduced it (row decision/embolden-local-witness). Upstream paint changes are read under that leg.

@history_item n=55 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the key press `select_popup` skips and the panic no check reads) — the `position_in_containing_block` panic after a dropped anonymous block, owned by a CARRY on the sync. It was later retired by `deallocate_anonymous_block`'s fix (see 2026-11-01).

@history_item n=56 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the stand-repeat leg, seventeen named words; per-OS tallies on CI#41235503227) — the named list is at seventeen, and the stand-repeat tally reading the take's re-read extends.

@history_item n=57 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the list leaves unmeasured; `scroll_into_view` on a row) — the list paints through `paint_scene` alone, with no shell code (row decision/select-list-painted-by-paint-scene), and `scroll_into_view` on a row moves the list's `scroll_y` alone (row contract/harness-input/scroll-into-view). Upstream paint or scroll changes must keep both.

@history_item n=58 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait) — `f6edb9701`'s fix stopped at `CountedHandler` because `NetHandler::error`'s default compiled silently through the wrapper. Rule: a take that extends a trait sweeps that trait's implementers. This is a trap for this take's trait changes, for example AccessKit, net and the Dioxus bridge.

@history_item n=59 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` reads fmt → clippy → test → ci-scripts → embolden, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (rows decision/local-pre-push-gate, embolden-leg-local-only). This chunk's proof includes the `embolden` witness.

@history_item n=60 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired and its check) — `BaseDocument::deallocate_anonymous_block` sets `layout_parent` to `None`, and `key_press_after_container_lost_its_box` covers it in both modes (row decision/key-after-dropped-anonymous-block). This is the merge surface `cry-lurql7hp` that P3 reads against `563d3db73`.

@history_item n=61 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the matrix job's free-disk step gone) — `matrix_test` runs no free-disk step, pinned by `test_only_stand_repeat_frees_disk`. The CI-scripts pins the action-pinning widening sits beside.

@history_item n=62 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the workspace tally and `run all`; citations re-pointed after the take) — the last sync's per-OS tallies (CI#41333172440) and citation sweep, the base this take's re-read starts from.

@history_item n=63 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `url_search_params` target; `render_blocking_stylesheet`; blitz-net's unit tests and test-only loopback binds; blitz-dom's unit tests from the take) — the precedent for inventorying upstream's arriving tests. blitz-net's two test-only `TcpListener::bind` on loopback were recorded as served by no binary, which is how a take's new listeners in tests are recorded.

@history_item n=64 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `keyboard_compat.rs` exclusion through this take; the coverage readings) — 36 of 57 were taken and 21 left out, `286874cc1` among them, so the exclusion stands, owned by the determinism-and-exploration entry. Coverage was re-read after the take.

@history_item n=65 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (blending and the inline box's paint skip read on no frame) — `mix-blend-mode`/`isolation` paint with no frame test, held only by 3 `layers.rs` unit tests, and `render_element`'s inline-decoration skip is held only by 4 unit tests. The owner is a CARRY on this chunk (`cry-jbkftesv`): two headless frame tests, each in both layout modes.

@history_item n=66 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (upstream's commits with no test here) — four upstream commits have no test of their own, with `incremental_oracle` their witness. `element_removed_before_its_mounted_event_is_flushed` was left out, so the `6ef4b532d` guard has no test (row decision/upstream-mounted-flush-test-left-out).

@history_item n=67 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`text_contrast` past a blend) — `text_contrast` reads `None` past a non-`normal` `mix-blend-mode`, and the lint reads the same with no second formula (rows contract/harness-contrast/blend-mode, decision/one-contrast-formula). New paint features in the take (text-shadow, masks) may need the same treatment.

@history_item n=68 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the CI scripts' count) — the `ci-scripts` leg is at 110 tests and test_ci_workflows.py at 35. Widening the SHA-pin guard past ci.yml moves this count (row decision/ci-scripts-unit-tested).

@history_item n=69 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — recorded `mutation_writer.rs` as byte-identical to upstream's at `434e0661`. If the take touches it, the fork's create_text_node test and the buffered render (which replays through `MutationWriter`) meet it.

@history_item n=70 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the red Dependency audit and its green re-read) — the audit turned red on an unchanged graph from new advisories. It was fixed by a semver-compatible bump plus an ignore by ID, on the operator's directive, with the cause and its reproduction pointed to security-plan. This is the direct precedent for this chunk's RUSTSEC-2026-0459 red.

@history_item n=71 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the dioxus_runtime_probe target, its legs and its pages; the MountedData hazard re-measured) — the probe reads Dioxus 0.7.10 public API with no fork-level change, and measures listeners, readers and the hazard. The scope names it among the checks that decide for dioxus-native-dom. A Dioxus 0.8 move would change what it reads.

@history_item n=72 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed to residual rows, among them `res-6ssktw6x` (the left-out delta), which this chunk owns through `cry-mjwr45xr`.

@history_item n=73 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the fix's witness on Dioxus 0.7, the probe's two pins) — the probe's `…_spawned_acts_*` pins are the discriminating witness. Upstream's ported `task_polled_during_render_can_access_document` is insensitive on 0.7 and kept as written (rows decision/buffered-render-on-dioxus-07-bridge, upstream-buffered-render-test-kept-as-written). This is the buffered render's merge surface, `cry-246ouqff`.

@history_item n=74 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the per-OS CI tallies re-read, and attributing a tally move) — in a job log, stderr `Running` lines and stdout result lines do not interleave in order, so a move is attributed by grepping the test's name (row decision/tally-move-attributed-by-test-name). This is the method for this chunk's tally re-read.

@history_item n=75 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the MountedData hazard closed; dioxus-native-dom's unit tests and the mutation buffer; coverage and `mutation_buffer.rs`) — `initial_build` and `poll` render into the crate-private `MutationBuffer` with the document unborrowed (row decision/buffered-render-unborrowed-document). `mutation_buffer.rs` is counted inside the 83 % floor. Upstream's 0.8 `mutation_buffer.rs` replaces it whole if the 0.8 move is taken.

@history_item n=76 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (upstream's mounted-event test stays out; residual owners re-said) — `element_removed_before_its_mounted_event_is_flushed` returns only with the Dioxus 0.8 move, owned by `res-6ssktw6x` through `cry-mjwr45xr` on this chunk (CARRY `cry-jlxpxul2`). The `keyboard_compat.rs` exclusion has the same owner.

@history_item n=77 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit scope and its one test-only shim; the stand's state reaches the UI graph through the fork's registry) — the one `cfg(test)` shim `DioxusDocument::dispatch_to_vdom`. The stand registers signals through `use_graph_state` with no dioxus-core change (rows decision/bridge-tests-without-widening, graph-signal-registration). `stand_graph` decides the take's resolutions in dioxus-native-dom.

@history_item n=78 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unconfirmed flag's stated limit; the graph verb held in session) — a conditional write unrun after its event reads `unconfirmed`, and `GRAPH_DRIVES` drives each one once. A graph answer holds no value, and no served id holds `{owner}///{key}` (rows decision/unrun-declared-write-reads-unconfirmed, graph-drives-confirm-every-write, graph-answer-holds-no-value, reserved-graph-id-form).

@history_item n=79 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (dioxus-native-dom's actionable reading, one rule for two readers; stand_actions) — both actionable readers read the exact current listeners, never `data-dioxus-id`, which outlives a removed listener (rows decision/one-actionable-rule, listeners-read-from-vdom-walk). `harness.click` waits on no load (row decision/harness-click-waits-on-nothing). Bridge or listener changes in the take meet these.

@history_item n=80 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the legs with stand_actions, the baseline re-count and the coverage floors) — the local baseline was re-counted at 223 result lines with passed/ignored not read, and the per-crate coverage figures were not read. The take's re-read starts from these partial readings.

@history_item n=81 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage and its tables; handler coverage's stated limits; the session binary's coverage role) — handler coverage by replay, with the bubbling, canvas and unreached-screen limits (rows decision/handler-coverage-by-replay, contract/handler-coverage-limits/*, contract/coverage-line/*, contract/coverage-role/unreadable-check). `HANDLERS_AT_BOOT` is held equal to the graph's reading, so a take that moves listeners moves these tables.

@history_item n=82 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (host_log's coverage run; the stand reading with stand_handler_coverage) — `host_log`'s coverage run reads 0 needles at `trace` under both builds, part of the sink re-check. `run stand` reads 262 · 0 · 7 over 54 files, and the stand-repeat leg runs 54 files: the readings the take's re-run compares against.

@history_item n=83 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master text became rows (601: decision 272, contract 326), each verified against the code. Every picked row's handle comes from this conversion, and a take that changes a guarded behaviour amends the row, not prose.

@history_item n=84 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/handler-coverage-reads-every-replayed-screen`: the state moved from provisional to standing on the founder's ratification. It is the most recent amendment to a picked row.
