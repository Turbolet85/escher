@task_prose part=chunk
A single select (no multiple) whose size reads a valid value above 1 renders as HTML renders it: an in-flow list box size rows tall, its rows the options and group headers, the chosen row marked, the rest scrolling.
Keys move the choice in that box, a click on a row chooses it, and no engine-painted popup list opens for it; size 1, 0 or an invalid value leave a single select a closed box that opens the popup as today.
The founder's earlier ruling that size is read on a multiple select alone is reversed in part: the multiple reading stands as built (a valid size sets its rows, no size shows all rows, no default of 4).
Engine change reaching every document, remote pages included; snapshot, diff, accessibility, lint, actions, layout and screenshot readings of the sized single select follow from the engine like a multiple select's list box.
Re-verifies at take-up the residual that a single select's size stays unread, which this chunk closes.
@task_file n=1 path=packages/blitz-dom/src/node/select_list.rs mark=edits
what: the select list component: the select's size rows read through a typed unsigned parse and capped, and which select shows as a list — edits: size_rows
@task_file n=2 path=packages/blitz-dom/src/node/select.rs mark=edits
what: the select control's state: its options, chosen option and single-choice selectedness
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=edits
what: the document's select list open gate, which select opens the popup list — edits: select_list_allowed
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: the select key models, single select keys and multiple select keys under a closed key match — edits: press_select_key, press_multiple_select_key
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the pointer model of a select, a click on a select row choosing it — edits: click_select_row
@task_file n=6 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: presentational hints read by a typed parse, the select size hint giving a height of size rows, capped — edits: the select size hint
@task_file n=7 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the accessibility adapter tree, a select's role as list box or combo box and its expanded state
@task_file n=8 path=packages/blitz-dom/assets/default.css mark=edits
what: the engine's user-agent stylesheet, a list box's look
@task_file n=9 path=packages/blitz-paint/src/render.rs mark=edits
what: the painter of a select's face, closed box or list box
@task_file n=10 path=packages/escher-driver/src/json/snapshot.rs
what: the snapshot reader of a select's state readings, expanded and highlighted
@task_file n=11 path=packages/escher-driver/src/lint.rs
what: the lint findings over controls
@task_file n=12 path=packages/escher-driver/src/actions.rs
what: the actions answer, the verbs a select offers
@task_file n=13 path=packages/escher-driver/src/layout.rs
what: the layout answer and its placing words for a select's rows
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: the select interaction checks, keys and clicks on single and multiple selects, both layout modes
@task_file n=15 path=tests/blitz-tests/tests/stand_act_select.rs mark=edits
what: the stand checks of driver acts on a select, type on a select and its readings
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs mark=re-reads
what: the select popup list checks, the open gate and the list's behaviour for an unsized single select
@task_file n=17 path=tests/blitz-tests/tests/stand_act_popup.rs mark=re-reads
what: the stand checks of driver acts on an open select list
@task_item n=1 part=needs
The select size reading: size read on a single select as well as a multiple one, through the same typed unsigned parse and the same cap, a value above 1 making it a list box; 0, 1 or an invalid value leave it a closed box; hostile size values never panic.
@task_item n=2 part=needs
The size presentational hint: the height of size rows now also applies to a sized single select, still read by a typed parse and still capped.
@task_item n=3 part=needs
The select list open gate: which select opens the engine-painted popup list changes, a sized single select opening none; an unsized single select keeps its popup, its closing rules, twenty-row limit, wheel and pointer behaviour unchanged.
@task_item n=4 part=needs
The single select key model in a list box: keys move the choice under the closed key match, typeahead kept private and short-lived, no popup opened by any key.
@task_item n=5 part=needs
A click on a row of a sized single select chooses that row; option-disabled and optgroup rules hold for its rows as for a multiple select's.
@task_item n=6 part=needs
Every choice of a sized single select goes through the one select write path, keeps single-choice selectedness, and leaves only by the ratified exits of a select's choice, with change events as today.
@task_item n=7 part=needs
Select expansion read from engine state only: a sized single select reads as a list box, not expanded and not openable; its rows' highlighted reading matches a multiple list box's.
@task_item n=8 part=needs
Accessibility role of a sized single select: list box rather than combo box, its rows built nodes, its accessible names crossing only by ratified exits and kept out of logs.
@task_item n=9 part=needs
Driver readings of a sized single select (snapshot state words, diff keys, actions, lint, layout placing words, screenshot pixels) follow from the engine with no new key, state word, field or value; a reading that would differ from a multiple list box's or from HTML's is a founder fork.
@task_item n=10 part=needs
Driver type on a select reaches a sized single select through the same target check and choice path; unknown or unfit targets refused as today.
@task_item n=11 part=needs
The change crosses no new carrier, verb, argument, key name, result field, refusal cause, session error, exit code, log field or span unless the founder rules it; any such would fall under the boundary widening rule.
@task_item n=12 part=needs
The security plan's stated limit on selects, that a single select's size is not read, falls; the select rules ride the founder's ratification of this change, including the partial reversal of the multiple-only size ruling.
@task_item n=13 part=needs
Row labels of a sized single select's list box never reach a log, and no screen content enters a log line the chunk adds.
@task_item n=14 part=needs
A list box under hostile markup stays finite: row heights and scroll offsets finite, rows scrolled into view by their own offset, never-styled rows read as none.
@task_item n=15 part=needs
Paint of a sized single select uses the engine's own colours and paints its rows in flow, not over the page.
@task_standing n=1
convention: every new log line or span field holds no screen content, label or caller input; only closed names and counts
home: logging-monitoring
@task_standing n=2
convention: new engine and driver code stays above its crate's line coverage floor
home: input-validation
@task_standing n=3
convention: a new test file under the test crate takes no tracing and no style dependency and is never published
home: dependency-security
@task_item n=1 part=not_needed
Authentication and authorization, CI tokens and publish secrets: the chunk adds no access control.
@task_item n=2 part=not_needed
Secret management and environment variables: the chunk reads no environment and holds no secret.
@task_item n=3 part=not_needed
Outbound fetches, script fetchers, CORS and same-origin, file URLs, load waits and per-host limits: a select's rendering fetches nothing.
@task_item n=4 part=not_needed
Stored data: history database, cookies, HTTP cache, TLS, clipboard and file dialog.
@task_item n=5 part=not_needed
Recording, kept check files, recording directories, state directories and session cleanup: the chunk changes no recorded or kept file.
@task_item n=6 part=not_needed
Session socket and MCP transport framing, lifecycle, request line bounds, idle expiry and protocol errors.
@task_item n=7 part=not_needed
Cold-agent, agent-run, install and build-sweep scripts and their argument checks.
@task_item n=8 part=not_needed
Range input, password and file masking, cells grid and custom widget rules: other controls than the select.
@task_item n=9 part=not_needed
JavaScript opt-in, CSSOM script API, bootstrap scripts, Dioxus bridge and WPT runner rules.
@task_item n=10 part=not_needed
CDP devtools server, iframe navigation and nesting limits, external navigation schemes.
