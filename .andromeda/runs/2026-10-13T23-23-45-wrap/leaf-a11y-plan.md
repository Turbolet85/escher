group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:14   class: stale
  leaf-text: «; the headless keyboard harness itself is still to come.»
  source: .andromeda/a11y-plan.md:75 «since 2026-10-13-stand-keyboard-harness `stand_keyboard` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks through the same `press` / `press_with`»
  replace-with: «; since 2026-10-13-stand-keyboard-harness `stand_keyboard.rs` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks through the same `press` / `press_with`, reading the focused node back per step, in both layout modes.»

- leaf: .claude/docs/a11y-summary.md:22   class: stale
  leaf-text: «Two gaps stand open: a row is not focusable, so no keyboard reaches an option ("Stand keyboard harness"), and the list reads no accessible name ("Stand a11y assertions").»
  source: .andromeda/a11y-plan.md:196 «since 2026-10-13-stand-keyboard-harness the list `crud-list` takes `tabindex="0"` and an `onkeydown` — the third CRUD Tab stop — and, focused, ArrowDown selects the next visible row»
  replace-with: «Since 2026-10-13-stand-keyboard-harness (the operator's decision) the list `crud-list` takes `tabindex="0"` and an `onkeydown` — the third CRUD Tab stop — and, focused, ArrowDown selects the next visible row (the first when none is selected), ArrowUp the previous, Home the first and End the last, as a row's click does. Two gaps stand open: the rows are not focusable and the tree's focus is the list, never the option — the builder reads no `aria-activedescendant`, so an assistive technology learns the current row only from its `selected` reading (a CARRY on "Stand a11y assertions") — and the list reads no accessible name ("Stand a11y assertions").»

- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «focus resets to body on removal;»
  source: .andromeda/a11y-plan.md:158 «A pointer click on a `button` (any type) or an `input` of type `submit`, `reset` or `button` focuses it through `generate_focus_events` / `set_focus_to`»
  replace-with: «focus resets to body on removal; since 2026-10-13-stand-keyboard-harness, in every document (ratified by the founder), a pointer click on a `button` (any type) or an `input` of type `submit`, `reset` or `button` focuses it, then submits the form owner when it is a submit button — a `disabled` attribute still stops the walk first, and a click on any other element (a CRUD row, the list) still clears focus;»

- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «`:focus-visible` / `:focus-within` never match.»
  source: .andromeda/a11y-plan.md:166 «A pressed Enter whose target is the focused node, after the text-input and range arms, dispatches a Click on it when it carries no `disabled` attribute (presence) and is a `button`, a `summary`»
  replace-with: «`:focus-visible` / `:focus-within` never match. Enter and Space (since 2026-10-13-stand-keyboard-harness, in every document, ratified by the founder): a pressed Enter on the focused node with no `disabled` attribute (presence) dispatches a Click on a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`; a released Space (`handle_key_up`) dispatches the same Click on a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` — a link takes no Space, and a text input and a range input take neither key there; the Click (`Node::synthetic_click_event_data`) runs the app's handlers (a Dioxus `onclick`), then `handle_click`.»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «Disabled-ness is keyed two ways: the `DISABLED` state (`:disabled`) and click targeting on the attribute's presence, focusability on its value parsed as a bool — `disabled="false"` matches `:disabled` yet stays focusable.»
  source: .andromeda/a11y-plan.md:148 «Disabled-ness is keyed one way since 2026-10-13-stand-keyboard-harness: the `DISABLED`/`ENABLED` element state (hence `:disabled`), the pointer click target, the Enter and Space activation»
  replace-with: «Disabled-ness is keyed one way since 2026-10-13-stand-keyboard-harness (in every document, ratified by the founder): the `DISABLED` state (`:disabled`), click targeting, the Enter and Space activation and focusability all read the attribute's presence — `disabled="false"` and a bare `disabled` match `:disabled` and are not focusable, and a bare `disabled` set after creation takes focusability away; a known limit, pinned by `stand_keyboard`: a `<div disabled tabindex=0>` is not focusable where browsers focus it, its narrowing to `button`, `input`, `select` and `textarea` a CARRY on "Stand requirement sweep".»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «a framework-written `inert: false` is removed, so it no longer makes content inert)»
  source: .andromeda/a11y-plan.md:186 «a framework-written `inert: false` is removed — `inert` the 28th name `dioxus_falsy_boolean_attrs` reads since 2026-10-13-stand-keyboard-harness — while the engine honours no `inert`»
  replace-with: «a framework-written `inert: false` is removed, while the engine honours no `inert`: a truthy one takes no focusability away, and Tab reaches the buttons under `inert: true` and `inert: false` alike — a CARRY on "Stand a11y assertions")»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «the Flight Booker's Book names eight nodes that only moved, each `differs` = `[bounds]`»
  source: .andromeda/a11y-plan.md:281 «seven that only moved, `differs` = `[bounds]`, and since 2026-10-13-stand-keyboard-harness `flight-book` itself, which the click now focuses, `differs` = `[focused, bounds]`»
  replace-with: «the Flight Booker's Book names eight nodes — seven that only moved, each `differs` = `[bounds]`, and since 2026-10-13-stand-keyboard-harness `flight-book` itself, which the click now focuses, `differs` = `[focused, bounds]`»

- leaf: .claude/docs/a11y-summary.md:25   class: omission
  leaf-text: «`stand_accessibility_ids.rs` pins each lean task's Tab sequence (`focus_next_node` from the root element)»
  source: .andromeda/a11y-plan.md:326 «against the shared table `tab_order` — moved into the stand checks' shared module since 2026-10-13-stand-keyboard-harness, CRUD's sequence restated there to six stops»
  replace-with: «`stand_accessibility_ids.rs` pins each lean task's Tab sequence (`focus_next_node` from the root element) against the shared table `tab_order` in the stand checks' shared module — CRUD's restated since 2026-10-13-stand-keyboard-harness to six stops, `back-btn`, `crud-filter`, `crud-list`, `crud-name`, `crud-surname`, `crud-create`, the three other tasks' unchanged»

- leaf: .claude/docs/a11y-summary.md:25   class: stale
  leaf-text: «no test asserts SC-level keyboard behaviour.»
  source: .andromeda/a11y-plan.md:166 «SC 2.1.1 on the stand's buttons is met by measurement: Enter and Space on each focused stand button read as its click (20 rows»
  replace-with: «since 2026-10-13-stand-keyboard-harness SC 2.1.1 on the stand's buttons is met by measurement: `stand_keyboard.rs` reads Enter and Space on each focused stand button as its click (20 rows, `counter-increment`, `flight-return`, `flight-book` and `crud-create` changing the screen), and through the driver `stand_act_keys.rs` presses `enter` and `space` on a Tab-focused `counter-increment`, each naming `counter-value`.»

- leaf: .claude/rules/a11y.md:31   class: stale
  leaf-text: «or `disabled` does not parse as `true` and `tabindex >= 0`»
  source: .andromeda/a11y-plan.md:144 «or if it carries no `disabled` attribute and either has `tabindex >= 0`»
  replace-with: «or it carries no `disabled` attribute (read by presence, any value, since 2026-10-13-stand-keyboard-harness) and `tabindex >= 0`»

- leaf: .claude/rules/a11y.md:32   class: stale
  leaf-text: «`disabled` is keyed two ways: the `DISABLED` state (`:disabled`) and click targeting on PRESENCE, focusability on the PARSED bool — `disabled="false"` reads disabled to a presence check yet stays focusable.»
  source: .andromeda/a11y-plan.md:148 «Disabled-ness is keyed one way since 2026-10-13-stand-keyboard-harness: the `DISABLED`/`ENABLED` element state (hence `:disabled`), the pointer click target, the Enter and Space activation»
  replace-with: «`disabled` is keyed one way since 2026-10-13-stand-keyboard-harness (in every document, ratified by the founder): the `DISABLED` state (`:disabled`), click targeting, the Enter and Space activation and focusability all read PRESENCE — `disabled="false"` and a bare `disabled` match `:disabled` and are not focusable, and a bare `disabled` set after creation takes focusability away; a known limit, pinned by `stand_keyboard`: a `<div disabled tabindex=0>` is not focusable where browsers apply `disabled` to form controls only and focus it — its narrowing to `button`, `input`, `select` and `textarea` a CARRY on "Stand requirement sweep".»

- leaf: .claude/rules/a11y.md:32   class: stale
  leaf-text: «a framework-written `inert: false` is removed, so it no longer makes content inert)»
  source: .andromeda/a11y-plan.md:186 «a framework-written `inert: false` is removed — `inert` the 28th name `dioxus_falsy_boolean_attrs` reads since 2026-10-13-stand-keyboard-harness — while the engine honours no `inert`»
  replace-with: «a framework-written `inert: false` is removed, while the engine honours no `inert`: a truthy one takes no focusability away, and Tab reaches the buttons under `inert: true` and `inert: false` alike — a CARRY on "Stand a11y assertions")»

- leaf: .claude/rules/a11y.md:33   class: omission
  leaf-text: «Tab/Shift+Tab move through `focus_next_node` / `focus_prev_node`;»
  source: .andromeda/a11y-plan.md:166 «A pressed Enter whose target is the focused node, after the text-input and range arms, dispatches a Click on it when it carries no `disabled` attribute (presence) and is a `button`, a `summary`»
  replace-with: «Tab/Shift+Tab move through `focus_next_node` / `focus_prev_node`; since 2026-10-13-stand-keyboard-harness (in every document, ratified by the founder) a pressed Enter on the focused node with no `disabled` attribute (presence) dispatches a Click on a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`, and a released Space (`handle_key_up`, routed by `events/mod.rs`) the same Click on a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` — a link takes no Space, a text input and a range input take neither key there; the Click (`Node::synthetic_click_event_data`) is queued by the event driver, so the app's handlers run, then `handle_click`;»

- leaf: .claude/rules/a11y.md:33   class: omission
  leaf-text: «removing the focused node resets focus to body and runs blur side effects.»
  source: .andromeda/a11y-plan.md:158 «A pointer click on a `button` (any type) or an `input` of type `submit`, `reset` or `button` focuses it through `generate_focus_events` / `set_focus_to`»
  replace-with: «removing the focused node resets focus to body and runs blur side effects. Since 2026-10-13-stand-keyboard-harness (in every document, ratified by the founder) a pointer click on a `button` (any type) or an `input` of type `submit`, `reset` or `button` focuses it through `generate_focus_events` / `set_focus_to`, then submits the form owner when it is a submit button; a `disabled` attribute still stops the walk first, and a click on any other element — a CRUD row or the list — still clears focus.»

- leaf: .claude/rules/a11y.md:35   class: stale
  leaf-text: «; the headless keyboard harness is owned by the working route's "Stand keyboard harness" chunk.»
  source: .andromeda/a11y-plan.md:75 «since 2026-10-13-stand-keyboard-harness `stand_keyboard` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks through the same `press` / `press_with`»
  replace-with: «; since 2026-10-13-stand-keyboard-harness `stand_keyboard` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks through the same `press` / `press_with`, reading the focused node back per step in both layout modes — each task's stated sequence is the shared table `tab_order` (CRUD six stops, `crud-list` third: it takes `tabindex="0"` and an `onkeydown`, ArrowDown / ArrowUp / Home / End moving the selection, its rows not focusable), and through the driver `stand_act_keys` presses `enter` and `space` on a Tab-focused `counter-increment`.»
