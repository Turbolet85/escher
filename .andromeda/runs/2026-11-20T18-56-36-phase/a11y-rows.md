# a11y-plan · the rows that bind 2026-11-20-driver-debts — picked 38 of 239 (22 by every draw) · rule any · pin 2748

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/a11y-tier-0 · Accessibility tier 0 — The accessibility tier is 0, covering WCAG success criteria 2.1.1, 1.4.3 and 2.4.3.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/accessibility-reach-tier-0 · Accessibility reach at tier 0 — The published accessibility reach is tier 0: WCAG SC 2.1.1, 1.4.3 and 2.4.3 and no other criterion, with what an assistive technology reads unmeasured.
decision/capability-statement-reach-by-hand · Accessibility reach edited by hand — A change to the accessibility reach is written by hand into the capability statement's Accessibility reach section. · why: The capability statement's pin reads only the a11y leg's file count among the section's claims; no check reads the rest.
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
contract/accessibility-tree-states/native-checkbox-or-radio · Accessibility tree states · native checkbox or radio — A native checkbox or radio under a checkable role writes toggled from its own checked state, reading neither aria-checked nor aria-pressed; under any other role it writes nothing.
contract/accessibility-tree-states/select-option · Accessibility tree states · select option — A select's option reads its chosen state into the tree.

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/agent-readers-behind-accessibility · Snapshot readers behind the accessibility feature — The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
decision/native-checkable-toggle · Native checkbox toggle from its checked state — A native checkbox or radio on a checkable role writes toggled true or false from its own checked state, and under any other role writes nothing. · guard: a_click_and_a_space_move_a_native_control_in_the_tree

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
contract/keyboard-activation/enter · keyboard activation · Enter — A pressed Enter on the focused node dispatches a Click when the node is no disabled form control and is a button, a summary, an input of type submit, reset or button, or an a with href.
contract/keyboard-activation/handler-order · keyboard activation · handler order — On a keyboard-dispatched Click the app's handlers run first, then the engine's handle_click.
contract/keyboard-activation/space · keyboard activation · Space — A released Space dispatches a Click for a summary, or for a button or an input of type submit, reset, button, checkbox or radio that is no disabled form control, never for a link.
contract/multiple-select/keys-and-row-clicks · multiple select · keys and row clicks — A multiple select takes its own keys and row clicks, apart from the single-choice select's handling.
decision/select-list-coverage-headless · Select list coverage headless — The select list's keyboard and focus coverage is headless, asserting SC 2.1.1 and 2.4.3 on it in both layout modes; the windowed list is read and not run. · guard: select_popup, stand_act_popup, stand_keyboard
decision/stated-tab-order-walked · Stated Tab order walked — Each lean task, the load page and Cells state their Tab order, walked forward and back in both layout modes with the engine's focus, the snapshot's focused node and the tree's focus agreeing at every step, and every lean task's control lying in one of the two lean orders. · guard: stand_keyboard

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/disabled-optgroup-one-rule · Disabled optgroup rows read disabled by one rule — A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share. · why: form entry building, the list's row readings and key choice all call option_disabled; pinned by a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen
decision/keyboard-activation-enter-space · Enter and Space activate as a click — A pressed Enter on a focused enabled button, summary, submit/reset/button input or link with href, and a released Space on a focused enabled button, summary, or submit/reset/button/checkbox/radio input, dispatch a synthetic Click that runs the app's handlers and then the default click action. · why: ratified; a disabled form control takes neither key · guard: stand_keyboard, stand_act_keys · except: A disabled attribute on a summary or a link gates neither key, as HTML does; a link takes no Space.
decision/tab-and-shift-tab-move-focus · Tab and Shift+Tab move focus with focus events — Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events. · why: pinned by tab_and_shift_tab_walk_the_stated_sequence_and_wrap and tab_and_shift_tab_skip_an_inert_subtree; the driver's press tab relies on it
contract/dioxus-document/reached-handlers · Dioxus document reached-handler record · reached_handlers — Returns the stable id and event name of each node whose listeners held the dispatched event, in first-reached order, deduplicated by id and event, a handler that removes its own element still counting; empty when the record was never switched on.
contract/dioxus-document/record-reached · Dioxus document reached-handler record · record_reached — Switches on one reached record for the document, a second call keeping the one record; every other document pays one context lookup per dispatch.
contract/ui-graph-state/no-registered-state · UI graph state registration · no registered state — A page with no registered state reaches no registry, and its events dispatch with no registry recording.
contract/ui-graph-state/unconfirmed · UI graph state registration · unconfirmed — A declared write is flagged unconfirmed only once a dispatch of the handler's event passed through its element and no run wrote the state.
contract/ui-graph-state/undeclared · UI graph state registration · undeclared — A signal write is flagged undeclared where no declaration names it.
contract/ui-graph-state/use-graph-state · UI graph state registration · use_graph_state — Apps register state for the UI graph with authored DeclaredWrite declarations, each naming an element's stable id or a prefix ending in *, and an event named without on.
decision/click-focuses-buttons · A click focuses a button — A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus. · why: Ratified. · guard: stand_keyboard; stand_act_obstructed
decision/disabled-control-stops-click-walk · A disabled form control stops the click walk — A click on a disabled form control is stopped by the walk before it focuses or acts, while a disabled ancestor div does not stop it. · why: Ratified.
decision/driver-type-focuses-select-no-click · Driver type focuses a select with no click — The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. · why: pinned by type_leaves_no_list_open_and_chooses_by_its_text in the stand_act_popup driver tests
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/ui-graph-one-node-per-snapshot-node · UI graph mirrors the snapshot — The read-only UI graph holds one node per snapshot node, names and closed words only.
contract/snapshot/multiple-select-rows-and-value · snapshot · multiple select rows and value — A multiple select's rows read their own client rects in the snapshot, and its value reads its first chosen option in tree order.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/driver-schema-has-no-role-set · Driver schema defines no role or name set — The driver's schema defines no role or name set of its own; role and name are field names of its result nodes.
decision/listeners-from-vdom-walk · Listeners read from the vdom walk — An element's current listeners are read from the vdom walk's dynamic attributes, never from the bridge's listener mark, so a listener removed on a re-render is not counted. · why: the bridge's listener attributes over-report: a removed listener leaves its event attribute and data-dioxus-id mark standing · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/ui-graph-carries-no-values · UI graph is read-only and carries no values — A UI graph read changes nothing and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text.
contract/form-event-values/multiple-select · form event values · a multiple select's set — On Input and Change, evt.values() gives one (name, text value) form entry per chosen option of a multiple select, while value stays the select's single value.
contract/handler-coverage/answer · handler coverage · answer — The answer lists the graph's handlers no check reached and holds ids, owner ids, event names and two counts, checks and held, only, no role or name.
contract/mcp-result/snapshot · MCP tool results · snapshot — An accepted snapshot result is the text form itself for a whole screen, and for a screen in parts the JSON line {text, part, parts}; an acting call's result is its JSON line with the diff.
contract/snapshot-text/parts · the snapshot text form · parts — A screen over the text budget is read in parts of consecutive pre-order lines, never a split line, concatenating to the whole text with every node id on exactly one line.
decision/keyed-wire-form · The keyed field on the carriers — The keyed reading on the actions answer is the actionable-key check's wire form, true where the id holds no slash.
contract/snapshot/multiple-select-row-bounds · the snapshot · a multiple select's rows and groups — A multiple select's rows and groups read their own client rects as their bounds, unlike a closed select's options.

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
