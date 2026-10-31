# Task — arch · 2026-10-31-engine-debts

## The chunk
Takes six recorded engine debts to the founder as forks, each then fixed and proven on the stand or a fixture, pinned as a stated limit, or kept as a residual: the transformed scrolling box in `visible_region`, the missing `inert` focus fixup, native checkedness in the accessibility tree, the select model's limits, the select-option `overlap` in `lint`, and the served pages' missing keyboard and contrast rows.
A fix to the transformed scrolling box is proven on a pan/zoom container, a CSS `transform` on a container holding absolutely positioned nodes, beside the existing translated fixture; each engine fix is an engine change for every document, ratified by the founder's word at its fork.
Adds Tab-order, keyboard-activation and contrast rows for Cells and the load page, including its `load-fetch` Button, as the ground of the route's later a11y re-read of SC 2.1.1, 2.4.3 and 1.4.3; a row that reads red becomes a further fork.
Adds a check of its own for three behaviours the upstream take brought (`:open` on `details` and `dialog`, the windowed shell's redraw route, `pseudo_element_flow_classification`), and carries one stale name, `DocumentMutator::remove_attribute`, to the wrap's amendments.
Re-asserts the standing witnesses `stand_keyboard`, `stand_contrast`, `inert_subtree`, `select_interaction` and `incremental_oracle`; no new verb, node key or refusal cause, no dependency bump, nothing sent upstream.

## Reached files
- packages/blitz-dom/src/scrolling.rs · edits `visible_region`
- packages/blitz-dom/src/node/node.rs
- packages/blitz-dom/src/accessibility.rs · edits the `toggled` write of a native checkbox or radio
- packages/blitz-dom/src/events/
- packages/escher-driver/src/lint.rs · edits `overlaps`
- packages/escher-driver/src/schema.rs · edits the `off-screen` and `covered` schema text
- docs/capabilities.md · edits the stated limits of `off-screen` and `covered`
- tests/blitz-tests/tests/transformed_client_rect.rs · edits the pin `a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`
- tests/blitz-tests/tests/inert_subtree.rs · edits the pin `removing_inert_restores_focus_hit_and_tree`
- tests/blitz-tests/tests/stand_lint.rs · edits the pinned reading `[0, 1, 0, 0]`
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · edits its run, to `for incremental in [false, true]`
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads the standing witness of SC 2.1.1
- tests/blitz-tests/tests/stand_contrast.rs · re-reads the standing witness of SC 1.4.3
- tests/blitz-tests/tests/select_interaction.rs · re-reads the standing witness of the select model
- tests/blitz-tests/tests/incremental_oracle.rs · re-reads incremental and non-incremental layout staying identical
- .andromeda/residuals.md · edits lines 31, 32, 33, 34, 39 and 40

## Needs
- `visible_region` and its neighbours in the scrolling contract: its measured limit under a transform, its read-only standing, seeing never read from a hit; a fix stays consistent with `get_client_bounding_rect`, `Node::absolute_position`, `scroll_into_view`, a box's client rect not subtracting its own scroll offset, and a hit stopping at the padding box of a clipped box.
- CSS transforms over a container of absolutely positioned nodes, translate and scale: how transforms are applied, hoisted positions and stacking-context bounds memoised by geometry generation, the culling of non-finite transformed boxes, the limits on 3D and `offset` transforms; this is what the pan/zoom fixture exercises.
- The refusal causes `off-screen` and `covered`: how a driver call reaches them through `visible_region` and the hit, their meaning in the schema text, and the capability statement's pin, which a fix moves.
- The upstream sync record: the merge base, upstream `main` last read at `ebacd68bd`, upstream `0980cce5f` not taken and owned by Epoch 3's sync, `scrolling.rs` and `layout/writing_mode.rs` untouched by the take, and escher's scrolled-box bounds and hit, which that commit would collide with.
- `inert`: honoured by presence on the element or an ancestor, not focusable, not hit, out of the accessibility tree and the snapshot; the stated limit that focus fixup is not built, and the residual line that owns it.
- What a focus fixup must act on: a focus change dispatched as `blur`, `focusout`, `focus`, `focusin`, event-producing functions taking a `dispatch_event` callback, a redraw requested on a focus set or cleared, `:focus-within` held on ancestors, interaction state cleared or retargeted, and SC 2.4.3.
- The mutation path an `inert` attribute change takes: `DocumentMutator`, its deferred flush on `Drop`, the Stylo snapshot before an attribute mutation, and the changed set's four kinds of write site, which a new write must stay among.
- `build_accessibility_tree`'s state writes, each attribute read once, `aria-pressed` and native checkedness as the one `toggled` property, and the measured limit that the tree states no native checkbox's or radio's checked state, with the residual line that owns it.
- `toggled` crossing to the platform adapter as a closed value, and the platform-tree refresh on a non-empty changed set; the snapshot's `checked` and `Reading`'s `pressed`, written from the same node property, whose readings a fix must be measured against.
- The select model: `apply_select_choice` as the one path, a choice of the chosen option firing nothing, a choice held as its options' `CHECKED` state settled by HTML's selectedness algorithm, `BaseDocument::select_value`, the crate-private `node::select`, keys, typeahead and click, and the stated limits on `multiple`, `optgroup`, the unpainted popup and the unclicked option.
- The `change` event: the engine dispatching it only after `input` on a select's choice and on no checkbox, radio or text input; default actions queued after handlers; a select's `input` and `change` reaching the app as form data; a controlled app writing `selected` from its `input` value; the Dioxus bridge skipping event kinds with zero handlers.
- What a painted popup or a modelled `multiple` select would reach: `paint_scene`, paint's one-module-per-concern split, layer limits, how a select lays out, and the hit over an option, which today refuses a driver `click` as `covered`.
- The snapshot's `ListBoxOption` bounds, each option given its select's client rect, a select's options as its children, `Session::lint` reading the screen and changing nothing, the `overlap` finding between `flight-type-one-way` and `flight-type-return`, and the dependence of an option's bounds on a painted popup.
- The served stand pages the rows run on: `boot_cells`, `boot_load`, `LoadNet`, `LOAD_ANSWER_DELAY`, `LOAD_WAIT`, the `load-fetch` Button, the six served tasks' one private table, the pinned 800x600 Light viewport and its one bundled font.
- The harness surfaces the rows call: `text_contrast` for a WCAG reading, the input helpers that pump and never settle, `settle` reporting a load in flight, and `tick` and `advance_time` as the only movers of the shared `VirtualClock`.
- Keyboard activation and focus order: Enter and a released Space dispatching a queued Click, `ElementData::is_disabled` as the one disabled reading gating them, the engine's focusability rules, and the stand colours a red contrast row would move.
- `:open` on `details` and `dialog`: the `open` attribute read by presence and its matching, which no test asserts today; a check that reads it wrong surfaces as a fork, never a re-pin.
- The windowed shell's redraw route through its event loop: every window event followed by a Poll, the waker's Poll events, redraws requested through the shell provider and skipped while resources are pending or the window occluded; read today only by the boot smoke and `focus_redraw`.
- `pseudo_element_flow_classification`: pseudo-element flow in layout, which the test asserts in neither layout mode; it is upstream's file, so running it in both modes is an edit to an upstream test, for the additivity note to record, and `tests/all.rs` is upstream's held target.
- Both layout modes: every pipeline check runs `for incremental in [false, true]`, the one incremental pipeline and `incremental_oracle` holding the two identical, over any engine fix.
- The driver's bounds a fix must stay inside or bring as a fork: `NODE_FIELDS` thirteen, `VERBS` twelve, eight closed causes, the `'static` schema, answers written by hand in the `json` module, no `NodeId` crossing, the committed kept checks `stand_replay` replays.
- The `ElementData::classes` row and its sidecar entry, which name `DocumentMutator::remove_attribute` where `mutator.rs` holds `clear_attribute`; a wrap amendment listed in the plan's expected amendments, never a phase edit.
- The record of direct engine changes for every document, each flagged upstreamable with no upstream pull request open: a fix joins it with the founder's ratification, its merge surface goes to Epoch 3's sync CARRY, and upstream-owned `scrolling.rs`, `node/node.rs`, `accessibility.rs` and `events/` keep escher's edits unsent, under the standing hold.
- The dependency pins stand unmoved: this chunk bumps no dependency, so any fix works on the Stylo, Taffy, Parley and Dioxus versions already pinned.

## Standing conventions
- S1 · Each new integration test file covers one behavior, opens with a `//!` doc naming it, is one test target, and first asserts its fixture produces the condition · Conventions (Tests); .andromeda/architecture-detail/conventions.md
- S2 · Each new scenario runs in both layout modes · Conventions (Tests); .andromeda/architecture-detail/conventions.md
- S3 · A shared fixture page, boot or hold lives once in `tests/blitz-tests/tests/fixtures/mod.rs`, stand helpers in modules that are no test target, and a page with one reader stays in its check's file · Conventions (Tests); .andromeda/architecture-detail/conventions.md
- S4 · Unit tests sit in `#[cfg(test)] mod tests` at the end of the source file or in a `#[path]` child file of their own · Conventions (Tests); .andromeda/architecture-detail/conventions.md
- S5 · A new HTML fixture follows the fixture layout: numbered cases under headings with a CSS comment per case and `./`-relative sibling assets · Conventions (HTML fixtures); .andromeda/architecture-detail/conventions.md
- S6 · New code carries `//!` and `///` docs and cites the spec URL above or next to the implementation · Conventions (Documentation); Design Philosophy
- S7 · New code passes `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and rustdoc `-D warnings` through `.github/scripts/ci-leg.sh`, with `clippy::expect_used` and `clippy::unwrap_used` next to a justification · Conventions (Formatting and lints); .andromeda/architecture-detail/conventions.md
- S8 · A new check holds the `stand-repeat` leg's zero-failure budget and the per-crate coverage floors · Inherited Defaults; .andromeda/architecture-detail/inherited-defaults.md
- S9 · New engine code reads time only through `DocumentConfig.clock`, with no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs` · Standard Contracts (Document core); .andromeda/architecture-detail/standard-contracts.md
- S10 · New engine code keeps persistent interaction state on DOM nodes only, takes `NodeId` in every public API, resolves ids through `get_node`, and mutates through `doc.mutate()` · Design Philosophy; Standard Contracts (Document core, Mutation)
- S11 · A new engine log line gates tracing per call site with a `cfg(not(feature = "tracing"))` fallback · Conventions (Feature gating); .andromeda/architecture-detail/conventions.md
- S12 · New crate internals stay `pub(crate)` or `pub(super)`, and a known gap left in code is marked `TODO` or `FIXME` · Conventions (Visibility and naming, Known gaps); .andromeda/architecture-detail/conventions.md

## Not needed
- The 0.4.0 capabilities: the canvas renderer, the CPU renderer's custom-paint gap, `drag`, and virtual-node production are out of this chunk, so custom widgets and the wgpu texture example are not reached.
- `accesskit_xplat`'s adapter API and platform implementations, and the windowed AT witness: stated out of this version, and no fix here edits that crate.
- Cells' selected-cell state: its own residual line, out of this chunk.
- The browser app and rdme: history store, URL suggester, about pages, favicon cache, polling and document loader.
- The WPT runner, its outputs, its workflow and the PR-body script.
- blitz-net's provider, its cache, per-host cap and User-Agent, and the outbound hosts; the load page's fetch is answered by the in-process stand-in.
- The JavaScript runtime's bootstrap, module loader, timer thread and `document_may_load`.
- Nix packaging, cross-compilation, the browser bundles and the publish workflow.
- Build profiles, release versioning, the licensing exceptions and the database and migration defaults.
- `@font-face` registration, resource-loading caps, iframe nesting, inline SVG serialization and text-selection endpoints.
- escher-telemetry's sink, its field sets and its panic hook: no fix edits that crate.
- The launch paths, hot reload, the Android and wasm entry points and the Dioxus `MemoryHistory` navigation.
- The window's zoom and debug keyboard shortcuts and the clipboard and file dialog services.
- The google and graphite fixture forms and the Preact pages.
