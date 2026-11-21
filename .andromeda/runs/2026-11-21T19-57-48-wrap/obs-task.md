@task_prose part=session
A single select whose size attribute parses above 1 is now an in-flow list box in every document: it opens no list, its keys and row clicks choose a row alone, and it may stand with nothing chosen.
The engine reads a single select's size through the same capped closed parse a multiple select uses; display size 1 (absent, 0, 1, empty, unparsable, negative) keeps the drop-down.
The accessibility tree reads ListBox for a list box with no expanded state; the bridge's closed-select reading answers none for it, so its rows reach the snapshot, actions and click with their own bounds and no covered cause.
The driver's click and press help texts were re-worded; no verb, argument, key name, field, cause, session error, exit code, log site, span or field was added.
Retired: the stated limit that a single select's size is unread, from the capability statement and the size hint comment.
Measured: the list-box page lints 4 findings at boot (rows scrolled out of the box read clipped, truncated, off-screen; no overlap), and a node a restyle turns display none keeps its last client rect.
@task_file n=1 path=packages/blitz-dom/src/node/select.rs
what: the engine's select model: display size, list box and drop-down readings, the chosen option's settling and selectedness
@task_file n=2 path=packages/blitz-dom/src/node/select_list.rs
what: the select's open list (the painted popup) and the gate on which selects may open one
@task_file n=3 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: the presentational hints, the select size attribute read and the list box rows' chosen-row colours
@task_file n=4 path=packages/blitz-dom/src/mutator.rs
what: the document mutator: attribute writes (size, multiple) that restyle a select and its rows and close an open list
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: the engine document, its public read of whether a select is a list box
@task_file n=6 path=packages/blitz-dom/src/events/keyboard.rs
what: the engine's key dispatch: select keys, typeahead and the list box's arrow, Home and End choosing
@task_file n=7 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's click handling: select clicks, option row clicks, list opening
@task_file n=8 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree: a select's role (ListBox or ComboBox) and its expanded state
@task_file n=9 path=packages/blitz-dom/src/layout_reason.rs
what: the layout read's placing reasons for a select's options
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot builder and the closed-select reading that hides a closed select's rows from the snapshot and the lint
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: the driver's verb schema and fixed help texts of the click and press verbs
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's unit checks of the verb help texts
@task_file n=13 path=docs/capabilities.md
what: the capability statement, its select model and stated limits
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs
what: the select interaction checks: keys, clicks, size writes and chosen state, in both layout modes
@task_file n=15 path=tests/blitz-tests/tests/select_popup.rs
what: the select open-list (popup) checks, among them that a sized single select opens no list
@task_file n=16 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand's driver checks of selects: snapshot role and row bounds, click, actions, press, type and lint at boot
@task_item n=1 part=touches
decision/select-size-attribute-read: the size attribute is now read on a single select too, in every document; still no log site, span or field, the read carries no text out.
@task_item n=2 part=touches
Any row stating a single select's size is unread or not read is now stale; the limit is retired from the capability statement.
@task_item n=3 part=touches
decision/select-chosen-set-never-logged: a single list box's chosen row is a new chosen state, written by its keys and row clicks; it crosses only in the snapshot and its diff, never logged.
@task_item n=4 part=touches
decision/select-list-label-never-logged and decision/open-list-shot-carries-labels: a single list box opens no list, so its row labels cross as snapshot rows, not as an open list's painted rows.
@task_item n=5 part=touches
contract/snapshot-diff/open-list-row-bounds and contract/snapshot-node/clickable: a single list box's rows now stand in the snapshot with their own bounds and as clickable, with no list open.
@task_item n=6 part=touches
contract/refusal-cause/covered: a click on a list box row off the select's centre no longer reads covered, since the closed-select reading answers none for a list box.
@task_item n=7 part=touches
contract/actions-answer/cause and contract/actions-answer/content: actions lists a sized single select's rows in view with no cause.
@task_item n=8 part=touches
contract/command-answer/linted: a list box's rows scrolled out at boot lint as clipped, truncated and off-screen findings (4 on the stand's list-box page), no overlap.
@task_item n=9 part=touches
contract/cold-agent-verdict/list-opens: a click or key on a single list box opens no list, so no list open is counted for it.
@task_item n=10 part=touches
contract/page-events/single-change: a list box's key or row click is one write path through the select choice, firing the same change as a drop-down's choice.
@task_item n=11 part=touches
contract/snapshot/multiple-select-value: a single list box may stand with nothing chosen and reads no value, beside the multiple select's value.
@task_item n=12 part=touches
contract/escher-session-commands/help and contract/tool-call-span/help: the click and press help texts are re-worded fixed program text; verb, argument, field, cause and count unchanged.
@task_item n=13 part=touches
decision/escher-session-fixed-diagnostics and decision/fixed-diagnostics: the re-worded help remains fixed strings carrying no page content.
@task_item n=14 part=touches
decision/layout-read-opens-no-span: the layout read's select reason now keys on the list-box reading; it still opens no span.
@task_item n=15 part=touches
decision/engine-sites-fixed-messages and decision/driver-emits-no-event: the engine and driver changes add no log site, event or span.
@task_item n=16 part=touches
decision/capability-statement-updated-by-hand: the capability statement was edited by hand to state the list box and drop the size limit, its counts unmoved.
@task_item n=17 part=touches
decision/actions-read-changes-nothing: actions on a list box page reads rows and writes nothing.
@task_item n=18 part=touches
decision/session-logs-carry-no-page-content: the list box's row labels and chosen state reach no session log.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: A single select of display size above 1 is a list box that opens no list; its chosen row and row labels cross only in the snapshot, the diff and the actions answer, never in a log, span or field.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: A single list box may stand with nothing chosen; the snapshot then reads no value for it, and a drop-down left with nothing chosen settles to its first enabled option.
@task_item n=1 part=untouched
Telemetry strategy and the tracing crate's feature gating: no crate, feature or dependency changed.
@task_item n=2 part=untouched
The telemetry sink's log line format, resource keys, RUST_LOG handling, target allowlist and scrub set: no log field, span or scrub shape added.
@task_item n=3 part=untouched
The command and tool_call spans' fields, endings and line layout: no span or field added, no verb added.
@task_item n=4 part=untouched
Refusal causes other than covered, session error objects, exit codes and the host wire: none added or moved.
@task_item n=5 part=untouched
The cold-agent pipe and agent-run harness events, verdict fields, transcripts and counted verbs other than list opens: no harness change.
@task_item n=6 part=untouched
Metric coverage, error capture, script error surfaces, the panic hook and WPT reporting: not reached.
@task_item n=7 part=untouched
CI integration, artifacts, recording files and directories, session state and the build-directory sweep: no harness or gate surface changed.
@task_item n=8 part=untouched
The multiple select's keys, row clicks, form entries and value: unchanged.
