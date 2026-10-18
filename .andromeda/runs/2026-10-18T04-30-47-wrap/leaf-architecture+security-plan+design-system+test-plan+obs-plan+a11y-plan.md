group: architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «a hit stops at the padding box of a box that clips by `overflow`;»
  source: .andromeda/architecture.md:137 «a node paint clips — either axis of its computed `overflow` not `visible`, or a computed `contain` that includes `paint` on a box paint applies it to»
  replace-with: «a hit stops at the padding box of a box paint clips — by `overflow`, or by a computed `contain` that includes `paint` (PROVISIONAL) — and passes through an element carrying `inert`;»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «a scrolled box reads its own client rect where it stands)»
  source: .andromeda/architecture.md:141 «The client rect under transforms (PROVISIONAL): `BaseDocument::get_client_bounding_rect` and `node_client_rects` read an element where its transforms move it»
  replace-with: «a scrolled box reads its own client rect where it stands, an element under a transform where its transforms move it, and `Node::absolute_position` subtracts only its containing blocks' scroll offsets — the last two PROVISIONAL)»

- leaf: CLAUDE.md:22   class: stale
  leaf-text: «`aria-checked` unread;»
  source: .andromeda/architecture.md:135 «`aria-checked`: the same three values set `toggled` on a role in the private `is_checkable` set — `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` — and on no other»
  replace-with: «`toggled` read from `aria-checked` in place of `aria-pressed` on the five checkable roles (`CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio`, `role="switch"` mapping to `Switch`), a native checkbox or radio reading neither (PROVISIONAL); an inert subtree left out of the tree;»

- leaf: CLAUDE.md:22   class: stale
  leaf-text: «each with no `disabled` attribute, dispatch a Click»
  source: .andromeda/security-plan.md:130 «that is not disabled — a `disabled` attribute on a `summary` or an `a` gating neither key»
  replace-with: «each not a disabled form control (`ElementData::is_disabled`, PROVISIONAL) — a `disabled` attribute on a `summary` or an `a` gating neither key — dispatch a Click»

- leaf: CLAUDE.md:22   class: stale
  leaf-text: «focusability reads `disabled` by its presence on any element»
  source: .andromeda/architecture.md:97 «on a form control alone (PROVISIONAL): `ElementData::is_disabled` is `can_be_disabled()` (`button` · `input` · `select` · `textarea`) and a `disabled` attribute present»
  replace-with: «focusability reads `disabled` by its presence on a form control alone (`ElementData::is_disabled`: `button` · `input` · `select` · `textarea`, PROVISIONAL), and no element carrying `inert` or under one is focusable (PROVISIONAL)»

- leaf: CLAUDE.md:22   class: stale
  leaf-text: «the password mask its one caller»
  source: .andromeda/architecture.md:134 «its two callers are blitz-paint's password mask, which holds no font context of its own, and blitz-dom's password click caret (`click_password_input`»
  replace-with: «blitz-paint's password mask and blitz-dom's password click caret (`click_password_input`) its two callers»

- leaf: CLAUDE.md:26   class: stale
  leaf-text: «a click inside a field holding a value still places the caret by the real characters' widths (`password_caret_click`: index 5 after eight narrow characters, 2 after eight wide), a known 0.1.0 limit whose fix is owned by a `.andromeda/residuals.md` line (target: next)»
  source: .andromeda/architecture.md:156 «a click inside a field holding a value places the caret by the bullets — blitz-dom's pointer path lays them out through `layout_as_text_input`, scrolled as paint scrolls them»
  replace-with: «a click inside a field holding a value places the caret by the bullets — blitz-dom's `click_password_input` lays them out through `layout_as_text_input` and moves the real editor to the character under the point, so no width of the real characters reaches the caret (`password_caret_click`: one click at one offset into two values of one length lands on one index, a double click selects the whole value; PROVISIONAL)»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «`selected` · `pressed` · `invalid` are taken from the accessibility node, a mixed toggle reading not pressed»
  source: .andromeda/architecture.md:160 «`checked` a native checkbox's or radio's own state, else the toggle of the five checkable roles (`aria-checked`), a mixed one reading false»
  replace-with: «`checked` is a native checkbox's or radio's own state, else the toggle of the five checkable roles (`aria-checked`), a mixed one reading false (PROVISIONAL); `selected` · `pressed` · `invalid` are taken from the accessibility node, `pressed` left out for those five roles, a mixed toggle reading not pressed»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «Two readings are measured off and said by the schema's own text, `covered`'s meaning: a hit reaches content clipped by `contain: paint`, so a control lying where such content extends can read `covered`, and a box clipped on one axis»
  source: .andromeda/architecture.md:160 «a box with no width or no height is hit at its centre on the element that holds it, so it can read `covered` though nothing covers it»
  replace-with: «Three readings are measured off and said by the schema's own text — `off-screen`'s meaning: a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows; `covered`'s: a box with no width or no height is hit at its centre on the element that holds it, so it can read `covered` though nothing covers it, and a box clipped on one axis»

- leaf: CLAUDE.md:45   class: stale
  leaf-text: «the engine honours no `inert`, owned by a `.andromeda/residuals.md` line (target: next) on the founder's F3 (2026-10-14), `stand_keyboard::the_engine_honours_no_inert` its pin;»
  source: .andromeda/test-plan.md:26 «`stand_keyboard` and `inert_subtree` pinning a truthy `inert` honoured, a falsy one cleared by the bridge»
  replace-with: «a truthy `inert` makes its subtree inert — skipped by Tab, passed through by a hit, out of the accessibility tree (PROVISIONAL), `stand_keyboard` and `inert_subtree` its pins — while HTML's focus fixup is not built (a node made inert keeps focus until focus moves), owned by a `.andromeda/residuals.md` line (target: next);»

- leaf: CLAUDE.md:76   class: omission
  leaf-text: «| Accessibility tree · its three state attributes · its merged name and description reads | `packages/blitz-dom/src/accessibility.rs` · checks `tests/blitz-tests/tests/accessibility_{states,label,description}.rs` |»
  source: .andromeda/architecture.md:135 «so what `build_accessibility_tree` returns changes for every document whose markup states one of the four»
  replace-with: «| Accessibility tree · its four state attributes · its merged name, description and state reads · `inert` | `packages/blitz-dom/src/accessibility.rs` · checks `tests/blitz-tests/tests/accessibility_{states,label,description,merged_states,active_descendant_unread}.rs` and `tests/blitz-tests/tests/inert_subtree.rs` |»

- leaf: CLAUDE.md:82   class: omission
  leaf-text: «checks `tests/blitz-tests/tests/{scrolled_box_client_rect,hit_clipped_at_scrolling_box,scrolled_box_absolute_position}.rs`»
  source: .andromeda/test-plan.md:26 «`transformed_client_rect` 4 — a translated box read moved by its translation, a scaled box scaled about its centre»
  replace-with: «checks `tests/blitz-tests/tests/{scrolled_box_client_rect,hit_clipped_at_scrolling_box,scrolled_box_absolute_position,transformed_client_rect}.rs`»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «the engine's builder writes them on the accessibility node from `aria-selected` · `aria-pressed` · `aria-invalid`, a third direct engine change»
  source: .andromeda/architecture.md:163 «the three are what the engine's builder writes from `aria-selected`, `aria-pressed` (or `aria-checked` on a checkable role) and `aria-invalid`»
  replace-with: «the engine's builder writes them on the accessibility node from `aria-selected` · `aria-pressed` (or `aria-checked` on a checkable role, PROVISIONAL) · `aria-invalid`, a third direct engine change»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «two readings are still off and said by the tool itself — a hit reaches content clipped by `contain: paint`, and a box clipped on one axis stops a hit at its whole padding box, so a target beside it along the axis it does not clip is refused `covered` though it shows — and the second position reader, `Node::absolute_position`, is measured and left as it reads, the `contain: paint` stop and `Node::absolute_position` each owned by a `.andromeda/residuals.md` line (target: next);»
  source: .andromeda/architecture.md:90 «the builder's toggle read per role, the hit's stop at a `contain: paint` box and `Node::absolute_position` less its containing blocks' scroll alone (above)»
  replace-with: «a hit also stops at a `contain: paint` box (PROVISIONAL); three readings are still off and said by the tool itself — a box clipped on one axis stops a hit at its whole padding box, so a target beside it along the axis it does not clip is refused `covered` though it shows, a box with no width or no height is hit at its centre on the element that holds it, so it can read `covered` though nothing covers it, and a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows — and the second position reader, `Node::absolute_position`, subtracts only its containing blocks' scroll offsets (PROVISIONAL);»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «while `BaseDocument::get_client_bounding_rect` reads it at its layout position, not where its transform moves it — a divergence from CSSOM View, measured and pinned, owned by a `.andromeda/residuals.md` line (target: next))»
  source: .andromeda/architecture.md:141 «The client rect under transforms (PROVISIONAL): `BaseDocument::get_client_bounding_rect` and `node_client_rects` read an element where its transforms move it»
  replace-with: «and `BaseDocument::get_client_bounding_rect` reads an element where its transforms move it (PROVISIONAL), so a driver `click` on a translated button is accepted and lands)»
