@task_prose part=session
The session merged upstream DioxusLabs/blitz main at 29a439e05 into the workspace as a two-parent merge of six upstream commits, with one import conflict resolved and one escher caller adapted; the next sync's merge base moves from 165d698c3 to 29a439e05.
It added OS drag-and-drop input to the engine: a DataTransfer model, drag UI events and DOM drag events (dragenter, dragover, dragleave, drop), a winit drop source in blitz-shell, drag dispatch in the event driver, and a Dioxus dropped-file reader that reads any dropped path through std::fs with no path restriction.
It also took a one-axis overflow clip paint fix, preventDefault ignored on a non-cancelable event, a br after text joining the text's anonymous flex item, two root examples (dnd, file_drop) and two new blitz-tests targets (drag_and_drop, flex_br).
It retired the workspace's one git dependency: Taffy moved from the git pin to 0.16.0 from crates.io; blitz-shell gained the workspace url dependency; no other lock package moved and no telemetry, exporter or error-reporting crate was added.
It added no log site, span, metric or sink: the take reads silent, the 8 println prints are all in the dnd example as example stdout, and nothing in the drop path logs dropped data.
It measured dioxus-native-dom line coverage falling from 91.14 % to 88.59 % against an unchanged 83 % floor; CI read 17 of 17 checks green and the capture checks held 0 needles.
@task_file n=1 path=examples/dnd.rs mark=merge
what: a new root example, a windowed drag-and-drop demo whose println prints are example stdout
@task_file n=2 path=examples/file_drop.rs mark=merge
what: a new root example, a windowed file-drop demo listing dropped files with previews, its file: image and HTML-file previews refused
@task_file n=3 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the engine's new OS drop source in the shell, turning winit drag-and-drop into drag UI events, accepting only file:// URIs
@task_file n=4 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the engine's new DataTransfer model, the dragged items, files and drop effects
@task_file n=5 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: a new upstream integration test target over the engine's drag event dispatch
@task_file n=6 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: a new upstream integration test target over a br inside flex and grid text layout
@task_file n=7 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus native event converter, now with the drag data and the dropped-file reader reading dropped paths through std::fs
@task_file n=8 path=packages/blitz-dom/src/layout_reason.rs
what: escher's layout reason reading inside the engine document crate, its one calc-value call adapted to the new Taffy handle
@task_file n=9 path=Cargo.toml mark=merge
what: the workspace manifest, the Taffy engine dependency moved off its git pin to crates.io and the root dev features of dioxus-native
@task_file n=10 path=Cargo.lock mark=merge
what: the workspace lock, now with no git source, Taffy at 0.16.0
@task_file n=11 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine's base document, gaining the drag target node
@task_file n=12 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the engine's event driver, its drag dispatch and the default-prevented check ignoring preventDefault on a non-cancelable event
@task_file n=13 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the engine's UI event handling, forwarding a drag into a sub-document
@task_file n=14 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout tree construction, a br after text joining the text's anonymous flex item
@task_file n=15 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout entry, adapted to Taffy 0.16
@task_file n=16 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's replaced-element layout, adapted to Taffy 0.16
@task_file n=17 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: the engine's table layout, adapted to Taffy 0.16
@task_file n=18 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: the engine's custom widget interface, a widget accepting a drop only by setting the drop effect
@task_file n=19 path=packages/blitz-paint/src/render.rs mark=merge
what: the engine's paint renderer, clipping only the clipped axis for one-axis overflow clip
@task_file n=20 path=packages/blitz-shell/Cargo.toml mark=merge
what: the shell crate's manifest, gaining the workspace url dependency
@task_file n=21 path=packages/blitz-shell/src/application.rs mark=merge
what: the shell's application event loop, wiring the drop source
@task_file n=22 path=packages/blitz-shell/src/lib.rs mark=merge
what: the shell crate root, declaring the drop source module
@task_file n=23 path=packages/blitz-shell/src/window.rs mark=merge
what: the shell's window, handling drag position, drop and leave from the OS
@task_file n=24 path=packages/blitz-traits/src/events.rs mark=merge
what: the engine's shared UI and DOM event types, gaining the drag events
@task_file n=25 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus native document, its event converter's drag arm
@task_file n=26 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy conversion, its calc-value resolver taking the Taffy calc handle
@task_file n=27 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate target, listing the two new test modules
@task_file n=28 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the upstream pointer-leave integration test, extended with preventDefault not cancelling pointerleave
@task_item n=1 part=touches
Engine tracing behind a cargo feature and engine tracing set by the build's invocation: the engine crates gained new modules (the drop source, the DataTransfer model, drag dispatch) and the root dev features of dioxus-native gained net and html, not tracing; the rule now reaches the new drag code, which adds no tracing site.
@task_item n=2 part=touches
Engine log sites write fixed messages and engine targets print safe fields only: the event driver, the shell window and the drop source changed and add no log site; a row stating which engine paths log must hold for the drag path.
@task_item n=3 part=touches
Session logs carry no page content: dropped file paths, dragged text and dropped file contents are new page content that no log site writes.
@task_item n=4 part=touches
Reach of the scrubbing sink and the sink target allowlist: two new engine module targets exist (the shell drop source, the DataTransfer model) and emit nothing; the capture checks held 0 needles.
@task_item n=5 part=touches
Capture counts framework targets at warn and above: the merged engine and Dioxus native code is framework code under that count; the fast leg's capture read 0.
@task_item n=6 part=touches
escher-telemetry's scrub set url and error fields: the shell now takes the url crate and parses file:// URIs from the OS drop; no such value reaches a log line.
@task_item n=7 part=touches
No file: loads from non-file: documents: the Dioxus dropped-file reader reads any dropped path through std::fs with no path restriction, and the file_drop example refuses its file: image and HTML-file previews while the srcdoc and text previews stand.
@task_item n=8 part=touches
The layout read opens no span: escher's layout reason reading had its one calc-value call adapted to the Taffy calc handle and opens no span.
@task_item n=9 part=touches
The metric coverage overview naming blitz-paint's layer counters as in-process figures: the paint renderer changed for one-axis clip and added no counter or metric.
@task_item n=10 part=touches
The absences of a telemetry backend, metrics exporters, metric emission, an error-reporting service and OTel export: the lock moved only Taffy to 0.16.0 and gave the shell the existing url crate; every absence still holds.
@task_item n=11 part=touches
Coverage report uploaded on success: dioxus-native-dom line coverage fell from 91.14 % to 88.59 % with the floor at 83 % unchanged; a row holding that coverage value or floor reads stale.
@task_item n=12 part=touches
The upstream relation: upstream DioxusLabs/blitz main merged at 29a439e05, the next sync's merge base; a row naming the merge base or the upstream sync's state reads stale.
@task_fact n=1 slug=log-coverage
fact: The upstream take at 29a439e05 added no log site, span or metric: the OS drag-and-drop input path in the engine and the Dioxus dropped-file reader log nothing, by the rule that engine input carries no log site.
@task_fact n=2 slug=log-coverage
fact: The dnd root example's println prints (8, 6 live and 2 commented, none in the file_drop example) are example stdout, not telemetry, and pass through no sink.
@task_fact n=3 slug=pii-scrubbing-compliance
fact: Dropped data (dropped file paths, dragged text, file contents a Dioxus handler reads) is never logged: no site in the drop source, the drag dispatch or the dropped-file reader writes it.
@task_item n=1 part=untouched
The escher-driver command and tool_call spans and their field contracts: no driver code changed and no span was added.
@task_item n=2 part=untouched
The agent-run harness and cold-agent pipe events, verdicts, counted verbs, kept checks and transcripts: none of that harness changed.
@task_item n=3 part=untouched
escher-session's commands, exit codes, stdout, stderr, MCP and coverage roles and host wire: the session binary did not change.
@task_item n=4 part=untouched
escher's sink log line format, the telemetry sink's stream and RUST_LOG, escher-telemetry's events and panic hook, the sink's resource keys: escher-telemetry and its sink did not change.
@task_item n=5 part=untouched
CI integration rows (workflows, per-leg log and artifact uploads, recording and session state directories, the build-directory sweep line, the package-install diagnostic): no CI step, leg list or status shape was edited.
@task_item n=6 part=untouched
The script error surface, window error event, script error source labels, script diagnostics drain and WPT report contracts, and WPT scores published upstream only: no script engine or WPT runner change.
@task_item n=7 part=untouched
The select, optgroup, snapshot, actions answer, layout answer and graph answer PII rows: none of those answers changed.
@task_item n=8 part=untouched
Sections 10, 11 and 12 (SLO budgets, anti-patterns, decisions log) hold no recorded intent and gain none.
