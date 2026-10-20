group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:12   class: stale
  leaf-text: «`accessibility_label` (10 tests: `aria-label`, `aria-labelledby` and its precedence, fallback and hidden targets)»
  source: .andromeda/a11y-plan.md:70 «`accessibility_label`, 13 tests (a `<label for>` and a wrapping `<label>` each naming a `select` and a `textarea`, a `<label>` naming no hidden `input`»
  replace-with: «`accessibility_label` (13 tests: a `<label for>` and a wrapping `<label>` each naming a `select` and a `textarea`, a `<label>` naming no hidden `input`; `aria-label`, `aria-labelledby` and its precedence, fallback and hidden targets)»

- leaf: .claude/docs/a11y-summary.md:14   class: omission
  leaf-text: «reading the focused node back per step, in both layout modes.»
  source: .andromeda/a11y-plan.md:220 «A select key and typeahead check runs in both layout modes: `select_interaction` (7 tests) drives a 14-row keys table and a 7-row typeahead table through the harness»
  replace-with: «reading the focused node back per step, in both layout modes; a select key and typeahead check runs in both layout modes — `select_interaction` (7 tests: a 14-row keys table and a 7-row typeahead table, one `input` and one `change` per change and none for no change, the typed text kept for one second of the document's clock, a disabled select taking no key and no click) and, through the driver, `stand_act_select` (5 tests: Tab `fx-before` → `fx-pick` → `fx-named` → `fx-after` → `fx-before`, the disabled `fx-off` skipped, an option chosen by a click then a key) — neither file in the a11y leg's six.»

- leaf: .claude/docs/a11y-summary.md:15   class: stale
  leaf-text: «(`Session::lint`, PROVISIONAL: in process only, no verb carries it)»
  source: .andromeda/a11y-plan.md:236 «escher-driver's in-process visual lint (`Session::lint`; in process only, no verb carries it)»
  replace-with: «(`Session::lint`, in process only, no verb carries it)»

- leaf: .claude/docs/a11y-summary.md:20   class: omission
  leaf-text: «`hidden` / `display:none` / `visibility:hidden` exclude the subtree;»
  source: .andromeda/a11y-plan.md:112 «an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes: it stays in the tree»
  replace-with: «`hidden` / `display:none` / `visibility:hidden` exclude the subtree — an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes: it stays in the tree, `hidden`, `inert` and `visibility: hidden` still excluding it;»

- leaf: .claude/docs/a11y-summary.md:21   class: omission
  leaf-text: «the builder reads `aria-selected`, `aria-pressed` and `aria-invalid` by value on every element»
  source: .andromeda/a11y-plan.md:46 «on every element — a select's option excepted, its `selected` its chosen state (`select::is_chosen`), true or false, with no `aria-selected` read»
  replace-with: «the builder reads `aria-selected`, `aria-pressed` and `aria-invalid` by value on every element — a select's option excepted, its `selected` its chosen state (`select::is_chosen`), true or false, with no `aria-selected` read, and its non-empty `label` attribute its `label` where it has no `aria-label` —»

- leaf: .claude/docs/a11y-summary.md:23   class: stale
  leaf-text: «and a click on any other element (a CRUD row, the list) still clears focus»
  source: .andromeda/a11y-plan.md:161 «a click on a `select` focuses it through `generate_focus_events` / `set_focus_to`, clearing its typeahead and the document's text selection»
  replace-with: «a click on a `select` focuses it, clearing its typeahead and the document's text selection (a disabled select stopped by the walk's gate first, so it neither focuses nor acts; an option is not clicked — a driver `click` on one is refused `covered`), and a click on any other element (a CRUD row, the list) still clears focus»

- leaf: .claude/docs/a11y-summary.md:23   class: stale
  leaf-text: «by the `FOCUS_WITHIN` element state (PROVISIONAL), while»
  source: .andromeda/a11y-plan.md:155 «`:focus-within` the focused element and its ancestors by the `FOCUS_WITHIN` element state, while `:focus-visible` never matches»
  replace-with: «by the `FOCUS_WITHIN` element state, while»

- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «clamped to [0, 1].»
  source: .andromeda/a11y-plan.md:170 «A focused single-choice `select` (no `multiple`) that is no disabled form control (`ElementData::is_disabled`) takes a pressed ArrowDown / ArrowUp to the next / previous enabled option»
  replace-with: «clamped to [0, 1]. Select: a focused single-choice `select` (no `multiple`) that is no disabled form control (`ElementData::is_disabled`) takes ArrowDown / ArrowUp to the next / previous enabled option and Home / End to the first / last enabled option (with none chosen, ArrowDown reads as Home and ArrowUp as End), and a character pressed with no Ctrl, Alt or Meta as typeahead — the first enabled option whose label starts with the text typed, compared in lower case, the text kept for one select until 1 s of the document's clock passes; every other key does nothing (Enter and Space open nothing, a released Space activates no select); each change marks the changed set and dispatches one `input` then one `change`, a key that changes nothing firing none; a `multiple` select takes no key, an `optgroup`'s options are not modelled and no popup is painted.»

- leaf: .claude/docs/a11y-summary.md:24   class: omission
  leaf-text: «`value` from the element (a range input's `value` its current value»
  source: .andromeda/a11y-plan.md:288 «the snapshot reads a `select`'s `value` as `BaseDocument::select_value` (the chosen option's value, nothing with none chosen) and its options as its children»
  replace-with: «`value` from the element (a `select`'s `value` `BaseDocument::select_value` — the chosen option's value, nothing with none chosen — its options its children, kept in the tree as `ListBoxOption` nodes, each `selected` from its chosen state and with its select's client rect as its `bounds`; a range input's `value` its current value»

- leaf: .claude/docs/a11y-summary.md:26   class: stale
  leaf-text: «`<label>` association (`for`-target or nested `<input>`)»
  source: .andromeda/a11y-plan.md:287 «A `<label>` names the labelable element it is bound to — its `for`-target by id, else its first nested labelable element»
  replace-with: «`<label>` association (`for`-target, else first nested labelable element — a `button`, an `input` whose `type` is not `hidden`, a `meter`, `output`, `progress`, `select` or `textarea`; a hidden input is named by no label)»

- leaf: .claude/rules/a11y.md:24   class: omission
  leaf-text: «`hidden`, `display: none`, `visibility: hidden` exclude a node and its children;»
  source: .andromeda/a11y-plan.md:112 «an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes: it stays in the tree»
  replace-with: «`hidden`, `display: none`, `visibility: hidden` exclude a node and its children — an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes: it stays in the tree, `hidden`, `inert` and `visibility: hidden` still excluding it;»

- leaf: .claude/rules/a11y.md:25   class: omission
  leaf-text: «State = ARIA state attributes read by value on every element, the toggle by role»
  source: .andromeda/a11y-plan.md:46 «on every element — a select's option excepted, its `selected` its chosen state (`select::is_chosen`), true or false, with no `aria-selected` read»
  replace-with: «State = ARIA state attributes read by value on every element — a select's option excepted, its `selected` its chosen state (`select::is_chosen`), true or false, with no `aria-selected` read, and its non-empty `label` attribute its `label` where it has no `aria-label` —, the toggle by role»

- leaf: .claude/rules/a11y.md:26   class: stale
  leaf-text: «a `<label>` is pushed onto its bound `<input>`'s `labelled_by` (`for`-target, else first nested input) after the whole tree is built»
  source: .andromeda/a11y-plan.md:287 «A `<label>` names the labelable element it is bound to — its `for`-target by id, else its first nested labelable element»
  replace-with: «a `<label>` is pushed onto its bound labelable element's `labelled_by` (`for`-target, else first nested labelable element — a `button`, an `input` whose `type` is not `hidden`, a `meter`, `output`, `progress`, `select` or `textarea`; a hidden input is named by no label) after the whole tree is built»

- leaf: .claude/rules/a11y.md:26   class: omission
  leaf-text: «the `aria-label` the fallback when the targets hold no text.»
  source: .andromeda/a11y-plan.md:288 «A `select` (role `ComboBox`, `ListBox` for `multiple`) keeps its `option` children in the tree as `ListBoxOption` nodes, under `display: none` too»
  replace-with: «the `aria-label` the fallback when the targets hold no text. A `select` (role `ComboBox`, `ListBox` for `multiple`) keeps its `option` children in the tree as `ListBoxOption` nodes, under `display: none` too, each `selected` true or false from its chosen state and named by its non-empty `label` attribute, else its text; the snapshot reads a `select`'s `value` as `BaseDocument::select_value` (the chosen option's value, nothing with none chosen) and its options as its children, each with its select's client rect as its `bounds`.»

- leaf: .claude/rules/a11y.md:33   class: stale
  leaf-text: «and a click on any other element — a CRUD row or the list — still clears focus»
  source: .andromeda/a11y-plan.md:161 «a click on a `select` focuses it through `generate_focus_events` / `set_focus_to`, clearing its typeahead and the document's text selection»
  replace-with: «a click on a `select` focuses it through `generate_focus_events` / `set_focus_to`, clearing its typeahead and the document's text selection (a disabled select stopped by the walk's gate first, so it neither focuses nor acts; an option is not clicked — a driver `click` on one is refused `covered`), and a click on any other element — a CRUD row or the list — still clears focus»

- leaf: .claude/rules/a11y.md:33   class: omission
  leaf-text: «and a main-button pointerdown sets it from the hit's fraction of the content box.»
  source: .andromeda/a11y-plan.md:170 «A focused single-choice `select` (no `multiple`) that is no disabled form control (`ElementData::is_disabled`) takes a pressed ArrowDown / ArrowUp to the next / previous enabled option»
  replace-with: «and a main-button pointerdown sets it from the hit's fraction of the content box. A focused single-choice `select` (no `multiple`) that is no disabled form control (`ElementData::is_disabled`) takes a pressed ArrowDown / ArrowUp to the next / previous enabled option and Home / End to the first / last enabled option (with none chosen, ArrowDown reads as Home and ArrowUp as End), and a character pressed with no Ctrl, Alt or Meta as typeahead — the first enabled option whose label starts with the text typed, compared in lower case, the text kept for one select until 1 s of the document's clock passes; every other key does nothing (Enter and Space open nothing, a released Space activates no select); each change marks the changed set, dispatches one `input` then one `change` carrying the select's value and asks for a redraw, a key that changes nothing firing none; a `multiple` select and an `optgroup`'s options are not modelled (a `multiple` select takes no key) and no popup is painted.»

- leaf: .claude/rules/a11y.md:34   class: stale
  leaf-text: «by the `FOCUS_WITHIN` element state (PROVISIONAL).»
  source: .andromeda/a11y-plan.md:155 «`:focus-within` the focused element and its ancestors by the `FOCUS_WITHIN` element state, while `:focus-visible` never matches»
  replace-with: «by the `FOCUS_WITHIN` element state.»

- leaf: .claude/rules/a11y.md:35   class: omission
  leaf-text: «so a control enabled after boot needs its place in `ENABLED_TAB_ORDER`.»
  source: .andromeda/a11y-plan.md:220 «A select key and typeahead check runs in both layout modes: `select_interaction` (7 tests) drives a 14-row keys table and a 7-row typeahead table through the harness»
  replace-with: «so a control enabled after boot needs its place in `ENABLED_TAB_ORDER`; a select key and typeahead check runs in both layout modes — `select_interaction` (7 tests: a 14-row keys table and a 7-row typeahead table, one `input` and one `change` per change and none for no change, `has_changes` asserted per row, the typed text kept for one second of the document's clock, a disabled select taking no key and no click, an app writing `selected` re-seeding the choice) and, through the driver, `stand_act_select` (5 tests: Tab `fx-before` → `fx-pick` → `fx-named` → `fx-after` → `fx-before`, the disabled `fx-off` skipped, equal before and after the choices, an option chosen by a click then a key) — neither file in the a11y leg's six.»

- leaf: .claude/rules/a11y.md:38   class: stale
  leaf-text: «(`Session::lint`, PROVISIONAL: in process only, no verb carries it)»
  source: .andromeda/a11y-plan.md:236 «escher-driver's in-process visual lint (`Session::lint`; in process only, no verb carries it)»
  replace-with: «(`Session::lint`, in process only, no verb carries it)»

- leaf: .claude/rules/a11y.md:42   class: stale
  leaf-text: «upstream's `accessibility_label` (10)»
  source: .andromeda/a11y-plan.md:70 «`accessibility_label`, 13 tests (a `<label for>` and a wrapping `<label>` each naming a `select` and a `textarea`, a `<label>` naming no hidden `input`»
  replace-with: «upstream's `accessibility_label` (13)»
