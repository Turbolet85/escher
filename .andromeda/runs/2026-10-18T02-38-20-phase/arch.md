# arch extract

## Relevance
relevant — every one of the nine debts and the CARRY lands in a contract or locked decision this plan records (blitz-dom readers, the accessibility builder, the Dioxus bridge and snapshot, the driver's refusal schema), and no new crate, resource or surface is in scope.

## Constraints
- A fix that changes an engine reading for every document belongs to the class of direct engine changes this plan records one by one. Each entry names the founder's word, flags the change upstreamable against a named upstream ref, notes that no upstream pull request is open, and lists its merge surface by file. A debt answered «fixed» is recorded the same way, against the current merge base (per architecture §Established Decisions → [Scrolling and input]; the base itself per architecture §Project Intent, «Upstream sync» `23b9b26a9`).
- Incremental and non-incremental layout run one pipeline and must produce identical layouts. Any change to hit, position, client-rect, focusability or caret code must keep the two modes equal (per architecture §Design Philosophy → One incremental pipeline, safe identities).
- The accessibility builder reads each attribute once, and the three state attributes by a closed-set match on the trimmed lower-case value, with no role consulted. `aria-checked` is unread, and nothing else in the builder writes `toggled`. A role-aware `aria-checked` read (debt 2) and an `aria-activedescendant` read (debt 3) must keep that closed shape. Whether the builder can tell checked from pressed without a second `toggled` writer is research's question (per architecture §Standard Contracts → Mutation, query and CSSOM).
- The geometry readers are four separate contracts:
  - the hit walk stops at an `overflow` clip and not at a `contain: paint` box whose `overflow` is `visible`;
  - `visible_region` reads clipping boxes where the bounds reader places them;
  - `get_client_bounding_rect` reads a transformed box at its layout position;
  - `Node::absolute_position` reads a self-scrolled box less its own offset, and the harness's `layout_rect`, `layout_rect_of` and `center_of` are built on it.

  A fix to one reader (debts 4, 5, 8) must state which of the others it moves (per architecture §Standard Contracts → Scrolling, selection, tree).
- The snapshot's `bounds` is `get_client_bounding_rect`. Its `pressed` is the accessibility node's `toggled()` and is never parsed a second time from the element. Its `enabled` is the `disabled`-presence reading on a `can_be_disabled()` element. So a change to the client rect (debt 8), to `toggled` (debt 2) or to `disabled` (debt 7) moves the snapshot, its diff and its text (per architecture §Standard Contracts → Dioxus DOM bridge).
- The driver checks a `click` or `type` in a fixed order: `disabled`, then `off-screen` (the centre of `bounds` against `visible_region`), then `covered` (the raw hit never reaches the target). The `Cause` set is closed at nine, each cause with fixed `meaning()` and `remedy()` texts. `covered`'s meaning states the `contain: paint` and one-axis limits. A pinned debt (5, 9) changes wording inside that closed set, and a fixed debt removes its limit sentence (per architecture §Standard Contracts → Driver session).
- `disabled` is read by presence on any element for focusability, the `:disabled` state, the click target and the activation keys. The Tab walk, autofocus and `DioxusDocument::unkeyed_actionable` move with it. Narrowing it to form controls (debt 7) moves all of these. `inert` is recorded as not implemented (debt 1), and the bridge clears a framework-written `inert: false` through upstream's `is_bool_attr`, which leaves `mutation_writer.rs` byte-identical to upstream (per architecture §Established Decisions → [DOM semantics]; [Unsupported features]; the bridge per §Standard Contracts → Dioxus DOM bridge).

## Patterns to follow
- **State a measured limit where an agent reads the tool.** A driver reading the chunk keeps as a limit is written into the cause's `meaning()` in `packages/escher-driver/src/refusal.rs` and the crate docs in `packages/escher-driver/src/lib.rs`. It is then served unchanged in every refusal's JSON line and in `describe()`'s causes on the command line's and the MCP server's `help` (per architecture §Standard Contracts → Driver session).
- **Write a pin as the reading stands, then rewrite it to the fixed reading.** The pinned debts are held by:
  - `scrolled_box_absolute_position`
  - `hit_clipped_at_scrolling_box` (its `contain: paint` and transformed-and-scrolled cases)
  - `password_caret_click`
  - `stand_act_ids::refused_at_boot`
  - `stand_keyboard::the_engine_honours_no_inert` and `a_disabled_attribute_of_any_value_is_skipped`
  - `accessibility_states`' fourth test

  A new check follows the test conventions: one behaviour per file with a `//!` doc, the fixture's condition asserted first, `for incremental in [false, true]`, and stand tables in `tests/blitz-tests/tests/common/mod.rs` or `tests/blitz-tests/tests/session_common/mod.rs` (per architecture §Conventions → Tests).
- **Engine arms stay small and route changes through one path.** Keep the public handler signatures and put arm bodies in private named functions, as `click_button` and `press_range_input_key` do. A focus move goes through `generate_focus_events` / `set_focus_to`, and value changes through one write · mark · event path (per architecture §Standard Contracts → Events).
- **Changed-set marks follow the existing sites.** A focus or checked change marks through `snapshot_node_and`. Node creation, hover and layout mark nothing. Any new focusability rule (debts 1, 7) must leave the changed set and the shell's refresh-then-drain reading true (per architecture §Cross-cutting Patterns → Invalidation and state integrity).
- **DOM writes go through the mutator.** Writes go through `DocumentMutator` (`doc.mutate()`), and the mutator is extended rather than reached through `DocumentMutator::doc` (per architecture §Conventions → DOM API patterns).

## Anti-patterns to avoid
- **No second reader or method beside an engine one.** `scroll_into_view` was widened in place, with no second method added. A position, hit or rect fix goes into the shared reader, not a parallel one, and the snapshot never re-parses a state the builder already wrote (per architecture §Established Decisions → [Scrolling and input]; the single read per §Standard Contracts → Dioxus DOM bridge).
- **No indexing by a `NodeId` that may be stale.** Ids that may be stale (an `aria-activedescendant` target, a changed-set entry) are read through `get` / `contains_key` (per architecture §Standard Contracts → Scrolling, selection, tree, `NodeTree`; §Standard Contracts → Document core).
- **No new workspace crate, env var, listener or port.** None may be added without an §Occupied Resources registration, and none is in this chunk's scope (per architecture §Occupied Resources).

## Contract bindings
- **arch ↔ a11y.** Focusability changes (debts 1 and 7) move the Tab sequence. `stand_keyboard`'s tables (`common::tab_order`, `common::ENABLED_TAB_ORDER`) move with them, because the Tab walk reads the focusability flag (architecture §Established Decisions → [DOM semantics]). Debts 2 and 3 widen what the adapter receives.
- **arch ↔ security.** A new attribute read keeps the closed shape, and the adapter crossing carries its content classes (names, states, descriptions). `aria-activedescendant` and a role-aware `aria-checked` must stay inside the states class, with no text carried out (architecture §Standard Contracts → Mutation, query and CSSOM).
- **arch ↔ tests/harness.** The harness's `layout_rect`, `layout_rect_of` and `center_of` sit on `Node::absolute_position` (architecture §Standard Contracts → Test harness), so a debt-4 fix moves harness readings that existing checks depend on. Which checks depend on them is research's question.
- **arch ↔ driver schema/MCP.** A reworded `covered` meaning (debts 5 and 9) reaches `describe()`, the MCP tool listing's help and every `covered` refusal line (architecture §Standard Contracts → Driver session).
- **snapshot ↔ builder/readers.** A debt-8 fix moves the snapshot's `bounds`, so it moves the driver's click point and its `off-screen` check. A debt-2 `toggled` write moves the snapshot's `pressed` (architecture §Standard Contracts → Dioxus DOM bridge).

## Acceptance criteria contributions
- (arch) Every debt answered «fixed» is proven by a check run in both layout modes, and its plan entry names its merge surface by file against the merge base `23b9b26a9`, with no upstream pull request sent (per architecture §Established Decisions → [Scrolling and input]; base per §Project Intent).
- (arch) The CARRY check asserts the tree `build_accessibility_tree` returns for `aria-expanded`, `aria-disabled` and `aria-level`, including the default levels by tag and role. It also asserts that `aria-disabled` moves no snapshot `enabled` reading (per architecture §Standard Contracts → Mutation, query and CSSOM; the snapshot half per §Standard Contracts → Dioxus DOM bridge).
- (arch) The `Cause` set stays the nine of `CAUSES`. Any reworded `meaning()` reads identically in a refusal's JSON line and in `describe()`'s causes (per architecture §Standard Contracts → Driver session).
- (arch) The chunk's diff adds no workspace crate, manifest dependency, env var read, listener or port (per architecture §Occupied Resources).
