@row_add n=1 row=contract/range-input/click area=5-keyboard-navigation rec=1
boundary: range input
item: click
promise: A click on a range input focuses it and changes no value.
@row_add n=2 row=contract/range-input/main-button-pointerdown area=5-keyboard-navigation rec=1
boundary: range input
item: main-button pointerdown
promise: A main-button pointerdown on a range input that is not disabled sets the value from the hit's fraction of the content box, clamped to [0, 1].
@row_none n=1 rec=2 type=S
@row_add n=3 row=contract/range-input/arrowright-arrowup area=5-keyboard-navigation rec=3
boundary: range input
item: ArrowRight / ArrowUp
promise: A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp.
@row_add n=4 row=contract/range-input/arrowleft-arrowdown area=5-keyboard-navigation rec=3
boundary: range input
item: ArrowLeft / ArrowDown
promise: A focused range input with no disabled attribute steps down on ArrowLeft and ArrowDown.
@row_add n=5 row=contract/range-input/home area=5-keyboard-navigation rec=3
boundary: range input
item: Home
promise: A focused range input with no disabled attribute steps to its minimum on Home.
@row_add n=6 row=contract/range-input/end area=5-keyboard-navigation rec=3
boundary: range input
item: End
promise: A focused range input with no disabled attribute steps to its maximum on End.
@row_add n=7 row=contract/range-input/other-keys area=5-keyboard-navigation rec=3
boundary: range input
item: other keys
promise: On a focused range input every key other than the arrows, Home and End does nothing.
@row_add n=8 row=contract/range-input/value area=5-keyboard-navigation rec=3
boundary: range input
item: value
promise: A range input's value is held apart from the value attribute and sanitized to [min, max] on the step grid.
@row_add n=9 row=contract/range-input/input-event area=5-keyboard-navigation rec=3
boundary: range input
item: input event
promise: Each change of a range input's value fires one input event; a move that changes nothing fires none.
@row_add n=10 row=contract/select-closed-list/arrowdown-arrowup area=5-keyboard-navigation rec=4
boundary: single-choice select, list closed
item: ArrowDown / ArrowUp
promise: A focused single-choice select that is no disabled form control, its list closed, moves to the next / previous enabled option on ArrowDown / ArrowUp.
@row_add n=11 row=contract/select-closed-list/home-end area=5-keyboard-navigation rec=4
boundary: single-choice select, list closed
item: Home / End
promise: A focused single-choice select that is no disabled form control, its list closed, moves to the first / last enabled option on Home / End.
@row_add n=12 row=contract/select-closed-list/typeahead area=5-keyboard-navigation rec=4
boundary: single-choice select, list closed
item: typeahead
promise: A character typed with no Ctrl, Alt or Meta on a focused closed single-choice select is typeahead, kept for 1 s of the document's clock.
@row_add n=13 row=contract/select-closed-list/enter-space area=5-keyboard-navigation rec=4
boundary: single-choice select, list closed
item: Enter / Space
promise: Enter, and a Space with no typed text standing, open a focused single-choice select's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select opens nothing.
@row_add n=14 row=contract/select-closed-list/other-keys area=5-keyboard-navigation rec=4
boundary: single-choice select, list closed
item: other keys
promise: On a focused closed single-choice select every key other than the arrows, Home, End, typeahead, Enter and Space does nothing.
@row_add n=15 row=contract/single-select/change-events area=5-keyboard-navigation rec=4
boundary: single-choice select
item: input and change events
promise: Each change of a single-choice select's value dispatches one input then one change; a key that changes nothing dispatches none.
@row_add n=16 row=contract/select-open-list/navigation-keys area=5-keyboard-navigation rec=4
boundary: single-choice select, list open
item: navigation keys
promise: With a select's list open, the engine paints the list over the page, focus stays on the select, and the select's keys move the highlight over enabled rows, choosing nothing.
@row_add n=17 row=contract/select-open-list/enter-space area=5-keyboard-navigation rec=4
boundary: single-choice select, list open
item: Enter / Space
promise: With a select's list open, Enter and Space choose the highlighted row and close the list.
@row_add n=18 row=contract/select-open-list/tab area=5-keyboard-navigation rec=4
boundary: single-choice select, list open
item: Tab
promise: With a select's list open, Tab in either direction chooses the highlighted row and closes the list before focus moves.
@row_add n=19 row=contract/select-open-list/escape area=5-keyboard-navigation rec=4
boundary: single-choice select, list open
item: Escape
promise: With a select's list open, Escape closes the list with the value unchanged and no event, as a second click, a press outside and a page scroll do.
@row_add n=20 row=contract/select-open-list/close area=5-keyboard-navigation rec=4
boundary: single-choice select, list open
item: close
promise: Closing a select's list clears the typed typeahead text.
@row_add n=21 row=contract/single-select/optgroup-options area=5-keyboard-navigation rec=4
boundary: single-choice select
item: optgroup options
promise: An optgroup's options are the select's own options, in tree order.
@row_add n=22 row=contract/multiple-select/keys-and-row-clicks area=5-keyboard-navigation rec=4
boundary: multiple select
item: keys and row clicks
promise: A multiple select takes its own keys and row clicks, apart from the single-choice select's handling.
@row_add n=23 row=contract/keyboard-activation/enter area=5-keyboard-navigation rec=5
boundary: keyboard activation
item: Enter
promise: A pressed Enter on the focused node dispatches a Click when the node is no disabled form control and is a button, a summary, an input of type submit, reset or button, or an a with href.
@row_add n=24 row=contract/keyboard-activation/space area=5-keyboard-navigation rec=5
boundary: keyboard activation
item: Space
promise: A released Space dispatches a Click for a summary, or for a button or an input of type submit, reset, button, checkbox or radio that is no disabled form control, never for a link.
@row_add n=25 row=contract/keyboard-activation/handler-order area=5-keyboard-navigation rec=5
boundary: keyboard activation
item: handler order
promise: On a keyboard-dispatched Click the app's handlers run first, then the engine's handle_click.
@row_none n=2 rec=6 type=S
@row_add n=26 row=decision/falsy-boolean-attribute-removed area=5-keyboard-navigation rec=7
name: Falsy boolean attribute removed
rule: A falsy Dioxus value of one of the bridge's boolean attributes (upstream's is_bool_attr set) removes the attribute on both attribute paths; a truthy value is written, and an attribute outside the list holding false is written as "false".
why: So an enabled control does not match :disabled and hidden: false keeps an element laid out and in the tree.
@row_none n=3 rec=8 type=S
@row_add n=27 row=decision/active-descendant-unread area=5-keyboard-navigation rec=9,40
name: aria-activedescendant kept unread
rule: The CRUD person rows are listbox options that are not focusable, the tree's focus is the list and never the option, and the tree builder reads no aria-activedescendant.
why: SC 4.1.2 work outside the tier-0 target.
guard: accessibility_active_descendant_unread
@row_add n=28 row=contract/crud-list/tab-stop area=5-keyboard-navigation rec=9
boundary: seven_guis CRUD list
item: Tab stop
promise: The list crud-list, a listbox named People, is the third CRUD Tab stop; its person rows are not focusable.
@row_add n=29 row=contract/crud-list/arrowdown-arrowup-home-end area=5-keyboard-navigation rec=9
boundary: seven_guis CRUD list
item: ArrowDown / ArrowUp / Home / End
promise: With crud-list focused, ArrowDown, ArrowUp, Home and End move the selection as a row's click does.
@row_add n=30 row=contract/cells-grid/tab-stop area=5-keyboard-navigation rec=10
boundary: seven_guis Cells grid
item: Tab stop
promise: cells-grid, named Cells by aria_label with no role, is one Tab stop.
@row_add n=31 row=contract/cells-grid/arrows area=5-keyboard-navigation rec=10
boundary: seven_guis Cells grid
item: arrows
promise: The arrows move the Cells grid's current cell, clamped, and bring the cell into view.
@row_add n=32 row=contract/cells-grid/enter area=5-keyboard-navigation rec=10
boundary: seven_guis Cells grid
item: Enter
promise: Enter on the Cells grid opens the editor on the current cell, focused.
@row_add n=33 row=contract/cells-grid/editor-enter area=5-keyboard-navigation rec=10
boundary: seven_guis Cells grid
item: editor Enter
promise: Enter in the Cells editor commits and returns focus to the grid.
@row_add n=34 row=contract/cells-grid/tab-from-editor area=5-keyboard-navigation rec=10
boundary: seven_guis Cells grid
item: Tab from editor
promise: Tab leaving an open Cells editor commits it.
@row_add n=35 row=contract/cells-grid/focus-outline area=5-keyboard-navigation rec=10
boundary: seven_guis Cells grid
item: focus outline
promise: The focused Cells grid draws a 2px outline in the stand accent on the current cell.
@row_none n=4 rec=11 type=S
@row_none n=5 rec=12 type=S
@row_none n=6 rec=13 type=S
@row_add n=36 row=decision/stated-tab-order-walked area=5-keyboard-navigation rec=14
name: Stated Tab order walked
rule: Each lean task, the load page and Cells state their Tab order, walked forward and back in both layout modes with the engine's focus, the snapshot's focused node and the tree's focus agreeing at every step, and every lean task's control lying in one of the two lean orders.
guard: stand_keyboard
@row_none n=7 rec=15 type=5
@row_add n=37 row=decision/select-list-coverage-headless area=5-keyboard-navigation rec=16
name: Select list coverage headless
rule: The select list's keyboard and focus coverage is headless, asserting SC 2.1.1 and 2.4.3 on it in both layout modes; the windowed list is read and not run.
guard: select_popup, stand_act_popup, stand_keyboard
@row_none n=8 rec=17 type=7
@row_none n=9 rec=18 type=S
@row_none n=10 rec=19 type=S
@row_add n=38 row=decision/stand-contrast-measured area=6-visual-design-verification rec=20
name: Stand contrast measured from computed styles
rule: SC 1.4.3 is measured on the stand from computed styles with the WCAG 2.x ratio, 3.0 required of large text and else 4.5, a row exempt exactly where the snapshot's enabled reads Some(false).
guard: stand_contrast, the_load_page_meets_its_ratios, cells_meets_its_ratios
@row_add n=39 row=decision/lint-contrast-single-formula area=6-visual-design-verification rec=21
name: Lint contrast through one formula
rule: The driver's visual lint reads contrast through the harness's text contrast with the stand's exempt rule, never a second formula or threshold.
@row_add n=40 row=contract/visual-lint-finding/content area=6-visual-design-verification rec=21
boundary: visual lint finding
item: content
promise: A visual lint finding is written as ids, closed words and numbers only.
@row_add n=41 row=decision/select-list-colours-engine-constants area=6-visual-design-verification rec=22
name: Select list colours as engine constants
rule: An open select list's colours are engine constants, in no style sheet, and its highlight and disabled rows are asserted against them in the harness.
guard: select_popup_painted
@row_none n=11 rec=23 type=7
@row_none n=12 rec=24 type=S
@row_none n=13 rec=25 type=S
@row_add n=42 row=decision/select-list-no-motion area=6-visual-design-verification rec=26
name: Select list without motion
rule: A select list opens and closes with no motion.
@row_none n=14 rec=27 type=7
@row_none n=15 rec=28 type=S
@row_none n=16 rec=29 type=S
@row_none n=17 rec=30 type=S
@row_add n=43 row=contract/accessibility-tree/hidden-and-inert-nodes area=6-visual-design-verification rec=31
boundary: accessibility tree
item: hidden and inert nodes
promise: Nodes with display: none or visibility: hidden or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.
@row_add n=44 row=contract/hit-testing/inert area=6-visual-design-verification rec=31
boundary: hit-testing
item: inert
promise: A hit passes through an element carrying inert, and removing inert restores the tree and the hit.
@row_none n=18 rec=32 type=S
@row_none n=19 rec=33 type=S
@row_add n=45 row=contract/accessible-name/aria-label area=7-screen-reader-support rec=34
boundary: accessible name
item: aria-label
promise: An element's aria-label, non-empty after trimming, becomes its label; a whitespace-only one names nothing.
@row_add n=46 row=contract/accessible-name/aria-labelledby area=7-screen-reader-support rec=34,35
boundary: accessible name
item: aria-labelledby
promise: An aria-labelledby whose targets hold text outranks aria-label and a bound label element; aria-label is the fallback when the targets hold none.
@row_add n=47 row=contract/accessible-name/label-element area=7-screen-reader-support rec=35
boundary: accessible name
item: label element
promise: A label names the labelable element bound to it, its for-target by id or else its first nested labelable element, whether placed before or after it; a hidden input is named by no label.
@row_add n=48 row=contract/accessibility-tree/author-id area=7-screen-reader-support rec=36
boundary: accessibility tree
item: author_id
promise: In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
@row_add n=49 row=decision/adapter-tree-equals-harness-tree area=7-screen-reader-support rec=36
name: Adapter tree equals harness tree
rule: On the stand the tree the shell hands the adapter equals the harness's tree node for node, and every interactive-role node of the four lean tasks reads a non-empty name.
@row_add n=50 row=decision/role-name-tables-stated-once area=7-screen-reader-support rec=36,37
name: Expected roles and names stated once
rule: The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
@row_add n=51 row=decision/snapshot-reads-accessibility-tree area=7-screen-reader-support rec=37
name: Snapshot reads the accessibility tree
rule: The snapshot takes each node's role, name, focus, selected, pressed and invalid from the accessibility tree, never from the element.
@row_add n=52 row=decision/snapshot-model-single-source area=7-screen-reader-support rec=37
name: Snapshot model as single source
rule: The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else.
@row_add n=53 row=decision/ui-graph-one-node-per-snapshot-node area=7-screen-reader-support rec=37
name: UI graph mirrors the snapshot
rule: The read-only UI graph holds one node per snapshot node, names and closed words only.
@row_add n=54 row=contract/snapshot/enabled area=7-screen-reader-support rec=37
boundary: snapshot
item: enabled
promise: The snapshot's enabled field reads the presence of the element's disabled attribute.
@row_add n=55 row=contract/snapshot/value area=7-screen-reader-support rec=37
boundary: snapshot
item: value
promise: The snapshot's value field is read from the element.
@row_add n=56 row=contract/snapshot/checked area=7-screen-reader-support rec=37
boundary: snapshot
item: checked
promise: The snapshot's checked field reads a native checkbox's or radio's state, else a checkable role's toggled.
@row_add n=57 row=contract/snapshot/clickable area=7-screen-reader-support rec=37,39
boundary: snapshot
item: clickable
promise: The snapshot's clickable field says an element that is no disabled form control reacts to a click, read from the vdom walk's current click listeners and not the listener mark.
@row_add n=58 row=decision/password-value-masked area=7-screen-reader-support rec=38
name: Password value masked
rule: A password input's typed text appears in no snapshot field, tree node or painted frame; its value reads a fixed masked marker where it holds text and an empty string where it holds none.
@row_add n=59 row=contract/snapshot/password-value area=7-screen-reader-support rec=38
boundary: snapshot
item: password input value
promise: A password input's value reads MASKED_VALUE where it holds text and "" where it holds none.
@row_add n=60 row=contract/snapshot/file-input-value area=7-screen-reader-support rec=38
boundary: snapshot
item: file input value
promise: A file input with a non-empty value reads MASKED_VALUE.
@row_add n=61 row=contract/driver-actions/disabled area=7-screen-reader-support rec=38
boundary: driver actions
item: disabled
promise: The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.
@row_add n=62 row=decision/actionable-key-rule area=7-screen-reader-support rec=39
name: Actionable-key rule
rule: Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key.
why: So the author_id holds under any edit of the code around the element.
guard: unkeyed_actionable
@row_add n=63 row=decision/one-actionable-rule area=7-screen-reader-support rec=39
name: One actionable rule
rule: The unkeyed-actionable check and the actionable-nodes read share one actionable rule, its listener leg the vdom walk's exact current listeners holding an event other than mounted; the data-dioxus-id mark serves event dispatch alone.
@row_add n=64 row=contract/accessibility-tree/multiple-select area=7-screen-reader-support rec=41
boundary: accessibility tree
item: multiple select
promise: A multiple select carries set_multiselectable in the tree.
@row_add n=65 row=contract/snapshot/multiple-select-rows-and-value area=7-screen-reader-support rec=41
boundary: snapshot
item: multiple select rows and value
promise: A multiple select's rows read their own client rects in the snapshot, and its value reads its first chosen option in tree order.
@row_add n=66 row=contract/accessibility-tree/optgroup area=7-screen-reader-support rec=41
boundary: accessibility tree
item: optgroup
promise: An optgroup reads Role::Group, named by its non-empty label attribute.
@row_add n=67 row=contract/select-open-list/optgroup-header area=7-screen-reader-support rec=41
boundary: single-choice select, list open
item: optgroup header
promise: An optgroup is painted as a header of an open list, heavier than a row by weight on every build, and a header chooses nothing.
@row_add n=68 row=contract/accessibility-tree/single-select area=7-screen-reader-support rec=42
boundary: accessibility tree
item: single select
promise: A single select reads ComboBox and writes expanded from its list's open state, reading no aria-expanded; an author's open or aria-expanded on it steers nothing; its chosen option reads selected and the tree's focus stays on the select.
@row_add n=69 row=contract/snapshot/select-list-rows area=7-screen-reader-support rec=42
boundary: snapshot
item: select list rows
promise: The snapshot holds no expanded key and the highlighted row is in no snapshot and no tree; an open list's row reads its own bounds in the list.
@row_none n=20 rec=43 type=S
@row_none n=21 rec=44 type=S
@row_add n=70 row=contract/accesskit-adapter/creation-order area=7-screen-reader-support rec=45
boundary: AccessKit adapter
item: creation order
promise: Adapter creation must happen before the window is first shown; it panics if the window is already visible.
@row_none n=22 rec=46 type=S
@row_none n=23 rec=47 type=7
@row_none n=24 rec=48 type=S
@row_none n=25 rec=49 type=S
@row_none n=26 rec=50 type=S
@row_none n=27 rec=51 type=S
@row_none n=28 rec=52 type=S
@row_add n=71 row=decision/invalid-state-not-colour-alone area=8-cognitive-accessibility rec=53
name: Invalid state not by colour alone
rule: Invalid dates carry an invalid class with red styling and disable the Book button, and the two date inputs also state aria-invalid, so the invalid state is in the tree and the snapshot and not carried by the class alone.
@row_none n=29 rec=54 type=S
@row_none n=30 rec=55 type=S
