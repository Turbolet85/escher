@history_item n=1 marker=amd-xbwketkt
**amd-xbwketkt** — (2026-10-31-engine-debts) first recorded the MountedData hazard under dec-dioxus-integration as a measured fact, "no engine change": a key-handler-spawned task first polled inside `render_immediate`, which holds the document's `RefCell`, panics on a `MountedData` call at `events.rs:162`, so Cells calls its handles synchronously, measured through the harness's `poll(None)`, the windowed shell's same lines read and not run. This chunk is the engine change that clears that "no engine change" clause. Cells' synchronous-handle workaround is the precedent the buffered render makes unnecessary, though nothing here requires removing it.

@history_item n=2 marker=amd-ogtxzreb
**amd-ogtxzreb** — (2026-11-04-dioxus-runtime-probe) re-measured the hazard, naming its mechanism: `poll` holds the `RefCell` across `render_immediate`, where dioxus-core 0.7.10 polls the queued task through `handle_task_wakeup`, and `set_focus` borrows through `doc_mut`. The measurement was headless through the harness's pump in both layout modes, pinned by `dioxus_runtime_probe` (test-plan §6). It routed the fix, a port of upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge, to the route entry «MountedData buffered render», and added the matching clause to Project Intent's left-out `286874cc1`, the 0.8 move still waiting. This is the hazard row this chunk's wrap must re-say as fixed, as an engine change for every Dioxus document. Its Kept clause (Dioxus "0.7.3", no git dependency, windowed path read and not run) binds this chunk.

@history_item n=3 marker=amd-jy43j2t7
**amd-jy43j2t7** — registered the `dioxus_runtime_probe` scope record: one file and one target, 10 tests (8 looping both layout modes, plus the hazard pinned as two `should_panic` tests, one per mode). It has no print, env read, subscriber or listener and no `stand_` prefix, so it runs in neither `run stand`, `stand-repeat` nor the `a11y` leg. This chunk inverts the two pins, so the record must be re-said: still 10 tests in one target, still outside `stand-repeat` and `a11y`, its MountedData clause now saying that `set_focus` moves focus with no panic.

@history_item n=4 marker=amd-mebfv4v2
**amd-mebfv4v2** — recorded two facts read through Dioxus 0.7.10's public API with no fork-level change, each in both layout modes. First, a node's owning component, by a walk from `base_scope` through `try_root_node`, `template.roots`, `dynamic_nodes`, `mounted_root` and `mounted_scope`, with a deleted CRUD row resolving to none. Second, a `GlobalSignal`'s readers through `subscribers()` and `origin_scope`. These are the probe's other eight tests, which this chunk re-runs as witnesses that the document and the vdom read the same after a buffered render.

@history_item n=5 marker=amd-7yesshpe
**amd-7yesshpe** — recorded that the bridge's listener attributes over-report: `create_event_listener` writes an event attribute `"<rust func>"` plus the `data-dioxus-id` mark, and `remove_event_listener` lowers the per-kind count alone. dioxus-core 0.7.10 reaches `remove_event_listener` only through a spread; a listener turning to `None` is written through `set_attribute`. Neither listener method has a workspace caller; dioxus-core calls them. The recorder must capture `set_attribute`, `create_event_listener` and `remove_event_listener` as dioxus-core issues them, and the replay becomes their caller. It must reproduce the over-report the probe re-checks.

@history_item n=6 marker=amd-puzo7mbp
**amd-puzo7mbp** — (2026-11-01 upstream sync) added the guard in `flush_queued_mounted_events`, which skips an element removed since its event was queued, reading it through `try_element_to_node_id`. It recorded upstream's `element_removed_before_its_mounted_event_is_flushed` as left out: it reads 1 mounted event on Dioxus 0.7 where it asserts 0, and a `CARRY:` returns that test with `286874cc1`. The buffered apply must run before this unchanged flush, at both render sites. The carried upstream test is a trap: on 0.7 it may still read 1 after the port.

@history_item n=7 marker=amd-w3z3f3db
**amd-w3z3f3db** — (2026-10-13 sync) made the bridge drop a removed node with its descendants and clear both `element_id_mapping` and `node_id_mapping` (upstream #1185, via `remove_and_drop_node` / `replace_and_drop_node`). Before this, detached nodes kept resolving. It also replaced the 27-name list with upstream's 28-name `is_bool_attr`, which left `mutation_writer.rs` byte-identical to upstream at `434e0661`. That is the state this chunk must keep: every buffered remove goes through the unchanged writer in recorded order, and any edit to the file (e.g. a `DioxusState` field) is named with its reason.

@history_item n=8 marker=amd-o5d22sqx
**amd-o5d22sqx** — (2026-10-24 sync) recorded Dioxus 0.8.0-alpha.3, together with git Stylo 0.23-pre and the Taffy-main commits, as waiting for escher's next version, owned by a residual (target: next). Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency. This is the origin of the constraint that only `286874cc1`'s buffered render is ported and no Dioxus pin moves.

@history_item n=9 marker=amd-437umnpi
**amd-437umnpi** — moved the upstream line to `e42ee1f25`: 36 of 57 commits past merge base `23b9b26a9` taken, 21 left out, the base unmoved. Its Kept clause re-states Stylo "0.22.0", Taffy "0.15" and Dioxus "0.7.3" with no git dependency. This is the current sync line the port of `286874cc1` sits beside; it is a partial port, not a take that moves the line.

@history_item n=10 marker=amd-qi5jxqm4
**amd-qi5jxqm4** — (2026-10-17 sync) moved keyboard-types to "0.8" (lock 0.8.3) while Dioxus 0.7.3 carries its own 0.7, so two versions stand in the graph. A module that glob-imports `dioxus::prelude::*` therefore names the workspace's crate as `::keyboard_types`. The ported test and the probe's key-driven pins must follow this. It is also why upstream's keyboard-types 0.8 move in `286874cc1` stays unported.

@history_item n=11 marker=amd-mvbqmia6
**amd-mvbqmia6** — added the crate-private `keyboard_compat`, upstream's bridge converting keyboard-types 0.8 into Dioxus 0.7's types. It is excluded from both `cargo llvm-cov` lines by filename (`--ignore-filename-regex 'keyboard_compat\.rs$'`), and the floors are unchanged. `keyboard_compat` must stay in the module list this chunk edits, and the coverage exclusion is unaffected.

@history_item n=12 marker=amd-asm3wsx5
**amd-asm3wsx5** — re-said the `keyboard_compat` coverage exclusion as lasting "until escher takes upstream's retirement of it", since upstream retires the file in `286874cc1`, not taken. A partial port of `286874cc1` must not take that removal; if it did, the exclusion clause and the scope record would both move.

@history_item n=13 marker=amd-fuzh25ns
**amd-fuzh25ns** — set the one exception to the "`mod tests` at the end of the file" rule: dioxus-native-dom's bridge tests live in `dioxus_document_tests.rs`, declared at the end of `dioxus_document.rs` as `#[cfg(test)] #[path] mod bridge_tests;`. As a child of the module under test, they reach its private fields with no shim. The plan must settle and record whether the ported `task_polled_during_render_can_access_document` goes into upstream's `mod tests` or beside the bridge tests.

@history_item n=14 marker=amd-li2fn2ob
**amd-li2fn2ob** — generalised out-of-line `#[path]` test children to five, and kept the dioxus-native-dom row's "one `cfg(test)` child module, `bridge_tests`". If the ported test adds a second test module to `dioxus_document.rs`, that row and the count change.

@history_item n=15 marker=amd-5hxke3kf
**amd-5hxke3kf** — moved dioxus-native-dom's `bridge_tests` count from six to seven unit tests (the seventh: a created text node's mapping and stack read). The ported regression test moves this count (or adds a separate module), and the scope record states it.

@history_item n=16 marker=amd-anuhisgt
**amd-anuhisgt** — recorded upstream `7fc9ee3af`: a text input already focused when its editor is constructed enables IME. It is held by dioxus-native-dom's unit test `input_focused_on_mount_enables_ime` and recorded as an upstream engine change for every document. That witness sits in the test module the chunk edits and reads mount-time focus, so it must keep passing under the buffered render. Its record form is the shape an upstream-derived engine change takes.

@history_item n=17 marker=amd-ffneu5yd
**amd-ffneu5yd** — recorded that `BaseDocument::set_focus_to` returns `false` for a node not in the document, moves `FOCUS_WITHIN` along the ancestor chains, and requests a redraw. The trap for the ported test and the inverted pins: a task polled during render runs before the buffer is applied, so a `set_focus` there must name a node already in the document.

@history_item n=18 marker=amd-t5spowmn
**amd-t5spowmn** — put the inert focus fixup in `DocumentMutator::flush`, which calls `clear_focus` where focus resolves to an inert node and fires no `blur` or `focusout`. Replaying the whole buffer under one borrow must still end in that mutator flush, before the queued mounted events are flushed.

@history_item n=19 marker=amd-7zhm5mac
**amd-7zhm5mac** — set the changed-set contract: `has_changes` and `take_changed_nodes`, marked by the 15 mutation-flag sites of `DocumentMutator` (in-document nodes only), by focus or checked snapshots and by text input. `View::poll` drains it and refreshes the tree on change. Replaying through the same `doc.mutate()` calls must mark the same set as the direct write, or the shell's refresh and the snapshot diff drift.

@history_item n=20 marker=amd-gcswno32
**amd-gcswno32** — (2026-11-01 sync) set restyle on mutation: `set_attribute`, `clear_attribute`, removal and insertion restyle through `restyle_for_attribute_change` and `restyle_for_child_list_change`, with no blanket subtree restyle, and the changed-set insert in `remove_and_drop_all_children` comes first. The replayed buffer must reach these through the unchanged mutator path in recorded order, so that `incremental_oracle`'s layout identity holds.

@history_item n=21 marker=amd-4ls4ci6z
**amd-4ls4ci6z** — added the `Document` trait's defaulted `accessibility_tree()`, which `DioxusDocument` alone overrides to set each element's `author_id` to its stable id. The shell calls it through `&dyn Document`. The reshaped `poll` and `initial_build` must keep the `Document` impl as stated, the override and its ids reading the same after a buffered render.

@history_item n=22 marker=amd-lnabir6w
**amd-lnabir6w** — registered the stable element ids `element_id` and `element_ids`, computed on demand from a vdom walk joined with a DOM walk and written to neither the DOM nor the vdom. Later chunks address elements by this grammar. "The element ids must not change" rests on this; `stand_element_ids` is a witness the chunk re-runs.

@history_item n=23 marker=amd-hntk3rbe
**amd-hntk3rbe** — proved the ids persistent across re-render, remount and restart on the stand, in both layout modes. The trap it records: the four html/head/body/main skeleton elements are built outside the VirtualDom and never remounted. `initial_build`'s buffered `rebuild` must leave that skeleton and the remount behaviour exactly as the direct write did.

@history_item n=24 marker=amd-c5pnnkuf
**amd-c5pnnkuf** — registered the headless stand: `boot` and `boot_timer` build a fresh `VirtualDom` per boot, rendered through `TaskShell`. Every stand boot reaches `initial_build`, so each one now builds through the buffer. The `stand_*` checks the scope names as witnesses read these boots.

@history_item n=25 marker=amd-hkta5tux
**amd-hkta5tux** — built `Harness::settle`: one pass is pump's `poll(None)` then `resolve`, poll's answer kept, ending `Settled` or `NotSettled { Render | Loads | Layout }`. Settle reads no clock but the harness's and drains nothing. The inverted pins drive the buffered `poll` through this pump. `stand_act_spans`' `busy="render"` reading depends on `poll` still answering true on a fixture that re-renders every time.

@history_item n=26 marker=amd-soe7mupv
**amd-soe7mupv** — added the driver's bounded load wait: `settle_waiting` settles again every 10 ms while the answer is `Busy::Loads`, up to `LOAD_WAIT` (5 s). The stand's load page re-renders after its fetch lands, inside this wait, so the buffered `poll` must keep settle's answers as they read.

@history_item n=27 marker=amd-c5d6xlfz
**amd-c5d6xlfz** — scoped "tracing gated per call site" to the engine and upstream crates, using `#[cfg(feature = "tracing")]` with a `cfg(not)` fallback; escher-telemetry is ungated. dioxus-native-dom is an upstream crate, so any trace in the new buffer module follows the gated form. Through the crate-local `trace!` macro, the macro's 4-argument arm passes only two items.

@history_item n=28 marker=amd-6g4d6x3j
**amd-6g4d6x3j** — set the sink's target allowlist: an engine target prints only `SAFE_FIELDS`, an `escher_` target every field not in `CONTENT_FIELDS`, and any other target is dropped whole. The extract's arch↔obs binding rests here: the buffer reaches no telemetry and `SAFE_FIELDS` is not widened to carry mutation content.

@history_item n=29 marker=amd-ojykdwbt
**amd-ojykdwbt** — the founder's standing rule (2026-10-09): a diff's content never reaches a log, while its three sizes (added, removed, changed) may. The buffer's mutations (ids, tag and attribute names, text) are content of the same class: logged or printed nowhere, and no new field or span.

@history_item n=30 marker=amd-wxdpki7i
**amd-wxdpki7i** — closed `Cause` at eight (`CAUSES: [Cause; 8]`), `time-unavailable` retired. The scope's "nothing crosses" boundary holds this closed set, along with the verbs, session errors and span fields: the buffered render adds no cause.

@history_item n=31 marker=amd-tuit32gw
**amd-tuit32gw** — made `bounds` (the client rect) viewport-relative for every element: the shared reader subtracts the scroll offset of every box on the containing-block chain above a node, and never the node's own. The ported test calls `get_client_rect()` from a task during render, which must read through this unchanged reader with no borrow panic.

@history_item n=32 marker=amd-oqwmbksj
**amd-oqwmbksj** — widened `scroll_into_view` to scroll every box on the containing-block chain, innermost first, then the viewport, for every caller, `MountedData::scroll_to` among them. The scope lists `scroll_to` among the `MountedData` calls that must reach the document during render through this unchanged method.

@history_item n=33 marker=amd-2c27hy5n
**amd-2c27hy5n** — recorded Cells' grid: arrows move `current` into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)` on handles held by one `onmounted` per data cell. That synchronous call is the hazard's workaround. The mounted events' order and the handles they deliver must survive the buffered apply, which `stand_cells_served` and `stand_keyboard` witness.

@history_item n=34 marker=amd-ljouasiq
**amd-ljouasiq** — the record shape for escher's direct engine changes for every document: each is ratified, flagged upstreamable with nothing sent, and its merge surface is a line owned by the upstream-sync entry; the body's tally counts them. The buffered render is such an engine change. It must be recorded the same way, its merge surface (`dioxus_document.rs`, the buffer module, `lib.rs`) put on that line.

@history_item n=35 marker=amd-k3hvnyyp
**amd-k3hvnyyp** — the nearest precedent: a single direct engine change (the key-press panic fix) recorded as a new row beside the eight, ratified, flagged upstreamable with nothing sent, and its merge surface a line owned by «Upstream sync ahead of determinism and exploration». Its standing trap applies to the inverted pins: a probe of a panic can read green because its input never reaches the path, so assert the fixture first.

@history_item n=36 marker=amd-6lwyfqkb
**amd-6lwyfqkb** — moved every merge-surface owner line to «Upstream sync ahead of determinism and exploration», with each residual row re-read and given a named outcome, nothing sent. This is the owner the buffered render's merge surface joins, and the sync that will meet the `286874cc1` port.

@history_item n=37 marker=amd-yxeaxjta
**amd-yxeaxjta** — re-pointed the owner sentences that named the retired `.andromeda/residuals.md` to residual rows (`res-3v42legm`, `res-llkgqygv`). The merge-surface line the buffered render's record joins is `res-3v42legm`, read through `rec.py get`; no residuals file is to be named.

@history_item n=38 marker=amd-rvwyylpw
**amd-rvwyylpw** — (2026-10-14-quality-gates) gated `coverage` by per-crate floors through `coverage_floor.py`: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. It also added `stand-repeat`: three rounds of every `stand_*` target, a zero-failure budget, no retry. The new buffer module's lines count toward dioxus-native-dom's 83 % floor. The stand witnesses the chunk names must stay green over three rounds.

@history_item n=39 marker=amd-e55tuttu
**amd-e55tuttu** — ended the last exception to the both-layout-modes rule: scenarios run `for incremental in [false, true]`, with no clause standing. The inverted pins (one per mode, as they stand) and any new probe test must hold in both modes, with no exception available.

@history_item n=40 marker=amd-kslriomy
**amd-kslriomy** — committed the Counter checks (`tests/blitz-tests/checks/counter-cli.json`, `counter-mcp.json`), holding the screens' ids, names, values, snapshot text and diffs at rest. `stand_replay` reads them, and seven_guis `recording` keeps them fresh by a byte-for-byte comparison. A buffered render that moved any id, snapshot line or diff would turn these red, which makes them a witness of "behaviour otherwise unchanged".

@history_item n=41 marker=amd-e5ukpedb
**amd-e5ukpedb** — re-pinned the stand's click census: 68 of the 77 snapshot nodes of the four lean tasks are accepted at boot and 9 refused, the flight booker's two options refused `covered`. It also set the hosted snapshot sizes. These are stand readings the buffered render must leave as they read in both layout modes.

@history_item n=42 marker=amd-wodnmcnz
**amd-wodnmcnz** — made the lint skip a closed select's options, so `stand_lint` pins `at_boot` as `[0, 0, 0, 0]` in both layout modes. `stand_lint` reads the `DioxusDocument` snapshot the buffered render builds. It is among the `stand_*` witnesses that must pass unchanged.

@history_item n=43 marker=amd-tlx7pdkk
**amd-tlx7pdkk** — corrected Cells' served screen to 47,089 bytes in 740 lines and 5 parts. Its standing trap: a plan quoting a flagged-old reading did not re-read it. `stand_cells_served` is one of this chunk's witnesses; quote its current reading, re-read.
