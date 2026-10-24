## 1. A11y Scope Summary

Detail: .andromeda/a11y-plan-detail/a11y-scope-summary.md — read whole.

**A11y tier:** `0 — SC 2.1.1 · 1.4.3 · 2.4.3`

**A11y scope (entities needing assertions):**

- **Dioxus crates** — `accessibility` is a default feature of both Dioxus crates, which the workspace takes with `default-features = false`; it reaches seven_guis through `escher-driver`, which names it unconditionally, while seven_guis' own manifest names none, and no stand binary builds a platform adapter, so no assistive technology is given a tree by the stand binary — the adapter and its windowed AT witness owned by a `.andromeda/residuals.md` line (target: next).

**A11y surfaces & assistive tech reach:** the AccessKit platform adapters and their per-platform gaps are recorded in §2 (Platform adapters).

**A11y assertion harness specification:** accessibility tests cover hidden-element exclusion and HTML-to-AccessKit role mapping (tests/blitz-tests/tests/accessibility_hidden.rs:8-137; tests/blitz-tests/tests/accessibility_roles.rs:1-7); the harness is detailed in §3.

**Observed absent (scope searches):**

> NOT YET MEASURED — must-be-accessible critical paths and a11y triggers: no slice recorded them, and s03 recorded accessibility scope as out of slice.

---

## 2. A11y Strategy

Detail: .andromeda/a11y-plan-detail/a11y-strategy.md — read whole.

**Accessibility tree lifecycle:**

- The accessibility tree is built from the document on InitialTreeRequested and refreshed on change: a poll that takes a non-empty changed set rebuilds the platform tree, both paths through the `Document::accessibility_tree` trait method, so a wrapper's override (DioxusDocument's stable ids) reaches it; proven headlessly on the stand in both layout modes, its windowed witness at the platform adapter owed — owned by a `.andromeda/residuals.md` line (target: next).

- `changed_nodes` is the engine's changed set, the nodes written since the last `take_changed_nodes`: an in-document mutation, a focus or checked change, a text control's input and a select's changed choice mark it, node creation, hover, active, scroll and layout do not, so a pointer move alone reaches no refresh; a harness settle neither reads nor drains it and a fresh harness document's set is non-empty, so a check reading `has_changes()` as "this step changed something" drains first.

**Role derivation (semantic HTML and ARIA):**
- A node's role comes from its `role` attribute, else from the HTML element mapping, else `Role::Unknown` (packages/blitz-dom/src/accessibility.rs:139-149)
- A TODO notes that elements with strong native semantics can currently have their role overridden, contrary to WAI-ARIA 1.2 (packages/blitz-dom/src/accessibility.rs:142-143)

- The builder writes three states from ARIA state attributes, trimmed and matched in lower case: `aria-selected` → `selected`; `aria-pressed` → `toggled` on all but the five checkable roles, `aria-checked` → `toggled` on those alone; `aria-invalid` → `invalid`; a select's option reads its chosen state; nothing else writes `toggled`, so the tree never states a native checkbox or radio checked — that fix and the windowed AT witness owned by `.andromeda/residuals.md` lines (target: next).

- The builder also holds upstream's six reads — `aria-labelledby` (its targets' text the `label`, outranking a `<label>` and an `aria-label`, accname 2B), `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled` and `aria-level` — still one read per attribute; the description and the new closed values leave only to the platform adapter, which no stand binary builds, so what an assistive technology reads of them is not measured.

**Platform adapters (accesskit_xplat):**

**Feature exposure:**

> NOT YET MEASURED — a stated accessibility strategy (POUR coverage depth, agent-runnable invariants, naming conventions): no slice states one, and s01, s02, s03, s06, s07, s08, s10 and s13 recorded accessibility strategy as out of slice.

---

## 3. A11y Assertion Harness Contract

**A11y testing tool pick:** accessibility assertions are Rust tests in blitz-tests over the AccessKit tree.
- Tests call `document.build_accessibility_tree()` after `resolve` and match `tree_update.nodes` with `test_that` matchers on `role()` and `is_hidden()` (tests/blitz-tests/tests/accessibility_hidden.rs:9-26)
- An `assert_role(html, element_id, expected)` helper maps the element's `NodeId` to an AccessKit `NodeId(node_id.as_u64())` and compares roles (tests/blitz-tests/tests/accessibility_roles.rs:39-72)
- An `unknown_tags(html)` helper lists the `html_tag` of nodes with `Role::Unknown` (tests/blitz-tests/tests/accessibility_roles.rs:16-37)
- Since 2026-10-10-snapshot-states a state check reads each carrier's `selected`, `toggled` and `invalid` off the built tree, one stated table per attribute with its row count asserted, in both layout modes; it runs in the workspace test leg and per package, not in the `a11y` leg, which runs six files since 2026-10-14-stand-a11y-assertions (§9) — 5 tests, the fourth reading `aria-checked`'s three values on a checkable role alone and four unchecked natives reading neither attribute, the fifth `aria-pressed` on a checkable role reading nothing and a 5,500-character `aria-checked` nowhere in the tree (tests/blitz-tests/tests/accessibility_states.rs:134-316) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md
- Since 2026-10-17-upstream-sync-engine-debts two upstream files assert the builder's name and description reads off the built tree with `verify_that!` — `accessibility_label`, 13 tests (a `<label for>` and a wrapping `<label>` each naming a `select` and a `textarea`, a `<label>` naming no hidden `input` (tests/blitz-tests/tests/accessibility_label.rs:309-432); `aria-label`; `aria-labelledby` for one and two nodes; an id naming no element ignored; target text as the label; precedence over `aria-label` and content; a self-reference and nested text; a target's `aria-label`; hidden targets read without being referenced; the `aria-label` fallback) and `accessibility_description`, 3 (`aria-description`; `aria-describedby` targets in order; targets outside the tree excluded); they run in the workspace test leg and per package, outside the a11y leg's six files (§9) (tests/blitz-tests/tests/accessibility_label.rs:8-447; tests/blitz-tests/tests/accessibility_description.rs:8-101) — as measured at escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md
- `accessibility_merged_states` (3), `inert_subtree` (4) and `accessibility_active_descendant_unread` (1) assert the builder off the built tree in both layout modes — the merged `aria-expanded` (true and false and nothing else), `aria-disabled` (`true` alone sets disabled) and `aria-level` (its number less one, and the heading defaults), each a stated table with its count asserted; an inert subtree skipped by Tab and Shift+Tab, passed through by a hit, absent from the tree and restored when `inert` is removed; and a focused listbox reading focus and no active descendant — in the workspace test leg and per package, outside the a11y leg's six files (§9) (tests/blitz-tests/tests/accessibility_merged_states.rs:88-157; tests/blitz-tests/tests/inert_subtree.rs:68-194; tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:39-81)
- The test harness builds blitz-dom with the accessibility feature (packages/blitz-test-harness/Cargo.toml:15)
- accesskit_xplat ships a doc-comment usage example, not tests (packages/accesskit_xplat/src/lib.rs:13-86)

**Focus management test harness:**
- The harness exposes focused and hovered node queries (packages/blitz-test-harness/src/inspect.rs:107-115)

**Keyboard test harness:** a Tab and Shift+Tab key-press focus check exists on the stand, driven through the test harness's `press` / `press_with` (tests/blitz-tests/tests/stand_snapshot_state.rs:317-353), and since 2026-10-13-stand-keyboard-harness `stand_keyboard` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks through the same `press` / `press_with`, reading the focused node back per step (tests/blitz-tests/tests/stand_keyboard.rs:124-152) — see §5 (Test harness pattern).

**Contrast verification harness:** built since 2026-10-13-stand-contrast-harness — `Harness::text_contrast` in blitz-test-harness reads an element's text/background pair and font metrics from computed styles as a `TextContrast` (`ratio` by `contrast_ratio`, the WCAG 2.x formula; `required_ratio` 3.0 for large text, else 4.5), and the stand check `stand_contrast` states three tables, read by one shared reader and asserted row for row in both layout modes — the 18 control rows, beside a painted-pixel witness of each measured background, and, since 2026-10-14-stand-a11y-assertions, 14 `TEXT_ROWS` (the CRUD list rows and the text outside the controls) and 19 `STATE_ROWS` (the colour states it reaches, hover and active included); it runs in the workspace `test` leg, per package and, since that chunk, in the `a11y` leg (§9) (packages/blitz-test-harness/src/contrast.rs:56-91; tests/blitz-tests/tests/stand_contrast.rs:49-137; tests/blitz-tests/tests/stand_contrast.rs:168-187; tests/blitz-tests/tests/stand_contrast.rs:577-605; tests/blitz-tests/tests/stand_contrast.rs:607-635; tests/blitz-tests/tests/stand_contrast.rs:637-661) — the record in §6. Since 2026-10-19-visual-lint its first non-test caller is escher-driver's visual lint (`Session::lint`, in process and as the `lint` verb on every carrier), whose `contrast` reading keeps the same exempt rule (§6).

**CI integration:** ci.yml's `a11y` job runs the accessibility integration tests as their own leg — see §9.

**Observed absent (assertion searches):**
- accessibility tree or assertion tooling · searched: `accesskit|AccessKit` over the 32 s03 slice files
- accessibility assertions · searched: `#\[(tokio::)?test\]` over packages/accesskit_xplat/** and `aria-|role` over the 86 s04 slice files (role hits only in github-markdown.css)
- tests of the accessibility tree · searched: `#\[test\]` over packages/blitz-dom/src/accessibility.rs
- accessibility assertions in tests · searched: `accesskit|aria|role` over the 16 listed s08 files (only commented accesskit lines matched)
- accessibility assertions in tests · searched: `aria-|aria[A-Z]|\brole\b` over the 32 s10 slice files
- accessibility assertions in tests · searched: `aria|role|accesskit` over the 21 listed s11 files (matches only in crate docs and unrelated comments)

> NOT YET MEASURED — the structured violation JSON schema / log format, the WCAG criteria mapping of the existing tests and a screen-reader test pattern: no slice recorded them (s01, s02, s06, s07 and s13 recorded the a11y assertion harness as out of slice). The obs log format is now measured for escher's own sink (obs plan §3, §6 Log Coverage, §8): non-JSON text lines on stderr in two record classes — one line per printed event, `{RFC 3339 UTC time} {LEVEL} {target} service.name=… service.version=… {field}={value}…`, and, since 2026-10-07-driver-command-spans, one line per closed span of an admitted target, the same prefix then `span={the span's name}`, the span's recorded fields and the layer's close fields `message`, `time.busy` and `time.idle`, with no line when a span is created, recorded to, entered or exited (the second class was ratified by the founder, 2026-10-09, as obs plan §6 states) — both behind a target allowlist that, among others, prints only `node_id` / `status` / `waiting_nodes` / `property` / `log.*` fields for `blitz*` and `accesskit_xplat` targets, redacts `text`, `value`, `html` and `attrs` for escher's own targets (`escher_*`) and drops a record from any other target whole, at every level — a closed span's every pair judged by the same rule, an outside target's span writing nothing; an a11y violation schema is not yet defined against it.

Contracts: .andromeda/registries/a11y-plan-contracts.toml — ask registry.py contracts; read one contracts/a11y-plan/{key}.md; never whole.

## 4. ARIA Patterns & Roles

Detail: .andromeda/a11y-plan-detail/aria-patterns-roles.md — read whole.

**Landmark roles inventory (engine HTML-to-AccessKit mapping):**
- Native element mapping follows HTML-AAM: landmarks, headings, lists, tables, interactive and inline semantics (packages/blitz-dom/src/accessibility.rs:440-508)

**Per-component pattern catalog (engine mapping):**

- **ARIA state attributes** — `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` map to `selected`, `toggled` and `invalid` as §2 Role derivation states, `role="switch"` mapping to `Switch`.

- **Hidden content** — `aria-hidden="true"` keeps a node, marked hidden; `hidden`, `display: none` and `visibility: hidden` exclude it and its children, an `option` under a `select` excepted from `display: none`.

**Application markup:**

**Fixture markup (examples/assets):**

**Observed absent (ARIA searches):**

---

## 5. Keyboard Navigation

Detail: .andromeda/a11y-plan-detail/keyboard-navigation.md — read whole.

**Focus order per layout (engine):**

- An element is focusable if it holds a sub-document, or is not inert, is no disabled form control and has `tabindex >= 0` or, with no tabindex, is an `<a>`/`<area>` with `href` or a `button`, `input`, `select`, `textarea`, `frame`, `iframe` or `summary`; `disabled` counts by presence on a form control alone; HTML's focus fixup is not built, so a node focused as it becomes inert keeps focus until focus moves — its fix owned by a `.andromeda/residuals.md` line (target: next).

- Disabled-ness is keyed one way: the `DISABLED`/`ENABLED` state (hence `:disabled`), the pointer gates, Enter and Space activation and focusability read the `disabled` attribute's presence on a `button`, `input`, `select` or `textarea`, so `disabled="false"` and a bare `disabled` on a form control match `:disabled` and are not focusable.

**Focus restoration:**

- A pointer click on a `button` or an `input` of type `submit`, `reset` or `button` focuses it, submits the form owner when it is a submit button and ends the walk, and a click on a `select` focuses it; a disabled form control stops the walk first, a disabled ancestor `div` not; a click on any other element is unchanged — a CRUD row or the list still clears focus.

- Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock (a harness-built document's `VirtualClock`), each still focusing its target as one click does.

- A click on a range input focuses it and changes no value; a main-button pointerdown on one not disabled sets the value from the hit's fraction of the content box, clamped to [0, 1].

**Keyboard handling (engine and shell):**

- A focused `input type="range"` with no `disabled` attribute steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, to its minimum on Home and its maximum on End, every other key doing nothing; its value is held apart from the `value` attribute, sanitized to `[min, max]` on the step grid, and each change fires one `input` event, a move that changes nothing none.

- A focused single-choice `select` that is no disabled form control takes ArrowDown / ArrowUp to the next / previous enabled option, Home / End to the first / last, and a character with no Ctrl, Alt or Meta as typeahead kept for 1 s of the document's clock; every other key does nothing; each change dispatches one `input` then one `change`, a key that changes nothing none; a `multiple` select and an `optgroup`'s options are not modelled, and no popup is painted.

- A pressed Enter on the focused node dispatches a Click when it is no disabled form control and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`; a released Space dispatches it for a `summary`, or for a `button` or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is no disabled form control, never a link; the app's handlers run, then `handle_click`, so SC 2.1.1 is met by measurement on the stand's buttons.

**Script and framework exposure:**

- A falsy Dioxus value (Bool false, Text `"false"`, Int 0, Float 0.0, None) of one of the bridge's 28 boolean attributes — upstream's `is_bool_attr` set, `disabled`, `hidden` and `inert` among them — removes the attribute on both attribute paths, so an enabled control does not match `:disabled` and `hidden: false` keeps an element laid out and in the tree; a truthy value is still written, and an attribute outside the list holding `false` is written as `"false"`.

**Per-surface keyboard shortcuts:**

- seven_guis CRUD: the person rows are `option`s of a `listbox` by `role` and not focusable; the list `crud-list`, named `People`, is the third CRUD Tab stop and, focused, ArrowDown / ArrowUp / Home / End move the selection as a row's click does; the tree's focus is the list, never the option — the builder reads no `aria-activedescendant` (SC 4.1.2 work outside the tier-0 target), kept unread and pinned by `accessibility_active_descendant_unread`.

**Fixture markup:**

**Test harness pattern:**
- Focusability tests are listed under Focus order above (tests/blitz-tests/tests/focusability_updates.rs:34-57)

- `stand_keyboard` walks each lean task's stated Tab order (`tab_order`, and `ENABLED_TAB_ORDER` for the states enabled by keyboard alone) forward and back in both layout modes, the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step, every stand control lying in one of the two orders.

- Observed absent — tabindex handling in the searched s06, s07 and s10 slice files, and focus and keyboard handling in the s07 files.

> NOT YET MEASURED — skip links, modal focus traps and route-change focus restoration: no slice recorded them; s01 recorded keyboard focus handling code as out of slice and s13 recorded keyboard interaction as out of slice.

---

## 6. Visual Design Verification

Detail: .andromeda/a11y-plan-detail/visual-design-verification.md — read whole.

**Color contrast:**

- SC 1.4.3 on the stand: `Harness::text_contrast` reads the text colour, font metrics and the background composited behind it from computed styles (`None` under opacity below 1, a filter or a background image); `contrast_ratio` is the WCAG 2.x ratio, 3.0 required of large text, else 4.5; `stand_contrast` asserts three tables in both layout modes, a row exempt exactly where the snapshot's `enabled` reads `Some(false)`, so SC 1.4.3 is met on every measured non-exempt stand row.

- escher-driver's visual lint (`Session::lint`, the `lint` verb on every carrier) reads `contrast` through `Harness::text_contrast` with `stand_contrast`'s exempt rule and no second formula or threshold, a finding written as ids, closed words and numbers only.

- Observed absent — contrast checks in the searched s05 and s11 files, and user-preference media handling in the s11 files.

**Focus ring:**

**Motion:**

- Observed absent — prefers-reduced-motion and prefers-color-scheme rules in the searched s02 files.

**State color (not-color-alone):**

**Color scheme, zoom and user preferences:**

**Visibility and hit-testing:**

- Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot, a select's option excepted from `display: none` (§4 Hidden content); a hit passes through an element carrying `inert`, and removing `inert` restores the tree and the hit.

> NOT YET MEASURED — focus-ring, target-size and typography-readability tokens: no slice recorded them, and s01 and s07 recorded contrast and visual checks as out of slice. The stand controls' contrast pairs are measured since 2026-10-13-stand-contrast-harness (Color contrast above), and the CRUD list rows, the text outside the controls and the colour states reached by input, hover and active included, since 2026-10-14-stand-a11y-assertions.

---

## 7. Screen Reader Support

Detail: .andromeda/a11y-plan-detail/screen-reader-support.md — read whole.

**Accessibility tree output:**

- An element's `aria-label`, non-empty after trimming, becomes its `label` (accname §2C), a whitespace-only one naming nothing; an `aria-labelledby` whose targets hold text outranks it (accname 2B), the `aria-label` the fallback when they hold none.

- A `<label>` names the labelable element it is bound to — its `for`-target by id, else its first nested labelable element — by being pushed onto its `labelled_by` after the tree is built, so a label before or after its element both work; a hidden input is named by no label, and an input carrying `aria-labelledby` is named by its targets instead (accname 2B over 2D).

- In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, no `TextRun`, document-root or `Window` node one; on the stand every interactive-role node of the four lean tasks reads a non-empty name and the tree the shell hands the adapter equals the harness's tree node for node; the roles and names the checks expect are stated once, in the shared module's tables — never a second role or name mapping.

- The snapshot model consumes the tree: `DioxusDocument::snapshot` takes each node's role, name, focus and `selected`, `pressed` and `invalid` from `accessibility_tree`, never from the element, and `enabled` (the presence of `disabled`) and `value` from the element, `checked` from a native checkbox's or radio's state, else a checkable role's `toggled`; its text form, parts, diff, MCP results, node-key help and `screenshot` read that model and nothing else — never a second role or name mapping.

- A password input's value reads as the fixed `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no snapshot field, tree node or painted frame (blitz-paint paints bullets, and a click places the caret by them), and a file input with a non-empty `value` reads the same marker; the driver refuses a `click` or a `type` as `disabled` where the snapshot's `enabled` reads `Some(false)`, nothing dispatched.

- The actionable-key rule: every element an agent can act on — focusable, carrying an interactive role or an event listener — reads an author key, so its `author_id` is that key and holds under any edit of the code around it; `DioxusDocument::unkeyed_actionable` is the check, with no second role mapping, and returns nothing on every stand task; no snapshot field says an element reacts to a click — that reading owned by a `.andromeda/residuals.md` line (target: next).

- On the stand three kinds of control state a role or a state by attribute alone — the CRUD list `listbox` with `option` rows carrying `aria-selected`, the flight booker's mode buttons `aria-pressed`, its date inputs `aria-invalid`; one gap stands open: the rows are not focusable and the tree's focus is the list, never the option, the missing `aria-activedescendant` kept unread (§5 Per-surface keyboard shortcuts).

**Landmark roles:** the engine's landmark mapping is recorded in §4.

**Platform adapter:**
- `Adapter` creation must happen before the window is first shown and panics if it is already visible (packages/accesskit_xplat/src/lib.rs:180-196)

**Language and direction:**

- Observed absent — an html lang attribute in 10 of the 21 searched s02 fixtures.

**Text alternatives and hidden decoration (fixtures):**

**Input method (IME):**

**Observed absent:**

> NOT YET MEASURED — live regions, heading-hierarchy handling and per-surface screen-reader test specs: no slice recorded them; s01, s07, s10 and s13 recorded screen-reader handling as out of slice, and s08 recorded the screen-reader tree output as out of slice.

---

## 8. Cognitive Accessibility

Detail: .andromeda/a11y-plan-detail/cognitive-accessibility.md — read whole.

**Error recovery:**

- Invalid dates get an `invalid` class with red styling and disable the Book button; the two date inputs also state `aria-invalid`, so the invalid state is in the tree and the snapshot and not carried by the class alone; the headless stand asserts the cue without colour.

**Orientation and status cues:**

> NOT YET MEASURED — timeout extensions, plain-language targets, redundant-entry avoidance and accessible authentication: every slice except s04 recorded cognitive accessibility as out of slice.

---

## 9. CI Integration

Detail: .andromeda/a11y-plan-detail/ci-integration.md — read whole.

ci.yml's `a11y` job, "Accessibility (a11y) tests", runs `bash .github/scripts/ci-leg.sh a11y` — `cargo test --workspace --locked` over six files, `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` — in the slow tier behind the four fast jobs, with no required status check; the three `stand_*` files also run three times in the `stand-repeat` leg under the zero-failure budget (test-plan §10).

---

## 10. SLO Invariants & A11y Budgets

> NO RECORDED INTENT

---

## 11. A11y Anti-Patterns (NEVER do these)

> NO RECORDED INTENT

---

<!-- subjects: scope-entities · strategy · aria-landmarks · aria-patterns · keyboard-focus-order · keyboard-focus-restoration · keyboard-handling · keyboard-framework · keyboard-shortcuts · keyboard-fixtures · keyboard-harness · visual-contrast · visual-focus-ring · visual-motion · visual-state-color · visual-preferences · visual-visibility · sr-tree · sr-author-id · sr-snapshot · sr-actionable-keys · sr-stand-states · sr-platform-adapter · sr-language · sr-text-alternatives · sr-ime · cognitive-error-recovery · cognitive-orientation · ci -->

## 12. A11y Decisions Log

> NO RECORDED INTENT
