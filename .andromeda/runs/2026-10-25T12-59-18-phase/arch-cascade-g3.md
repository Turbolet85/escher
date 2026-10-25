# arch · cascade read · group 3 of 7

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:120) The stand the chunk widens must keep or amend, term by term, its pinned viewport, bundled font, offline boot through `DummyNetProvider` and its no-telemetry, no-env-read, no-`blitz_net` rule while `cells`, the select and a held load join it.
  > `seven_guis::stand` — `LeanTask { Counter, FlightBooker, Timer, Crud }` with `ALL` and `task()`; pinned `VIEWPORT_WIDTH = 800`, `VIEWPORT_HEIGHT = 600`, `HIDPI_SCALE = 1.0`, `COLOR_SCHEME = ColorScheme::Light`; `font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`, system fonts off; `options(incremental) -> HarnessOptions` with the pinned viewport and bundled font, `net_provider: None` (offline through `DummyNetProvider`) and `base_url: None`; `boot(LeanTask, HarnessOptions) -> Harness<DioxusDocument>` and `boot_timer(HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks)`, each building a fresh `VirtualDom` whose root provides a `TimerTicks` context and renders `app::task_in_shell` with a no-op back handler;
  (.andromeda/architecture-detail/standard-contracts.md:120)
  > it installs no telemetry, reads no env var and names no `blitz_net`.
  (.andromeda/architecture-detail/standard-contracts.md:121)
  > Supporting seven_guis API: `app::Task` (pub) and `app::task_in_shell(Task, EventHandler<()>) -> Element`, and `pub fn TaskShell`, the shell a test-local variant of a task mounts in so its unkeyed ids read as the task's
  (.andromeda/architecture.md:163)
  > - **Headless stand (seven_guis, native only):** `seven_guis::stand` boots the four lean tasks offline in a pinned 800x600 Light viewport with one bundled font, and `timer_step` hands the Timer its ticks for a driver session; it installs no telemetry, reads no env var and names no `blitz_net`.
- (.andromeda/architecture-detail/standard-contracts.md:107) The load wait the stand must carry is read by `settle`, which must report a load in flight at once and never wait on it or move time, so the stand's held load has to read as `Busy::Loads`.
  > `Harness::settle(&mut self) -> Result<Settled, NotSettled>` runs every piece of work that is due now and says whether anything is still outstanding:
  (.andromeda/architecture-detail/standard-contracts.md:107)
  > after a quiet pass a request in flight, or a pending render-blocking resource, returns `NotSettled { busy: Busy::Loads }` at once, and otherwise `Settled { passes: u32, animating: bool }`;
  (.andromeda/architecture-detail/standard-contracts.md:107)
  > Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves
  (.andromeda/architecture-detail/standard-contracts.md:108)
  > Loads are counted where every request crosses: `HarnessOptions`' `net_provider`, when one is supplied and its `is_noop()` is false, is wrapped in a crate-private counting `NetProvider` that forwards `fetch` and `is_noop`, counts a request in flight until its handler is consumed or dropped, and stores counts only; an offline harness (the stand's) and a document passed to `wrap` carry no counter and are read through `has_pending_critical_resources()` alone
  (.andromeda/architecture-detail/standard-contracts.md:106)
  > `HarnessOptions { width, height }` with `Default` (tests/blitz-tests/tests/dir_attribute.rs:23-27) — eight public fields: width, height, scale, color_scheme, base_url, net_provider, `font_ctx: Option<FontContext>` → `DocumentConfig.font_ctx` and `incremental: Option<bool>` → `DocumentConfig.incremental`
  (.andromeda/architecture.md:159)
  > - **Test harness:** blitz-test-harness's `Harness`, generic over the document, with constructors, inspection and input helpers that pump and never settle; `screenshot` and `screenshot_rect` paint a raster through blitz-paint, printing, panicking and logging nothing; `text_contrast` reads a WCAG contrast and changes nothing; `settle` drives render and layout to quiet, reports a load in flight and never waits on one or moves time.
- (.andromeda/architecture-detail/standard-contracts.md:104) The contrast rows `stand_contrast` re-pins for the select are read by `text_contrast`, which must change nothing and pick its text-bearing element by this rule.
  > `Harness::text_contrast(&self, NodeId) -> Option<TextContrast>` reads and changes nothing — no `doc.mutate()`, input, pump or settle, every node reached by `get_node`: the text-bearing element is the node itself when it is a text input, else the parent element of the first text node under it that holds a character other than whitespace;
- (.andromeda/architecture-detail/standard-contracts.md:105) `stand_actionable_keys`, a reached check, scrolls each card into view through this helper before its raw click, which settles nothing.
  > the input helper `scroll_into_view(&mut self, node_id: NodeId)` calls `BaseDocument::scroll_into_view` with `ScrollBehavior::Instant` and `ScrollLogicalPosition::Nearest` on both axes — every scrolling box that holds the node, then the viewport, by the least movement on each axis — and pumps; like every input helper it does not settle
- (.andromeda/architecture-detail/standard-contracts.md:103) A replayed agent check that holds a `screenshot` step is painted through this shot, which must resolve at the harness's own time and refuse a document that cannot paint text.
  > `Harness::screenshot(&mut self) -> Result<Raster, ScreenshotError>` and `Harness::screenshot_rect(&mut self, PixelRect) -> Result<Raster, ScreenshotError>` resolve at the harness's own time (no tick, no pump),
- (.andromeda/architecture-detail/standard-contracts.md:114) The stand's `advance` checks and the cold agent's `advance` goal must move only the harness's one `VirtualClock`, which every stand document reads and which a shot, a contrast read and a settle never move.
  > the harness holds a `VirtualClock` its documents share — `HarnessOptions::into_config(self, &VirtualClock)` puts a clone in `DocumentConfig.clock`, so `from_html_with` and `from_vdom`, and through them `from_html` and `from_component`, build documents that read it,
  (.andromeda/architecture-detail/standard-contracts.md:114)
  > `tick(dt)` advances it, a negative or non-finite `dt` moving nothing, and `advance_time(&mut self, by: Duration)` advances it by exactly `by`, each then pumping; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it
  (.andromeda/architecture.md:161)
  > - The harness holds one `VirtualClock` its documents share; `tick` and `advance_time` move it, and `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never do.
- (.andromeda/architecture-detail/standard-contracts.md:16) Every time read of a stand document, the select's typeahead window included, must come from the config's clock, so the agent's `advance` moves it and nothing reads wall time.
  > `DocumentConfig.clock: Option<Arc<dyn Clock>>`, `None` in `Default`; a document holds the config's clock, else a fresh `WallClock` — wall time, as before — and reads it for the overlay scrollbar fade, the double-click window, pan and the fling's generation, the smooth-scroll start and the fling and smooth frames, a sub-document's config carrying its parent's clock
  (.andromeda/architecture.md:135)
  > - The document's clock: `DocumentConfig.clock`, the config's clock or else a fresh `WallClock`; the crate root re-exports `Clock`, `WallClock` and `VirtualClock`, every clone of a `VirtualClock` reading one time; no `Instant::now()` or `SystemTime::now()` stands in blitz-dom outside `clock.rs`; `BaseDocument::select_value` reads a select's value, public and read-only, its writers crate-private.
- (.andromeda/architecture-detail/standard-contracts.md:17) The flight booker's select must report its choice through the read-only `select_value`, the snapshot's `value` and `apply_select_choice` its callers, its writers left crate-private.
  > `BaseDocument::select_value(&self, select: NodeId) -> Option<String>`, public and read-only: HTML's `select.value` — the chosen option's value, `None` with none chosen or for a node that is no select; dioxus-native-dom's snapshot `value` and `apply_select_choice` its callers
- (.andromeda/architecture-detail/standard-contracts.md:76) The select's arrows, `Home` · `End` and typeahead — the driver's `type` on the flight booker's select through it — must run on the crate-private rules, no public type added.
  > The crate-private module `node::select` holds the select's rules — `OptionReading`, `SelectStep` (`Next` · `Previous` · `First` · `Last`), `Typeahead` and `TYPEAHEAD_TIMEOUT` (1 s), and the readers `is_single_select`, `is_select_option`, `options`, `is_chosen`, `chosen_option`, `readings`, `option_label`, `option_value`, `settled`, `stepped` and `matched` — no public type
- (.andromeda/architecture-detail/standard-contracts.md:75) The flight booker's select is a form control, so its focusability in the re-pinned Tab order, its pointer gate and its keys must read `disabled` through this one reading.
  > `ElementData::is_disabled(&self) -> bool`, public: `can_be_disabled()` — `button` · `input` · `select` · `textarea` — and a `disabled` attribute present; the one reading focusability, the pointer gates and the activation keys share
  (.andromeda/architecture.md:151)
  > - `ElementData::is_disabled` is the one disabled reading — a form control with a `disabled` attribute present; the select's rules live in the crate-private `node::select`, with no public type.
- (.andromeda/architecture-detail/standard-contracts.md:34) The Enter and Space tables `stand_keyboard` re-pins read keyboard activation, which a select — no button — does not take, so the flight booker's two buttons leaving moves those rows.
  > a released Space on the focused node dispatches `DomEventData::Click`
  (.andromeda/architecture-detail/standard-contracts.md:35)
  > `handle_key_or_input_event` dispatches the same Click for a pressed `Enter` on the focused node after its text-input and range arms (packages/blitz-dom/src/events/keyboard.rs:114-119), both through `dispatch_activation_click`, whose data is `Node::synthetic_click_event_data(modifiers)` and which the event driver queues so the app's handlers (a Dioxus `onclick`) run before the default action `handle_click`
  (.andromeda/architecture.md:141)
  > - **Events:** `EventHandler` and `EventDriver` dispatch DOM events — mouse compatibility and touch events from pointers, non-bubbling events to the target only, a focus change as `blur`, `focusout`, `focus`, `focusin`; a range input's default actions dispatch `input`; Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action; the engine dispatches `change` only after `input` on a select's choice.
- (.andromeda/architecture-detail/standard-contracts.md:36) The flight booker's select handler must take the choice from the engine's `input` (then `change`) event, the only `change` the engine dispatches.
  > `DomEventKind::Change`, after `Input`, and `DomEventData::Change(BlitzInputEvent)` — named `change`, not cancelable, bubbling, with no UI event mapped and no default action — are dispatched by the engine only after `input` on a select's choice, a checkbox, radio or text input firing no engine `change`
- (.andromeda/architecture-detail/standard-contracts.md:24) The new select's options must be named by their labels in the tree and the snapshot the re-pinned snapshot, diff and id checks read.
  > a select's option's non-empty `label` attribute where it has no `aria-label`
  (.andromeda/architecture-detail/standard-contracts.md:25)
  > since 2026-10-10-snapshot-states the builder (`BaseDocument::build_accessibility_node`, private) also reads state attributes on every element — a select's option excepted, whose `selected` is its chosen state, true or false, with no `aria-selected` read
  (.andromeda/architecture-detail/standard-contracts.md:25)
  > `aria-pressed`: `true` · `false` · `mixed` sets `toggled` `True` · `False` · `Mixed`, anything else nothing, on every role but the five checkable ones;
  (.andromeda/architecture-detail/standard-contracts.md:26)
  > `aria-invalid`: `true` · `grammar` · `spelling` sets `invalid` `True` · `Grammar` · `Spelling`, an absent, empty (after trimming) or `false` value nothing, and any other value `Invalid::True`
  (.andromeda/architecture.md:137)
  > `build_accessibility_tree` returns an AccessKit `TreeUpdate`, naming nodes from text children, `aria-labelledby`, `aria-label`, a bound `<label>` and option labels, and writing state attributes each read once;
- (.andromeda/architecture-detail/standard-contracts.md:8) A load the stand holds must arrive through the document config's providers and the document's resource messages, never a path beside them.
  > `DocumentConfig` fields: viewport, base_url, ua_stylesheets, net/navigation/shell/html-parser providers, font_ctx, media_type, style_threading, incremental, abort_signal, subdocument_depth, clock
  (.andromeda/architecture-detail/standard-contracts.md:8)
  > `DocumentEvent` messages `ResourceLoad(ResourceLoadResponse)` and `NavigateIframe { node_id, url }`
  (.andromeda/architecture-detail/standard-contracts.md:9)
  > `Resource` enum Image, Svg, Css, ImportedCss, Font, DocumentSrc, None and `ResourceLoadResponse { request_id, node_id, resolved_url, Result<Resource, String> }`
- (.andromeda/architecture-detail/standard-contracts.md:9) The re-pinned diff checks and every acting step's diff on the moved stand read the changed set, which must mark from the first tracked write until the drain.
  > `BaseDocument::has_changes(&self) -> bool` reads true while the changed set (`changed_nodes`, a `pub(crate)` `HashSet<NodeId>`) is non-empty, from the first tracked write until the next drain, and `BaseDocument::take_changed_nodes(&mut self) -> HashSet<NodeId>` returns the set and leaves it empty;
  (.andromeda/architecture-detail/standard-contracts.md:10)
  > `BaseDocument::can_paint_text(&self) -> bool` — whether the font context resolves a family for the `sans-serif` generic
  (.andromeda/architecture.md:133)
  > the changed set read by `has_changes` and drained by `take_changed_nodes`, its ids read through `get_node`, never by indexing; the read-only `can_paint_text` and `layout_as_text_input`.
- (.andromeda/architecture-detail/standard-contracts.md:84) How the stand carries a load in flight turns on a `NetProvider` that is not a no-op, since a no-op one registers no pending-critical resource for settle to read.
  > `NetProvider::fetch(doc_id, Request, Box<dyn NetHandler>)` plus `is_noop()` defaulting to false, with callers not registering pending-critical resources when true
  (.andromeda/architecture-detail/standard-contracts.md:84)
  > `NetHandler::bytes(self: Box<Self>, resolved_url: String, bytes: Bytes)`
  (.andromeda/architecture.md:153)
  > - **Provider traits (blitz-traits):** `NetProvider` (a no-op one registering no pending-critical resources), `NetHandler`, `NetWaker`, `Request`, `Body`, `FormData`, `AbortController`/`AbortSignal`, `NavigationProvider`, `ShellProvider` with no-op defaults, `HtmlParserProvider`, `DevtoolSettings` and `Viewport`, its `scale()` hidpi_scale × zoom.
  (.andromeda/architecture.md:255)
  > tests dispatch loads through a pluggable `NetProvider` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24; tests/blitz-tests/tests/link_rel_attribute.rs:34-41)
- (.andromeda/architecture-detail/standard-contracts.md:90) blitz-net's `Provider` is a real fetch path, so a stand-held load must not route through it while the stand names no `blitz_net` — taking it is a P4 fork, not a silent add.
  > blitz-net `Provider` implements `NetProvider::fetch`, waking the waker for `doc_id` then calling `handler.bytes`
  (.andromeda/architecture.md:155)
  > - **HTML and networking crates:** blitz-html's `HtmlDocument`, `DocumentHtmlParser` (an html5ever `TreeSink`) and `HtmlProvider`; blitz-net's `Provider` implementing `NetProvider::fetch`, with typed `ProviderError`s and byte, form or empty request bodies.
- (.andromeda/architecture-detail/standard-contracts.md:43) `cells`' grid, served to the driver, must be brought into view through every scrolling box that holds a target when the agent or a re-pinned check scrolls to a cell.
  > `scroll_into_view` scrolls every scrolling box on its target's containing-block chain, innermost first, and then the viewport
  (.andromeda/architecture-detail/standard-contracts.md:44)
  > `visible_region(&self, node_id) -> Option<BoundingRect>` is the read-only reader beside it
  (.andromeda/architecture-detail/standard-contracts.md:44)
  > whether an element can be seen is read from that geometry and never from a hit;
  (.andromeda/architecture.md:143)
  > `scroll_into_view` scrolls every scrolling box on the target's chain, nested ones instantly with no `scroll` event, then the viewport; `visible_region` is the read-only reader of what can be seen, and seeing is never read from a hit; a hit stops at the padding box of a box paint clips and skips a node dropped since the last resolve;
- (.andromeda/architecture-detail/standard-contracts.md:54) The cold agent's lint goal and the moved stand screens are read through `text_extent`, which must stay read-only and answer `None` where no inline layout stands.
  > it reads `None` for an id that does not resolve (`get_node`), a node that is no inline root, or one with no inline layout data. It mutates nothing and has no log site, print, env read or panic path. Its limits: RTL and vertical text are read as LTR horizontal lines, and a transformed box is read at its client rect — each pinned by blitz-tests' `text_extent` in both layout modes (test-plan §1). Its one non-test caller is escher-driver's `Session::lint` (§Driver session).
  (.andromeda/architecture.md:145)
  > `BaseDocument::text_extent` is a read-only reader that mutates nothing.
- (.andromeda/architecture-detail/standard-contracts.md:128) The live run's wrong calls are counted from the sink's closed-span lines, so the pipe must read the `tool_call` span in this shape and the chunk adds no field to it.
  > is one line per closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited: the same prefix, then `span={the span's name}` (`span` a reserved field name), the span's own fields in the order recorded, then the layer's close fields `message`, `time.busy` and `time.idle`, level and target the span's;
  (.andromeda/architecture-detail/standard-contracts.md:127)
  > Line shape, one per printed event: `{RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…`
  (.andromeda/architecture-detail/standard-contracts.md:129)
  > an event whose target starts with a prefix in `ESCHER_TARGET_PREFIXES` prints every field not in `CONTENT_FIELDS`, those redacted; an event whose target starts with a prefix of neither set is dropped whole
  (.andromeda/architecture.md:165)
  > - **Telemetry bootstrap (escher-telemetry):** `init` installs one stderr sink once, `ForeignSubscriber` (never a panic) when another is global; one line per event and per closed span, stamped `service.name` and `service.version`; a target outside the engine and escher prefix sets is dropped whole at every level, an engine target prints only `SAFE_FIELDS`, `CONTENT_FIELDS` always redacted; the panic hook chains.
- (.andromeda/architecture.md:167) The chunk exercises the driver on the widened stand and must add no verb, cause or refusal content, every call still through `Session::run`, nothing logging an id, name or value of the new screens.
  > - **Driver session (escher-driver):** a `Session` (one booted app) runs every call through `Session::run`, which validates first; nothing public runs a `Command` that did not come from `validate`; the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
- (.andromeda/architecture.md:171) Recording with replay, lint and the load wait are three of the goals the cold agent exercises, and the acceptance's «passes on replay» reads one of these replays.
  > - The recording keeps a session's answered calls as a kept check, a masked `type` as its stand-in and a shot without its pixels; `Check::replay` runs them again through `Session::run`; `Session::lint` reads the screen and changes nothing; `LOAD_WAIT` (5 s) bounds an acting step's wait for a load, moving no time.
- (.andromeda/architecture.md:177) `cells` and the new select reach the driver through stable ids and the read-only snapshot, so every actionable cell and the select must read an author key.
  > `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
  (.andromeda/architecture.md:181)
  > - Every element an agent can act on reads an author key: `DioxusDocument::unkeyed_actionable` lists, read-only, each actionable element whose stable id holds a `/`, with a remedy naming the fix.
  (.andromeda/architecture.md:183)
  > - `is_masked_input` is the one masked-control test, a password or file `input`; a select's `input` and `change` reach the app as form data, and its options are its children in the snapshot.
  (.andromeda/architecture.md:179)
  > - No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
- (.andromeda/architecture.md:187) The task set `start` and the `mcp` role boot grows past the four lean tasks, and the command line's one-line answer and four exits must hold over it.
  > `escher-session <verb> [arguments] --session <dir>`, the driver's command line over the four lean stand tasks, with its `serve` host role and its `mcp` role, answering one line of JSON and exiting `0` accepted · `1` refused · `2` usage · `3` session error.
- (.andromeda/architecture.md:193) `scripts/cold-agent.sh`, whose task set and goals the chunk edits, is the cold-agent pipe contract stated in full in test-plan §3, and `ci-leg.sh` the one leg runner whose stand sets move with a new or renamed stand check.
  > `.github/scripts/ci-leg.sh {leg}` is the one leg runner of CI and the dev host; `apt-install.sh` the one bounded, retried package install; `coverage_floor.py` the per-crate coverage floor; `scripts/agent-run.sh` and `scripts/cold-agent.sh` the agent-run test and cold-agent pipe contracts, in full in test-plan §3.

## Bindings
- **Headless stand ↔ Occupied Resources**: a held load on the stand that adds a provider, a listener, a port, an env var or an outbound host is a registration, never a silent add — the stand's offline boot and no-`blitz_net` rule (.andromeda/architecture-detail/standard-contracts.md:120) against the one listener and the outbound hosts (.andromeda/architecture.md:201) (.andromeda/architecture.md:203) and the environment values (.andromeda/architecture.md:216).
- **Test harness settle ↔ Provider traits ↔ Driver session**: `LOAD_WAIT` waits on what `settle` reads as `Busy::Loads`, which an offline harness reads only through `has_pending_critical_resources()`, which a no-op `NetProvider` never registers — the stand's held load must satisfy all three (.andromeda/architecture-detail/standard-contracts.md:108) (.andromeda/architecture-detail/standard-contracts.md:84) (.andromeda/architecture.md:171).
- **Select contracts ↔ Established Decisions**: `select_value`, the crate-private `node::select`, `is_disabled` and the `input`-then-`change` dispatch carry the select model decision that binds the flight booker's select and `type` on it — (.andromeda/architecture-detail/standard-contracts.md:17) (.andromeda/architecture-detail/standard-contracts.md:76) (.andromeda/architecture-detail/standard-contracts.md:36).
- **Headless stand ↔ Conventions**: the stand's new and changed Dioxus tasks follow the apps-and-examples conventions — `#[component]` components, `#[derive(Store)]` state, CSS in `const` raw strings (.andromeda/architecture-detail/standard-contracts.md:121) (.andromeda/architecture.md:125).
- **Harness time ↔ Document clock ↔ Driver session**: the driver's `advance` moves the harness's `VirtualClock`, which every stand document reads through `DocumentConfig.clock` — (.andromeda/architecture-detail/standard-contracts.md:114) (.andromeda/architecture-detail/standard-contracts.md:16).
- **Telemetry bootstrap ↔ CI contracts**: the cold-agent pipe stated in test-plan §3 counts wrong calls from the closed-span lines the sink prints — (.andromeda/architecture-detail/standard-contracts.md:128) (.andromeda/architecture.md:193).
