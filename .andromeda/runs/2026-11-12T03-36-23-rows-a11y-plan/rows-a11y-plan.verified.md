@row_add n=1 row=decision/a11y-tier-0 area=1-a11y-scope-summary rec=fct-r5pehooz@1
name: Accessibility tier 0
rule: The accessibility tier is 0, covering WCAG success criteria 2.1.1, 1.4.3 and 2.4.3.
@row_check n=1 row=decision/a11y-tier-0 sha=4e13c677f133 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: Accessibility reach · in docs/capabilities.md (a document statement of the target, no code sets a tier)
why: not a code claim
@row_add n=2 row=absence/assistive-technology-reach area=1-a11y-scope-summary rec=fct-kbjv4usk@3
capability: A platform accessibility adapter in any stand binary, so no active assistive technology is given a tree
@row_check n=2 row=absence/assistive-technology-reach sha=c6c2cc2e7b75 v=holds judge=sonnet head=edd0cfaea7f0
searched: Adapter|accesskit_xplat|blitz-shell/accessibility|dioxus-native/accessibility · in examples/seven_guis
searched: dioxus-native/accessibility|blitz-shell/accessibility · in packages/escher-driver
searched: accessibility · in examples/seven_guis/Cargo.toml (its dioxus-native names only features ["prelude"], ["system-fonts", "woff"])
@row_add n=3 row=decision/accessibility-tree-through-document-method area=2-a11y-strategy rec=fct-zyrpeqlm@3
name: Accessibility tree built through the document's own method
rule: The accessibility tree is built from the document on the initial tree request and rebuilt when a poll takes a non-empty changed set, both paths going through the document trait's accessibility-tree method.
why: so a wrapper's override, such as DioxusDocument's stable ids, reaches the tree
@row_check n=3 row=decision/accessibility-tree-through-document-method sha=97964e36d10b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/application.rs:82
quote: window.build_accessibility_tree();
at: packages/blitz-shell/src/window.rs:379-380
quote: if !changed_nodes.is_empty() { self.accessibility.update_tree(&*self.doc);
at: packages/blitz-shell/src/accessibility.rs:45
quote: self.adapter.update_if_active(|| doc.accessibility_tree());
@row_add n=4 row=contract/accessibility-tree-states/aria-selected area=2-a11y-strategy rec=fct-cixbakze@3,fct-v47qtadu@1
boundary: Accessibility tree states
item: aria-selected
promise: Trimmed and matched in lower case, aria-selected writes the node's selected state.
@row_check n=4 row=contract/accessibility-tree-states/aria-selected sha=16b767fd968e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:189-192
quote: let selected = element_data.attr(local_name!("aria-selected")); match selected.map(str::trim_ascii) { Some("true") => builder.set_selected(true), Some("false") => builder.set_selected(false),
@row_add n=5 row=contract/accessibility-tree-states/aria-pressed area=2-a11y-strategy rec=fct-cixbakze@3,fct-v47qtadu@1
boundary: Accessibility tree states
item: aria-pressed
promise: Trimmed and matched in lower case, aria-pressed writes the node's toggled state on every role but the five checkable roles.
@row_check n=5 row=contract/accessibility-tree-states/aria-pressed sha=bb3aa008c797 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:214-219
quote: let toggled = if is_checkable(role) { element_data.attr(local_name!("aria-checked")) } else { element_data.attr(local_name!("aria-pressed")) }; match toggled.map(str::trim_ascii) {
@row_add n=6 row=contract/accessibility-tree-states/aria-checked area=2-a11y-strategy rec=fct-cixbakze@3,fct-v47qtadu@1
boundary: Accessibility tree states
item: aria-checked
promise: Trimmed and matched in lower case, aria-checked writes the node's toggled state on the five checkable roles alone.
@row_check n=6 row=contract/accessibility-tree-states/aria-checked sha=d996867567c2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:215
quote: element_data.attr(local_name!("aria-checked"))
@row_add n=7 row=contract/accessibility-tree-states/aria-invalid area=2-a11y-strategy rec=fct-cixbakze@3,fct-v47qtadu@1
boundary: Accessibility tree states
item: aria-invalid
promise: Trimmed and matched in lower case, aria-invalid writes the node's invalid state.
@row_check n=7 row=contract/accessibility-tree-states/aria-invalid sha=adbf0f148899 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:230-231
quote: let invalid = element_data.attr(local_name!("aria-invalid")); match invalid.map(str::trim_ascii) {
@row_add n=8 row=contract/accessibility-tree-states/select-option area=2-a11y-strategy rec=fct-cixbakze@3
boundary: Accessibility tree states
item: select option
promise: A select's option reads its chosen state into the tree.
@row_check n=8 row=contract/accessibility-tree-states/select-option sha=91ad7066c843 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:186-187
quote: if select::is_select_option(node) { builder.set_selected(select::is_chosen(node));
@row_add n=9 row=contract/accessibility-tree-states/native-checkbox-or-radio area=2-a11y-strategy rec=fct-cixbakze@3
boundary: Accessibility tree states
item: native checkbox or radio
promise: A native checkbox or radio under a checkable role writes toggled from its own checked state, reading neither aria-checked nor aria-pressed; under any other role it writes nothing.
@row_check n=9 row=contract/accessibility-tree-states/native-checkbox-or-radio sha=af51c92eb8cd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:205-207
quote: if is_native_checkable(element_data) { if is_checkable(role) { match element_data.checkbox_input_checked() {
@row_add n=10 row=contract/aria-property-reads/aria-labelledby area=2-a11y-strategy rec=fct-ra4scv5d@1
boundary: ARIA name and property reads
item: aria-labelledby
promise: The referenced targets' text becomes the node's label, outranking a <label> and an aria-label (accname step 2B).
@row_check n=10 row=contract/aria-property-reads/aria-labelledby sha=199623ccd89e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:83-85
quote: if !label.text.is_empty() { builder.set_label(label.text.as_str()); }
@row_add n=11 row=contract/aria-property-reads/aria-describedby area=2-a11y-strategy rec=fct-ra4scv5d@1
boundary: ARIA name and property reads
item: aria-describedby
promise: The builder reads aria-describedby, one read per attribute, as a source of the node's description.
@row_check n=11 row=contract/aria-property-reads/aria-describedby sha=0122328d7475 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:90
quote: let Some(id_refs) = self.nodes[node_id].attr(local_name!("aria-describedby")) else {
@row_add n=12 row=contract/aria-property-reads/aria-description area=2-a11y-strategy rec=fct-ra4scv5d@1
boundary: ARIA name and property reads
item: aria-description
promise: The builder reads aria-description, one read per attribute, as the node's description.
@row_check n=12 row=contract/aria-property-reads/aria-description sha=4289d75b8c9d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:252-253
quote: if let Some(aria_description) = element_data.attr(local_name!("aria-description")) { builder.set_description(aria_description);
@row_add n=13 row=contract/aria-property-reads/aria-expanded area=2-a11y-strategy rec=fct-ra4scv5d@1,fct-6aje67jv@1
boundary: ARIA name and property reads
item: aria-expanded
promise: aria-expanded is read as true or false and nothing else on every element but a single select, which writes its list's open state instead and reads no author attribute.
@row_check n=13 row=contract/aria-property-reads/aria-expanded sha=4a42c9ba2916 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:240-241
quote: if select::is_single_select(element_data) { builder.set_expanded(self.select_list_for(node.id).is_some());
at: packages/blitz-dom/src/accessibility.rs:242-245
quote: } else if let Some(aria_expanded) = element_data.attr(local_name!("aria-expanded")) && let Ok(aria_expanded) = aria_expanded.parse() { builder.set_expanded(aria_expanded);
@row_add n=14 row=contract/aria-property-reads/aria-disabled area=2-a11y-strategy rec=fct-ra4scv5d@1,fct-6aje67jv@1
boundary: ARIA name and property reads
item: aria-disabled
promise: Only the value true of aria-disabled sets the node disabled.
@row_check n=14 row=contract/aria-property-reads/aria-disabled sha=3abd225af74c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:248
quote: if element_data.attr(local_name!("aria-disabled")) == Some("true") {
@row_add n=15 row=contract/aria-property-reads/aria-level area=2-a11y-strategy rec=fct-ra4scv5d@1,fct-6aje67jv@1
boundary: ARIA name and property reads
item: aria-level
promise: aria-level is read as its number less one, headings keeping their default levels.
@row_check n=15 row=contract/aria-property-reads/aria-level sha=0f646a5aca88 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:255-256
quote: if let Some(aria_level) = element_data.attr(local_name!("aria-level")) && let Ok(aria_level) = aria_level.parse::<usize>()
at: packages/blitz-dom/src/accessibility.rs:261
quote: builder.set_level(aria_level);
@row_add n=16 row=decision/a11y-assertions-as-rust-tests area=3-a11y-assertion-harness-contract rec=fct-hmqmd5jd@1
name: Accessibility assertions as Rust tests over AccessKit
rule: Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
@row_check n=16 row=decision/a11y-assertions-as-rust-tests sha=5009397b02bf v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/accessibility_hidden.rs:29
quote: fn excludes_div_with_hidden_attribute() -> TestResult<()> {
@row_add n=17 row=decision/wcag-contrast-thresholds area=3-a11y-assertion-harness-contract rec=fct-2n2giycw@1
name: WCAG 2.x text contrast thresholds
rule: Text contrast is measured by the WCAG 2.x formula from computed styles and must reach 4.5, or 3.0 for large text.
guard: stand_contrast
@row_check n=17 row=decision/wcag-contrast-thresholds sha=b551787a8abf v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/contrast.rs:52
quote: if self.is_large_text() { 3.0 } else { 4.5 }
at: packages/blitz-test-harness/src/contrast.rs:19
quote: pub fn contrast_ratio(a: Color, b: Color) -> f32 {
guard_at: tests/blitz-tests/tests/stand_contrast.rs:602
@row_add n=18 row=decision/html-aam-role-mapping area=4-aria-patterns-roles rec=fct-4yuvm6ln@1
name: Native element roles follow HTML-AAM
rule: Native HTML elements map to accessibility roles as HTML-AAM states: landmarks, headings, lists, tables, interactive and inline semantics.
@row_check n=18 row=decision/html-aam-role-mapping sha=5050d3fa644a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:467-469
quote: fn role_from_element_data(element_data: &ElementData) -> Option<Role> { // <https://www.w3.org/TR/html-aam-1.0/> match &*element_data.name.local {
@row_add n=19 row=contract/aria-role-mapping/role-switch area=4-aria-patterns-roles rec=fct-v47qtadu@1
boundary: ARIA role mapping
item: role="switch"
promise: role="switch" maps to the Switch role.
@row_check n=19 row=contract/aria-role-mapping/role-switch sha=d0bbbb1bd3a8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:374
quote: "switch" => Some(Role::Switch),
@row_add n=20 row=contract/accessibility-tree-hidden-content/aria-hidden area=4-aria-patterns-roles rec=fct-zeglb7pr@1
boundary: Accessibility tree hidden content
item: aria-hidden="true"
promise: A node with aria-hidden="true" is kept in the tree, marked hidden.
@row_check n=20 row=contract/accessibility-tree-hidden-content/aria-hidden sha=94ce26f87bd2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:179-180
quote: if element_data.attr(local_name!("aria-hidden")) == Some("true") { builder.set_hidden();
@row_add n=21 row=contract/accessibility-tree-hidden-content/hidden area=4-aria-patterns-roles rec=fct-zeglb7pr@1
boundary: Accessibility tree hidden content
item: hidden
promise: The hidden attribute excludes the node and its children from the tree.
@row_check n=21 row=contract/accessibility-tree-hidden-content/hidden sha=3f042ca25da7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:1219-1221
quote: if *name == local_name!("hidden") { use style::values::specified::Display; push_style(PropertyDeclaration::Display(Display::None));
at: packages/blitz-dom/src/accessibility.rs:296
quote: (s.styles.is_display_none() && !is_select_option)
@row_add n=22 row=contract/accessibility-tree-hidden-content/display-none area=4-aria-patterns-roles rec=fct-zeglb7pr@1
boundary: Accessibility tree hidden content
item: display: none
promise: display: none excludes the node and its children from the tree, except an option under a select.
@row_check n=22 row=contract/accessibility-tree-hidden-content/display-none sha=4d349feb8f72 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:296
quote: (s.styles.is_display_none() && !is_select_option)
@row_add n=23 row=contract/accessibility-tree-hidden-content/visibility-hidden area=4-aria-patterns-roles rec=fct-zeglb7pr@1
boundary: Accessibility tree hidden content
item: visibility: hidden
promise: visibility: hidden excludes the node and its children from the tree.
@row_check n=23 row=contract/accessibility-tree-hidden-content/visibility-hidden sha=64c64d75e8e5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:297-298
quote: || s.styles.primary().clone_visibility() == visibility::computed_value::T::Hidden
@row_add n=24 row=decision/disabled-by-attribute-presence area=5-keyboard-navigation rec=fct-s5i3di32@1
name: Disabled-ness keyed one way by attribute presence
rule: On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
@row_check n=24 row=decision/disabled-by-attribute-presence sha=092d50e488e4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:519
quote: self.can_be_disabled() && self.has_attr(local_name!("disabled"))
@row_add n=25 row=decision/double-click-threshold area=5-keyboard-navigation rec=fct-qqmo7pmq@1
name: Double click threshold
rule: Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, each still focusing its target as one click does.
@row_check n=25 row=decision/double-click-threshold sha=85084a3abd9e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:399-402
quote: .map(|t| doc.elapsed_since(t) < Duration::from_millis(500)) .unwrap_or(false) && (doc.mousedown_position.x - x).abs() <= 2.0 && (doc.mousedown_position.y - y).abs() <= 2.0
@row_add n=26 row=contract/range-input/click area=5-keyboard-navigation rec=fct-uy6igcde@1
boundary: range input
item: click
promise: A click on a range input focuses it and changes no value.
@row_check n=26 row=contract/range-input/click sha=cff2c74f352e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:842
quote: if el.range_input_data().is_some() {
@row_add n=27 row=contract/range-input/main-button-pointerdown area=5-keyboard-navigation rec=fct-uy6igcde@1
boundary: range input
item: main-button pointerdown
promise: A main-button pointerdown on a range input that is not disabled sets the value from the hit's fraction of the content box, clamped to [0, 1].
@row_check n=27 row=contract/range-input/main-button-pointerdown sha=7d7254a1dda2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:746
quote: if button == MouseEventButton::Main {
at: packages/blitz-dom/src/events/pointer.rs:732
quote: (offset / width).clamp(0.0, 1.0)
@row_add n=28 row=contract/range-input/arrowright-arrowup area=5-keyboard-navigation rec=fct-hk63g4fa@1
boundary: range input
item: ArrowRight / ArrowUp
promise: A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp.
@row_check n=28 row=contract/range-input/arrowright-arrowup sha=bf31c751bd7a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:166
quote: Key::Named(NamedKey::ArrowRight) | Key::Named(NamedKey::ArrowUp) => RangeMove::StepUp,
@row_add n=29 row=contract/range-input/arrowleft-arrowdown area=5-keyboard-navigation rec=fct-hk63g4fa@1
boundary: range input
item: ArrowLeft / ArrowDown
promise: A focused range input with no disabled attribute steps down on ArrowLeft and ArrowDown.
@row_check n=29 row=contract/range-input/arrowleft-arrowdown sha=c024dbc4c67f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:167
quote: Key::Named(NamedKey::ArrowLeft) | Key::Named(NamedKey::ArrowDown) => RangeMove::StepDown,
@row_add n=30 row=contract/range-input/home area=5-keyboard-navigation rec=fct-hk63g4fa@1
boundary: range input
item: Home
promise: A focused range input with no disabled attribute steps to its minimum on Home.
@row_check n=30 row=contract/range-input/home sha=8cf89dd3252f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:168
quote: Key::Named(NamedKey::Home) => RangeMove::Minimum,
@row_add n=31 row=contract/range-input/end area=5-keyboard-navigation rec=fct-hk63g4fa@1
boundary: range input
item: End
promise: A focused range input with no disabled attribute steps to its maximum on End.
@row_check n=31 row=contract/range-input/end sha=f53b7e9e7e21 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:169
quote: Key::Named(NamedKey::End) => RangeMove::Maximum,
@row_add n=32 row=contract/range-input/other-keys area=5-keyboard-navigation rec=fct-hk63g4fa@1
boundary: range input
item: other keys
promise: On a focused range input every key other than the arrows, Home and End does nothing.
@row_check n=32 row=contract/range-input/other-keys sha=022eeef7effc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:170
quote: _ => return,
@row_add n=33 row=contract/range-input/value area=5-keyboard-navigation rec=fct-hk63g4fa@1
boundary: range input
item: value
promise: A range input's value is held apart from the value attribute and sanitized to [min, max] on the step grid.
@row_check n=33 row=contract/range-input/value sha=7e31ab065750 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/range.rs:14-16
quote: pub struct RangeInputData { value: f64, }
at: packages/blitz-dom/src/node/range.rs:23
quote: value: rules.sanitize_attribute(value_attribute),
@row_add n=34 row=contract/range-input/input-event area=5-keyboard-navigation rec=fct-hk63g4fa@1
boundary: range input
item: input event
promise: Each change of a range input's value fires one input event; a move that changes nothing fires none.
@row_check n=34 row=contract/range-input/input-event sha=089f4c72e543 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:548-551
quote: dispatch_event(DomEvent::new( node_id, DomEventData::Input(BlitzInputEvent { value }), ));
at: packages/blitz-dom/src/events/keyboard.rs:544-546
quote: else { return; };
@row_add n=35 row=contract/select-closed-list/arrowdown-arrowup area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list closed
item: ArrowDown / ArrowUp
promise: A focused single-choice select that is no disabled form control, its list closed, moves to the next / previous enabled option on ArrowDown / ArrowUp.
@row_check n=35 row=contract/select-closed-list/arrowdown-arrowup sha=885e6af3efe2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:272-273
quote: Key::Named(NamedKey::ArrowDown) => Some(SelectStep::Next), Key::Named(NamedKey::ArrowUp) => Some(SelectStep::Previous),
at: packages/blitz-dom/src/events/keyboard.rs:112-113
quote: } else if select::is_single_select(element_data) && !element_data.is_disabled()
@row_add n=36 row=contract/select-closed-list/home-end area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list closed
item: Home / End
promise: A focused single-choice select that is no disabled form control, its list closed, moves to the first / last enabled option on Home / End.
@row_check n=36 row=contract/select-closed-list/home-end sha=aefb460a6499 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:274-275
quote: Key::Named(NamedKey::Home) => Some(SelectStep::First), Key::Named(NamedKey::End) => Some(SelectStep::Last),
@row_add n=37 row=contract/select-closed-list/typeahead area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list closed
item: typeahead
promise: A character typed with no Ctrl, Alt or Meta on a focused closed single-choice select is typeahead, kept for 1 s of the document's clock.
@row_check n=37 row=contract/select-closed-list/typeahead sha=2307d814be36 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:282
quote: !modifiers.intersects(Modifiers::CONTROL | Modifiers::ALT | Modifiers::META)
at: packages/blitz-dom/src/node/select.rs:54
quote: pub(crate) const TYPEAHEAD_TIMEOUT: Duration = Duration::from_secs(1);
@row_add n=38 row=contract/select-closed-list/enter-space area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list closed
item: Enter / Space
promise: Enter, and a Space with no typed text standing, open a focused single-choice select's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select opens nothing.
@row_check n=38 row=contract/select-closed-list/enter-space sha=65fcaf8dd4f8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:195-198
quote: (Key::Named(NamedKey::Enter), None) => { doc.open_select_list(node_id); None }
at: packages/blitz-dom/src/events/keyboard.rs:200-201
quote: if text == " " && !doc.select_text_stands(node_id) { doc.open_select_list(node_id);
at: packages/blitz-dom/src/document.rs:802-804
quote: && !element.is_disabled() && !node.is_inert() && shown()
@row_add n=39 row=contract/select-closed-list/other-keys area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list closed
item: other keys
promise: On a focused closed single-choice select every key other than the arrows, Home, End, typeahead, Enter and Space does nothing.
@row_check n=39 row=contract/select-closed-list/other-keys sha=40e5906f67d2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:208
quote: _ => None,
@row_add n=40 row=contract/single-select/change-events area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select
item: input and change events
promise: Each change of a single-choice select's value dispatches one input then one change; a key that changes nothing dispatches none.
@row_check n=40 row=contract/single-select/change-events sha=adfab54e375b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:480-482
quote: if !self.set_select_choice(&options, chosen) { return; }
at: packages/blitz-dom/src/events/keyboard.rs:485-494
quote: dispatch_event(DomEvent::new( select, DomEventData::Input(BlitzInputEvent { value: value.clone(), }), )); dispatch_event(DomEvent::new( select, DomEventData::Change(BlitzInputEvent { value }), ));
@row_add n=41 row=contract/select-open-list/navigation-keys area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list open
item: navigation keys
promise: With a select's list open, the engine paints the list over the page, focus stays on the select, and the select's keys move the highlight over enabled rows, choosing nothing.
@row_check n=41 row=contract/select-open-list/navigation-keys sha=0537f862a82c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:248
quote: doc.move_select_list_current(option);
at: packages/blitz-paint/src/render.rs:248
quote: self.paint_select_list(scene);
@row_add n=42 row=contract/select-open-list/enter-space area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list open
item: Enter / Space
promise: With a select's list open, Enter and Space choose the highlighted row and close the list.
@row_check n=42 row=contract/select-open-list/enter-space sha=12786b96cef0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:228-229
quote: (Key::Named(NamedKey::Enter), None) => { choose_current_and_close(doc, node_id, dispatch_event);
at: packages/blitz-dom/src/events/keyboard.rs:237-238
quote: if text == " " && !doc.select_text_stands(node_id) { choose_current_and_close(doc, node_id, dispatch_event);
@row_add n=43 row=contract/select-open-list/tab area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list open
item: Tab
promise: With a select's list open, Tab in either direction chooses the highlighted row and closes the list before focus moves.
@row_check n=43 row=contract/select-open-list/tab sha=f8692a090102 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:31-33
quote: if event.key == Key::Named(NamedKey::Tab) { if let Some(select) = doc.open_select() { choose_current_and_close(doc, select, &mut dispatch_event);
@row_add n=44 row=contract/select-open-list/escape area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list open
item: Escape
promise: With a select's list open, Escape closes the list with the value unchanged and no event, as a second click, a press outside and a page scroll do.
@row_check n=44 row=contract/select-open-list/escape sha=1c4ad725b5d9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:232-233
quote: (Key::Named(NamedKey::Escape), None) => { doc.close_select_list();
at: packages/blitz-dom/src/events/pointer.rs:955
quote: doc.close_select_list();
at: packages/blitz-dom/src/events/pointer.rs:632
quote: doc.close_select_list();
at: packages/blitz-dom/src/events/pointer.rs:1201
quote: doc.close_select_list();
@row_add n=45 row=contract/select-open-list/close area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select, list open
item: close
promise: Closing a select's list clears the typed typeahead text.
@row_check n=45 row=contract/select-open-list/close sha=73b01ee76939 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:895-896
quote: if let Some(list) = self.select_list.take() { self.select_typeahead = None;
@row_add n=46 row=contract/single-select/optgroup-options area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: single-choice select
item: optgroup options
promise: An optgroup's options are the select's own options, in tree order.
@row_check n=46 row=contract/single-select/optgroup-options sha=b917f537e7ac v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select.rs:153-157
quote: } else if child_node .data .is_element_with_tag_name(&local_name!("optgroup")) { list.extend(child_node.children.iter().copied().filter(|grandchild| {
@row_add n=47 row=contract/multiple-select/keys-and-row-clicks area=5-keyboard-navigation rec=fct-qjlotlrc@1
boundary: multiple select
item: keys and row clicks
promise: A multiple select takes its own keys and row clicks, apart from the single-choice select's handling.
@row_check n=47 row=contract/multiple-select/keys-and-row-clicks sha=54a9cd19d2cf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:118-123
quote: } else if select::is_multiple_select(element_data) && !element_data.is_disabled() && let KeyboardOrTextInputEvent::KeyPress(key_event) = &event && key_event.state.is_pressed() { press_multiple_select_key(doc, node_id, key_event, &mut dispatch_event);
@row_add n=48 row=contract/keyboard-activation/enter area=5-keyboard-navigation rec=fct-hhqldh6v@1
boundary: keyboard activation
item: Enter
promise: A pressed Enter on the focused node dispatches a Click when the node is no disabled form control and is a button, a summary, an input of type submit, reset or button, or an a with href.
@row_check n=48 row=contract/keyboard-activation/enter sha=46da5ac6434e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:336-341
quote: local_name!("button") | local_name!("summary") => true, local_name!("input") => matches!( element.attr(local_name!("type")), Some("submit" | "reset" | "button") ), local_name!("a") => element.has_attr(local_name!("href")),
@row_add n=49 row=contract/keyboard-activation/space area=5-keyboard-navigation rec=fct-hhqldh6v@1
boundary: keyboard activation
item: Space
promise: A released Space dispatches a Click for a summary, or for a button or an input of type submit, reset, button, checkbox or radio that is no disabled form control, never for a link.
@row_check n=49 row=contract/keyboard-activation/space sha=ff0cff29575d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:353-358
quote: local_name!("button") | local_name!("summary") => true, local_name!("input") => matches!( element.attr(local_name!("type")), Some("submit" | "reset" | "button" | "checkbox" | "radio") ), _ => false,
@row_add n=50 row=contract/keyboard-activation/handler-order area=5-keyboard-navigation rec=fct-hhqldh6v@1
boundary: keyboard activation
item: handler order
promise: On a keyboard-dispatched Click the app's handlers run first, then the engine's handle_click.
@row_check n=50 row=contract/keyboard-activation/handler-order sha=43005e32879a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:354-356
quote: let event_state = self.run_handler_event(&mut event, EventState::default()); if !event_state.is_cancelled() { self.run_default_action(&mut event);
@row_add n=51 row=decision/falsy-boolean-attribute-removed area=5-keyboard-navigation rec=fct-fi2um7zb@1
name: Falsy boolean attribute removed
rule: A falsy Dioxus value of one of the bridge's boolean attributes (upstream's is_bool_attr set) removes the attribute on both attribute paths; a truthy value is written, and an attribute outside the list holding false is written as "false".
why: So an enabled control does not match :disabled and hidden: false keeps an element laid out and in the tree.
@row_check n=51 row=decision/falsy-boolean-attribute-removed sha=4c6d98910456 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/mutation_writer.rs:428-429
quote: if is_falsy && is_bool_attr(local_name) { docm.clear_attribute(node_id, name);
@row_add n=52 row=contract/crud-list/tab-stop area=5-keyboard-navigation rec=fct-utkipxkl@1
boundary: seven_guis CRUD list
item: Tab stop
promise: The list crud-list, a listbox named People, is the third CRUD Tab stop; its person rows are not focusable.
@row_check n=52 row=contract/crud-list/tab-stop sha=88eb4a8693c5 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/common/mod.rs:83
quote: "crud-list",
at: examples/seven_guis/src/tasks/crud.rs:147
quote: role: "option",
@row_add n=53 row=contract/crud-list/arrowdown-arrowup-home-end area=5-keyboard-navigation rec=fct-utkipxkl@1
boundary: seven_guis CRUD list
item: ArrowDown / ArrowUp / Home / End
promise: With crud-list focused, ArrowDown, ArrowUp, Home and End move the selection as a row's click does.
@row_check n=53 row=contract/crud-list/arrowdown-arrowup-home-end sha=d21ff95216a8 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:122-128
quote: let to = match (e.key(), at) { (Key::ArrowDown, None) => 0, (Key::ArrowDown, Some(k)) => k + 1, (Key::ArrowUp, Some(k)) if k > 0 => k - 1, (Key::Home, _) => 0, (Key::End, _) if !visible.is_empty() => visible.len() - 1,
@row_add n=54 row=contract/cells-grid/tab-stop area=5-keyboard-navigation rec=fct-5i3imozy@3
boundary: seven_guis Cells grid
item: Tab stop
promise: cells-grid, named Cells by aria_label with no role, is one Tab stop.
@row_check n=54 row=contract/cells-grid/tab-stop sha=aeaa75c52c4f v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:414-415
quote: tabindex: "0", aria_label: "Cells",
@row_add n=55 row=contract/cells-grid/arrows area=5-keyboard-navigation rec=fct-5i3imozy@3
boundary: seven_guis Cells grid
item: arrows
promise: The arrows move the Cells grid's current cell, clamped, and bring the cell into view.
@row_check n=55 row=contract/cells-grid/arrows sha=2fb0da32e58a v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:423-426
quote: Key::ArrowLeft => (r, c.saturating_sub(1)), Key::ArrowRight => (r, (c + 1).min(COLS - 1)), Key::ArrowUp => (r.saturating_sub(1), c), Key::ArrowDown => ((r + 1).min(ROWS - 1), c),
@row_add n=56 row=contract/cells-grid/enter area=5-keyboard-navigation rec=fct-5i3imozy@3
boundary: seven_guis Cells grid
item: Enter
promise: Enter on the Cells grid opens the editor on the current cell, focused.
@row_check n=56 row=contract/cells-grid/enter sha=bb9ecf5d2056 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:427-432
quote: Key::Enter => { edit_val.set(sheet().get_raw(r, c).to_string()); keyed_open.set(true); selected.set(Some((r, c))); return; }
@row_add n=57 row=contract/cells-grid/editor-enter area=5-keyboard-navigation rec=fct-5i3imozy@3
boundary: seven_guis Cells grid
item: editor Enter
promise: Enter in the Cells editor commits and returns focus to the grid.
@row_check n=57 row=contract/cells-grid/editor-enter sha=a478deedd375 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:511-516
quote: sheet.write().raw.insert((r, c), edit_val()); selected.set(None); if keyed_open() { keyed_open.set(false); if let Some(grid) = grid() { drop(grid.set_focus(true));
@row_add n=58 row=contract/cells-grid/tab-from-editor area=5-keyboard-navigation rec=fct-5i3imozy@3
boundary: seven_guis Cells grid
item: Tab from editor
promise: Tab leaving an open Cells editor commits it.
@row_check n=58 row=contract/cells-grid/tab-from-editor sha=c9a5b8467610 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:500-502
quote: onblur: move |_| { if selected() == Some((r, c)) { sheet.write().raw.insert((r, c), edit_val());
@row_add n=59 row=contract/cells-grid/focus-outline area=5-keyboard-navigation rec=fct-5i3imozy@3
boundary: seven_guis Cells grid
item: focus outline
promise: The focused Cells grid draws a 2px outline in the stand accent on the current cell.
@row_check n=59 row=contract/cells-grid/focus-outline sha=82ff8c196cd5 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:600-601
quote: .cells-grid:focus .cell.current { outline: 2px solid #3a5ce5;
@row_add n=60 row=decision/stated-tab-order-walked area=5-keyboard-navigation rec=fct-ivsezocx@1
name: Stated Tab order walked
rule: Each lean task, the load page and Cells state their Tab order, walked forward and back in both layout modes with the engine's focus, the snapshot's focused node and the tree's focus agreeing at every step, and every lean task's control lying in one of the two lean orders.
guard: stand_keyboard
@row_check n=60 row=decision/stated-tab-order-walked sha=d0f584ac27a3 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_keyboard.rs:162
quote: assert_walk(&mut harness, &LOAD_TAB_ORDER, &mode);
at: tests/blitz-tests/tests/stand_keyboard.rs:198
quote: assert_walk(&mut harness, &CELLS_TAB_ORDER, &mode);
guard_at: tests/blitz-tests/tests/stand_keyboard.rs:337
@row_add n=61 row=decision/select-list-coverage-headless area=5-keyboard-navigation rec=fct-wj74cpt7@1
name: Select list coverage headless
rule: The select list's keyboard and focus coverage is headless, asserting SC 2.1.1 and 2.4.3 on it in both layout modes; the windowed list is read and not run.
guard: select_popup, stand_act_popup, stand_keyboard
@row_check n=61 row=decision/select-list-coverage-headless sha=1121fe4b1955 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_keyboard.rs:1083
quote: fn the_flight_type_walk_holds_with_its_list_open_and_after_each_close() {
guard_at: tests/blitz-tests/tests/select_popup.rs:1
@row_add n=62 row=decision/stand-contrast-measured area=6-visual-design-verification rec=fct-o6t6iprq@1
name: Stand contrast measured from computed styles
rule: SC 1.4.3 is measured on the stand from computed styles with the WCAG 2.x ratio, 3.0 required of large text and else 4.5, a row exempt exactly where the snapshot's enabled reads Some(false).
guard: stand_contrast, the_load_page_meets_its_ratios, cells_meets_its_ratios
@row_check n=62 row=decision/stand-contrast-measured sha=2b734569111b v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_contrast.rs:185
quote: Some(contrast) if enabled == Some(Some(false)) => ExemptInactive(measured(contrast)),
guard_at: tests/blitz-tests/tests/stand_contrast.rs:671
@row_add n=63 row=decision/lint-contrast-single-formula area=6-visual-design-verification rec=fct-d6ibr4tu@1
name: Lint contrast through one formula
rule: The driver's visual lint reads contrast through the harness's text contrast with the stand's exempt rule, never a second formula or threshold.
@row_check n=63 row=decision/lint-contrast-single-formula sha=7da8085eb5fd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:548
quote: let reading = harness.text_contrast(element)?;
@row_add n=64 row=contract/visual-lint-finding/content area=6-visual-design-verification rec=fct-d6ibr4tu@1
boundary: visual lint finding
item: content
promise: A visual lint finding is written as ids, closed words and numbers only.
@row_check n=64 row=contract/visual-lint-finding/content sha=970e9e3f7803 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:550-553
quote: (reading.ratio < required).then_some(Reason::Contrast { ratio: reading.ratio, required, })
@row_add n=65 row=decision/select-list-no-motion area=6-visual-design-verification rec=fct-wku6yqyk@1
name: Select list without motion
rule: A select list opens and closes with no motion.
@row_check n=65 row=decision/select-list-no-motion sha=a2bf21ccb973 v=holds judge=sonnet head=edd0cfaea7f0
searched: transition|animation|animat|duration|motion · in packages/blitz-dom/src/node/select_list.rs
searched: transition|animation|animate|fade · in packages/blitz-dom/src/node/select*
@row_add n=66 row=contract/accessibility-tree/hidden-and-inert-nodes area=6-visual-design-verification rec=fct-xkzcu6wr@1
boundary: accessibility tree
item: hidden and inert nodes
promise: Nodes with display: none or visibility: hidden or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.
@row_check n=66 row=contract/accessibility-tree/hidden-and-inert-nodes sha=4e967ebf838a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:289
quote: element.has_attr(local_name!("inert"))
at: packages/blitz-dom/src/accessibility.rs:296
quote: (s.styles.is_display_none() && !is_select_option)
at: packages/blitz-dom/src/accessibility.rs:20-26
quote: if node.is_hidden_from_accessibility_tree() || node .parent .map(|p| hidden_nodes.contains(&p)) .unwrap_or(false) { hidden_nodes.insert(node_id);
@row_add n=67 row=contract/hit-testing/inert area=6-visual-design-verification rec=fct-xkzcu6wr@1
boundary: hit-testing
item: inert
promise: A hit passes through an element carrying inert, and removing inert restores the tree and the hit.
@row_check n=67 row=contract/hit-testing/inert sha=515378495571 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1454-1459
quote: if self .element_data() .is_some_and(|element| element.has_attr(local_name!("inert"))) { return None; }
@row_add n=68 row=contract/accessible-name/aria-label area=7-screen-reader-support rec=fct-nbzuj5df@1
boundary: accessible name
item: aria-label
promise: An element's aria-label, non-empty after trimming, becomes its label; a whitespace-only one names nothing.
@row_check n=68 row=contract/accessible-name/aria-label sha=1b0018a134b1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:159-163
quote: if let Some(label) = element_data .attr(local_name!("aria-label")) .filter(|label| !label.trim().is_empty()) { builder.set_label(label);
@row_add n=69 row=contract/accessible-name/aria-labelledby area=7-screen-reader-support rec=fct-nbzuj5df@1,fct-c5th5uwj@1
boundary: accessible name
item: aria-labelledby
promise: An aria-labelledby whose targets hold text outranks aria-label and a bound label element; aria-label is the fallback when the targets hold none.
@row_check n=69 row=contract/accessible-name/aria-labelledby sha=8c862a6daa96 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:83-85
quote: if !label.text.is_empty() { builder.set_label(label.text.as_str()); }
@row_add n=70 row=contract/accessible-name/label-element area=7-screen-reader-support rec=fct-c5th5uwj@1
boundary: accessible name
item: label element
promise: A label names the labelable element bound to it, its for-target by id or else its first nested labelable element, whether placed before or after it; a hidden input is named by no label.
@row_check n=70 row=contract/accessible-name/label-element sha=7612ea9d0555 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:54
quote: let Some(input_id) = self.label_bound_input_element(label_id).map(|n| n.id) else {
at: packages/blitz-dom/src/document.rs:3540-3542
quote: local_name!("input") => !element .attr(local_name!("type")) .is_some_and(|kind| kind.eq_ignore_ascii_case("hidden")),
@row_add n=71 row=decision/adapter-tree-equals-harness-tree area=7-screen-reader-support rec=fct-iq4fjjyz@1
name: Adapter tree equals harness tree
rule: On the stand the tree the shell hands the adapter equals the harness's tree node for node, and every interactive-role node of the four lean tasks reads a non-empty name.
@row_check n=71 row=decision/adapter-tree-equals-harness-tree sha=b0a6609d531a v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_accessibility_ids.rs:323
quote: by_id(&handed) == by_id(&overridden)
@row_add n=72 row=decision/role-name-tables-stated-once area=7-screen-reader-support rec=fct-iq4fjjyz@1,fct-uz4kdss3@2
name: Expected roles and names stated once
rule: The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
@row_check n=72 row=decision/role-name-tables-stated-once sha=972e36742c83 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/common/mod.rs:133
quote: pub const INPUT_NAMES: [(&str, &str); 6] = [
@row_add n=73 row=decision/snapshot-reads-accessibility-tree area=7-screen-reader-support rec=fct-uz4kdss3@2
name: Snapshot reads the accessibility tree
rule: The snapshot takes each node's role, name, focus, selected, pressed and invalid from the accessibility tree, never from the element.
@row_check n=73 row=decision/snapshot-reads-accessibility-tree sha=5b445cf6e488 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:192-197
quote: selected: tree_node.is_selected(), pressed: tree_node .toggled() .filter(|_| !checkable) .map(|toggled| toggled == Toggled::True), invalid: tree_node.invalid().is_some(),
at: packages/dioxus-native-dom/src/snapshot.rs:178-179
quote: role: tree_node.role(), name: self.name(tree_node, &mut HashSet::new()).trim().to_string(),
at: packages/dioxus-native-dom/src/snapshot.rs:198
quote: focused: id == self.focus,
@row_add n=74 row=decision/snapshot-model-single-source area=7-screen-reader-support rec=fct-uz4kdss3@2
name: Snapshot model as single source
rule: The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else.
@row_check n=74 row=decision/snapshot-model-single-source sha=fb09b57bc861 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: Snapshot · in packages/dioxus-native-dom/src/snapshot_text.rs
searched: fn to_text · in packages/dioxus-native-dom/src
why: no evidence found
@row_add n=75 row=decision/ui-graph-one-node-per-snapshot-node area=7-screen-reader-support rec=fct-uz4kdss3@2
name: UI graph mirrors the snapshot
rule: The read-only UI graph holds one node per snapshot node, names and closed words only.
@row_check n=75 row=decision/ui-graph-one-node-per-snapshot-node sha=f438bc91ba6f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:200
quote: for snapshot_node in snapshot.nodes() {
@row_add n=76 row=contract/snapshot/enabled area=7-screen-reader-support rec=fct-uz4kdss3@2
boundary: snapshot
item: enabled
promise: The snapshot's enabled field reads the presence of the element's disabled attribute.
@row_check n=76 row=contract/snapshot/enabled sha=7049df81669b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:183
quote: .then(|| !element.has_attr(local_name!("disabled"))),
@row_add n=77 row=contract/snapshot/checked area=7-screen-reader-support rec=fct-uz4kdss3@2
boundary: snapshot
item: checked
promise: The snapshot's checked field reads a native checkbox's or radio's state, else a checkable role's toggled.
@row_check n=77 row=contract/snapshot/checked sha=3f62a08881c7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:184
quote: checked: element.checkbox_input_checked().or_else(|| {
@row_add n=78 row=contract/snapshot/clickable area=7-screen-reader-support rec=fct-uz4kdss3@2,fct-77ewqepn@4
boundary: snapshot
item: clickable
promise: The snapshot's clickable field says an element that is no disabled form control reacts to a click, read from the vdom walk's current click listeners and not the listener mark.
@row_check n=78 row=contract/snapshot/clickable sha=ff934cab72ae v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:199
quote: clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
at: packages/dioxus-native-dom/src/snapshot.rs:112
quote: clicked: crate::element_id::element_listeners(&self.vdom, &self.vdom_state)
@row_add n=79 row=decision/password-value-masked area=7-screen-reader-support rec=fct-2hrkecxm@1
name: Password value masked
rule: A password input's typed text appears in no snapshot field, tree node or painted frame; its value reads a fixed masked marker where it holds text and an empty string where it holds none.
@row_check n=79 row=decision/password-value-masked sha=1a736b4eeac3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
@row_add n=80 row=contract/snapshot/password-value area=7-screen-reader-support rec=fct-2hrkecxm@1
boundary: snapshot
item: password input value
promise: A password input's value reads MASKED_VALUE where it holds text and "" where it holds none.
@row_check n=80 row=contract/snapshot/password-value sha=970e51d86b0c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
@row_add n=81 row=contract/driver-actions/disabled area=7-screen-reader-support rec=fct-2hrkecxm@1
boundary: driver actions
item: disabled
promise: The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.
@row_check n=81 row=contract/driver-actions/disabled sha=b0c0cc0d432f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:531-532
quote: if target.enabled == Some(false) { return Err(Refusal::new(Cause::Disabled));
@row_add n=82 row=contract/accessibility-tree/multiple-select area=7-screen-reader-support rec=fct-uo5zb3r6@1
boundary: accessibility tree
item: multiple select
promise: A multiple select carries set_multiselectable in the tree.
@row_check n=82 row=contract/accessibility-tree/multiple-select sha=9ed559d11f75 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:174-175
quote: if select::is_multiple_select(element_data) { builder.set_multiselectable();
@row_add n=83 row=contract/snapshot/multiple-select-rows-and-value area=7-screen-reader-support rec=fct-uo5zb3r6@1
boundary: snapshot
item: multiple select rows and value
promise: A multiple select's rows read their own client rects in the snapshot, and its value reads its first chosen option in tree order.
@row_check n=83 row=contract/snapshot/multiple-select-rows-and-value sha=41b2a5d468ee v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:165
quote: None => self.doc.get_client_bounding_rect(boxed_by)?,
at: packages/blitz-dom/src/node/select.rs:201-204
quote: pub(crate) fn first_chosen_option(doc: &BaseDocument, select: NodeId) -> Option<NodeId> { options(doc, select) .into_iter() .find(|option| doc.get_node(*option).is_some_and(is_chosen))
@row_add n=84 row=contract/accessibility-tree/optgroup area=7-screen-reader-support rec=fct-uo5zb3r6@1
boundary: accessibility tree
item: optgroup
promise: An optgroup reads Role::Group, named by its non-empty label attribute.
@row_check n=84 row=contract/accessibility-tree/optgroup sha=fe23fd80b712 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:521
quote: "optgroup" => Some(Role::Group),
at: packages/blitz-dom/src/accessibility.rs:164-170
quote: } else if (select::is_select_option(node) || element_data.name.local == local_name!("optgroup")) && let Some(label) = element_data .attr(local_name!("label")) .filter(|label| !label.is_empty()) { builder.set_label(label);
@row_add n=85 row=contract/select-open-list/optgroup-header area=7-screen-reader-support rec=fct-uo5zb3r6@1
boundary: single-choice select, list open
item: optgroup header
promise: An optgroup is painted as a header of an open list, heavier than a row by weight on every build, and a header chooses nothing.
@row_check n=85 row=contract/select-open-list/optgroup-header sha=872dd693d060 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1087-1088
quote: if node.data.is_element_with_tag_name(&local_name!("optgroup")) { style.font_weight = parley::FontWeight::BOLD;
@row_add n=86 row=contract/accessibility-tree/single-select area=7-screen-reader-support rec=fct-sme4dbib@1
boundary: accessibility tree
item: single select
promise: A single select reads ComboBox and writes expanded from its list's open state, reading no aria-expanded; an author's open or aria-expanded on it steers nothing; its chosen option reads selected and the tree's focus stays on the select.
@row_check n=86 row=contract/accessibility-tree/single-select sha=2984fe598795 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:240-241
quote: if select::is_single_select(element_data) { builder.set_expanded(self.select_list_for(node.id).is_some());
at: packages/blitz-dom/src/accessibility.rs:186-187
quote: if select::is_select_option(node) { builder.set_selected(select::is_chosen(node));
@row_add n=87 row=contract/snapshot/select-list-rows area=7-screen-reader-support rec=fct-sme4dbib@1
boundary: snapshot
item: select list rows
promise: The snapshot holds no expanded key and the highlighted row is in no snapshot and no tree; an open list's row reads its own bounds in the list.
@row_check n=87 row=contract/snapshot/select-list-rows sha=5a6280308d28 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:163
quote: let bounds = match self.doc.select_list_bounds(node_id) {
searched: expanded · in packages/dioxus-native-dom/src/snapshot.rs
@row_add n=88 row=contract/accesskit-adapter/creation-order area=7-screen-reader-support rec=fct-xy7rrkj5@1
boundary: AccessKit adapter
item: creation order
promise: Adapter creation must happen before the window is first shown; it panics if the window is already visible.
@row_check n=88 row=contract/accesskit-adapter/creation-order sha=79325b4ec49d v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: panic! · in packages/accesskit_xplat/src
searched: assert! · in packages/accesskit_xplat/src
searched: Panics if the window is already visible · in packages/accesskit_xplat/src/lib.rs
why: no evidence found; the panic is stated only in a doc comment, no checking line in the tree
@row_add n=89 row=decision/invalid-state-not-colour-alone area=8-cognitive-accessibility rec=fct-x32cq65m@1
name: Invalid state not by colour alone
rule: Invalid dates carry an invalid class with red styling and disable the Book button, and the two date inputs also state aria-invalid, so the invalid state is in the tree and the snapshot and not carried by the class alone.
@row_check n=89 row=decision/invalid-state-not-colour-alone sha=39f7e59f32fc v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:111-112
quote: aria_invalid: if start_valid { "false" } else { "true" }, class: if start_valid { "date-input" } else { "date-input invalid" },
at: examples/seven_guis/src/tasks/flight_booker.rs:128
quote: disabled: !dates_ok,
@row_add n=90 row=decision/a11y-test-leg area=9-ci-integration rec=fct-6kcxmj7u@1
name: Accessibility test leg in the slow tier
rule: The accessibility test files run in the a11y leg of the slow tier behind the fast jobs, with no required status check; the stand-facing ones also run three times in the stand-repeat leg under the zero-failure budget.
guard: ci.yml a11y job
@row_check n=90 row=decision/a11y-test-leg sha=cd8d9ba54fce v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:247-250
quote: a11y: name: "Accessibility (a11y) tests" runs-on: ubuntu-latest needs: [fmt, clippy, test-features-default, ci-scripts]
at: .github/scripts/ci-leg.sh:41
quote: a11y) cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids --test accessibility_states
at: .github/scripts/ci-leg.sh:63
quote: for round in 1 2 3; do
guard_at: .github/workflows/ci.yml:247
@row_add n=91 row=decision/accesskit-accessibility-tree area=a11y-scope-summary rec=fct-w555gaqe@1
name: AccessKit for the accessibility tree
rule: AccessKit is the technology the accessibility tree is built in, taken as a workspace dependency.
@row_check n=91 row=decision/accesskit-accessibility-tree sha=cb573db67131 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:142
quote: accesskit = "0.25"
@row_add n=92 row=decision/accesskit-xplat-platform-adapter area=a11y-scope-summary rec=fct-mpp6ld7n@1,fct-w555gaqe@1
name: accesskit_xplat as the platform adapter
rule: The in-repo accesskit_xplat package, a workspace member, is the AccessKit platform adapter crate.
@row_check n=92 row=decision/accesskit-xplat-platform-adapter sha=caf1c5f5a64f v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:3
quote: "packages/accesskit_xplat",
at: packages/accesskit_xplat/Cargo.toml:5
quote: description = "AccessKit UI accessibility infrastructure: cross-platform adapter"
@row_add n=93 row=decision/accessibility-feature-default-on area=a11y-scope-summary rec=fct-lszvr7oi@1,fct-4z5libun@1,fct-2adxmtbt@1,fct-chsg6uib@1
name: Accessibility as a default-on feature
rule: The accessibility tree is built only under the accessibility cargo feature, which is on by default in blitz-dom, blitz-shell and blitz and forwarded down that chain; custom widgets require it.
@row_check n=93 row=decision/accessibility-feature-default-on sha=885f5d41daf8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/Cargo.toml:17
quote: "accessibility",
at: packages/blitz-shell/Cargo.toml:14
quote: default = ["accessibility", "clipboard", "file-dialog"]
at: packages/blitz/Cargo.toml:14
quote: default = ["net", "accessibility", "text-transform-icu", "tracing"]
at: packages/blitz/Cargo.toml:16
quote: accessibility = ["blitz-shell/accessibility"]
at: packages/blitz-shell/Cargo.toml:15-18
quote: accessibility = [     "dep:accesskit",     "dep:accesskit_xplat",     "blitz-dom/accessibility",
at: packages/blitz-dom/Cargo.toml:23
quote: custom-widget = ["dep:anyrender", "accessibility"]
@row_add n=94 row=decision/dioxus-accessibility-feature-by-name area=a11y-scope-summary rec=fct-r6zswm2d@2
name: Dioxus accessibility feature taken only by name
rule: The workspace takes the Dioxus crates without default features, so a crate gets accessibility only by naming it or by depending on a crate that names it.
@row_check n=94 row=decision/dioxus-accessibility-feature-by-name sha=bd14d27635f4 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:54-55
quote: dioxus-native = { version = "0.7.0", path = "./packages/dioxus-native", default-features = false } dioxus-native-dom = { version = "0.7.0", path = "./packages/dioxus-native-dom", default-features = false }
@row_add n=95 row=decision/driver-requires-accessibility area=a11y-scope-summary rec=fct-r6zswm2d@2
name: Session library always enables accessibility
rule: The escher-driver session library enables accessibility on dioxus-native-dom unconditionally, with no feature table of its own, so the feature reaches every binary that depends on it.
why: Its executor reads the snapshot, which stands behind that feature.
@row_check n=95 row=decision/driver-requires-accessibility sha=87e85fad9297 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/Cargo.toml:15
quote: dioxus-native-dom = { workspace = true, features = ["accessibility"] }
@row_add n=96 row=decision/stand-builds-no-platform-adapter area=a11y-scope-summary rec=fct-r6zswm2d@2
name: No platform adapter on the stand binaries
rule: No stand binary builds a platform adapter: neither accesskit_xplat nor accesskit_winit is in the stand's dependency graph, so no assistive technology is given a tree by the stand.
why: Ratified as deferred: the adapter and its windowed witness at an active assistive technology are carried to a later version.
@row_check n=96 row=decision/stand-builds-no-platform-adapter sha=d008d0d0edbb v=holds judge=sonnet head=edd0cfaea7f0
searched: accesskit_xplat|accesskit_winit · in **/Cargo.toml (only blitz-shell's optional dep behind its accessibility feature, the workspace entry and accesskit_xplat itself)
searched: blitz-shell/accessibility · in packages/*/Cargo.toml and examples/seven_guis/Cargo.toml (only dioxus-native's non-default accessibility feature; seven_guis names dioxus-native features "prelude", "system-fonts", "woff" only, and escher-driver names none)
@row_add n=97 row=decision/browser-accessibility-off-by-default area=a11y-scope-summary rec=fct-keyqn7iz@1
name: Browser accessibility feature off by default
rule: The browser app's accessibility feature, dioxus-native accessibility, is not in its default feature set.
@row_check n=97 row=decision/browser-accessibility-off-by-default sha=8de19859aa5b v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:14
quote: default = ["hybrid", "cookies", "cache", "screenshot", "apple-font-embolden", "scrollbars", "mimalloc"]
at: apps/browser/Cargo.toml:39
quote: accessibility = ["dioxus-native/accessibility"]
@row_add n=98 row=decision/accessibility-reach-tier-0 area=a11y-scope-summary rec=fct-fichvpza@1
name: Accessibility reach at tier 0
rule: The published accessibility reach is tier 0: WCAG SC 2.1.1, 1.4.3 and 2.4.3 and no other criterion, with what an assistive technology reads unmeasured.
@row_check n=98 row=decision/accessibility-reach-tier-0 sha=cbaa0c82988e v=holds judge=sonnet head=edd0cfaea7f0
at: docs/capabilities.md:355
quote: The target is tier 0: WCAG SC 2.1.1 (keyboard), SC 1.4.3 (contrast) and SC 2.4.3 (focus order). No other criterion
@row_add n=99 row=decision/capability-statement-reach-by-hand area=a11y-scope-summary rec=fct-fichvpza@1
name: Accessibility reach edited by hand
rule: A change to the accessibility reach is written by hand into the capability statement's Accessibility reach section.
why: The capability statement's pin reads none of the section's claims.
@row_check n=99 row=decision/capability-statement-reach-by-hand sha=cc717f15151d v=holds judge=sonnet head=edd0cfaea7f0
at: docs/capabilities.md:353
quote: ## Accessibility reach
@row_add n=100 row=decision/a11y-tree-refresh-on-change area=a11y-strategy rec=fct-p7eqwwyx@3
name: Accessibility tree refreshed on change
rule: The accessibility tree is built from the document when first requested and rebuilt on a poll whose drained changed set was non-empty.
why: Ratified, one item with the engine's changed-set contract.
guard: stand_accessibility_ids
@row_check n=100 row=decision/a11y-tree-refresh-on-change sha=0c30dbdf09be v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/application.rs:81-82
quote: accesskit_xplat::WindowEvent::InitialTreeRequested => { window.build_accessibility_tree();
at: packages/blitz-shell/src/window.rs:377-380
quote: let changed_nodes = self.doc.inner_mut().take_changed_nodes(); #[cfg(feature = "accessibility")] if !changed_nodes.is_empty() { self.accessibility.update_tree(&*self.doc);
guard_at: tests/blitz-tests/tests/stand_accessibility_ids.rs:1
@row_add n=101 row=decision/a11y-tree-through-document-trait area=a11y-strategy rec=fct-p7eqwwyx@3
name: Accessibility tree through the document trait
rule: Both the initial build and the refresh of the platform accessibility tree go through the document trait's accessibility tree method.
why: So a wrapper's override, such as DioxusDocument's stable ids, reaches the platform tree.
@row_check n=101 row=decision/a11y-tree-through-document-trait sha=6fde8ded39f8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/accessibility.rs:45
quote: self.adapter.update_if_active(|| doc.accessibility_tree());
at: packages/blitz-dom/src/document.rs:219-220
quote: fn accessibility_tree(&self) -> accesskit::TreeUpdate { self.inner().build_accessibility_tree()
@row_add n=102 row=decision/changed-set-marks area=a11y-strategy rec=fct-3hktsq24@1
name: What the engine's changed set marks
rule: The engine's changed set marks a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing; node creation, hover, active, scroll and layout do not.
why: Ratified, one item with the accessibility tree's refresh on change.
@row_check n=102 row=decision/changed-set-marks sha=3934b7aca122 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:393-395
quote: /// Nodes written since the last [`Self::take_changed_nodes`]: a mutation of an in-document /// node, a focus or checked change, a text control's input. Node creation, hover, active, /// scroll and layout do not write it.
at: packages/blitz-dom/src/document.rs:897
quote: self.changed_nodes.insert(list.select);
at: packages/blitz-dom/src/events/keyboard.rs:484
quote: self.changed_nodes.insert(select);
@row_add n=103 row=decision/harness-settle-leaves-changed-set area=a11y-strategy rec=fct-3hktsq24@1
name: Harness settle leaves the changed set
rule: A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first.
why: A settled step leaves the refresh and a later diff exactly what the step marked.
guard: stand_settle
@row_check n=103 row=decision/harness-settle-leaves-changed-set sha=1cad47e1bcfb v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:156-157
quote: /// Settle neither reads nor drains the document's changed set. pub fn settle(&mut self) -> Result<Settled, NotSettled> {
guard_at: tests/blitz-tests/tests/stand_settle.rs:1
@row_add n=104 row=decision/role-mapping-html-aam area=a11y-strategy rec=fct-sla4in4q@1
name: Role mapping by HTML-AAM
rule: Element roles in the accessibility tree are mapped by the HTML-AAM specification.
guard: accessibility_roles
@row_check n=104 row=decision/role-mapping-html-aam sha=6c7c9d7fa494 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:467-469
quote: fn role_from_element_data(element_data: &ElementData) -> Option<Role> { // <https://www.w3.org/TR/html-aam-1.0/> match &*element_data.name.local {
guard_at: tests/blitz-tests/tests/accessibility_roles.rs:1
@row_add n=105 row=decision/aria-states-on-every-element area=a11y-strategy rec=fct-ngdl2hcv@3,fct-wcjja6re@1
name: ARIA states written on every element
rule: The accessibility builder writes the selected, toggled and invalid states from ARIA state attributes on every element, and an element whose role does not support an attribute reads it as one whose role does.
why: Ratified.
guard: accessibility_states
except: A select's option, whose selected state is its chosen state, with no aria-selected read.
@row_check n=105 row=decision/aria-states-on-every-element sha=900e55046c9b v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:186-187
quote: if select::is_select_option(node) { builder.set_selected(select::is_chosen(node));
at: packages/blitz-dom/src/accessibility.rs:230-231
quote: let invalid = element_data.attr(local_name!("aria-invalid")); match invalid.map(str::trim_ascii) {
guard_at: tests/blitz-tests/tests/accessibility_states.rs:142
@row_add n=106 row=decision/aria-state-lower-case-closed-values area=a11y-strategy rec=fct-ngdl2hcv@3
name: ARIA state values matched in lower case
rule: Each ARIA state value is trimmed of ASCII whitespace and compared exactly in lower case against its closed set; an unrecognized value sets nothing, but for aria-invalid, where any value other than absent, empty or false reads true.
why: The lower-case match is a choice, the specification as read stating no case rule; aria-invalid follows WAI-ARIA 1.2's rule.
guard: accessibility_states
@row_check n=106 row=decision/aria-state-lower-case-closed-values sha=ce381ad39dc8 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:190-192
quote: match selected.map(str::trim_ascii) { Some("true") => builder.set_selected(true), Some("false") => builder.set_selected(false),
at: packages/blitz-dom/src/accessibility.rs:232-235
quote: None | Some("" | "false") => {} Some("grammar") => builder.set_invalid(Invalid::Grammar), Some("spelling") => builder.set_invalid(Invalid::Spelling), Some(_) => builder.set_invalid(Invalid::True),
guard_at: tests/blitz-tests/tests/accessibility_states.rs:142
@row_add n=107 row=decision/toggle-read-by-role area=a11y-strategy rec=fct-ngdl2hcv@3,fct-wcjja6re@1
name: Toggle state read per role
rule: The aria-checked attribute sets toggled on the CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles alone, aria-pressed on every other role, and a native checkbox or radio input reads neither.
why: Ratified; follows HTML-AAM and ARIA in HTML.
guard: accessibility_states
@row_check n=107 row=decision/toggle-read-by-role sha=aac2bb4cf0a6 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:214-217
quote: let toggled = if is_checkable(role) { element_data.attr(local_name!("aria-checked")) } else { element_data.attr(local_name!("aria-pressed"))
guard_at: tests/blitz-tests/tests/accessibility_states.rs:142
@row_add n=108 row=decision/native-checkable-toggle area=a11y-strategy rec=fct-7jdlrfnu@1
name: Native checkbox toggle from its checked state
rule: A native checkbox or radio on a checkable role writes toggled true or false from its own checked state, and under any other role writes nothing.
guard: a_click_and_a_space_move_a_native_control_in_the_tree
@row_check n=108 row=decision/native-checkable-toggle sha=c0bcb403382c v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:205-209
quote: if is_native_checkable(element_data) { if is_checkable(role) { match element_data.checkbox_input_checked() { Some(true) => builder.set_toggled(Toggled::True), Some(false) => builder.set_toggled(Toggled::False),
guard_at: tests/blitz-tests/tests/accessibility_states.rs:387
@row_add n=109 row=decision/aria-labelledby-outranks-label area=a11y-strategy rec=fct-4gzpgxiu@1
name: aria-labelledby outranks label and aria-label
rule: The aria-labelledby attribute sets the accessible name from the referenced nodes' text, outranking a label element and aria-label, with aria-label the fallback when the targets hold no text.
why: Upstream's behaviour, kept and ratified (accname 2B).
guard: accessibility_label
@row_check n=109 row=decision/aria-labelledby-outranks-label sha=bda4171e55f5 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:74
quote: builder.clear_labelled_by();
at: packages/blitz-dom/src/accessibility.rs:83-84
quote: if !label.text.is_empty() { builder.set_label(label.text.as_str());
at: packages/blitz-dom/src/accessibility.rs:159-163
quote: if let Some(label) = element_data .attr(local_name!("aria-label")) .filter(|label| !label.trim().is_empty()) { builder.set_label(label);
guard_at: tests/blitz-tests/tests/accessibility_label.rs:1
@row_add n=110 row=decision/aria-description-to-adapter area=a11y-strategy rec=fct-4gzpgxiu@1
name: ARIA description to the platform adapter
rule: The aria-describedby and aria-description attributes reach the platform adapter as the node's description; the snapshot reads no description.
why: Ratified as upstream's behaviour.
guard: accessibility_description
@row_check n=110 row=decision/aria-description-to-adapter sha=40ebb6290c6d v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:98
quote: builder.push_described_by(NodeId(target.as_u64()));
at: packages/blitz-dom/src/accessibility.rs:252-253
quote: if let Some(aria_description) = element_data.attr(local_name!("aria-description")) { builder.set_description(aria_description);
guard_at: tests/blitz-tests/tests/accessibility_description.rs:1
@row_add n=111 row=decision/one-read-per-aria-attribute area=a11y-strategy rec=fct-4gzpgxiu@1
name: One read per ARIA attribute
rule: The accessibility builder reads each ARIA attribute once, keeping the trimmed non-empty aria-label read and the closed-set state reads.
why: Differs from upstream: upstream's boolean-parse aria-selected and unfiltered aria-label reads were dropped in the merge.
@row_check n=111 row=decision/one-read-per-aria-attribute sha=9c9ebf6270f4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:159-161
quote: if let Some(label) = element_data .attr(local_name!("aria-label")) .filter(|label| !label.trim().is_empty())
at: packages/blitz-dom/src/accessibility.rs:189-190
quote: let selected = element_data.attr(local_name!("aria-selected")); match selected.map(str::trim_ascii) {
@row_add n=112 row=decision/agent-readers-behind-accessibility area=a11y-strategy rec=fct-d5bs73pt@4
name: Snapshot readers behind the accessibility feature
rule: The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
@row_check n=112 row=decision/agent-readers-behind-accessibility sha=a5f961d53256 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/lib.rs:18-19
quote: #[cfg(feature = "accessibility")] mod actionable;
at: packages/dioxus-native-dom/src/lib.rs:23-24
quote: #[cfg(feature = "accessibility")] mod graph;
at: packages/dioxus-native-dom/src/lib.rs:30-35
quote: #[cfg(feature = "accessibility")] mod snapshot; #[cfg(feature = "accessibility")] mod snapshot_diff; #[cfg(feature = "accessibility")] mod snapshot_text;
at: packages/dioxus-native-dom/src/lib.rs:28-29
quote: mod reached; mod signal_registry;
@row_add n=113 row=decision/single-select-expanded-from-list area=aria-patterns-roles rec=fct-hkcbpsam@1,fct-4gzpgxiu@1
name: Single select expanded from its list
rule: A single select is a ComboBox that reads no aria-expanded and writes expanded from its list's open state, its chosen option selected.
guard: select_popup
@row_check n=113 row=decision/single-select-expanded-from-list sha=469b16779442 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:516-518
quote: "select" => match element_data.attr(local_name!("multiple")) { Some(_) => Some(Role::ListBox), None => Some(Role::ComboBox),
at: packages/blitz-dom/src/accessibility.rs:240-241
quote: if select::is_single_select(element_data) { builder.set_expanded(self.select_list_for(node.id).is_some());
guard_at: tests/blitz-tests/tests/select_popup.rs:562
@row_add n=114 row=decision/hidden-content-excluded area=aria-patterns-roles rec=fct-x7vrlpec@1
name: Hidden content kept out of the tree
rule: An aria-hidden true node is kept but marked hidden, while hidden, inert, display none and visibility hidden exclude the node and its children from the accessibility tree.
why: Every option of a single select but its chosen one computes display none.
guard: accessibility_hidden
except: A select's option is excepted from display none and stays in the tree; hidden, inert and visibility hidden still exclude it.
@row_check n=114 row=decision/hidden-content-excluded sha=255e657951e0 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:179-180
quote: if element_data.attr(local_name!("aria-hidden")) == Some("true") { builder.set_hidden();
at: packages/blitz-dom/src/accessibility.rs:296-298
quote: (s.styles.is_display_none() && !is_select_option) || s.styles.primary().clone_visibility() == visibility::computed_value::T::Hidden
guard_at: tests/blitz-tests/tests/accessibility_hidden.rs:1
@row_add n=115 row=decision/stand-checks-repeated-three-rounds area=ci-integration rec=fct-3xv5s7f6@1
name: Stand checks repeated three rounds
rule: Every stand_ test file and the named a11y witnesses outside the stand_ prefix run three rounds in the stand-repeat leg under the zero-failure budget.
guard: test_stand_repeat_leg_runs_three_rounds; stand_requirement_sweep
@row_check n=115 row=decision/stand-checks-repeated-three-rounds sha=a48c313a163b v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:53-54
quote: for f in "$SCRIPTS"/../../tests/blitz-tests/tests/stand_*.rs; do stand+=(--test "$(basename "$f" .rs)")
at: .github/scripts/ci-leg.sh:57
quote: stand+=(--test inert_subtree --test accessibility_states --test scrolled_box_absolute_position
at: .github/scripts/ci-leg.sh:59
quote: --test accessibility_active_descendant_unread --test virtual_clock_engine --test select_interaction
guard_at: .github/scripts/test_ci_workflows.py:377
@row_add n=116 row=decision/invalid-state-not-by-class-alone area=cognitive-accessibility rec=fct-d4aqaj2h@1
name: Invalid date state stated beyond its class
rule: The flight booker's invalid date inputs state aria-invalid beside the invalid class and its red styling, and the Book button is disabled, so the error cue does not rest on colour.
why: So the invalid state is in the accessibility tree and the snapshot and not carried by the class alone.
guard: stand_flight_booker; stand_snapshot_state
@row_check n=116 row=decision/invalid-state-not-by-class-alone sha=f2082b2cfe25 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:111
quote: aria_invalid: if start_valid { "false" } else { "true" },
at: examples/seven_guis/src/tasks/flight_booker.rs:112
quote: class: if start_valid { "date-input" } else { "date-input invalid" },
at: examples/seven_guis/src/tasks/flight_booker.rs:119
quote: aria_invalid: if return_valid { "false" } else { "true" },
at: examples/seven_guis/src/tasks/flight_booker.rs:128
quote: disabled: !dates_ok,
guard_at: tests/blitz-tests/tests/stand_flight_booker.rs:14
@row_add n=117 row=decision/stand-shared-tab-order-table area=cognitive-accessibility rec=fct-d4aqaj2h@1
name: Stand focus order asserted against a shared table
rule: The stand asserts each lean task's forward focus sequence against one shared tab-order table kept in the stand checks' shared module.
guard: stand_accessibility_ids
@row_check n=117 row=decision/stand-shared-tab-order-table sha=f5d6175f7141 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/common/mod.rs:49
quote: pub fn tab_order(task: LeanTask) -> &'static [&'static str] {
guard_at: tests/blitz-tests/tests/stand_accessibility_ids.rs:236
@row_add n=118 row=decision/focusable-elements area=keyboard-navigation rec=fct-3kktmzb3@1
name: What is focusable
rule: An element is focusable if it holds a sub-document, or if it is not inert on itself or an ancestor, is no disabled form control, and has tabindex of zero or more or, with no tabindex, is a link or area with href or a button, input, select, textarea, frame, iframe or summary.
why: The narrowing of disabled to form controls and the inert read were ratified.
guard: stand_keyboard
@row_check n=118 row=decision/focusable-elements sha=bdf749e922fe v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:747-749
quote: self.is_focussable = contains_sub_document || (!disabled && match tabindex {
at: packages/blitz-dom/src/node/element.rs:761-770
quote: const DEFAULT_FOCUSSABLE_ELEMENTS: [LocalName; 7] = [ local_name!("button"), local_name!("input"), local_name!("select"), local_name!("textarea"), local_name!("frame"), local_name!("iframe"), local_name!("summary"), ];
at: packages/blitz-dom/src/node/node.rs:574
quote: && !self.is_inert()
guard_at: tests/blitz-tests/tests/stand_keyboard.rs:889
@row_add n=119 row=decision/disabled-by-presence area=keyboard-navigation rec=fct-3kktmzb3@1,fct-37ibnveb@1
name: Disabled read by attribute presence
rule: The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count.
why: The presence reading and its narrowing to form controls were ratified.
guard: focusability_updates; stand_keyboard
@row_check n=119 row=decision/disabled-by-presence sha=0577893e0bcd v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:519
quote: self.can_be_disabled() && self.has_attr(local_name!("disabled"))
at: packages/blitz-dom/src/node/element.rs:513
quote: local_names!("button", "input", "select", "textarea").contains(&self.name.local)
guard_at: tests/blitz-tests/tests/focusability_updates.rs:59
@row_add n=120 row=decision/focus-fixup-reads-inert-alone area=keyboard-navigation rec=fct-z3o2j5z4@1,fct-r67nqedq@1
name: Focus fixup reads inertness alone
rule: A focused node that becomes inert gives focus up at the flush, in every document, with no DOM blur or focusout firing; disabled and hidden take no part, so a focused control that no longer shows keeps focus and takes keys.
why: The limit of reading inertness alone was ratified.
guard: inert_subtree; key_press_after_container_lost_its_box
@row_check n=120 row=decision/focus-fixup-reads-inert-alone sha=8c1d85498e19 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1132
quote: .is_some_and(|node| node.is_inert())
at: packages/blitz-dom/src/mutator.rs:1134
quote: self.doc.clear_focus();
guard_at: tests/blitz-tests/tests/inert_subtree.rs:229
@row_add n=121 row=decision/click-focuses-buttons area=keyboard-navigation rec=fct-6d3ax3jg@1
name: A click focuses a button
rule: A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus.
why: Ratified.
guard: stand_keyboard; stand_act_obstructed
@row_check n=121 row=decision/click-focuses-buttons sha=e0666d68e5fd v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1035-1044
quote: local_name!("input") | local_name!("button") if el.name.local == local_name!("button") || matches!( el.attr(local_name!("type")), Some("submit" | "reset" | "button") ) => { let submits = el.is_submit_button() || el.attr(local_name!("type")) == Some("submit"); click_button(doc, node_id, submits, dispatch_event);
at: packages/blitz-dom/src/events/pointer.rs:1178
quote: if submits && let Some(form_owner) = doc.controls_to_form.get(&node_id) {
at: packages/blitz-dom/src/events/pointer.rs:1094-1095
quote: if !matched { generate_focus_events(doc, &mut |doc| doc.clear_focus(), dispatch_event);
guard_at: tests/blitz-tests/tests/stand_keyboard.rs:850
@row_add n=122 row=decision/disabled-control-stops-click-walk area=keyboard-navigation rec=fct-6d3ax3jg@1
name: A disabled form control stops the click walk
rule: A click on a disabled form control is stopped by the walk before it focuses or acts, while a disabled ancestor div does not stop it.
why: Ratified.
@row_check n=122 row=decision/disabled-control-stops-click-walk sha=925c62498c56 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:832-833
quote: if el.is_disabled() { break 'matched true;
at: packages/blitz-dom/src/node/element.rs:519
quote: self.can_be_disabled() && self.has_attr(local_name!("disabled"))
@row_add n=123 row=decision/double-click-on-document-clock area=keyboard-navigation rec=fct-w426ypc6@1
name: Double click read on the document's clock
rule: Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, a harness-built document using the harness's virtual clock, while each click still focuses its target as one click does.
why: Ratified.
guard: stand_virtual_clock
@row_check n=123 row=decision/double-click-on-document-clock sha=f5a6b68b9222 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:397-404
quote: doc.click_count = if doc .last_mousedown_time .map(|t| doc.elapsed_since(t) < Duration::from_millis(500)) .unwrap_or(false) && (doc.mousedown_position.x - x).abs() <= 2.0 && (doc.mousedown_position.y - y).abs() <= 2.0 {
guard_at: tests/blitz-tests/tests/stand_virtual_clock.rs:117
@row_add n=124 row=decision/range-click-focuses-not-value area=keyboard-navigation rec=fct-tbywmb3f@1
name: Range input click gives focus, not value
rule: A click on a range input, including a label's synthetic click, focuses the input and changes no value.
why: ratified
@row_check n=124 row=decision/range-click-focuses-not-value sha=1e18dac30093 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:842-850
quote: if el.range_input_data().is_some() { generate_focus_events( doc, &mut |doc| { doc.set_focus_to(node_id); }, dispatch_event, ); break 'matched true;
@row_add n=125 row=decision/range-pointerdown-sets-value area=keyboard-navigation rec=fct-tbywmb3f@1
name: Range input pointerdown sets value
rule: A main-button pointerdown on an enabled range input sets its value from the hit's fraction of the content box, clamped to [0, 1], a content box of no width reading 0.
why: ratified
@row_check n=125 row=decision/range-pointerdown-sets-value sha=ede0aa7eab5b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:746-751
quote: if button == MouseEventButton::Main { doc.apply_range_move( target, RangeMove::Fraction(f64::from(fraction)), dispatch_event, );
at: packages/blitz-dom/src/events/pointer.rs:728-735
quote: fn range_track_fraction(layout: &taffy::Layout, x: f32) -> f32 { let width = layout.content_box_width(); let offset = x - (layout.padding.left + layout.border.left); if width > 0.0 { (offset / width).clamp(0.0, 1.0) } else { 0.0 }
@row_add n=126 row=decision/range-input-keys area=keyboard-navigation rec=fct-eom4byip@1
name: Range input keys
rule: A focused enabled range input steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum on Home and maximum on End, every other key doing nothing; each change fires one input event and a move that changes nothing fires none.
why: ratified
@row_check n=126 row=decision/range-input-keys sha=60e9b2657407 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:165-171
quote: let to = match key { Key::Named(NamedKey::ArrowRight) | Key::Named(NamedKey::ArrowUp) => RangeMove::StepUp, Key::Named(NamedKey::ArrowLeft) | Key::Named(NamedKey::ArrowDown) => RangeMove::StepDown, Key::Named(NamedKey::Home) => RangeMove::Minimum, Key::Named(NamedKey::End) => RangeMove::Maximum, _ => return,
at: packages/blitz-dom/src/events/keyboard.rs:543-547
quote: .and_then(|element| element.move_range_input(to)) else { return; }; self.changed_nodes.insert(node_id);
@row_add n=127 row=decision/keyboard-activation-enter-space area=keyboard-navigation rec=fct-wgt25dte@1
name: Enter and Space activate as a click
rule: A pressed Enter on a focused enabled button, summary, submit/reset/button input or link with href, and a released Space on a focused enabled button, summary, or submit/reset/button/checkbox/radio input, dispatch a synthetic Click that runs the app's handlers and then the default click action.
why: ratified; a disabled form control takes neither key
guard: stand_keyboard, stand_act_keys
except: A disabled attribute on a summary or a link gates neither key, as HTML does; a link takes no Space.
@row_check n=127 row=decision/keyboard-activation-enter-space sha=c980dc91b1fe v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:335-341
quote: match element.name.local { local_name!("button") | local_name!("summary") => true, local_name!("input") => matches!( element.attr(local_name!("type")), Some("submit" | "reset" | "button") ), local_name!("a") => element.has_attr(local_name!("href")),
at: packages/blitz-dom/src/events/keyboard.rs:348-357
quote: fn activates_on_space(element: &ElementData) -> bool { if element.is_disabled() { return false; } match element.name.local { local_name!("button") | local_name!("summary") => true, local_name!("input") => matches!( element.attr(local_name!("type")), Some("submit" | "reset" | "button" | "checkbox" | "radio")
at: packages/blitz-dom/src/events/keyboard.rs:370-371
quote: let data = doc.nodes[node_id].synthetic_click_event_data(modifiers); dispatch_event(DomEvent::new(node_id, DomEventData::Click(data)));
guard_at: tests/blitz-tests/tests/stand_keyboard.rs:367
@row_add n=128 row=contract/ui-graph-state/use-graph-state area=keyboard-navigation rec=fct-wfj657us@1
boundary: UI graph state registration
item: use_graph_state
promise: Apps register state for the UI graph with authored DeclaredWrite declarations, each naming an element's stable id or a prefix ending in *, and an event named without on.
@row_check n=128 row=contract/ui-graph-state/use-graph-state sha=aad4ce68519c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:51
quote: pub fn use_graph_state(
at: packages/dioxus-native-dom/src/signal_registry.rs:53
quote: writers: &'static [DeclaredWrite],
at: packages/dioxus-native-dom/src/signal_registry.rs:40-43
quote: match self.id.strip_suffix('*') { Some(prefix) => element.starts_with(prefix), None => element == self.id, }
@row_add n=129 row=contract/ui-graph-state/undeclared area=keyboard-navigation rec=fct-wfj657us@1
boundary: UI graph state registration
item: undeclared
promise: A signal write is flagged undeclared where no declaration names it.
@row_check n=129 row=contract/ui-graph-state/undeclared sha=3c68a3dc3e4c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:222
quote: (false, true) => Some(WriteFlag::Undeclared),
@row_add n=130 row=contract/ui-graph-state/unconfirmed area=keyboard-navigation rec=fct-wfj657us@1
boundary: UI graph state registration
item: unconfirmed
promise: A declared write is flagged unconfirmed only once a dispatch of the handler's event passed through its element and no run wrote the state.
@row_check n=130 row=contract/ui-graph-state/unconfirmed sha=1ddc5e02f7f8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:223
quote: (true, false) if ran(fact.node, event) => Some(WriteFlag::Unconfirmed),
@row_add n=131 row=contract/ui-graph-state/no-registered-state area=keyboard-navigation rec=fct-wfj657us@1
boundary: UI graph state registration
item: no registered state
promise: A page with no registered state reaches no registry, and its events dispatch with no registry recording.
@row_check n=131 row=contract/ui-graph-state/no-registered-state sha=0c6c61ad6b87 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:461-464
quote: let registry = runtime.has_context::<SignalRegistry>(ScopeId::ROOT); if let Some(registry) = &registry { registry.begin_dispatch(chain, event.name()); }
@row_add n=132 row=contract/dioxus-document/record-reached area=keyboard-navigation rec=fct-srvhkkuq@1
boundary: Dioxus document reached-handler record
item: record_reached
promise: Switches on one reached record for the document, a second call keeping the one record; every other document pays one context lookup per dispatch.
@row_check n=132 row=contract/dioxus-document/record-reached sha=0cdc375d45fa v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/reached.rs:89-94
quote: if runtime .has_context::<ReachedRecord>(ScopeId::ROOT) .is_none() { runtime.provide_context(ScopeId::ROOT, ReachedRecord::default()); }
at: packages/dioxus-native-dom/src/dioxus_document.rs:452
quote: if let Some(record) = runtime.has_context::<ReachedRecord>(ScopeId::ROOT) {
@row_add n=133 row=contract/dioxus-document/reached-handlers area=keyboard-navigation rec=fct-srvhkkuq@1
boundary: Dioxus document reached-handler record
item: reached_handlers
promise: Returns the stable id and event name of each node whose listeners held the dispatched event, in first-reached order, deduplicated by id and event, a handler that removes its own element still counting; empty when the record was never switched on.
@row_check n=133 row=contract/dioxus-document/reached-handlers sha=821658f637c0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/reached.rs:99
quote: pub fn reached_handlers(&self) -> Vec<ReachedHandler> {
at: packages/dioxus-native-dom/src/reached.rs:103
quote: .map(|record| record.handlers())
at: packages/dioxus-native-dom/src/reached.rs:67-69
quote: if !reached             .iter()             .any(|handler| handler.id == *id && handler.event == event)
@row_add n=134 row=decision/buffered-dioxus-render area=keyboard-navigation rec=fct-gmhtpdel@3
name: Dioxus render into a mutation buffer
rule: A Dioxus document renders into a mutation buffer with the document unborrowed, then borrows it once and replays the buffer, so a MountedData call in a task spawned from a key handler acts and does not panic.
why: A task first polled inside the render reaches the document as the same handler calling set_focus with no task does.
guard: mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}
@row_check n=134 row=decision/buffered-dioxus-render sha=6d6e97c232bc v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:318-321
quote: self.vdom.render_immediate(&mut self.buffered_mutations); let mut inner = self.inner.borrow_mut(); let mut writer = MutationWriter::new(&mut inner, &mut self.vdom_state); self.buffered_mutations.replay(&mut writer);
guard_at: tests/blitz-tests/tests/dioxus_runtime_probe.rs:607
@row_add n=135 row=decision/falsy-boolean-attributes-removed area=keyboard-navigation rec=fct-2nvw4wcm@1
name: Falsy boolean attributes are removed
rule: A falsy Dioxus value of one of the bridge's 28 boolean attributes removes the attribute on both the dynamic and the static template path, while a truthy value is still written.
why: Uses upstream's own boolean-attribute predicate, the mutation writer kept identical to upstream's.
guard: dioxus_falsy_boolean_attrs, dioxus_falsy_disabled
except: An attribute outside the list holding false, such as aria-hidden or a data- attribute, is still written as "false".
@row_check n=135 row=decision/falsy-boolean-attributes-removed sha=cf6b48ffe9e7 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/mutation_writer.rs:428
quote: if is_falsy && is_bool_attr(local_name) {
at: packages/dioxus-native-dom/src/mutation_writer.rs:385
quote: set_attribute_inner(docm, name, *namespace, Some(value), falsy, node_id);
at: packages/dioxus-native-dom/src/mutation_writer.rs:443-450
quote: fn is_bool_attr(name: &str) -> bool { matches!( name, "allowfullscreen" | "allowpaymentrequest" | "async" | "autofocus" | "autoplay"
guard_at: tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:57
@row_add n=136 row=decision/inert-subtree-excluded area=keyboard-navigation rec=fct-2nvw4wcm@1
name: Inert subtree excluded
rule: A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed.
why: ratified
guard: inert_subtree
@row_check n=136 row=decision/inert-subtree-excluded sha=02676fb5f812 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:585
quote: .is_some_and(|element| element.has_attr(local_name!("inert")))
at: packages/blitz-dom/src/accessibility.rs:289
quote: element.has_attr(local_name!("inert"))
at: packages/dioxus-native-dom/src/mutation_writer.rs:428
quote: if is_falsy && is_bool_attr(local_name) {
guard_at: tests/blitz-tests/tests/inert_subtree.rs:229
@row_add n=137 row=decision/accessible-name-precedence area=screen-reader-support rec=fct-wd4dgp2y@1,fct-dlbh7xpr@1
name: Accessible name precedence
rule: An aria-labelledby whose targets hold text names an element first, then a non-empty trimmed aria-label, then a bound label; a whitespace-only aria-label names nothing.
why: follows accname 1.2 (2B over 2C and 2D)
@row_check n=137 row=decision/accessible-name-precedence sha=9eae8f4cd751 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:83-85
quote: if !label.text.is_empty() { builder.set_label(label.text.as_str()); }
at: packages/blitz-dom/src/accessibility.rs:159-163
quote: if let Some(label) = element_data .attr(local_name!("aria-label")) .filter(|label| !label.trim().is_empty()) { builder.set_label(label);
at: packages/blitz-dom/src/accessibility.rs:58
quote: input.push_labelled_by(NodeId(label_id.as_u64()));
@row_add n=138 row=decision/label-names-its-labelable-element area=screen-reader-support rec=fct-dlbh7xpr@1
name: A label names its bound labelable element
rule: A label names its for-target by id, else its first nested labelable element, whether it stands before or after that element; a hidden input is named by no label.
why: follows HTML's labelable elements: button, non-hidden input, meter, output, progress, select, textarea
@row_check n=138 row=decision/label-names-its-labelable-element sha=ad0b31833aad v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:704
quote: if let Some(target_element_dom_id) = label_element.attr(local_name!("for")) {
at: packages/blitz-dom/src/document.rs:721
quote: TreeTraverser::new_with_root(self, label_node_id)
at: packages/blitz-dom/src/document.rs:3540-3542
quote: local_name!("input") => !element .attr(local_name!("type")) .is_some_and(|kind| kind.eq_ignore_ascii_case("hidden")),
at: packages/blitz-dom/src/accessibility.rs:51
quote: // A label may precede or follow its input, so association waits for every node.
@row_add n=139 row=contract/accessibility-tree/select-role area=screen-reader-support rec=fct-vee7m5at@1
boundary: the accessibility tree
item: select
promise: A select reads ComboBox, a multiple select ListBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes, each selected true or false and named by its non-empty label attribute, else its text.
@row_check n=139 row=contract/accessibility-tree/select-role sha=3432049ec0b3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:517-518
quote: Some(_) => Some(Role::ListBox), None => Some(Role::ComboBox),
at: packages/blitz-dom/src/accessibility.rs:520
quote: "option" => Some(Role::ListBoxOption),
at: packages/blitz-dom/src/accessibility.rs:186-187
quote: if select::is_select_option(node) { builder.set_selected(select::is_chosen(node));
at: packages/blitz-dom/src/accessibility.rs:164-170
quote: } else if (select::is_select_option(node) || element_data.name.local == local_name!("optgroup")) && let Some(label) = element_data .attr(local_name!("label")) .filter(|label| !label.is_empty()) { builder.set_label(label);
@row_add n=140 row=contract/snapshot/select-value area=screen-reader-support rec=fct-vee7m5at@1
boundary: the snapshot
item: a select's value
promise: A select's value is its chosen option's value; for a multiple select its first chosen option's in tree order; nothing when none is chosen.
@row_check n=140 row=contract/snapshot/select-value sha=208242785b39 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:741-744
quote: let option = if select::is_single_select(element) { select::chosen_option(self, select)? } else if select::is_multiple_select(element) { select::first_chosen_option(self, select)?
at: packages/dioxus-native-dom/src/snapshot.rs:280
quote: return doc.select_value(node_id);
@row_add n=141 row=contract/snapshot/closed-select-option-bounds area=screen-reader-support rec=fct-vee7m5at@1
boundary: the snapshot
item: a closed select's options
promise: A closed select's options read under it, each with the select's client rect as its bounds.
@row_check n=141 row=contract/snapshot/closed-select-option-bounds sha=39815c605191 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:159-166
quote: let boxed_by = match closed_select_of(self.doc, node_id) { Some(select) => select, None => node_id, }; let bounds = match self.doc.select_list_bounds(node_id) { Some(bounds) => bounds, None => self.doc.get_client_bounding_rect(boxed_by)?,
@row_add n=142 row=contract/snapshot/open-list-row-bounds area=screen-reader-support rec=fct-vee7m5at@1
boundary: the snapshot
item: an open list's rows
promise: A select whose list is open is no closed select; an open row's bounds is its painted row's in the snapshot and the diff, the diff's differs naming bounds.
@row_check n=142 row=contract/snapshot/open-list-row-bounds sha=4f1239e51370 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:262
quote: (!is_multiple_select(element) && doc.open_select() != Some(select)).then_some(select)
at: packages/dioxus-native-dom/src/snapshot.rs:163
quote: let bounds = match self.doc.select_list_bounds(node_id) {
at: packages/dioxus-native-dom/src/snapshot_diff.rs:209
quote: (Reading::Bounds, was.bounds != node.bounds),
@row_add n=143 row=contract/accessibility-tree/select-expanded area=screen-reader-support rec=fct-kogkylbi@1
boundary: the accessibility tree
item: a single select's expanded
promise: A single select's expanded comes from engine state, never from an author aria-expanded, and focus stays on the select while its list is open.
@row_check n=143 row=contract/accessibility-tree/select-expanded sha=9fe3527bd109 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:240-241
quote: if select::is_single_select(element_data) { builder.set_expanded(self.select_list_for(node.id).is_some());
@row_add n=144 row=contract/snapshot/open-list-highlighted-row area=screen-reader-support rec=fct-axbrqk6r@1
boundary: the snapshot and the accessibility tree
item: an open list's highlighted row
promise: An open list's highlighted row is in no snapshot and no tree, a stated limit: the tree says the list is open, the row shows only in a screenshot.
@row_check n=144 row=contract/snapshot/open-list-highlighted-row sha=8e29530da732 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:240-241
quote: if select::is_single_select(element_data) { builder.set_expanded(self.select_list_for(node.id).is_some());
searched: highlight · in packages/dioxus-native-dom/src
searched: highlight · in packages/blitz-dom/src/accessibility.rs
searched: active_option · in packages/dioxus-native-dom/src
searched: hover · in packages/dioxus-native-dom/src
@row_add n=145 row=contract/snapshot/multiple-select-row-bounds area=screen-reader-support rec=fct-qf4gn4dl@1
boundary: the snapshot
item: a multiple select's rows and groups
promise: A multiple select's rows and groups read their own client rects as their bounds, unlike a closed select's options.
@row_check n=145 row=contract/snapshot/multiple-select-row-bounds sha=9c2807dd0944 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:262
quote: (!is_multiple_select(element) && doc.open_select() != Some(select)).then_some(select)
at: packages/dioxus-native-dom/src/snapshot.rs:165
quote: None => self.doc.get_client_bounding_rect(boxed_by)?,
@row_add n=146 row=contract/form-event-values/multiple-select area=screen-reader-support rec=fct-fzroxaby@1
boundary: form event values
item: a multiple select's set
promise: On Input and Change, evt.values() gives one (name, text value) form entry per chosen option of a multiple select, while value stays the select's single value.
@row_check n=146 row=contract/form-event-values/multiple-select sha=fa42b2e508f9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:424
quote: values: multiple_select_values(&doc.inner(), event.target),
@row_add n=147 row=decision/interactive-nodes-named area=screen-reader-support rec=fct-kf4qrij2@1
name: Interactive nodes carry a name
rule: Every interactive-role tree node of the lean stand tasks reads a non-empty accessible name and an HTML-AAM role, in both layout modes.
guard: stand_accessibility_ids
@row_check n=147 row=decision/interactive-nodes-named sha=1873245e5d21 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_accessibility_ids.rs:270-273
quote: assert!( !name(&nodes, node).trim().is_empty(), "{mode}, {task:?}, tree node {index}: an interactive node is named" );
guard_at: tests/blitz-tests/tests/stand_accessibility_ids.rs:256
@row_add n=148 row=decision/adapter-tree-is-document-tree area=screen-reader-support rec=fct-kf4qrij2@1
name: The adapter reads the document's own tree
rule: The tree the shell hands the platform adapter equals the document's accessibility tree node for node, with the same focus and root.
guard: stand_accessibility_ids
@row_check n=148 row=decision/adapter-tree-is-document-tree sha=cdfd77d8fe33 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/accessibility.rs:45
quote: self.adapter.update_if_active(|| doc.accessibility_tree());
at: packages/dioxus-native-dom/src/dioxus_document.rs:341-342
quote: fn accessibility_tree(&self) -> accesskit::TreeUpdate { let mut tree = self.inner.borrow().build_accessibility_tree();
guard_at: tests/blitz-tests/tests/stand_accessibility_ids.rs:311
@row_add n=149 row=decision/stand-expectation-tables area=screen-reader-support rec=fct-kf4qrij2@1
name: Stand expectations stated once
rule: The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
@row_check n=149 row=decision/stand-expectation-tables sha=2ea37e520172 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/common/mod.rs:17
quote: pub fn controls(task: LeanTask) -> &'static [(&'static str, Role)] {
at: tests/blitz-tests/tests/common/mod.rs:133
quote: pub const INPUT_NAMES: [(&str, &str); 6] = [
at: tests/blitz-tests/tests/common/mod.rs:144
quote: pub const REGION_NAMES: [(&str, &str); 1] = [("crud-list", "People")];
@row_add n=150 row=contract/node-id/reserved-separator area=screen-reader-support rec=fct-73ao4vi3@1
boundary: served node ids
item: the separator ///
promise: No served id holds ///; the form {owner}///{key} is reserved.
@row_check n=150 row=contract/node-id/reserved-separator sha=2f2948c73322 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:46
quote: (!key.is_empty() && !key.contains('/')).then(|| format!("{owner}{VIRTUAL_KEY_SEPARATOR}{key}"))
@row_add n=151 row=decision/snapshot-reads-the-tree area=screen-reader-support rec=fct-ge4n5uhw@1
name: Snapshot takes its readings from the tree
rule: The snapshot takes each node's role, name, focus and its selected, pressed and invalid readings from the accessibility tree, never from the element, with no second role or name mapping.
@row_check n=151 row=decision/snapshot-reads-the-tree sha=20b1ff086d55 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:178-179
quote: role: tree_node.role(), name: self.name(tree_node, &mut HashSet::new()).trim().to_string(),
at: packages/dioxus-native-dom/src/snapshot.rs:192
quote: selected: tree_node.is_selected(),
at: packages/dioxus-native-dom/src/snapshot.rs:193-196
quote: pressed: tree_node .toggled() .filter(|_| !checkable) .map(|toggled| toggled == Toggled::True),
at: packages/dioxus-native-dom/src/snapshot.rs:197
quote: invalid: tree_node.invalid().is_some(),
at: packages/dioxus-native-dom/src/snapshot.rs:198
quote: focused: id == self.focus,
@row_add n=152 row=decision/boolean-state-limits area=screen-reader-support rec=fct-ge4n5uhw@1
name: Limits of the boolean state model
rule: A mixed toggle reads pressed false and a grammar or spelling error reads invalid true, the node keeping its full value.
@row_check n=152 row=decision/boolean-state-limits sha=ebd666d4ca7d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:193-196
quote: pressed: tree_node .toggled() .filter(|_| !checkable) .map(|toggled| toggled == Toggled::True),
at: packages/dioxus-native-dom/src/snapshot.rs:197
quote: invalid: tree_node.invalid().is_some(),
@row_add n=153 row=decision/checked-reading area=screen-reader-support rec=fct-ge4n5uhw@1
name: Source of the checked reading
rule: checked is a native checkbox's or radio's state, else on a checkable role its toggled reading, True true and False and Mixed false, nothing where the role states none.
why: ratified
@row_check n=153 row=decision/checked-reading sha=49f2b6ff4f41 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:184-190
quote: checked: element.checkbox_input_checked().or_else(|| { checkable .then(|| { tree_node.toggled().map(|toggled| toggled == Toggled::True) }) .flatten() }),
@row_add n=154 row=decision/enabled-by-disabled-presence area=screen-reader-support rec=fct-ge4n5uhw@1
name: Enabled follows the disabled attribute's presence
rule: A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading.
guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
@row_check n=154 row=decision/enabled-by-disabled-presence sha=f87c0f65eeb5 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:181-183
quote: enabled: element .can_be_disabled() .then(|| !element.has_attr(local_name!("disabled"))),
at: packages/blitz-dom/src/node/element.rs:518-519
quote: pub fn is_disabled(&self) -> bool { self.can_be_disabled() && self.has_attr(local_name!("disabled"))
guard_at: packages/dioxus-native-dom/src/snapshot.rs:605
@row_add n=155 row=contract/driver-refusal/disabled area=screen-reader-support rec=fct-ge4n5uhw@1
boundary: driver refusal causes
item: disabled
promise: A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.
@row_check n=155 row=contract/driver-refusal/disabled sha=8139f2c4cf52 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:531-532
quote: if target.enabled == Some(false) { return Err(Refusal::new(Cause::Disabled));
@row_add n=156 row=decision/driver-schema-has-no-role-set area=screen-reader-support rec=fct-ge4n5uhw@1
name: Driver schema defines no role or name set
rule: The driver's schema defines no role or name set of its own; role and name are field names of its result nodes.
@row_check n=156 row=decision/driver-schema-has-no-role-set sha=742731099b69 v=holds judge=sonnet head=edd0cfaea7f0
searched: Role|role_from|accessibility_tree · in packages/escher-driver/src/{schema,execute,actions}.rs
searched: enum of roles / role set · in packages/escher-driver/src/schema.rs (only help strings naming aria- attributes at lines 79, 90, 95, 101)
@row_add n=157 row=decision/password-never-revealed area=screen-reader-support rec=fct-ge4n5uhw@1
name: Password text is never revealed
rule: A password input's typed text is in no snapshot field, tree or painted frame: the snapshot reads MASKED_VALUE where it holds text and empty where none, and blitz-paint paints its value as bullets in every renderer.
why: ratified
@row_check n=157 row=decision/password-never-revealed sha=48f2451f228f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
at: packages/blitz-paint/src/render.rs:1030
quote: let bullets: String = text.chars().map(|_| BULLET).collect();
@row_add n=158 row=decision/password-caret-by-bullets area=screen-reader-support rec=fct-ge4n5uhw@1
name: Password caret placed by bullets
rule: A click in a password input places the caret by bullets laid out one per character, so no width of the real characters reaches the caret.
why: ratified
guard: password_caret_click
@row_check n=158 row=decision/password-caret-by-bullets sha=b3f38c815902 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:687-688
quote: let bullets: String = text.chars().map(|_| BULLET).collect(); let Some(layout) = doc.layout_as_text_input(node_id, &bullets) else {
guard_at: tests/blitz-tests/tests/password_caret_click.rs:1
@row_add n=159 row=decision/readers-of-one-snapshot-model area=screen-reader-support rec=fct-ge4n5uhw@1
name: One snapshot model, many readers
rule: The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping.
@row_check n=159 row=decision/readers-of-one-snapshot-model sha=829779459f6f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:199
quote: (Reading::Role, was.role != node.role),
at: packages/dioxus-native-dom/src/snapshot.rs:105-106
quote: pub fn snapshot(&self) -> Snapshot { let tree = self.accessibility_tree();
searched: accessibility_tree|local_name!("aria · in packages/dioxus-native-dom/src/{snapshot_text,snapshot_diff}.rs (none) and Role|snapshot in packages/escher-driver/src/mcp/result.rs (no role mapping)
@row_add n=160 row=contract/snapshot-text/state-tokens area=screen-reader-support rec=fct-ge4n5uhw@1
boundary: the snapshot text form
item: state tokens
promise: Each node is one line; selected or unselected, pressed or unpressed and invalid are written after value= and before focused.
@row_check n=160 row=contract/snapshot-text/state-tokens sha=01136ac87559 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:125-126
quote: if node.state.focused { text.push_str(" focused");
@row_add n=161 row=contract/snapshot-diff/entries area=screen-reader-support rec=fct-ge4n5uhw@1
boundary: the snapshot diff
item: entries
promise: Two snapshots are compared by stable element id, entries carrying the later role and name; a text change is reported on the element whose name it changes, never as a text-run entry.
@row_check n=161 row=contract/snapshot-diff/entries sha=f5ad6d19f76a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:123
quote: match earlier_at.get(node.id.as_str()) {
@row_add n=162 row=contract/snapshot-diff/differs area=screen-reader-support rec=fct-ge4n5uhw@1
boundary: the snapshot diff
item: differs
promise: A changed node names the readings that differ from before the step, in node-key order.
@row_check n=162 row=contract/snapshot-diff/differs sha=c146db7ae2be v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:211-212
quote: .into_iter() .filter_map(|(reading, differs)| differs.then_some(reading))
@row_add n=163 row=contract/snapshot-text/parts area=screen-reader-support rec=fct-ge4n5uhw@1
boundary: the snapshot text form
item: parts
promise: A screen over the text budget is read in parts of consecutive pre-order lines, never a split line, concatenating to the whole text with every node id on exactly one line.
@row_check n=163 row=contract/snapshot-text/parts sha=bec0fa6434f1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:73
quote: if !part.ids.is_empty() && part.text.len() + line.len() > budget {
@row_add n=164 row=contract/mcp-result/snapshot area=screen-reader-support rec=fct-ge4n5uhw@1
boundary: MCP tool results
item: snapshot
promise: An accepted snapshot result is the text form itself for a whole screen, and for a screen in parts the JSON line {text, part, parts}; an acting call's result is its JSON line with the diff.
@row_check n=164 row=contract/mcp-result/snapshot sha=cd8dd4d07b82 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:230
quote: Ok(Outcome::Screen { text }) => ToolResult::accepted(text),
at: packages/escher-driver/src/json.rs:464
quote: object.count("part", u64::from(*part));
@row_add n=165 row=contract/driver-screenshot/element-shot area=screen-reader-support rec=fct-ge4n5uhw@1
boundary: the driver's screenshot
item: an element shot
promise: An element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen.
@row_check n=165 row=contract/driver-screenshot/element-shot sha=858d6f79437e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:569-572
quote: let left = (x.max(seen.x) * scale).floor().max(0.0); let top = (y.max(seen.y) * scale).floor().max(0.0); let right = ((x + width).min(seen.x + seen.width) * scale).ceil(); let bottom = ((y + height).min(seen.y + seen.height) * scale).ceil();
@row_add n=166 row=decision/screenshot-changes-nothing area=screen-reader-support rec=fct-ge4n5uhw@1
name: A screenshot changes nothing
rule: Taking a screenshot moves no focus, leaves has_changes false and leaves the next diff empty.
@row_check n=166 row=decision/screenshot-changes-nothing sha=27d4f624ae96 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:295-297
quote: Command::Screenshot { ref id } => { let shot = match id { None => harness.screenshot(),
@row_add n=167 row=contract/snapshot-node/clickable area=screen-reader-support rec=fct-6nobpd2c@1
boundary: the snapshot node
item: clickable
promise: clickable is true where the element's current listeners hold click and it is no disabled form control; the JSON node always carries it beside focused.
@row_check n=167 row=contract/snapshot-node/clickable sha=f5bdacdcc475 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:199
quote: clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
@row_add n=168 row=contract/snapshot-text/clickable area=screen-reader-support rec=fct-6nobpd2c@1
boundary: the snapshot text form
item: clickable
promise: The token clickable is written after focused only when true.
@row_check n=168 row=contract/snapshot-text/clickable sha=1420fc0ba1c3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:128-129
quote: if node.state.clickable { text.push_str(" clickable");
@row_add n=169 row=decision/ui-graph-carries-no-values area=screen-reader-support rec=fct-gg5bgkx2@1,fct-73ao4vi3@1
name: UI graph is read-only and carries no values
rule: A UI graph read changes nothing and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text.
@row_check n=169 row=decision/ui-graph-carries-no-values sha=5a8ef3169b62 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:138
quote: pub fn graph(&self) -> UiGraph {
at: packages/dioxus-native-dom/src/graph.rs:47
quote: pub reads: Vec<String>,
@row_add n=170 row=contract/handler-coverage/answer area=screen-reader-support rec=fct-ae6kukps@1
boundary: handler coverage
item: answer
promise: The answer lists the graph's handlers no check reached and holds ids, owner ids, event names and two counts, checks and held, only, no role or name.
@row_check n=170 row=contract/handler-coverage/answer sha=38b5b1c62f54 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:437-438
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX)); object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
@row_add n=171 row=decision/actions-verb-changes-nothing area=screen-reader-support rec=fct-vlvcclau@1
name: The actions verb changes nothing
rule: Answering the actions verb moves no focus, snapshot or diff, and an open list stays open.
@row_check n=171 row=decision/actions-verb-changes-nothing sha=490d01e4eab4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:54-55
quote: let (harness, seen) = (&self.harness, &mut self.seen); let screen = harness.doc.snapshot();
@row_add n=172 row=contract/actions-answer/causes area=screen-reader-support rec=fct-vlvcclau@1
boundary: the actions answer
item: causes
promise: For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.
@row_check n=172 row=contract/actions-answer/causes sha=793673be51e1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:82
quote: None => Some(Cause::Stale),
@row_add n=173 row=contract/actions-answer/fields area=screen-reader-support rec=fct-vlvcclau@1
boundary: the actions answer
item: fields
promise: The answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId.
@row_check n=173 row=contract/actions-answer/fields sha=b50f814690df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:259
quote: object.flag("keyed", node.keyed);
@row_add n=174 row=absence/snapshot-list-open-state area=screen-reader-support rec=fct-yfrcl4qo@2
capability: A snapshot reading of whether a list is open (no expanded node key)
@row_check n=174 row=absence/snapshot-list-open-state sha=7ecd667f7dc6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:51
quote: pub const NODE_FIELDS: [NodeFieldSpec; 14] = [
searched: expanded · in packages/dioxus-native-dom/src/**/*.rs
searched: pub expanded · in packages/dioxus-native-dom/src/snapshot.rs (NodeState, lines 47-90, holds no such field)
@row_add n=175 row=decision/option-labels-leave-only-as-pixels area=screen-reader-support rec=fct-ljepgzwi@1
name: Option labels leave an open list only as pixels
rule: An option's label and a group's label leave an open list only as painted pixels in a screenshot and the snapshot's name, with no log site.
why: ratified
@row_check n=175 row=decision/option-labels-leave-only-as-pixels sha=a777b0f0abcb v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: option label log site · in packages/escher-driver/src (no bounded search settles an absent log site)
searched: a_shot_of_a_row_crops_to_it_and_leaves_the_list_open · in tests/blitz-tests/tests/stand_act_popup.rs:288 (the test exists but shows no label channel)
why: no evidence found
@row_add n=176 row=contract/driver-screenshot/row-shot area=screen-reader-support rec=fct-ljepgzwi@1
boundary: the driver's screenshot
item: a shot of an open list's row
promise: A shot of a row crops to it and leaves the list open.
@row_check n=176 row=contract/driver-screenshot/row-shot sha=f870b615a913 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:518
quote: let rect = crop(harness, target).ok_or(Refusal::new(Cause::OffScreen))?;
@row_add n=177 row=contract/accessibility-tree/native-checked area=screen-reader-support rec=fct-nyxiqshg@1
boundary: the accessibility tree
item: a native checkbox's or radio's checked state
promise: On a checkable role toggled reads True or False from the native checked state, no author attribute read on a native control; under any other role a native control writes nothing.
@row_check n=177 row=contract/accessibility-tree/native-checked sha=ddd6e437bf93 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:205-211
quote: if is_native_checkable(element_data) { if is_checkable(role) { match element_data.checkbox_input_checked() { Some(true) => builder.set_toggled(Toggled::True), Some(false) => builder.set_toggled(Toggled::False),
@row_add n=178 row=contract/actionable-nodes/listing area=screen-reader-support rec=fct-xecp6yhu@1
boundary: actionable nodes
item: listing
promise: The list holds, in snapshot pre-order, the UI graph's nodes the actionable rule holds for or whose owner is set, each with id, owner and keyed.
@row_check n=178 row=contract/actionable-nodes/listing sha=c2737480a53f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:132
quote: node.owner.is_some()
@row_add n=179 row=decision/keyed-wire-form area=screen-reader-support rec=fct-wdhdt6pl@1,fct-xecp6yhu@1
name: The keyed field on the carriers
rule: The keyed reading on the actions answer is the actionable-key check's wire form, true where the id holds no slash.
state: provisional
@row_check n=179 row=decision/keyed-wire-form sha=a0819cba2120 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:141
quote: keyed: !node.id.contains('/'),
@row_add n=180 row=decision/listeners-from-vdom-walk area=screen-reader-support rec=fct-cbzuzrms@3,fct-mpysuonn@1
name: Listeners read from the vdom walk
rule: An element's current listeners are read from the vdom walk's dynamic attributes, never from the bridge's listener mark, so a listener removed on a re-render is not counted.
why: the bridge's listener attributes over-report: a removed listener leaves its event attribute and data-dioxus-id mark standing
guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
@row_check n=180 row=decision/listeners-from-vdom-walk sha=7527d12c8abb v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:74
quote: listener: listeners.iter().any(|event| *event != "mounted"),
guard_at: packages/dioxus-native-dom/src/actionable.rs:325
@row_add n=181 row=decision/active-descendant-kept-unread area=screen-reader-support rec=fct-nsojnu3x@1
name: Missing active descendant kept unread on the CRUD list
rule: The CRUD list's rows are not focusable and the accessibility tree's focus stays on the list, never the option; the missing aria-activedescendant is kept unread.
why: ratified
guard: accessibility_active_descendant_unread
@row_check n=181 row=decision/active-descendant-kept-unread sha=96a0e3c18953 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:77-78
quote: node.active_descendant().is_none(), "{mode}: the listbox reads no active descendant"
at: tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:63
quote: tree.focus == NodeId(list.as_u64()),
at: examples/seven_guis/src/tasks/crud.rs:147
quote: role: "option",
guard_at: tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:1
@row_add n=182 row=contract/accessibility-tree/root area=screen-reader-support rec=fct-srqyknt3@1
boundary: the accessibility tree
item: root
promise: The tree root is a Window node with id u64::MAX.
@row_check n=182 row=contract/accessibility-tree/root sha=efe47ff03cc8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:115
quote: nodes.push((NodeId(u64::MAX), window));
at: packages/blitz-dom/src/accessibility.rs:13
quote: let mut window = AccessKitNode::new(Role::Window);
@row_add n=183 row=contract/accessibility-tree/focus area=screen-reader-support rec=fct-srqyknt3@1
boundary: the accessibility tree
item: focus
promise: Tree focus is the focused DOM node only where that node is among the built nodes, else u64::MAX, the window, so the tree never names a node it did not build.
@row_check n=183 row=contract/accessibility-tree/focus sha=bbf82f7e3cd2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:106-109
quote: let focus = self .focus_node_id .filter(|id| nodes.contains_key(id)) .map_or(u64::MAX, |id| id.as_u64());
@row_add n=184 row=decision/accessibility-feature-gate area=screen-reader-support rec=fct-g5whxvnn@1
name: Screen-reader integration behind the accessibility feature
rule: Screen-reader integration through AccessKit is exposed only behind the `accessibility` feature, in blitz via blitz-shell and in the Dioxus crates.
@row_check n=184 row=decision/accessibility-feature-gate sha=21e3687f35e9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz/Cargo.toml:16
quote: accessibility = ["blitz-shell/accessibility"]
at: packages/dioxus-native/src/lib.rs:7
quote: //!  - `accessibility`: Enables [`accesskit`](https://docs.rs/accesskit/latest/accesskit/) accessibility support.
at: packages/dioxus-native-dom/src/lib.rs:7
quote: //!  - `accessibility`: Enables [`accesskit`](https://docs.rs/accesskit/latest/accesskit/) accessibility support,
@row_add n=185 row=decision/wcag-contrast-ratio-helper area=visual-design-verification rec=fct-xkhizlzs@1
name: WCAG contrast ratio matching Chrome
rule: Border bevels and scrollbar thumbs use a WCAG contrast ratio helper that matches Chrome's GetContrastRatio.
@row_check n=185 row=decision/wcag-contrast-ratio-helper sha=b172a3afd437 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/color.rs:29
quote: pub(crate) fn contrast_ratio(a: Color, b: Color) -> f32 {
at: packages/blitz-paint/src/render/border.rs:74
quote: if contrast_ratio(color, dark_color) < MIN_CONTRAST_RATIO {
at: packages/blitz-paint/src/color.rs:59
quote: if contrast_ratio(color, color.lerp_rect(pole, mid)) < target {
@row_add n=186 row=decision/scrollbar-thumb-contrast-stroke area=visual-design-verification rec=fct-234q4bki@1
name: Contrast stroke on default scrollbar thumbs
rule: Default scrollbar thumbs carry a thin contrast stroke.
why: so the thumbs read over same-coloured content
@row_check n=186 row=decision/scrollbar-thumb-contrast-stroke sha=dcbdd19c5ea3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:878-884
quote: if custom_thumb.is_none() { let stroke_width = self.scale; let stroke_rect = rect.inset(-stroke_width / 2.0); scene.stroke( &Stroke::new(stroke_width), self.transform, stroke_color.multiply_alpha(opacity),
@row_add n=187 row=decision/author-thumb-visible-state-change area=visual-design-verification rec=fct-2s3o3j2r@1
name: Near-white author thumb visibly changes state
rule: A near-white author scrollbar thumb must still visibly change on hover and on drag.
@row_check n=187 row=decision/author-thumb-visible-state-change sha=648f8721f4bc v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/scrollbar_drag.rs:274-277
quote: assert_ne!( base, hovered, "a white author thumb must still visibly change on hover" );
at: packages/blitz-paint/src/render.rs:858
quote: Some(base) if is_active => crate::color::blend_for_contrast(base, ACTIVE_CONTRAST),
@row_add n=188 row=decision/text-contrast-required-ratio area=visual-design-verification rec=fct-rwtottqu@1
name: Text contrast required ratio per SC 1.4.3
rule: Text contrast is the WCAG 2.x ratio of the text colour against the background composited source-over down to the first opaque layer, and 3.0 is required of large text (at least 24 px, or 14 pt at weight 700 or more) and 4.5 of other text.
@row_check n=188 row=decision/text-contrast-required-ratio sha=069adc661112 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/contrast.rs:47
quote: self.font_size_px >= 24.0 || (self.font_size_px >= 56.0 / 3.0 && self.font_weight >= 700.0)
at: packages/blitz-test-harness/src/contrast.rs:52
quote: if self.is_large_text() { 3.0 } else { 4.5 }
at: packages/blitz-test-harness/src/contrast.rs:145
quote: .fold(base, |below, layer| over(layer, below)),
@row_add n=189 row=decision/contrast-unread-past-non-colour-paint area=visual-design-verification rec=fct-rwtottqu@1,fct-hhy2npjb@1
name: Contrast unread past non-colour paint
rule: The text contrast reading reads nothing where an element on its walk has opacity below 1, a non-empty filter, a background-image layer or a mix-blend-mode other than normal.
guard: a_blended_ancestor_reads_none_and_an_unblended_one_its_pair
@row_check n=189 row=decision/contrast-unread-past-non-colour-paint sha=1ce316b4dd7b v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/contrast.rs:151
quote: if styles.clone_opacity() < 1.0 || !styles.get_effects().filter.0.is_empty() {
at: packages/blitz-test-harness/src/contrast.rs:154
quote: if doc.resolved_style_value(node.id, "mix-blend-mode") != "normal" {
at: packages/blitz-test-harness/src/contrast.rs:158
quote: Some(images.split(',').any(|layer| layer.trim() != "none"))
guard_at: packages/blitz-test-harness/src/contrast.rs:305
@row_add n=190 row=decision/stand-accent-colour area=visual-design-verification rec=fct-rwtottqu@1
name: Stand accent colour
rule: The stand's accent is rest #3a5ce5, hover #2a4cd3, active #1f3fc1, stand-wide in every task CSS that declares it, a stand change and no engine change.
why: white on the earlier accent #4a6cf7 read 4.39 against 4.5, failing
@row_check n=190 row=decision/stand-accent-colour sha=70c02b8148f5 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:224
quote: background-color: #3a5ce5;
at: examples/seven_guis/src/stand.rs:232
quote: background-color: #2a4cd3;
at: examples/seven_guis/src/stand.rs:236
quote: background-color: #1f3fc1;
@row_add n=191 row=decision/contrast-exempt-disabled-controls area=visual-design-verification rec=fct-rwtottqu@1
name: Disabled controls exempt from contrast
rule: A contrast row is exempt exactly where the snapshot's enabled reads Some(false); a label[for=…] row is never exempt.
@row_check n=191 row=decision/contrast-exempt-disabled-controls sha=b941727edb50 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_contrast.rs:185
quote: Some(contrast) if enabled == Some(Some(false)) => ExemptInactive(measured(contrast)),
at: tests/blitz-tests/tests/stand_contrast.rs:174
quote: let enabled = selector.strip_prefix('#').and_then(|id| {
@row_add n=192 row=decision/stand-contrast-asserted area=visual-design-verification rec=fct-rwtottqu@1,fct-4ahxhbqz@1
name: SC 1.4.3 asserted on the stand
rule: SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg.
guard: stand_contrast
@row_check n=192 row=decision/stand-contrast-asserted sha=9812295563ff v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_contrast.rs:671
quote: fn the_load_page_meets_its_ratios() {
at: tests/blitz-tests/tests/stand_contrast.rs:703
quote: fn cells_meets_its_ratios() {
at: .github/scripts/ci-leg.sh:41
quote: a11y) cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids --test accessibi
guard_at: tests/blitz-tests/tests/stand_contrast.rs:1
@row_add n=193 row=decision/lint-contrast-shares-one-reader area=visual-design-verification rec=fct-67ycdssr@1,fct-pasv6fgo@1
name: Lint contrast shares the harness reader
rule: The visual lint's contrast reading uses the harness's one contrast reader, its required ratio and the stand check's exempt rule, so a blended text's pair is unread by the lint as well.
why: no second formula or threshold
@row_check n=193 row=decision/lint-contrast-shares-one-reader sha=57c938087c14 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:548
quote: let reading = harness.text_contrast(element)?;
at: packages/escher-driver/src/lint.rs:549
quote: let required = reading.required_ratio();
at: packages/escher-driver/src/lint.rs:542
quote: if node.state.enabled == Some(false) {
@row_add n=194 row=contract/lint-contrast-finding/id area=visual-design-verification rec=fct-67ycdssr@1
boundary: the lint's contrast finding
item: id
promise: A contrast finding carries the id of the node read below its required ratio.
@row_check n=194 row=contract/lint-contrast-finding/id sha=482b5aa82629 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:390
quote: object.text("id", &finding.id);
@row_add n=195 row=contract/lint-contrast-finding/defect area=visual-design-verification rec=fct-67ycdssr@1
boundary: the lint's contrast finding
item: defect
promise: A contrast finding carries its defect as a closed word.
@row_check n=195 row=contract/lint-contrast-finding/defect sha=773583020e3d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:391
quote: object.text("defect", finding.defect.name());
@row_add n=196 row=contract/lint-contrast-finding/bounds area=visual-design-verification rec=fct-67ycdssr@1
boundary: the lint's contrast finding
item: bounds
promise: A contrast finding carries the node's bounds as numbers.
@row_check n=196 row=contract/lint-contrast-finding/bounds sha=1bac5117f42d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:392
quote: object.json("bounds", &bounds(finding.bounds));
@row_add n=197 row=contract/lint-contrast-finding/ratio area=visual-design-verification rec=fct-67ycdssr@1
boundary: the lint's contrast finding
item: ratio
promise: A contrast finding carries the measured contrast ratio as a number.
@row_check n=197 row=contract/lint-contrast-finding/ratio sha=c0301ebe3349 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:410
quote: ratio(object.member("ratio"), *measured);
@row_add n=198 row=contract/lint-contrast-finding/required area=visual-design-verification rec=fct-67ycdssr@1
boundary: the lint's contrast finding
item: required
promise: A contrast finding carries the required contrast ratio as a number.
@row_check n=198 row=contract/lint-contrast-finding/required sha=d0dbc27ee921 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:411
quote: ratio(object.member("required"), *required);
@row_add n=199 row=decision/stand-lint-clean-at-boot area=visual-design-verification rec=fct-67ycdssr@1,fct-zlp57bv7@1
name: Stand reads no lint finding at boot
rule: The stand at boot reads no visual lint finding of any kind on the four lean tasks, in both layout modes.
guard: the_stand_at_boot_reads_its_measured_findings
@row_check n=199 row=decision/stand-lint-clean-at-boot sha=aa6ac1c55167 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_lint.rs:338
quote: assert_eq!(LeanTask::ALL.map(|task| at_boot(task).len()), [0, 0, 0, 0]);
guard_at: tests/blitz-tests/tests/stand_lint.rs:337
@row_add n=200 row=decision/highlighted-row-contrast-unread area=visual-design-verification rec=fct-qcoamodi@1
name: Highlighted list row contrast unread by the lint
rule: The visual lint's contrast reads an open list row's plain pair even when the row is highlighted, so a highlighted row's contrast is unread by the lint, a stated limit.
@row_check n=200 row=decision/highlighted-row-contrast-unread sha=69dc8c67a449 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:548
quote: let reading = harness.text_contrast(element)?;
@row_add n=201 row=decision/multiple-select-chosen-row-pair area=visual-design-verification rec=fct-zunxl7qy@1
name: Multiple select chosen-row colour pair
rule: A multiple select's chosen rows read #000000 on #c8c8c8, Chromium's inactive selected pair, hardcoded in the default sheet.
why: hardcoded by design, as the engine holds no tokens
guard: select_interaction
@row_check n=201 row=decision/multiple-select-chosen-row-pair sha=27ed94a99d29 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/assets/default.css:196-197
quote: background-color: #c8c8c8; color: #000000;
guard_at: tests/blitz-tests/tests/select_interaction.rs:708
@row_add n=202 row=decision/select-list-rows-one-height area=visual-design-verification rec=fct-enkqurpr@1
name: Open select list rows take one height
rule: An open select list's rows take one height, the select's font on one line, so a row with a larger authored font clips, a stated limit.
@row_check n=202 row=decision/select-list-rows-one-height sha=70dfeb6daa84 v=holds judge=sonnet head=edd0cfaea7f0
at: docs/capabilities.md:263-264
quote: rows. All rows take one height, measured from the select's font at opening, so a row with a larger authored font clips. Pinned by `select_popup`.
@row_add n=203 row=absence/select-list-motion area=visual-design-verification rec=fct-di5qcazq@1
capability: Motion on an open select's list
@row_check n=203 row=absence/select-list-motion sha=e280f25485d7 v=holds judge=sonnet head=edd0cfaea7f0
searched: animat|transition|fade|easing · in packages/blitz-dom/src/node/select_list.rs
searched: animat|transition|fade|easing · in packages/blitz-paint/src/render/select_list.rs
@row_add n=204 row=decision/hidden-content-excluded-from-a11y-tree area=visual-design-verification rec=fct-lq4s5uty@1
name: Hidden content excluded from the accessibility tree
rule: Nodes with `display: none`, `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot.
except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
@row_check n=204 row=decision/hidden-content-excluded-from-a11y-tree sha=5e36112932ab v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:296
quote: (s.styles.is_display_none() && !is_select_option)
at: packages/blitz-dom/src/accessibility.rs:289
quote: element.has_attr(local_name!("inert"))
at: packages/blitz-dom/src/accessibility.rs:297-298
quote: || s.styles.primary().clone_visibility() == visibility::computed_value::T::Hidden
@row_add n=205 row=decision/inert-element-takes-no-hit area=visual-design-verification rec=fct-lq4s5uty@1
name: An inert element takes no hit
rule: A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit.
why: The `inert` read was ratified.
guard: inert_subtree
@row_check n=205 row=decision/inert-element-takes-no-hit sha=3c9cdd5dabff v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1456
quote: .is_some_and(|element| element.has_attr(local_name!("inert")))
guard_at: tests/blitz-tests/tests/inert_subtree.rs:154
@row_add n=206 row=contract/driver-click-refusals/covered area=visual-design-verification rec=fct-h2b6iawo@1
boundary: Driver click refusals
item: covered
promise: A driver click on an element under an open select list is refused `covered`.
@row_check n=206 row=contract/driver-click-refusals/covered sha=09aa73080abb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:542-543
quote: if covered(harness, target, page) {         return Err(Refusal::new(Cause::Covered));
@row_add n=207 row=decision/a11y-ci-gate area=bootstrap-phases-derive-for-route-setup-project rec=fct-oyb4kdm6@1
name: Accessibility checks gate CI
rule: Accessibility checks run in CI as the `a11y` job.
guard: a11y
@row_check n=207 row=decision/a11y-ci-gate sha=fcb726354529 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:247
quote: a11y:
guard_at: .github/workflows/ci.yml:247
@row_add n=208 row=decision/a11y-ci-leg area=3-a11y-assertion-harness-contract rec=fct-hd6zq7jw@1,fct-3xv5s7f6@1
name: The a11y CI leg
rule: The CI job "Accessibility (a11y) tests" runs the accessibility integration tests as their own leg, on one cargo test line in the slow tier, with no required status check.
why: The existing a11y job was widened rather than a new job, required status check or repository setting added, by the chosen option.
guard: LegScriptTest's a11y tuple
@row_check n=208 row=decision/a11y-ci-leg sha=9a1d3bb0b176 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:248
quote: name: "Accessibility (a11y) tests"
at: .github/scripts/ci-leg.sh:41
quote: a11y) cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids --test accessibility_
guard_at: .github/scripts/test_ci_workflows.py:305
@row_add n=209 row=decision/active-descendant-unread area=5-keyboard-navigation rec=fct-utkipxkl@1,fct-ldzd7n5j@1,fct-26wrao3c@1
name: aria-activedescendant kept unread
rule: The CRUD person rows are listbox options that are not focusable, the tree's focus is the list and never the option, and the tree builder reads no aria-activedescendant, so an assistive technology learns the current row only from its selected reading.
why: SC 4.1.2 work outside the tier-0 target, kept recorded and unread; ratified
guard: accessibility_active_descendant_unread
@row_check n=209 row=decision/active-descendant-unread sha=7b0fd8eb3f4d v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:147
quote: role: "option",
at: tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:40
quote: fn a_focused_listbox_reads_focus_and_no_active_descendant() {
guard_at: tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:40
@row_add n=210 row=decision/select-list-colours-engine-constants area=6-visual-design-verification rec=fct-jalqtol6@1,fct-cjf3ebi4@1
name: Open select list colours are engine constants
rule: An open select list's highlight text, highlight background, disabled text and border colours are engine constants, in no sheet and not tokens, and the harness asserts the list's highlight and disabled rows as painted against them.
why: the engine holds no tokens
guard: select_popup_painted
@row_check n=210 row=decision/select-list-colours-engine-constants sha=7316cadd4b7c v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select_list.rs:113
quote: pub const HIGHLIGHT_TEXT: [u8; 3] = [0x00, 0x00, 0x00];
at: packages/blitz-dom/src/node/select_list.rs:115
quote: pub const HIGHLIGHT_BACKGROUND: [u8; 3] = [0xc8, 0xc8, 0xc8];
at: packages/blitz-dom/src/node/select_list.rs:117
quote: pub const DISABLED_TEXT: [u8; 3] = [0x6d, 0x6d, 0x6d];
at: packages/blitz-dom/src/node/select_list.rs:120
quote: pub const BORDER: [u8; 3] = [0x99, 0x99, 0x99];
guard_at: tests/blitz-tests/tests/select_popup_painted.rs:133
@row_add n=211 row=contract/accessibility-tree/author-id area=7-screen-reader-support rec=fct-iq4fjjyz@1,fct-kf4qrij2@1
boundary: accessibility tree
item: author_id
promise: In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
@row_check n=211 row=contract/accessibility-tree/author-id sha=14121bdddd93 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:349-350
quote: if let Some(id) = ids.remove(&node_id.0) { node.set_author_id(id);
@row_add n=212 row=contract/snapshot/value area=7-screen-reader-support rec=fct-uz4kdss3@2,fct-ge4n5uhw@1
boundary: snapshot
item: value
promise: The snapshot's value is read from the element: the editor's text, else a range input's current value, else the value attribute.
@row_check n=212 row=contract/snapshot/value sha=181ccef8a2de v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:296
quote: Some(text) => Some(text.editor.text().to_string()),
@row_add n=213 row=contract/snapshot/file-input-value area=7-screen-reader-support rec=fct-2hrkecxm@1,fct-ge4n5uhw@1
boundary: snapshot
item: file input value
promise: A file input reads MASKED_VALUE where its value attribute is non-empty, its path in no field of the snapshot.
@row_check n=213 row=contract/snapshot/file-input-value sha=c2bcb3fdc660 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:253
quote: input_type.eq_ignore_ascii_case("password") || input_type.eq_ignore_ascii_case("file")
@row_add n=214 row=decision/actionable-key-rule area=7-screen-reader-support rec=fct-77ewqepn@4,fct-pfylw46r@4
name: Actionable-key rule
rule: Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it.
why: ratified
guard: unkeyed_actionable
@row_check n=214 row=decision/actionable-key-rule sha=185221c73368 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:78-80
quote: fn actionable(&self) -> bool { self.focusable || self.interactive_role || self.listener }
guard_at: packages/dioxus-native-dom/src/actionable.rs:93
@row_add n=215 row=decision/one-actionable-rule area=7-screen-reader-support rec=fct-77ewqepn@4,fct-d5bs73pt@4,fct-pfylw46r@4,fct-s7pqksxc@1
name: One actionable rule
rule: The unkeyed-actionable check and the actionable-nodes read share one crate-private actionable rule, focusable or an interactive role or the vdom walk's exact current listeners holding an event other than mounted, with no second role mapping; the data-dioxus-id mark serves event dispatch alone.
@row_check n=215 row=decision/one-actionable-rule sha=8d1228a558a8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:70-76
quote: fn of(node: &Node, role: Option<Role>, listeners: &[&'static str]) -> Self { Self { focusable: node.is_focussable(), interactive_role: role.is_some_and(is_interactive), listener: listeners.iter().any(|event| *event != "mounted"),
@row_none n=1 rec=fct-hie7hqxu@1 type=S
@row_none n=2 rec=fct-dftqemyf@1 type=S
@row_none n=3 rec=fct-g3zp52x2@1 type=5
@row_none n=4 rec=fct-7lutyqkj@1 type=S
@row_none n=5 rec=fct-6hv3mhaq@1 type=S
@row_none n=6 rec=fct-dsfeza7t@1 type=S
@row_none n=7 rec=fct-xhpyuh5z@1 type=4
relied: A check reading has_changes() as "this step changed something" must drain the changed set first, since a harness settle does not drain it, a fresh harness document's set is non-empty and a pointer move alone marks nothing.
@row_none n=8 rec=fct-e3wnsw77@1 type=S
@row_none n=9 rec=fct-sidyoogt@1 type=4
relied: A node's role is taken from its role attribute, else from the HTML element mapping, else Unknown.
@row_none n=10 rec=fct-amrxvujw@1 type=9
@row_none n=11 rec=fct-thxig5eh@1 type=S
@row_none n=12 rec=fct-5tc4qazb@1 type=S
@row_none n=13 rec=fct-h2nystzi@1 type=S
@row_none n=14 rec=fct-3q6vl62y@1 type=4
@row_none n=15 rec=fct-f775sfwq@1 type=5
@row_none n=16 rec=fct-su6vq775@1 type=5
@row_none n=17 rec=fct-4hu7mg3p@1 type=5
@row_none n=18 rec=fct-fpxbcxc3@1 type=5
@row_none n=19 rec=fct-f4s2gocd@1 type=6
@row_none n=20 rec=fct-iqbzofhd@1 type=5
@row_none n=21 rec=fct-ec5a6ffj@1 type=S
@row_none n=22 rec=fct-mmbkt4gl@1 type=5
@row_none n=23 rec=fct-zvvhy2hs@1 type=5
@row_none n=24 rec=fct-xvapsh73@1 type=S
@row_none n=25 rec=fct-5fqvc7s7@1 type=7
@row_none n=26 rec=fct-mcqrznpq@1 type=7
@row_none n=27 rec=fct-tv46gxfu@1 type=7
@row_none n=28 rec=fct-b76pff7y@1 type=7
@row_none n=29 rec=fct-k44a4wae@1 type=7
@row_none n=30 rec=fct-yslq7j6b@1 type=7
@row_none n=31 rec=fct-osdgo4d4@2 type=S
@row_none n=32 rec=fct-gcw6huup@1 type=S
@row_none n=33 rec=fct-i2b77uw3@1 type=S
@row_none n=34 rec=fct-f7ktjxbm@1 type=S
@row_none n=35 rec=fct-4tdb5mmh@1 type=S
@row_none n=36 rec=fct-em7kuquv@1 type=S
@row_none n=37 rec=fct-ecypmwdz@1 type=S
@row_none n=38 rec=fct-po7msgys@1 type=S
@row_none n=39 rec=fct-2bz7tofn@1 type=4
relied: Focus order relies on an element being focusable only when not inert, not a disabled form control and holding tabindex >= 0 or being a native focusable element, and a focused node coming under inert gives focus up with no blur or focusout.
@row_none n=40 rec=fct-klohodrw@1 type=S
@row_none n=41 rec=fct-7hugl7cl@1 type=4
relied: A pointer click focuses a button, a button-type input or a select, while a click on any other element, such as a CRUD row or the list, clears focus.
@row_none n=42 rec=fct-nnsges7y@1 type=S
@row_none n=43 rec=fct-pbselwmi@1 type=S
@row_none n=44 rec=fct-lytivmcd@1 type=S
@row_none n=45 rec=fct-ub7swfi7@1 type=S
@row_none n=46 rec=fct-3i7trexn@1 type=S
@row_none n=47 rec=fct-xfzzcokf@1 type=S
@row_none n=48 rec=fct-j36rkqej@1 type=5
@row_none n=49 rec=fct-tpqcrpqj@1 type=7
@row_none n=50 rec=fct-l4n3nkb4@1 type=S
@row_none n=51 rec=fct-wyuynhpy@1 type=S
@row_none n=52 rec=fct-2f2lr7nw@1 type=7
@row_none n=53 rec=fct-bbpbx7rl@1 type=S
@row_none n=54 rec=fct-7culk7fl@1 type=S
@row_none n=55 rec=fct-gw6bxgxx@1 type=7
@row_none n=56 rec=fct-jftdtrit@1 type=S
@row_none n=57 rec=fct-u76aco4o@1 type=S
@row_none n=58 rec=fct-y3wp262b@1 type=S
@row_none n=59 rec=fct-xh6al5fm@1 type=S
@row_none n=60 rec=fct-yp634jv2@1 type=S
@row_none n=61 rec=fct-acrc5osz@1 type=S
@row_none n=62 rec=fct-bg7hh3yd@1 type=S
@row_none n=63 rec=fct-oofduxy4@1 type=S
@row_none n=64 rec=fct-yja67mxj@1 type=7
@row_none n=65 rec=fct-pnqtioe5@1 type=S
@row_none n=66 rec=fct-paoqqlkq@1 type=S
@row_none n=67 rec=fct-v3kpbczt@1 type=S
@row_none n=68 rec=fct-aubljpsh@1 type=S
@row_none n=69 rec=fct-wcojmwq5@1 type=S
@row_none n=70 rec=fct-m7tyutvp@1 type=S
@row_none n=71 rec=fct-vowlv6ep@1 type=S
@row_none n=72 rec=fct-o47hlvb7@1 type=S
@row_none n=73 rec=fct-uzpjzwdv@1 type=S
@row_none n=74 rec=fct-7mxmdqi4@1 type=S
@row_none n=75 rec=fct-dzuhkwn2@1 type=4
relied: A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root.
@row_none n=76 rec=fct-giorfan4@1 type=9
@row_none n=77 rec=fct-gwtg4dlh@1 type=4
@row_none n=78 rec=fct-lxyhqmtz@1 type=5
@row_none n=79 rec=fct-ip333e77@1 type=6
@row_none n=80 rec=fct-qp3u343y@1 type=5
@row_none n=81 rec=fct-yqbax437@1 type=7
@row_none n=82 rec=fct-lkb6kitz@1 type=7
@row_none n=83 rec=fct-zvunullx@1 type=7
@row_none n=84 rec=fct-64wjmswb@1 type=7
@row_none n=85 rec=fct-fsmp337h@1 type=9
@row_none n=86 rec=fct-d33ix35a@1 type=4
@row_none n=87 rec=fct-kz6ld5qc@1 type=5
@row_none n=88 rec=fct-dikxlnrr@1 type=4
@row_none n=89 rec=fct-3cjxst5d@1 type=0
what: an upstream warning that the platform adapter crate's approach may not be optimal
@row_none n=90 rec=fct-fvgcbgtf@1 type=4
@row_none n=91 rec=fct-72dwu4ob@1 type=4
@row_none n=92 rec=fct-fklvjzfl@1 type=4
@row_none n=93 rec=fct-ghi42deg@1 type=4
@row_none n=94 rec=fct-lxxlmaur@1 type=4
@row_none n=95 rec=fct-5m2yc5ak@1 type=4
@row_none n=96 rec=fct-twowtdbq@1 type=4
@row_none n=97 rec=fct-4bekc2yb@1 type=4
@row_none n=98 rec=fct-5srxbxx3@1 type=4
@row_none n=99 rec=fct-coaalecn@1 type=4
@row_none n=100 rec=fct-cnvkjgev@1 type=4
@row_none n=101 rec=fct-e4xc55aq@1 type=4
@row_none n=102 rec=fct-jhcfwcrm@1 type=4
@row_none n=103 rec=fct-saem6dco@1 type=4
@row_none n=104 rec=fct-h5c66ocw@1 type=4
@row_none n=105 rec=fct-yf3wxdki@1 type=4
@row_none n=106 rec=fct-5ly4jibl@1 type=5
@row_none n=107 rec=fct-cjlsx75u@1 type=5
@row_none n=108 rec=fct-pnxiuuri@1 type=5
@row_none n=109 rec=fct-qrtfns3v@1 type=5
@row_none n=110 rec=fct-us23gjcc@1 type=5
@row_none n=111 rec=fct-siepbd22@1 type=5
@row_none n=112 rec=fct-ztp5ui7k@1 type=5
@row_none n=113 rec=fct-jp753ryg@1 type=5
@row_none n=114 rec=fct-lepjwtue@1 type=5
@row_none n=115 rec=fct-t7wmqkln@1 type=5
@row_none n=116 rec=fct-5tnfq57l@1 type=5
@row_none n=117 rec=fct-ylek52tj@1 type=5
@row_none n=118 rec=fct-qqa3hbwj@1 type=5
@row_none n=119 rec=fct-gwltxapt@1 type=7
@row_none n=120 rec=fct-qxu7y5sk@1 type=7
@row_none n=121 rec=fct-l5sybnx2@1 type=7
@row_none n=122 rec=fct-iqv74rxa@1 type=7
@row_none n=123 rec=fct-b2lpozik@1 type=7
@row_none n=124 rec=fct-j7rd5ccg@1 type=7
@row_none n=125 rec=fct-j2ypg53m@1 type=7
@row_none n=126 rec=fct-vnfmrv5b@1 type=7
@row_none n=127 rec=fct-bvkme5jk@2 type=6
@row_none n=128 rec=fct-jbqw4l4e@1 type=4
@row_none n=129 rec=fct-cjowbalb@1 type=4
@row_none n=130 rec=fct-pfz6gvm7@1 type=4
@row_none n=131 rec=fct-5djcjety@1 type=4
@row_none n=132 rec=fct-lenizgqm@1 type=4
@row_none n=133 rec=fct-2jvvlu33@1 type=4
@row_none n=134 rec=fct-ymvhhj3q@2 type=9
@row_none n=135 rec=fct-pqyrk5rl@1 type=4
relied: Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events.
@row_none n=136 rec=fct-ag43xb7g@1 type=4
relied: With a select's list open, Tab chooses the current row and closes the list before moving focus, in either direction.
@row_none n=137 rec=fct-yxl65pxf@1 type=4
relied: Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close.
@row_none n=138 rec=fct-is66hfzy@1 type=4
relied: Cached focusability is recomputed when tabindex, href or disabled is set or removed, so a roving tabindex works.
@row_none n=139 rec=fct-cv6ftigg@1 type=4
@row_none n=140 rec=fct-p6maz6zw@1 type=4
@row_none n=141 rec=fct-eu423yer@1 type=4
@row_none n=142 rec=fct-mnypypmc@1 type=4
@row_none n=143 rec=fct-fsgtktln@1 type=4
@row_none n=144 rec=fct-jc7cdake@1 type=4
relied: The :focus-visible selector never matches, while :focus and :focus-within do.
@row_none n=145 rec=fct-7355ytfg@1 type=4
@row_none n=146 rec=fct-3rgd2unt@1 type=4
relied: Removing the focused node resets focus to the body and runs blur side effects.
@row_none n=147 rec=fct-n3cvi4zm@1 type=4
@row_none n=148 rec=fct-t3c72ndi@1 type=4
@row_none n=149 rec=fct-5qucsbnc@1 type=4
@row_none n=150 rec=fct-jl7hn3eh@1 type=4
@row_none n=151 rec=fct-ihhqbxkm@1 type=4
@row_none n=152 rec=fct-odi3bxub@1 type=4
@row_none n=153 rec=fct-po7ckr56@1 type=4
@row_none n=154 rec=fct-7uphfywk@1 type=4
relied: Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control.
@row_none n=155 rec=fct-o3zz2arg@1 type=4
relied: A page's script hears exactly one change per checkable click, from the engine, since the script runtime synthesizes none.
@row_none n=156 rec=fct-7bpmdraf@1 type=4
@row_none n=157 rec=fct-y4ncelk5@1 type=4
relied: Each key change on a single select dispatches one input then one change carrying the select's value, a key that changes nothing firing none.
@row_none n=158 rec=fct-4d5z4pje@1 type=4
@row_none n=159 rec=fct-k6bfqrau@1 type=4
relied: Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing.
@row_none n=160 rec=fct-yf5ymqdh@1 type=4
@row_none n=161 rec=fct-gwp2aynd@1 type=4
@row_none n=162 rec=fct-6pzcqzv4@1 type=4
@row_none n=163 rec=fct-ecdd776h@1 type=4
@row_none n=164 rec=fct-fvogna2h@1 type=4
@row_none n=165 rec=fct-tclwblcz@1 type=4
@row_none n=166 rec=fct-4pxgx3v2@1 type=4
@row_none n=167 rec=fct-lvplfrnk@1 type=4
relied: A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share.
@row_none n=168 rec=fct-5nijysoi@1 type=4
@row_none n=169 rec=fct-6opcw6lo@1 type=4
relied: The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits.
@row_none n=170 rec=fct-br3yxafx@2 type=9
@row_none n=171 rec=fct-v6wecopm@1 type=4
@row_none n=172 rec=fct-xwhfomdm@1 type=4
@row_none n=173 rec=fct-n2djrwjt@1 type=4
@row_none n=174 rec=fct-gp3teo4p@1 type=4
relied: A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it.
@row_none n=175 rec=fct-55wog4z5@1 type=4
relied: A synthetic click is a primary mouse click at the node's center.
@row_none n=176 rec=fct-gnqzcqsq@1 type=4
@row_none n=177 rec=fct-fanke5ok@1 type=4
@row_none n=178 rec=fct-k22537n2@1 type=4
@row_none n=179 rec=fct-6utqi2pl@1 type=4
@row_none n=180 rec=fct-6vmzib72@1 type=4
@row_none n=181 rec=fct-7tv5aotq@1 type=4
@row_none n=182 rec=fct-qnkhv7kk@1 type=4
@row_none n=183 rec=fct-v3jtrgvp@1 type=4
relied: A script's element.style read or write goes through the inline style, so display and visibility reach the tree's hidden-content exclusion.
@row_none n=184 rec=fct-afn7ufjk@1 type=4
@row_none n=185 rec=fct-v4en4yup@1 type=4
@row_none n=186 rec=fct-zo6wthev@1 type=4
@row_none n=187 rec=fct-ws34tgkb@1 type=4
@row_none n=188 rec=fct-yhqlhdr3@2 type=9
@row_none n=189 rec=fct-oiwvy277@1 type=6
@row_none n=190 rec=fct-gdlcgog2@1 type=7
@row_none n=191 rec=fct-7wl6pqbe@1 type=4
@row_none n=192 rec=fct-zl35mqhl@1 type=4
@row_none n=193 rec=fct-4v6yeiau@1 type=4
@row_none n=194 rec=fct-pycsiqbo@1 type=4
@row_none n=195 rec=fct-ny4tl4gt@1 type=4
@row_none n=196 rec=fct-jdm3igmq@1 type=4
@row_none n=197 rec=fct-sw4xryez@1 type=4
@row_none n=198 rec=fct-6luzygdm@1 type=4
@row_none n=199 rec=fct-gqyqow26@1 type=4
@row_none n=200 rec=fct-m565xe5y@1 type=4
@row_none n=201 rec=fct-tc36t6wc@1 type=4
relied: The Android native view is focusable in touch mode and requests focus, so key presses reach the app.
@row_none n=202 rec=fct-do4nkxue@1 type=5
@row_none n=203 rec=fct-4iiwudfe@1 type=5
@row_none n=204 rec=fct-ecjrk6na@1 type=5
@row_none n=205 rec=fct-5o6fv7r5@1 type=5
@row_none n=206 rec=fct-ubjcy5bg@1 type=5
@row_none n=207 rec=fct-4ju6tpsb@1 type=5
@row_none n=208 rec=fct-of6umyfn@1 type=5
@row_none n=209 rec=fct-l52ua2ga@1 type=5
@row_none n=210 rec=fct-2gn3is6h@1 type=5
@row_none n=211 rec=fct-bz6mwbin@1 type=5
@row_none n=212 rec=fct-ovpaig3d@1 type=5
@row_none n=213 rec=fct-g26ezntj@1 type=5
@row_none n=214 rec=fct-w752lxrn@1 type=5
@row_none n=215 rec=fct-pp2bx5ss@1 type=5
@row_none n=216 rec=fct-h7iayydu@1 type=5
@row_none n=217 rec=fct-ssx34oo7@1 type=5
@row_none n=218 rec=fct-5o7fbnn6@1 type=5
@row_none n=219 rec=fct-wlwuhyej@1 type=5
@row_none n=220 rec=fct-ztgj2bii@1 type=7
@row_none n=221 rec=fct-5edvixvz@1 type=7
@row_none n=222 rec=fct-qhaeu42h@1 type=7
@row_none n=223 rec=fct-jn5hbj7a@1 type=4
relied: Text nodes become TextRun nodes carrying their text, and their parent is labelled by them, so an element's text reaches its accessible name.
@row_none n=224 rec=fct-ienybbqr@1 type=5
@row_none n=225 rec=fct-ov5qbtxn@1 type=9
@row_none n=226 rec=fct-iwb2hvdb@1 type=5
@row_none n=227 rec=fct-crtqcqwo@1 type=5
@row_none n=228 rec=fct-ymr6jwcn@1 type=5
@row_none n=229 rec=fct-iaxyko5a@1 type=5
@row_none n=230 rec=fct-7vfy7k5v@2 type=5
@row_none n=231 rec=fct-lqycjcmz@1 type=5
@row_none n=232 rec=fct-pofw2owv@3 type=9
@row_none n=233 rec=fct-6q7odjnr@1 type=5
@row_none n=234 rec=fct-znjf7phu@1 type=4
@row_none n=235 rec=fct-pkfps3f3@1 type=0
what: a test's stated goal, with no rule of its own
@row_none n=236 rec=fct-5vvi2itr@1 type=4
relied: Callers receive no initial tree synchronously; initial-tree requests, action requests and deactivation all reach one EventHandler.
@row_none n=237 rec=fct-n6ofebyd@1 type=4
@row_none n=238 rec=fct-x2vspwsb@1 type=4
@row_none n=239 rec=fct-ixd3ledq@1 type=4
@row_none n=240 rec=fct-d47u24wa@1 type=5
@row_none n=241 rec=fct-wahwuvog@1 type=5
@row_none n=242 rec=fct-q5moqmgi@1 type=7
@row_none n=243 rec=fct-z7atlojp@1 type=5
@row_none n=244 rec=fct-oxkvlnp7@1 type=5
@row_none n=245 rec=fct-jy65uf52@1 type=5
@row_none n=246 rec=fct-7oyxiopr@1 type=5
@row_none n=247 rec=fct-rfdeaqum@1 type=5
@row_none n=248 rec=fct-eosfdylj@1 type=5
@row_none n=249 rec=fct-cylvsgvb@1 type=4
@row_none n=250 rec=fct-fod53y3h@1 type=4
relied: A text input already focused when its text editor is constructed has IME enabled with no fresh focus, in every document.
@row_none n=251 rec=fct-47ieezwj@1 type=4
@row_none n=252 rec=fct-ighwjc7o@1 type=7
@row_none n=253 rec=fct-ma4gqj52@1 type=4
@row_none n=254 rec=fct-6m3lz7h5@1 type=4
@row_none n=255 rec=fct-ja7kgvg6@1 type=5
@row_none n=256 rec=fct-kg2yfrci@1 type=7
@row_none n=257 rec=fct-e4kvi57u@1 type=7
@row_none n=258 rec=fct-nah5qvbw@1 type=5
@row_none n=259 rec=fct-tih7aw5i@1 type=5
@row_none n=260 rec=fct-cwgche4k@1 type=5
@row_none n=261 rec=fct-bxr3ntdl@1 type=5
@row_none n=262 rec=fct-zca62nyc@1 type=4
@row_none n=263 rec=fct-zmloptxd@1 type=5
@row_none n=264 rec=fct-peioehxo@1 type=5
@row_none n=265 rec=fct-6k7yogtz@1 type=4
@row_none n=266 rec=fct-w3yq6a2v@1 type=5
@row_none n=267 rec=fct-uxmsg5j2@1 type=4
@row_none n=268 rec=fct-vtfpkxed@1 type=5
@row_none n=269 rec=fct-h4snawdy@1 type=7
@row_none n=270 rec=fct-5g6o7ztk@1 type=4
@row_none n=271 rec=fct-ajar2qgv@1 type=4
relied: Author styles keyed on `:open` apply to an open details or dialog, `:open` reading the element's open state.
@row_none n=272 rec=fct-jajzvsxm@1 type=4
relied: An open select's list is no `:open` state, so no author style keys on a list standing open.
@row_none n=273 rec=fct-56z4qbio@1 type=4
@row_none n=274 rec=fct-4zqailjn@1 type=4
@row_none n=275 rec=fct-cohcolmo@1 type=0
what: a note on a code comment in an example about an input's color
@row_none n=276 rec=fct-o4ijraeq@1 type=4
relied: The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed.
@row_none n=277 rec=fct-zd6d5frj@1 type=4
relied: A color-scheme change on the viewport is a device change that rebuilds the stylist device.
@row_none n=278 rec=fct-dwhu55j2@1 type=4
relied: A total scale change from hidpi or zoom invalidates text shaping.
@row_none n=279 rec=fct-4afbo2oh@1 type=4
@row_none n=280 rec=fct-lvkpd6n6@1 type=4
@row_none n=281 rec=fct-tfafpxyz@1 type=5
@row_none n=282 rec=fct-co4iopdk@1 type=4
@row_none n=283 rec=fct-rhzrwqyh@1 type=4
@row_none n=284 rec=fct-pvwedula@1 type=4
@row_none n=285 rec=fct-zjxhbn6w@1 type=4
@row_none n=286 rec=fct-3g2s6aqs@1 type=4
@row_none n=287 rec=fct-yvg2oihd@1 type=4
@row_none n=288 rec=fct-anrtdftj@1 type=4
relied: Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first.
@row_none n=289 rec=fct-srrp45jw@1 type=4
@row_none n=290 rec=fct-3usqkvsk@1 type=5
@row_none n=291 rec=fct-ew4vp4xj@1 type=8
@row_none n=292 rec=fct-2looemaf@1 type=S
