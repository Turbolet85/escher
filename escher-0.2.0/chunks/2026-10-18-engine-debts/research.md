# Codebase Research — 2026-10-18-engine-debts

## Scope
- **Depth:** deep · **Reads:** 22 · **Globs/Greps:** 19
- **Harness rules consulted:** none — no live leg in this chunk (every proof is a headless blitz-tests check; `.claude/rules/testing.md` and `.claude/rules/a11y.md` auto-loaded and read whole)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (Setup 5a read HEAD green)
- **External inputs:** `inputs#I1` — the operator's directive: the nine founder forks in one file, answered by the operator in his stead, each answer recorded as a delegate's, PROVISIONAL

## Files inspected
- `packages/blitz-dom/src/node/element.rs` (439-488, 690-735) — `flush_is_focussable` (:702) reads `has_attr(disabled)` on any element and is the only focusability computation; `can_be_disabled` (:486) is `button` · `input` · `select` · `textarea`; the `DISABLED`/`ENABLED` state is already gated on `can_be_disabled` (:456-462)
- `packages/blitz-dom/src/node/node.rs` (725-740, 1366-1520, 1675-1691, 1925-1930) — `focus` sets the IME area from `absolute_position` (:731); `hit_inner` (:1374) stops at the padding box only when `clips_by_overflow` (:1476-1495), an `overflow` test alone; `absolute_position` (:1683) subtracts the node's OWN `scroll_offset` before recursing — the debt-4 mechanism, at :1684-1685
- `packages/blitz-dom/src/accessibility.rs` (100-265) — one read per attribute; `toggled` written only from `aria-pressed` (:171-178); no `aria-checked`, no `aria-activedescendant`, no `inert` read; tree focus is `focus_node_id` (:114); the merged `aria-expanded` (:191) · `aria-disabled` (:197) · `aria-level` (:204-215, tag and role defaults) reads
- `packages/blitz-dom/src/document.rs` (1052-1080, 1735-1750, 2352-2390) — `layout_as_text_input` (:1067) lays a text out as the field's editor would, its one caller the paint mask; `focus_next_node` / `focus_prev_node` filter on `is_focussable` (:1740, :1748); `physical_unrounded_geometry` (:2352) sums `location − ancestors' scroll` with no transform; `get_client_bounding_rect` (:2373) is that sum less the viewport scroll
- `packages/blitz-dom/src/events/pointer.rs` (325-335, 455-470, 520-560) — `ClickTarget::Disabled` for ANY element carrying `disabled` (:462); a text input's pointerdown places the caret by `driver.move_to_point` on the editor's own layout of the real text (:545) — the debt-6 mechanism, password or not
- `packages/blitz-dom/src/scrolling.rs` (705-735) — `scroll_into_view` calls `absolute_position(scroll_offset)` (:719-720), adding the target's own offset back
- `packages/blitz-paint/src/render.rs` (327-335) — paint already clips `contain: paint` (and `strict` / `content`) to the padding box, so a debt-5 hit stop would agree with paint
- `packages/blitz-paint/src/render.rs` (992) — the mask's `layout_as_text_input(node, bullets)`
- `packages/dioxus-native-dom/src/snapshot.rs` (135-160, 196-230) — `bounds` is `get_client_bounding_rect` (:139); `enabled` reads `disabled` only on a `can_be_disabled()` element (:154-156); `pressed` is `toggled() == True` (:160); no node is dropped for `hidden`
- `packages/dioxus-native-dom/src/mutation_writer.rs` (445-470) — `inert` in upstream's `is_bool_attr` (:458): a falsy `inert` is removed
- `packages/escher-driver/src/execute.rs` (330-367) — `action_point`: `disabled` (snapshot `enabled == Some(false)`), then `in_view`, then `covered` at the centre of `bounds`
- `packages/escher-driver/src/refusal.rs` (15-90, 215-275) — `covered`'s meaning carries the two limit sentences (`contain: paint`, one-axis clip); the cause table and its in-test copy
- `examples/seven_guis/src/app.rs` (170-180, 320-340) — `#task-header-spacer` is the header's last child with `flex: 1` and no content: 0 high
- `examples/seven_guis/src/tasks/crud.rs` (55-105) — `crud-list` `role="listbox"` `aria-label="People"` `tabindex="0"`, rows `role="option"` `aria-selected`, no `aria-activedescendant`
- `tests/blitz-tests/tests/password_caret_click.rs` (1-30) — the pin: index 5 (`iiiiiiii`) and 2 (`WWWWWWWW`) at 30 px
- `escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-run.md` (55, 91-93) — `spacer_calls` 0
- `escher-0.2.0-incubator/deferred-from-0.1.0.md` (11-75) — the nine as the overseer listed them, with the founder's earlier labels

## Graph impact (from the code-graph query)
- **absolute_position** — 6 engine/harness callers: `handle_dom_event` @ packages/blitz-dom/src/events/mod.rs:108, `focus` (IME area) @ packages/blitz-dom/src/node/node.rs:731, `synthetic_click_event_data` @ packages/blitz-dom/src/node/node.rs:1928, `scroll_into_view` @ packages/blitz-dom/src/scrolling.rs:719, `layout_rect_of` @ packages/blitz-test-harness/src/inspect.rs:59, `write_node` @ packages/blitz-test-harness/src/inspect.rs:141; test callers in `anonymous_block_restyle.rs`, `stale_dirty_descendants.rs`, `scrolled_box_absolute_position.rs` — a fix moves every one, and `scroll_into_view`'s add-back must go with it
- **flush_is_focussable** — 3 callers: `set_attribute` @ packages/blitz-dom/src/mutator.rs:344, `clear_attribute` @ packages/blitz-dom/src/mutator.rs:482, `ElementData::new` @ packages/blitz-dom/src/node/element.rs:439 — the hooks list `tabindex` · `href` · `disabled`; an inherited `inert` needs a subtree recompute these hooks do not make
- **is_focussable** — `focus_next_node` / `focus_prev_node` @ packages/blitz-dom/src/document.rs:1740/1748, `process_added_subtree` (autofocus) @ packages/blitz-dom/src/mutator.rs:1041, `unkeyed_actionable` @ packages/dioxus-native-dom/src/actionable.rs:69, plus `focusability_updates`, `stand_actionable_keys`, `stand_snapshot_state` and a snapshot unit test
- **get_client_bounding_rect** — `node_client_rects` @ packages/blitz-dom/src/document.rs:2462, `adjust_element_coords` @ packages/blitz-dom/src/events/driver.rs:366, `get_bounding_client_rect` (script) @ packages/blitz-vibey-script/src/dom/element.rs:1415, `get_client_rect` @ packages/dioxus-native-dom/src/events.rs:222, the snapshot `visit` @ packages/dioxus-native-dom/src/snapshot.rs:139, and nine test sites
- **physical_unrounded_geometry** — `get_client_bounding_rect`, `offset_rect`, `inline_fragment_rects` (packages/blitz-dom/src/document.rs:2380/2440/2482), `resolved_style_value` (packages/blitz-dom/src/resolved_style.rs:508/520), `visible_region` (packages/blitz-dom/src/scrolling.rs:773) — a transform belongs in `get_client_bounding_rect` alone: CSSOM's `offsetTop` ignores transforms, and `visible_region` would then read the untransformed box beside transformed bounds

## Patterns detected
- **The closed-shape state read** (packages/blitz-dom/src/accessibility.rs:163-189): trim, exact match on a closed set, write an AccessKit enum or bool, nothing else.
- **A fixed limit leaves its sentence** (packages/escher-driver/src/refusal.rs:15-25, :78-82): each measured engine limit is one sentence of `covered`'s meaning, served by `describe()`; a fix removes the sentence and rewrites the pin.
- **The pin as measured** (tests/blitz-tests/tests/password_caret_click.rs:1-9): a `//!` doc stating the reading, a fix "turns this check red by design, and it is restated then".
- **Arms as private named functions** (packages/blitz-dom/src/events/pointer.rs, `click_button`, `press_range_input`): a fix in the pointer path keeps the public handler signature.

## Conventions to follow
- **Both layout modes** in every check (`for incremental in [false, true]`), per tests/blitz-tests/tests/password_caret_click.rs and `.claude/rules/testing.md` §Determinism.
- **Both builds** for a geometry check: `cargo test -p blitz-tests --locked` and `cargo test --workspace --locked` compile `physical_unrounded_geometry`'s two bodies (test-plan §9).
- **One file per behaviour** under `tests/blitz-tests/tests/`, auto-discovered; a new file moves the recorded 127 files / 126 targets.

## New files to create
- `tests/blitz-tests/tests/accessibility_merged_states.rs` — the CARRY's check of `aria-expanded` · `aria-disabled` · `aria-level` on the built tree
- `tests/blitz-tests/tests/inert_subtree.rs` — F1 A: an inert subtree takes no focus, no hit and no tree node, on a fixture
- `tests/blitz-tests/tests/accessibility_active_descendant_unread.rs` — F3 B: the pin of the unread `aria-activedescendant`
- `tests/blitz-tests/tests/transformed_client_rect.rs` — F8 A: the client rect under a transform and the `visible_region` limit, on a fixture
- `tests/blitz-tests/tests/stand_act_transformed.rs` — F8 A: a driver `click` on a translated control accepted where it shows

## Files to modify
- `packages/blitz-dom/src/node/node.rs` — F1 A (inert in `is_focussable` and the hit), F4 A (`absolute_position`), F5 A (the clip stop)
- `packages/blitz-dom/src/node/element.rs` — F7 A: `flush_is_focussable` reads `disabled` on `can_be_disabled()` elements only
- `packages/blitz-dom/src/scrolling.rs` — F4 A: the add-back in `scroll_into_view` dropped
- `packages/blitz-dom/src/accessibility.rs` — F1 A (an inert subtree left out), F2 A (`aria-checked` role-aware, the `switch` role mapping)
- `packages/blitz-dom/src/document.rs` — F8 A: `get_client_bounding_rect` under transforms
- `packages/blitz-dom/src/events/pointer.rs` — F6 A (the password caret), F7 A (the three `disabled` gates)
- `packages/blitz-dom/src/events/keyboard.rs` — F7 A: the two activation gates
- `packages/dioxus-native-dom/src/snapshot.rs` — F2 A: `checked` / `pressed`, and the CARRY's `aria-disabled` unit test
- `packages/escher-driver/src/schema.rs` — F2 A: the `checked` and `pressed` node-key help
- `packages/escher-driver/src/refusal.rs` — F5 A and F9 A: `covered`'s meaning and its unit table; F8 A: `off-screen`'s meaning if step 8 measures the limit
- `packages/escher-driver/src/json.rs` — F5 A and F9 A: the unit test that reads `covered`'s limits in a refusal line
- `packages/escher-driver/src/lib.rs` — F5 A and F9 A (and F8 A as `refusal.rs`): the crate docs' statement of the limits
- `tests/blitz-tests/tests/stand_keyboard.rs` — F1 A and F7 A: the two pins rewritten
- `tests/blitz-tests/tests/accessibility_states.rs` — F2 A: the fourth test rewritten, the `aria-checked` rows
- `tests/blitz-tests/tests/scrolled_box_absolute_position.rs` — F4 A: the pin rewritten
- `tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs` — F5 A (the `contain: paint` test) and F8 A (the transformed-and-scrolled fixture condition)
- `tests/blitz-tests/tests/password_caret_click.rs` — F6 A: the pin rewritten

## Sweeps
- `inert|contain:` over examples/seven_guis/src: 0 hits — no stand file sets `inert` or `contain`, so debts 1 and 5 are proven on fixtures
- `disabled` over examples/seven_guis/src: 9 hits · all on `button` / `input` controls or `:disabled` CSS — debt 7 moves no stand table
- `aria-activedescendant|aria-checked` over examples/seven_guis/src: 0 hits
- Mechanism re-derivation at HEAD `34bcf71c` of the nine folded claims: each named mechanism found where its line
  says — `inert` unread outside the bridge (`inert` over packages/blitz-dom/src: 1 hit, a doc comment at mutator.rs:166);
  `aria-checked` unread (accessibility.rs, `toggled` from `aria-pressed` alone); no `aria-activedescendant` read; the
  own-offset subtraction (node.rs:1684-1685); the `overflow`-only clip stop (node.rs:1476-1480); the caret by the real
  text (pointer.rs:545); `disabled` by presence on any element (element.rs:703); the client rect with no transform
  (document.rs:2352-2390); `covered` from the hit at the centre of `bounds` (execute.rs:351-367); the four test pins
  present by name (`fn` grep over tests/blitz-tests/tests). Measured readings (−30, −70), (0, 40, 300, 140), 5 / 2
  are the pins' own and are re-read when those checks run at implement

## Open questions
- Each of F1–F9 decides its debt's write set → blocks: implementation-scope (the lists above are every branch's, marked provisional; P4 narrows them to the answered branches)

Resolved here: AccessKit 0.25.1 carries `active_descendant` as a node property (`set_active_descendant`, `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/accesskit-0.25.1/src/lib.rs:2075`), so an F3 fix can set it on the focused list node and leave the tree's `focus` — and with it the snapshot's `focused` — on the list.
