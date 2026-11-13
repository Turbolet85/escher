# a11y-plan · the rows that bind 2026-11-13-cognitive-load-corrective — picked 81 of 240 (60 by every draw) · rule any · pin 627

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered
decision/driver-requires-accessibility · Session library always enables accessibility — The escher-driver session library enables accessibility on dioxus-native-dom unconditionally, with no feature table of its own, so the feature reaches every binary that depends on it. · why: Its executor reads the snapshot, which stands behind that feature.

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
contract/accessibility-tree-states/native-checkbox-or-radio · Accessibility tree states · native checkbox or radio — A native checkbox or radio under a checkable role writes toggled from its own checked state, reading neither aria-checked nor aria-pressed; under any other role it writes nothing.
contract/accessibility-tree-states/select-option · Accessibility tree states · select option — A select's option reads its chosen state into the tree.
contract/aria-property-reads/aria-expanded · ARIA name and property reads · aria-expanded — aria-expanded is read as true or false and nothing else on every element but a single select, which writes its list's open state instead and reads no author attribute.

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/agent-readers-behind-accessibility · Snapshot readers behind the accessibility feature — The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
decision/changed-set-marks · What the engine's changed set marks — The engine's changed set marks a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing, a scroll that closes a list included; node creation, hover, active, any other scroll and layout do not. · why: Ratified, one item with the accessibility tree's refresh on change.
decision/harness-settle-leaves-changed-set · Harness settle leaves the changed set — A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first. · why: A settled step leaves the refresh and a later diff exactly what the step marked. · guard: stand_settle
decision/native-checkable-toggle · Native checkbox toggle from its checked state — A native checkbox or radio on a checkable role writes toggled true or false from its own checked state, and under any other role writes nothing. · guard: a_click_and_a_space_move_a_native_control_in_the_tree

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/single-select-expanded-from-list · Single select expanded from its list — A single select is a ComboBox that reads no aria-expanded and writes expanded from its list's open state, its chosen option selected. · guard: select_popup

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
decision/disabled-by-attribute-presence · Disabled-ness keyed one way by attribute presence — On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
decision/double-click-threshold · Double click threshold — Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, each still focusing its target as one click does.
decision/select-list-coverage-headless · Select list coverage headless — The select list's keyboard and focus coverage is headless, asserting SC 2.1.1 and 2.4.3 on it in both layout modes; the windowed list is read and not run. · guard: select_popup, stand_act_popup, stand_keyboard
contract/keyboard-activation/enter · keyboard activation · Enter — A pressed Enter on the focused node dispatches a Click when the node is no disabled form control and is a button, a summary, an input of type submit, reset or button, or an a with href.
contract/keyboard-activation/handler-order · keyboard activation · handler order — On a keyboard-dispatched Click the app's handlers run first, then the engine's handle_click.
contract/keyboard-activation/space · keyboard activation · Space — A released Space dispatches a Click for a summary, or for a button or an input of type submit, reset, button, checkbox or radio that is no disabled form control, never for a link.
contract/multiple-select/keys-and-row-clicks · multiple select · keys and row clicks — A multiple select takes its own keys and row clicks, apart from the single-choice select's handling.
contract/range-input/click · range input · click — A click on a range input focuses it and changes no value.
contract/range-input/main-button-pointerdown · range input · main-button pointerdown — A main-button pointerdown on a range input that is not disabled sets the value from the hit's fraction of the content box, clamped to [0, 1].
contract/select-open-list/close · single-choice select, list open · close — Closing a select's list clears the typed typeahead text.
contract/single-select/change-events · single-choice select · input and change events — Each change of a single-choice select's value dispatches one input then one change; a key that changes nothing dispatches none.
contract/range-input/input-event · range input · input event — Each change of a range input's value fires one input event; a move that changes nothing fires none.
contract/range-input/value · range input · value — A range input's value is held apart from the value attribute and sanitized to [min, max] on the step grid.
contract/select-open-list/escape · single-choice select, list open · Escape — With a select's list open, Escape closes the list with the value unchanged and no event, as a second click, a press outside and a page scroll do.
contract/select-open-list/navigation-keys · single-choice select, list open · navigation keys — With a select's list open, the engine paints the list over the page, focus stays on the select, and the select's keys move the highlight over enabled rows, choosing nothing.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/checkable-click-toggles-and-focuses · Checkbox or radio click toggles, fires input, focuses — Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control. · why: pinned by a_click_and_a_space_on_a_checkbox_each_fire_input_then_change; the stand's checkable controls and the driver's click rely on it
decision/click-focuses-buttons · A click focuses a button — A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus. · why: Ratified. · guard: stand_keyboard; stand_act_obstructed
decision/disabled-by-presence · Disabled read by attribute presence — The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count. · why: The presence reading and its narrowing to form controls were ratified. · guard: focusability_updates; stand_keyboard
decision/disabled-control-stops-click-walk · A disabled form control stops the click walk — A click on a disabled form control is stopped by the walk before it focuses or acts, while a disabled ancestor div does not stop it. · why: Ratified.
decision/disabled-optgroup-one-rule · Disabled optgroup rows read disabled by one rule — A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share. · why: form entry building, the list's row readings and key choice all call option_disabled; pinned by a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen
decision/double-click-on-document-clock · Double click read on the document's clock — Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, a harness-built document using the harness's virtual clock, while each click still focuses its target as one click does. · why: Ratified. · guard: stand_virtual_clock
decision/driver-type-focuses-select-no-click · Driver type focuses a select with no click — The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. · why: pinned by type_leaves_no_list_open_and_chooses_by_its_text in the stand_act_popup driver tests
decision/focus-stays-on-select-with-list-open · Focus stays on a select while its list is open — Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close. · why: pinned by the_tree_reads_the_list_open_on_the_select_its_focus_staying_there and escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box
decision/hidden-checkable-label-toggle · Native checkable state kept with no box — A native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when the control is hidden by display none. · why: Ratified as upstream's behaviour. · guard: clicking_label_toggles_hidden_checkbox
decision/keyboard-activation-enter-space · Enter and Space activate as a click — A pressed Enter on a focused enabled button, summary, submit/reset/button input or link with href, and a released Space on a focused enabled button, summary, or submit/reset/button/checkbox/radio input, dispatch a synthetic Click that runs the app's handlers and then the default click action. · why: ratified; a disabled form control takes neither key · guard: stand_keyboard, stand_act_keys · except: A disabled attribute on a summary or a link gates neither key, as HTML does; a link takes no Space.
decision/label-click-focuses-select-only · Label click focuses its select and opens nothing — A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it. · why: pinned by a_labels_click_focuses_the_select_and_opens_nothing in the select_popup tests
decision/one-engine-change-per-checkable-click · One engine change per checkable click — A page's script hears exactly one change per checkable click, from the engine, since the script runtime synthesizes none. · why: pinned by checkbox_click_fires_input_and_change_events in the vibey-script dom tests and by the checkable_change_event tests
decision/range-click-focuses-not-value · Range input click gives focus, not value — A click on a range input, including a label's synthetic click, focuses the input and changes no value. · why: ratified
decision/range-pointerdown-sets-value · Range input pointerdown sets value — A main-button pointerdown on an enabled range input sets its value from the hit's fraction of the content box, clamped to [0, 1], a content box of no width reading 0. · why: ratified
decision/select-list-choice-one-write-path · List choices share one write path — Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing. · why: pinned by enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change and a_choice_of_the_chosen_option_fires_nothing
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it
contract/dioxus-document/reached-handlers · Dioxus document reached-handler record · reached_handlers — Returns the stable id and event name of each node whose listeners held the dispatched event, in first-reached order, deduplicated by id and event, a handler that removes its own element still counting; empty when the record was never switched on.
contract/dioxus-document/record-reached · Dioxus document reached-handler record · record_reached — Switches on one reached record for the document, a second call keeping the one record; every other document pays one context lookup per dispatch.
contract/ui-graph-state/no-registered-state · UI graph state registration · no registered state — A page with no registered state reaches no registry, and its events dispatch with no registry recording.
contract/ui-graph-state/unconfirmed · UI graph state registration · unconfirmed — A declared write is flagged unconfirmed only once a dispatch of the handler's event passed through its element and no run wrote the state.
contract/ui-graph-state/undeclared · UI graph state registration · undeclared — A signal write is flagged undeclared where no declaration names it.
contract/ui-graph-state/use-graph-state · UI graph state registration · use_graph_state — Apps register state for the UI graph with authored DeclaredWrite declarations, each naming an element's stable id or a prefix ending in *, and an event named without on.
decision/inert-subtree-excluded · Inert subtree excluded — A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed. · why: ratified · guard: inert_subtree

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
contract/hit-testing/inert · hit-testing · inert — A hit passes through an element carrying inert, and removing inert restores the tree and the hit.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/driver-click-in-shown-scrollbar-drags · Driver click in a shown scrollbar band drags — Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first. · why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade
decision/open-pseudo-class-on-details-dialog · :open matches an open details or dialog — Author styles keyed on `:open` apply to an open details or dialog, `:open` reading the element's open state. · why: pinned by the open_state tests, among them at_construction_open_is_read_from_the_attribute and a_summary_click_flips_its_details
contract/driver-click-refusals/covered · Driver click refusals · covered — A driver click on an element under an open select list is refused `covered`.
decision/inert-element-takes-no-hit · An inert element takes no hit — A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit. · why: The `inert` read was ratified. · guard: inert_subtree
decision/open-select-list-not-open-state · Open select list sets no :open state — An open select's list is no `:open` state, so no author style keys on a list standing open. · why: pinned by an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match in the select_popup tests

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/actionable-key-rule · Actionable-key rule — Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it. · why: ratified · guard: unkeyed_actionable
decision/one-actionable-rule · One actionable rule — The unkeyed-actionable check and the actionable-nodes read share one crate-private actionable rule, focusable or an interactive role or the vdom walk's exact current listeners holding an event other than mounted, with no second role mapping; the data-dioxus-id mark serves event dispatch alone.
decision/ui-graph-one-node-per-snapshot-node · UI graph mirrors the snapshot — The read-only UI graph holds one node per snapshot node, names and closed words only.
contract/accessibility-tree/single-select · accessibility tree · single select — A single select reads ComboBox and writes expanded from its list's open state, reading no aria-expanded; an author's open or aria-expanded on it steers nothing; its chosen option reads selected and the tree's focus stays on the select.
contract/driver-actions/disabled · driver actions · disabled — The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.
contract/accessible-name/label-element · accessible name · label element — A label names the labelable element bound to it, its for-target by id or else its first nested labelable element, whether placed before or after it; a hidden input is named by no label.
contract/select-open-list/optgroup-header · single-choice select, list open · optgroup header — An optgroup is painted as a header of an open list, heavier than a row by weight on every build, and a header chooses nothing.
contract/snapshot/enabled · snapshot · enabled — The snapshot's enabled field reads the presence of the element's disabled attribute.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/actions-verb-changes-nothing · The actions verb changes nothing — Answering the actions verb moves no focus, snapshot or diff, and an open list stays open.
decision/interactive-nodes-named · Interactive nodes carry a name — Every interactive-role tree node of the lean stand tasks reads a non-empty accessible name and an HTML-AAM role, in both layout modes. · guard: stand_accessibility_ids
decision/keyed-wire-form · The keyed field on the carriers — The keyed reading on the actions answer is the actionable-key check's wire form, true where the id holds no slash.
decision/listeners-from-vdom-walk · Listeners read from the vdom walk — An element's current listeners are read from the vdom walk's dynamic attributes, never from the bridge's listener mark, so a listener removed on a re-render is not counted. · why: the bridge's listener attributes over-report: a removed listener leaves its event attribute and data-dioxus-id mark standing · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/screenshot-changes-nothing · A screenshot changes nothing — Taking a screenshot moves no focus, leaves has_changes false and leaves the next diff empty.
decision/ui-graph-carries-no-values · UI graph is read-only and carries no values — A UI graph read changes nothing and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text.
contract/accessibility-tree/select-expanded · the accessibility tree · a single select's expanded — A single select's expanded comes from engine state, never from an author aria-expanded, and focus stays on the select while its list is open.
contract/actionable-nodes/listing · actionable nodes · listing — The list holds, in snapshot pre-order, the UI graph's nodes the actionable rule holds for or whose owner is set, each with id, owner and keyed.
contract/actions-answer/causes · the actions answer · causes — For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.
contract/actions-answer/fields · the actions answer · fields — The answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId.
contract/driver-refusal/disabled · driver refusal causes · disabled — A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.
contract/driver-screenshot/element-shot · the driver's screenshot · an element shot — An element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen.
contract/driver-screenshot/row-shot · the driver's screenshot · a shot of an open list's row — A shot of a row crops to it and leaves the list open.
contract/handler-coverage/answer · handler coverage · answer — The answer lists the graph's handlers no check reached and holds ids, owner ids, event names and two counts, checks and held, only, no role or name.
decision/enabled-by-disabled-presence · Enabled follows the disabled attribute's presence — A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading. · guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
decision/label-names-its-labelable-element · A label names its bound labelable element — A label names its for-target by id, else its first nested labelable element, whether it stands before or after that element; a hidden input is named by no label. · why: follows HTML's labelable elements: button, non-hidden input, meter, output, progress, select, textarea
decision/option-labels-leave-only-as-pixels · Option labels leave an open list only as pixels — An option's label and a group's label leave an open list only as painted pixels in a screenshot and the snapshot's name, with no log site. · why: ratified
contract/form-event-values/multiple-select · form event values · a multiple select's set — On Input and Change, evt.values() gives one (name, text value) form entry per chosen option of a multiple select, while value stays the select's single value.
contract/node-id/reserved-separator · served node ids · the separator /// — No served id holds ///; the form {owner}///{key} is reserved.

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
