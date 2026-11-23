@task_prose part=session
The session made what an app paints from Rust through a custom widget appear in a headless shot: the harness shot and the driver's screenshot, whole and cropped to the element, in both layout modes, in process only.
It turned on blitz-paint's custom-widget feature in the test harness, so the widget paint pass is compiled into every build selection that reaches the harness, the driver and the escher-session binary included.
It added one stand check, the canvas shot check, which joins the repeated stand checks and the run stand selection through the existing stand_ prefix and declares the shared session module.
It added peniko as a dev-dependency of the blitz-tests stand checks; the lock gains one line and no package.
It added no span, span field, log field, event, cause, session error, exit, verb, argument, answer field or CI leg; CI went green 17 of 17 on the pushed commit.
The founder ratified a canvas's app-painted content crossing as pixels in a shot on every carrier, and in process only for now.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_shot.rs
what: a new headless stand check that paints a widget from Rust and reads its pixels in a harness shot and a driver shot, whole and element-cropped; a failure names a layout mode, a row and the pixel it read
@task_file n=2 path=packages/blitz-test-harness/Cargo.toml
what: the test harness's build manifest and feature set, now naming blitz-paint's custom-widget feature
@task_file n=3 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests stand checks' manifest and dev-dependencies, now with peniko
@task_file n=4 path=Cargo.lock
what: the workspace lock file, one line added to the blitz-tests entry and no package
@task_item n=1 part=touches
The shot part of the driver command answer: its image may now hold an app-painted widget's pixels, whole or cropped to the element; its fields are unmoved.
@task_item n=2 part=touches
The rule that a shot image leaves only in its answer: it now carries a canvas's app-painted content, which crosses only as pixels in that image.
@task_item n=3 part=touches
The open-list shot carrying labels as pixels: the precedent beside which a canvas's content crossing as pixels in a shot now stands.
@task_item n=4 part=touches
Session logs carrying no page content and engine log sites writing fixed messages: the widget paint pass now compiled into the harness, the driver and escher-session must keep to them; it adds no log site.
@task_item n=5 part=touches
Driver spans carrying no content and one span per driver command, with the command span's screenshot field: a screenshot now paints widget content, and no span or field was added for it.
@task_item n=6 part=touches
The kept check file and its answer and answer digest: a kept check keeps no pixels, so a canvas's content is not kept with it.
@task_item n=7 part=touches
The cold-agent carriers, the model provider on a live run included: a canvas's pixels would ride a screenshot answer to it once a served app holds a canvas; none does yet.
@task_item n=8 part=touches
Silent checks and shared modules: one more stand check declares the shared session module and must stay silent.
@task_item n=9 part=touches
Log check failures naming a kind and a count, and the headless stand running with no sink: the new stand check's failure message carries a layout mode, a row and the pixel's RGBA it read only.
@task_item n=10 part=touches
Engine tracing set by a cargo feature and by the build's invocation: the harness's feature set moved by custom-widget alone; no tracing feature moved.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: A canvas's app-painted content crosses only as pixels in a shot (the harness shot and the driver's screenshot, whole or element-cropped) on every ratified carrier, the model provider on a cold-agent live run included, in process only for now; no span, log field, cause, verdict field or kept check carries it, a kept check keeping no pixels.
@task_fact n=2 slug=3-observability-harness-contract
fact: A stand check reading pixels names in its failure message only a layout mode, a row and the RGBA of the pixel it read, a measured number and never page text or a control value.
@task_item n=1 part=untouched
The telemetry sink, its log line format, scrub set, resource keys, target allowlist and RUST_LOG handling: no sink or log site changed.
@task_item n=2 part=untouched
The command and tool_call span contracts, their fields, endings and refusal causes: no span, field or cause added.
@task_item n=3 part=untouched
Metric coverage and the absences of a telemetry backend, metrics exporters or an error-reporting service: none added.
@task_item n=4 part=untouched
Error capture: the panic hook, the script error surface, the window error event and the WPT report are unchanged.
@task_item n=5 part=untouched
The agent-run and cold-agent events, the cold-agent verdict fields, reasons and counted verbs: no event, field or verb added.
@task_item n=6 part=untouched
The escher-session wire, exit codes, error object, coverage role, recording file and session lifecycle: unchanged.
@task_item n=7 part=untouched
CI integration: no workflow, leg, artifact upload or build-directory sweep changed.
@task_item n=8 part=untouched
The select, snapshot, layout, graph and actions answer contracts and page events: no field moved.
