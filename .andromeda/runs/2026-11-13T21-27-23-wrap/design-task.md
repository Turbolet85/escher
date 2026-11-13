@task_prose part=session
A restructure for cognitive load, with no change in behaviour: the engine's click default-action handler, the driver's command dispatch and the UI graph builder were each split into private per-arm or per-step functions. No public symbol, verb, answer, refusal, exit status, span field or config key was added, removed or re-signed.
Cognitive complexity measured: the click handler 97 to 2, the driver's execute 32 to 1, the graph builder 50 to 4; three engine functions over 15 are left in the pointer handling (pointermove 77, pointerdown 39, touch-action pan axes 17).
One new integration check added, the click default actions: link navigation and fragment links, a link with no href walking on to its label, an unmatched click clearing focus (focus then reads the root element), a disabled submit stopping the walk, a label of a multiple select focusing it and choosing nothing, and two clicks with no time between them being one double click; both layout modes, silent (no logging).
Coverage readings moved with every floor held: escher-driver 98.01 percent, dioxus-native-dom 90.09 percent, blitz-test-harness 95.74 percent. CI green, 17 of 17 checks.
Nothing retired; no capability claimed.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling — the click default actions (checkbox and radio toggling with their change events, summary toggling, select opening, option choosing in an open select list, label re-dispatch to its bound control, link navigation, file input, button submit), focus clearing on an unmatched click, and the double-click dispatch
@task_file n=2 path=packages/escher-driver/src/execute.rs
what: the driver's verb dispatch for escher-session — snapshot with its parts, click, type with its select focus and acting order, press, advance on the virtual clock, scroll, screenshot with its crop and refusals, and the graph answer feeding the record of ids
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph builder of the dioxus-native document — the state readings, each node's handlers and the writes they make, the reads of each node, and the graph's states that the UI graph line and the graph answer print
@task_file n=4 path=tests/blitz-tests/tests/click_default_actions.rs
what: a new integration check of the click default actions — link navigation and fragment links, a link with no href walking to its label, an unmatched click clearing focus, a disabled submit stopping the walk, a label of a multiple select, and two clicks with no time between them as one double click
@task_item n=1 part=touches
decision/single-select-open-gestures — the click arm that opens a single select moved into its own private function; the gesture holds unchanged, a row citing where the click opens the list is stale at most in its evidence.
@task_item n=2 part=touches
decision/selects-that-open-no-list — the select click arm now reads single-select through its own function; a multiple select still opens no list, and the new check asserts a label of a multiple select focuses it and chooses nothing, a guard beside this row.
@task_item n=3 part=touches
contract/select-list-events/open and contract/select-list-events/choose — the open on a click of a single select and the choose of an option in an open list moved into separate private click functions; the events and their order unchanged.
@task_item n=4 part=touches
contract/select-list-events/choose-disabled-row, choose-chosen-option, close and close-on-select-change — the option click arm moved; behaviour unchanged, a check of the move.
@task_item n=5 part=touches
decision/open-select-list-hit-first — the click walk that reaches an open select list's rows was restructured; the list still answers hits first.
@task_item n=6 part=touches
contract/driver-verbs-and-virtual-clock/click, type, press and advance — each verb's dispatch moved into its own private function in the driver; the type verb's acting order (select focus or click, select-all and backspace on an empty text, typing that skips a text opening on a space on a select) is now a named step; answers and the clock moves unchanged.
@task_item n=7 part=touches
decision/driver-advance-moves-one-clock and decision/load-wait-moves-no-clock — the advance verb's settle step moved into its own function; the one virtual clock and its moves unchanged.
@task_item n=8 part=touches
decision/driver-scroll-instant — the scroll verb moved into its own function; still instant.
@task_item n=9 part=touches
contract/escher-session-commands/screenshot and contract/driver-screenshot-refusals/fonts-unavailable and contract/driver-screenshot/fonts-unavailable — the screenshot verb with its crop and its screenshot error map moved into its own function; refusals unchanged.
@task_item n=10 part=touches
decision/screenshot-never-skips-text — the screenshot path was moved, not changed; text is still painted.
@task_item n=11 part=touches
contract/escher-session-commands/snapshot — the snapshot verb with its text budget and parts moved into its own function; unchanged.
@task_item n=12 part=touches
contract/escher-session-commands/graph-answer and contract/ui-graph-line/content — the UI graph builder was split into state readings, node handlers, handler writes, node reads and graph states, and the graph verb into its own function; the graph line's content unchanged.
@task_item n=13 part=touches
decision/actions-share-the-verbs-checks and contract/actions-line/content — the verbs' dispatch these rows rule over was restructured; the checks they share (target check, resolve) unedited.
@task_item n=14 part=touches
decision/boxless-focused-select-keeps-focus — the click handler's focus clearing after the walk is unchanged; the new check asserts an unmatched click clears focus, which then reads as the root element, never none.
@task_item n=15 part=touches
contract/escher-session-tasks/counter — the hand smoke drove the counter task (a click to 1, the button focused) on the restructured click path; unchanged.
@task_item n=16 part=untouched
Color palette, typography, spacing, depth strategy, border radius and iconography rows — no colour, font, spacing, paint order, stacking, cursor or favicon rule moved; the screenshot and select-list colours unchanged.
@task_item n=17 part=untouched
Motion rows on animation, smooth scroll, overlay scrollbar fade and thumb drag — pointermove and pointerdown handling were left as they are.
@task_item n=18 part=untouched
escher-session exit statuses and exit codes, stderr and session error objects, the mcp role results and stdout lines — no refusal cause, exit status or output form added or changed.
@task_item n=19 part=untouched
WPT runner output and result colours, the shell provider, the JS window object, MountedData during a render, and the dioxus-native configuration rows — none of their components was edited.
@task_item n=20 part=untouched
Coverage check files and the coverage line contracts — the coverage readings moved are CI figures, not the coverage verb's read bound, refusal or content.
@task_item n=21 part=untouched
The web-spa and cli surface areas and the brand, anti-pattern, self-validation and design-decisions-log sections — no WASM page, argument parsing or recorded intent moved.
