# arch extract

## Relevance
relevant — an engine-wide change to a locked select model across blitz-dom, blitz-paint, dioxus-native-dom and escher-driver, whose limits and refusal readings the plan names this chunk to restate.

## Constraints
- Placement: arch §Project Intent → Crate purposes assigns the DOM to blitz-dom and painting through anyrender to blitz-paint, and arch §Infrastructure Patterns → Directory structure fixes the member crates; the list's state, keys and hit belong in the engine crates, its pixels in blitz-paint, with no new workspace crate. Whether the code's existing seams fit an overlay is research's question.
- One write path: arch §Established Decisions → Scrolling and input requires every choice that changes the chosen option to go through `apply_select_choice`, and a choice of the chosen option to fire nothing; a choice from the opened list, by pointer, key or driver, takes that path and adds no writer.
- Ending limits this chunk owns: arch §Established Decisions → Scrolling and input states "no popup is painted", Enter and Space open nothing on a single select, a closed single select's option takes no click and stays refused `covered`, and `size` is unread, with the route entry «Select popup» the named owner of the first; each reading this chunk changes is restated at wrap in that section, and each it keeps stays stated as a limit. The `multiple` select's keys and pointer in the same section stay as built unless a fork moves them.
- Event order: arch §Standard Contracts → Events requires `input` then `change` on a select's choice, a queued Click so app handlers run before the default action, and a focus change as `blur`, `focusout`, `focus`, `focusin`; opening or closing the list without a choice dispatches no choice event, and any new dispatch is named on a fork.
- Driver surface: arch §Standard Contracts → Driver session requires every call to run through `Session::run` after `validate`, its schema stated once as `'static` data with twelve verbs and eight closed causes, no `NodeId` crossing and no refusal holding what a call supplied; the chunk's scope adds no verb or cause, so `covered`'s changed reading is restated in that one schema text. Any new item is named on a fork.
- Occupied resources: arch §Occupied Resources → Environment variables, §Occupied Resources → Network ports and listeners and §Occupied Resources → Names bound the chunk to no new env var, listener, port or binary; arch §Established Decisions → Driver session makes a new carrier or exit a boundary widening.
- Clock and identity: arch §Standard Contracts → Document core requires every blitz-dom time read to go through `DocumentConfig.clock`, with no `Instant::now()` or `SystemTime::now()` outside `clock.rs`, so any list timing, reduced motion's included, reads that clock; arch §Design Philosophy → One incremental pipeline requires interaction state to reference DOM nodes only, by `NodeId`, never layout-generated nodes, so the open state and any list rows held across resolves obey it.

## Patterns to follow
- The select's rules in the crate-private `node::select`, with no public type, writers crate-private and public readers read-only, per arch §Standard Contracts → Nodes and widgets and arch §Standard Contracts → Document core.
- The driver reads the select through element-naming public tests, `closed_select_of` and `is_multiple_select`, never the role, per arch §Standard Contracts → Dioxus DOM bridge; a test for the open state follows the same shape.
- Seeing is read through the read-only `visible_region`, never from a hit, and a hit stops at the padding box of a box paint clips, per arch §Standard Contracts → Scrolling, selection, tree; the list's boxes, escaping or not their ancestors' clips, are held to these readers, and how is research's question.
- A direct engine change for every document is flagged upstreamable with its merge surface named by file, per arch §Established Decisions → Scrolling and input; the owner of that line is Epoch 3's sync.
- Integration tests: one behaviour per file opening with a `//!` doc, both layout modes, the fixture's condition asserted first, per arch §Conventions → Tests; rationale for new constants and spec URLs next to the code, per arch §Conventions → Documentation.

## Anti-patterns to avoid
- A second choice writer or a shell-side list: a separate OS window or a list held in blitz-shell or dioxus-native would split the one engine model that arch §Design Philosophy → Radically modular requires any renderer to paint; the same list reaches every document and renderer.
- Direct DOM state writes around the mutator where one is offered: arch §Conventions → DOM API patterns treats `DocumentMutator::doc` as an escape hatch, with a request to prefer extending the mutator.
- Logging or returning snapshot or option content: arch §Standard Contracts → Dioxus DOM bridge bars a log or event from carrying the snapshot, its text or a diff, except a diff's three counts, and arch §Standard Contracts → Driver session has nothing log an id, a name or a value.

## Contract bindings
- arch ↔ tests: the harness's `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move the clock, per arch §Standard Contracts → Test harness; the stand tables the scope names (`stand_contrast`, `stand_keyboard`, `stand_act_select`, `stand_lint`) and the two committed kept checks of arch §Occupied Resources → Filesystem may read a changed select.
- arch ↔ a11y: `build_accessibility_tree` names option and group labels and writes state attributes each read once, per arch §Standard Contracts → Mutation, query and CSSOM; `inert` keeps a node out of the tree and the snapshot and unhit, per arch §Established Decisions → DOM semantics, a rule an opened list also falls under. Whether the builder reports an open select is research's question.
- arch ↔ security: a remote page reaches the select model too, per arch §Established Decisions → Scrolling and input; a validated surface admitting new input and pixels is the Boundary widening the scope halts wrap on, and option text in a frame falls under whatever security rules bind that surface.
- arch ↔ design: colours arrive as a new engine default on a founder fork; arch §Standard Contracts → Paint and shell keeps the painting entry `paint_scene`, and whether `assets/default.css` or paint holds them is that fork's to name.

## Acceptance criteria contributions
- Every choice from the opened list, by pointer, key and driver, passes through `apply_select_choice`, firing `input` then `change` once, and a choice of the chosen option fires nothing (per arch §Established Decisions → Scrolling and input).
- The list shows and acts identically through every renderer that paints with blitz-paint and in headless and windowed documents, with no select logic in blitz-shell or dioxus-native (per arch §Standard Contracts → Paint and shell).
- The driver's verb list stays twelve and its causes eight, with `covered`'s text and the `overlaps` reading restated once in the schema and lint sources, unless a fork named otherwise (per arch §Standard Contracts → Driver session).
- No new env var, listener, port or workspace crate lands, and no `Instant::now()` or `SystemTime::now()` is added in blitz-dom outside `clock.rs` (per arch §Occupied Resources → Environment variables).
