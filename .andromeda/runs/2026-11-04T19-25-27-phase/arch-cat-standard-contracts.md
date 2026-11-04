@carried n=1 label=architecture/standard-contracts fact=fct-kllkxz3d
why: Link (2), node to handlers, and the "reacts to a click" reading must be read through the bridge's listener registration as it stands: one `data-dioxus-id` per listened element and a placeholder value, so the probe must say what this does and does not expose (event names, handler identity).
quote: DOM events route to the vdom via the nearest `data-dioxus-id` attribute parsed as `usize` (packages/dioxus-native-dom/src/dioxus_document.rs:47-55; packages/dioxus-native-dom/src/dioxus_document.rs:415-421); listener registration sets a `"<rust func>"` placeholder and `data-dioxus-id` (packages/dioxus-native-dom/src/mutation_writer.rs:338-345)
@carried n=2 label=architecture/standard-contracts fact=fct-kllkxz3d
why: The bridge-held APIs an "exposed" verdict may name are the ones this contract lists (`vdom`, `inner`, `vdom_state.try_element_to_node_id`), and the MountedData hazard sits in the contract's `NodeHandle` / `MountedData` surface, so the reading must cite these and change none of them.
quote: event converter for form, mouse, keyboard, focus, mounted, pointer, scroll, touch, wheel (packages/dioxus-native-dom/src/events.rs:42-138); `NodeHandle` backs `MountedData` (packages/dioxus-native-dom/src/events.rs:195-307); `DioxusDocument` exposes `vdom`, `inner`, `vdom_state.try_element_to_node_id(ElementId)`, which the guard of `flush_queued_mounted_events` (below) also reads (tests/blitz-tests/tests/stale_node_mapping.rs:66; tests/blitz-tests/tests/stale_node_mapping.rs:76-80)
@carried n=3 label=architecture/standard-contracts fact=fct-kllkxz3d
why: Link (1), node to component, must be measured against the vdom walk the stable ids already do (`VComponent.name`, the framework scopes skipped) and must not write anything to the DOM or the vdom.
quote: **stable element ids** — `DioxusDocument::element_id(NodeId) -> Option<String>` (`None` for a non-element, stale or detached node, never panics) and `element_ids() -> Vec<(NodeId, String)>` (every element reachable from the document root, document pre-order, ids pairwise distinct) compute the id on demand from a vdom walk joined with a DOM walk and write it to neither the DOM nor the vdom
@carried n=3 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: (3) the **component path** — `/`-joined component segments below the app root, where dioxus-core's four framework scopes (`ScopeId::ROOT` · `ROOT_SUSPENSE_BOUNDARY` · `ROOT_ERROR_BOUNDARY` · `APP`, the user root dioxus-core names `"root"`) add none, a segment is the last `::` segment of `VComponent.name` (dioxus-core's full type path) with generics stripped
@carried n=4 label=architecture/standard-contracts fact=fct-kllkxz3d
why: Whatever the probe reads of a component, scope or element, no `NodeId`, `ElementId`, `ScopeId` or pointer may enter a stable id.
quote: no `NodeId`, `ElementId`, `ScopeId` or pointer enters an id
@carried n=5 label=architecture/standard-contracts fact=fct-kllkxz3d
why: The listener reading already defined for the actionable check is `data-dioxus-id` for every listener except `onmounted`, read with no second id or role computation, so link (2)'s verdict must name that reader and say where it falls short.
quote: the `data-dioxus-id` attribute the bridge writes for every listener but `onmounted` — with ids from `element_ids()` and roles from `accessibility_tree`, never a second id or role computation
@carried n=6 label=architecture/standard-contracts fact=fct-llnppo2j
why: The MountedData fix options touch the mounted-event flush and its guard, and the Dioxus 0.7 divergence from upstream's test, so each option must say what it does to them.
quote: `flush_queued_mounted_events` skips an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239); upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0.
@carried n=7 label=architecture/standard-contracts fact=fct-2ylcpw3b
why: The stand's one key-handler `MountedData` caller is Cells' synchronous `scroll_to`, which must keep working under every fix option the founder is offered, and which marks the line between the synchronous case and the spawned-task case.
quote: arrows move `current`, clamped, into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, one `onmounted` per data cell holding the handles
@carried n=8 label=architecture/standard-contracts fact=fct-qmowki7i
why: `MountedData::scroll_to` is a named caller of the engine's `scroll_into_view`, whose signature is held for every caller, so no fix option may move that signature.
quote: its signature unchanged and for every caller: script `scrollIntoView`, Dioxus `MountedData::scroll_to`, fragment navigation and the test harness's input helper
@carried n=9 label=architecture/standard-contracts fact=fct-6qyfixyh
why: The hazard's mechanism lives in `DioxusDocument`'s `Document::poll`, so P3's re-verification at HEAD and any fix option must be stated against this trait contract, which the probe leaves as it is.
quote: `Document` trait — `inner`, `inner_mut`, default `handle_ui_event` via `EventDriver` with `NoopEventHandler`, default `poll` returning `false`, `id`, and — under the `accessibility` feature — a default `accessibility_tree() -> accesskit::TreeUpdate` returning `self.inner().build_accessibility_tree()`, which a wrapper may override to enrich nodes (`DioxusDocument` alone does, with stable ids) (packages/blitz-dom/src/document.rs:188-222), implemented for `PlainDocument`, `BaseDocument`, `Rc<RefCell<BaseDocument>>` (packages/blitz-dom/src/document.rs:224-251), `HtmlDocument` (packages/blitz-html/src/html_document.rs:7-70), `ScriptDocument` (packages/blitz-vibey-script/src/document.rs:402-446) and `DioxusDocument` (packages/dioxus-native-dom/src/dioxus_document.rs:259-331)
@carried n=10 label=architecture/standard-contracts fact=fct-6qyfixyh
why: Any probe that reads nodes from the changed set or from a mapping, for example for link (4), must resolve each id through `get_node` and never by indexing.
quote: a returned id may name a node dropped since it was written, so it is read through `get_node`, never by indexing
@carried n=11 label=architecture/standard-contracts fact=fct-kllkxz3d
why: Link (4), state to the nodes that read it, must not be read off the changed set as if it were the diff: the contract keeps the two apart and accepts a set flag next to an empty diff.
quote: the diff is not derived from the engine's changed set — that set is the cheap "something was written since the last drain", the diff what a snapshot reads differently, and the flag true beside an empty diff is a designed case
@carried n=12 label=architecture/standard-contracts fact=fct-oq4wpu6j
why: Link (2)'s event names, `click` among them, must be read through the existing event-kind mapping, which strips `on`, and not through a second name table.
quote: `DomEventKind::from_str` strips `on`, maps `composition` to `Ime` and `change` to `Change` (packages/blitz-traits/src/events.rs:165-212)
@carried n=13 label=architecture/standard-contracts fact=fct-et27vuvt
why: The hazard was measured through the harness's `poll(None)`, so a probe run that drives a handler must go through this settle/pump path and must not change it.
quote: `Harness::settle(&mut self) -> Result<Settled, NotSettled>` runs every piece of work that is due now and says whether anything is still outstanding: one pass is `pump`'s two calls in its order — `poll(None)`, then `resolve` at the harness's clock — with poll's answer kept
@carried n=14 label=architecture/standard-contracts fact=fct-syf56xsm
why: Each verdict must rest on a run of the stand's boots (Counter or CRUD, beside a served task), offline, in both layout modes through `options(incremental)`, and any test-local variant must mount in `TaskShell`; the stand must stay free of telemetry, env reads and `blitz_net`.
quote: `seven_guis::stand` — `LeanTask { Counter, FlightBooker, Timer, Crud }` with `ALL` and `task()`; pinned `VIEWPORT_WIDTH = 800`, `VIEWPORT_HEIGHT = 600`, `HIDPI_SCALE = 1.0`, `COLOR_SCHEME = ColorScheme::Light`; `font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`, system fonts off; `options(incremental) -> HarnessOptions` with the pinned viewport and bundled font, `net_provider: None` (offline through `DummyNetProvider`) and `base_url: None`, kept `None` for every caller (unit test `the_stand_options_stay_offline`), so every stand boot is offline but `boot_load`'s, which replaces that one field (below); `boot(LeanTask, HarnessOptions) -> Harness<DioxusDocument>` and `boot_timer(HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks)`, each building a fresh `VirtualDom` whose root provides a `TimerTicks` context and renders `app::task_in_shell` with a no-op back handler
@carried n=14 run=2 label=architecture/standard-contracts fact=fct-syf56xsm
quote: it installs no telemetry, reads no env var and names no `blitz_net`, its load page's stand-in included.
@carried n=14 run=3 label=architecture/standard-contracts fact=fct-syf56xsm
quote: `pub fn TaskShell`, the shell a test-local variant of a task mounts in so its unkeyed ids read as the task's
@carried n=15 label=architecture/standard-contracts fact=fct-jelmr2gx
why: A probe check placed in `tests/blitz-tests/` must be one auto-discovered file with no `[[test]]` table; if it is named `stand_*.rs` it joins `stand-repeat`'s zero-failure rounds; and any kept bridge code must hold the dioxus-native-dom coverage floor.
quote: the blitz-tests target contract — `[package]` does not set `autotests` to false, and the manifest holds exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false` — pinned by `test_blitz_tests_targets.py`
@carried n=15 run=2 label=architecture/standard-contracts fact=fct-jelmr2gx
quote: `stand-repeat` (since 2026-10-14-quality-gates, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs`
@carried n=15 run=3 label=architecture/standard-contracts fact=fct-jelmr2gx
quote: the first red command ends the leg with its status — a zero-failure budget, no retry, no `--ignored`, no env var
@carried n=15 run=4 label=architecture/standard-contracts fact=fct-jelmr2gx
quote: the `coverage` leg calls it with `packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`
@carried n=16 label=architecture fact=fct-ksgwc2yv
why: The probe adds nothing to the driver's surface: the schema counts stay as they are, no `NodeId` crosses, and nothing the probe reads is logged.
quote: the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
@carried n=17 label=architecture fact=fct-p37qjcmu
why: The bridge already skips event kinds that have no handler, so link (2)'s verdict must say whether that per-kind handler count can be read.
quote: Dioxus skips event kinds with zero handlers
@carried n=18 label=architecture fact=fct-7in62t3p
why: The MountedData fix is an engine change for every document. The last panic fix sets the precedent for how such a fix is classed (ratified, flagged upstreamable, merge surface owned by Upstream sync), and the fork's options must state their class the same way.
quote: the key-press panic's fix, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still naming the block, is one more direct engine change for every document, ratified, flagged upstreamable with nothing sent upstream, its merge surface a line owned by Upstream sync ahead of determinism and exploration.
@carried n=19 label=architecture fact=fct-656exhoj
why: Every reading must be taken on the pinned Dioxus version, with no pin move.
quote: Dioxus "0.7.3" (Cargo.toml:85-102)
@binding n=1
**Bridge listener registration ↔ scope link (2) and the "reacts to a click" reading**: link (2)'s verdict must say which readers already exist: the `data-dioxus-id` marker with its `"<rust func>"` placeholder, the actionable check's listener reading (every listener but `onmounted`) and the per-kind skip. It must also say whether a `click` listener can be told apart from any other listener without a fork-level change (architecture/standard-contracts fct-kllkxz3d) (architecture fct-p37qjcmu)
@binding n=2
**Stable-id vdom walk ↔ scope link (1)**: an "exposed" verdict for node to component must name the existing vdom walk (`VComponent.name`, the framework scopes skipped) as the API. Any reading kept may write nothing to the DOM or vdom and may put no `ElementId`, `ScopeId` or pointer into an id (architecture/standard-contracts fct-kllkxz3d)
@binding n=3
**MountedData / poll contract ↔ scope's folded CARRY and the founder's fork**: the reading must state the hazard against `DioxusDocument`'s `Document::poll` and the `NodeHandle` / `MountedData` surface. Each fix option must state its class: an engine change for every document, classed like the key-press panic fix. Each option must also state its effect on Cells' synchronous `MountedData::scroll_to`, on `scroll_into_view`'s held signature and on `flush_queued_mounted_events`' guard. This chunk decides none of it (architecture/standard-contracts fct-6qyfixyh) (architecture/standard-contracts fct-llnppo2j) (architecture/standard-contracts fct-2ylcpw3b) (architecture/standard-contracts fct-qmowki7i) (architecture fct-7in62t3p)
@binding n=4
**Headless stand and harness ↔ scope's "Measured, not read"**: each verdict must be run on `stand::boot` (Counter or CRUD) beside a served task, through `Harness::settle`'s `poll(None)` then `resolve` pass, in both layout modes via `options(incremental)`. Nodes must be resolved through `get_node`, and the changed set must not be read as the diff (architecture/standard-contracts fct-syf56xsm) (architecture/standard-contracts fct-et27vuvt) (architecture/standard-contracts fct-6qyfixyh) (architecture/standard-contracts fct-kllkxz3d)
@binding n=5
**CI contracts ↔ scope's "a probe check would live in tests/blitz-tests"**: a kept probe check must be one auto-discovered file and add no `[[test]]` table. A `stand_*.rs` name puts it in `stand-repeat`'s three zero-failure rounds. Bridge code it keeps must hold the dioxus-native-dom floor of 83 (architecture/standard-contracts fct-jelmr2gx)
@binding n=6
**Driver schema and pins ↔ scope's "Nothing crosses" and "No fork-level change"**: the twelve verbs, the eight causes and the no-`NodeId`, no-log rule must stay as they are, and Dioxus must stay at "0.7.3" (architecture fct-ksgwc2yv) (architecture fct-656exhoj)
