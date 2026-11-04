@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** (stand checks, HarnessOptions fields, bundled font) — made `seven_guis::stand::{boot, boot_timer}` with `stand::options(incremental)` the stand checks' drive surface, added `incremental` and `font_ctx` to HarnessOptions, and made stand checks assert with no condition under the bundled DejaVu Sans with system fonts off, with no skip path. This is the boot and font rule every probe run on the stand rests on.
@history_item n=2 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** (the agent-run 5-command contract) — set the `agent-run.sh` selections (`stand` · `all` · `{name}` matching `^[a-z0-9_]+$` with an existing file), the exit grammar and the `run.end` event. A kept probe check is reached only through these selections, and `run stand` only with the `stand_` prefix.
@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** (element_id unit tests) — recorded the `element_id` unit-test helper: build a `DioxusDocument` from a plain app and re-render through a `GlobalSignal` flipped in `vdom.in_runtime`. Added `stand_element_ids`, one stable id per element in both layout modes. This is the template for a bridge-level probe of link (1) and of a signal-driven re-render.
@history_item n=4 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — the stand keeps the same id across a re-render, a remount (fresh `NodeId`s except the four document-skeleton elements) and a fresh process. Link (1)'s node→component reading must go through the stable id, never a `NodeId` that a remount refreshes.
@history_item n=5 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** (dioxus-native-dom unit tests re-counted) — the `snapshot` unit tests call `resolve(0.0)` through `inner_mut()` after `initial_build`. Standing practice: each count is appended as a dated link and earlier links are kept as history. A unit-level probe that reads layout must resolve first, and a kept probe re-counts the crate in the same way.
@history_item n=6 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added `actionable.rs` (`unkeyed_actionable`, four unit tests) and the anchored-tier element-id tests (thirteen on the id). These are the bridge-held reads that links (1) and (2) must weigh before giving an «exposed» verdict.
@history_item n=7 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — recorded that the unit `build` helper carries the no-op HTML parser, so `dangerous_inner_html` renders nothing at unit level and parsed markup is pinned in blitz-tests. Trap: a headless key press is not the same on every platform (the editor's Backspace arm is compiled out on macOS). A key-driven probe, the MountedData path among them, must not rest on a deleting key.
@history_item n=8 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — added `Snapshot::diff` and `stand_diff`: real steps on the lean tasks in both layout modes, each step held against the file's own reading of the two snapshots. This is the standing ground truth (click on `counter-increment` naming `counter-value`) for the handler→state and state→node verdicts of links (3) and (4).
@history_item n=9 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (bridge tests, the shared stand module) — created the `cfg(test)` child module `dioxus_document::bridge_tests` in `dioxus_document_tests.rs`. It is a child of `dioxus_document` because `DioxusEventHandler`'s two fields are private to that module and it has no constructor. It keeps the `EventState` it passes, the one way to read `propagation_is_stopped()`. Also made `tests/common/mod.rs` the one shared module that is no target. This is the precedent and the constraint for any probe that reads a handler from inside the crate, and it is why no `pub` or shim may be added.
@history_item n=10 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — added `settle`: pump's pass repeated, with no clock read, no time advanced and the changed set neither read nor drained. Traps: a tick delivered after an input helper needs a later pass of its own, and the changed set is non-empty on a freshly booted harness, so a check that reads the flag drains first. Both bind a probe that reads which nodes changed (link (4)).
@history_item n=11 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — set the assertion discipline: every assertion over a call carries a row index only, so a failing log prints no argument value. A kept probe's assertions must not print a component name, a handler or a signal value.
@history_item n=12 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — re-exported `Key` and `Modifiers` from the harness. Rules: a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads, and no deleting key appears in any check but `stand_act_keys`' one. This binds the messages and key choice of a probe's key-handler drive.
@history_item n=13 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (engine features by runner) — the same check file read green per package and red in the workspace leg. A check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green. A probe verdict names its build.
@history_item n=14 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (readings on the merged tree) — sharpened the feature split: `writing-mode` is on in the workspace build only, so the bounds reader compiles a different body per runner. Gave `run all`'s first stated total. A layout-touching probe reading differs by runner.
@history_item n=15 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — measured the stand log format (`{time} {LEVEL} {target} service.name=… {field}=…`) with the allowlist scrub, bound to obs-plan §3. Any stderr a probe run produces falls under it.
@history_item n=16 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — restated the scrub as the three-outcome target allowlist and covered it by `host_log` needles: no id or name on stderr, and a failure prints counts only. A probe adds no log field, and anything it reads must stay off every stream.
@history_item n=17 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This matters if a probe reads a sink capture.
@history_item n=18 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure flakiness budget on the stand checks. Also recorded that no font test skips. These are the floors any probe code kept in dioxus-native-dom or the harness must hold, and the budget a kept `stand_` file joins.
@history_item n=19 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — listed the out-of-line `#[path]` `cfg(test)` modules, `bridge_tests` the first, each declared at its parent with test paths unchanged. A probe placed beside the bridge tests follows this form.
@history_item n=20 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — dioxus-native-dom 68 → 69 unit tests, the bridge tests six → seven (a text node created and mapped). This is the bridge-test count a probe added to `bridge_tests` re-counts.
@history_item n=21 marker=2026-10-19-test-splits
**2026-10-19-test-splits** (a split moves the coverage reading) — a `*_tests.rs` `#[path]` file is no record of the per-crate coverage reading, so a test module moved out of line moves its crate's reading. A later test split should forecast a reading change. A probe kept in `dioxus_document_tests.rs` moves dioxus-native-dom's reading without adding a covered line.
@history_item n=22 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge's unit tests) — dioxus-native-dom 61 → 67. A re-render now drops a removed node (#1185), and `a_removed_element_reads_no_id` asserts it resolves to nothing. A link (1) probe across a re-render reads dropped, not detached, nodes.
@history_item n=23 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the harness re-exports `NamedKey`; test-that) — the harness re-exports keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey`. blitz-tests carries test-that 0.6.0 as the merged checks' assertion crate, the only third-party test framework present. A probe drives keys with the re-exported types and adds no framework.
@history_item n=24 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the coverage leg excludes `keyboard_compat.rs`) — both llvm-cov lines exclude dioxus-native-dom's `keyboard_compat.rs` until upstream retires it, with the floors unchanged. This is the coverage exclusion a dioxus-native-dom probe meets.
@history_item n=25 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the `keyboard_compat.rs` exclusion re-read) — `286874cc1` (Dioxus 0.8.0-alpha.3) retires `keyboard_compat.rs` and escher has not taken it. Taking `286874cc1` as a MountedData fix option implies a Dioxus pin move, which the scope forbids, and it lifts this coverage exclusion. Both are part of that option's cost.
@history_item n=26 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (dioxus-native-dom's IME test and an upstream test absent) — dioxus-native-dom 81 → 82 unit tests. Recorded upstream's `task_polled_during_render_can_access_document` as absent, its commit `286874cc1` not taken. This is the crate count a kept probe re-counts, and the upstream test the MountedData reading weighs as the fix option's witness.
@history_item n=27 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `keyboard_compat` coverage exclusion re-read and its owner moved) — `286874cc1` is again left out, and the exclusion's owner is the left-out-delta residual. This is the history of the fix option the founder's fork would weigh.
@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' keyboard walk, its handle calls and its open editor) — recorded in e2e-test-strategy that Cells calls its `MountedData` handles synchronously, measured headlessly with the windowed path read and not run. A `MountedData` call in a task spawned from a key handler panics `RefCell already borrowed` at `events.rs:162`, and the async form was tried, panicked and did not ship. This is the hazard record P3 re-verifies at HEAD and the reading names as the founder's fork.
@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' grid as one Tab stop) — `cells-grid` took `tabindex` 0, `aria_label` and an `onkeydown`, with no role. `stand_keyboard` reads Cells' Tab, arrows, Enter and commit sequences and the load page's. These are the key-driven handlers on the served task that the probe and the hazard path reproduce.
@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' reacting elements read 677) — the pin moved from elements that react to a click (676) to those that react to input (677: the data cells and `cells-grid`), counted by the listener mark reading `GenericContainer`. This is the pinned «reacts to a click» count the probe's link (2) `click` answer is read against.
@history_item n=31 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (the non-lean pin at 0) — `stand_non_lean_tasks` pins the elements reacting with no role at 0 · 0 · 0 (was 2 · 3 · 676), with the snapshot field for it deferred. The probe's «reacts to a click» answer leaves that field deferred.
@history_item n=32 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand's tables re-pinned to the flight booker's select) — `stand_act_ids`: of the 77 nodes, click accepted on 68 and refused on 9, with 14 controls. This is the standing reading of `element_ids()` and the snapshot ids that link (1)'s probe resolves against.
@history_item n=33 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages' boots, `LoadNet`) — added `stand::boot_cells` and `stand::boot_load` (`net_provider` replaced by `LoadNet`, every other field kept, every other boot offline). `LoadNet` answers after 200 ms from its own thread. `stand_load_page` reads no clock and waits it out through `LOAD_WAIT`. These are the served-task boots the probe uses beside the lean one.
@history_item n=34 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — added the second wall-time exception (`stand_load_wait`'s `Instant::now()` and `LateNet`'s sleep): only a check whose subject is wall time may read it. A probe reads no clock and sleeps nowhere.
@history_item n=35 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the harness's clock) — the harness's time is a `VirtualClock` its documents read. `tick` and `advance_time` move it, and `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never do. `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep. A probe moves time only through these.
@history_item n=36 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder) — the founder ratified the virtual-clock marks, the harness's time never moved by `screenshot` · `text_contrast` · `settle` among them (2026-10-18). This is the authority of the time rule a probe obeys.
@history_item n=37 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — added `tests/fixtures/mod.rs` with `boot(app, incremental)`. Rule: a page two checks share goes in `fixtures`, and a page with one reader stays in its file. Measured the test-data bootstrap: nothing seeded, fixtures compiled in, no `run fixtures` selection. This governs where a probe's fixture pages live.
@history_item n=38 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the shared-module readers and the blitz-tests targets) — `mod fixtures;` is read by 31 checks, counted anchored at line start (an unanchored count reads one more). blitz-tests 156 `.rs` files and 155 targets. A kept probe re-counts these, with the anchoring trap.
@history_item n=39 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`run stand` 237 and `run all` 727) — `run stand` reads 237 · 0 · 7 over 51 files, with `agent-run.sh` unchanged. This is the newest `run stand` reading the chunk's acceptance holds at or above.
@history_item n=40 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the stand-repeat leg, seventeen named words and 51 files) — stand-repeat runs one `--test` per `stand_*.rs` (51) plus seventeen named words. A kept `stand_` probe joins it by prefix alone, and a non-prefixed one joins only by a named word.
@history_item n=41 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the per-OS and stand-repeat tallies) — stand repeat 228 · 1155 · 0 · 21 on CI#41235503227. This is the flakiness-budget reading that a kept `stand_` probe moves.
@history_item n=42 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` runs ten named targets) — the founder chose a fixed list in the script over renaming to `stand_*`. `run stand` keeps its prefix rule, and non-prefixed targets are reached through `run all` and `run {name}` alone. This is the selection precedent for where a probe check sits.
@history_item n=43 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired and its check) — added `key_press_after_container_lost_its_box` (3 tests, both layout modes, no `stand_` prefix, reached by the workspace `test` leg and `run all` only). blitz-tests is at 157 files and 156 targets. This is the nearest precedent for a non-prefixed regression check named after its bug. The key-press path the hazard drives no longer panics at `position_in_containing_block`.
@history_item n=44 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the workspace tally and `run all`) — linux, macos and windows tallies on CI#41333172440, a local baseline of 219 · 1299 · 0 · 12, and `run all` 733 · 0 · 12. `run stand` is unmoved at 237 over 51. These are the newest readings a kept probe target re-counts.
@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (upstream's commits with no test here) — upstream's `element_removed_before_its_mounted_event_is_flushed` is left out (it reads 1 mounted event on Dioxus 0.7 where it asserts 0) and returns with `286874cc1`, so the `6ef4b532d` guard in `flush_queued_mounted_events` has no test. This is part of the `286874cc1` fix option's cost in the MountedData fork.
@history_item n=46 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `keyboard_compat.rs` exclusion through this take) — the take of `e42ee1f25` leaves `286874cc1` out again, and the left-out delta's owner is now «Upstream sync ahead of determinism and exploration». This is where a `286874cc1` fix option would land if the founder sends it to a sync.
@history_item n=47 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the coverage readings) — dioxus-native-dom 2791/3241 = 86.12 % and blitz-test-harness 96.06 %, inferred from counts, 3 of 3 floors held. This is the margin above the 83 % and 90 % floors that probe code kept in those crates spends.
@history_item n=48 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `ci-leg.sh fast` now runs fmt → clippy → test → ci-scripts → embolden. This is the local pre-push gate the chunk's acceptance names.
@history_item n=49 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`pseudo_element_flow_classification` runs both layout modes) — ended the one both-modes exception: pipeline tests run `for incremental in [false, true]`, with `incremental_oracle` the layout-identity witness. A probe reading that layout touches runs both modes.
