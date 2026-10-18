group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:12   class: omission
  leaf-text: «in the workspace test leg and per package, outside the a11y leg's six files.»
  source: .andromeda/a11y-plan.md:71 «`accessibility_merged_states` (3), `inert_subtree` (4) and `accessibility_active_descendant_unread` (1) assert the builder off the built tree in both layout modes»
  replace-with: «in the workspace test leg and per package, outside the a11y leg's six files; beside them `accessibility_merged_states` (3: the merged `aria-expanded`, `aria-disabled` and `aria-level`, each a stated table with its count asserted), `inert_subtree` (4) and `accessibility_active_descendant_unread` (1) assert the builder off the built tree in both layout modes, in the same legs and outside the a11y leg.»

- leaf: .claude/docs/a11y-summary.md:20   class: omission
  leaf-text: «`aria-hidden` marks hidden.»
  source: .andromeda/a11y-plan.md:267 «Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree, and so from the snapshot»
  replace-with: «`aria-hidden` marks hidden; an element carrying `inert` is excluded with its subtree too, and so from the snapshot, and a hit passes through it, removing `inert` restoring the tree and the hit (the `inert` read PROVISIONAL).»

- leaf: .claude/docs/a11y-summary.md:21   class: stale
  leaf-text: «`aria-checked` is not read — reading it as `checked` or `pressed` is owned by a `.andromeda/residuals.md` line (target: next) on the founder's C7 (2026-10-14), `accessibility_states`' fourth test standing as its pin.»
  source: .andromeda/a11y-plan.md:46 «`aria-checked` `true` / `false` / `mixed` sets `toggled` on a `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` role alone»
  replace-with: «`aria-pressed` sets `toggled` on every role but the five checkable ones, and `aria-checked` `true` / `false` / `mixed` sets it on a `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` role alone (`role="switch"` mapping to `Switch`), a native `input` of type `checkbox` or `radio` reading neither — the toggle read per role PROVISIONAL, `accessibility_states`' fourth test its pin; each attribute is read once and nothing else writes `toggled`, so the builder writes none from a native checkbox's or radio's own checked state — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next).»

- leaf: .claude/docs/a11y-summary.md:21   class: stale
  leaf-text: «Check: `accessibility_states` (4, both layout modes; outside the `a11y` leg).»
  source: .andromeda/a11y-plan.md:69 «5 tests, the fourth reading `aria-checked`'s three values on a checkable role alone and four unchecked natives reading neither attribute»
  replace-with: «Check: `accessibility_states` (5, both layout modes; outside the `a11y` leg — the fourth reading `aria-checked`'s three values on a checkable role alone and four unchecked natives reading neither attribute, the fifth `aria-pressed` on a checkable role reading nothing).»

- leaf: .claude/docs/a11y-summary.md:22   class: stale
  leaf-text: «is SC 4.1.2 work outside the tier-0 target, owned by a `.andromeda/residuals.md` line (target: next) on the founder's F2 (2026-10-14).»
  source: .andromeda/a11y-plan.md:199 «and pinned by `accessibility_active_descendant_unread` (PROVISIONAL)»
  replace-with: «is SC 4.1.2 work outside the tier-0 target, kept unread on the founder's F2 (2026-10-14) and pinned by `accessibility_active_descendant_unread` (PROVISIONAL).»

- leaf: .claude/docs/a11y-summary.md:23   class: stale
  leaf-text: «a `disabled` attribute still stops the walk first, and a click on any other element»
  source: .andromeda/a11y-plan.md:161 «a disabled form control still stops the walk first, a disabled ancestor `div` not (PROVISIONAL)»
  replace-with: «a disabled form control still stops the walk first, a disabled ancestor `div` not (PROVISIONAL), and a click on any other element»

- leaf: .claude/docs/a11y-summary.md:23   class: stale
  leaf-text: «a pressed Enter on the focused node with no `disabled` attribute (presence) dispatches a Click»
  source: .andromeda/a11y-plan.md:169 «dispatches a Click on it when it is no disabled form control (`ElementData::is_disabled`; PROVISIONAL)»
  replace-with: «a pressed Enter on the focused node that is no disabled form control (`ElementData::is_disabled`; PROVISIONAL — a `disabled` attribute on a `summary` or an `a` gating neither key, as HTML does) dispatches a Click»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «the `DISABLED` state (`:disabled`), click targeting, the Enter and Space activation and focusability all read the attribute's presence — `disabled="false"` and a bare `disabled` match `:disabled` and are not focusable, and a bare `disabled` set after creation takes focusability away; a known limit, pinned by `stand_keyboard`: a `<div disabled tabindex=0>` is not focusable where browsers focus it, its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next) on the founder's C15 (2026-10-14), `stand_keyboard`'s pin standing.»
  source: .andromeda/a11y-plan.md:151 «that presence on a `button`, `input`, `select` or `textarea` alone (PROVISIONAL)»
  replace-with: «the `DISABLED` state (`:disabled`) reads the attribute's presence, and the pointer-move gate, `ClickTarget::Disabled`, `handle_click`'s walk gate, the Enter and Space activation and focusability read `ElementData::is_disabled` — that presence on a `button`, `input`, `select` or `textarea` alone (PROVISIONAL) — so `disabled="false"` and a bare `disabled` on a form control match `:disabled` and are not focusable, a bare `disabled` set after creation takes its focusability away, and a `<div disabled tabindex=0>` takes focus.»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «while the engine honours no `inert`: a truthy one takes no focusability away, and Tab reaches the buttons under `inert: true` and `inert: false` alike — owned by a `.andromeda/residuals.md` line (target: next) on the founder's F3 (2026-10-14), `stand_keyboard::the_engine_honours_no_inert` staying its pin and whether an inert subtree takes a hit unmeasured)»
  source: .andromeda/a11y-plan.md:189 «while a truthy `inert` takes focusability, hits and the tree away from its subtree (Focus order above; §6 Visibility and hit-testing; PROVISIONAL)»
  replace-with: «while a truthy `inert` (by presence on itself or an ancestor element) takes focusability, hits and the tree away from its subtree (PROVISIONAL) and Tab reaches the buttons under `inert: false`; HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next))»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «the bullet a click lands on depends on the real characters' widths — measured since 2026-10-14-stand-requirement-sweep: a click 30 px into the content box (the bundled font, 20 px) places the caret at index 5 after eight narrow characters and at 2 after eight wide ones, in both layout modes (`password_caret_click.rs`); placing it by the bullet layout is owned by a `.andromeda/residuals.md` line (target: next) on the founder's C11 (2026-10-14), this reading its starting point;»
  source: .andromeda/a11y-plan.md:284 «a click inside a field holding a value places the caret by the bullets»
  replace-with: «a click inside a field holding a value places the caret by the bullets — `click_password_input` lays out one U+2022 per character through `BaseDocument::layout_as_text_input`, finds the bullet under the point and moves the real editor to that character, so no width of the real characters reaches the caret: two values of one length read one index, in both layout modes (`password_caret_click.rs`; PROVISIONAL);»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «`checked` and `value` from the element (»
  source: .andromeda/a11y-plan.md:284 «`checked` is a native checkbox's or radio's state, else, on a checkable role, its `toggled` reading»
  replace-with: «`checked` a native checkbox's or radio's state, else, on a checkable role, the node's `toggled` reading (`True` true, `False` and `Mixed` false; PROVISIONAL), and `value` from the element (»

- leaf: .claude/rules/a11y.md:24   class: omission
  leaf-text: «`aria-hidden="true"` keeps it, marked hidden.»
  source: .andromeda/a11y-plan.md:267 «Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree, and so from the snapshot»
  replace-with: «`aria-hidden="true"` keeps it, marked hidden; an element carrying `inert` is excluded with its children too, and so from the snapshot, and a hit passes through it, removing `inert` restoring the tree and the hit (the `inert` read PROVISIONAL).»

- leaf: .claude/rules/a11y.md:25   class: stale
  leaf-text: «State = three ARIA attributes read by value on every element, whatever its role (trimmed, lower-case, exact)»
  source: .andromeda/a11y-plan.md:46 «The builder writes three states on an element node from its ARIA state attributes, on every element, the toggle by role»
  replace-with: «State = ARIA state attributes read by value on every element, the toggle by role (trimmed, lower-case, exact)»

- leaf: .claude/rules/a11y.md:25   class: stale
  leaf-text: «`aria-pressed` `true`/`false`/`mixed` → `toggled`;»
  source: .andromeda/a11y-plan.md:46 «`aria-pressed` `true` / `false` / `mixed` sets `toggled` True / False / Mixed, anything else nothing, on every role but the five checkable ones»
  replace-with: «`aria-pressed` `true`/`false`/`mixed` → `toggled` on every role but the five checkable ones;»

- leaf: .claude/rules/a11y.md:25   class: stale
  leaf-text: «`aria-checked` is not read — do not fold it into `checked` or `pressed`: reading it so is owned by a `.andromeda/residuals.md` line (target: next) on the founder's C7 (2026-10-14), `accessibility_states`' fourth test standing as its pin.»
  source: .andromeda/a11y-plan.md:46 «`aria-checked` `true` / `false` / `mixed` sets `toggled` on a `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` role alone»
  replace-with: «`aria-checked` `true`/`false`/`mixed` → `toggled` on a `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` role alone (`role="switch"` mapping to `Switch`), a native `input` of type `checkbox` or `radio` reading neither attribute — the toggle read per role PROVISIONAL, `accessibility_states`' fourth test its pin; each attribute is read once and nothing else writes `toggled`, so the builder writes none from a native checkbox's or radio's own checked state — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next).»

- leaf: .claude/rules/a11y.md:25   class: omission
  leaf-text: «The snapshot takes the three from the tree node, never from the element,»
  source: .andromeda/a11y-plan.md:284 «`checked` is a native checkbox's or radio's state, else, on a checkable role, its `toggled` reading»
  replace-with: «The snapshot takes the three from the tree node, never from the element, its `checked` is a native checkbox's or radio's state, else, on a checkable role, the node's `toggled` (`Mixed` reading false; PROVISIONAL),»

- leaf: .claude/rules/a11y.md:31   class: stale
  leaf-text: «or it carries no `disabled` attribute (read by presence, any value, since 2026-10-13-stand-keyboard-harness) and `tabindex >= 0`»
  source: .andromeda/a11y-plan.md:147 «is not inert (by presence on itself or an ancestor element, `Node::is_inert`), is no disabled form control and either has `tabindex >= 0`»
  replace-with: «or it is not inert (by presence on itself or an ancestor element, `Node::is_inert`) and is no disabled form control (`ElementData::is_disabled`: a `disabled` attribute present, any value, on a `button`, `input`, `select` or `textarea` alone; the narrowing and the `inert` read PROVISIONAL) and has `tabindex >= 0`»

- leaf: .claude/rules/a11y.md:32   class: stale
  leaf-text: «the `DISABLED` state (`:disabled`), click targeting, the Enter and Space activation and focusability all read PRESENCE — `disabled="false"` and a bare `disabled` match `:disabled` and are not focusable, and a bare `disabled` set after creation takes focusability away; a known limit, pinned by `stand_keyboard`: a `<div disabled tabindex=0>` is not focusable where browsers apply `disabled` to form controls only and focus it — its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next) on the founder's C15 (2026-10-14), `stand_keyboard`'s pin standing.»
  source: .andromeda/a11y-plan.md:151 «that presence on a `button`, `input`, `select` or `textarea` alone (PROVISIONAL)»
  replace-with: «the `DISABLED` state (`:disabled`) reads PRESENCE, and the pointer-move gate, `ClickTarget::Disabled`, `handle_click`'s walk gate, the Enter and Space activation and focusability read `ElementData::is_disabled` — that presence on a `button`, `input`, `select` or `textarea` alone (PROVISIONAL) — so `disabled="false"` and a bare `disabled` on a form control match `:disabled` and are not focusable, a bare `disabled` set after creation takes its focusability away, and a `<div disabled tabindex=0>` takes focus.»

- leaf: .claude/rules/a11y.md:32   class: stale
  leaf-text: «while the engine honours no `inert`: a truthy one takes no focusability away, and Tab reaches the buttons under `inert: true` and `inert: false` alike — owned by a `.andromeda/residuals.md` line (target: next) on the founder's F3 (2026-10-14), `stand_keyboard::the_engine_honours_no_inert` staying its pin and whether an inert subtree takes a hit unmeasured)»
  source: .andromeda/a11y-plan.md:189 «while a truthy `inert` takes focusability, hits and the tree away from its subtree (Focus order above; §6 Visibility and hit-testing; PROVISIONAL)»
  replace-with: «while a truthy `inert` (by presence on itself or an ancestor element, `Node::is_inert`) takes focusability, hits and the tree away from its subtree (PROVISIONAL) and Tab reaches the buttons under `inert: false`; HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next))»

- leaf: .claude/rules/a11y.md:33   class: stale
  leaf-text: «a pressed Enter on the focused node with no `disabled` attribute (presence) dispatches a Click»
  source: .andromeda/a11y-plan.md:169 «dispatches a Click on it when it is no disabled form control (`ElementData::is_disabled`; PROVISIONAL)»
  replace-with: «a pressed Enter on the focused node that is no disabled form control (`ElementData::is_disabled`; PROVISIONAL — a `disabled` attribute on a `summary` or an `a` gating neither key, as HTML does) dispatches a Click»

- leaf: .claude/rules/a11y.md:33   class: stale
  leaf-text: «a `disabled` attribute still stops the walk first, and a click on any other element»
  source: .andromeda/a11y-plan.md:161 «a disabled form control still stops the walk first, a disabled ancestor `div` not (PROVISIONAL)»
  replace-with: «a disabled form control still stops the walk first, a disabled ancestor `div` not (PROVISIONAL), and a click on any other element»

- leaf: .claude/rules/a11y.md:42   class: stale
  leaf-text: «`accessibility_states` (the three state attributes, one stated table per attribute with its row count asserted)»
  source: .andromeda/a11y-plan.md:69 «5 tests, the fourth reading `aria-checked`'s three values on a checkable role alone and four unchecked natives reading neither attribute»
  replace-with: «`accessibility_states` (5 tests: the three state attributes, one stated table per attribute with its row count asserted, the fourth reading `aria-checked` on a checkable role alone and four unchecked natives reading neither attribute, the fifth `aria-pressed` on a checkable role reading nothing)»

- leaf: .claude/rules/a11y.md:42   class: omission
  leaf-text: «and `accessibility_description` (3), which assert the name and description reads with `verify_that!`, ride the workspace test leg, not this one.»
  source: .andromeda/a11y-plan.md:71 «`accessibility_merged_states` (3), `inert_subtree` (4) and `accessibility_active_descendant_unread` (1) assert the builder off the built tree in both layout modes»
  replace-with: «and `accessibility_description` (3), which assert the name and description reads with `verify_that!`, and `accessibility_merged_states` (3: `aria-expanded`, `aria-disabled`, `aria-level`), `inert_subtree` (4) and `accessibility_active_descendant_unread` (1), which assert the builder off the built tree in both layout modes, ride the workspace test leg, not this one.»
