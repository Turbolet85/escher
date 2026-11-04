@carried n=1 label=architecture/standard-contracts fact=fct-6qyfixyh
why: the chunk must re-read at HEAD the `poll` of `DioxusDocument`'s `Document` impl, the path whose `render_immediate` holds the document's RefCell, and link (1) may read the stable ids its `accessibility_tree` override adds, all with no change to that impl.
quote: default `poll` returning `false`, `id`, and — under the `accessibility` feature — a default `accessibility_tree() -> accesskit::TreeUpdate` returning `self.inner().build_accessibility_tree()`, which a wrapper may override to enrich nodes (`DioxusDocument` alone does, with stable ids) (packages/blitz-dom/src/document.rs:188-222), implemented for `PlainDocument`, `BaseDocument`, `Rc<RefCell<BaseDocument>>` (packages/blitz-dom/src/document.rs:224-251), `HtmlDocument` (packages/blitz-html/src/html_document.rs:7-70), `ScriptDocument` (packages/blitz-vibey-script/src/document.rs:402-446) and `DioxusDocument` (packages/dioxus-native-dom/src/dioxus_document.rs:259-331)
@carried n=1 run=2 label=architecture fact=fct-srb7slak
quote: DOM events reach the vdom through `data-dioxus-id`; a falsy value of upstream's 28 boolean attributes clears the attribute; `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
@carried n=1 run=3 label=architecture fact=fct-54e4fuk7
quote: every window event is followed by a Poll (packages/blitz-shell/src/application.rs:162-165)
@carried n=2 label=architecture/standard-contracts fact=fct-oq4wpu6j
why: link (2)'s answer for `click` and link (3)'s reading must state that an activation Click is queued so the app's Dioxus `onclick` runs before the engine's default action, the order a handler's state write is read in.
quote: both through `dispatch_activation_click`, whose data is `Node::synthetic_click_event_data(modifiers)` and which the event driver queues so the app's handlers (a Dioxus `onclick`) run before the default action `handle_click` (packages/blitz-dom/src/events/keyboard.rs:362-372)
@carried n=2 run=2 label=architecture fact=fct-3zkmzaom
quote: Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action;
@carried n=3 label=architecture/standard-contracts fact=fct-oq4wpu6j
why: link (2)'s verdict must name a listener's event as the listener map names it, and an `on`-prefixed name, `composition` or `change` maps to its event kind by this rule, so the `click` reading names `click`.
quote: `DomEventKind::from_str` strips `on`, maps `composition` to `Ime` and `change` to `Change` (packages/blitz-traits/src/events.rs:165-212)
@carried n=4 label=architecture/standard-contracts fact=fct-et27vuvt
why: the MountedData hazard must be re-measured at HEAD through the harness's `poll(None)` as the engine-debts reading measured it, and every probe run on `Harness<DioxusDocument>` pumps that poll then a resolve.
quote: one pass is `pump`'s two calls in its order — `poll(None)`, then `resolve` at the harness's clock — with poll's answer kept;
@carried n=4 run=2 label=architecture/standard-contracts fact=fct-w5qojtql
quote: `time()` reads that clock in seconds and `pump` resolves at it; `tick(dt)` advances it, a negative or non-finite `dt` moving nothing, and `advance_time(&mut self, by: Duration)` advances it by exactly `by`, each then pumping;
@carried n=4 run=3 label=architecture fact=fct-gkkvjkmi
quote: blitz-test-harness's `Harness`, generic over the document, with constructors, inspection and input helpers that pump and never settle;
@carried n=4 run=4 label=architecture fact=fct-23rvgess
quote: The harness holds one `VirtualClock` its documents share; `tick` and `advance_time` move it, and `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never do.
@carried n=5 label=architecture/standard-contracts fact=fct-syf56xsm
why: each verdict must rest on a run of a stand lean task (Counter or CRUD) booted by `boot` with `options(incremental)`, offline in the pinned viewport, in both layout modes where layout touches the reading.
quote: `seven_guis::stand` — `LeanTask { Counter, FlightBooker, Timer, Crud }` with `ALL` and `task()`; pinned `VIEWPORT_WIDTH = 800`, `VIEWPORT_HEIGHT = 600`, `HIDPI_SCALE = 1.0`, `COLOR_SCHEME = ColorScheme::Light`; `font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`, system fonts off; `options(incremental) -> HarnessOptions` with the pinned viewport and bundled font, `net_provider: None` (offline through `DummyNetProvider`) and `base_url: None`, kept `None` for every caller (unit test `the_stand_options_stay_offline`), so every stand boot is offline but `boot_load`'s, which replaces that one field (below); `boot(LeanTask, HarnessOptions) -> Harness<DioxusDocument>`
@carried n=5 run=2 label=architecture/standard-contracts fact=fct-ajw53qdr
quote: `boot_cells(HarnessOptions) -> Harness<DioxusDocument>` mounts `Task::Cells` in `TaskShell` through the private `cells_root`, Cells staying no `LeanTask`
@carried n=5 run=3 label=architecture/standard-contracts fact=fct-hczk2fzd
quote: `boot_load(HarnessOptions) -> Harness<DioxusDocument>` mounts a load page in `TaskShell` titled "Load"
@carried n=5 run=4 label=architecture fact=fct-tewgiusr
quote: `seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`) in a pinned 800x600 Light viewport with one bundled font, every boot offline but the load page's
@carried n=5 run=5 label=architecture fact=fct-fukxpbvx
quote: `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks — the four lean ones, Cells and the load page,
@carried n=6 label=architecture/standard-contracts fact=fct-syf56xsm
why: probe code kept on the stand must keep it installing no telemetry, reading no env var and naming no `blitz_net`, the scope adding no env var and logging nothing.
quote: it installs no telemetry, reads no env var and names no `blitz_net`, its load page's stand-in included.
@carried n=7 label=architecture/standard-contracts fact=fct-gv2u2chj
why: the probe must add no log field or span and put no component name, handler or signal value it reads in a field, since under an escher target every field outside `CONTENT_FIELDS` prints.
quote: an event whose target starts with a prefix in `ESCHER_TARGET_PREFIXES` prints every field not in `CONTENT_FIELDS`, those redacted;
@carried n=7 run=2 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@carried n=7 run=3 label=architecture fact=fct-ksgwc2yv
quote: the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
@binding n=1
**Document core ↔ Dioxus DOM bridge ↔ the scope's MountedData re-read**: the `poll` the chunk re-verifies at HEAD is `DioxusDocument`'s `Document` impl (architecture/standard-contracts fct-6qyfixyh), the same document whose events reach the vdom through `data-dioxus-id` and whose `element_id` tiers link (1) reads, written to neither DOM nor vdom (architecture fct-srb7slak); the windowed path the reading states as read, not run, is the shell's Poll after every window event (architecture fct-54e4fuk7).
@binding n=2
**Events ↔ Event processing (cross-cutting)**: link (2)'s listener names and link (3)'s handler order are read through the engine's naming of event kinds and its queued activation Click, handlers before the default action (architecture/standard-contracts fct-oq4wpu6j), beside Dioxus skipping event kinds with zero handlers (architecture fct-p37qjcmu), so a `click` with no listener reads as no listener.
@binding n=3
**Test harness ↔ Headless stand**: the hazard's re-measurement and every verdict's run go through a stand boot that returns `Harness<DioxusDocument>` (architecture/standard-contracts fct-syf56xsm), whose pump is `poll(None)` then a resolve at the harness's one `VirtualClock` (architecture/standard-contracts fct-et27vuvt; architecture/standard-contracts fct-w5qojtql), both layout modes reached through `options(incremental)`.
@binding n=4
**Headless stand ↔ Driver session ↔ CLIs**: the served task run beside a lean one is one of the six served stand tasks (architecture fct-fukxpbvx), Cells or the load page among them (architecture/standard-contracts fct-ajw53qdr; architecture/standard-contracts fct-hczk2fzd), reached through a `Session` whose twelve verbs and eight causes the probe leaves as they are (architecture fct-ksgwc2yv).
@binding n=5
**Telemetry bootstrap ↔ Occupied resources (environment variables)**: the probe logs no value it reads and adds no field or span (architecture/standard-contracts fct-gv2u2chj), and the stand it runs on installs no telemetry and reads no env var (architecture/standard-contracts fct-syf56xsm), so the probe adds no env var to the list `RUST_LOG` heads (architecture fct-yhllvhvr).
@binding n=6
**Document core ↔ Inherited Defaults (Dioxus pin)**: every link is read through the bridge-held `DioxusDocument` API (architecture/standard-contracts fct-6qyfixyh) against Dioxus held at "0.7.3" (architecture fct-656exhoj; architecture fct-7lmrueug), with no patch, vendored copy or pin move.
