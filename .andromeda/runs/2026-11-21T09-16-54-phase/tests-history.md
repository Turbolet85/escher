@history_item n=1 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus-fixup limit ends in `inert_subtree`) — `inert_subtree` 4 → 6: a node coming under `inert` gives focus up with no `blur`/`focusout` (the mutator's flush clears it), over five ways of being made inert, and the tree writes `focus` only where the node was built, else the window. This is the exact focus fixup item 1 (res-6k56erh6) would extend from `is_inert()` to a control that no longer renders: the new check sits beside `inert_subtree`, keeps its no-blur/no-focusout shape, and the AT reading stays unmeasured.

@history_item n=2 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired and its check) — added `key_press_after_container_lost_its_box` (3 tests, both layout modes, for `hidden`, `style="display:none"` and CSS alone): a key press on a focused control whose anonymous block was dropped no longer panics, focus stays and the key still acts (a select's choice still moves). The founder's F1 A there kept focus on a hidden control, so this check pins the behaviour item 1 would change. A fix that gives focus up rewrites its "focus stays" assertions, and the picked row decision/key-after-dropped-anonymous-block has to be amended. Legs: workspace `test` and `run all` only, not `stand-repeat`, `run stand` or `a11y`. Its `format!` pages (`in_anonymous_block`, `in_no_anonymous_block`, `Lost`, `boot`) are the fixture precedent.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the key press `select_popup` skips) — first measured the panic at `position_in_containing_block` for any focused control once an ancestor gains `hidden`, a path a remote page can reach. It is the origin of the hidden-focused-control path that n=2 closed, and the reason `select_popup` now presses the key in every row.

@history_item n=4 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§1 coverage) — the earlier engine-debts chunk added `inert_subtree` 4 and restated `stand_keyboard`'s `disabled`/`inert` readings; its fixes went in PROVISIONAL (delegate operator) and the founder ratified them later. Precedent for this chunk's shape (fork, fix, both-mode check, ratification) and for which readings a focus change moves.

@history_item n=5 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `stand_keyboard` (Tab walk with wrap, three focus readings agreeing, `disabled` skipped, no `inert`) and `focusability_updates` +1 (a bare `disabled` set after creation removes focusability). A hidden-control focus fixup must keep these walks and the three focus readings agreeing. `stand_keyboard` is an a11y-leg file.

@history_item n=6 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — `+ contract/harness-inspect/focused`: with nothing focused, the harness's `focused` reading answers the root html element, never none. A check that a hidden control gave focus up must assert the root element, not none.

@history_item n=7 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard re-measured, pinned and routed; the probe's target, legs and pages) — added `dioxus_runtime_probe` (10 tests, 8 in both modes) and measured the `RefCell already borrowed` mechanism: `poll` held `inner.borrow_mut()` across `render_immediate`, and dioxus-core 0.7.10 polls the queued task in `handle_task_wakeup`. The pins needed `hazard_ready` first (handle held, key target focused by Tab), with `mounted_data_asked_at_once_in_a_key_handler_acts` as the synchronous contrast. Legs: workspace `test`, `run all` and `run dioxus_runtime_probe` only. Its `hazard(spawned)` pages are the base item 2 would extend.

@history_item n=8 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's limit) — records the exact residual of item 2 (res-433ttjb2): by construction a task polled during render acts on the document the previous render left, so `set_focus_to` returns `false` for a node created in the same render. The probe's two `…_spawned_acts_*` pins act on nodes mounted by an earlier render. A fix needs a new pin on a same-render node, and keeping the limit needs this entry restated.

@history_item n=9 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the fix's witness on Dioxus 0.7, the probe's two pins) — the pins became `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}` through `spawned_focus_acts`, asserting `#presses` 1 and engine, snapshot and tree focus all on `focus-target`. They were proven discriminating by a mutation control: with the borrow restored, both pins FAILED while upstream's ported test still read `ok`, because that test is insensitive on 0.7. dioxus-core 0.7.10 scheduling: a task an event handler spawns is first polled inside `render_immediate`'s `Work::PollTask`. Rows: decision/upstream-buffered-render-test-kept-as-written, decision/buffered-render-on-dioxus-07-bridge. Trap: an upstream test can be green and not discriminate on 0.7, so a deferred or replayed act needs its own control.

@history_item n=10 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (dioxus-native-dom unit tests, the mutation buffer, the hazard closed, coverage) — `initial_build` and `poll` render into the crate-private `MutationBuffer`, then borrow once and replay through one `MutationWriter`. `mutation_buffer.rs` holds no unit test of its own: 116 lines, 93.97 %, inside dioxus-native-dom's 83 % floor, crate 2937/3387 = 86.71 %. The crate holds 83 unit tests (later 100+). Item 2's fix lands in this module and is read under the same floor.

@history_item n=11 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (upstream's mounted-event test stays out) — `element_removed_before_its_mounted_event_is_flushed` reads 1 mounted event on Dioxus 0.7 where it asserts 0, from 0.7's scheduling, not the borrow. It returns with the Dioxus 0.8 move (res-6ssktw6x, absorbed into v030-03). This is the picked row decision/upstream-mounted-flush-test-left-out: any render or task-scheduling change in item 2 must not claim that guard tested.

@history_item n=12 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the per-OS CI tallies; attributing a tally move) — rule: in a GitHub Actions log the stderr `Running …` lines and the stdout `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name, after stripping ANSI codes. Applies to this chunk's tally re-read.

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (dioxus-native-dom's IME test and an upstream test absent) — upstream's `task_polled_during_render_can_access_document` (`286874cc1`) first recorded absent, later ported as the buffered-render witness. The history of the row upstream-buffered-render-test-kept-as-written.

@history_item n=14 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (upstream's commits with no test here) — first recorded that `element_removed_before_its_mounted_event_is_flushed` was left out, so the `6ef4b532d` guard in `flush_queued_mounted_events` has no test of its own. Precedent for n=11.

@history_item n=15 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit scope and its one test-only shim) — the bridge tests' harness gained its one `cfg(test)` shim, `DioxusDocument::dispatch_to_vdom`, and no function under test gained a constructor or a `pub`. This is the except of the picked row decision/bridge-tests-without-widening: an item-2 or item-4 unit test in `bridge_tests` must add no further widening.

@history_item n=16 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — set the convention behind decision/bridge-tests-child-module and decision/unit-tests-in-source-file. The bridge tests are a `cfg(test)` child of `dioxus_document`, because `DioxusEventHandler`'s two fields are private and it has no constructor. They keep the `EventState` they pass. Unit tests are inline, except a `#[path]` module. Trap: a sed or grep address that ends at a function's name also selects every item whose name starts with it.

@history_item n=17 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** (inline cfg(test) modules) — the unit-test location convention now allows a `_tests` sibling file declared `cfg(test)` by path, with fixture builders in a child module. This covers where new dioxus-native-dom or blitz-dom unit tests may sit.

@history_item n=18 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — a `*_tests.rs` `#[path]` file is no record of the per-crate coverage reading, so moving a test module out of line moves its crate's reading. A test split should forecast a reading change, not none.

@history_item n=19 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ contract/harness-construction/from-vdom`: `from_vdom` first provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context where the root holds none, and a builder's root context stands. This is the seeded source item 3 (res-jqbadec2) would extend to Boa's `Math.random` through a document-level seed. Any reach into `blitz-vibey-script` restates this contract or adds a sibling.

@history_item n=20 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script tests the take added; blitz-vibey-script's inline-test absence; the script tests' shared page and files on disk) — blitz-vibey-script's script tests live in `tests/` (`tests/dom.rs`, later 31 functions, plus `selection.rs` and `url_search_params.rs`), and its 13 `runtime/` and `dom/element/` modules hold no test module. `temp_dir_with_files` is a test-only `TMPDIR` read. A `Math.random` seeding test follows that layout: a `tests/` integration test with inline HTML per test, not an inline module in `runtime/`.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`change` on a checkbox and a radio, from the engine) — precedent for an engine change that reaches the script runtime. `runtime/events.rs`'s synthesis was removed, `dom.rs` gained a test (30 → 31), and the engine targets (`checkable_change_event`, `text_range_no_change_event`) each run in both layout modes on the founder's fork.

@history_item n=22 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — latest `~ decision/file-urls-only-for-file-documents` rule (SVG image and feImage href included; user loads not gated). Earlier steps: 2026-11-13-local-files-for-local-documents widened it to blitz-net sub-resource loads and added the guard `local_files_for_local_documents`; 2026-11-14-remote-pages-local-files added the browser's silent drops. This is the picked row naming the script runtime's crossing rules that item 3 must respect: a seed reads no file, env or carrier.

@history_item n=23 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — `+ decision/root-boundary-err-on-own-session`: a handler Err the app's root boundary takes is checked on a session of its own, because the root boundary replaces the app. Bears on item 4 (res-k2zggoqx): any check that remounts or prunes boundaries must keep the root-boundary Err on its own session.

@history_item n=24 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `~ decision/root-boundary-err-on-own-session`: `caused` now carries the root Err's error text and its chain (guard `stand_act_errors`). Also `+ decision/expected-value-pinned-as-literal`: a control does not hold until the check pins its measured value. Item 4's registry pruning must keep `caused`'s boundary readings, and a growth-count check should pin a measured literal.

@history_item n=25 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `+ decision/no-change-read-control-needs-pending-work`: the control of a read that must change nothing needs pending work, because a pump on a settled screen is invisible. Also `+ contract/session-layout/changes-nothing`, later widened to the `layout` verb by 2026-11-17-layout-on-the-carriers. A focus-fixup "after the next layout" (option B) must keep the layout read changing nothing, with Timer ticks unapplied and an open list open.

@history_item n=26 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — `~ decision/mutation-controlled-tests`: a new check is controlled, meaning a named mutation turns it red and the restored tree reads green. A control that removes a bound loosens it to twice the budget instead. This applies to each fix's new check in this chunk.

@history_item n=27 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `~ decision/layout-tests-in-both-layout-modes` except: upstream integration targets taken in a sync stay as written, and those running one layout mode stand as named exceptions. New fork checks for items 1–3 run in both modes (picked rows decision/layout-tests-in-both-layout-modes and decision/stand-checks-in-both-layout-modes).

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`pseudo_element_flow_classification` runs both layout modes) — ended upstream's both-modes exception by editing the file to loop `for incremental in [false, true]`, with `incremental_oracle` as the identity witness. Precedent for bringing a ported upstream test under the both-modes rule.

@history_item n=29 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (`advance` on every session, the harness's clock; the clock checks counted) — the origin of decision/one-virtual-clock, decision/harness-virtual-clock and decision/wrapped-document-keeps-own-clock. The harness's `VirtualClock` is moved by `tick` and `advance_time` only, never by `screenshot`, `text_contrast` or `settle`. `into_config` puts it in `DocumentConfig.clock`, and a wrapped document keeps its own. Guards: `virtual_clock_engine` and `stand_virtual_clock` (whose 800 ms sleep is the one recorded sleep). A deferred act for item 2, or a re-layout for item 1, must not read wall time.

@history_item n=30 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — the origin of the settle contracts in the picked rows (`settle`, NotSettled, side effects, `SETTLE_PASS_LIMIT` 64). Quiet means poll false, the hover node unchanged and no load finished; settle reads no clock, advances no time and neither reads nor drains the changed set. Traps: a delivered tick needs a pass of the check's own only when it follows the input helper, and the changed set is non-empty on a freshly booted harness, so drain first. A deferred render-time act (item 2) has to leave settle's quiet test true.

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait) — `CountedHandler` forwards `NetHandler::error`, so a failed stylesheet unblocks the document and the settle wait ends (decision/failed-load-ends-settle-wait). Trap: a trait's do-nothing default compiled silently through a wrapper, so a change that extends a trait must sweep its implementers. This applies to any hook added on `Document`, `NetHandler` or the script runtime here.

@history_item n=32 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ decision/stand-boots-offline` except: the stand boots keep no network provider except the load page's `LoadNet` and the error-text check's `FailingNet`. LoadNet was introduced in 2026-10-25-cold-agent-bar and HeldNet in 2026-10-19-shared-fixtures. A new stand boot for these fixes stays offline or is named as a further exception.

@history_item n=33 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — added `from_component_alone`, `vdom_alone`, `stand::boot_alone`, decision/component-alone-props-as-maker (an EventHandler prop is made only inside a Dioxus runtime) and decision/component-alone-root-context-before-build (a context added after the build reaches no render), plus decision/timer-context-read-by-delivered-ticks. Constraint for item 3: a seed provided as a root context must be provided before the build.

@history_item n=34 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the harness re-exports `NamedKey`; the `keyboard_compat.rs` exclusion) — the harness re-exports keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` (decision/harness-reexports-key-types). dioxus-native-dom's coverage leg excludes `keyboard_compat.rs` by filename. Item 1's key-press checks use these re-exports, and the dioxus-native-dom floor is read with that exclusion.

@history_item n=35 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (residual owners re-said to their rows' status) — the `keyboard_compat.rs` exclusion and the Dioxus 0.8 delta are owned by res-6ssktw6x, absorbed into v030-03, with the re-read owned by CARRY cry-mjwr45xr. This is the "wait on a dioxus-core release" path item 4 may name, and it ties the 0.8 move to the left-out mounted-flush test.

@history_item n=36 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and a zero-failure flakiness budget on `stand-repeat`. A fix in dioxus-native-dom (items 2 and 4) or in the harness (item 3's `from_vdom`) is read against these floors.

@history_item n=37 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — a green per-package run is not the workspace leg's green, because the workspace build enables blitz-dom's `tracing` (picked row decision/per-package-green-not-workspace-green). The same file read green per package and red in the workspace leg. Engine fixes in blitz-dom must read green under both runners.

@history_item n=38 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (three engine checks, the re-measured counts) — the bounds reader has two bodies chosen by the `writing-mode` feature (on in the workspace build only), and each holds a check of its own, `scrolled_box_client_rect` (decision/bounds-reader-bodies-each-checked). A fix in one body reads green under the other build. This matters if item 1's "no longer renders" test reads layout geometry.

@history_item n=39 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the unit-level `build` helper carries the no-op HTML parser, so parsed markup is pinned in blitz-tests (decision/parsed-markup-pinned-with-real-parser). Trap: a headless key press differs by platform, because the editor's Backspace arm is compiled out on macOS. Prove state changes with clicks and typed characters, which matters for item 1's key-taking assertions.

@history_item n=40 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — rules for driver checks: a failure message names a layout mode, a task and an index, never an id or screen text. Also, no deleting key in any check but `stand_act_keys`' one. These bind new stand checks for items 1 and 2.

@history_item n=41 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `telemetry_drop` proves the three-outcome target allowlist (decision/telemetry-target-allowlist): an engine target prints safe fields, an `escher_` target prints with content fields redacted, and any other target is dropped. A fix that logs (for example a pruned boundary or a refocus) must stay inside this allowlist and add no log field without the founder's ruling.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand-repeat` names fifteen targets) — `inert_subtree`, among others, is a named `--test` of `stand-repeat`, pinned by `test_stand_repeat_leg_runs_three_rounds` and read by the capability statement's count. The list was later seventeen (2026-10-31-select-popup) and started with 2026-10-24-stand-requirement-sweep's ten. A new non-`stand_` witness of these fixes joins the leg only by editing `ci-leg.sh`, its pin and the capability count together.

@history_item n=43 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-check-names-held-to-schema` (and its sibling rows): the claim section's names and counts are pinned, the a11y leg's file count and `stand-repeat`'s named-target count to `ci-leg.sh`. A renamed check, or a leg membership change, reddens `ci-scripts` until `docs/capabilities.md` is restated by hand. Kept limits for items 1–4 belong in that claim map (decision/capability-claims-name-their-checks).

@history_item n=44 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — `~ decision/a11y-leg-membership`: a file joins the a11y leg if it witnesses SC 2.1.1 or 2.4.3, a tree reading, or a recorded gap. 2026-10-24-a11y-re-read added `inert_subtree` and `key`-reading files to the fourteen. A focus-fixup witness for item 1 is a 2.4.3 candidate, and its leg is decided by that rule.

@history_item n=45 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ overview/session-lifecycle`: the session is no verb of the five-command agent-run contract, and `run session` runs `cli_flow`. This is the overview the picked session-lifecycle rows sit under (act, LOAD_WAIT, acting verbs, refusal order). A fix moves those rows only if it changes a driver step's settle or refusal.
