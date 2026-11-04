@history_item n=1 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — registered `DioxusDocument::element_id` / `element_ids` on the bridge, computed on demand from a vdom walk joined with a DOM walk, written to neither DOM nor vdom, and the component-path tier whose segment is the last `::` segment of `VComponent.name` with generics stripped (the four framework scopes adding none, a 2nd+ same-name instance `{name}:{k}`); no `NodeId`, `ElementId`, `ScopeId` or pointer in an id. Recorded as BUILT after dioxus-core 0.7.10 disproved three plan assumptions (base scope `root`, a bare `VComponent.name`, collision-free grammar). Matters here: link (1)'s «exposed» verdict weighs this existing walk as its named API, and the correction is a precedent to measure Dioxus' scope/name shapes rather than assume them.

@history_item n=2 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the anchored tier (`{key}//{segment}`, inside one component only) and `DioxusDocument::unkeyed_actionable()`, whose readers include `data-dioxus-id`; trap: `UnkeyedActionable.node` is a process-local `NodeId`, so a first wire form must not carry it. Matters here: the four tiers link (1) reads, a listener-mark reader link (2) may name, and the no-`NodeId` rule any probe output keeps.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — proved ids persist across re-render, remount and restart in both layout modes; the CRUD row's Dioxus key became the person's `u64`; trap: the html/head/body/#main skeleton built outside the VirtualDom is never remounted. Matters here: a link (1)/(4) reading across a CRUD re-render rests on these persistence facts and must not assume every node is fresh after a remount.

@history_item n=4 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node) — was «`node_id_mapping` entries intentionally not cleared on removal so dioxus-core can reuse detached nodes»; now upstream #1185 drops a removed node with its descendants and clears `element_id_mapping` / `node_id_mapping`, so it reads no id and `get_node` is `None`; the falsy clear became upstream's 28-name `is_bool_attr`. Matters here: links (1) and (4) must be run across a CRUD delete and state what a removed node resolves to through these mappings.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the MountedData hazard of a spawned task) — new Dioxus-integration record, no engine change: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, holding the document's `RefCell`, and a `MountedData` call in it panics at `events.rs:162`; measured through the harness's `poll(None)`, the windowed path read and not run; Cells' async handles were tried, panicked and did not ship. Matters here: this is the CARRY the chunk folds; P3 re-verifies exactly this mechanism and coordinate at HEAD and the reading names it.

@history_item n=6 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes; Cells' grid) — eight more ratified direct engine changes for every document, each flagged upstreamable, nothing sent, merge surface a line owned by an upstream-sync entry; and `cells-grid` with `onkeydown` calling `MountedData::scroll_to` synchronously, the count of elements that react to input and state no role moving 676 → 677 (every element with the listener mark reading `GenericContainer`). Matters here: the class the MountedData fork's options must state, and Cells as the served contrast to the spawned-task path; link (2)'s listener reading must agree with or explain the pinned 677.

@history_item n=7 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic's fix) — `BaseDocument::deallocate_anonymous_block` clearing stale `layout_parent`s stands beside the eight as a direct engine change for every document, ratified on the founder's F1 A, flagged upstreamable, merge surface owned by «Upstream sync ahead of determinism and exploration»; standing trap: a probe of a panic can read green because its input never reaches the path, so assert the fixture first. Matters here: the precedent the MountedData fix's fork follows, and the trap binds P3's re-measurement of the hazard.

@history_item n=8 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the queued `mounted` event's guard; citations re-pointed) — `flush_queued_mounted_events` skips an element removed since queued, reading it through `try_element_to_node_id`; upstream's test `element_removed_before_its_mounted_event_is_flushed` left out because it reads 1 mounted event on Dioxus 0.7 where it asserts 0, returned with `286874cc1` through a CARRY; `dioxus_document.rs` citations moved +4. Matters here: the mounted flush after each `poll` is on the hazard's path, Dioxus 0.7 vs 0.8 mounted behaviour differs, and the hazard's coordinates must be re-read at HEAD after this sync.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (merge surfaces' owner; the upstream line at `e42ee1f25`) — the merge surfaces' owner moved to «Upstream sync ahead of determinism and exploration», each flagged upstreamable, nothing sent; upstream read at `e42ee1f25`, merge base unmoved, Dioxus "0.7.3" with no git dependency. Matters here: names the residuals owner a MountedData fix's merge surface would join, and restates the pin the chunk must not move.

@history_item n=10 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — Project Intent records Dioxus 0.8.0-alpha.3 (`286874cc1`) waiting for escher's next version, owned by a `.andromeda/residuals.md` line, Dioxus staying "0.7.3"; the `keyboard_compat.rs` coverage exclusion holds «until escher takes upstream's retirement of it» in `286874cc1`. Matters here: the source of the no-pin-move constraint; a link that only Dioxus 0.8 would expose stops at «not exposed without a fork-level change».

@history_item n=11 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take at `ebacd68bd`) — sixteen commits left out with measured reasons, Dioxus 0.8.0-alpha.3 among those waiting, Dioxus "0.7.3" unchanged; IME at editor construction added dioxus-native-dom's unit test `input_focused_on_mount_enables_ime` (81 → 82). Matters here: restates the pin and the bridge crate's test count a probe check inside dioxus-native-dom would extend.

@history_item n=12 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (keyboard-types 0.8) — keyboard-types moved to "0.8" while Dioxus 0.7.3 carries its own 0.7, so a module glob-importing `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`; dioxus-native-dom gained the crate-private `keyboard_compat` converting 0.8 into Dioxus 0.7's types for `events.rs`, excluded from coverage. Matters here: probe code driving the hazard's key handler must name `::keyboard_types`, and `events.rs` (the panic site) is reached through this bridge.

@history_item n=13 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the `Document` trait gained a default `accessibility_tree()` that `DioxusDocument` alone overrides to set stable ids as `author_id`; dioxus-native-dom's `accessibility` feature reads `["blitz-dom/accessibility", "dep:accesskit"]` and a crate gets it only by naming it; trap: a feature-gated item can pass under `cargo test --workspace` and be compiled out under `-p blitz-tests` — check `cargo tree -e features -i`. Matters here: any probe check reading the snapshot must get `accessibility` the way blitz-tests names it, and the trap applies to its run.

@history_item n=14 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — registered `DioxusDocument::snapshot`, read-only, no field holding a `NodeId`, tree id, `ElementId`, `ScopeId` or pointer; standing rule: the snapshot derives no role or name of its own. Matters here: a probe reading through the snapshot stays read-only and puts no process-local id into anything it reports.

@history_item n=15 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — dioxus-native-dom's bridge tests live in `dioxus_document_tests.rs`, declared as `#[cfg(test)] #[path] mod bridge_tests;` at the end of `dioxus_document.rs`, a child of the module under test so they reach its private fields (the only place some behaviour is observable); stand checks share tables through `tests/common/mod.rs`, a subdirectory module being no test target. Matters here: a reading reachable only from that child is classed «reachable only by a change inside dioxus-native-dom», never «exposed».

@history_item n=16 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — dioxus-native-dom's bridge tests went six → seven (a created text node's mapping and stack read). Matters here: the count a probe test added to `bridge_tests` would move and re-state.

@history_item n=17 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — registered `seven_guis::stand` (pinned 800 × 600 · 1.0 · Light, offline `options(incremental)`, a fresh `VirtualDom` per boot, no telemetry, env read or `blitz_net`) and the falsy-`disabled` clear inside dioxus-native-dom, recorded PROVISIONAL as an upstreamable widening and later ratified. Matters here: the boot every verdict runs on, and a precedent that a change inside the bridge reaches every Dioxus document and was put to the founder — weigh this when classing a cost «inside `dioxus-native-dom`».

@history_item n=18 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the bridge's falsy clear widened to 27 names copied from Dioxus 0.7.10's web interpreter, recorded PROVISIONAL until the founder; trap: the list is a copy, a Dioxus upgrade does not move it. Matters here: another bridge-side behaviour change for every Dioxus document carried to the founder, and a reminder that bridge code shadowing Dioxus internals drifts with the pin.

@history_item n=19 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `Harness::settle` (one pass is pump's `poll` then `resolve`, poll's answer kept; never waits on a load, reads no clock but the harness's, neither reads nor drains the changed set) and `Session::act` mapping a not-quiet instance onto `NotSettled(Busy)`, the step not rolled back. Matters here: the hazard was measured through `poll(None)`, which pumps and never settles, while a Session's `act` settles — the re-verification must say which it drove.

@history_item n=20 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — `Session::act` and the acting step settle through the bounded load wait (`LOAD_WAIT` 5 s wall time) over `Harness::settle`, moving no time on the held instance. Matters here: a served-task run through `act` differs from a raw `poll(None)` pump, part of naming which path the hazard reading drove.

@history_item n=21 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness holds one `VirtualClock` its documents share, moved only by `tick` / `advance_time`, never by `screenshot`, `text_contrast` or `settle`; `advance` moves the session's clock by exactly `ms`; `CAUSES` closed at eight. Matters here: the harness contract the probe's runs pump through, and one of the closed counts the probe leaves as they are.

@history_item n=22 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — seven engine changes for every document answered by the operator as the founder's delegate were recorded PROVISIONAL, then ratified in one batch at the Epoch 2 boundary. Matters here: if the MountedData fork is answered by a delegate rather than the founder, the record would read PROVISIONAL; the chunk brings the fork, it lands no fix.

@history_item n=23 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the changed set (`take_changed_nodes`) is engine-touched, a superset of snapshot-changed, marked by mutation flags, focus/checked and text input, not by creation, hover, layout or loads; the diff is not derived from it. Matters here: a link (4) state → readers reading must not take the changed set for a reader map, and the Session neither reads nor drains it.

@history_item n=24 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation: Enter and a released Space dispatch a Click through `dispatch_activation_click`, queued so the app's handlers (a Dioxus `onclick`) run before `handle_click`; a direct engine change for every document. Matters here: link (2)'s `click` answer and link (3)'s handler-order reading rest on this queue.

@history_item n=25 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — `DomEventKind::Change` / `DomEventData::Change` added, `from_str` mapping `change`; the bridge hands `Input | Change` to the app. Matters here: link (2) names a listener's event as the listener map names it, by this mapping rule.

@history_item n=26 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink prints only targets in `ENGINE_TARGET_PREFIXES` / `ESCHER_TARGET_PREFIXES`; an escher target prints every field not in `CONTENT_FIELDS`, any other target dropped whole. Matters here: a probe that logged a component name, handler or signal value under an `escher_` target would print it — the probe adds no log field or span.

@history_item n=27 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the `command` span's eight fields, never fielding the screen text, any argument or any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`. Matters here: the probe's served-task calls keep the eight fields and add none.

@history_item n=28 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `tool_call` span; standing rule: no span field holds what a call supplied or a screen reads. Matters here: the same no-field rule binds anything the probe reads.

@history_item n=29 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the one verb table every surface reads; rule: `Command`, `Call` and `ArgValue` print an id and typed text under `Debug`, so nothing fields them; a later entry adds verbs to `VERBS`, never a second table. Matters here: probe code must print no `Debug` of those values, assertion failures included.

@history_item n=30 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` the one entry point, nothing public running a `Command` that did not come from `validate`; nothing fields an `Outcome` (its `Debug` prints ids, names, values); escher-driver names `accessibility` on dioxus-native-dom. Matters here: the served task's calls go through `run` and the probe adds no other path or print.

@history_item n=31 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the session socket carries calls and answers (ids, names, masked values, snapshot text, diffs, no `NodeId`), ratified by the founder; from here «the next surface that carries them asks again». Matters here: any new carrier or crossing of what the probe reads is a founder's question, and the scope adds none.

@history_item n=32 marker=2026-10-12-self-description
**2026-10-12-self-description** — fixed `'static` table text on the ratified carriers ratified; «text from a page, a call or a screen, a new carrier or a new exit still widens»; `NODE_FIELDS` became thirteen keys. Matters here: a component name or handler the probe reads becomes no key, field or text on a carrier.

@history_item n=33 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — `lint` became the twelfth verb, `describe` counting 12 verbs and 8 causes. Matters here: `VERBS` must still list exactly twelve after the probe.

@history_item n=34 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `SessionError` went to fifteen variants; `Session::recording_into` joined the off-unix `Unsupported` list while a `Session` held in process without a recording works on every target. Matters here: the probe adds no session error, and a blitz-tests check holding its served Session in process without a recording holds on the windows and macos matrix.

@history_item n=35 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the two committed checks `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` hold the stand Counter's ids, names, values, snapshot text and diffs, compared byte for byte with a fresh recording. Matters here: probe code kept under `examples/seven_guis/` must leave the stand Counter's screen unchanged.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — counts the probe would extend: `session_common` declarers 31 → 32, `fixtures` 28 → 31, `stand-repeat`'s named targets outside the prefix 15 → 17; the snapshot carries no `expanded` key, `NODE_FIELDS` keeping thirteen. Matters here: a kept probe check reading a shared module raises its count, and a non-`stand_` target joins `stand-repeat` only by name.

@history_item n=37 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the stand serves Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`, its `LoadNet`, the one non-offline boot), `escher-session` six served tasks; the served-page checks `stand_cells_served` / `stand_load_page` held as sessions; `unkeyed_actionable` empty on all six. Matters here: the served task run beside a lean one is booted and held the way these checks do.

@history_item n=38 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `stand-repeat` three rounds with a zero-failure budget and no retry; the coverage floor dioxus-native-dom 83 %; test-only binds and `TMPDIR` reads registered as occupied resources even though no crossing. Matters here: a kept stand check must be deterministic over three rounds, bridge code keeps its floor, and any env read or bind a probe test adds still registers.

@history_item n=39 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` names blitz-tests targets outside the `stand_` prefix as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds` and read by `stand_requirement_sweep`. Matters here: a probe check's file name decides its `stand-repeat` membership.

@history_item n=40 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — `stand-repeat`'s membership is held jointly by the leg test and the sweep's `contains` read, which cannot see a target dropped from `stand-repeat` alone (gap owned by a CARRY on «Stand requirement sweep»); the cognitive-complexity ceiling of 15 is a dev-host reading only. Matters here: naming a non-prefixed probe target needs the leg test, not the sweep, to pin it.

@history_item n=41 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — blitz-tests keeps one test target per file directly under `tests/`, upstream's `all.rs` held at `test = false`, pinned by `BlitzTestsTargetsTest`, because `mod common;` / `mod session_common;` and re-executing checks need per-file targets. Matters here: a probe check is its own target file, never a module in `all.rs`.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (upstream's both-modes exception ended) — `pseudo_element_flow_classification` now runs `for incremental in [false, true]`, so the both-layout-modes convention holds no exception. Matters here: a probe check whose reading layout touches runs in both modes, with no exception to cite.
