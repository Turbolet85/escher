@task_prose part=session
The session added a canvas lint: the lint verb now reads truncated once per label a canvas widget paints whose ink box is not inside its region, the region being the canvas content box cut by each pushed clip, layer or mask path and by the element's clip region.
A canvas finding crosses as numbers alone: the label's ink box and its visible part inside truncated's text and visible fields; no label text, glyph id or font data crosses, and the finding keys stay the closed set; the six-defect set and the lint answer's form are unchanged.
The driver crate now takes skrifa, already in the graph through blitz-dom and blitz-paint, from the workspace version 0.49, to read each glyph's outline bounds from the canvas widget's own font bytes; the lockfile moves by one dependency edge, no package is added, and the audit leg reads advisories and licences ok.
The lint verb's help and its findings field's help, fixed program text served by help and the MCP listing, now name the canvas label and its ink box; no IPC method, endpoint, port, socket, env var or span is added, and the canvas reading opens no span and logs nothing.
A new integration test target proves the canvas lint on a planted page and on Circle Drawer at four states; shared Circle Drawer test helpers moved into the common test module; nothing was retired.
@task_file n=1 path=packages/escher-driver/src/canvas_lint.rs
what: new crate-private module of the driver that reads, from a canvas widget's paint commands, each label's ink box through the font's glyph outline bounds and its clip region
@task_file n=2 path=packages/escher-driver/src/canvas_lint_tests.rs
what: new unit tests of the canvas label reader, among them a font that cannot be read adding no label with no panic
@task_file n=3 path=tests/blitz-tests/tests/stand_canvas_lint.rs
what: new integration test target of the lint verb's canvas findings, asserting the lint line holds no label text and only keys from the closed set
@task_file n=4 path=packages/escher-driver/Cargo.toml
what: the driver crate's manifest, edited to take skrifa as a workspace-inherited dependency
@task_file n=5 path=Cargo.lock
what: the workspace lockfile, edited by one dependency edge on skrifa in the driver's list, no package added
@task_file n=6 path=packages/escher-driver/src/lib.rs
what: the driver crate root, edited to declare the private canvas lint module and to restate the lint paragraph of its crate doc
@task_file n=7 path=packages/escher-driver/src/lint.rs
what: the driver's visual lint, the reader of lint findings and the truncated defect, edited to chain canvas label truncations after the DOM reading
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the closed driver verb schema, whose lint verb help and findings help text, fixed program text served by help and the MCP listing, were edited
@task_file n=9 path=docs/capabilities.md
what: the committed capability statement, whose visual lint section and its witnesses were edited to name the canvas lint
@task_file n=10 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared integration test helpers, edited to receive the Circle Drawer canvas helpers
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the canvas verb integration test target, edited to import the moved Circle Drawer helpers
@task_item n=1 part=touches
decision/canvas-content-crossing: a canvas label's ink box now crosses as four numbers inside truncated's text and visible fields; no label text crosses, asserted by the new test.
@task_item n=2 part=touches
decision/lint-findings-closed-content: a lint finding's numbers now include a canvas label's box and its visible part; the finding still holds no page content.
@task_item n=3 part=touches
decision/lint-findings-crossing: lint findings that cross every ratified carrier now include canvas truncated findings, one per label in paint order.
@task_item n=4 part=touches
decision/mcp-listing-fixed-text: the lint verb's help and its findings help, fixed program text in the MCP listing, were reworded to name the canvas label; still fixed text.
@task_item n=5 part=touches
decision/closed-driver-verb-schema: the lint entry of the closed verb schema changed its help text only; no argument, verb or field was added.
@task_item n=6 part=touches
decision/capability-statement-mcp-crossing: the committed capability statement's visual lint section and witnesses gained the canvas lint.
@task_item n=7 part=touches
decision/workspace-inherited-dependency-versions: the driver takes skrifa with workspace = true, the workspace's 0.49.
@task_item n=8 part=touches
decision/locked-builds and decision/upstream-sync-dependency-set and decision/engine-dependency-pins: the lockfile moved by one edge, the driver now depending on skrifa already in the graph; no package added.
@task_item n=9 part=touches
decision/cargo-deny-audit-tool and decision/dependency-audit-ci-gate and decision/licence-gate: the audit leg read the new edge, advisories ok and licences ok.
@task_item n=10 part=touches
decision/untrusted-bytes-decoded-without-panic and decision/no-panic-on-call-input: a canvas widget's font bytes are now parsed by skrifa through Result and Option; a font it cannot read adds no label, with no panic.
@task_item n=11 part=touches
decision/no-screen-content-in-logs and decision/driver-outcomes-never-logged and decision/driver-installs-no-subscriber: the new canvas reading logs nothing and opens no span.
@task_item n=12 part=touches
decision/custom-widget-content-box-frame: the canvas lint takes the custom widget's content box as the start of a label's region.
@task_item n=13 part=touches
decision/blitz-tests-not-published: the unpublished test crate gained a new test target.
@task_item n=14 part=touches
decision/cold-agent-canvas-to-provider and decision/cold-agent-screen-to-provider: lint answers a cold-agent run carries may now hold canvas label boxes as numbers, no text.
@task_fact n=1 slug=dependency-security
fact: The driver crate takes skrifa, workspace-inherited at 0.49 and already in the graph through blitz-dom and blitz-paint, to read glyph outline bounds from a canvas widget's own font bytes; one lock edge, no package added; PROVISIONAL.
@task_item n=1 part=untouched
Authentication and authorization rows: no auth code, token or CI credential changed.
@task_item n=2 part=untouched
Secret management and environment variable rows: no env var was added or read.
@task_item n=3 part=untouched
API security rows on listeners, outbound HTTP, load waits and idle expiry: no endpoint, port, socket, IPC method or request path was added.
@task_item n=4 part=untouched
Session socket and MCP wire contracts, request lines, size bounds and protocol errors: the wire and its bounds are unchanged.
@task_item n=5 part=untouched
Error handling rows and refusal causes: no refusal cause, error text or exit code was added.
@task_item n=6 part=untouched
Data protection rows on history, cookies, cache, TLS and kept check files: nothing stored or kept changed.
@task_item n=7 part=untouched
Recording, replay and recorded check rows: the recorder and check file format are unchanged.
@task_item n=8 part=untouched
Select list, masked value, stable id and snapshot text rows: none of those readings or carriers changed.
@task_item n=9 part=untouched
Script fetch, URL scheme, iframe and stylesheet loading rows: no page loading path changed.
@task_item n=10 part=untouched
CI workflow, publish and signing rows: no workflow, harness leg or gate changed.
