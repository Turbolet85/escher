@task_prose part=session
The session fixes where a custom widget is painted (upstream issue 1578). Its paint size is now its content box times the scale, and its scene is placed at the content origin inside the element's transform, unclipped.
A custom widget's pointer events now arrive in its content-box frame. Sub-document event forwarding is unchanged.
The Widget trait gains a defaulted fonts method, and BaseDocument gains a public font_ctx accessor, so before each paint a widget is handed the document's own shared font context. Canvas text is then shaped from the bundled fonts.
Three new headless stand checks witness these fixes: stand_canvas_placement, stand_canvas_pointer_frame and stand_canvas_text. The stand_*.rs files under blitz-tests move from 66 to 69, and stand-repeat's named-target count is unchanged.
blitz-tests takes parley as a dev-dependency (one lock line, no package added). The lock alone moves cookie 0.18.2 to 0.18.3 to fix RUSTSEC-2026-0594, with no deny.toml change. The audit reads advisories ok, licenses ok, and CI reads green with 17 of 17 checks.
No log site, span, field, event, IPC method, verb, carrier, cause, exit or env var is added. The three checks read silent.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: blitz-paint's custom-widget scene building behind the custom-widget feature. It sets a widget's paint size (content box times scale) and hands the widget the document's font context just before paint.
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: blitz-paint's element renderer. It appends a custom widget's scene at the content origin inside the element's transform.
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs
what: blitz-dom's DOM event handling. It forwards pointer events to a custom widget, now offset into its content-box frame, and its sub-document forwarding arm is unchanged.
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: the Widget trait that custom widgets implement (paint, handle_event, and now the defaulted fonts method).
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: blitz-dom's BaseDocument, the engine document. It holds the shared font context, now exposed through the public font_ctx accessor.
@task_file n=6 path=tests/blitz-tests/Cargo.toml
what: the manifest of the blitz-tests integration-test crate and its dev-dependencies, which now include parley.
@task_file n=7 path=Cargo.lock
what: the workspace lockfile, read by the dependency audit leg. It pins dependency versions, including cookie, which is now 0.18.3.
@task_file n=8 path=tests/blitz-tests/tests/stand_canvas_placement.rs
what: a headless stand check that a custom widget paints its content box at the content origin, inside its transform, in both layout modes.
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_pointer_frame.rs
what: a headless stand check that a padded custom widget hears its pointer events from its content box, in both layout modes.
@task_file n=10 path=tests/blitz-tests/tests/stand_canvas_text.rs
what: a headless stand check that canvas text is shaped from the document's own font context and matches the bundled-font route, including under font-embolden.
@task_item n=1 part=touches
Silent checks and shared modules: the three new stand checks join the silent checks. They declare neither shared module (session_common, fixtures), and they print and log nothing.
@task_item n=2 part=touches
Headless stand runs with no sink: the three new checks run a headless stand session with no telemetry sink installed.
@task_item n=3 part=touches
Engine log sites write fixed messages, and engine tracing behind a cargo feature or set by the build's invocation: the custom-widget code paths in blitz-paint and blitz-dom changed, and no log site or tracing span was added to them.
@task_item n=4 part=touches
Shot image leaves only in its answer: the new checks read the stand's shot raster in process and compare its pixels, and no image is written out or logged.
@task_item n=5 part=touches
Session logs carry no page content, and escher targets redact the content-named set: app-drawn canvas text now reaches the shot only as painted pixels, and no log, span or answer field carries it.
@task_item n=6 part=touches
The fonts-unavailable refusal cause: canvas text is now shaped from the document's own bundled font context, handed to the widget, and no refusal cause is added or changed.
@task_item n=7 part=touches
The absences of a telemetry backend, metrics exporters, metric emission and an error-reporting service: the lockfile moved (cookie 0.18.3, plus one parley line for blitz-tests), and no telemetry, metrics or error-capture crate was added, so these absences still hold.
@task_fact n=1 slug=6-log-coverage
fact: Three custom-widget changes add no log site, span or field, and the stand checks that witness them read silent: paint placement at the content box, the content-box pointer frame, and the document font-context handoff to a widget.
@task_item n=1 part=untouched
The driver's command and tool_call spans, their fields, endings and refusal-cause names, other than fonts-unavailable: no verb, span or field changed.
@task_item n=2 part=untouched
escher-session's roles, stdout, stderr, exit codes, error object, host wire and mcp role: nothing in the session binary changed.
@task_item n=3 part=untouched
The cold-agent pipe and the agent-run harness: their events, verdict fields, reasons, counted verbs, kept checks, transcripts and cleanup.
@task_item n=4 part=untouched
escher's telemetry sink and escher-telemetry: the log line format, escaping, scrub set, target allowlist, RUST_LOG handling, resource keys, panic hook and sink installation in each native main.
@task_item n=5 part=untouched
Script error capture and reporting: the window error event, window.onerror, the script diagnostics drain, script error source labels and the WPT report.
@task_item n=6 part=untouched
The recording file, the recording and session state directories, the session lifecycle and the build-directory sweep line.
@task_item n=7 part=untouched
CI integration: no workflow, artifact upload or package-install diagnostic changed. CI simply ran green on the existing workflows.
@task_item n=8 part=untouched
The PII rows on select, optgroup, snapshot, actions, layout, graph and coverage answers and on the mutation buffer: none of those answers or paths changed.
