group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «and skips a node dropped since the last resolve»
  source: .andromeda/architecture.md:143 «answers nothing for the whole subtree of a transform whose determinant is 0 or not finite»
  replace-with: «and skips a node dropped since the last resolve and answers nothing for the whole subtree of a transform whose determinant is 0 or not finite»
- leaf: .claude/docs/services/blitz-dom.md:15   class: retired
  leaf-text: «while `visible_region` still places a scrolling box under a transform where it stands untransformed, owned by a `.andromeda/residuals.md` line (target: next)»
  source: .andromeda/architecture-detail/standard-contracts.md:59 «`BaseDocument::visible_region` keeps its signature and reads each clipping box through its transforms via the private `padding_box_in_view` over `transformed_bounds`»
  replace-with: «and `visible_region` reads each clipping box through its transforms via the private `padding_box_in_view` over `transformed_bounds` — exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show (a measured limit), `None` for a mapped box with a non-finite edge»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «a select's option's non-empty `label` attribute where it has no `aria-label` and,»
  source: .andromeda/architecture-detail/standard-contracts.md:26 «a select's option's non-empty `label` attribute where it has no `aria-label`, and an `optgroup`'s (`Role::Group`), named in the tree and unpainted»
  replace-with: «a select's option's non-empty `label` attribute where it has no `aria-label`, an `optgroup`'s on its `Role::Group`, named in the tree and unpainted, and,»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «neither on a native `checkbox` or `radio` `input`, whose checked state writes no `toggled` either»
  source: .andromeda/architecture-detail/standard-contracts.md:32 «`build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing»
  replace-with: «neither on a native `checkbox` or `radio` `input`, whose own `checkbox_input_checked()` writes `Toggled::True` or `False` under a checkable role and nothing under any other, an engine change for every document»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «flagged upstreamable; check `accessibility_states`)»
  source: .andromeda/architecture-detail/standard-contracts.md:31 «`build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build»
  replace-with: «flagged upstreamable; check `accessibility_states`; the tree's `focus` is written only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build, an engine change for every document)»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a click on a `select` focusing it through its own arm of the walk and a click on any other element still clearing focus»
  source: .andromeda/architecture-detail/established-decisions.md:68 «a click on a `multiple` select's row focuses that select through the walk's `option` arm (below), and a click on any other element still clears focus»
  replace-with: «a click on a `select` focusing it through its own arm of the walk, a click on a `multiple` select's row focusing that select through the walk's `option` arm and a click on any other element still clearing focus»
- leaf: .claude/docs/services/blitz-dom.md:15   class: retired
  leaf-text: «HTML's focus fixup is not built — a node holding focus when it becomes inert keeps it until focus moves — its fix owned by a `.andromeda/residuals.md` line (target: next).»
  source: .andromeda/architecture.md:74 «The focus fixup: a node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives it up at the mutator's flush through `BaseDocument::clear_focus`»
  replace-with: «HTML's focus fixup stands without its events: a node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives it up at `DocumentMutator::flush` through `BaseDocument::clear_focus`, firing no DOM `blur` or `focusout`, the fixup reading neither `disabled` nor `hidden`.»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «The select model, a direct engine change for every document, remote pages included (flagged upstreamable, no upstream pull request open)»
  source: .andromeda/architecture-detail/established-decisions.md:76 «Eight more ratified direct engine changes for every document stand beside those seven»
  replace-with: «Beside these, eight more ratified direct engine changes for every document — `visible_region` through transforms, a non-invertible transform's no hit, the focus fixup under `inert`, the tree's focus falling back to the window, a native control's `toggled` on a checkable role, the `multiple` select, `optgroup`, and `change` on a checkbox and a radio (`handle_click`'s arms dispatching `DomEventData::Change` after `input` with the same value, a checkbox always, a radio only where it was not checked before the click) — each flagged upstreamable and nothing sent, their merge surface a line owned by Epoch 3's upstream sync, upstream `0980cce5f`'s `hit_inner` hunk a textual conflict with the non-invertible answer. The select model, a direct engine change for every document, remote pages included (flagged upstreamable, no upstream pull request open)»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a `select`'s choice is held as its options' `CHECKED` element state»
  source: .andromeda/architecture.md:72 «a `select`'s choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state»
  replace-with: «a `select`'s choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` element state»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a select's options are its `option` children in tree order (an `optgroup`'s not listed)»
  source: .andromeda/architecture-detail/established-decisions.md:119 «a select's options are its `option` children and its `optgroup` children's, in tree order»
  replace-with: «a select's options are its `option` children and its `optgroup` children's, in tree order, an option's disabled reading the one rule `option_disabled`, its own attribute or its group's»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «the mutator records every select whose children an entry point touches and every select entering the document»
  source: .andromeda/architecture-detail/standard-contracts.md:19 «A write or removal of `multiple` through the mutator records the select (`record_select_of_multiple_write`), so a removal settles its set to one option»
  replace-with: «the mutator records every select whose children an entry point touches, every select entering the document and the select of a `multiple` write or removal (`record_select_of_multiple_write`), so a removal settles its set to one option»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a `selected` write sets that option chosen and its select's other options not»
  source: .andromeda/architecture-detail/established-decisions.md:119 «a `selected` write sets that option chosen and, on a single-choice select, its select's other options not, a `multiple` select's others left alone»
  replace-with: «a `selected` write sets that option chosen and, on a single-choice select, its select's other options not, a `multiple` select's others left alone»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «every other key doing nothing, so Enter and Space open nothing and a released Space activates no select»
  source: .andromeda/architecture-detail/established-decisions.md:80 «A `multiple` select's keys, through `press_multiple_select_key`: `ArrowDown`, `ArrowUp`, `Home` and `End` alone choose that row alone, with Shift span from the anchor»
  replace-with: «every other key doing nothing, so Enter and Space open nothing and a released Space activates no select; a focused `multiple` select takes its keys through `press_multiple_select_key`: `ArrowDown`, `ArrowUp`, `Home` and `End` alone choose that row alone, with Shift span from the anchor, and with Ctrl or Meta move the current row and change nothing, a Space flips the current row with no typed text standing and joins the text otherwise, a character with no Ctrl, Alt or Meta chooses its match alone, and disabled rows are never chosen»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «a click on a select clears the typeahead and the text selection and focuses it, a disabled select neither focusing nor acting»
  source: .andromeda/architecture-detail/established-decisions.md:81 «A `multiple` select's pointer: `click_select_row`, behind an `option` arm of `handle_click`, stops at a disabled select, focuses the select and clears the typeahead and the text selection»
  replace-with: «a click on a select clears the typeahead and the text selection and focuses it, a disabled select neither focusing nor acting; on a `multiple` select's row `click_select_row`, behind an `option` arm of `handle_click`, stops at a disabled select, focuses the select and clears the typeahead and the text selection, a plain click choosing the row alone, Ctrl or Meta flipping it, Shift spanning, Shift with no cursor yet spanning from the last chosen row, and a disabled row changing nothing»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «every choice that changes the chosen option goes through `apply_select_choice`»
  source: .andromeda/architecture-detail/established-decisions.md:78 «Every choice that changes the chosen option or a `multiple` select's chosen set goes through the one path `apply_select_choice`, which takes the chosen set»
  replace-with: «every choice that changes the chosen option or a `multiple` select's chosen set goes through `apply_select_choice`, which takes the chosen set»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «and a choice of the chosen option fires nothing»
  source: .andromeda/architecture.md:62 «a single select's choice of its chosen option fires nothing»
  replace-with: «and a single select's choice of its chosen option fires nothing»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «is HTML's `select.value` (`None` with none chosen or for a node that is no select), its callers dioxus-native-dom's snapshot `value` and `apply_select_choice`»
  source: .andromeda/architecture-detail/standard-contracts.md:17 «HTML's `select.value` — the chosen option's value, a `multiple` select's first chosen option in tree order, `None` with none chosen or for a node that is no select»
  replace-with: «is HTML's `select.value` — the chosen option's value, a `multiple` select's first chosen option in tree order (`None` with none chosen or for a node that is no select), its callers dioxus-native-dom's snapshot `value` and `apply_select_choice` — beside the public read-only `select_values(&self, select: NodeId) -> Vec<String>` (the chosen set) and `select_of(&self, node: NodeId) -> Option<NodeId>` (an option's or group's select) and the public `clear_select_typeahead(&mut self)`, its one non-test caller escher-driver's `focus_multiple_select`»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «the accessibility builder keeps an option whose parent is a `select` although it computes `display: none`»
  source: .andromeda/architecture-detail/established-decisions.md:120 «The accessibility builder keeps an option whose parent is a `select`, or an `optgroup` under one, although it computes `display: none`»
  replace-with: «the accessibility builder keeps an option whose parent is a `select`, or an `optgroup` under one, although it computes `display: none`»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «, so a select lays out as a box holding its chosen option.»
  source: .andromeda/architecture-detail/established-decisions.md:120 «so a single-choice select lays out as a box holding its chosen option»
  replace-with: «, so a single-choice select lays out as a box holding its chosen option; the default sheet lays out `optgroup` and `multiple` rows — under a single select a group `display: contents` and its chosen option `display: inline`, under a `multiple` select rows and groups `display: block` — and holds the engine-default chosen-row pair `background-color: #c8c8c8; color: #000000` (12.56:1, hardcoded since the engine holds no tokens).»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «Its limits: no popup is painted, an option is not clicked (a driver `click` on one is refused `covered`), and a `multiple` select or an `optgroup`'s options are not modelled.»
  source: .andromeda/architecture.md:62 «a `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge; no popup is painted, Enter and Space open nothing on a single select»
  replace-with: «Its limits: no popup is painted (owner: the route entry Select popup), Enter and Space open nothing on a single select, an option of a closed single select, under a group too, is not clicked (a driver `click` on one is refused `covered`), `size` is unread and Ctrl+A is not modelled; a `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.»
- leaf: .claude/docs/services/blitz-dom.md:19   class: stale
  leaf-text: «by `BaseDocument::apply_select_choice`, which marks a select where a key or typeahead changed its chosen option and nothing»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:40 «marks a select where a key or typeahead or, on a `multiple` select, a click on a row (packages/blitz-dom/src/events/pointer.rs:1013-1054) changed its chosen option or set»
  replace-with: «by `BaseDocument::apply_select_choice`, which marks a select where a key or typeahead or, on a `multiple` select, a click on a row changed its chosen option or set and nothing»
- leaf: .claude/docs/services/blitz-dom.md:21   class: stale
  leaf-text: «(`pseudo_element_flow_classification`, which does not assert both layout modes)»
  source: .andromeda/architecture-detail/established-decisions.md:102 «its test file asserting both layout modes, every test running `for incremental in [false, true]`»
  replace-with: «(`pseudo_element_flow_classification`, each of whose three tests runs both layout modes)»
- leaf: .claude/docs/services/blitz-shell.md:17   class: omission
  leaf-text: «held by the windowed `seven_guis_native` smoke boot and `focus_redraw`»
  source: .andromeda/architecture-detail/established-decisions.md:147 «held by the windowed `seven_guis_native` smoke boot, `focus_redraw` and blitz-shell's unit test `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`»
  replace-with: «held by the windowed `seven_guis_native` smoke boot, `focus_redraw` and blitz-shell's unit test `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read»
- leaf: .claude/docs/services/blitz-shell.md:31   class: stale
  leaf-text: «- No in-crate tests; headless behaviour is tested through `blitz-test-harness`.»
  source: .andromeda/architecture-detail/existing-scopes.md:64 «event (holding the crate's one `#[cfg(test)]` module, one unit test, a redraw request reaching the receiver and waking the loop once: packages/blitz-shell/src/event.rs:128-172)»
  replace-with: «- One in-crate unit test, in `src/event.rs`'s one `#[cfg(test)]` module: a redraw request reaching the receiver and waking the loop once; headless behaviour is tested through `blitz-test-harness`.»
- leaf: .claude/docs/services/blitz-traits.md:16   class: stale
  leaf-text: «dispatched by blitz-dom only after `input` on a select's choice, a checkbox, radio or text input firing no engine `change`»
  source: .andromeda/architecture-detail/standard-contracts.md:40 «are dispatched by the engine after `input` on a select's choice and on a checkbox's or radio's click (below), a text or range input firing no engine `change`»
  replace-with: «dispatched by blitz-dom after `input` on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, a text or range input firing no engine `change`»
- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «an option whose parent is a `select` reading its select's client rect (the private `select_parent`), so a select's options are its children in the snapshot, named by their labels»
  source: .andromeda/architecture-detail/standard-contracts.md:279 «an option of a closed select, under an `optgroup` too, reads its select's client rect as its `bounds` (the public `closed_select_of` and `BaseDocument::select_of`)»
  replace-with: «an option of a closed select, under an `optgroup` too, reading its select's client rect (the public, re-exported `closed_select_of`, called by the snapshot's builder and escher-driver's `lint.rs`, over the public, re-exported `is_multiple_select`, also read by `execute.rs` `locate`, and `BaseDocument::select_of`), so a select's options are its children in the snapshot, a group's its group's, named by their labels, a `multiple` select's rows and groups reading client rects of their own»
- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «`value` is a `select`'s chosen option's value (`BaseDocument::select_value`»
  source: .andromeda/architecture-detail/standard-contracts.md:260 «`value` a `select`'s chosen option's value, a `multiple` select's first chosen option in tree order (`BaseDocument::select_value`, below)»
  replace-with: «`value` is a `select`'s chosen option's value, a `multiple` select's first chosen option in tree order (`BaseDocument::select_value`»
- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: omission
  leaf-text: «`aria-pressed` (or `aria-checked` on a checkable role) and `aria-invalid`), so the snapshot takes them»
  source: .andromeda/architecture-detail/standard-contracts.md:261 «and on a native checkable control it reads neither `aria-pressed` nor `aria-checked`, writing `toggled` from `checkbox_input_checked()` under a checkable role and nothing under any other»
  replace-with: «`aria-pressed` (or `aria-checked` on a checkable role) and `aria-invalid`, and on a native checkable control from neither, writing `toggled` from `checkbox_input_checked()` under a checkable role and nothing under any other, so `pressed` never reads on one), so the snapshot takes them»
- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «so a choice names the old and the new option with `selected` in `differs`»
  source: .andromeda/architecture-detail/standard-contracts.md:262 «so a single select's choice names the old and the new option with `selected` in `differs` and a `multiple` select's names each row whose chosen state moved»
  replace-with: «so a single select's choice names the old and the new option with `selected` in `differs` and a `multiple` select's names each row whose chosen state moved»
- leaf: .claude/docs/services/dioxus-native-dom.md:22   class: omission
  leaf-text: «`Input | Change` reach the app as `NativeFormData { value }`, so a select's `onchange` handler fires»
  source: .andromeda/architecture-detail/standard-contracts.md:281 «`multiple_select_values` in `dioxus_document.rs` fills `values` on `Input` and `Change` with one `(name, FormValue::Text(value))` per chosen option of a `multiple` select»
  replace-with: «`Input | Change` reach the app as `NativeFormData { value }`, `multiple_select_values` filling `values` with one `(name, FormValue::Text(value))` per chosen option of a `multiple` select so `evt.values()` reads the set while `value` stays `select_value`, so a select's `onchange` handler fires»
- leaf: .claude/docs/services/dioxus-native-dom.md:31   class: omission
  leaf-text: «- `element_to_node_id` unwraps; `NodeHandle::node` panics if the node is gone.»
  source: .andromeda/architecture-detail/established-decisions.md:171 «such a task that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in it panics at `events.rs:162`»
  replace-with: «- `element_to_node_id` unwraps; `NodeHandle::node` panics if the node is gone. A task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in it panics at `events.rs:162` (`RefCell already borrowed`) — a measured hazard any app meets, no engine change; Cells calls its handles synchronously.»
