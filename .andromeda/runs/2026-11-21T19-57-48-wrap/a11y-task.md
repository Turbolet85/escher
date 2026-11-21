@task_prose part=session
The engine now reads `size` on a single-choice select in every document: a display size above 1 makes it an in-flow list box of its rows, display size 1 (absent, 1, 0, empty or unparsable) keeps the drop-down.
A single list box opens no list on a click, Enter or Space; its arrows, Home, End and typeahead choose a row through the one write path and scroll it into view; a click on a row focuses the select and chooses that row alone.
In the accessibility tree a select reads ListBox wherever it is a list box (multiple, or display size above 1), else ComboBox; a single list box writes no expanded, no active descendant and reads no author aria-expanded.
A single list box may stand with nothing chosen; the snapshot bridge no longer treats it as a closed select, so its rows carry their own bounds and a click on one is accepted, not covered.
Measured: a list-box page with rows scrolled out of the box lints four findings at boot (clipped, truncated, off-screen; no overlap), and a node a restyle turns display none keeps its last client rect.
Retired: the stated limit that a single select's size is unread; the driver's click and press help texts re-worded, no verb, key, field or cause added.
@task_file n=1 path=packages/blitz-dom/src/node/select.rs
what: the select model readings — display size, list box or drop-down, the settled (first enabled) choice, typeahead and stepping
@task_file n=2 path=packages/blitz-dom/src/node/select_list.rs
what: the open select list and its open gate, which now admits a drop-down alone
@task_file n=3 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: the presentational hints — the size attribute's rows height and the list-box rows and chosen-row colour pair
@task_file n=4 path=packages/blitz-dom/src/mutator.rs
what: the attribute write path — a size or multiple write restyles the select's rows and re-settles its choice
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: the engine document — the new public is_list_box reading and the select choice settle
@task_file n=6 path=packages/blitz-dom/src/events/keyboard.rs
what: the keyboard event dispatch — the select keys, now split into drop-down, single list box and multiple select keys
@task_file n=7 path=packages/blitz-dom/src/events/pointer.rs
what: the pointer and click handling — select click opening a list, and a single list box's row click
@task_file n=8 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree builder — the select role (ListBox or ComboBox), expanded and active descendant
@task_file n=9 path=packages/blitz-dom/src/layout_reason.rs
what: the layout reason reading — a list box's option reads its own placing
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot builder and its closed-select bridge, which answers none for a list box
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: the driver verb schema — the click and press help texts
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema tests over the help texts
@task_file n=13 path=docs/capabilities.md
what: the capability statement — the select model and its stated limits
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs
what: the select interaction checks — role, keys, row clicks, display size and size writes of a sized single select
@task_file n=15 path=tests/blitz-tests/tests/select_popup.rs
what: the select popup checks — a sized single select opens no list
@task_file n=16 path=tests/blitz-tests/tests/stand_act_select.rs
what: the driver-level select checks — snapshot role and row bounds, click, actions, press, type and boot lint on a list-box page
@task_item n=1 part=touches
contract/accessibility-tree/select-role and contract/accessibility-tree/single-select: a select no longer reads ListBox only when multiple; a single select of display size above 1 reads ListBox, only a drop-down reads ComboBox.
@task_item n=2 part=touches
decision/single-select-expanded-from-list and contract/accessibility-tree/select-expanded: expanded and the active descendant are written on a drop-down alone; a single list box writes neither and reads no author aria-expanded.
@task_item n=3 part=touches
contract/aria-property-reads/aria-expanded: an author aria-expanded is unread on a single list box, as on a drop-down.
@task_item n=4 part=touches
decision/single-select-engine-overlay-list and decision/select-list-coverage-headless: only a drop-down (display size 1) opens the engine overlay list; a sized single select opens none.
@task_item n=5 part=touches
contract/select-closed-list/enter-space: Enter and Space open nothing on a single list box (Space joins typeahead only while typed text stands).
@task_item n=6 part=touches
contract/select-closed-list/arrowdown-arrowup, home-end, typeahead and other-keys: these now hold for a drop-down alone; a single list box has its own key set that chooses and scrolls the row into view with no list.
@task_item n=7 part=touches
contract/select-open-list rows, decision/focus-stays-on-select-with-list-open and decision/tab-chooses-and-closes-select-list: an open list is now a drop-down's alone; a size write turning a drop-down into a list box closes its open list.
@task_item n=8 part=touches
decision/select-list-choice-one-write-path and decision/select-key-change-fires-input-change: the list box's keys and row clicks are new callers of the one choice write path, firing input then change.
@task_item n=9 part=touches
decision/role-mapping-html-aam and decision/html-aam-role-mapping: the select row of the role mapping now follows HTML-AAM's size rule (listbox for multiple or size above 1).
@task_item n=10 part=touches
contract/snapshot/closed-select-option-bounds: the closed-select bridge answers none for a list box, so a single list box's options are no closed select's options; they read their own bounds.
@task_item n=11 part=touches
contract/snapshot/multiple-select-row-bounds and contract/snapshot/select-value: a single list box's rows read their own bounds as a multiple select's do, and a list box with nothing chosen reads no value.
@task_item n=12 part=touches
contract/driver-click-refusals/covered: a click on a single list box's row is accepted, not refused covered.
@task_item n=13 part=touches
decision/multiple-select-chosen-row-pair and decision/wcag-contrast-thresholds: the chosen-row pair #000000 on #c8c8c8 (12.56:1) now also paints a single list box's chosen row.
@task_item n=14 part=touches
decision/stand-lint-clean-at-boot: a list-box page whose box scrolls rows out reads four lint findings at boot (clipped, truncated, off-screen; no overlap), the lint's reading of any scrolling box's hidden rows.
@task_item n=15 part=touches
contract/accessibility-tree-states/select-option and contract/accessibility-tree-states/aria-selected: a single list box's rows read selected from the chosen row, and none may be chosen.
@task_item n=16 part=touches
decision/driver-type-focuses-select-no-click: type on a single list box chooses by the text's start through the list box's typeahead.
@task_item n=17 part=touches
decision/active-descendant-unread and decision/active-descendant-kept-unread: the active descendant is written on a drop-down alone, never on a list box.
@task_item n=18 part=touches
decision/hidden-content-excluded-from-a11y-tree and contract/accessibility-tree-hidden-content/display-none: measured that a node a restyle turns display none keeps its last client rect; a check reads computed display beside the rect.
@task_fact n=1 slug=keyboard-navigation
fact: A single-choice select of display size above 1 is a list box: arrows, Home and End step and choose, a printable key with no Ctrl, Alt or Meta is typeahead, the reached row is scrolled into view; Enter, Space without typed text and every other key do nothing, and no list opens.
@task_fact n=2 slug=keyboard-navigation
fact: A click on a single list box's row focuses the select, clears its typeahead and chooses that row alone whatever modifier is held; a disabled select or row changes nothing.
@task_fact n=3 slug=aria-patterns-roles
fact: Display size is the select's size attribute parsed above 1, else 1 (absent, 1, 0, negative, empty or unparsable); a select is a list box when multiple or of display size above 1, a drop-down otherwise.
@task_fact n=4 slug=screen-reader-support
fact: A single list box reads ListBox with no expanded, no active descendant, no author aria-expanded and no multiselectable.
@task_fact n=5 slug=keyboard-navigation
fact: A single list box may stand with nothing chosen; only a drop-down settles to its first enabled option, including a list box turned back into a drop-down by a size write.
@task_fact n=6 slug=keyboard-navigation
fact: A size or multiple write moves a select between list box and drop-down, restyles its rows and closes an open list.
@task_fact n=7 slug=screen-reader-support
fact: The snapshot reads a single list box's rows with their own bounds, actions lists them with no cause, and a click on a row is accepted with the diff naming value and the rows' selected.
@task_fact n=8 slug=visual-design-verification
fact: A single list box's chosen row paints the engine pair #000000 on #c8c8c8, 12.56:1, as a multiple select's does.
@task_fact n=9 slug=visual-design-verification
fact: The lint reads rows scrolled out of a scrolling list box as clipped, truncated or off-screen findings, never overlap; a list-box page with hidden rows is no lint-clean page at boot.
@task_item n=1 part=untouched
The range input, CRUD list and Cells grid keyboard contracts: no key or focus rule of theirs moved.
@task_item n=2 part=untouched
Native checkbox and radio toggle, checked state and keyboard activation rows: unchanged.
@task_item n=3 part=untouched
Accessible name precedence, label, aria-label and aria-labelledby rows: no name rule moved.
@task_item n=4 part=untouched
Password masking, file input value and the UI graph state registration rows: not reached.
@task_item n=5 part=untouched
The platform adapter, the accessibility feature defaults and the stand binaries' adapter absence: no feature or adapter change.
@task_item n=6 part=untouched
The CI legs and gate rows (a11y leg, slow tier, three-round stand repeats): no leg, gate or threshold changed; test counts moved in no row's cell.
@task_item n=7 part=untouched
Cognitive accessibility rows (invalid state not by colour or class alone, shared Tab order table): not reached.
@task_item n=8 part=untouched
The obs log format and the a11y assertion harness schema: no log site, span or field added.
@task_item n=9 part=untouched
Inert, hidden and aria-hidden exclusion rules and inert hit-testing: their rules stand unchanged.
