proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: "## Standard Contracts"
    change: "In the `select_value` rule, read: HTML's `select.value` — the chosen option's value, a `multiple` select's first chosen option in tree order, `None` with none chosen or for a node that is no select; and say of the crate-private writers that `set_select_choice` takes the chosen set."
    sidecar: "standard-contracts: `select_value` reads a `multiple` select's first chosen option; `set_select_choice` takes a set (2026-10-31-engine-debts)."
    rationale: "The report's Symbols / APIs under its F6 state both, and `multiple-select` lists `standard-contracts.md:17` as changed."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: "## Standard Contracts"
    change: "Add a bullet: - Since 2026-10-31-engine-debts `BaseDocument::select_values(&self, select: NodeId) -> Vec<String>` and `BaseDocument::select_of(&self, node: NodeId) -> Option<NodeId>`, public and read-only; public `BaseDocument::clear_select_typeahead(&mut self)`, its one non-test caller `execute.rs` `focus_multiple_select`; and `dioxus_native_dom::closed_select_of`, called by the snapshot's builder and `lint.rs`, and `is_multiple_select`, called by `closed_select_of` and `execute.rs` `locate`, public and re-exported because escher-driver has no blitz-dom dependency to name `local_name!` with, `is_masked_input` the precedent (packages/blitz-dom/src/document.rs:736-744; packages/blitz-dom/src/document.rs:746-750; packages/blitz-dom/src/document.rs:752-755; packages/dioxus-native-dom/src/snapshot.rs:242-248; packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/lib.rs:35-38)."
    sidecar: "standard-contracts: registers the public select readers and the two re-exported snapshot readers (2026-10-31-engine-debts)."
    rationale: "The report's sole-caller facts and Deviations 4 name these public items, and D-arch-resources wants each new public surface registered."
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: "## Standard Contracts"
    change: "Add a bullet: - Since 2026-10-31-engine-debts a `multiple` write or removal through the mutator records the select (`record_select_of_multiple_write`), so removal settles it to one, and `set_option_selectedness` leaves a `multiple` select's other options alone (packages/blitz-dom/src/mutator.rs:1262-1271)."
    sidecar: "standard-contracts: states the mutator's record of a `multiple` write and its settling on removal (2026-10-31-engine-debts)."
    rationale: "The report's `mutator.rs` line under its F6 states it; the entity beside it holds the choice's writers."
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 135
    section: "## Standard Contracts"
    change: "End the line: `BaseDocument::select_value` reads a select's value, a `multiple` select's first chosen option, `select_values` its chosen set and `select_of` an option's or group's select, each public and read-only, the choice's writers crate-private."
    sidecar: "architecture body: the document-core line names the new public select readers (2026-10-31-engine-debts)."
    rationale: "`multiple-select` lists `architecture.md:135` as changed; the public `clear_select_typeahead` is why the wording narrows to the choice's writers."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 24
    section: "## Standard Contracts"
    change: "After the source `a select's option's non-empty label attribute where it has no aria-label`, add: and an `optgroup`, `Role::Group`, named by its non-empty `label` attribute, unpainted; the count of five sources stands, the label attribute being one source."
    sidecar: "standard-contracts: the tree names an `optgroup` from its `label` (2026-10-31-engine-debts)."
    rationale: "The report's F7 states `optgroup` → `Role::Group`, its non-empty `label` its name; the fuller `optgroup` fact is F12, established-decisions, and this names both sides."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 26
    section: "## Standard Contracts"
    change: "Replace `and the builder writes no toggled from that native checked state either ... (target: next)` with: and, since 2026-10-31-engine-debts, the builder writes `toggled` from that checked state under a checkable role (below), so the tree states it as the snapshot's `checked` reads the element; and replace `nothing else in the file writes toggled` with: only these reads and that native write set `toggled`."
    sidecar: "standard-contracts: the measured absence of native `toggled` ends, and its residual owner with it (2026-10-31-engine-debts)."
    rationale: "The report's F5 writes `Toggled` on checkable roles from `checkbox_input_checked()`, contradicting the stated absence, and `native-toggled` lists 9 of its 10 hits as changed."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 27
    section: "## Standard Contracts"
    change: "In the states content class, after `on each element node whose markup states them`, add: , a native control's `toggled` and a `multiple` select's multiselectable beside them."
    sidecar: "standard-contracts: the adapter crossing's states class gains native `toggled` and multiselectable (2026-10-31-engine-debts)."
    rationale: "The report's F5 and its `set_multiselectable` line under F6 write states no markup attribute states, so the sentence on what the adapter is handed moves; the values stay closed, and each write is ratified by the founder's word."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 28
    section: "## Standard Contracts"
    change: "Add a bullet: - The tree's focus (ratified by the founder, 2026-10-31, «Откат на окно (Рекомендую)»): `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build; an engine change for every document (packages/blitz-dom/src/accessibility.rs:104-110; packages/blitz-dom/src/accessibility.rs:122)."
    sidecar: "standard-contracts: states the tree's window fallback for focus (2026-10-31-engine-debts)."
    rationale: "The report's Symbols / APIs state the rule, and the entity beside it does not state the fallback."
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 28
    section: "## Standard Contracts"
    change: "Add a bullet: - A native control's state in the tree (ratified by the founder, 2026-10-31, «Писать на checkable-ролях (Рекомендую)»): `build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never begins to read on it, and neither author attribute is read on a native control; an engine change for every document (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427)."
    sidecar: "standard-contracts: states the native `toggled` write and its role limit (2026-10-31-engine-debts)."
    rationale: "The report's Symbols / APIs state the rule, which replaces the measured absence the entity held."
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 137
    section: "## Standard Contracts"
    change: "Read: naming nodes from text children, `aria-labelledby`, `aria-label`, a bound `<label>` and option and group labels, writing state attributes each read once and a native control's checked state on a checkable role."
    sidecar: "architecture body: the mutation line carries group labels and native checked state (2026-10-31-engine-debts)."
    rationale: "The broad line the entity details moves with the report's F5 and F7."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 139
    section: "## Standard Contracts"
    change: "Read: A measured limit: the platform-tree refresh's windowed witness is still owed, the refresh held by the document-level tests and both feature builds of blitz-shell."
    sidecar: "architecture body: the native checked-state limit and its residual owner end (2026-10-31-engine-debts)."
    rationale: "The report's F5 ends the limit; the platform-tree witness stays owed, the task's Touches reading it as standing."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 34
    section: "## Standard Contracts"
    change: "After `focus change dispatches blur, focusout, focus, focusin`, add: , and the inert focus fixup none ([DOM semantics])."
    sidecar: "standard-contracts: the focus-change dispatch carves out the inert fixup, which fires no blur (2026-10-31-engine-debts)."
    rationale: "The report's F3 has `DocumentMutator::flush` clear focus with no DOM `blur` or `focusout`; the rule itself is established-decisions' F3, and this names both sides."
    basis: packages/blitz-dom/src/mutator.rs:1051-1063
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 36
    section: "## Standard Contracts"
    change: "Read: are dispatched by the engine after `input` on a select's choice and, since 2026-10-31-engine-debts, a checkbox's or radio's click, a text or range input firing no engine `change` ([Scrolling and input])."
    sidecar: "standard-contracts: `change` is no longer a select's alone (2026-10-31-engine-debts)."
    rationale: "`change-event` and `text-range-limit` each list `standard-contracts.md:36` as changed, and `text_range_no_change_event` measures the text and range half."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 37
    section: "## Standard Contracts"
    change: "Add a bullet: - `change` on a checkable control (ratified by the founder, 2026-10-31, «Шлёт движок (Рекомендую)»): `handle_click`'s checkbox and radio arms dispatch `DomEventData::Change` after `input` with the same value, a checkbox always, a radio only where it was not checked before the click; blitz-vibey-script's runtime no longer synthesizes one, so a page's script hears one `change`, from the engine; an engine change for every document (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836; tests/blitz-tests/tests/checkable_change_event.rs:68-98; tests/blitz-tests/tests/checkable_change_event.rs:100-119)."
    sidecar: "standard-contracts: states the engine's `change` on a checkbox and a radio (2026-10-31-engine-debts)."
    rationale: "The report's Symbols / APIs state the rule, and its Retired bullet names the runtime's synthesis."
    fact: F15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 141
    section: "## Standard Contracts"
    change: "End the line: the engine dispatches `change` after `input` on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and none on a text or range input."
    sidecar: "architecture body: the events line reads `change` on a checkbox and a radio (2026-10-31-engine-debts)."
    rationale: "The broad line the entity details moves with the same fact the report states."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 44
    section: "## Standard Contracts"
    change: "After `each clipping box is placed where the bounds reader puts it — the box's own position, with no add-back of the box's own scroll offset since 2026-10-10-scrolling-box-bounds-and-hit, the region reading as it read before`, add: , and since 2026-10-31-engine-debts through its transforms where its chain carries one (below)."
    sidecar: "standard-contracts: `visible_region`'s placement clause gains the transforms (2026-10-31-engine-debts)."
    rationale: "The report's disposition of `standard-contracts.md:44` holds for the reader's naming, which this edit leaves; the placement clause is what its F1 moves, so it is read on its own against the changed `scrolling.rs`."
    basis: packages/blitz-dom/src/scrolling.rs:790-812
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 46
    section: "## Standard Contracts"
    change: "After `BaseDocument::get_client_bounding_rect reads that box where its transform moves it, as CSSOM View's getBoundingClientRect does (the client rect under transforms, below)`, add: , and since 2026-10-31-engine-debts `visible_region` reads it through its transform too."
    sidecar: "standard-contracts: the transformed-and-scrolled reading gains `visible_region` (2026-10-31-engine-debts)."
    rationale: "`visible-region` lists `standard-contracts.md:46` as changed; the one-axis sentence on the same line stands, the report moving no axis reading."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 53
    section: "## Standard Contracts"
    change: "Read `the private transformed_bounds` as `transformed_bounds`, `pub(crate)` since 2026-10-31-engine-debts for `visible_region`; and take `visible_region` out of the list `physical_unrounded_geometry, offset_rect ..., inline_fragment_rects and visible_region are not edited`."
    sidecar: "standard-contracts: `transformed_bounds` is `pub(crate)` and `visible_region` leaves the not-edited list (2026-10-31-engine-debts)."
    rationale: "`new-readers` lists `standard-contracts.md:53` as changed, and the report's F1 edits `visible_region` over `transformed_bounds`."
    basis: packages/blitz-dom/src/document.rs:2574-2578
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 53
    section: "## Standard Contracts"
    change: "Replace the sentence `A measured limit: visible_region places a scrolling box under a transform where it stands untransformed ... (target: next).` with: Since 2026-10-31-engine-debts a target inside a translated scrolling box reads inside its visible region, the pin renamed and inverted (tests/blitz-tests/tests/transformed_client_rect.rs:183-216), and the limit that stands is the rotation's, stated in `off-screen`'s meaning (§Driver session)."
    sidecar: "standard-contracts: the transformed-box limit, its residual owner and the old pin name retire (2026-10-31-engine-debts)."
    rationale: "The report's Retired bullet names the pin name and `off-screen`'s transformed-box clause; its F1 states the rotation's reading and the `off-screen` text."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 53
    section: "## Standard Contracts"
    change: "Add a bullet: - `visible_region` under transforms (ratified by the founder, 2026-10-31, «Починить (Рекомендую)»): `BaseDocument::visible_region` keeps its signature and reads each clipping box through its transforms via the private `padding_box_in_view` over `transformed_bounds`: exact under a translate and a scale; under a rotation the rotated padding box's bounding box, so it holds corners the box does not show; `None` for a mapped box with a non-finite edge; as before with no transform on the chain. Its callers `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region` are unchanged; an engine change for every document (packages/blitz-dom/src/scrolling.rs:790-812; tests/blitz-tests/tests/visible_region_transformed.rs:201-216; tests/blitz-tests/tests/visible_region_transformed.rs:221-244; tests/blitz-tests/tests/visible_region_transformed.rs:246-276)."
    sidecar: "standard-contracts: states `visible_region` under transforms (2026-10-31-engine-debts)."
    rationale: "The report's Symbols / APIs state the rule and its readings; Deviations 5 name the helper."
    fact: F1
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 145
    section: "## Standard Contracts"
    change: "Read: `get_client_bounding_rect` reads an element where its transforms move it, and `visible_region` reads each clipping box through its transforms, under a rotation the rotated padding box's bounding box; `BaseDocument::text_extent` is a read-only reader that mutates nothing."
    sidecar: "architecture body: the transformed-box limit retires from the scrolling line (2026-10-31-engine-debts)."
    rationale: "`visible-region` lists `architecture.md:145` as changed; `architecture.md:143` stands, naming the reader only."
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 55
    section: "## Standard Contracts"
    change: "Add a bullet: - A non-invertible transform takes no hit (ratified by the founder, 2026-10-31, «Починить (Рекомендую)»): `Node::hit_inner` answers `None` for the whole subtree under a transform whose determinant is 0 or not finite; an engine change for every document (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56; tests/blitz-tests/tests/hit_non_invertible_transform.rs:58-71)."
    sidecar: "standard-contracts: states `hit_inner`'s answer under a non-invertible transform (2026-10-31-engine-debts)."
    rationale: "The report's Symbols / APIs state the rule, and `non-invertible`'s control is unfired, so no master holds it yet."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 76
    section: "## Standard Contracts"
    change: "Add a bullet: - Since 2026-10-31-engine-debts `node::select` also holds `SelectCursor`, `RowGesture`, `Gestured`, `first_chosen_option`, `last_chosen`, `spanned`, `gestured`, `owner_select` and `option_disabled`, still no public type, and `stepped` takes the row it steps from; `apply_select_choice` takes a set and stays the one write path, and `document.rs` holds a `pub(crate)` field `select_cursor` (packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89; packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/node/select.rs:200-205; packages/blitz-dom/src/node/select.rs:269-272; packages/blitz-dom/src/node/select.rs:306-315; packages/blitz-dom/src/node/select.rs:317-358)."
    sidecar: "standard-contracts: `node::select` gains the row types and rules; the select's one write path takes a set (2026-10-31-engine-debts)."
    rationale: "The report's rule types and rules under its F6 and F7 state them; `architecture.md:151`'s no public type stands."
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 78
    section: "## Standard Contracts"
    change: "Read `kept in sync by DocumentMutator::set_attribute's class arm and by remove_attribute` as `kept in sync by DocumentMutator::set_attribute's class arm and by clear_attribute`."
    sidecar: "standard-contracts: `remove_attribute` reads `clear_attribute`, CARRY 3 (2026-10-31-engine-debts)."
    rationale: "The report's Expected amendments carry CARRY 3: `remove-attr` has 1 hit, and `mutator.rs` holds `clear_attribute`."
note: "Standard-contracts sites the report moves that are not in this group's listing, so proposed nowhere here: `standard-contracts.md:169`, `:171`, `:209`, `:249`, `:260`, `:267`, `:298`; F10, F11, F16, F17, F18 and F20 have no picked carrier and go to the new-detail pass; F3 and F12 are established-decisions', and the script runtime's `change` is also contract-javascript's."
