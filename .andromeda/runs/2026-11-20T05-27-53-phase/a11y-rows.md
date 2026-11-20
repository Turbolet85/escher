# a11y-plan · the rows that bind 2026-11-20-codebase-health — picked 83 of 240 (56 by every draw) · rule any · pin 2532

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/driver-requires-accessibility · Session library always enables accessibility — The escher-driver session library enables accessibility on dioxus-native-dom unconditionally, with no feature table of its own, so the feature reaches every binary that depends on it. · why: Its executor reads the snapshot, which stands behind that feature.
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/a11y-tree-refresh-on-change · Accessibility tree refreshed on change — The accessibility tree is built from the document when first requested and pushed again in a redraw, after style resolve, only when the changed set is non-empty, the set then cleared; a poll pushes no tree. · why: Ratified, one item with the engine's changed-set contract. · guard: stand_accessibility_ids
decision/agent-readers-behind-accessibility · Snapshot readers behind the accessibility feature — The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
decision/aria-description-to-adapter · ARIA description to the platform adapter — The aria-describedby and aria-description attributes reach the platform adapter as the node's description; the snapshot reads no description. · why: Ratified as upstream's behaviour. · guard: accessibility_description
decision/harness-settle-leaves-changed-set · Harness settle leaves the changed set — A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first. · why: A settled step leaves the refresh and a later diff exactly what the step marked. · guard: stand_settle

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract
decision/a11y-ci-leg · The a11y CI leg — The CI job "Accessibility (a11y) tests" runs the accessibility integration tests as their own leg, on one cargo test line in the slow tier, with no required status check. · why: The existing a11y job was widened rather than a new job, required status check or repository setting added, by the chosen option. · guard: LegScriptTest's a11y tuple

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
decision/disabled-by-attribute-presence · Disabled-ness keyed one way by attribute presence — On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
contract/single-select/optgroup-options · single-choice select · optgroup options — An optgroup's options are the select's own options, in tree order.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/disabled-optgroup-one-rule · Disabled optgroup rows read disabled by one rule — A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share. · why: form entry building, the list's row readings and key choice all call option_disabled; pinned by a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen
decision/driver-type-focuses-select-no-click · Driver type focuses a select with no click — The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. · why: pinned by type_leaves_no_list_open_and_chooses_by_its_text in the stand_act_popup driver tests
contract/dioxus-document/reached-handlers · Dioxus document reached-handler record · reached_handlers — Returns the stable id and event name of each node whose listeners held the dispatched event, in first-reached order, deduplicated by id and event, a handler that removes its own element still counting; empty when the record was never switched on.
contract/dioxus-document/record-reached · Dioxus document reached-handler record · record_reached — Switches on one reached record for the document, a second call keeping the one record; every other document pays one context lookup per dispatch.
contract/ui-graph-state/unconfirmed · UI graph state registration · unconfirmed — A declared write is flagged unconfirmed only once a dispatch of the handler's event passed through its element and no run wrote the state.
contract/ui-graph-state/undeclared · UI graph state registration · undeclared — A signal write is flagged undeclared where no declaration names it.
contract/ui-graph-state/use-graph-state · UI graph state registration · use_graph_state — Apps register state for the UI graph with authored DeclaredWrite declarations, each naming an element's stable id or a prefix ending in *, and an event named without on.
decision/checkable-click-toggles-and-focuses · Checkbox or radio click toggles, fires input, focuses — Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control. · why: pinned by a_click_and_a_space_on_a_checkbox_each_fire_input_then_change; the stand's checkable controls and the driver's click rely on it
decision/disabled-by-presence · Disabled read by attribute presence — The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count. · why: The presence reading and its narrowing to form controls were ratified. · guard: focusability_updates; stand_keyboard

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/stand-contrast-measured · Stand contrast measured from computed styles — SC 1.4.3 is measured on the stand from computed styles with the WCAG 2.x ratio, 3.0 required of large text and else 4.5, a row exempt exactly where the snapshot's enabled reads Some(false). · guard: stand_contrast, the_load_page_meets_its_ratios, cells_meets_its_ratios
contract/accessibility-tree/hidden-and-inert-nodes · accessibility tree · hidden and inert nodes — Nodes with display: none, visibility: hidden or visibility: collapse or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
contract/driver-click-refusals/covered · Driver click refusals · covered — A driver click on an element under an open select list is refused `covered`.
decision/contrast-exempt-disabled-controls · Disabled controls exempt from contrast — A contrast row is exempt exactly where the snapshot's enabled reads Some(false); a label[for=…] row is never exempt.
decision/driver-click-in-shown-scrollbar-drags · Driver click in a shown scrollbar band drags — Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first. · why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade
decision/hidden-content-excluded-from-a11y-tree · Hidden content excluded from the accessibility tree — Nodes with `display: none`, `visibility: hidden` or `visibility: collapse` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot. · except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/adapter-tree-equals-harness-tree · Adapter tree equals harness tree — On the stand the tree the shell hands the adapter equals the harness's tree node for node, and every interactive-role node of the four lean tasks reads a non-empty name.
decision/one-actionable-rule · One actionable rule — The unkeyed-actionable check and the actionable-nodes read share one crate-private actionable rule, focusable or an interactive role or the vdom walk's exact current listeners holding an event other than mounted, with no second role mapping; the data-dioxus-id mark serves event dispatch alone.
decision/password-value-masked · Password value masked — A password input's typed text appears in no snapshot field, tree node or painted frame; its value reads a fixed masked marker where it holds text and an empty string where it holds none.
decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
decision/snapshot-reads-accessibility-tree · Snapshot reads the accessibility tree — The snapshot takes each node's role, name, focus, selected, pressed and invalid from the accessibility tree, never from the element.
decision/ui-graph-one-node-per-snapshot-node · UI graph mirrors the snapshot — The read-only UI graph holds one node per snapshot node, names and closed words only.
contract/driver-actions/disabled · driver actions · disabled — The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.
contract/snapshot/checked · snapshot · checked — The snapshot's checked field reads a native checkbox's or radio's state, else a checkable role's toggled.
contract/snapshot/clickable · snapshot · clickable — The snapshot's clickable field says an element that is no disabled form control reacts to a click, read from the vdom walk's current click listeners and not the listener mark.
contract/snapshot/enabled · snapshot · enabled — The snapshot's enabled field reads the presence of the element's disabled attribute.
contract/snapshot/file-input-value · snapshot · file input value — A file input reads MASKED_VALUE where its value attribute is non-empty, its path in no field of the snapshot.
contract/snapshot/multiple-select-rows-and-value · snapshot · multiple select rows and value — A multiple select's rows read their own client rects in the snapshot, and its value reads its first chosen option in tree order.
contract/snapshot/password-value · snapshot · password input value — A password input's value reads MASKED_VALUE where it holds text and "" where it holds none.
contract/snapshot/select-list-rows · snapshot · select list rows — The snapshot holds no expanded key and the highlighted row is in no snapshot and no tree; an open list's row reads its own bounds in the list.
contract/snapshot/value · snapshot · value — The snapshot's value is read from the element: the editor's text, else a range input's current value, else the value attribute.
decision/actionable-key-rule · Actionable-key rule — Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it. · why: ratified · guard: unkeyed_actionable
decision/snapshot-model-single-source · Snapshot model as single source — The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else.
contract/accessible-name/aria-label · accessible name · aria-label — An element's aria-label, non-empty after trimming, becomes its label; a whitespace-only one names nothing.
contract/accessible-name/aria-labelledby · accessible name · aria-labelledby — An aria-labelledby whose targets hold text outranks aria-label and a bound label element; aria-label is the fallback when the targets hold none.
contract/accessible-name/label-element · accessible name · label element — A label names the labelable element bound to it, its for-target by id or else its first nested labelable element, whether placed before or after it; a hidden input is named by no label.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/accessibility-feature-gate · Screen-reader integration behind the accessibility feature — Screen-reader integration through AccessKit is exposed only behind the `accessibility` feature, in blitz via blitz-shell and in the Dioxus crates.
decision/actions-verb-changes-nothing · The actions verb changes nothing — Answering the actions verb moves no focus, snapshot or diff, and an open list stays open.
decision/checked-reading · Source of the checked reading — checked is a native checkbox's or radio's state, else on a checkable role its toggled reading, True true and False and Mixed false, nothing where the role states none. · why: ratified
decision/enabled-by-disabled-presence · Enabled follows the disabled attribute's presence — A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading. · guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
decision/interactive-nodes-named · Interactive nodes carry a name — Every interactive-role tree node of the lean stand tasks reads a non-empty accessible name and an HTML-AAM role, in both layout modes. · guard: stand_accessibility_ids
decision/keyed-wire-form · The keyed field on the carriers — The keyed reading on the actions answer is the actionable-key check's wire form, true where the id holds no slash.
decision/listeners-from-vdom-walk · Listeners read from the vdom walk — An element's current listeners are read from the vdom walk's dynamic attributes, never from the bridge's listener mark, so a listener removed on a re-render is not counted. · why: the bridge's listener attributes over-report: a removed listener leaves its event attribute and data-dioxus-id mark standing · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/password-never-revealed · Password text is never revealed — A password input's typed text is in no snapshot field, tree or painted frame: the snapshot reads MASKED_VALUE where it holds text and empty where none, and blitz-paint paints its value as bullets in every renderer. · why: ratified
decision/screenshot-changes-nothing · A screenshot changes nothing — Taking a screenshot moves no focus, leaves has_changes false and leaves the next diff empty.
decision/snapshot-reads-the-tree · Snapshot takes its readings from the tree — The snapshot takes each node's role, name, focus and its selected, pressed and invalid readings from the accessibility tree, never from the element, with no second role or name mapping.
decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
decision/text-runs-label-their-parent · Text runs label their parent element — Text nodes become TextRun nodes carrying their text, and their parent is labelled by them, so an element's text reaches its accessible name. · why: accessible names of buttons and controls come from it; pinned by every_interactive_role_element_reads_a_name and role_and_name_match_the_accessibility_tree
contract/actionable-nodes/listing · actionable nodes · listing — The list holds, in snapshot pre-order, the UI graph's nodes the actionable rule holds for or whose owner is set, each with id, owner and keyed.
contract/actions-answer/causes · the actions answer · causes — For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.
contract/actions-answer/fields · the actions answer · fields — The answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId.
contract/driver-refusal/disabled · driver refusal causes · disabled — A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.
contract/driver-screenshot/element-shot · the driver's screenshot · an element shot — An element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen.
contract/driver-screenshot/row-shot · the driver's screenshot · a shot of an open list's row — A shot of a row crops to it and leaves the list open.
contract/handler-coverage/answer · handler coverage · answer — The answer lists the graph's handlers no check reached and holds ids, owner ids, event names and two counts, checks and held, only, no role or name.
contract/node-id/reserved-separator · served node ids · the separator /// — No served id holds ///; the form {owner}///{key} is reserved.
contract/snapshot-diff/differs · the snapshot diff · differs — A changed node names the readings that differ from before the step, in node-key order.
contract/snapshot-diff/entries · the snapshot diff · entries — Two snapshots are compared by stable element id, entries carrying the later role and name; a text change is reported on the element whose name it changes, never as a text-run entry.
contract/snapshot-node/clickable · the snapshot node · clickable — clickable is true where the element's current listeners hold click and it is no disabled form control; the JSON node always carries it beside focused.
contract/snapshot/closed-select-option-bounds · the snapshot · a closed select's options — A closed select's options read under it, each with the select's client rect as its bounds.
contract/snapshot/multiple-select-row-bounds · the snapshot · a multiple select's rows and groups — A multiple select's rows and groups read their own client rects as their bounds, unlike a closed select's options.
contract/snapshot/open-list-highlighted-row · the snapshot and the accessibility tree · an open list's highlighted row — An open list's highlighted row is in no snapshot and no tree, a stated limit: the tree says the list is open, the row shows only in a screenshot.
contract/snapshot/open-list-row-bounds · the snapshot · an open list's rows — A select whose list is open is no closed select; an open row's bounds is its painted row's in the snapshot and the diff, the diff's differs naming bounds.
contract/snapshot/select-value · the snapshot · a select's value — A select's value is its chosen option's value; for a multiple select its first chosen option's in tree order; nothing when none is chosen.
decision/accessible-name-precedence · Accessible name precedence — An aria-labelledby whose targets hold text names an element first, then a non-empty trimmed aria-label, then a bound label; a whitespace-only aria-label names nothing. · why: follows accname 1.2 (2B over 2C and 2D)
decision/boolean-state-limits · Limits of the boolean state model — A mixed toggle reads pressed false and a grammar or spelling error reads invalid true, the node keeping its full value.
decision/driver-schema-has-no-role-set · Driver schema defines no role or name set — The driver's schema defines no role or name set of its own; role and name are field names of its result nodes.
decision/label-names-its-labelable-element · A label names its bound labelable element — A label names its for-target by id, else its first nested labelable element, whether it stands before or after that element; a hidden input is named by no label. · why: follows HTML's labelable elements: button, non-hidden input, meter, output, progress, select, textarea
decision/option-labels-leave-only-as-pixels · Option labels leave an open list only as pixels — An option's label and a group's label leave an open list only as painted pixels in a screenshot and the snapshot's name, with no log site. · why: ratified
decision/readers-of-one-snapshot-model · One snapshot model, many readers — The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping.
contract/snapshot-text/clickable · the snapshot text form · clickable — The token clickable is written after focused only when true.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/stand-shared-tab-order-table · Stand focus order asserted against a shared table — The stand asserts each lean task's forward focus sequence against one shared tab-order table kept in the stand checks' shared module. · guard: stand_accessibility_ids · except: A Tab order reached only after a state change, such as the order page with its failure shown, is stated as a local table of the keyboard check, not in the shared table.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/a11y-test-leg · Accessibility test leg in the slow tier — The accessibility test files run in the a11y leg of the slow tier behind the fast jobs, with no required status check; the stand-facing ones also run three times in the stand-repeat leg under the zero-failure budget. · guard: ci.yml a11y job

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/stand-checks-repeated-three-rounds · Stand checks repeated three rounds — Every stand_ test file and the named a11y witnesses outside the stand_ prefix run three rounds in the stand-repeat leg under the zero-failure budget. · guard: test_stand_repeat_leg_runs_three_rounds; stand_requirement_sweep

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. A11y Assertion Harness Contract
decision/a11y-ci-gate · Accessibility checks gate CI — Accessibility checks run in CI as the `a11y` job. · guard: a11y

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
