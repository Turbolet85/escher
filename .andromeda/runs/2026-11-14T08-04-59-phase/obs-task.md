@task_prose part=chunk
Bundles a run's fonts: a run's text is shaped and measured from one bundled font set, the stand's DejaVu Sans, with the host's system fonts off, on every path a run boots through (the stand's boot, the driver session's served tasks, the shared fixture pages, a harness-built document that names no font context of its own).
The generic families a run's styles name (sans-serif, serif, monospace, an unknown family) reach the bundled set, not the host.
Keeps the fonts-unavailable refusal as it is: an app whose font context holds no paintable family is still refused before anything is painted or read.
Proves that every CI OS (the test leg's Linux, macOS and Windows runners) reads one state of a run the same way, compared by its readings (snapshot text, bounds, text extents, lint findings), never by a shot's bytes.
Leaves the bundled set reachable by a later canvas painter (0.4.0 canvas text as a future reader); builds no canvas renderer.
@task_file n=1 path=packages/blitz-test-harness/src/harness.rs mark=edits
what: the test harness that builds a document for a check, and its options — edits: HarnessOptions::font_ctx, its default font context
@task_file n=2 path=packages/blitz-dom/src/document.rs
what: the engine document and its font context, which BaseDocument::new fills from the system collection when none is given
@task_file n=3 path=examples/seven_guis/src/stand.rs
what: the headless stand's boot, which builds the single-font context from the bundled DejaVu Sans
@task_file n=4 path=examples/seven_guis/assets/DejaVuSans.woff2
what: the bundled font asset, the stand's DejaVu Sans
@task_file n=5 path=packages/escher-driver/src/refusal.rs
what: the driver's refusal causes, among them fonts-unavailable
@task_file n=6 path=packages/escher-driver/src/screenshot.rs
what: the driver's shot command, which refuses with fonts-unavailable before anything is painted
@task_item n=1 part=needs
The fonts-unavailable refusal cause is kept: its name and text unchanged, still returned before anything is painted or read when the font context holds no family that can paint text.
@task_item n=2 part=needs
No new carrier, verb, argument, field, cause, log field or span: the driver's command span and tool_call span keep their closed field sets, and the refusal cause names keep their closed list.
@task_item n=3 part=needs
The readings two hosts compare: the driver command answer's snapshot text and node bounds, text extents and lint findings; these must read the same on Linux, macOS and Windows for one state of a run.
@task_item n=4 part=needs
The shot (the command answer's PNG image) is never compared by bytes across operating systems; a shot's bytes already differ between linux x86_64 and macOS aarch64.
@task_item n=5 part=needs
Every run path takes the bundled font context with system fonts off: the stand's boot, the driver session's served tasks (escher-session), the shared fixture pages and a harness-built document with no font context of its own.
@task_item n=6 part=needs
Generic font families (sans-serif, serif, monospace, an unknown family) resolve to the bundled set rather than a host font.
@task_item n=7 part=needs
The bundled set is held in the document's font context, not a stand-local constant alone, so a later canvas painter (canvas text, GlyphRun, 0.4.0) can read it; the acceptance names canvas text as a future reader of it.
@task_item n=8 part=needs
The cross-OS proof runs in CI's test leg on its Linux, macOS and Windows runners; whether it is a committed reading each OS replays or a kept check, it is held by readings, not image bytes.
@task_item n=9 part=needs
If the cross-OS proof is held as a committed reading or kept check file, its arguments, answer and answer digest follow the kept check file's shape and hold no private data.
@task_item n=10 part=needs
No change to which font a remote page or the browser app reads, unless brought to the founder as an engine-change fork.
@task_item n=11 part=needs
No new dependency crate; the font asset is a file already in the tree.
@task_item n=12 part=needs
The headless stand keeps running with no telemetry sink, and the driver library still installs no subscriber and emits no event; the chunk's font change adds no log line to either.
@task_standing n=1
convention: each new check or test runs silent, printing nothing on success, and a check that asserts empty stderr unsets RUST_LOG
home: 3-observability-harness-contract
@task_standing n=2
convention: each new committed check or reading holds no private data and no secrets
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: each new CI job or leg uploads its per-leg log on failure and keeps local state local
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
The cold-agent pipe, its events, counted verbs and verdict fields and reasons: the chunk runs no cold agent and changes no verdict.
@task_item n=2 part=not_needed
The agent-run harness events (run.start, boot, test, cleanup, run.end): the chunk adds no agent run.
@task_item n=3 part=not_needed
Metrics, metric emission, exporters, telemetry backends and OTel export: the chunk adds no metric and no backend.
@task_item n=4 part=not_needed
The script error surface, window error event, window.onerror, script error source labels and the WPT report: the chunk touches no script error path and publishes no WPT result.
@task_item n=5 part=not_needed
The select, optgroup, multiple-select and page-event rows of PII scrubbing (chosen sets, list labels, change counts): the chunk changes no form control.
@task_item n=6 part=not_needed
The session recording file, recording bounds and recording directory privacy: the chunk adds nothing to a recording.
@task_item n=7 part=not_needed
The sink log line format, its escaping, scrub set and resource keys (service.name, service.version): the chunk adds no log line and no sink.
@task_item n=8 part=not_needed
The escher-session mcp role, coverage role, help command and host wire: the chunk adds no command, role or wire version.
