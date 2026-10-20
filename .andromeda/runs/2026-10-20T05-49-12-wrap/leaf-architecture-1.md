group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/gotchas.md:41   class: stale
  leaf-text: «by the `FOCUS_WITHIN` element state (PROVISIONAL);»
  source: .andromeda/architecture.md:85 «while `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state (packages/blitz-dom/src/stylo.rs:477-479; [DOM semantics])»
  replace-with: «by the `FOCUS_WITHIN` element state;»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «Focus keeps `:focus-within` on its ancestors and asks for a frame (PROVISIONAL; upstream #1226):»
  source: .andromeda/architecture.md:104 «- Focus keeps `:focus-within` on its ancestors and asks for a frame: `BaseDocument::set_focus_to` returns `false` for a node not in the document»
  replace-with: «Focus keeps `:focus-within` on its ancestors and asks for a frame (upstream #1226, a partial take now ratified):»

- leaf: .claude/docs/services/blitz-dom.md:19   class: stale
  leaf-text: «and the node that gained it — PROVISIONAL)»
  source: .andromeda/architecture.md:104 «The `FOCUS_WITHIN` writes go through `snapshot_node_and` and mark no changed-set entry, so a focus move still marks exactly the node that lost focus and the node that gained it»
  replace-with: «and the node that gained it)»

- leaf: .claude/docs/services/blitz-dom.md:19   class: omission
  leaf-text: «and nothing for a move that changes nothing — and NOT by node creation»
  source: .andromeda/architecture.md:294 «and `BaseDocument::apply_select_choice`, which marks a select where a key or typeahead changed its chosen option and nothing for a choice of the chosen option»
  replace-with: «and nothing for a move that changes nothing, and by `BaseDocument::apply_select_choice`, which marks a select where a key or typeahead changed its chosen option and nothing for a choice of the chosen option — and NOT by node creation»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «`build_accessibility_tree` (names from four sources — text children, a trimmed-non-empty `aria-label`, `<label>` association and, since the 2026-10-17 upstream sync, `aria-labelledby`,»
  source: .andromeda/architecture.md:162 «naming a node from five sources»
  replace-with: «`build_accessibility_tree` (names from five sources — text children, a trimmed-non-empty `aria-label`, `<label>` association (its `for`-target labelable element by id, else its first nested labelable element — HTML's labelable elements, `button`, an `input` whose `type` is not `hidden`, `meter`, `output`, `progress`, `select` and `textarea`, matched by the private `is_labelable` — pushed onto the bound element's `labelled_by`), a select's option's non-empty `label` attribute where it has no `aria-label` and, since the 2026-10-17 upstream sync, `aria-labelledby`,»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «four state attributes read by value on every element, the toggle by role —»
  source: .andromeda/architecture.md:162 «also reads state attributes on every element — a select's option excepted, whose `selected` is its chosen state, true or false, with no `aria-selected` read»
  replace-with: «four state attributes read by value on every element — a select's option excepted, whose `selected` is its chosen state, true or false, with no `aria-selected` read —, the toggle by role —»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «— a click on any other element still clearing focus;»
  source: .andromeda/architecture.md:86 «a click on a `select` focuses it through its own arm of the walk (below), and a click on any other element still clears focus»
  replace-with: «— a click on a `select` focusing it through its own arm of the walk and a click on any other element still clearing focus;»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «the Tab key asks for no redraw of its own.»
  source: .andromeda/architecture.md:92 «- The select model stands beside them, a direct engine change for every document, remote pages included: a focused single-choice `select` (no `multiple`) that is no disabled form control»
  replace-with: «the Tab key asks for no redraw of its own. The select model, a direct engine change for every document, remote pages included (flagged upstreamable, no upstream pull request open): a `select`'s choice is held as its options' `CHECKED` element state, so `:checked` matches the chosen option, and an option created with a `selected` attribute is seeded `CHECKED`; its rules sit in the crate-private `node::select` (no public type), cited to the WHATWG `select` element — a select's options are its `option` children in tree order (an `optgroup`'s not listed), an option's label its non-empty `label` attribute else its stripped and collapsed text, its value its `value` attribute when present else the text, and a single-choice select is settled by HTML's selectedness setting algorithm (the last chosen option, else the first option not disabled); the mutator records every select whose children an entry point touches and every select entering the document, a `selected` write sets that option chosen and its select's other options not, a cleared one sets it not chosen, and `flush` settles every recorded select, firing no event; a focused single-choice select that is no disabled form control takes a pressed `ArrowDown` / `ArrowUp` to the next / previous enabled option and `Home` / `End` to the first / last (with none chosen `ArrowDown` reads as `Home` and `ArrowUp` as `End`), and a character pressed with no Ctrl, Alt or Meta as typeahead — the first enabled option whose label starts with the buffered text, in lower case, the buffer (`select_typeahead`) starting afresh for another select or once the document's clock has moved 1 s (`TYPEAHEAD_TIMEOUT`) — every other key doing nothing, so Enter and Space open nothing and a released Space activates no select; a click on a select clears the typeahead and the text selection and focuses it, a disabled select neither focusing nor acting; every choice that changes the chosen option goes through `apply_select_choice` — the `CHECKED` write, the changed set, `input` then `change`, each carrying `select_value`, and a redraw — and a choice of the chosen option fires nothing; the public read-only `BaseDocument::select_value(&self, select: NodeId) -> Option<String>` is HTML's `select.value` (`None` with none chosen or for a node that is no select), its callers dioxus-native-dom's snapshot `value` and `apply_select_choice`; form submission sends every chosen option that is not disabled, by its value; the accessibility builder keeps an option whose parent is a `select` although it computes `display: none` (`hidden`, `inert` and `visibility: hidden` still exclude it); and the user-agent stylesheet gives `select` the border, padding and background of `input, textarea`, `display: inline-block`, and shows a single-choice select's chosen option by `select:not([multiple]) > option:checked { display: inline; }`, so a select lays out as a box holding its chosen option. Its limits: no popup is painted, an option is not clicked (a driver `click` on one is refused `covered`), and a `multiple` select or an `optgroup`'s options are not modelled.»

- leaf: .claude/docs/services/blitz-traits.md:16   class: omission
  leaf-text: «- `UiEvent`, `DomEvent`, `DomEventKind`, `EventState`, `BlitzPointerId`, per-event `cancelable()` / `bubbles()` tables, `BlitzImeEvent`.»
  source: .andromeda/architecture.md:163 «`DomEventKind::from_str` strips `on`, maps `composition` to `Ime` and `change` to `Change`»
  replace-with: «- `UiEvent`, `DomEvent`, `DomEventKind` (`from_str` strips `on`, maps `composition` to `Ime` and `change` to `Change`; `DomEventKind::Change`, after `Input`, and `DomEventData::Change(BlitzInputEvent)` — named `change`, not cancelable, bubbling, with no UI event mapped and no default action — dispatched by blitz-dom only after `input` on a select's choice, a checkbox, radio or text input firing no engine `change`), `EventState`, `BlitzPointerId`, per-event `cancelable()` / `bubbles()` tables, `BlitzImeEvent`.»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «`value` is the editor's text, else — for a range `input`»
  source: .andromeda/architecture.md:209 «`value` a `select`'s chosen option's value (`BaseDocument::select_value`, below), else the editor text, else — for a range `input`»
  replace-with: «`value` is a `select`'s chosen option's value (`BaseDocument::select_value`, the reader taking the document and the node, no mask applying), else the editor's text, else — for a range `input`»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: omission
  leaf-text: «`selected` is `is_selected()`,»
  source: .andromeda/architecture.md:209 «— a select's option's `selected` its chosen state, so a choice names the old and the new option with `selected` in `differs`»
  replace-with: «`selected` is `is_selected()` (a select's option's is its chosen state, which the builder writes with no `aria-selected` read, so a choice names the old and the new option with `selected` in `differs`),»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: omission
  leaf-text: «`bounds` (`get_client_bounding_rect` — viewport-relative, for every element:»
  source: .andromeda/architecture.md:214 «an option whose parent is a `select` reads its select's client rect as its `bounds` (the private `select_parent`), so a select's options are its children in the snapshot, named by their labels»
  replace-with: «`bounds` (`get_client_bounding_rect` — viewport-relative, for every element, an option whose parent is a `select` reading its select's client rect (the private `select_parent`), so a select's options are its children in the snapshot, named by their labels:»

- leaf: .claude/docs/services/dioxus-native-dom.md:22   class: omission
  leaf-text: «- DOM events route to the vdom via the nearest `data-dioxus-id` attribute (parsed as `usize`); listener registration sets a `"<rust func>"` placeholder.»
  source: .andromeda/architecture.md:214 «`Input | Change` reach the app as `NativeFormData { value }`, so an `onchange` handler fires»
  replace-with: «- DOM events route to the vdom via the nearest `data-dioxus-id` attribute (parsed as `usize`); listener registration sets a `"<rust func>"` placeholder; `Input | Change` reach the app as `NativeFormData { value }`, so a select's `onchange` handler fires — and since Dioxus writes a dynamic `selected` again on each re-render, a controlled app writes `selected` from its `input` event's value or a choice reverts.»
