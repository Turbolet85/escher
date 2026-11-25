@task_prose part=session
The session added canvas lint: the lint verb now reads the labels a canvas widget paints and names a label truncated, once per label in paint order, when its ink box is not inside its region.
A label's region is the element's content box cut by each pushed clip, layer or mask path's bounding box and by the element's clip region; a label's ink box is read from its glyph outline bounds through skrifa, which escher-driver now takes as a dependency already in the graph.
A canvas finding crosses as numbers alone, its ink box in text and its visible region in visible; no label text or glyph id leaves, and the lint answer's keys and its six defects are unchanged.
The canvas lint reading opens no span and logs nothing; no IPC method, env var, span or span field was added and none retired.
A new stand target proves it on a planted page of numbered cases and on Circle Drawer at four states, in both layout modes; the capability statement's visual-lint witnesses gain it.
The plan's placement of one planted case was disproved by measurement and moved; four further canvas checks were left out and stay unowned residuals.
@task_file n=1 path=packages/escher-driver/src/canvas_lint.rs
what: the driver's canvas read for the lint: the labels a canvas widget paints, each with its ink box and its clipping region
@task_file n=2 path=packages/escher-driver/src/canvas_lint_tests.rs
what: the unit tests of the canvas lint's clipping region and label reading
@task_file n=3 path=tests/blitz-tests/tests/stand_canvas_lint.rs
what: a headless stand test proving the lint's canvas truncated finding on a planted page and on Circle Drawer, and the lint verb's answer keys
@task_file n=4 path=packages/escher-driver/Cargo.toml
what: the escher-driver library's manifest and dependency list
@task_file n=5 path=Cargo.lock
what: the workspace lock file
@task_file n=6 path=packages/escher-driver/src/lib.rs
what: the escher-driver library's crate root, its module list and its crate doc
@task_file n=7 path=packages/escher-driver/src/lint.rs
what: the driver's lint verb: Session::lint, its findings, the six defects and their reasons, the truncated reading
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the driver's verb schema: each verb's fixed help text, served by the help command and the MCP tool listing
@task_file n=9 path=docs/capabilities.md
what: the capability statement: the visual lint and where each claim is proven
@task_file n=10 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared helpers of the stand tests: the Circle Drawer session and its canvas draw and reach helpers
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the stand test of the canvas verb on Circle Drawer
@task_item n=1 part=touches
decision/canvas-read-opens-no-span: its reach widens; the lint's canvas label reading and the new canvas lint read also open no span and log nothing, and the row must now say so.
@task_item n=2 part=touches
decision/driver-emits-no-event: a new driver module, the canvas lint read, emits no event; the rule now rules over it.
@task_item n=3 part=touches
decision/driver-library-installs-no-subscriber: indirect, through the driver library gaining a private module and a dependency; it installs no subscriber.
@task_item n=4 part=touches
contract/command-answer/linted: the lint answer's findings now include a canvas element's truncated finding once per label, with the label's ink box in text and its visible region in visible; keys and the defect set unchanged.
@task_item n=5 part=touches
decision/driver-spans-carry-no-content and decision/command-span-no-call-content: indirect; the lint command's span carries no canvas label text, ink box or glyph id.
@task_item n=6 part=touches
decision/session-logs-carry-no-page-content: a canvas label's text is page content that is now read by the lint; it is never logged and leaves only as box numbers in the answer.
@task_item n=7 part=touches
decision/debug-types-never-logged: the lint finding now carries canvas label boxes and remains an answer-bearing type never logged.
@task_item n=8 part=touches
decision/open-list-shot-carries-labels: a sibling crossing rule; canvas labels, which crossed as pixels alone, now also cross as ink-box numbers in the lint answer, never as text.
@task_item n=9 part=touches
contract/escher-session-commands/help and decision/fixed-diagnostics: the lint verb's help text and its findings field's help changed to name the canvas label and its ink box; still fixed program text.
@task_item n=10 part=touches
decision/capability-statement-updated-by-hand: the capability statement was edited by hand; the visual lint section and its witnesses gain the new canvas lint stand target.
@task_item n=11 part=touches
decision/headless-stand-no-sink: a new headless stand target runs the lint on a planted page and on Circle Drawer with no sink.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: A canvas label read by the lint crosses only as numbers, its ink box and its visible region inside a truncated finding; its text and glyph ids never leave the process and are never logged.
@task_item n=1 part=untouched
The telemetry sink, its log line format, its scrub set, its target allowlist and RUST_LOG handling: no event, field, target or level was added.
@task_item n=2 part=untouched
The command span and tool_call span contracts and their fields: the lint verb still opens one command span with the same verb and fields; no span or span field was added.
@task_item n=3 part=untouched
Refusal cause names, escher-session exit codes, the host wire and the error object: no refusal, exit code or wire method was added.
@task_item n=4 part=untouched
The cold-agent pipe and agent-run harness rows: their events, verdict fields, counted verbs and lint call counts are unchanged.
@task_item n=5 part=untouched
Metric coverage, error capture and reporting, WPT reporting and the backend absences: nothing was measured, exported or captured anew.
@task_item n=6 part=untouched
CI integration rows: no workflow, leg, artifact or upload changed; the new stand target joins the test and stand-repeat legs by the existing glob, and stand-repeat's named-target count is unchanged.
@task_item n=7 part=untouched
Recording, session state directory and kept check rows: no recording or state file changed.
@task_item n=8 part=untouched
Engine tracing feature gating and the tracing crate choice: the new dependency adds no tracing and no feature.
