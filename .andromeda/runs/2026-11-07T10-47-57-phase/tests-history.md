@history_item n=1 marker=amd-3llbasfo
**amd-3llbasfo** — 2026-11-04-dioxus-runtime-probe: §6 records the MountedData hazard's mechanism re-measured at HEAD (`poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, dioxus-core 0.7.10 polls the queued task there, `set_focus` borrows through `doc_mut` and panics). It adds the two `should_panic` pins `mounted_data_in_a_task_a_key_handler_spawned_panics_{non_incremental,incremental}`, with `hazard_ready` asserted first and the synchronous sibling `mounted_data_asked_at_once_in_a_key_handler_acts`, and routes the fix to «MountedData buffered render», with `task_polled_during_render_can_access_document` as its witness. This is the entry the chunk discharges: it must restate the mechanism as fixed and invert both pins, precondition still first.

@history_item n=2 marker=amd-hqsdue54
**amd-hqsdue54** — 2026-11-04-dioxus-runtime-probe: records the legs that reach `dioxus_runtime_probe`. Those are the workspace `test` leg, `run all` and `run dioxus_runtime_probe`. The target has no `stand_` prefix and is none of stand-repeat's seventeen words or the a11y leg's fourteen. The entry also adds the §7 pages row (`toggled_listener`, `shared_state`, `hazard(spawned)`) and the §1 row of 10 tests, 8 looping both modes and 2 `should_panic`. The §1, §7 and §9 rows that state the `should_panic` pins must all be restated together. Trap: its Kept says the per-OS tallies were NOT re-read from CI#41914044942, so this chunk's tally re-read has to attribute that gap.

@history_item n=3 marker=amd-q5fbireg
**amd-q5fbireg** — 2026-11-04-dioxus-runtime-probe: `run all` reads 743 · 0 · 12 with `dioxus_runtime_probe` picked up with no script change, and `run stand` stays at 237 · 0 · 7 over 51 files. blitz-tests holds 158 `.rs` files and 157 targets. These are the baselines this chunk's agent-run proof is read against. Inverting the pins changes their outcome, not the count, and the ported unit test lands in dioxus-native-dom, not blitz-tests.

@history_item n=4 marker=amd-2ho3wh7t
**amd-2ho3wh7t** — 2026-11-04-dioxus-runtime-probe: pins link 2 (`link_2_a_removed_listeners_attribute_stays`: a removed listener's attribute stays, `remove_event_listener` reached only on a spread) and links 3 and 4 (`links_3_and_4_a_held_signals_subscribers_are_its_readers`), both layout modes. These are among the probe's other eight tests that must stay green over the buffered replay, so the buffer must replay listener creation and removal into `MutationWriter` exactly as the direct write did.

@history_item n=5 marker=amd-uh6brrox
**amd-uh6brrox** — 2026-10-31-engine-debts: first recorded the `RefCell already borrowed` panic at `events.rs:162` for a `MountedData` call in a key-handler-spawned task. Cells keeps its handle calls synchronous because the async form was tried, panicked and did not ship. It adds `stand_cells_served`'s keyboard walk (5 tests), which drives the same Tab/key path the inverted pins take, so it must stay green over the buffered poll. Precedent: this is where a fix enables the async form.

@history_item n=6 marker=amd-wbtnfb6d
**amd-wbtnfb6d** — 2026-10-27-upstream-sync-engine-debts: records that `task_polled_during_render_can_access_document` (`286874cc1` not taken) has no counterpart in the fork's `dioxus_document.rs` test module, and sets dioxus-native-dom at 82 unit tests in nine files. Upstream's `input_focused_on_mount_enables_ime` sits in the same module. The chunk ports the absent test into that module and moves the count 82 → 83.

@history_item n=7 marker=amd-qbevnqg6
**amd-qbevnqg6** — 2026-11-01-upstream-sync-ui-graph: upstream's `element_removed_before_its_mounted_event_is_flushed` is left out because it reads 1 mounted event on Dioxus 0.7 where it asserts 0. It returns with `286874cc1`, which leaves the `6ef4b532d` guard in `flush_queued_mounted_events` without a test of its own. Porting `286874cc1`'s buffered render reopens this test, so the plan must decide whether to port it and what it asserts, or record why it stays out.

@history_item n=8 marker=amd-le75xddb
**amd-le75xddb** — 2026-10-16-fast-failing-mutants: dioxus-native-dom 68 → 69, with seven tests on the bridge. The new test pins a text node that `WriteMutations::create_text_node` creates, mapped to its element id and left on the writer's stack. It also states `mutation_writer.rs` byte-identical to upstream's at `434e0661`. This is the source of the byte-identity the scope keeps; if the buffer lives on `DioxusState`, the claim breaks and the change must be named with its reason.

@history_item n=9 marker=amd-w5bh6ujd
**amd-w5bh6ujd** — 2026-10-13-upstream-sync-polish-and-ship: dioxus-native-dom 61 → 67, adding upstream's four drop tests in `dioxus_document.rs`'s module and two in a new `mutation_writer::tests`. The §3 helper was restated: a re-render now drops the node (#1185), and `a_removed_element_reads_no_id` asserts that it no longer resolves, reads no id and is in no pair of `element_ids()`. These element-id and writer tests must stay green unchanged over the buffered replay.

@history_item n=10 marker=amd-ur7tfp4h
**amd-ur7tfp4h** — 2026-10-07-audit-corrections: puts the six bridge tests in the `cfg(test)` child module `dioxus_document::bridge_tests` (`dioxus_document_tests.rs`, via `#[path]`). They sit there because `DioxusEventHandler`'s fields are private and it has no constructor, so the tests build it over `doc.vdom` and `doc.vdom_state`. This is the precedent for the no-shim, test-path-unchanged rule. A buffer living on `vdom_state` changes what these tests construct over, so they must be re-read.

@history_item n=11 marker=amd-6bfrgkxn
**amd-6bfrgkxn** — 2026-10-06-stable-element-ids: the `element_id` unit tests build a `DioxusDocument` from a plain app and re-render through a `GlobalSignal` inside `vdom.in_runtime`. This is the harness shape the ported witness follows: `DocumentConfig::default()`, `initial_build`, then a re-render that spawns a task, through `mark_dirty` + `poll(None)`.

@history_item n=12 marker=amd-tunjc5rm
**amd-tunjc5rm** — 2026-10-06-snapshot-model: the snapshot unit tests call `resolve(0.0)` through `inner_mut()` after `initial_build`, the crate-local helper the witness's `get_client_rect` read may need. The entry also sets the standing practice that each count is appended as a dated link with earlier links kept as history, which governs how this chunk moves the 82 count.

@history_item n=13 marker=amd-dvdqody5
**amd-dvdqody5** — 2026-10-16-file-splits: records the out-of-line `#[path]` unit-test modules, each declared `#[cfg(test)]` with `#[path]` at its parent and moved out whole with test paths unchanged. This is the rule the extract cites for keeping the ported test inline in `mod tests`, or in a `#[path]` child with an unchanged path.

@history_item n=14 marker=amd-n5xqwtgz
**amd-n5xqwtgz** — 2026-10-14-quality-gates: sets the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the stand checks' zero-failure flakiness budget (stand-repeat three rounds, no retry), on the founder's word. The new buffer module counts toward the 83 % floor, and the stand checks re-run over the buffered render carry the zero-failure budget.

@history_item n=15 marker=amd-h4ip3qlm
**amd-h4ip3qlm** — 2026-10-19-test-splits: a `*_tests.rs` `#[path]` file is no record of the per-crate coverage reading, so moving a test module out of line moves its crate's reading. Escher-driver's reading fell this way, and the entry says the next split should forecast a change, not none. If the ported witness or a buffer test sits in a `#[path]` file, the dioxus-native-dom reading moves.

@history_item n=16 marker=amd-dpfqyxgg
**amd-dpfqyxgg** — 2026-11-01-upstream-sync-ui-graph: dioxus-native-dom reads 2791/3241 = 86.12 % at CI#41333172440, inferred from counts and not from the lcov, with 3 of 3 crates at or above their floors. This is the current reading the buffer module's added lines move against the 83 % floor.

@history_item n=17 marker=amd-xpqvmmzd
**amd-xpqvmmzd** — 2026-10-17-upstream-sync-engine-debts: both `cargo llvm-cov` lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`, excluding upstream's keyboard-types bridge until upstream retires it. The floors and `coverage_floor.py` are unchanged, pinned by `test_coverage_leg_reaches_the_floor_on_each_crate`. The chunk ports `286874cc1`'s buffered render but not its `keyboard_compat.rs` removal, so both lines must stand.

@history_item n=18 marker=amd-hayhbwsb
**amd-hayhbwsb** — 2026-10-24-upstream-sync-polish-and-ship: restates the exclusion as lasting «until escher takes upstream's retirement of it». Upstream retires `keyboard_compat.rs` in `286874cc1` (Dioxus 0.8.0-alpha.3), the very commit this chunk partly ports. The partial port must say plainly that the retirement is not taken and the exclusion stays.

@history_item n=19 marker=amd-vrpzudrd
**amd-vrpzudrd** — 2026-11-01-upstream-sync-ui-graph: the take of `e42ee1f25` leaves `286874cc1` out, so `keyboard_compat.rs` and the exclusion stand. The exclusion's owner moves to «Upstream sync ahead of determinism and exploration». This chunk takes part of `286874cc1` outside a sync, so that owner and the exclusion stay as they are.

@history_item n=20 marker=amd-s53lzeiq
**amd-s53lzeiq** — 2026-10-07-driver-command-spans: adds §9 «Engine features by runner». `cargo test --workspace` compiles blitz-dom's `tracing` call sites in, and `-p blitz-tests` does not. A check reading a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green. This is the source of the rule that the witness and the inverted pins must be read green under both builds.

@history_item n=21 marker=amd-cqdq6a7d
**amd-cqdq6a7d** — 2026-10-10-upstream-sync-agent-surfaces: sharpens the feature split. `writing-mode` is on only in the workspace build and selects which body of the bounds reader is compiled. The witness's `get_client_rect` and the stand checks therefore read different code under each runner.

@history_item n=22 marker=amd-bjdcpa2t
**amd-bjdcpa2t** — 2026-11-01-upstream-sync-ui-graph: the per-OS tallies on CI#41333172440 at `45c333f16` read linux 219 · 1299, macos and coverage 200 · 1296, windows 200 · 1248, with a11y 14 · 114 and stand repeat 228 · 1155 unmoved. The local baseline is 219 · 1299 · 0 · 12, and `run all` read 733. This is the last per-OS reading. The ported unit test adds +1 passed in dioxus-native-dom's lib line on linux, macos, coverage and windows. The probe's 10 tests from the previous chunk were never attributed in the tallies.

@history_item n=23 marker=amd-jef7zjyx
**amd-jef7zjyx** — 2026-10-18-upstream-sync-determinism-and-traces: sets a rule for later chunks: a wrap that does not re-read the tallies leaves the next chunk to attribute the gap. The skipped reading is named in the body, with each move attributed per target. This applies because dioxus-runtime-probe skipped its re-read (amd-hqsdue54).

@history_item n=24 marker=amd-kube42bu
**amd-kube42bu** — 2026-10-31-select-popup: stand-repeat runs one `--test` per `stand_*.rs` (51 files) and seventeen named words, pinned by `test_stand_repeat_leg_runs_three_rounds`. `dioxus_runtime_probe` is none of the seventeen, so stand-repeat never runs the inverted pins. Every stand check it runs boots a `DioxusDocument` through the new buffer.

@history_item n=25 marker=amd-zmnln6ug
**amd-zmnln6ug** — 2026-10-31-select-popup: `run stand` reads 237 · 0 · 7 over 51 files with `scripts/agent-run.sh` unchanged. This is the stand reading the chunk's 5-command proof must reproduce.

@history_item n=26 marker=amd-2xzkhreo
**amd-2xzkhreo** — 2026-10-06-stand-test-contract: sets the agent-run 5-command contract (`boot | run {selection} | status | cleanup | logs`). It covers the exit grammar, the `run.end` event, the `stand` · `all` · `{name}` selections and the state under `target/agent-run/`. This is the discipline the tests ↔ obs harness binding runs, `run dioxus_runtime_probe` being a `{name}` selection.

@history_item n=27 marker=amd-jhx4wjqd
**amd-jhx4wjqd** — 2026-10-07-settle-detection: `settle` repeats pump's pass and keeps poll's answer. A pass is quiet when poll answered false, the hover node is unchanged and no load finished. `Render`, `Loads` or `Layout` after 64 passes, and pump and the input helpers are unchanged. The buffered `poll` must return the same answer, or `stand_settle` and every settled step's reading moves.

@history_item n=28 marker=amd-2gzx73ga
**amd-2gzx73ga** — 2026-10-06-headless-stand: makes the stand boot `seven_guis::stand::{boot, boot_timer}` with `stand::options(incremental)` (HarnessOptions `font_ctx`, `incremental`) the stand checks' drive surface. Every stand check, and the Harness wrapping `DioxusDocument` through `from_vdom`, reaches the buffered `initial_build`/`poll` this way.

@history_item n=29 marker=amd-wb5mdgi4
**amd-wb5mdgi4** — 2026-10-31-engine-debts: ends upstream's both-modes exception for `pseudo_element_flow_classification`, and `incremental_oracle` stays the layout-identity witness. This holds the rule that pipeline tests run `for incremental in [false, true]`, which the inverted pins and the witness checks must follow, with `incremental_oracle` the witness that the buffer leaves both layout modes identical.

@history_item n=30 marker=amd-ce7fplwt
**amd-ce7fplwt** — 2026-10-27-upstream-sync-engine-debts: precedent for the §9 «Legs reaching `{target}`» record of a target with no `stand_` prefix: the workspace `test` leg and `run all` with no script line naming it, and outside stand-repeat, `run stand` and the a11y leg. `dioxus_runtime_probe`'s record has the same shape and is restated in place when the pins invert.

@history_item n=31 marker=amd-a42cryfq
**amd-a42cryfq** — 2026-10-27-upstream-sync-engine-debts: re-points the cited line ranges into `packages/dioxus-native-dom/src/dioxus_document.rs`, stretching the test-module range over `input_focused_on_mount_enables_ime`. The chunk rewrites `initial_build` (`:161–167`) and `poll` (`:295–300`) and adds a test to that module. Those citations, including the hazard's `:295`/`:297` in §6, must be re-pointed the same way.

@history_item n=32 marker=amd-s4wow4l3
**amd-s4wow4l3** — 2026-10-07-sink-target-allowlist: adds `host_log`, which runs the real `escher-session` host at `RUST_LOG=trace` with needles read from the stand and both streams drained. It asserts no id and no name on stderr, and says both binaries read none at any level. This is the no-id, no-name standing check for the tests ↔ obs binding, and the buffer's ids, names and text must never reach it.

@history_item n=33 marker=amd-sam3o45d
**amd-sam3o45d** — 2026-10-25-cold-agent-bar: `host_log` 4 serves `cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell and reads `snapshot --part 2`, with 0 id and 0 name needles under both builds. This is one of the log checks carried for the buffered render. They must still read 0 needles.

@history_item n=34 marker=amd-r5npmhk4
**amd-r5npmhk4** — 2026-10-31-select-popup: `host_log` reads 5 and `mcp_log` 3. The new tests `a_select_list_opened_and_chosen_through_the_{host,role}_logs_no_option_at_trace` read 0 option needles at `trace`, and `option_needles()` is added to the shared module. These are the two log checks the binding names, which must stay green over the buffer.

@history_item n=35 marker=amd-lrluctc2
**amd-lrluctc2** — 2026-10-07-driver-session: adds `stand_session_quiet`, which proves the session library and harness path write nothing of the screen to stdout, stderr or the state directory. Traps: an `#[ignore]` host child blocks a bare `--ignored` run, and a child's inherited libtest lines are counted by `agent-run.sh`. This is the witness that the buffer's mutations never reach an output stream.

@history_item n=36 marker=amd-ieh35jjn
**amd-ieh35jjn** — 2026-10-10-driver-cli: the session host's stderr by level under the workspace build reads one engine line at INFO and none more at `debug` or `trace`. That is the «one engine line more» clause in the carried log-line fact, which the buffered render must not move.

@history_item n=37 marker=amd-7qex4fkt
**amd-7qex4fkt** — 2026-10-19-recorded-check-replay: `stand_replay` replays the committed checks through `Check::replay`, every step through `Session::run`, and is red at the first differing acceptance or kept answer. The checks hold ids, snapshot text and diffs. Over the buffered render each recorded step's answer must stay byte-equal, or `stand_replay` and seven_guis `recording` go red.

@history_item n=38 marker=amd-hzideodq
**amd-hzideodq** — 2026-10-06-id-persistence: stand coverage gains the same id across a re-render, a remount (fresh `NodeId`s bar the four skeleton elements) and a fresh process (`stand_id_persistence`). The buffered replay must keep element ids stable across these three cases.

@history_item n=39 marker=amd-6ku7k52f
**amd-6ku7k52f** — 2026-10-31-engine-debts: makes Cells' grid one Tab stop. `stand_keyboard` 21 covers Cells' Tab sequence, arrows, and Enter opening and committing. `stand_contrast` adds `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`. These are carried stand witnesses over the same Tab/key path the inverted pins drive, and they must stay green over the buffered poll.

@history_item n=40 marker=amd-63o3ty3w
**amd-63o3ty3w** — 2026-10-31-engine-debts: `stand_act_transformed` 3 adds `a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows`, in both layout modes. This is a carried stand witness re-run over the buffered render.

@history_item n=41 marker=amd-dqo5clox
**amd-dqo5clox** — 2026-10-31-select-popup: `stand_act_popup` holds 12 tests, each in both layout modes, driving the select list through a held session. Run by `run stand` and stand-repeat by prefix, it is a carried stand witness whose readings must not move under the buffer.

@history_item n=42 marker=amd-glk62rg7
**amd-glk62rg7** — 2026-10-31-select-popup: `stand_lint` 9 gains `an_open_lists_rows_read_no_overlap_on_the_stand`, and the lint pins the stand at `[0, 0, 0, 0]` at boot. This is a carried stand witness, whose readings must stay unchanged over the buffered render.

@history_item n=43 marker=amd-wzxmumz2
**amd-wzxmumz2** — 2026-10-25-cold-agent-bar: re-pins the stand's tables to the flight booker's select (`flight-type` a `Role::ComboBox`, exactly the chosen option `selected`). `stand_snapshot`'s `the_flight_type_select_holds_its_two_options` reads it, and that snapshot witness must read identically after the buffered replay.

@history_item n=44 marker=amd-myqerkgx
**amd-myqerkgx** — 2026-10-06-snapshot-state-fidelity: a trap for later chunks. A headless key press is not the same on every platform: the editor's Backspace arm is compiled out on macOS, so a deleting-key check read green on Linux and red on macOS CI. Prove a state change with clicks and typed characters. This bears on the keys the inverted pins and the witness press.
