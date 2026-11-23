@task_prose part=chunk
The Circle Drawer sample moves its drawing area onto a real canvas: an object element carrying a custom widget painted from Rust, the circles painted from the app's own circle state, each circle's diameter painted as number text at its centre from the document's font context, no longer one DOM node per circle.
The circle list reaches the widget as one data-circles attribute, parsed by the widget when the attribute changes; malformed text reads as no circle, never a panic; the canvas states itself as a group named Drawing area.
One engine change: a pointer press and release on a custom-widget target still makes a click and a double click after the widget hears the event, so an app's click and double-click handlers on a canvas fire, for every document behind the custom-widget feature.
Circle Drawer gets an in-process stand boot read by its checks, not served by the session host; checks in both layout modes cover its flow, its shot, its boot lint (0 findings) and its keyed actionables; its canvas element shot joins the committed cross-OS digest table.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer sample of the stand — edits: its drawing area as a canvas object with a painted custom widget, the data-circles attribute it writes, the canvas's role and name, its click and double-click handlers
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's headless boot and its lean task list — edits: an in-process boot of Circle Drawer (no lean task added)
@task_file n=3 path=examples/seven_guis/Cargo.toml mark=edits
what: the sample crate's manifest, its dependencies at workspace-inherited versions — edits: the widget's crates
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: the engine's event dispatch, where a pointer event on a custom-widget target is handed to the widget — edits: the custom-widget target's dispatch so the event goes on to click synthesis
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's pointer handling and its click and double-click synthesis on pointer release — edits: the click and double-click synthesis for a custom-widget target
@task_file n=6 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's cross-OS canvas shot check and its committed digest table — edits: a digest row for Circle Drawer's canvas element shot
@task_file n=7 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the stand checks of tasks the session host does not serve, read through driver calls — edits: the Circle Drawer checks
@task_file n=8 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared test tables and fixtures — edits: Circle Drawer's rows (no served or lean list grows)
@task_file n=9 path=docs/capabilities.md mark=edits
what: the committed capability statement — edits: the canvas sample and its stated limit (a driver click lands at the canvas's centre only)
@task_item n=1 part=needs
The engine's click synthesis on a custom-widget target: after the widget hears a pointer press and release, the engine still queues the click and double-click events, so app handlers fire; the widget still hears every event; the change sits in a function of the fork's own and reaches every document behind the custom-widget feature, checked by a new engine check in both layout modes.
@task_item n=2 part=needs
The synthesised click and double click on a canvas follow the engine's existing click rules: a cancelled pointer release still clicks, the event's trusted flag is set by its dispatch source, and the focus the canvas object takes on a click stays as it is.
@task_item n=3 part=needs
Canvas content crosses only as screenshot pixels: the circles and their painted diameter text leave the snapshot, the accessibility tree, the UI graph and the lint, and no carrier gains a field for them.
@task_item n=4 part=needs
The data-circles attribute is markup attribute text read by the widget's attribute-changed reader: malformed or hostile text reads as no circle, with no panic and no log line; its text leaves only by the ratified carriers and never in a driver reply.
@task_item n=5 part=needs
The widget's attribute replay and paint path as earlier canvas chunks built them: the initial attribute is replayed to the widget, an attribute write is a tracked mutation that reaches the widget, paint and pointer frame sit in the element's content box.
@task_item n=6 part=needs
Canvas text drawn from the document's own font context handed to the widget, from the bundled fonts; no font source taken from page or agent input.
@task_item n=7 part=needs
The canvas's accessible role group and its name Drawing area: the name exits only by the ratified carriers of accessible names and is kept out of logs.
@task_item n=8 part=needs
Every element an agent can act on in Circle Drawer is keyed, so each carries a stable element id from its author key; the unkeyed-actionable reading is none on its screens under the one actionable rule.
@task_item n=9 part=needs
Boot lint on Circle Drawer reads 0 findings in both layout modes; lint findings hold no page content.
@task_item n=10 part=needs
A driver click on the canvas lands at the element's centre through the shared target check; two clicks there with no advance read as a double click; no offset argument, no new verb, carrier, served task or field joins the closed driver verb schema.
@task_item n=11 part=needs
The resize dialog's diameter slider is a range input: its pointer and key actions, its sanitised finite on-grid value and where its current value exits stand as they are and are reached by the flow checks.
@task_item n=12 part=needs
Circle Drawer is booted in process by the stand only: the session host's served task list, its socket grammar and its replies do not move.
@task_item n=13 part=needs
Circle Drawer's canvas element shot joins the committed cross-OS digest table; its whole screen holds page text and stays outside every cross-OS byte check; shots are PNG-encoded and carried in band.
@task_item n=14 part=needs
The sample's manifest takes the widget's crates at workspace-inherited versions, no git dependency, under locked builds; Dioxus stays at 0.7.3; the harness keeps the paint crate's custom-widget feature on.
@task_item n=15 part=needs
The capability statement names the canvas sample and its stated limit, with no served-task count moving.
@task_standing n=1
convention: each new test or test file in the blitz-tests harness takes the harness's dependency set — no tracing, no style crate added
home: dependency-security
@task_standing n=2
convention: new engine code is held by the per-crate line coverage floors
home: input-validation
@task_standing n=3
convention: a new check stays a committed, in-process stand check; committed checks and digest tables hold no page content beyond what the plan ratifies
home: data-protection
@task_standing n=4
convention: no new log line carries screen content, attribute text, accessible names or stable ids; escher targets scrub content-named fields
home: logging-monitoring
@task_item n=1 part=not_needed
Authentication and authorization rows on CI tokens, publish job guards and publish secrets: the chunk adds no CI job, release or publish step.
@task_item n=2 part=not_needed
Secret management and environment-variable rows: the chunk reads no environment variable, secret or credential.
@task_item n=3 part=not_needed
Outbound request rows — CORS and same-origin, request timeouts, response size limits, per-host limits, script fetcher schemes, sub-resource GETs, file-URL rules: the canvas and the sample fetch nothing.
@task_item n=4 part=not_needed
Open select list rows — its keys, wheel, rows, typeahead, colours and closing: the sample has no select element.
@task_item n=5 part=not_needed
Recording and kept-check rows — recorded check files, recording directory, kept check files and the coverage role: the chunk records nothing and adds no recording field.
@task_item n=6 part=not_needed
Cold-agent run rows — its prompt, allowlist, replay isolation, transcripts and recordings: Circle Drawer is not served to a cold agent in this chunk.
@task_item n=7 part=not_needed
Password and file input masking rows: the sample holds no password or file input.
@task_item n=8 part=not_needed
Browser app rows — history in sqlite, cookies, HTTP cache, TLS, clipboard, native file dialog, external navigation, error pages, CDP devtools server: the chunk touches no browser shell.
@task_item n=9 part=not_needed
Script and page-parsing rows — JS opt-in, JS fetch, CSSOM script API, JS module loading, URLSearchParams bootstrap, iframe and import nesting limits, HTML parsed without scripting: the sample runs no script and loads no remote content.
@task_item n=10 part=not_needed
WPT runner and WPT post-results workflow rows: the chunk adds no WPT test or quarantine entry.
@task_item n=11 part=not_needed
MCP protocol rows — initialize revision, protocol error codes, request and result line bounds, tool result shape: no MCP tool, argument or result changes.
