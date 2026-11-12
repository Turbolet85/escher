@row_none n=1 rec=1 type=4
relied: The Android native view is focusable in touch mode and requests focus, so key presses reach the app.
@row_none n=2 rec=2 type=5
@row_none n=3 rec=3 type=5
@row_none n=4 rec=4 type=5
@row_none n=5 rec=5 type=5
@row_none n=6 rec=6 type=5
@row_none n=7 rec=7 type=5
@row_none n=8 rec=8 type=5
@row_none n=9 rec=9 type=5
@row_none n=10 rec=10 type=5
@row_none n=11 rec=11 type=5
@row_none n=12 rec=12 type=5
@row_none n=13 rec=13 type=5
@row_none n=14 rec=14 type=5
@row_none n=15 rec=15 type=5
@row_none n=16 rec=16 type=5
@row_none n=17 rec=17 type=5
@row_none n=18 rec=18 type=5
@row_none n=19 rec=19 type=5
@row_none n=20 rec=20 type=7
@row_none n=21 rec=21 type=7
@row_none n=22 rec=22 type=7
@row_none n=23 rec=23 type=4
relied: Text nodes become TextRun nodes carrying their text, and their parent is labelled by them, so an element's text reaches its accessible name.
@row_add n=1 row=decision/accessible-name-precedence area=screen-reader-support rec=24,25
name: Accessible name precedence
rule: An aria-labelledby whose targets hold text names an element first, then a non-empty trimmed aria-label, then a bound label; a whitespace-only aria-label names nothing.
why: follows accname 1.2 (2B over 2C and 2D)
@row_add n=2 row=decision/label-names-its-labelable-element area=screen-reader-support rec=25
name: A label names its bound labelable element
rule: A label names its for-target by id, else its first nested labelable element, whether it stands before or after that element; a hidden input is named by no label.
why: follows HTML's labelable elements: button, non-hidden input, meter, output, progress, select, textarea
@row_add n=3 row=contract/accessibility-tree/select-role area=screen-reader-support rec=26
boundary: the accessibility tree
item: select
promise: A select reads ComboBox, a multiple select ListBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes, each selected true or false and named by its non-empty label attribute, else its text.
@row_add n=4 row=contract/snapshot/select-value area=screen-reader-support rec=26
boundary: the snapshot
item: a select's value
promise: A select's value is its chosen option's value; for a multiple select its first chosen option's in tree order; nothing when none is chosen.
@row_add n=5 row=contract/snapshot/closed-select-option-bounds area=screen-reader-support rec=26
boundary: the snapshot
item: a closed select's options
promise: A closed select's options read under it, each with the select's client rect as its bounds.
@row_add n=6 row=contract/snapshot/open-list-row-bounds area=screen-reader-support rec=26
boundary: the snapshot
item: an open list's rows
promise: A select whose list is open is no closed select; an open row's bounds is its painted row's in the snapshot and the diff, the diff's differs naming bounds.
@row_none n=24 rec=27 type=5
@row_add n=7 row=contract/accessibility-tree/select-expanded area=screen-reader-support rec=28
boundary: the accessibility tree
item: a single select's expanded
promise: A single select's expanded comes from engine state, never from an author aria-expanded, and focus stays on the select while its list is open.
@row_add n=8 row=contract/snapshot/open-list-highlighted-row area=screen-reader-support rec=29
boundary: the snapshot and the accessibility tree
item: an open list's highlighted row
promise: An open list's highlighted row is in no snapshot and no tree, a stated limit: the tree says the list is open, the row shows only in a screenshot.
@row_none n=25 rec=30 type=9
@row_add n=9 row=contract/snapshot/multiple-select-row-bounds area=screen-reader-support rec=31
boundary: the snapshot
item: a multiple select's rows and groups
promise: A multiple select's rows and groups read their own client rects as their bounds, unlike a closed select's options.
@row_add n=10 row=contract/form-event-values/multiple-select area=screen-reader-support rec=32
boundary: form event values
item: a multiple select's set
promise: On Input and Change, evt.values() gives one (name, text value) form entry per chosen option of a multiple select, while value stays the select's single value.
@row_add n=11 row=contract/accessibility-tree/author-id area=screen-reader-support rec=33
boundary: the accessibility tree
item: author_id
promise: In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
@row_add n=12 row=decision/interactive-nodes-named area=screen-reader-support rec=33
name: Interactive nodes carry a name
rule: Every interactive-role tree node of the lean stand tasks reads a non-empty accessible name and an HTML-AAM role, in both layout modes.
guard: stand_accessibility_ids
@row_add n=13 row=decision/adapter-tree-is-document-tree area=screen-reader-support rec=33
name: The adapter reads the document's own tree
rule: The tree the shell hands the platform adapter equals the document's accessibility tree node for node, with the same focus and root.
guard: stand_accessibility_ids
@row_add n=14 row=decision/stand-expectation-tables area=screen-reader-support rec=33
name: Stand expectations stated once
rule: The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
@row_add n=15 row=contract/node-id/reserved-separator area=screen-reader-support rec=34
boundary: served node ids
item: the separator ///
promise: No served id holds ///; the form {owner}///{key} is reserved.
@row_add n=16 row=decision/snapshot-reads-the-tree area=screen-reader-support rec=35
name: Snapshot takes its readings from the tree
rule: The snapshot takes each node's role, name, focus and its selected, pressed and invalid readings from the accessibility tree, never from the element, with no second role or name mapping.
@row_add n=17 row=decision/boolean-state-limits area=screen-reader-support rec=35
name: Limits of the boolean state model
rule: A mixed toggle reads pressed false and a grammar or spelling error reads invalid true, the node keeping its full value.
@row_add n=18 row=decision/checked-reading area=screen-reader-support rec=35
name: Source of the checked reading
rule: checked is a native checkbox's or radio's state, else on a checkable role its toggled reading, True true and False and Mixed false, nothing where the role states none.
why: ratified
@row_add n=19 row=decision/enabled-by-disabled-presence area=screen-reader-support rec=35
name: Enabled follows the disabled attribute's presence
rule: A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading.
guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
@row_add n=20 row=contract/driver-refusal/disabled area=screen-reader-support rec=35
boundary: driver refusal causes
item: disabled
promise: A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.
@row_add n=21 row=decision/driver-schema-has-no-role-set area=screen-reader-support rec=35
name: Driver schema defines no role or name set
rule: The driver's schema defines no role or name set of its own; role and name are field names of its result nodes.
@row_add n=22 row=contract/snapshot/value area=screen-reader-support rec=35
boundary: the snapshot
item: value
promise: value is the editor's text, else a range input's current value, else the value attribute.
@row_add n=23 row=decision/password-never-revealed area=screen-reader-support rec=35
name: Password text is never revealed
rule: A password input's typed text is in no snapshot field, tree or painted frame: the snapshot reads MASKED_VALUE where it holds text and empty where none, and blitz-paint paints its value as bullets in every renderer.
why: ratified
@row_add n=24 row=decision/password-caret-by-bullets area=screen-reader-support rec=35
name: Password caret placed by bullets
rule: A click in a password input places the caret by bullets laid out one per character, so no width of the real characters reaches the caret.
why: ratified
guard: password_caret_click
@row_add n=25 row=contract/snapshot/file-input-value area=screen-reader-support rec=35
boundary: the snapshot
item: a file input's value
promise: A file input reads MASKED_VALUE where its value attribute is non-empty, its path in no field of the snapshot.
@row_add n=26 row=decision/readers-of-one-snapshot-model area=screen-reader-support rec=35
name: One snapshot model, many readers
rule: The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping.
@row_add n=27 row=contract/snapshot-text/state-tokens area=screen-reader-support rec=35
boundary: the snapshot text form
item: state tokens
promise: Each node is one line; selected or unselected, pressed or unpressed and invalid are written after value= and before focused.
@row_add n=28 row=contract/snapshot-diff/entries area=screen-reader-support rec=35
boundary: the snapshot diff
item: entries
promise: Two snapshots are compared by stable element id, entries carrying the later role and name; a text change is reported on the element whose name it changes, never as a text-run entry.
@row_add n=29 row=contract/snapshot-diff/differs area=screen-reader-support rec=35
boundary: the snapshot diff
item: differs
promise: A changed node names the readings that differ from before the step, in node-key order.
@row_add n=30 row=contract/snapshot-text/parts area=screen-reader-support rec=35
boundary: the snapshot text form
item: parts
promise: A screen over the text budget is read in parts of consecutive pre-order lines, never a split line, concatenating to the whole text with every node id on exactly one line.
@row_add n=31 row=contract/mcp-result/snapshot area=screen-reader-support rec=35
boundary: MCP tool results
item: snapshot
promise: An accepted snapshot result is the text form itself for a whole screen, and for a screen in parts the JSON line {text, part, parts}; an acting call's result is its JSON line with the diff.
@row_add n=32 row=contract/driver-screenshot/element-shot area=screen-reader-support rec=35
boundary: the driver's screenshot
item: an element shot
promise: An element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen.
@row_add n=33 row=decision/screenshot-changes-nothing area=screen-reader-support rec=35
name: A screenshot changes nothing
rule: Taking a screenshot moves no focus, leaves has_changes false and leaves the next diff empty.
@row_add n=34 row=contract/snapshot-node/clickable area=screen-reader-support rec=36
boundary: the snapshot node
item: clickable
promise: clickable is true where the element's current listeners hold click and it is no disabled form control; the JSON node always carries it beside focused.
@row_add n=35 row=contract/snapshot-text/clickable area=screen-reader-support rec=36
boundary: the snapshot text form
item: clickable
promise: The token clickable is written after focused only when true.
@row_add n=36 row=decision/ui-graph-carries-no-values area=screen-reader-support rec=37,34
name: UI graph is read-only and carries no values
rule: A UI graph read changes nothing and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text.
@row_add n=37 row=contract/handler-coverage/answer area=screen-reader-support rec=38
boundary: handler coverage
item: answer
promise: The answer lists the graph's handlers no check reached and holds ids, owner ids, event names and two counts, checks and held, only, no role or name.
@row_add n=38 row=decision/actions-verb-changes-nothing area=screen-reader-support rec=39
name: The actions verb changes nothing
rule: Answering the actions verb moves no focus, snapshot or diff, and an open list stays open.
@row_add n=39 row=contract/actions-answer/causes area=screen-reader-support rec=39
boundary: the actions answer
item: causes
promise: For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.
@row_add n=40 row=contract/actions-answer/fields area=screen-reader-support rec=39
boundary: the actions answer
item: fields
promise: The answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId.
@row_none n=26 rec=40 type=5
@row_add n=41 row=absence/snapshot-list-open-state area=screen-reader-support rec=41
capability: A snapshot reading of whether a list is open (no expanded node key)
@row_add n=42 row=decision/option-labels-leave-only-as-pixels area=screen-reader-support rec=42
name: Option labels leave an open list only as pixels
rule: An option's label and a group's label leave an open list only as painted pixels in a screenshot and the snapshot's name, with no log site.
why: ratified
@row_add n=43 row=contract/driver-screenshot/row-shot area=screen-reader-support rec=42
boundary: the driver's screenshot
item: a shot of an open list's row
promise: A shot of a row crops to it and leaves the list open.
@row_add n=44 row=contract/accessibility-tree/native-checked area=screen-reader-support rec=43
boundary: the accessibility tree
item: a native checkbox's or radio's checked state
promise: On a checkable role toggled reads True or False from the native checked state, no author attribute read on a native control; under any other role a native control writes nothing.
@row_add n=45 row=decision/actionable-key-rule area=screen-reader-support rec=44
name: Actionable elements carry an author key
rule: Every element an agent can act on, focusable, carrying an interactive role or an event listener, reads an author key, so its author_id holds under any edit of the code around it.
why: ratified
guard: unkeyed_actionable
@row_add n=46 row=decision/one-actionable-rule area=screen-reader-support rec=44,53
name: One actionable rule
rule: One actionable rule, focusable or an interactive role or current listeners holding an event other than mounted, is shared by the unkeyed check and the actionable-node list, with no second role mapping.
@row_add n=47 row=contract/actionable-nodes/listing area=screen-reader-support rec=45
boundary: actionable nodes
item: listing
promise: The list holds, in snapshot pre-order, the UI graph's nodes the actionable rule holds for or whose owner is set, each with id, owner and keyed.
@row_add n=48 row=decision/keyed-wire-form area=screen-reader-support rec=46,45
name: The keyed field on the carriers
rule: The keyed reading on the actions answer is the actionable-key check's wire form, true where the id holds no slash.
state: provisional
@row_none n=27 rec=47 type=5
@row_none n=28 rec=48 type=5
@row_none n=29 rec=49 type=5
@row_none n=30 rec=50 type=5
@row_none n=31 rec=51 type=5
@row_add n=49 row=decision/listeners-from-vdom-walk area=screen-reader-support rec=52,54
name: Listeners read from the vdom walk
rule: An element's current listeners are read from the vdom walk's dynamic attributes, never from the bridge's listener mark, so a listener removed on a re-render is not counted.
why: the bridge's listener attributes over-report: a removed listener leaves its event attribute and data-dioxus-id mark standing
guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
@row_none n=32 rec=55 type=9
