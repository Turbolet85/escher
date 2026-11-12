@row_add n=1 row=decision/a11y-tier-0 area=1-a11y-scope-summary rec=1
name: Accessibility tier 0
rule: The accessibility tier is 0, covering WCAG success criteria 2.1.1, 1.4.3 and 2.4.3.
@row_none n=1 rec=2 type=S
@row_add n=2 row=absence/assistive-technology-reach area=1-a11y-scope-summary rec=3
capability: A platform accessibility adapter in any stand binary, so no active assistive technology is given a tree
@row_none n=2 rec=4 type=S
@row_none n=3 rec=5 type=5
@row_none n=4 rec=6 type=S
@row_none n=5 rec=7 type=S
@row_none n=6 rec=8 type=S
@row_add n=3 row=decision/accessibility-tree-through-document-method area=2-a11y-strategy rec=9
name: Accessibility tree built through the document's own method
rule: The accessibility tree is built from the document on the initial tree request and rebuilt when a poll takes a non-empty changed set, both paths going through the document trait's accessibility-tree method.
why: so a wrapper's override, such as DioxusDocument's stable ids, reaches the tree
@row_none n=7 rec=10 type=4
relied: A check reading has_changes() as "this step changed something" must drain the changed set first, since a harness settle does not drain it, a fresh harness document's set is non-empty and a pointer move alone marks nothing.
@row_none n=8 rec=11 type=S
@row_none n=9 rec=12 type=4
relied: A node's role is taken from its role attribute, else from the HTML element mapping, else Unknown.
@row_none n=10 rec=13 type=9
@row_add n=4 row=contract/accessibility-tree-states/aria-selected area=2-a11y-strategy rec=14,44
boundary: Accessibility tree states
item: aria-selected
promise: Trimmed and matched in lower case, aria-selected writes the node's selected state.
@row_add n=5 row=contract/accessibility-tree-states/aria-pressed area=2-a11y-strategy rec=14,44
boundary: Accessibility tree states
item: aria-pressed
promise: Trimmed and matched in lower case, aria-pressed writes the node's toggled state on every role but the five checkable roles.
@row_add n=6 row=contract/accessibility-tree-states/aria-checked area=2-a11y-strategy rec=14,44
boundary: Accessibility tree states
item: aria-checked
promise: Trimmed and matched in lower case, aria-checked writes the node's toggled state on the five checkable roles alone.
@row_add n=7 row=contract/accessibility-tree-states/aria-invalid area=2-a11y-strategy rec=14,44
boundary: Accessibility tree states
item: aria-invalid
promise: Trimmed and matched in lower case, aria-invalid writes the node's invalid state.
@row_add n=8 row=contract/accessibility-tree-states/select-option area=2-a11y-strategy rec=14
boundary: Accessibility tree states
item: select option
promise: A select's option reads its chosen state into the tree.
@row_add n=9 row=contract/accessibility-tree-states/native-checkbox-or-radio area=2-a11y-strategy rec=14
boundary: Accessibility tree states
item: native checkbox or radio
promise: A native checkbox or radio under a checkable role writes toggled from its own checked state, reading neither aria-checked nor aria-pressed; under any other role it writes nothing.
@row_add n=10 row=contract/aria-property-reads/aria-labelledby area=2-a11y-strategy rec=15
boundary: ARIA name and property reads
item: aria-labelledby
promise: The referenced targets' text becomes the node's label, outranking a <label> and an aria-label (accname step 2B).
@row_add n=11 row=contract/aria-property-reads/aria-describedby area=2-a11y-strategy rec=15
boundary: ARIA name and property reads
item: aria-describedby
promise: The builder reads aria-describedby, one read per attribute, as a source of the node's description.
@row_add n=12 row=contract/aria-property-reads/aria-description area=2-a11y-strategy rec=15
boundary: ARIA name and property reads
item: aria-description
promise: The builder reads aria-description, one read per attribute, as the node's description.
@row_add n=13 row=contract/aria-property-reads/aria-expanded area=2-a11y-strategy rec=15,25
boundary: ARIA name and property reads
item: aria-expanded
promise: aria-expanded is read as true or false and nothing else on every element but a single select, which writes its list's open state instead and reads no author attribute.
@row_add n=14 row=contract/aria-property-reads/aria-disabled area=2-a11y-strategy rec=15,25
boundary: ARIA name and property reads
item: aria-disabled
promise: Only the value true of aria-disabled sets the node disabled.
@row_add n=15 row=contract/aria-property-reads/aria-level area=2-a11y-strategy rec=15,25
boundary: ARIA name and property reads
item: aria-level
promise: aria-level is read as its number less one, headings keeping their default levels.
@row_none n=11 rec=16 type=S
@row_none n=12 rec=17 type=S
@row_none n=13 rec=18 type=S
@row_add n=16 row=decision/a11y-assertions-as-rust-tests area=3-a11y-assertion-harness-contract rec=19
name: Accessibility assertions as Rust tests over AccessKit
rule: Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
@row_none n=14 rec=20 type=4
@row_none n=15 rec=21 type=5
@row_none n=16 rec=22 type=5
@row_none n=17 rec=23 type=5
@row_none n=18 rec=24 type=5
@row_none n=19 rec=26 type=6
@row_none n=20 rec=27 type=5
@row_none n=21 rec=28 type=S
@row_none n=22 rec=29 type=5
@row_none n=23 rec=30 type=5
@row_add n=17 row=decision/wcag-contrast-thresholds area=3-a11y-assertion-harness-contract rec=31
name: WCAG 2.x text contrast thresholds
rule: Text contrast is measured by the WCAG 2.x formula from computed styles and must reach 4.5, or 3.0 for large text.
guard: stand_contrast
@row_add n=18 row=decision/a11y-ci-leg area=3-a11y-assertion-harness-contract rec=32
name: Accessibility tests as their own CI leg
rule: The accessibility integration tests run as their own CI leg.
guard: ci.yml a11y job
@row_none n=24 rec=33 type=S
@row_none n=25 rec=34 type=7
@row_none n=26 rec=35 type=7
@row_none n=27 rec=36 type=7
@row_none n=28 rec=37 type=7
@row_none n=29 rec=38 type=7
@row_none n=30 rec=39 type=7
@row_none n=31 rec=40 type=S
@row_none n=32 rec=41 type=S
@row_add n=19 row=decision/html-aam-role-mapping area=4-aria-patterns-roles rec=42
name: Native element roles follow HTML-AAM
rule: Native HTML elements map to accessibility roles as HTML-AAM states: landmarks, headings, lists, tables, interactive and inline semantics.
@row_none n=33 rec=43 type=S
@row_add n=20 row=contract/aria-role-mapping/role-switch area=4-aria-patterns-roles rec=44
boundary: ARIA role mapping
item: role="switch"
promise: role="switch" maps to the Switch role.
@row_add n=21 row=contract/accessibility-tree-hidden-content/aria-hidden area=4-aria-patterns-roles rec=45
boundary: Accessibility tree hidden content
item: aria-hidden="true"
promise: A node with aria-hidden="true" is kept in the tree, marked hidden.
@row_add n=22 row=contract/accessibility-tree-hidden-content/hidden area=4-aria-patterns-roles rec=45
boundary: Accessibility tree hidden content
item: hidden
promise: The hidden attribute excludes the node and its children from the tree.
@row_add n=23 row=contract/accessibility-tree-hidden-content/display-none area=4-aria-patterns-roles rec=45
boundary: Accessibility tree hidden content
item: display: none
promise: display: none excludes the node and its children from the tree, except an option under a select.
@row_add n=24 row=contract/accessibility-tree-hidden-content/visibility-hidden area=4-aria-patterns-roles rec=45
boundary: Accessibility tree hidden content
item: visibility: hidden
promise: visibility: hidden excludes the node and its children from the tree.
@row_none n=34 rec=46 type=S
@row_none n=35 rec=47 type=S
@row_none n=36 rec=48 type=S
@row_none n=37 rec=49 type=S
@row_none n=38 rec=50 type=S
@row_none n=39 rec=51 type=4
relied: Focus order relies on an element being focusable only when not inert, not a disabled form control and holding tabindex >= 0 or being a native focusable element, and a focused node coming under inert gives focus up with no blur or focusout.
@row_add n=25 row=decision/disabled-by-attribute-presence area=5-keyboard-navigation rec=52
name: Disabled-ness keyed one way by attribute presence
rule: On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
@row_none n=40 rec=53 type=S
@row_none n=41 rec=54 type=4
relied: A pointer click focuses a button, a button-type input or a select, while a click on any other element, such as a CRUD row or the list, clears focus.
@row_add n=26 row=decision/double-click-threshold area=5-keyboard-navigation rec=55
name: Double click threshold
rule: Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, each still focusing its target as one click does.
