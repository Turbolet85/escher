@task_prose part=chunk
A single select (no multiple) whose size reads a valid value above 1 is shown as HTML renders it: an in-flow list box size rows tall, its rows the options and group headers, the chosen row marked, the rest scrolling.
Keys move the choice inside the box, a click on a row chooses it, and no engine-painted popup list opens for it; a size of 1, 0 or an invalid value leaves it a drop-down box.
The founder's earlier ruling that a single select's size stays unread falls; the multiple select's size reading (valid size that many rows, no size all rows, no default of 4) is re-asserted as it stands.
Snapshot, diff, accessibility, lint, actions, layout and screenshot readings of the sized single select are re-measured against a multiple select's list box and against HTML.
@task_file n=1 path=packages/blitz-dom/src/node/select_list.rs mark=edits
what: blitz-dom form control readers, the select list state (size rows, list rows, chosen-row model) — edits: size_rows and its multiple-only use
@task_file n=2 path=packages/blitz-dom/src/node/select.rs mark=edits
what: blitz-dom form control readers, the select rules (single versus multiple select, select choice writers)
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=edits
what: BaseDocument open select list and select readers, the gate that allows a select's popup list to open — edits: select_list_allowed
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: DOM event dispatch of keys, the select keyboard model for single and multiple selects — edits: press_select_key, press_multiple_select_key
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: DOM event dispatch of pointer events, select list default actions on click and pointerdown, select row clicks — edits: click_select_row
@task_file n=6 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: legacy attributes cascaded at the presentational-hints level, the select size height hint — edits: the size height hint of size times 1.2em
@task_file n=7 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: blitz-dom accessibility tree, a select's role (list box versus combo box) and its expanded and selected states
@task_file n=8 path=packages/blitz-dom/assets/default.css mark=edits
what: the user-agent stylesheet, the select element's rendering and a list box's look
@task_file n=9 path=packages/blitz-paint/src/render.rs
what: blitz-paint paint_scene, the painted face of a select form control
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot reader, select options in the snapshot, the expanded and highlighted node keys of a select
@task_file n=11 path=packages/escher-driver/src/execute.rs
what: escher-driver verb execution, driver type on a select and the per-verb target check
@task_file n=12 path=packages/escher-driver/src/lint.rs
what: the lint verb, overlap of select options and list rows
@task_file n=13 path=packages/escher-driver/src/actions.rs
what: the actions verb's answer, actionable select rows and their target verbs
@task_file n=14 path=packages/escher-driver/src/json/layout.rs
what: answer JSON layout, the layout placing words of a select's rows
@task_file n=15 path=tests/blitz-tests/tests/select_interaction.rs mark=re-reads
what: the select interaction checks, both layout modes
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs mark=re-reads
what: the select popup list checks, both layout modes
@task_file n=17 path=tests/blitz-tests/tests/stand_act_select.rs mark=re-reads
what: the stand acting-verb checks on a select
@task_file n=18 path=tests/blitz-tests/tests/stand_act_popup.rs mark=re-reads
what: the stand acting-verb checks on an open select popup list
@task_item n=1 part=needs
HTML's rendering rule for a select: multiple or a valid size above 1 renders a list box, otherwise a drop-down box; size 1, 0 or an invalid value keep a single select a drop-down box.
@task_item n=2 part=needs
The select size reading (parsed as an unsigned integer, capped at 65534) and its height hint of size rows times 1.2em now apply to a sized single select as well as a multiple select.
@task_item n=3 part=needs
Opening a single select's list: a sized single select opens no popup list on click, label click, Enter or Space; the open select list machinery and its gate never engage for it.
@task_item n=4 part=needs
The select keyboard model inside a sized single select's list box: arrow keys, Home and End move the single choice, with typeahead and Tab as for a single select, and no key opens a list.
@task_item n=5 part=needs
Row clicks on a sized single select's list box choose that one row; disabled options and optgroup headers are not chosen, as for multiple select row clicks.
@task_item n=6 part=needs
Every choice of a sized single select goes through the one select choice write path held as options' checked state, with input and change events as a single select fires them and nothing when the chosen option is re-chosen.
@task_item n=7 part=needs
The list box's rows scroll: the chosen row is kept in view, and scrolling a select row into view moves only the list box, never the page.
@task_item n=8 part=needs
The founder's F8 falls in part: a single select's size is no longer a stated limit; the multiple select reading stands (valid size that many rows, no size all rows, HTML's default of 4 not adopted).
@task_item n=9 part=needs
Accessibility tree of a sized single select: list box role rather than a combo box, no single select expanded state, selected state on its option rows, active descendant as a multiple select's list box carries it.
@task_item n=10 part=needs
Snapshot readings of a sized single select: its rows are real boxes like a multiple select's rows (row bounds, selected, highlighted), it carries no expanded key, and its options are not refused as a closed select's covered options.
@task_item n=11 part=needs
Snapshot diff, actions answer, lint overlap of select rows, layout placing words and screenshots of a sized single select follow from the engine as for a multiple select's list box; a reading that would differ from a multiple list box's or from HTML's is a founder fork.
@task_item n=12 part=needs
Driver type on a select and click on a row of a sized single select reach the list box rows as for a multiple select, through the per-verb target check.
@task_item n=13 part=needs
Painted face and look of a sized single select: drawn as a list box like a multiple select (chosen-row colours from the engine constants), with no closed box face and no popup painted in the scene.
@task_item n=14 part=needs
Select model and select list as direct engine changes: this is an engine change for every document, remote pages included, that changes the security plan's stated select limits and rides its ratification, flagged upstreamable.
@task_item n=15 part=needs
The change crosses no new carrier, verb, argument, key name, result field, refusal cause, session error, exit status, log field or span unless the founder rules it.
@task_item n=16 part=needs
The select model's stated limits, wherever the plan or the capability statement states a single select's size as unread, are withdrawn; residual res-ysz67imk is closed.
@task_item n=17 part=needs
The existing select interaction, popup and stand acting checks keep passing in both layout modes, and new checks of the sized single select stand beside them.
@task_standing n=1
convention: every new scenario runs in both layout modes and both give identical layouts
home: conventions
@task_standing n=2
convention: one behaviour per integration test file, one test target per file in blitz-tests, declared in its one test table
home: conventions
@task_standing n=3
convention: tests assert the fixture's own condition first, with numbered fixture cases and relative fixture URLs
home: conventions
@task_standing n=4
convention: descriptive test names, and select tests name the element they act on
home: conventions · standard-contracts
@task_standing n=5
convention: spec URLs cited at implementations, behaving like a browser
home: conventions · design-philosophy
@task_standing n=6
convention: unit tests at the end of the source file, track-caller assertion helpers, justified unwrap and expect
home: conventions
@task_standing n=7
convention: direct engine changes flagged upstreamable
home: established-decisions
@task_item n=1 part=not_needed
The WPT runner, its suites, reftests, result kinds, statuses and PR body reporting: the chunk adds no WPT test and changes no runner.
@task_item n=2 part=not_needed
The browser application: chrome, tabs, about pages, history store and its migrations, persistence and bookmarks.
@task_item n=3 part=not_needed
The JavaScript runtime and script globals: Boa, microtasks, script fetching and the script document API.
@task_item n=4 part=not_needed
Networking and resources: the net provider, per-host request caps, HTTP cache, image and favicon caches, URL scheme routing.
@task_item n=5 part=not_needed
The cold-agent tooling and its verdict fields, the agent-run script, the target sweep script and their exit codes and events.
@task_item n=6 part=not_needed
Kept check files and session recording: their format, digest, masking, recording directory and replay.
@task_item n=7 part=not_needed
The UI graph and the graph verb, signal registration and write resolution.
@task_item n=8 part=not_needed
Stable element id tiers and their persistence: a sized single select keeps the ids its options have today.
@task_item n=9 part=not_needed
The session socket wire, the MCP server protocol, the session host process model and session state directories.
@task_item n=10 part=not_needed
Text and font layout: Parley, inline layout, font fallback, text decoration and writing modes.
@task_item n=11 part=not_needed
Tables as grid, flex and grid alignment, anchor positioning, container queries and other CSS absences.
@task_item n=12 part=not_needed
Canvas work, custom widget embedding, and the wgpu, Vello and window renderer choices.
@task_item n=13 part=not_needed
Dependency and licence audit, toolchain and MSRV policy, package versioning, publishing and the Nix flake.
