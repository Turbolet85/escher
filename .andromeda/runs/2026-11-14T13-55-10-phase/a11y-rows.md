# a11y-plan · the rows that bind 2026-11-14-auto-exploration — picked 79 of 240 (61 by every draw) · rule any · pin 922

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/driver-requires-accessibility · Session library always enables accessibility — The escher-driver session library enables accessibility on dioxus-native-dom unconditionally, with no feature table of its own, so the feature reaches every binary that depends on it. · why: Its executor reads the snapshot, which stands behind that feature.
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered
decision/dioxus-accessibility-feature-by-name · Dioxus accessibility feature taken only by name — The workspace takes the Dioxus crates without default features, so a crate gets accessibility only by naming it or by depending on a crate that names it.

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
decision/accessibility-tree-through-document-method · Accessibility tree built through the document's own method — The accessibility tree is built from the document on the initial tree request and rebuilt when a poll takes a non-empty changed set, both paths going through the document trait's accessibility-tree method. · why: so a wrapper's override, such as DioxusDocument's stable ids, reaches the tree
contract/aria-property-reads/aria-disabled · ARIA name and property reads · aria-disabled — Only the value true of aria-disabled sets the node disabled.
contract/aria-property-reads/aria-expanded · ARIA name and property reads · aria-expanded — aria-expanded is read as true or false and nothing else on every element but a single select, which writes its list's open state instead and reads no author attribute.

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/a11y-tree-refresh-on-change · Accessibility tree refreshed on change — The accessibility tree is built from the document when first requested and rebuilt on a poll whose drained changed set was non-empty. · why: Ratified, one item with the engine's changed-set contract. · guard: stand_accessibility_ids
decision/a11y-tree-through-document-trait · Accessibility tree through the document trait — Both the initial build and the refresh of the platform accessibility tree go through the document trait's accessibility tree method. · why: So a wrapper's override, such as DioxusDocument's stable ids, reaches the platform tree.
decision/agent-readers-behind-accessibility · Snapshot readers behind the accessibility feature — The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
decision/changed-set-marks · What the engine's changed set marks — The engine's changed set marks a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing, a scroll that closes a list included; node creation, hover, active, any other scroll and layout do not. · why: Ratified, one item with the accessibility tree's refresh on change.
decision/harness-settle-leaves-changed-set · Harness settle leaves the changed set — A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first. · why: A settled step leaves the refresh and a later diff exactly what the step marked. · guard: stand_settle
decision/role-mapping-html-aam · Role mapping by HTML-AAM — Element roles in the accessibility tree are mapped by the HTML-AAM specification. · guard: accessibility_roles

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
contract/accessibility-tree-hidden-content/display-none · Accessibility tree hidden content · display: none — display: none excludes the node and its children from the tree, except an option under a select.
decision/html-aam-role-mapping · Native element roles follow HTML-AAM — Native HTML elements map to accessibility roles as HTML-AAM states: landmarks, headings, lists, tables, interactive and inline semantics.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/hidden-content-excluded · Hidden content kept out of the tree — An aria-hidden true node is kept but marked hidden, while hidden, inert, display none and visibility hidden exclude the node and its children from the accessibility tree. · why: Every option of a single select but its chosen one computes display none. · guard: accessibility_hidden · except: A select's option is excepted from display none and stays in the tree; hidden, inert and visibility hidden still exclude it.
decision/single-select-expanded-from-list · Single select expanded from its list — A single select is a ComboBox that reads no aria-expanded and writes expanded from its list's open state, its chosen option selected. · guard: select_popup

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
decision/disabled-by-attribute-presence · Disabled-ness keyed one way by attribute presence — On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
decision/falsy-boolean-attribute-removed · Falsy boolean attribute removed — A falsy Dioxus value of one of the bridge's boolean attributes (upstream's is_bool_attr set) removes the attribute on both attribute paths; a truthy value is written, and an attribute outside the list holding false is written as "false". · why: So an enabled control does not match :disabled and hidden: false keeps an element laid out and in the tree.
contract/select-closed-list/enter-space · single-choice select, list closed · Enter / Space — Enter, and a Space with no typed text standing, open a focused single-choice select's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select opens nothing.
contract/single-select/optgroup-options · single-choice select · optgroup options — An optgroup's options are the select's own options, in tree order.
contract/keyboard-activation/enter · keyboard activation · Enter — A pressed Enter on the focused node dispatches a Click when the node is no disabled form control and is a button, a summary, an input of type submit, reset or button, or an a with href.
contract/keyboard-activation/space · keyboard activation · Space — A released Space dispatches a Click for a summary, or for a button or an input of type submit, reset, button, checkbox or radio that is no disabled form control, never for a link.
contract/select-open-list/navigation-keys · single-choice select, list open · navigation keys — With a select's list open, the engine paints the list over the page, focus stays on the select, and the select's keys move the highlight over enabled rows, choosing nothing.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/click-focuses-buttons · A click focuses a button — A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus. · why: Ratified. · guard: stand_keyboard; stand_act_obstructed
decision/disabled-by-presence · Disabled read by attribute presence — The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count. · why: The presence reading and its narrowing to form controls were ratified. · guard: focusability_updates; stand_keyboard
decision/disabled-control-stops-click-walk · A disabled form control stops the click walk — A click on a disabled form control is stopped by the walk before it focuses or acts, while a disabled ancestor div does not stop it. · why: Ratified.
decision/disabled-optgroup-one-rule · Disabled optgroup rows read disabled by one rule — A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share. · why: form entry building, the list's row readings and key choice all call option_disabled; pinned by a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen
decision/falsy-boolean-attributes-removed · Falsy boolean attributes are removed — A falsy Dioxus value of one of the bridge's 28 boolean attributes removes the attribute on both the dynamic and the static template path, while a truthy value is still written. · why: Uses upstream's own boolean-attribute predicate, the mutation writer kept identical to upstream's. · guard: dioxus_falsy_boolean_attrs, dioxus_falsy_disabled · except: An attribute outside the list holding false, such as aria-hidden or a data- attribute, is still written as "false".
decision/focus-stays-on-select-with-list-open · Focus stays on a select while its list is open — Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close. · why: pinned by the_tree_reads_the_list_open_on_the_select_its_focus_staying_there and escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box
decision/inert-subtree-excluded · Inert subtree excluded — A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed. · why: ratified · guard: inert_subtree
decision/checkable-click-toggles-and-focuses · Checkbox or radio click toggles, fires input, focuses — Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control. · why: pinned by a_click_and_a_space_on_a_checkbox_each_fire_input_then_change; the stand's checkable controls and the driver's click rely on it
decision/driver-type-focuses-select-no-click · Driver type focuses a select with no click — The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. · why: pinned by type_leaves_no_list_open_and_chooses_by_its_text in the stand_act_popup driver tests
decision/focusable-elements · What is focusable — An element is focusable if it holds a sub-document, or if it is not inert on itself or an ancestor, is no disabled form control, and has tabindex of zero or more or, with no tabindex, is a link or area with href or a button, input, select, textarea, frame, iframe or summary. · why: The narrowing of disabled to form controls and the inert read were ratified. · guard: stand_keyboard
decision/keyboard-activation-enter-space · Enter and Space activate as a click — A pressed Enter on a focused enabled button, summary, submit/reset/button input or link with href, and a released Space on a focused enabled button, summary, or submit/reset/button/checkbox/radio input, dispatch a synthetic Click that runs the app's handlers and then the default click action. · why: ratified; a disabled form control takes neither key · guard: stand_keyboard, stand_act_keys · except: A disabled attribute on a summary or a link gates neither key, as HTML does; a link takes no Space.
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
contract/accessibility-tree/hidden-and-inert-nodes · accessibility tree · hidden and inert nodes — Nodes with display: none or visibility: hidden or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.
contract/hit-testing/inert · hit-testing · inert — A hit passes through an element carrying inert, and removing inert restores the tree and the hit.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/hidden-content-excluded-from-a11y-tree · Hidden content excluded from the accessibility tree — Nodes with `display: none`, `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot. · except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
decision/inert-element-takes-no-hit · An inert element takes no hit — A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit. · why: The `inert` read was ratified. · guard: inert_subtree
contract/driver-click-refusals/covered · Driver click refusals · covered — A driver click on an element under an open select list is refused `covered`.
decision/open-select-list-not-open-state · Open select list sets no :open state — An open select's list is no `:open` state, so no author style keys on a list standing open. · why: pinned by an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match in the select_popup tests

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/actionable-key-rule · Actionable-key rule — Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it. · why: ratified · guard: unkeyed_actionable
decision/one-actionable-rule · One actionable rule — The unkeyed-actionable check and the actionable-nodes read share one crate-private actionable rule, focusable or an interactive role or the vdom walk's exact current listeners holding an event other than mounted, with no second role mapping; the data-dioxus-id mark serves event dispatch alone.
decision/password-value-masked · Password value masked — A password input's typed text appears in no snapshot field, tree node or painted frame; its value reads a fixed masked marker where it holds text and an empty string where it holds none.
decision/snapshot-model-single-source · Snapshot model as single source — The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else.
decision/snapshot-reads-accessibility-tree · Snapshot reads the accessibility tree — The snapshot takes each node's role, name, focus, selected, pressed and invalid from the accessibility tree, never from the element.
decision/ui-graph-one-node-per-snapshot-node · UI graph mirrors the snapshot — The read-only UI graph holds one node per snapshot node, names and closed words only.
contract/accessibility-tree/author-id · accessibility tree · author_id — In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
contract/accessibility-tree/single-select · accessibility tree · single select — A single select reads ComboBox and writes expanded from its list's open state, reading no aria-expanded; an author's open or aria-expanded on it steers nothing; its chosen option reads selected and the tree's focus stays on the select.
contract/driver-actions/disabled · driver actions · disabled — The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.
contract/snapshot/clickable · snapshot · clickable — The snapshot's clickable field says an element that is no disabled form control reacts to a click, read from the vdom walk's current click listeners and not the listener mark.
contract/snapshot/enabled · snapshot · enabled — The snapshot's enabled field reads the presence of the element's disabled attribute.
contract/snapshot/password-value · snapshot · password input value — A password input's value reads MASKED_VALUE where it holds text and "" where it holds none.
contract/snapshot/select-list-rows · snapshot · select list rows — The snapshot holds no expanded key and the highlighted row is in no snapshot and no tree; an open list's row reads its own bounds in the list.
contract/select-open-list/optgroup-header · single-choice select, list open · optgroup header — An optgroup is painted as a header of an open list, heavier than a row by weight on every build, and a header chooses nothing.
contract/snapshot/file-input-value · snapshot · file input value — A file input reads MASKED_VALUE where its value attribute is non-empty, its path in no field of the snapshot.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/accessibility-feature-gate · Screen-reader integration behind the accessibility feature — Screen-reader integration through AccessKit is exposed only behind the `accessibility` feature, in blitz via blitz-shell and in the Dioxus crates.
decision/actions-verb-changes-nothing · The actions verb changes nothing — Answering the actions verb moves no focus, snapshot or diff, and an open list stays open.
decision/enabled-by-disabled-presence · Enabled follows the disabled attribute's presence — A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading. · guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
decision/keyed-wire-form · The keyed field on the carriers — The keyed reading on the actions answer is the actionable-key check's wire form, true where the id holds no slash.
decision/listeners-from-vdom-walk · Listeners read from the vdom walk — An element's current listeners are read from the vdom walk's dynamic attributes, never from the bridge's listener mark, so a listener removed on a re-render is not counted. · why: the bridge's listener attributes over-report: a removed listener leaves its event attribute and data-dioxus-id mark standing · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/option-labels-leave-only-as-pixels · Option labels leave an open list only as pixels — An option's label and a group's label leave an open list only as painted pixels in a screenshot and the snapshot's name, with no log site. · why: ratified
decision/password-never-revealed · Password text is never revealed — A password input's typed text is in no snapshot field, tree or painted frame: the snapshot reads MASKED_VALUE where it holds text and empty where none, and blitz-paint paints its value as bullets in every renderer. · why: ratified
decision/readers-of-one-snapshot-model · One snapshot model, many readers — The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping.
decision/snapshot-reads-the-tree · Snapshot takes its readings from the tree — The snapshot takes each node's role, name, focus and its selected, pressed and invalid readings from the accessibility tree, never from the element, with no second role or name mapping.
decision/ui-graph-carries-no-values · UI graph is read-only and carries no values — A UI graph read changes nothing and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text.
contract/accessibility-tree/select-expanded · the accessibility tree · a single select's expanded — A single select's expanded comes from engine state, never from an author aria-expanded, and focus stays on the select while its list is open.
contract/accessibility-tree/select-role · the accessibility tree · select — A select reads ComboBox, a multiple select ListBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes, each selected true or false and named by its non-empty label attribute, else its text.
contract/actionable-nodes/listing · actionable nodes · listing — The list holds, in snapshot pre-order, the UI graph's nodes the actionable rule holds for or whose owner is set, each with id, owner and keyed.
contract/actions-answer/causes · the actions answer · causes — For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.
contract/actions-answer/fields · the actions answer · fields — The answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId.
contract/driver-refusal/disabled · driver refusal causes · disabled — A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.
contract/node-id/reserved-separator · served node ids · the separator /// — No served id holds ///; the form {owner}///{key} is reserved.
contract/snapshot-node/clickable · the snapshot node · clickable — clickable is true where the element's current listeners hold click and it is no disabled form control; the JSON node always carries it beside focused.
contract/snapshot/open-list-highlighted-row · the snapshot and the accessibility tree · an open list's highlighted row — An open list's highlighted row is in no snapshot and no tree, a stated limit: the tree says the list is open, the row shows only in a screenshot.
contract/snapshot/open-list-row-bounds · the snapshot · an open list's rows — A select whose list is open is no closed select; an open row's bounds is its painted row's in the snapshot and the diff, the diff's differs naming bounds.
decision/driver-schema-has-no-role-set · Driver schema defines no role or name set — The driver's schema defines no role or name set of its own; role and name are field names of its result nodes.
contract/snapshot-diff/differs · the snapshot diff · differs — A changed node names the readings that differ from before the step, in node-key order.
contract/snapshot-diff/entries · the snapshot diff · entries — Two snapshots are compared by stable element id, entries carrying the later role and name; a text change is reported on the element whose name it changes, never as a text-run entry.

# a11y-plan · its absences, names and overviews, whole

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
> NOT YET MEASURED — must-be-accessible critical paths and a11y triggers: no slice recorded them, and s03 recorded accessibility scope as out of slice.
absence/assistive-technology-reach — A platform accessibility adapter in any stand binary, so no active assistive technology is given a tree

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
> NOT YET MEASURED — a stated accessibility strategy (POUR coverage depth, agent-runnable invariants, naming conventions): no slice states one, and s01, s02, s03, s06, s07, s08, s10 and s13 recorded accessibility strategy as out of slice.

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract
> NOT YET MEASURED — the structured violation JSON schema / log format, the WCAG criteria mapping of the existing tests and a screen-reader test pattern: no slice recorded them (s01, s02, s06, s07 and s13 recorded the a11y assertion harness as out of slice). The obs log format is now measured for escher's own sink (obs plan §3, §6 Log Coverage, §8): non-JSON text lines on stderr in two record classes — one line per printed event, `{RFC 3339 UTC time} {LEVEL} {target} service.name=… service.version=… {field}={value}…`, and, since 2026-10-07-driver-command-spans, one line per closed span of an admitted target, the same prefix then `span={the span's name}`, the span's recorded fields and the layer's close fields `message`, `time.busy` and `time.idle`, with no line when a span is created, recorded to, entered or exited (the second class was ratified by the founder, 2026-10-09, as obs plan §6 states) — both behind a target allowlist that, among others, prints only `node_id` / `status` / `waiting_nodes` / `property` / `log.*` fields for `blitz*` and `accesskit_xplat` targets, redacts `text`, `value`, `html` and `attrs` for escher's own targets (`escher_*`) and drops a record from any other target whole, at every level — a closed span's every pair judged by the same rule, an outside target's span writing nothing; an a11y violation schema is not yet defined against it.

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
> NOT YET MEASURED — skip links, modal focus traps and route-change focus restoration: no slice recorded them; s01 recorded keyboard focus handling code as out of slice and s13 recorded keyboard interaction as out of slice.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
> NOT YET MEASURED — focus-ring, target-size and typography-readability tokens: no slice recorded them, and s01 and s07 recorded contrast and visual checks as out of slice. The stand controls' contrast pairs are measured since 2026-10-13-stand-contrast-harness (Color contrast above), and the CRUD list rows, the text outside the controls and the colour states reached by input, hover and active included, since 2026-10-14-stand-a11y-assertions.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
absence/select-list-motion — Motion on an open select's list

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
> NOT YET MEASURED — live regions, heading-hierarchy handling and per-surface screen-reader test specs: no slice recorded them; s01, s07, s10 and s13 recorded screen-reader handling as out of slice, and s08 recorded the screen-reader tree output as out of slice.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
absence/snapshot-list-open-state — A snapshot reading of whether a list is open (no expanded node key)

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
> NOT YET MEASURED — timeout extensions, plain-language targets, redundant-entry avoidance and accessible authentication: every slice except s04 recorded cognitive accessibility as out of slice.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
> NO RECORDED INTENT

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## 12-a11y-decisions-log · ## 12. A11y Decisions Log
> NO RECORDED INTENT
