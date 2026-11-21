@task_prose part=session
A single select whose size attribute parses above 1 is now an in-flow list box in every document: its rows lay out with boxes, no list opens on click, Enter or Space, and it may stand with nothing chosen; display size 1 (absent, 0, 1, empty or unparsable) keeps the drop-down.
The arrows, Home, End and typeahead choose a row of a focused single list box and scroll it into view; a row click chooses that row alone whatever modifier is held; all choices go through the one select choice write path.
A new public reader, BaseDocument::is_list_box, reads a select with multiple or display size above 1; the open-list gate, the click and key dispatch read a drop-down alone; the accessibility tree reads ListBox for a list box with no expanded or active descendant.
closed_select_of answers None for a list box, so its rows are no longer refused as covered; the layout reason places a list box's option by itself; the driver's click and press help texts were reworded with no verb, argument, cause or count moved.
Retired: the stated limit that a single select's size is unread. Measured: a node a restyle turns display none keeps its last client rect, and a scrolling list box's hidden rows lint as clipped, truncated and off-screen at boot, with no overlap.
@task_file n=1 path=packages/blitz-dom/src/node/select.rs
what: the select rules of blitz-dom (node::select) - display size, list box and drop-down readings, the settle of a single select's choice to its first enabled option, step and typeahead matching
@task_file n=2 path=packages/blitz-dom/src/node/select_list.rs
what: the open select list's state and gate (node::select_list), the gate that decides whether a select may open a list
@task_file n=3 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: legacy attributes as presentational hints - the select size height hint and the list box rows' and chosen row's hints
@task_file n=4 path=packages/blitz-dom/src/mutator.rs
what: DocumentMutator, the DOM mutation path - a size or multiple attribute write restyling a select's descendants and settling its choice
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: BaseDocument, the core document and its select readers - the new public is_list_box reader
@task_file n=6 path=packages/blitz-dom/src/events/keyboard.rs
what: keyboard event default actions - the select keyboard model and its dispatch to drop-down, multiple select and single list box
@task_file n=7 path=packages/blitz-dom/src/events/pointer.rs
what: pointer event default actions - click on a select, click on an option row, the select list default actions
@task_file n=8 path=packages/blitz-dom/src/accessibility.rs
what: the blitz-dom accessibility tree build - a select's role, expanded state and active descendant
@task_file n=9 path=packages/blitz-dom/src/layout_reason.rs
what: the blitz-dom layout reason - the placing an option of a select reads
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs
what: the dioxus-native-dom snapshot builder and its closed_select_of reader, which refuses a closed select's options as covered
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: the driver verb table's fixed help texts for the click and press verbs
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's unit tests over the verb help texts
@task_file n=13 path=docs/capabilities.md
what: the capability statement - the select model and its stated limits
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs
what: the select interaction integration checks, in both layout modes
@task_file n=15 path=tests/blitz-tests/tests/select_popup.rs
what: the select popup (open select list) integration checks
@task_file n=16 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand's acting checks on selects through the driver session - snapshot, click, actions, press, type, lint
@task_item n=1 part=touches
decision select-size-hint: the clause that a single select's size stays unread is retired; size now decides a single select's display size and makes it a list box above 1, through the same capped size_rows read.
@task_item n=2 part=touches
decision single-select-list-opening and select-list-painted-overlay: opening a list, and the list gate, now hold for a drop-down alone (display size 1); a single list box opens no list on click, Enter or Space, and a size write that makes a list box closes an open list.
@task_item n=3 part=touches
decision hit-focus-form-engine-changes and direct-engine-changes-flagged-upstreamable: the list box's row click and keys are a new direct engine change for every document, upstreamable.
@task_item n=4 part=touches
contract dioxus-native-dom closed-select-of and driver-forms closed-select-of: closed_select_of now answers None for any select is_list_box reads true, not only a multiple select.
@task_item n=5 part=touches
decision closed-select-options-covered and contract snapshot closed-select-option: a sized single select's rows are no longer refused as covered; the snapshot carries them with their own bounds, as a multiple select's rows (snapshot multiple-select-bounds, driver-snapshot row-bounds).
@task_item n=6 part=touches
contract accessibility-tree select-expanded and aria-expanded and active-descendant: expanded and the active descendant are written on a drop-down alone; a single list box reads neither, nor an author aria-expanded.
@task_item n=7 part=touches
contract node-keys role and the snapshot role: a select reads ListBox where it is a list box (multiple or display size above 1), else ComboBox; a sized single select now reads ListBox.
@task_item n=8 part=touches
decision select-keyboard-model, contract select-list-default-actions open-keys and click-select: a single list box takes arrows, Home, End and typeahead to choose with no list open; Enter and Space open nothing on it; a click on it opens no list.
@task_item n=9 part=touches
decision select-choice-one-path, select-choice-single-path and select-choice-single-write-path: the list box's keys and row clicks choose through the one write path, apply_select_choice.
@task_item n=10 part=touches
decision multiple-and-optgroup-modelled and multiple-select-row-clicks: a single list box's row click chooses that row alone whatever modifier is held, unlike a multiple select's flip or span; the multiple select's clicks and keys are unchanged.
@task_item n=11 part=touches
decision multiple-select-chosen-row-colours and select-list-colours-engine-constants: the chosen row of a single list box now paints the same engine constant pair, black on grey c8c8c8, as a multiple select's.
@task_item n=12 part=touches
contract form-controls select-rules and select-readers: the single select's settle to the first enabled option now holds at display size 1 alone; a list box may stand with nothing chosen and read no value.
@task_item n=13 part=touches
contract base-document layout-reason and answer-json layout-placing: an option of a list box (not only of a multiple select) reads its own placing; no placing word added.
@task_item n=14 part=touches
contract driver-verbs click and press: their help texts are reworded to say a sized single select's row is chosen by click and arrows, Home and End choose with nothing opening; verb, argument, field, cause and count unchanged.
@task_item n=15 part=touches
contract geometry get-client-bounding-rect: measured that a node a restyle turns display none keeps the client rect it last laid out at; the read is stale for such a node.
@task_item n=16 part=touches
decision select-ctrl-a-unmodelled: Ctrl+click and Ctrl+A remain stated limits, now reaching a single list box as well.
@task_item n=17 part=touches
decision capability-statement and capabilities-doc-beside-upstream: the capability statement now states the single list box and drops the unread-size limit; its counts are unmoved.
@task_item n=18 part=touches
decision legacy-attributes-as-pres-hints, snapshot-before-attribute-mutation and targeted-restyle-on-mutation: a multiple write now restyles a select's descendants as a size write does, since both decide the rows' hints.
@task_item n=19 part=touches
contract lint-verb clipped-off-screen and truncated: a scrolling list box's rows scrolled out of view lint as clipped, truncated and off-screen at boot, with no overlap (lint-verb overlap-select-rows unchanged).
@task_item n=20 part=touches
decision select-row-scroll-into-view and contract scroll-hit scroll-into-view: a list box's key-reached row is scrolled into view instantly, nearest, moving the box's own scroll.
@task_fact n=1 slug=established-decisions
fact: A single select whose size parses above 1 is an in-flow list box in every document: it opens no list, its arrows, Home, End and typeahead choose and scroll the row into view, a row click chooses that row alone whatever modifier, and it may stand with nothing chosen; display size 1 keeps the drop-down.
@task_fact n=2 slug=standard-contracts
fact: BaseDocument::is_list_box(select) is a public read-only reader: true for a select with multiple or a display size above 1; a stale or non-select id reads false.
@task_fact n=3 slug=standard-contracts
fact: In the accessibility tree a select reads ListBox where it is a list box, else ComboBox; a single list box carries no expanded, no active descendant, no author aria-expanded and is not multiselectable.
@task_fact n=4 slug=standard-contracts
fact: get_client_bounding_rect reads stale layout for a node a restyle turned display none: it keeps the client rect it was last laid out at.
@task_fact n=5 slug=established-decisions
fact: A size or a multiple attribute write on a select restyles its descendants, since both decide its rows' presentational hints.
@task_item n=1 part=untouched
The session socket, its wire versions, the MCP role, recording directories and kept check files: no carrier, call or file shape moved.
@task_item n=2 part=untouched
Telemetry, scrubbing, spans and log fields: no log site, span, field or environment variable was added.
@task_item n=3 part=untouched
Key names, refusal causes, session errors and exit codes: none added or removed; their counts stand.
@task_item n=4 part=untouched
Stack, dependencies, CI legs, gates, coverage floors and the toolchain: no dependency, leg, threshold or tool moved.
@task_item n=5 part=untouched
WPT runner, the browser app, browser history, networking, scripting and CSS layout features beyond the select: not reached.
@task_item n=6 part=untouched
Occupied names - crates, binaries, servers and other names: none taken or freed.
@task_item n=7 part=untouched
The UI graph, stable element ids, the diff and the actions verb's answer shape: unchanged; only a sized select's rows now appear actionable with no cause.
