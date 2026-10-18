group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/gotchas.md:41   class: retired
  leaf-text: «; `inert` is not implemented — a truthy `inert` takes no focusability away, so Tab reaches a button under `inert: true` as under `inert: false`.»
  source: .andromeda/architecture.md:101 «`inert` is honoured by its presence on the element or an ancestor element (PROVISIONAL): `Node::is_focussable` also reads false for an inert node through the crate-private `Node::is_inert`»
  replace-with: «.»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «stops at the padding box of a box that clips by `overflow`, the root element excepted; it still reaches content clipped by `contain: paint`, its widening owned by a `.andromeda/residuals.md` line (target: next);»
  source: .andromeda/architecture.md:137 «a node paint clips — either axis of its computed `overflow` not `visible`, or a computed `contain` that includes `paint` on a box paint applies it to»
  replace-with: «stops at the padding box of every box paint clips — either axis of its computed `overflow` not `visible`, or a computed `contain` that includes `paint` (that half PROVISIONAL) — the root element excepted;»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «`Node::absolute_position` (for a box that is itself scrolled, its position less its own scroll offset — measured and pinned, not fixed, while `get_client_bounding_rect` reads where the box stands — its fix owned by a `.andromeda/residuals.md` line (target: next) — though it reads a transformed box at its layout position, not where its transform moves it, a divergence from CSSOM View's `getBoundingClientRect` measured and pinned since 2026-10-14-stand-requirement-sweep, not fixed, owned by another such line)»
  source: .andromeda/architecture.md:137 «`Node::absolute_position`, the second position reader, adds the node's own location and hands the sum to the private `position_in_containing_block`»
  replace-with: «`Node::absolute_position` (only its containing blocks' scroll offsets subtracted, so a box that is itself scrolled reads where it stands, its callers — `scroll_into_view` and the test harness's `layout_rect`, `layout_rect_of` and `center_of` among them — moving with it (PROVISIONAL)), `get_client_bounding_rect` and `node_client_rects` (an element read where its transforms move it, as CSSOM View's `getBoundingClientRect` does (PROVISIONAL), while `visible_region` still places a scrolling box under a transform where it stands untransformed, owned by a `.andromeda/residuals.md` line (target: next))»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «since 2026-10-10-snapshot-states three states read by value on every element, whatever its role — `aria-selected` → `selected`, `aria-pressed` → `toggled`, `aria-invalid` → `invalid`, each a closed-set match on the trimmed lower-case value, an unrecognized `aria-invalid` reading `Invalid::True`, `aria-checked` unread, no log site and no panic path»
  source: .andromeda/architecture.md:135 «`aria-checked`: the same three values set `toggled` on a role in the private `is_checkable` set — `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` — and on no other»
  replace-with: «since 2026-10-10-snapshot-states four state attributes read by value on every element, the toggle by role — `aria-selected` → `selected`, `aria-pressed` → `toggled` on every role but the five checkable ones, `aria-checked` → `toggled` on those five alone (`CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio`), neither on a native `checkbox` or `radio` `input`, whose checked state writes no `toggled` either (the toggle read per role PROVISIONAL), `aria-invalid` → `invalid`, each a closed-set match on the trimmed lower-case value, an unrecognized `aria-invalid` reading `Invalid::True`, no log site and no panic path»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «its one caller is blitz-paint's password mask»
  source: .andromeda/architecture.md:134 «its two callers are blitz-paint's password mask, which holds no font context of its own, and blitz-dom's password click caret (`click_password_input`»
  replace-with: «its two callers are blitz-paint's password mask and blitz-dom's password click caret (`click_password_input`)»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «neither with a `disabled` attribute»
  source: .andromeda/architecture.md:86 «for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled»
  replace-with: «neither a disabled form control»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «and focusability reads `disabled` by presence (`ElementData::flush_is_focussable` reads `has_attr(disabled)`), so a bare, a `"false"` and a `"true"` `disabled` on any element remove focusability — a `<div disabled tabindex=0>` included, a known limit pinned by `stand_keyboard`, its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next).»
  source: .andromeda/architecture.md:97 «`disabled` is read one way, by its presence — bare, `"false"` or `"true"` — on a form control alone (PROVISIONAL): `ElementData::is_disabled` is `can_be_disabled()`»
  replace-with: «and `disabled` is read by presence on a form control alone (PROVISIONAL): the public `ElementData::is_disabled` — `can_be_disabled()` (`button` · `input` · `select` · `textarea`) and a `disabled` attribute present — is what focusability (`ElementData::flush_is_focussable`), the pointer gates and the activation keys read, so a bare, a `"false"` and a `"true"` `disabled` on a form control remove its focusability while a `<div disabled tabindex=0>` takes focus and a `<summary disabled>` or an `<a href disabled>` takes Enter, Space and a click. `inert` is honoured by its presence on an element or an ancestor (PROVISIONAL): `Node::is_focussable` reads false through the crate-private `Node::is_inert`, so the Tab walk, autofocus and `DioxusDocument::unkeyed_actionable` skip it, `hit_inner` answers no hit at it, and the accessibility builder leaves its subtree out of the tree; HTML's focus fixup is not built — a node holding focus when it becomes inert keeps it until focus moves — its fix owned by a `.andromeda/residuals.md` line (target: next).»

- leaf: .claude/docs/services/blitz-paint.md:18   class: stale
  leaf-text: «a click still places the caret by the real characters' widths (blitz-dom's pointer path, unchanged), so which bullet it lands on depends on them — measured since 2026-10-14-stand-requirement-sweep by `password_caret_click` (both layout modes, the bundled font at 20 px): a click 30 px into the content box places the caret at index 5 after eight narrow characters and at 2 after eight wide ones, a known 0.1.0 limit whose fix, placing a click's caret by the bullet layout, is owned by a `.andromeda/residuals.md` line (target: next).»
  source: .andromeda/architecture.md:156 «a click inside a field holding a value places the caret by the bullets — blitz-dom's pointer path lays them out through `layout_as_text_input`, scrolled as paint scrolls them»
  replace-with: «a click inside a field holding a value places the caret by the bullets — blitz-dom's pointer path (`click_password_input`) lays them out through `layout_as_text_input`, scrolled as paint scrolls them, and moves the real editor to the character under the point, so no width of the real characters reaches the caret: one click at one offset into two values of one length lands on one index, and a double click selects the whole value (PROVISIONAL; `password_caret_click`).»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «`checked` is the checkbox or radio state»
  source: .andromeda/architecture.md:163 «`checked` `checkbox_input_checked()` for a native checkbox or radio, else, for a node whose tree role is one of the five checkable roles»
  replace-with: «`checked` is a native checkbox's or radio's own state, else, for a node whose tree role is one of the five checkable roles, `toggled == Toggled::True` — `None` where the role states nothing, a mixed one `Some(false)` (PROVISIONAL)»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «(blitz-dom's builder writes them from `aria-selected`, `aria-pressed` and `aria-invalid`), so the snapshot takes them from the tree and never parses the element a second time: `selected` is `is_selected()`, `pressed` is `toggled()` with only `Toggled::True` reading `Some(true)`»
  source: .andromeda/architecture.md:163 «`pressed` its `toggled()` off the five checkable roles, with only `Toggled::True` reading `Some(true)` and both `False` and `Mixed` reading `Some(false)`»
  replace-with: «(blitz-dom's builder writes them from `aria-selected`, `aria-pressed` (or `aria-checked` on a checkable role) and `aria-invalid`), so the snapshot takes them from the tree and never parses the element a second time: `selected` is `is_selected()`, `pressed` is `toggled()` off the five checkable roles, with only `Toggled::True` reading `Some(true)`»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: omission
  leaf-text: «`bounds` (`get_client_bounding_rect` — viewport-relative, for every element:»
  source: .andromeda/architecture.md:163 «an element under a transform read where its transforms move it (PROVISIONAL; the Scrolling, selection, tree contract), so a driver `click` on a translated button is accepted and lands»
  replace-with: «`bounds` (`get_client_bounding_rect` — viewport-relative, for every element: an element under a transform read where its transforms move it (PROVISIONAL), so a driver `click` on a translated button is accepted and lands;»

- leaf: .claude/docs/services/dioxus-native-dom.md:23   class: retired
  leaf-text: «a framework-written `inert: false` is cleared — and the engine honours no `inert`, so neither a truthy nor a falsy one makes content inert)»
  source: .andromeda/architecture.md:163 «a framework-written `inert: false` is cleared and leaves content as it was, while a truthy `inert` makes its subtree inert — not focusable, not hit, out of the accessibility tree and the snapshot»
  replace-with: «a framework-written `inert: false` is cleared and leaves content as it was, while a truthy `inert` makes its subtree inert — not focusable, not hit, out of the accessibility tree and the snapshot)»

- leaf: .claude/docs/services/dioxus-native-dom.md:41   class: retired
  leaf-text: «the attribute half only — `stand_keyboard` pins that the engine honours neither a truthy nor a falsy `inert`)»
  source: .andromeda/architecture.md:290 «the attribute half only — a truthy `inert` honoured — `inert_subtree`, `stand_keyboard` — and a falsy one cleared by the bridge)»
  replace-with: «the attribute half only — a truthy `inert` honoured — `inert_subtree`, `stand_keyboard` — and a falsy one cleared by the bridge)»
