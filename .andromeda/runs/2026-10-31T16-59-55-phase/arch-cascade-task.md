# Task — arch · 2026-10-31-select-popup

## The chunk
Builds a painted list for a closed single `select`, opened in every document and renderer as a new engine overlay in paint, hit testing and keys, capability `v030-17`.
A person or an agent chooses an option from the opened list by pointer and by key, the choice going through the one write path `apply_select_choice`; an option of the list can be acted on through the driver; the list closes without a choice.
Restates what the open list ends: the lint's F9 `overlaps` exclusion, the `covered` refusal of a closed option and its schema text, a closed select's options reading their select's client rect in the snapshot; paints or reads a group's label and `size`.
Brings the focus and key contract, reduced motion and the engine's new default colours to the founder as P4 forks; widens the Playbook Boundary so this entry's wrap halts once as his answer card.
Adds `escher-0.4.0-incubator/` to `.gitignore` before the wrap's P1, and re-reads the free-disk step of `Stand checks repeated` as a watch.

## Reached files
- packages/blitz-dom/src/node/select.rs
- packages/blitz-dom/src/events/keyboard.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/assets/default.css
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-paint/src/lib.rs
- packages/blitz-paint/src/render.rs
- packages/dioxus-native-dom/src/snapshot.rs · edits `snapshot`
- packages/escher-driver/src/execute.rs · edits `execute`
- packages/escher-driver/src/lint.rs · edits `lint`'s `overlaps` exclusion
- packages/escher-driver/src/schema.rs · edits the schema help, the `covered` refusal's text
- packages/escher-driver/src/schema/spec.rs · edits the schema help, the `covered` refusal's text
- examples/seven_guis/src/tasks/flight_booker.rs
- tests/blitz-tests/tests/stand_contrast.rs · edits its stand table, a new row
- tests/blitz-tests/tests/stand_keyboard.rs · edits its stand table, a new row
- tests/blitz-tests/tests/stand_act_select.rs · edits its stand table, a new row
- tests/blitz-tests/tests/stand_lint.rs · edits its stand table, a new row
- .gitignore · edits the incubator lines
- .github/workflows/ci.yml · re-reads the free-disk step in `Stand checks repeated`

## Needs
- The select model as a whole: this chunk ends or restates its standing statements that no popup is painted, that Enter and Space open nothing on a single select, that a closed single select's option takes no click and stays refused `covered`, and that `size` is unread.
- `apply_select_choice` stays the one path for every choice, the choice's writers crate-private, `node::select` holding the rules with no public type; a single select's choice of its already chosen option fires nothing.
- `input` then `change` on a select's choice, as the engine dispatches them, and what a list closed without a choice dispatches; a controlled app writes `selected` from its `input` event's value or the choice reverts, and the bridge's form data for a select's events.
- The public read-only select readers the driver and the bridge read: `select_value`, `select_values`, `select_of`, `closed_select_of`, `is_multiple_select`, `clear_select_typeahead`; what `closed_select_of` answers once a select can be open, and any new reader staying read-only.
- `option_disabled` and `is_disabled` as the one disabled readings, for a list's row and for a group's options: the pointer and key gates must read them.
- A `multiple` select's rows, keys and clicks stay as 2026-10-31-engine-debts built them, its rows and groups reading their own client rects, its `size` read or not by this chunk's lean or fork.
- A group's label and `optgroup` options: named in the tree and unpainted today, and pseudo-element `content: attr()` stands unbuilt and non-string pseudo-element `content` unsupported, which bears on how a label gets painted.
- The overlay in paint: `paint_scene`, every renderer that paints with blitz-paint, the harness's `screenshot` and `screenshot_rect`, and the layer cap; painting assumes styles and layout already resolved, and the overlay lives in the engine.
- Hit testing under the overlay: a hit stops at the padding box of a box paint clips, skips a node dropped since the last resolve and answers nothing under a non-invertible transform, and `hit_inner` is a recorded textual conflict with upstream's `0980cce5f`.
- `visible_region`, `get_client_bounding_rect` and `scroll_into_view` as the driver's readings of an option in an open list: seeing is never read from a hit, and these readers mutate nothing.
- Keys: Enter, a released Space, arrows and typeahead on a single select; Enter and Space dispatch a Click the driver queues so the app's handlers run before the default action; default actions queue in order and a handler can cancel them. Whether a closed select's arrows keep moving the choice is a P4 fork.
- Focus with a list open: focus order under SC 2.1.1 and 2.4.3, the focus event order, the focus fixup under `inert` and its reading of neither `disabled` nor `hidden`, and `inert` keeping a node out of hit, tree and snapshot.
- Where the open state lives: persistent interaction state references DOM nodes only, never layout-generated ones; over versioned `NodeId`s; on a removed node it is cleared or retargeted before the node is freed.
- Invalidation and redraw when a list opens or closes: mutations through `doc.mutate()`, damage propagation, redraw requested after a change, and the changed set, written only at its four kinds of site, which is what makes the shell refresh the platform tree.
- The accessibility tree says whether the select is open: `build_accessibility_tree`'s state attributes each read once, `expanded` read from `aria-expanded` by upstream's builder, a native control's state writes, and the option and group names.
- Colours: a new engine default, since the engine holds no tokens, in the user-agent sheet embedded at compile time; a config's `ua_stylesheets` is used as given, `DioxusDocument` gets the sheet once through `DocumentConfig::default()`, and headless documents read `ColorScheme::Light`; SC 1.4.3 and the harness's `text_contrast` read the result.
- Any time the list reads, reduced motion and typeahead included, goes through `DocumentConfig.clock`: no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs`, and the harness's `screenshot`, `text_contrast` and `settle` never move time.
- The snapshot: a closed select's options read `bounds` equal to its client rect today and an open list's read real boxes; its text form and diff read the screen read-only and hold no `NodeId`; its options stay the select's children.
- The driver's acting on an option: `Session::run` validates first, `execute` reads the option's reachability and the `covered` refusal; no refusal holds what a call supplied, no `NodeId` crosses, and answers are hand-written in the `json` module.
- `Session::lint` reads the screen and changes nothing; the F9 `overlaps` exclusion today covers a closed single select's options, those under an `optgroup` included, and is restated for an open list's own boxes. Record: `.andromeda/runs/2026-10-31T03-24-40-phase/p5-card-1.md` rows 1-2.
- Stable ids and actionability of an open list's options: `element_id`, `element_ids` and `unkeyed_actionable`, an agent-actable element reading an author key; the list's rows being the options themselves keeps this whole.
- The chunk adds no verb, node key, cause, session error, exit status, carrier, listener, port, env var or workspace crate, so the counts of twelve verbs and eight causes, the `v3` wire and the exit codes hold; anything a fork would need is named on that fork.
- `dioxus-native`'s windowed path shows the same list through the engine's overlay, with no separate OS window assumed; the windowed path builds its documents `Parallel` and the shell's redraw and poll loop carry it.
- The stand: flight booker's `select#flight-type` is the stand's popup, the four stand tables move with any new row, and the two committed kept checks that `stand_replay` replays hold what the snapshot change leaves of them.
- This is a direct engine change for every document: the plan records such changes flagged upstreamable with no upstream pull request open and their merge surface owned by Epoch 3's sync. The Playbook Boundary widening holds a validated surface admitting new input and pixels.

## Standing conventions
- S1 · Unit tests in a `#[cfg(test)] mod tests` at the end of the source file or a `#[path]` child file; an integration test file covers one behaviour and opens with a `//!` doc naming it; blitz-tests keeps one test target per file · .andromeda/architecture-detail/conventions.md
- S2 · Scenarios run in both layout modes, and a test first asserts its fixture produces the condition · .andromeda/architecture-detail/conventions.md
- S3 · Stand checks share helpers through modules under `tests/` that are no test target; `tests/blitz-tests/tests/fixtures/mod.rs` states the boot, hold and shared pages once, and a page with one reader stays in its check's file · .andromeda/architecture-detail/conventions.md
- S4 · Modules and files open with `//!`, public items carry `///`, spec URLs are cited beside implementations, and a constant's doc states its rationale · .andromeda/architecture-detail/conventions.md
- S5 · Every change passes `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and rustdoc warnings as errors, through `.github/scripts/ci-leg.sh`; `expect_used` and `unwrap_used` allowed only beside a justification · Conventions, Formatting and lints
- S6 · Crate internals `pub(crate)` or `pub(super)`, paint rendering split one module per concern, known gaps marked `TODO` or `FIXME` · .andromeda/architecture-detail/conventions.md
- S7 · In engine crates tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback, and optional functionality sits behind cargo features, `accessibility` gating the snapshot modules · .andromeda/architecture-detail/conventions.md
- S8 · A new log line carries no snapshot, snapshot text or diff, and escher-driver logs no id, name or value; engine targets print only safe fields · .andromeda/architecture-detail/cross-cutting-patterns.md
- S9 · New code and tests stand under the per-crate coverage floor and the `stand-repeat` leg's zero-failure budget · Inherited Defaults, Code quality

## Not needed
- The browser app's history database, URL suggester, about pages and document loader: the list reaches the browser through the engine alone.
- The WPT runner's execution, budgets, outputs and PR diff script: the chunk changes none of them, and WPT runs only on upstream's repository.
- Boa's runtime, script timers, the module loader and `document_may_load`: the chunk adds no script surface.
- blitz-net, blitz-html, resource loading and the provider traits' fetch paths, the stand's `LoadNet` included: the list loads nothing.
- Nix packaging, browser bundles, signed builds, cross-compilation and the named build profiles: no packaging moves.
- Dependency pinning and the waiting Stylo, Taffy and Dioxus bumps: upstream sync work is the next epoch's.
- The session socket's and state directory's permissions, idle expiry, and the recording directory's medium: the chunk leaves session lifetime and storage as they stand.
- Non-adjacent `multiple` sets through the driver and Ctrl in `press`: out of scope, a residual on the founder's word.
- HTML table layout, inline SVG re-parsing and the google and graphite fixture forms: no select list runs through them.
- The cold-agent pipe, its model provider and the agent-run scripts: this chunk's change does not touch them.
- Android activities, the wasm canvas entry, the macOS handler and the Windows icon resource: the overlay lives in the engine, not in a platform entry point.
- The `accesskit_xplat` platform adapters: the chunk writes through the tree's builder and changes no adapter.
