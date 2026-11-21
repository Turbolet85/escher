@task_prose part=session
The engine now reads the size attribute of a single select in every document: a single select whose size parses above 1 is an in-flow list box of its rows, opening no popup list, and display size 1 (size 1, 0, negative, non-numeric, empty or absent) keeps the drop-down.
A single list box chooses by its own keys (arrows, Home, End, typeahead) and by a click on a row, both through the one select write path; Enter, Space and other keys do nothing, and it may stand with nothing chosen.
The accessibility tree reads ListBox for any list box (multiple or sized), ComboBox otherwise; expanded and the active descendant are written on a drop-down alone, and a list box reads no author aria-expanded.
The bridge's closed-select reader answers none for a list box, so its rows are snapshotted, acted on and linted like page content; the driver's click and press help texts are re-worded with no verb, argument, key name, field, cause, exit, log field or span added.
Retired: the stated limit that a single select's size is unread. Measured: a sized list box with rows scrolled out lints its hidden rows as clipped, truncated or off-screen at boot, and a node a restyle turns display none keeps its last client rect.
@task_file n=1 path=packages/blitz-dom/src/node/select.rs
what: edited — the engine's select model: selectedness settling, the display size read, the list-box and drop-down readings, typeahead and key stepping
@task_file n=2 path=packages/blitz-dom/src/node/select_list.rs
what: edited — the open select list and its open gate (which select opens a list)
@task_file n=3 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: edited — the presentational hint attributes read by a typed parse, including the capped select size hint and the list box row and chosen-row colours
@task_file n=4 path=packages/blitz-dom/src/mutator.rs
what: edited — the document mutator: attribute writes on a select (size, multiple) restyling its rows and closing an open list
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: edited — the engine document: the public read-only list-box reading that resolves the node and reads false for a stale id, and selectedness settling on a select
@task_file n=6 path=packages/blitz-dom/src/events/keyboard.rs
what: edited — the engine's key handling: the closed match of select keys, now with a single list box's arrows, Home, End and typeahead
@task_file n=7 path=packages/blitz-dom/src/events/pointer.rs
what: edited — the engine's pointer handling: a click on a select and on a select row, the list opening on a drop-down alone and a list box row chosen alone
@task_file n=8 path=packages/blitz-dom/src/accessibility.rs
what: edited — the accessibility adapter tree: a select's role (ListBox or ComboBox), its expanded state and active descendant from engine state
@task_file n=9 path=packages/blitz-dom/src/layout_reason.rs
what: edited — the layout reason reader: an option of a list box reads its own placing
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the Dioxus native bridge's snapshot builder and the closed-select reader that skips a closed select's rows, now answering none for a list box
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: edited — the closed driver verb schema and its fixed help texts (click and press re-worded)
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: edited — the driver verb schema's unit checks
@task_file n=13 path=docs/capabilities.md
what: edited — the committed capability statement, its select model limits (the single-select size limit retired)
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs
what: edited — the select interaction checks (keys, clicks, selectedness, display size), both layout modes
@task_file n=15 path=tests/blitz-tests/tests/select_popup.rs
what: edited — the select popup list checks, including that a sized single select opens no list
@task_file n=16 path=tests/blitz-tests/tests/stand_act_select.rs
what: edited — the stand checks of driver verbs on selects (snapshot, click, actions, press, type, lint at boot)
@task_item n=1 part=touches
decision/single-select-selectedness: the fallback to the first enabled option now holds only at display size 1; a single list box may stand with nothing chosen, and a drop-down a size write leaves with nothing chosen settles to its first enabled option.
@task_item n=2 part=touches
Any row stating a single select's size is unread or not read (beside decision/multiple-select-size-rows) is now stale: size sets the rows of a single select too, making it a list box.
@task_item n=3 part=touches
decision/multiple-select-size-rows: the size-sets-rows rule now reaches every list box, multiple or single, through the same row hints.
@task_item n=4 part=touches
decision/select-size-hint-cap: the hint arm now reads the list-box reading through the same capped size parse (cap 65534 unchanged); a row holding the cap or the multiple-only scope of the hint is touched.
@task_item n=5 part=touches
decision/select-list-open-gate: the gate now reads the drop-down reading (a select neither multiple nor of display size above 1) in place of the single-select reading; a sized single select opens no list on click, Enter or Space.
@task_item n=6 part=touches
decision/select-keys-closed-match: a new closed key match for a focused single list box (arrows, Home, End step; a character with no Ctrl, Alt or Meta is typeahead; Enter and other keys do nothing); the drop-down key path is now reached by a drop-down alone.
@task_item n=7 part=touches
decision/select-choice-one-write-path: a list box's keys and row clicks choose through the one select write path; a row brought into view by an instant nearest scroll.
@task_item n=8 part=touches
decision/select-typeahead-private: typeahead now also runs on a single list box, and a row click clears it; a Space joins typeahead only while typed text stands.
@task_item n=9 part=touches
decision/multiple-select-keys-and-clicks: unchanged for a multiple select, but a single list box's row click chooses that row alone whatever modifier is held; any Ctrl+click or Ctrl+A stated limit gains a list-box reading.
@task_item n=10 part=touches
decision/select-readers-resolve-node-closed-select-none: the closed-select reader answers none for any select the list-box reading holds, so a list box's rows are snapshotted and acted on as page content.
@task_item n=11 part=touches
decision/select-expanded-from-engine-state and decision/merged-aria-states-closed-parse: expanded and the active descendant are written on a drop-down alone; a single list box reads neither, nor an author aria-expanded.
@task_item n=12 part=touches
decision/accessibility-adapter-content-classes and decision/accessibility-tree-refresh-on-change: a select's role now follows its display size (ListBox or ComboBox) and a size write moves it between the two.
@task_item n=13 part=touches
decision/select-choice-exits and decision/select-list-crossing: a list box's choice leaves by the snapshot's value and its rows' selected state and row bounds, with no popup list to cross; no new carrier.
@task_item n=14 part=touches
decision/closed-driver-verb-schema, decision/mcp-listing-fixed-text and contract/mcp-server/help: the click and press help texts are re-worded fixed program text; verbs, arguments, fields, causes and counts unmoved.
@task_item n=15 part=touches
decision/driver-type-on-select: type on a single list box chooses by the text's start, with no list open.
@task_item n=16 part=touches
decision/unknown-target-refused and contract/refusal-causes/covered: a click on a list box row off its centre is accepted and lists no covered cause in the actions answer; the cause set is unchanged.
@task_item n=17 part=touches
decision/lint-findings-closed-content and decision/lint-findings-crossing: a sized list box with rows scrolled out lints those rows as clipped, truncated or off-screen at boot; findings stay closed content.
@task_item n=18 part=touches
decision/presentational-hints-typed-parse: the list box's row and chosen-row hints are built from fixed program CSS, gated on the typed size parse.
@task_item n=19 part=touches
decision/select-list-engine-colours: the engine's own chosen pair, black on #c8c8c8, now also paints a single list box's chosen row.
@task_item n=20 part=touches
decision/stale-node-ids-never-panic and decision/id-lookup-anew-each-call: the new public list-box read resolves its node and reads false for a stale or non-select id.
@task_item n=21 part=touches
decision/option-disabled-one-rule and decision/disabled-read-by-presence: a disabled list box or disabled row changes nothing on a key or a click.
@task_item n=22 part=touches
decision/select-list-closes-when-select-unfit: a size write that turns a drop-down into a list box closes its open list.
@task_item n=23 part=touches
decision/snapshot-state-readings and contract/diff-node/selected: a list box click's diff names the value and the rows' selected state.
@task_fact n=1 slug=input-validation
fact: A single select whose size parses above 1 is an in-flow list box of its rows in every document, opening no popup list and readable with nothing chosen; display size 1 (size 1, 0, negative, non-numeric, empty or absent) keeps the drop-down.
@task_fact n=2 slug=input-validation
fact: A single list box takes a closed key set (arrows, Home, End, plain-character typeahead) and a row click chooses that row alone whatever modifier is held; Enter, Space without typed text and every other key do nothing, and it offers no multi-choice by Ctrl+click or Ctrl+A.
@task_item n=1 part=untouched
Authentication and authorization, CI tokens and publish guards: no session, token or permission change.
@task_item n=2 part=untouched
Data protection: cookies, HTTP cache, history store, TLS and kept check files are not reached.
@task_item n=3 part=untouched
Dependency security: no dependency, lockfile, audit or licence change.
@task_item n=4 part=untouched
Secret management and environment variables: no env var, secret or credential read added.
@task_item n=5 part=untouched
API security: no listener, socket, port, outbound request, load wait or idle-expiry change; the session socket and MCP stdio transport and their bounds are unmoved.
@task_item n=6 part=untouched
Logging and monitoring: no log site, span, span field, scrub shape or sink target added.
@task_item n=7 part=untouched
Session socket and MCP protocol contracts, CLI flags, roles and exit codes, session errors and refusal JSON fields: unchanged.
@task_item n=8 part=untouched
Recording, replay, cold-agent and coverage rows: no recorded-check, kept-check or coverage format change.
@task_item n=9 part=untouched
Masked control values, password painting and the stable element id: not reached.
@task_item n=10 part=untouched
Script fetch, URL schemes, iframe and import nesting, and HTML parsing rows: not reached.
