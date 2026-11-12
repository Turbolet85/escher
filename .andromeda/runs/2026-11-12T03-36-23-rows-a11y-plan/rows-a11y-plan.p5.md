@row_none n=1 rec=1 type=4
relied: Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control.
@row_none n=2 rec=2 type=4
relied: A page's script hears exactly one change per checkable click, from the engine, since the script runtime synthesizes none.
@row_add n=1 row=decision/range-click-focuses-not-value area=keyboard-navigation rec=3
name: Range input click gives focus, not value
rule: A click on a range input, including a label's synthetic click, focuses the input and changes no value.
why: ratified
@row_add n=2 row=decision/range-pointerdown-sets-value area=keyboard-navigation rec=3
name: Range input pointerdown sets value
rule: A main-button pointerdown on an enabled range input sets its value from the hit's fraction of the content box, clamped to [0, 1], a content box of no width reading 0.
why: ratified
@row_none n=3 rec=4 type=4
@row_add n=3 row=decision/range-input-keys area=keyboard-navigation rec=5
name: Range input keys
rule: A focused enabled range input steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum on Home and maximum on End, every other key doing nothing; each change fires one input event and a move that changes nothing fires none.
why: ratified
@row_none n=4 rec=6 type=4
relied: Each key change on a single select dispatches one input then one change carrying the select's value, a key that changes nothing firing none.
@row_none n=5 rec=7 type=4
@row_none n=6 rec=8 type=4
relied: Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing.
@row_none n=7 rec=9 type=4
@row_none n=8 rec=10 type=4
@row_none n=9 rec=11 type=4
@row_none n=10 rec=12 type=4
@row_none n=11 rec=13 type=4
@row_none n=12 rec=14 type=4
@row_none n=13 rec=15 type=4
@row_none n=14 rec=16 type=4
relied: A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share.
@row_none n=15 rec=17 type=4
@row_none n=16 rec=18 type=4
relied: The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits.
@row_none n=17 rec=19 type=9
@row_add n=4 row=decision/keyboard-activation-enter-space area=keyboard-navigation rec=20
name: Enter and Space activate as a click
rule: A pressed Enter on a focused enabled button, summary, submit/reset/button input or link with href, and a released Space on a focused enabled button, summary, or submit/reset/button/checkbox/radio input, dispatch a synthetic Click that runs the app's handlers and then the default click action.
why: ratified; a disabled form control takes neither key
guard: stand_keyboard, stand_act_keys
except: A disabled attribute on a summary or a link gates neither key, as HTML does; a link takes no Space.
@row_none n=18 rec=21 type=4
@row_none n=19 rec=22 type=4
@row_none n=20 rec=23 type=4
@row_none n=21 rec=24 type=4
relied: A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it.
@row_none n=22 rec=25 type=4
relied: A synthetic click is a primary mouse click at the node's center.
@row_none n=23 rec=26 type=4
@row_none n=24 rec=27 type=4
@row_none n=25 rec=28 type=4
@row_none n=26 rec=29 type=4
@row_none n=27 rec=30 type=4
@row_none n=28 rec=31 type=4
@row_none n=29 rec=32 type=4
@row_none n=30 rec=33 type=4
relied: A script's element.style read or write goes through the inline style, so display and visibility reach the tree's hidden-content exclusion.
@row_none n=31 rec=34 type=4
@row_none n=32 rec=35 type=4
@row_add n=5 row=contract/ui-graph-state/use-graph-state area=keyboard-navigation rec=36
boundary: UI graph state registration
item: use_graph_state
promise: Apps register state for the UI graph with authored DeclaredWrite declarations, each naming an element's stable id or a prefix ending in *, and an event named without on.
@row_add n=6 row=contract/ui-graph-state/undeclared area=keyboard-navigation rec=36
boundary: UI graph state registration
item: undeclared
promise: A signal write is flagged undeclared where no declaration names it.
@row_add n=7 row=contract/ui-graph-state/unconfirmed area=keyboard-navigation rec=36
boundary: UI graph state registration
item: unconfirmed
promise: A declared write is flagged unconfirmed only once a dispatch of the handler's event passed through its element and no run wrote the state.
@row_add n=8 row=contract/ui-graph-state/no-registered-state area=keyboard-navigation rec=36
boundary: UI graph state registration
item: no registered state
promise: A page with no registered state reaches no registry, and its events dispatch with no registry recording.
@row_add n=9 row=contract/dioxus-document/record-reached area=keyboard-navigation rec=37
boundary: Dioxus document reached-handler record
item: record_reached
promise: Switches on one reached record for the document, a second call keeping the one record; every other document pays one context lookup per dispatch.
@row_add n=10 row=contract/dioxus-document/reached-handlers area=keyboard-navigation rec=37
boundary: Dioxus document reached-handler record
item: reached_handlers
promise: Returns the stable id and event name of each node whose listeners held the dispatched event, in first-reached order, deduplicated by id and event, a handler that removes its own element still counting; empty when the record was never switched on.
@row_none n=33 rec=38 type=4
@row_none n=34 rec=39 type=4
@row_add n=11 row=decision/buffered-dioxus-render area=keyboard-navigation rec=40
name: Dioxus render into a mutation buffer
rule: A Dioxus document renders into a mutation buffer with the document unborrowed, then borrows it once and replays the buffer, so a MountedData call in a task spawned from a key handler acts and does not panic.
why: A task first polled inside the render reaches the document as the same handler calling set_focus with no task does.
guard: mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}
@row_none n=35 rec=41 type=9
@row_none n=36 rec=42 type=6
@row_none n=37 rec=43 type=7
@row_add n=12 row=decision/falsy-boolean-attributes-removed area=keyboard-navigation rec=44
name: Falsy boolean attributes are removed
rule: A falsy Dioxus value of one of the bridge's 28 boolean attributes removes the attribute on both the dynamic and the static template path, while a truthy value is still written.
why: Uses upstream's own boolean-attribute predicate, the mutation writer kept identical to upstream's.
guard: dioxus_falsy_boolean_attrs, dioxus_falsy_disabled
except: An attribute outside the list holding false, such as aria-hidden or a data- attribute, is still written as "false".
@row_add n=13 row=decision/inert-subtree-excluded area=keyboard-navigation rec=44
name: Inert subtree excluded
rule: A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed.
why: ratified
guard: inert_subtree
@row_none n=38 rec=45 type=4
@row_none n=39 rec=46 type=4
@row_none n=40 rec=47 type=4
@row_none n=41 rec=48 type=4
@row_none n=42 rec=49 type=4
@row_none n=43 rec=50 type=4
@row_none n=44 rec=51 type=4
@row_none n=45 rec=52 type=4
@row_none n=46 rec=53 type=4
@row_add n=14 row=decision/active-descendant-unread area=keyboard-navigation rec=54
name: aria-activedescendant left unread
rule: The accessibility tree builder reads no aria-activedescendant, so a focused listbox's current row is learned only from its selected reading, and the tree's focus is the list, never the option.
why: SC 4.1.2 work outside the tier-0 target, kept recorded and unread; ratified
guard: accessibility_active_descendant_unread
@row_none n=47 rec=55 type=4
