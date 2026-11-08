@carried n=1 label=architecture/standard-contracts fact=fct-6qyfixyh
why: DioxusDocument's Document implementation must keep its trait contract while the exercised record is added in dioxus_document.rs, reached whole by this chunk.
quote: `Document` trait — `inner`, `inner_mut`, default `handle_ui_event` via `EventDriver` with `NoopEventHandler`, default `poll` returning `false`, `id`,
@carried n=1 run=2 label=architecture/standard-contracts fact=fct-6qyfixyh
quote: `ScriptDocument` (packages/blitz-vibey-script/src/document.rs:402-446) and `DioxusDocument` (packages/dioxus-native-dom/src/dioxus_document.rs:279-354)
@carried n=1 run=3 label=architecture fact=fct-c5wyjq5i
quote: the `Document` trait every document wrapper implements
@carried n=2 label=architecture/standard-contracts fact=fct-7zhttwuk
why: the actions answer the owed replay test compares key by key is computed through target_of and is_select, which must keep answering as they do so a kept actions step replays equal.
quote: `is_select`, the driver's select test, read within the workspace by `execute.rs`'s crate-private `target_of` alone (packages/escher-driver/src/execute.rs:443), which `locate` ends in and `Session::actions` also calls
@carried n=2 run=2 label=architecture fact=fct-d5zuqqfb
quote: beside it the read-only `DioxusDocument::actionable_nodes` lists, in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an `ActionableNode` (id, owner, `keyed` where the id holds no `/`)
@carried n=3 label=architecture/standard-contracts fact=fct-oq4wpu6j
why: the exercised record must count every (node, event) listener a dispatch reaches through this contract, bubbling and compatibility events included, not only the target's own event.
quote: `EventHandler::handle_event(node chain, &mut DomEvent, document, EventState)` (packages/blitz-dom/src/events/driver.rs:9-17); `NoopEventHandler` (packages/blitz-dom/src/events/driver.rs:19-30); `EventDriver::new(doc, handler)` with `handle_ui_event` and `handle_dom_event` (packages/blitz-dom/src/events/driver.rs:38-45; packages/blitz-dom/src/events/driver.rs:168; packages/blitz-dom/src/events/driver.rs:310-313; tests/blitz-tests/tests/scrollbar_drag.rs:35-50), re-exported from the events module (packages/blitz-dom/src/events/mod.rs:9); pointer events produce mouse compatibility events for mouse input and touch events for finger and pen (packages/blitz-dom/src/events/driver.rs:327-349); non-bubbling events reach only the target (packages/blitz-dom/src/events/driver.rs:378-383)
@carried n=3 run=2 label=architecture/standard-contracts fact=fct-oq4wpu6j
quote: `DomEventKind::from_str` strips `on`, maps `composition` to `Ime` and `change` to `Change` (packages/blitz-traits/src/events.rs:165-212)
@carried n=3 run=3 label=architecture/standard-contracts fact=fct-oq4wpu6j
quote: both through `dispatch_activation_click`, whose data is `Node::synthetic_click_event_data(modifiers)` and which the event driver queues so the app's handlers (a Dioxus `onclick`) run before the default action `handle_click` (packages/blitz-dom/src/events/keyboard.rs:362-372)
@carried n=3 run=4 label=architecture fact=fct-srb7slak
quote: DOM events reach the vdom through `data-dioxus-id`
@carried n=4 label=architecture/standard-contracts fact=fct-d6tfdc2g
why: a click on a checkbox or radio reaches the app's change listener through an engine dispatch, so the exercised record must count that change handler as reached exactly where the engine dispatches it.
quote: `handle_click`'s checkbox and radio arms dispatch `DomEventData::Change` after `input` with the same value, a checkbox always, a radio only where it was not checked before the click
@carried n=4 run=2 label=architecture fact=fct-3zkmzaom
quote: the engine dispatches `change` after `input` on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and a page's script hears that one `change`
@carried n=5 label=architecture/standard-contracts fact=fct-syf56xsm
why: the recorded checks run on the stand and the owed replay test must replay on a fresh session of the same app, which each stand boot builds as a fresh VirtualDom.
quote: `boot(LeanTask, HarnessOptions) -> Harness<DioxusDocument>` and `boot_timer(HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks)`, each building a fresh `VirtualDom` whose root provides a `TimerTicks` context and renders `app::task_in_shell` with a no-op back handler
@carried n=5 run=2 label=architecture fact=fct-tewgiusr
quote: `seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`) in a pinned 800x600 Light viewport with one bundled font
@carried n=5 run=3 label=architecture fact=fct-elnqiowc
quote: The recording keeps a session's answered calls as a kept check, a masked `type` as its stand-in and a shot without its pixels; `Check::replay` runs them again through `Session::run`
@carried n=6 label=architecture/standard-contracts fact=fct-fonypmyx
why: every stand task registers graphed state through use_graph_state, so the exercised record must be shown to hold on an app with no registry too, not only on the stand's registry-bearing apps.
quote: `tasks::use_graph_signal(name, writers, init) -> Signal<T>` wraps `use_signal` and `use_graph_state` (§Dioxus DOM bridge) in `tasks/mod.rs`
@carried n=6 run=2 label=architecture fact=fct-srb7slak
quote: an app registers graphed state through the ungated `use_graph_state` (`DeclaredWrite`, `SignalRegistry`), and the bridge's `handle_event` sets the registry's dispatch around the vdom's only where a registry stands.
@carried n=7 label=architecture/standard-contracts fact=fct-gv2u2chj
why: a coverage verb's span sits under an escher_ target that prints every field outside CONTENT_FIELDS, so the coverage code must put no graph id, component or event name of the list into a span or event field.
quote: an event whose target starts with a prefix in `ESCHER_TARGET_PREFIXES` prints every field not in `CONTENT_FIELDS`, those redacted
@carried n=7 run=2 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@carried n=7 run=3 label=architecture fact=fct-ksgwc2yv
quote: no `NodeId` crosses, and nothing logs an id, a name or a value.
@binding n=1
**Events contract ↔ Dioxus DOM bridge (UI graph)**: the exercised record must name a reached handler by the graph's own (node id, `GraphHandler.event`) key, the dispatched event's name read as `DomEventKind::from_str` maps it, and must count every listener the dispatch reaches — bubbling ancestors, mouse compatibility events, the queued activation click and an engine `change` — so the unexercised set is read against the same handlers the graph lists (architecture/standard-contracts fct-oq4wpu6j) (architecture/standard-contracts fct-d6tfdc2g) (architecture fct-srb7slak)
@binding n=2
**Headless stand graph hooks ↔ graph-state registry**: the stand's tasks all register through `use_graph_state`, and the bridge sets the registry's dispatch only where a registry stands, so the record of reached handlers must sit outside the registry and be measured on an app with no registry beside the stand's (architecture/standard-contracts fct-fonypmyx) (architecture fct-srb7slak)
@binding n=3
**Headless stand ↔ kept checks and replay**: the owed replay test must record an `actions` step on a stand boot, keep it, read it back through `Check::from_bytes` and replay it through `Check::replay` on a second fresh boot of the same task, its actions answer — computed through `target_of` and `actionable_nodes` — equal key by key (architecture/standard-contracts fct-syf56xsm) (architecture/standard-contracts fct-7zhttwuk) (architecture fct-elnqiowc)
@binding n=4
**Telemetry bootstrap ↔ driver session**: a coverage verb or read must emit its `command`/`tool_call` span with no field holding the coverage list's ids or names, since an escher target prints every field outside `CONTENT_FIELDS` and the driver logs no id, name or value (architecture/standard-contracts fct-gv2u2chj) (architecture fct-ksgwc2yv) (architecture fct-tev7ieys)
@binding n=5
**Document core ↔ Dioxus DOM bridge**: any event-path change in `DioxusDocument` that records reached handlers must keep its `Document` implementation's `handle_ui_event` and `poll` contract for every caller (architecture/standard-contracts fct-6qyfixyh) (architecture fct-c5wyjq5i)
