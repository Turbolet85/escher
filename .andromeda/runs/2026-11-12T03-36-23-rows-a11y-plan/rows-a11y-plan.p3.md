@row_add n=1 row=decision/a11y-test-leg area=9-ci-integration rec=1
name: Accessibility test leg in the slow tier
rule: The accessibility test files run in the a11y leg of the slow tier behind the fast jobs, with no required status check; the stand-facing ones also run three times in the stand-repeat leg under the zero-failure budget.
guard: ci.yml a11y job
@row_none n=1 rec=2 type=S
@row_none n=2 rec=3 type=S
@row_none n=3 rec=4 type=S
@row_add n=2 row=decision/accesskit-accessibility-tree area=a11y-scope-summary rec=5
name: AccessKit for the accessibility tree
rule: AccessKit is the technology the accessibility tree is built in, taken as a workspace dependency.
@row_add n=3 row=decision/accesskit-xplat-platform-adapter area=a11y-scope-summary rec=6,5
name: accesskit_xplat as the platform adapter
rule: The in-repo accesskit_xplat package, a workspace member, is the AccessKit platform adapter crate.
@row_add n=4 row=decision/accessibility-feature-default-on area=a11y-scope-summary rec=7,10,11,12
name: Accessibility as a default-on feature
rule: The accessibility tree is built only under the accessibility cargo feature, which is on by default in blitz-dom, blitz-shell and blitz and forwarded down that chain; custom widgets require it.
@row_none n=4 rec=8 type=4
relied: A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root.
@row_none n=5 rec=9 type=9
@row_none n=6 rec=13 type=4
@row_add n=5 row=decision/dioxus-accessibility-feature-by-name area=a11y-scope-summary rec=14
name: Dioxus accessibility feature taken only by name
rule: The workspace takes the Dioxus crates without default features, so a crate gets accessibility only by naming it or by depending on a crate that names it.
@row_add n=6 row=decision/driver-requires-accessibility area=a11y-scope-summary rec=14
name: Session library always enables accessibility
rule: The escher-driver session library enables accessibility on dioxus-native-dom unconditionally, with no feature table of its own, so the feature reaches every binary that depends on it.
why: Its executor reads the snapshot, which stands behind that feature.
@row_add n=7 row=decision/stand-builds-no-platform-adapter area=a11y-scope-summary rec=14
name: No platform adapter on the stand binaries
rule: No stand binary builds a platform adapter: neither accesskit_xplat nor accesskit_winit is in the stand's dependency graph, so no assistive technology is given a tree by the stand.
why: Ratified as deferred: the adapter and its windowed witness at an active assistive technology are carried to a later version.
@row_none n=7 rec=15 type=5
@row_add n=8 row=decision/browser-accessibility-off-by-default area=a11y-scope-summary rec=16
name: Browser accessibility feature off by default
rule: The browser app's accessibility feature, dioxus-native accessibility, is not in its default feature set.
@row_none n=8 rec=17 type=6
@row_none n=9 rec=18 type=5
@row_add n=9 row=decision/accessibility-reach-tier-0 area=a11y-scope-summary rec=19
name: Accessibility reach at tier 0
rule: The published accessibility reach is tier 0: WCAG SC 2.1.1, 1.4.3 and 2.4.3 and no other criterion, with what an assistive technology reads unmeasured.
@row_add n=10 row=decision/capability-statement-reach-by-hand area=a11y-scope-summary rec=19
name: Accessibility reach edited by hand
rule: A change to the accessibility reach is written by hand into the capability statement's Accessibility reach section.
why: The capability statement's pin reads none of the section's claims.
@row_none n=10 rec=20 type=7
@row_none n=11 rec=21 type=7
@row_none n=12 rec=22 type=7
@row_none n=13 rec=23 type=7
@row_add n=11 row=decision/a11y-tree-refresh-on-change area=a11y-strategy rec=24
name: Accessibility tree refreshed on change
rule: The accessibility tree is built from the document when first requested and rebuilt on a poll whose drained changed set was non-empty.
why: Ratified, one item with the engine's changed-set contract.
guard: stand_accessibility_ids
@row_add n=12 row=decision/a11y-tree-through-document-trait area=a11y-strategy rec=24
name: Accessibility tree through the document trait
rule: Both the initial build and the refresh of the platform accessibility tree go through the document trait's accessibility tree method.
why: So a wrapper's override, such as DioxusDocument's stable ids, reaches the platform tree.
@row_add n=13 row=decision/changed-set-marks area=a11y-strategy rec=25
name: What the engine's changed set marks
rule: The engine's changed set marks a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing; node creation, hover, active, scroll and layout do not.
why: Ratified, one item with the accessibility tree's refresh on change.
@row_add n=14 row=decision/harness-settle-leaves-changed-set area=a11y-strategy rec=25
name: Harness settle leaves the changed set
rule: A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first.
why: A settled step leaves the refresh and a later diff exactly what the step marked.
guard: stand_settle
@row_none n=14 rec=26 type=9
@row_none n=15 rec=27 type=4
@row_add n=15 row=decision/role-mapping-html-aam area=a11y-strategy rec=28
name: Role mapping by HTML-AAM
rule: Element roles in the accessibility tree are mapped by the HTML-AAM specification.
guard: accessibility_roles
@row_none n=16 rec=29 type=5
@row_add n=16 row=decision/aria-states-on-every-element area=a11y-strategy rec=30,42
name: ARIA states written on every element
rule: The accessibility builder writes the selected, toggled and invalid states from ARIA state attributes on every element, and an element whose role does not support an attribute reads it as one whose role does.
why: Ratified.
guard: accessibility_states
except: A select's option, whose selected state is its chosen state, with no aria-selected read.
@row_add n=17 row=decision/aria-state-lower-case-closed-values area=a11y-strategy rec=30
name: ARIA state values matched in lower case
rule: Each ARIA state value is trimmed of ASCII whitespace and compared exactly in lower case against its closed set; an unrecognized value sets nothing, but for aria-invalid, where any value other than absent, empty or false reads true.
why: The lower-case match is a choice, the specification as read stating no case rule; aria-invalid follows WAI-ARIA 1.2's rule.
guard: accessibility_states
@row_add n=18 row=decision/toggle-read-by-role area=a11y-strategy rec=30,42
name: Toggle state read per role
rule: The aria-checked attribute sets toggled on the CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles alone, aria-pressed on every other role, and a native checkbox or radio input reads neither.
why: Ratified; follows HTML-AAM and ARIA in HTML.
guard: accessibility_states
@row_add n=19 row=decision/native-checkable-toggle area=a11y-strategy rec=31
name: Native checkbox toggle from its checked state
rule: A native checkbox or radio on a checkable role writes toggled true or false from its own checked state, and under any other role writes nothing.
guard: a_click_and_a_space_move_a_native_control_in_the_tree
@row_add n=20 row=decision/aria-labelledby-outranks-label area=a11y-strategy rec=32
name: aria-labelledby outranks label and aria-label
rule: The aria-labelledby attribute sets the accessible name from the referenced nodes' text, outranking a label element and aria-label, with aria-label the fallback when the targets hold no text.
why: Upstream's behaviour, kept and ratified (accname 2B).
guard: accessibility_label
@row_add n=21 row=decision/aria-description-to-adapter area=a11y-strategy rec=32
name: ARIA description to the platform adapter
rule: The aria-describedby and aria-description attributes reach the platform adapter as the node's description; the snapshot reads no description.
why: Ratified as upstream's behaviour.
guard: accessibility_description
@row_add n=22 row=decision/one-read-per-aria-attribute area=a11y-strategy rec=32
name: One read per ARIA attribute
rule: The accessibility builder reads each ARIA attribute once, keeping the trimmed non-empty aria-label read and the closed-set state reads.
why: Differs from upstream: upstream's boolean-parse aria-selected and unfiltered aria-label reads were dropped in the merge.
@row_none n=17 rec=33 type=4
@row_none n=18 rec=34 type=0
what: an upstream warning that the platform adapter crate's approach may not be optimal
@row_none n=19 rec=35 type=4
@row_none n=20 rec=36 type=4
@row_none n=21 rec=37 type=4
@row_add n=23 row=decision/agent-readers-behind-accessibility area=a11y-strategy rec=38
name: Snapshot readers behind the accessibility feature
rule: The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
@row_add n=24 row=decision/one-actionable-rule area=a11y-strategy rec=38
name: One actionable rule
rule: The actionable-node reader and the unkeyed-actionable check share one crate-private actionable rule.
@row_none n=22 rec=39 type=4
@row_none n=23 rec=40 type=4
@row_none n=24 rec=41 type=4
@row_none n=25 rec=43 type=4
@row_none n=26 rec=44 type=4
@row_none n=27 rec=45 type=4
@row_add n=25 row=decision/single-select-expanded-from-list area=aria-patterns-roles rec=46,32
name: Single select expanded from its list
rule: A single select is a ComboBox that reads no aria-expanded and writes expanded from its list's open state, its chosen option selected.
guard: select_popup
@row_none n=28 rec=47 type=4
@row_none n=29 rec=48 type=4
@row_none n=30 rec=49 type=4
@row_none n=31 rec=50 type=4
@row_none n=32 rec=51 type=4
@row_none n=33 rec=52 type=4
@row_add n=26 row=decision/hidden-content-excluded area=aria-patterns-roles rec=53
name: Hidden content kept out of the tree
rule: An aria-hidden true node is kept but marked hidden, while hidden, inert, display none and visibility hidden exclude the node and its children from the accessibility tree.
why: Every option of a single select but its chosen one computes display none.
guard: accessibility_hidden
except: A select's option is excepted from display none and stays in the tree; hidden, inert and visibility hidden still exclude it.
@row_none n=34 rec=54 type=4
@row_none n=35 rec=55 type=5
