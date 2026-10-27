# arch · cascade read · group 6 of 12

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:9) The resolution of the mutator.rs conflict and upstream's new mutator and construction paths (`:open`, the no-computed-style hit, pseudo-elements voting in flow) must keep the changed set's contract exactly: written by tracked writes, drained by `take_changed_nodes`, and marked by no node creation.
  > the changed set — `BaseDocument::has_changes(&self) -> bool` reads true while the changed set (`changed_nodes`, a `pub(crate)` `HashSet<NodeId>`) is non-empty, from the first tracked write until the next drain, and `BaseDocument::take_changed_nodes(&mut self) -> HashSet<NodeId>` returns the set and leaves it empty; a returned id may name a node dropped since it was written, so it is read through `get_node`, never by indexing; node creation marks nothing — `create_node` no longer inserts, so the constructor's root node, the mutator's `create_element` / `create_text_node` and layout's anonymous blocks and pseudo-element nodes leave the set empty; what does mark it is stated under §Cross-cutting Patterns → Invalidation and state integrity;
  (.andromeda/architecture.md:133)
  > the changed set read by `has_changes` and drained by `take_changed_nodes`, its ids read through `get_node`, never by indexing
- (.andromeda/architecture-detail/standard-contracts.md:8) document.rs and dioxus_document.rs are merge surfaces (upstream adds 36 lines to the bridge's document): the `Document` trait, `DioxusDocument`'s `accessibility_tree` override carrying stable ids and the `DocumentConfig` field set must stand after the take.
  > `Document` trait — `inner`, `inner_mut`, default `handle_ui_event` via `EventDriver` with `NoopEventHandler`, default `poll` returning `false`, `id`, and — under the `accessibility` feature — a default `accessibility_tree() -> accesskit::TreeUpdate` returning `self.inner().build_accessibility_tree()`, which a wrapper may override to enrich nodes (`DioxusDocument` alone does, with stable ids)
  (.andromeda/architecture-detail/standard-contracts.md:8)
  > `DocumentConfig` fields: viewport, base_url, ua_stylesheets, net/navigation/shell/html-parser providers, font_ctx, media_type, style_threading, incremental, abort_signal, subdocument_depth, clock
  (.andromeda/architecture.md:177)
  > `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
  (.andromeda/architecture.md:173)
  > `ScriptFetcher` and the `ScriptDocument` API run a document's scripts on Boa, JS listeners before Blitz default actions
- (.andromeda/architecture-detail/standard-contracts.md:16) The chunk re-reads the clock invariant on the take's tree, and upstream's rewritten scrolling code (`0980cce5f`) must read time only through the document's clock; the harness's shared `VirtualClock` reaches the documents through that same `DocumentConfig.clock`, so the time the stand checks re-read stays the harness's.
  > a document holds the config's clock, else a fresh `WallClock` — wall time, as before — and reads it for the overlay scrollbar fade, the double-click window, pan and the fling's generation, the smooth-scroll start and the fling and smooth frames
  (.andromeda/architecture-detail/standard-contracts.md:16)
  > no `Instant::now()` or `SystemTime::now()` stands in blitz-dom outside `clock.rs`
  (.andromeda/architecture.md:135)
  > The document's clock: `DocumentConfig.clock`, the config's clock or else a fresh `WallClock`;
  (.andromeda/architecture-detail/standard-contracts.md:114)
  > the harness holds a `VirtualClock` its documents share — `HarnessOptions::into_config(self, &VirtualClock)` puts a clone in `DocumentConfig.clock`
- (.andromeda/architecture-detail/standard-contracts.md:17) The select model is a residual-36 merge surface the chunk re-reads and keeps unchanged through the document.rs and mutator.rs moves: one public read-only reader, crate-private writers.
  > `BaseDocument::select_value(&self, select: NodeId) -> Option<String>`, public and read-only: HTML's `select.value` — the chosen option's value, `None` with none chosen or for a node that is no select; dioxus-native-dom's snapshot `value` and `apply_select_choice` its callers
  (.andromeda/architecture-detail/standard-contracts.md:17)
  > The choice's writers are crate-private — `settle_select_choice`, `set_select_choice` and `set_option_chosen`, the `CHECKED` write through `snapshot_node_and` for an in-document option — beside the private field `select_typeahead: Option<select::Typeahead>`, `None` at construction
  (.andromeda/architecture-detail/standard-contracts.md:76)
  > The crate-private module `node::select` holds the select's rules — `OptionReading`, `SelectStep` (`Next` · `Previous` · `First` · `Last`), `Typeahead` and `TYPEAHEAD_TIMEOUT` (1 s), and the readers `is_single_select`, `is_select_option`, `options`, `is_chosen`, `chosen_option`, `readings`, `option_label`, `option_value`, `settled`, `stepped` and `matched` — no public type
  (.andromeda/architecture.md:151)
  > the select's rules live in the crate-private `node::select`, with no public type.
- (.andromeda/architecture-detail/standard-contracts.md:23) mutator.rs is a conflict surface: every DOM mutation, upstream's new paths included, must still go through the mutator's methods.
  > `doc.mutate()` methods `create_element`, `create_text_node`, `set_attribute`, `clear_attribute`, `append_children`, `insert_nodes_before`, `remove_node`, `remove_and_drop_node`, `set_inner_html`, `set_node_text`, `set_custom_widget`, `set_style_property`
  (.andromeda/architecture.md:137)
  > mutations through `doc.mutate()`; selector queries returning `Result`, never matching the scope node;
- (.andromeda/architecture-detail/standard-contracts.md:24) The accessibility tree's names and closed-set state reads are re-proven on the take by the a11y leg and the `accessibility_*` checks, through upstream's IME, `:open` and hit-test commits.
  > `aria-labelledby`: after the `<label>` → input loop it clears the node's `labelled_by`, pushes the referenced nodes that have built nodes (an id naming no element ignored, `referenced_dom_ids_to_node_ids`) and sets their text, gathered through `write_text_alternative` · `write_flat_text_content` · `FlatText`, as the computed `label` (`set_label`), so it outranks a `<label>` and an `aria-label` (accname 2B over 2C/2D), the `aria-label` the fallback when the targets hold no text
  (.andromeda/architecture-detail/standard-contracts.md:26)
  > each attribute is read once, and nothing else in the file writes `toggled`; the added lines hold no log site and no panic path — a value of 5,500 characters builds the tree — so what `build_accessibility_tree` returns changes for every document whose markup states one of the four (packages/blitz-dom/src/accessibility.rs:169-213; packages/blitz-dom/src/accessibility.rs:347; packages/blitz-dom/src/accessibility.rs:395-415; tests/blitz-tests/tests/accessibility_states.rs:134-316 — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md)
  (.andromeda/architecture.md:137)
  > `build_accessibility_tree` returns an AccessKit `TreeUpdate`, naming nodes from text children, `aria-labelledby`, `aria-label`, a bound `<label>` and option labels, and writing state attributes each read once
- (.andromeda/architecture-detail/standard-contracts.md:27) blitz-shell's window.rs is a merge surface of upstream's redraw-through-the-event-loop commit (#1269): `View::poll` must still drain the changed set on every build and refresh the platform tree only when it took a non-empty one.
  > on a poll that reported work `View::poll` takes the document's changed set (`take_changed_nodes`, outside the `accessibility` cfg, so a shell built without the feature still drains the set and drops it) and, under `accessibility`, calls `update_tree` when the set it took was non-empty
  (.andromeda/architecture-detail/standard-contracts.md:28)
  > so it is held by the document-level tests and both feature builds of blitz-shell (packages/blitz-shell/src/accessibility.rs:44-46; packages/blitz-shell/src/window.rs:376-382; packages/blitz-shell/src/window.rs:521-524)
  (.andromeda/architecture.md:137)
  > blitz-shell refreshes the platform tree when a poll took a non-empty changed set.
- (.andromeda/architecture-detail/standard-contracts.md:34) events/pointer.rs, events/driver.rs and blitz-traits' events.rs are merge surfaces: dispatch order, the range arm's one write path, keyboard activation's queued Click and the engine's `change` must hold through the take.
  > pointer events produce mouse compatibility events for mouse input and touch events for finger and pen (packages/blitz-dom/src/events/driver.rs:327-349); non-bubbling events reach only the target (packages/blitz-dom/src/events/driver.rs:378-383); focus change dispatches `blur`, `focusout`, `focus`, `focusin`
  (.andromeda/architecture-detail/standard-contracts.md:34)
  > a range input's default actions dispatch `input` — `DomEventData::Input(BlitzInputEvent { value })`, the formatted current value — through the crate-internal `BaseDocument::apply_range_move`, the one write · mark · `Input` · redraw path of its key arms and its pointerdown arm, one per change and none for a move that changes nothing
  (.andromeda/architecture-detail/standard-contracts.md:35)
  > both through `dispatch_activation_click`, whose data is `Node::synthetic_click_event_data(modifiers)` and which the event driver queues so the app's handlers (a Dioxus `onclick`) run before the default action `handle_click`
  (.andromeda/architecture-detail/standard-contracts.md:35)
  > pointer/wheel target the hovered node, keyboard/IME the focused node, falling back to the root
  (.andromeda/architecture-detail/standard-contracts.md:36)
  > `EventState` cancelled / propagation_stopped / redraw_requested with `merge`
  (.andromeda/architecture-detail/standard-contracts.md:36)
  > `DomEventKind::Change`, after `Input`, and `DomEventData::Change(BlitzInputEvent)` — named `change`, not cancelable, bubbling, with no UI event mapped and no default action — are dispatched by the engine only after `input` on a select's choice, a checkbox, radio or text input firing no engine `change`
  (.andromeda/architecture.md:141)
  > Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action; the engine dispatches `change` only after `input` on a select's choice.
- (.andromeda/architecture-detail/standard-contracts.md:43) scrolling.rs is a conflict surface rewritten by upstream's reversed scroll origins (`0980cce5f`): `scroll_into_view` must still write every scrolling box on the chain then the viewport, and `visible_region` stay the read-only reader of what can be seen.
  > `scroll_into_view` scrolls every scrolling box on its target's containing-block chain, innermost first, and then the viewport — the CSSOM View "scroll an element into view" steps — its signature unchanged and for every caller
  (.andromeda/architecture-detail/standard-contracts.md:43)
  > a nested box is written at once through `scroll_to(.., ScrollBehavior::Instant)` whatever behaviour was asked — the document holds one scroll animation at a time
  (.andromeda/architecture-detail/standard-contracts.md:44)
  > `visible_region(&self, node_id) -> Option<BoundingRect>` is the read-only reader beside it
  (.andromeda/architecture-detail/standard-contracts.md:44)
  > whether an element can be seen is read from that geometry and never from a hit
  (.andromeda/architecture.md:143)
  > `scroll_into_view` scrolls every scrolling box on the target's chain, nested ones instantly with no `scroll` event, then the viewport; `visible_region` is the read-only reader of what can be seen, and seeing is never read from a hit;
- (.andromeda/architecture-detail/standard-contracts.md:45) node/node.rs is a conflict surface of `0980cce5f` and `ebacd68bd`: the hit stop at a box paint clips — by `overflow` or `contain: paint`, whose getter residual line 35 ties to Stylo 0.23 — must stand unchanged through the take.
  > either axis of its computed `overflow` not `visible`, or a computed `contain` that includes `paint` on a box paint applies it to (not an inline flow box, not an internal table box other than a cell), the root element excepted, the two axes not told apart: paint's own predicate, the `contain: paint` half ratified by the founder (2026-10-18) — stops the walk at its padding box
  (.andromeda/architecture-detail/standard-contracts.md:46)
  > a box both transformed and scrolled is hit where its rows show and not through its scrolled-out content
  (.andromeda/architecture.md:143)
  > a hit stops at the padding box of a box paint clips and skips a node dropped since the last resolve;
- (.andromeda/architecture-detail/standard-contracts.md:55) Upstream's hit test of an element with no computed styles yet (`ebacd68bd`) lands in `hit_inner`, whose dropped-node skip must survive the resolution in every loop.
  > The hit walk skips a node dropped since the last resolve: `Node::hit_inner` reads each child id through `self.tree().get(id)` and passes over one that no longer resolves, in its positive-z hoisted, paint-children and negative-z hoisted loops and its text-cluster lookup, and the paint tree's walk does the same; `hit_inner` keeps its signature, and its callers `hit` and `hit_with_scrollbar` are unchanged
- (.andromeda/architecture-detail/standard-contracts.md:46) `absolute_position` lives in node/node.rs beside upstream's rewritten scroll reading, and must still subtract only its containing blocks' scroll offsets, its callers (IME area among them) reading a scrolled box where it stands.
  > and `Node::absolute_position`, the second position reader, adds the node's own location and hands the sum to the private `position_in_containing_block`, which subtracts its containing block's scroll offset and recurses, so only ancestors' scroll offsets are subtracted — a box at (40, 50) reads (40, 50) at every scroll offset of its own, in both layout modes
  (.andromeda/architecture-detail/standard-contracts.md:47)
  > `handle_dom_event`, `Node::focus`'s IME area, `synthetic_click_event_data`, `scroll_into_view` and the test harness's `layout_rect`, `layout_rect_of` and `center_of`, which read such a box where it stands (ratified by the founder, 2026-10-18)
  (.andromeda/architecture.md:143)
  > `Node::absolute_position` subtracts only ancestors' scroll offsets;
- (.andromeda/architecture-detail/standard-contracts.md:53) The transformed client rect reads offsets from both bodies of `physical_unrounded_geometry`, one of them in the writing_mode.rs conflict, and the scope records whether the take moves its measured limit for the next entry.
  > on a containing-block chain that carries a transform (the private `is_transformed_chain`) the private `transformed_bounds` maps the border box's corners through the node's own transform and each containing block's, in device pixels as `hit_inner` and the paint tree map them, the offsets taken from `physical_unrounded_geometry` differences so both of its bodies are served, and reads the axis-aligned bounding box
  (.andromeda/architecture-detail/standard-contracts.md:53)
  > A measured limit: `visible_region` places a scrolling box under a transform where it stands untransformed, so a target inside a translated scrolling box reads outside its visible region (tests/blitz-tests/tests/transformed_client_rect.rs:183-220), stated in `off-screen`'s meaning (§Driver session), its fix owned by a `.andromeda/residuals.md` line (target: next).
  (.andromeda/architecture.md:145)
  > `get_client_bounding_rect` reads an element where its transforms move it, while `visible_region` reads a scrolling box under a transform where it stands untransformed (a measured limit);
- (.andromeda/architecture-detail/standard-contracts.md:61) The construct.rs conflict and any Taffy move must leave `LayoutPassState` the one implementer of Taffy's traits and the construction tasks crate-internal.
  > the crate-private `LayoutPassState<'doc>`, which wraps the document for one layout pass and derefs to `BaseDocument`, implements Taffy's `TraversePartialTree`, `TraverseTree`, `LayoutPartialTree`, `LayoutContainingBlock`, `CacheTree`, `LayoutBlockContainer`, `LayoutFlexboxContainer`, `LayoutGridContainer` and `RoundTree` — `BaseDocument` implements none of them
  (.andromeda/architecture-detail/standard-contracts.md:61)
  > crate-internal `ConstructionTask` types (packages/blitz-dom/src/layout/construct.rs:35-53)
  (.andromeda/architecture.md:147)
  > the crate-private `LayoutPassState` implements Taffy's traversal and layout traits for one pass — `BaseDocument` implements none;
- (.andromeda/architecture-detail/standard-contracts.md:77) `out_of_flow`'s recording and its one reader sit in node/node.rs (conflict surface) and the layout pass, and must survive the resolution as they read.
  > `Node::out_of_flow: AtomicBool`, public: written by the style traversal from the node's new primary style through the crate-private `Node::record_out_of_flow`, beside the crate-private `style_is_out_of_flow` — `position` `absolute` or `fixed`, and not `display: none`; the crate-private `LayoutPassState::is_out_of_flow` alone reads the recorded flag, while `Node::is_out_of_flow` keeps its signature and its style reading, now through `style_is_out_of_flow`, as do its other callers
- (.andromeda/architecture-detail/standard-contracts.md:68) node/element.rs (`:open`, `53e9d81c6`) and node/text.rs (winit beta.4, `a09f46792`) are merge surfaces: the special-element variants, `RangeInput` among them, and the text input's generated events must stand.
  > `SpecialElementData` `SubDocument`, `CustomWidget`, `Stylesheet`, `Image`, `Canvas`, `TableRoot`, `TextInput`, `CheckboxInput`, `RangeInput`, `FileInput`, `None` (packages/blitz-dom/src/node/element.rs:359-387) — `RangeInput` since 2026-10-13-range-input-interaction, a source break for an external exhaustive match, `SpecialElementData` not being `#[non_exhaustive]`
  (.andromeda/architecture-detail/standard-contracts.md:68)
  > `GeneratedTextInputEvent` `Input`, `Select`, `PreEditChange`, `Submit` (packages/blitz-dom/src/node/text.rs:56-62)
  (.andromeda/architecture.md:149)
  > `NodeData` and `SpecialElementData` (not `#[non_exhaustive]`, `RangeInput` among its variants); `RangeInputData` read through the `ElementData` range accessors;
- (.andromeda/architecture-detail/standard-contracts.md:75) `is_disabled` lives in node/element.rs (merge surface) and must stay the one disabled reading the conflicts' resolutions use.
  > `ElementData::is_disabled(&self) -> bool`, public: `can_be_disabled()` — `button` · `input` · `select` · `textarea` — and a `disabled` attribute present; the one reading focusability, the pointer gates and the activation keys share
  (.andromeda/architecture.md:151)
  > `ElementData::is_disabled` is the one disabled reading — a form control with a `disabled` attribute present;
- (.andromeda/architecture-detail/standard-contracts.md:78) The `class` arm of mutator.rs (conflict surface) and Stylo's readers in stylo.rs (merge surface of `53e9d81c6`) must keep `classes` in sync through the take.
  > `ElementData::classes: ThinVec<Atom>`, public: the `class` attribute's atoms, filled by `ElementData::new` and `Clone` through the crate-private `parse_classes`, kept in sync by `DocumentMutator::set_attribute`'s `class` arm and by `remove_attribute`, and read by Stylo's `has_class` and `each_class`; `ElementData` is named outside blitz-dom by type only, never by a struct literal
- (.andromeda/architecture-detail/standard-contracts.md:84) blitz-traits' shell.rs is a merge surface of winit's clipboard move (`527394724`) and the IME commit: `ShellProvider`'s defaults, clipboard and `set_ime_enabled` among them, must stay no-ops for a headless document.
  > `ShellProvider` methods default to no-ops, clipboard to `Err(ClipboardError)`, file dialog to an empty list (packages/blitz-traits/src/shell.rs:11-63), including `set_ime_enabled`
  (.andromeda/architecture.md:153)
  > `ShellProvider` with no-op defaults
- (.andromeda/architecture-detail/standard-contracts.md:96) blitz-shell's lib.rs and window.rs are merge surfaces of #1269, #1271 and #1273: its re-exports of winit's types, its event set and its provider's clipboard move with whatever winit pin the take carries.
  > blitz-shell re-exports `BlitzApplication`, `BlitzShellEvent`, `BlitzShellProxy`, `View`, `WindowConfig`, `DataUriNetProvider` and winit's `ControlFlow`, `EventLoop`, `EventLoopProxy`, `Window`
  (.andromeda/architecture-detail/standard-contracts.md:96)
  > `BlitzShellEvent` Poll, ResumeReady, RequestRedraw, CloseWindow, Accessibility, Embedder, Navigate, NavigationLoad, ResizeSettleCheck
  (.andromeda/architecture-detail/standard-contracts.md:96)
  > `BlitzShellProvider` implements `ShellProvider` for redraw, cursor, title, IME, close, minimize, maximize, decorations, drag, clipboard, file dialog
  (.andromeda/architecture.md:157)
  > blitz-shell's application, proxy, view, window config and shell provider, the embedder completing a resume;
- (.andromeda/architecture-detail/standard-contracts.md:97) The password caret path lives in events/pointer.rs (merge surface of `0980cce5f`) and lays out through document.rs's `layout_as_text_input`; both must keep a password's real character widths out of the caret.
  > a click inside a field holding a value places the caret by the bullets — blitz-dom's pointer path lays them out through `layout_as_text_input`, scrolled as paint scrolls them, and moves the real editor to the character under the point
  (.andromeda/architecture-detail/standard-contracts.md:10)
  > `BaseDocument::layout_as_text_input(&self, node_id, text) -> Option<parley::Layout<TextBrush>>` — `text` laid out with a text input's editor styles, scale and direction in the document's own fonts, one unbroken line, a fresh `LayoutContext` per call, `None` for a node that is no text input
  (.andromeda/architecture.md:157)
  > a password `input` paints its value masked in every document and through every renderer that paints with blitz-paint, its caret placed by the bullets, so no width of the real characters reaches a frame or the caret.
- (.andromeda/architecture-detail/standard-contracts.md:103) Screenshot pixels and contrast rows that a moved Taffy, Stylo or winit shifts are re-measured, and each reads the harness's white composite as an input of its value.
  > `Harness::screenshot(&mut self) -> Result<Raster, ScreenshotError>` and `Harness::screenshot_rect(&mut self, PixelRect) -> Result<Raster, ScreenshotError>` resolve at the harness's own time (no tick, no pump), paint the viewport at its scale and device size through blitz-paint into `VelloCpuImageRenderer`, then composite the frame over opaque white (premultiplied source-over: each channel `+ (255 - alpha)`, alpha 255)
  (.andromeda/architecture-detail/standard-contracts.md:104)
  > the background composites source-over each element's `background-color` from that element up through its ancestors to the first opaque one, else onto the canvas (the root element's background, else `body`'s when the root's is transparent) over opaque white
  (.andromeda/architecture.md:159)
  > `screenshot` and `screenshot_rect` paint a raster through blitz-paint, printing, panicking and logging nothing; `text_contrast` reads a WCAG contrast and changes nothing;
- (.andromeda/architecture-detail/standard-contracts.md:120) Every stand number the chunk re-measures after the take — bounds, snapshot byte sizes, pixels, contrast — is read at the stand's pinned viewport, scale, colour scheme and one bundled font, which stay as pinned.
  > pinned `VIEWPORT_WIDTH = 800`, `VIEWPORT_HEIGHT = 600`, `HIDPI_SCALE = 1.0`, `COLOR_SCHEME = ColorScheme::Light`; `font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`, system fonts off
  (.andromeda/architecture-detail/standard-contracts.md:127)
  > The stand's typography is DejaVu Sans, its one bundled font, stated in this contract and not on design-system's product surface: `font_ctx()` registers it for every generic family with system fonts off, on native; every stand boot holds it through `HarnessOptions.font_ctx`, `boot_cells` and `boot_load` included
  (.andromeda/architecture.md:163)
  > in a pinned 800x600 Light viewport with one bundled font
- (.andromeda/architecture-detail/standard-contracts.md:134) The Cells screen's reading over `SNAPSHOT_TEXT_BUDGET` is a snapshot byte size the chunk re-measures where the take moves layout, and must still read in parts.
  > `boot_cells(HarnessOptions) -> Harness<DioxusDocument>` mounts `Task::Cells` in `TaskShell` through the private `cells_root`, Cells staying no `LeanTask`: its screen passes the snapshot text's budget and reads in parts

## Bindings
- **standard-contracts (changed set) ↔ cross-cutting-patterns (Invalidation and state integrity)**: the changed set's drain and node-creation rule (.andromeda/architecture-detail/standard-contracts.md:9) bind with where it is written — four kinds of site and no node creation, hover, scroll, resize, load, animation or layout — and the shell's rebuild on a non-empty set (.andromeda/architecture.md:275), so the mutator.rs and window.rs resolutions keep both halves.
- **standard-contracts (shell and provider) ↔ cross-cutting-patterns (Service wiring)**: `BlitzShellProvider`'s redraw and clipboard and `ShellProvider`'s defaults (.andromeda/architecture-detail/standard-contracts.md:96; .andromeda/architecture-detail/standard-contracts.md:84) bind with redraws requested after mutations and skipped while critical resources are pending (.andromeda/architecture.md:271), over upstream's redraw-through-event-loop and clipboard commits.
- **standard-contracts (layout) ↔ cross-cutting-patterns (Rendering details)**: `LayoutPassState` and the construction tasks (.andromeda/architecture-detail/standard-contracts.md:61) bind with `display: contents` hoisting in construct.rs (.andromeda/architecture.md:277) over the construct.rs conflict.
- **standard-contracts (clock, hit stops, scrolling, select) ↔ project-intent (Upstream sync)**: the contracts the conflicts must keep (.andromeda/architecture-detail/standard-contracts.md:16; .andromeda/architecture-detail/standard-contracts.md:45; .andromeda/architecture-detail/standard-contracts.md:43; .andromeda/architecture-detail/standard-contracts.md:17) bind with the sync line the wrap moves to the new pin where the take moves it (.andromeda/architecture.md:287).
- **standard-contracts (stand pins, harness composite) ↔ the stand's re-measured numbers**: a moved bound, byte size, pixel or contrast reading is re-measured at the pinned viewport and font (.andromeda/architecture-detail/standard-contracts.md:120) and over the harness's white composite (.andromeda/architecture-detail/standard-contracts.md:103), never loosened.
