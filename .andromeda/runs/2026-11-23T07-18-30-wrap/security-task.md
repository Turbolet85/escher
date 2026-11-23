@task_prose part=session
The session proved that what an app paints from Rust through a custom widget shows up in a headless shot: the harness shot and the driver's whole and cropped element shot both carry the widget's pixels, in both layout modes, in process only.
It turned on blitz-paint's custom-widget feature in the test harness, so blitz-paint's widget paint pass is compiled into every build selection that reaches the harness: the harness, the driver and the escher-session binary.
It added one stand check (stand_canvas_shot) to the blitz-tests stand checks, and blitz-tests now takes peniko as a workspace-inherited dev-dependency. That adds one lockfile line and no package.
No public symbol, driver verb, argument, result field, refusal cause, session error, exit code, log field or span was added. No engine source changed, and no CI script, leg or verdict shape changed.
Every check was run locked and came back green, including the dependency audit (cargo-deny). CI came back green on all 17 checks. No served app holds a canvas yet, so no carrier outside the process carries one today.
The founder ratified that a canvas's app-painted content crosses as pixels in a shot on every ratified carrier, with no shot field moved and no pixels in a kept check.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_shot.rs
what: new stand check in the unpublished blitz-tests crate. It checks that a widget painted from Rust shows in a harness shot and in a driver shot, whole and cropped to the element, in both layout modes, in process only. A failure message carries a layout mode, a row and the RGBA pixel it read.
@task_file n=2 path=packages/blitz-test-harness/Cargo.toml
what: edited. The test harness's manifest, with its raster dependencies; its blitz-paint dependency now names the custom-widget feature.
@task_file n=3 path=tests/blitz-tests/Cargo.toml
what: edited. The manifest of blitz-tests, the unpublished test crate holding the stand checks; it gains peniko as a workspace-inherited dev-dependency.
@task_file n=4 path=Cargo.lock
what: edited. The lockfile that locked builds and the cargo-deny audit leg read; one line added (peniko in the blitz-tests entry), no package added.
@task_item n=1 part=touches
The select-list precedent: an open select list's labels cross as pixels in a shot, and its crossing is limited to bounds and pixels. A canvas's app-painted content now lands beside it as a second content class that crosses only as pixels in a screenshot.
@task_item n=2 part=touches
Cold-agent runs carry the screen to the provider. Since the founder's ratification, the screen sent to the model provider on a cold-agent live run may hold a widget's app-painted pixels.
@task_item n=3 part=touches
The rules that carry screen pixels and the screenshot in band, the screenshot answer as png, and the MCP tool result's screenshot: a shot now holds widget-painted content. No shot field or answer shape moved.
@task_item n=4 part=touches
The session socket carries what the screen reads: a driver shot read over the socket would now hold a widget's pixels. No verb, argument or result field was added.
@task_item n=5 part=touches
The value mask that holds on every carrier, and masked password and file input values: the mask covers inputs only. A widget's painted content crosses unmasked as pixels.
@task_item n=6 part=touches
The kept-check rows (a kept check keeps each answer whole, and what a kept check file contains): a kept check still keeps no pixels, and the canvas crossing states this as one of its terms.
@task_item n=7 part=touches
Screen content stays out of logs: widget pixels reach no log field or span. The check's own failure message prints a measured RGBA pixel, but no page text and no control value.
@task_item n=8 part=touches
Test harness takes no tracing and no style: the harness's raster dependencies are unchanged, but the feature set of one of them (blitz-paint) moved to include custom-widget.
@task_item n=9 part=touches
Custom widget initial attribute replay, which sits behind blitz-dom's custom-widget feature: blitz-paint's widget paint pass, part of the same feature, is now compiled into every selection that reaches the harness.
@task_item n=10 part=touches
Workspace-inherited dependency versions: blitz-tests takes peniko with workspace = true. The rows that name peniko as a type source remain true.
@task_item n=11 part=touches
Locked builds and the lockfile: one line was added to the lock and every check ran with --locked. Upstream sync takes the dependency set whole, and engine dependency pins: no package was added or moved.
@task_item n=12 part=touches
Dependency audit with cargo-deny, the audit CI gate and the local audit before a push: the audit leg was green over the changed graph, and no advisory ignore was added.
@task_item n=13 part=touches
blitz-tests not published: the unpublished test crate gains a dev-dependency and a stand check, and it stays unpublished.
@task_item n=14 part=touches
Closed driver verb schema, and the absence of a driver verb that takes a URL: both still hold. The canvas crossing adds no verb, argument kind, refusal cause, session error or exit.
@task_fact n=1 slug=threat-model-summary
fact: A canvas's app-painted content (a widget painted from Rust) crosses only as pixels in a screenshot, on every ratified carrier, including the model provider on a cold-agent live run. It needs no new carrier, field, cause, session error, exit, log field or span, and a kept check keeps no pixels. It was ratified by the founder, and today it is exercised in process only.
@task_fact n=2 slug=dependency-security
fact: blitz-test-harness turns on blitz-paint's custom-widget feature, so blitz-paint's widget paint pass is compiled into the harness, the driver and the escher-session binary in every build selection that reaches the harness. blitz-tests takes peniko as a workspace-inherited dev-dependency, which adds one lock line and no package.
@task_item n=1 part=untouched
Authentication and authorization: CI tokens, the publish job guard and publish secrets. No workflow, token or secret changed.
@task_item n=2 part=untouched
Secret management and the environment variables contract. No environment variable is read or added.
@task_item n=3 part=untouched
Outbound requests and navigation in API security (fetch methods, per-host limits, CORS and same-origin absences, external schemes) and the session socket's bounds, connections, idle expiry and request line. None of them changed.
@task_item n=4 part=untouched
Error handling: refusal causes, refusal and session error JSON, exit codes, MCP protocol errors and WPT results. No cause, error or exit was added.
@task_item n=5 part=untouched
The logging sink, target allowlists and the command and tool_call span contracts. No log field, target or span was added.
@task_item n=6 part=untouched
Data protection for the browser's history, cookies, HTTP cache, TLS and clipboard, and the absence of database encryption. Not reached.
@task_item n=7 part=untouched
Engine input-validation rows: select list behaviour, range input, password painting, stable element ids, recording and the state directory. No engine or driver source changed.
@task_item n=8 part=untouched
Counts of stand checks or session_common declarers. No row of security-plan holds such a count.
@task_item n=9 part=untouched
CI actions pinned to SHAs, the licence gate, skia fetch and signing-material rows. No CI script, leg or workflow changed.
