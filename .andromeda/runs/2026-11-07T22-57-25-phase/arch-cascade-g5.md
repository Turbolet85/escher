@carried n=1 label=architecture/standard-contracts fact=fct-6qyfixyh
why: the graph must be keyed on the agent-visible node the document trait's `accessibility_tree` hands out, the one `DioxusDocument` enriches with stable ids, never on a DOM `NodeId`.
quote: under the `accessibility` feature — a default `accessibility_tree() -> accesskit::TreeUpdate` returning `self.inner().build_accessibility_tree()`, which a wrapper may override to enrich nodes (`DioxusDocument` alone does, with stable ids) (packages/blitz-dom/src/document.rs:188-222)
@carried n=1 run=2 label=architecture fact=fct-srb7slak
quote: `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom
@carried n=1 run=3 label=architecture fact=fct-d5zuqqfb
quote: `DioxusDocument::unkeyed_actionable` lists, read-only, each actionable element whose stable id holds a `/`, with a remedy naming the fix.
@carried n=2 label=architecture/standard-contracts fact=fct-ivrm3he5
why: the graph's nodes must be the ones `build_accessibility_tree` builds, with their bounds, so the graph and the snapshot name the same agent-visible nodes.
quote: `build_accessibility_tree` returns an accesskit `TreeUpdate` of `(NodeId, accesskit::Node)` pairs, naming a node from five sources
@carried n=2 run=2 label=architecture fact=fct-4eadslhd
quote: `build_accessibility_tree` returns an AccessKit `TreeUpdate`
@carried n=3 label=architecture/standard-contracts fact=fct-oq4wpu6j
why: the «reacts to a click» reading must state which elements a Click reaches with an app handler, and Enter and a released Space reach the same Dioxus `onclick` through the queued activation Click.
quote: both through `dispatch_activation_click`, whose data is `Node::synthetic_click_event_data(modifiers)` and which the event driver queues so the app's handlers (a Dioxus `onclick`) run before the default action `handle_click`
@carried n=3 run=2 label=architecture fact=fct-3zkmzaom
quote: Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action
@carried n=4 label=architecture/standard-contracts fact=fct-qmowki7i
why: the graph reader that maps vdom nodes and readers' scopes onto document nodes must resolve each id through a checked read, never by indexing, since a stale id indexed into `NodeTree` panics.
quote: `NodeTree` `get`, `get_mut`, `contains_key`, `len`, `iter`, `iter_mut`, stale-id indexing panics (packages/blitz-dom/src/tree.rs:75-117; packages/blitz-dom/src/tree.rs:120-136) with "invalid SlotMap key used"
@carried n=4 run=2 label=architecture fact=fct-mjzuvlbq
quote: stale-id `NodeTree` indexing panics.
@carried n=4 run=3 label=architecture fact=fct-c5wyjq5i
quote: its ids read through `get_node`, never by indexing
@carried n=5 label=architecture/standard-contracts fact=fct-nycn5gmf
why: the pointer gates and activation keys read `is_disabled`, so the «reacts to a click» reading must state how a disabled control that carries a click handler reads, through this one reading.
quote: `ElementData::is_disabled(&self) -> bool`, public: `can_be_disabled()` — `button` · `input` · `select` · `textarea` — and a `disabled` attribute present; the one reading focusability, the pointer gates and the activation keys share
@carried n=5 run=2 label=architecture fact=fct-bedjim2b
quote: `ElementData::is_disabled` is the one disabled reading of a form control — a `disabled` attribute present
@carried n=6 label=architecture/standard-contracts fact=fct-syf56xsm
why: the stand's boots build each `VirtualDom` whose root provides its contexts, where the registry's context or wrapper hook must be handed the stand's signals, and the stand must still install no telemetry and read no env var.
quote: `boot(LeanTask, HarnessOptions) -> Harness<DioxusDocument>` and `boot_timer(HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks)`, each building a fresh `VirtualDom` whose root provides a `TimerTicks` context and renders `app::task_in_shell` with a no-op back handler
@carried n=6 run=2 label=architecture fact=fct-tewgiusr
quote: it installs no telemetry, reads no env var and names no `blitz_net`.
@carried n=7 label=architecture/standard-contracts fact=fct-ajw53qdr
why: Cells is one screen the starting «reacts to a click» reading is re-pinned on and whose signals move onto the registry, and its screen already passes the snapshot text's budget, so a graph read over it must be bounded or read in parts.
quote: `boot_cells(HarnessOptions) -> Harness<DioxusDocument>` mounts `Task::Cells` in `TaskShell` through the private `cells_root`, Cells staying no `LeanTask`: its screen passes the snapshot text's budget and reads in parts
@carried n=8 label=architecture/standard-contracts fact=fct-hczk2fzd
why: the load page is a served stand screen whose click-driven state moves onto the registry with the stand's other signals, its two keyed elements keeping their keys.
quote: `boot_load(HarnessOptions) -> Harness<DioxusDocument>` mounts a load page in `TaskShell` titled "Load" (the private `load_root` and `LoadPage`, its CSS `.load-root` · `.load-body` · `.load-btn` with `:hover` · `:active` · `.load-done`) holding two keyed elements — `load-fetch`, a Button "Load" in white on the stand accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), whose click links the page's one stylesheet
@carried n=9 label=architecture/standard-contracts fact=fct-gv2u2chj
why: a span field under an escher target prints unless named in `CONTENT_FIELDS`, so the graph read's `command` and `tool_call` spans must carry no node, component name, handler, state or value as a field.
quote: an event whose target starts with a prefix in `ESCHER_TARGET_PREFIXES` prints every field not in `CONTENT_FIELDS`, those redacted
@carried n=9 run=2 label=architecture fact=fct-ksgwc2yv
quote: no `NodeId` crosses, and nothing logs an id, a name or a value.
@carried n=9 run=3 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@binding n=1
**document core ↔ mutation, query and CSSOM**: the graph is keyed on the node `build_accessibility_tree` builds and `DioxusDocument`'s `accessibility_tree` enriches with stable ids, so a graph node and a snapshot node name one agent-visible node and its stable id, never a DOM `NodeId` (architecture/standard-contracts fct-6qyfixyh) (architecture/standard-contracts fct-ivrm3he5)
@binding n=2
**events ↔ nodes and widgets**: the «reacts to a click» boolean must be read so a pointer click and the keyboard's queued activation Click agree, and a disabled control, which the pointer gates and activation keys refuse through `is_disabled`, reads as the plan states it (architecture/standard-contracts fct-oq4wpu6j) (architecture/standard-contracts fct-nycn5gmf)
@binding n=3
**headless stand ↔ telemetry bootstrap**: the stand's signals move onto the registry with the stand still installing no telemetry, and the graph read's spans, printed under an escher prefix wherever a subscriber is installed, carry no graph content as a field (architecture/standard-contracts fct-syf56xsm) (architecture/standard-contracts fct-gv2u2chj)
