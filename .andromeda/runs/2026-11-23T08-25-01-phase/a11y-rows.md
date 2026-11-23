# a11y-plan · the rows that bind 2026-11-23-canvas-placement-and-text — picked 32 of 242 (10 by every draw) · rule any · pin 3273

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed drop-down single select (display size 1) opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root; a single list box opens none. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
contract/range-input/main-button-pointerdown · range input · main-button pointerdown — A main-button pointerdown on a range input that is not disabled sets the value from the hit's fraction of the content box, clamped to [0, 1].
decision/disabled-by-attribute-presence · Disabled-ness keyed one way by attribute presence — On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
decision/double-click-threshold · Double click threshold — Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, each still focusing its target as one click does.
contract/multiple-select/keys-and-row-clicks · multiple select · keys and row clicks — A multiple select takes its own keys and row clicks, apart from the single-choice select's handling.
contract/range-input/click · range input · click — A click on a range input focuses it and changes no value.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/inert-subtree-excluded · Inert subtree excluded — A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed. · why: ratified · guard: inert_subtree
decision/range-pointerdown-sets-value · Range input pointerdown sets value — A main-button pointerdown on an enabled range input sets its value from the hit's fraction of the content box, clamped to [0, 1], a content box of no width reading 0. · why: ratified
decision/checkable-click-toggles-and-focuses · Checkbox or radio click toggles, fires input, focuses — Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control. · why: pinned by a_click_and_a_space_on_a_checkbox_each_fire_input_then_change; the stand's checkable controls and the driver's click rely on it
decision/click-focuses-buttons · A click focuses a button — A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus. · why: Ratified. · guard: stand_keyboard; stand_act_obstructed
decision/disabled-control-stops-click-walk · A disabled form control stops the click walk — A click on a disabled form control is stopped by the walk before it focuses or acts, while a disabled ancestor div does not stop it. · why: Ratified.
decision/double-click-on-document-clock · Double click read on the document's clock — Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, a harness-built document using the harness's virtual clock, while each click still focuses its target as one click does. · why: Ratified. · guard: stand_virtual_clock
decision/driver-type-focuses-select-no-click · Driver type focuses a select with no click — The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. · why: pinned by type_leaves_no_list_open_and_chooses_by_its_text in the stand_act_popup driver tests
decision/hidden-checkable-label-toggle · Native checkable state kept with no box — A native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when the control is hidden by display none. · why: Ratified as upstream's behaviour. · guard: clicking_label_toggles_hidden_checkbox
decision/label-click-focuses-select-only · Label click focuses its select and opens nothing — A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it. · why: pinned by a_labels_click_focuses_the_select_and_opens_nothing in the select_popup tests
decision/one-engine-change-per-checkable-click · One engine change per checkable click — A page's script hears exactly one change per checkable click, from the engine, since the script runtime synthesizes none. · why: pinned by checkbox_click_fires_input_and_change_events in the vibey-script dom tests and by the checkable_change_event tests
decision/range-click-focuses-not-value · Range input click gives focus, not value — A click on a range input, including a label's synthetic click, focuses the input and changes no value. · why: ratified
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it
contract/single-list-box/row-click · single list box · row click — A click on a single list box's row focuses the select, clears its typeahead and chooses that row alone whatever modifier is held; a disabled select or row changes nothing.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
contract/hit-testing/inert · hit-testing · inert — A hit passes through an element carrying inert, and removing inert restores the tree and the hit.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/inert-element-takes-no-hit · An inert element takes no hit — A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit. · why: The `inert` read was ratified. · guard: inert_subtree
decision/viewport-scheme-and-zoom-level · Viewport carries scheme and zoom, 1.0 unzoomed — The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed. · why: the stylo device reads the scheme for prefers-color-scheme and the scale from zoom; the shell and harness configs set both
decision/driver-click-in-shown-scrollbar-drags · Driver click in a shown scrollbar band drags — Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first. · why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade
decision/hidpi-or-zoom-change-reshapes-text · Hidpi or zoom change reshapes text — A total scale change from hidpi or zoom invalidates text shaping. · why: the shell's scale-factor change and the browser zoom keys write hidpi scale and zoom, and rely on text being reshaped at the new scale
contract/driver-click-refusals/covered · Driver click refusals · covered — A driver click on an element under an open select list is refused `covered`.

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/password-value-masked · Password value masked — A password input's typed text appears in no snapshot field, tree node or painted frame; its value reads a fixed masked marker where it holds text and an empty string where it holds none.
decision/one-actionable-rule · One actionable rule — The unkeyed-actionable check and the actionable-nodes read share one crate-private actionable rule, focusable or an interactive role or the vdom walk's exact current listeners holding an event other than mounted, with no second role mapping; the data-dioxus-id mark serves event dispatch alone.
contract/driver-actions/disabled · driver actions · disabled — The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/password-never-revealed · Password text is never revealed — A password input's typed text is in no snapshot field, tree or painted frame: the snapshot reads MASKED_VALUE where it holds text and empty where none, and blitz-paint paints its value as bullets in every renderer. · why: ratified
decision/screenshot-changes-nothing · A screenshot changes nothing — Taking a screenshot moves no focus, leaves has_changes false and leaves the next diff empty.
decision/password-caret-by-bullets · Password caret placed by bullets — A click in a password input places the caret by bullets laid out one per character, so no width of the real characters reaches the caret. · why: ratified · guard: password_caret_click
contract/driver-refusal/disabled · driver refusal causes · disabled — A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.

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
