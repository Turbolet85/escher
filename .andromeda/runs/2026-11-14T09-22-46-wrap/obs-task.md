@task_prose part=session
The session added two stand checks to the test crate and changed no production code, manifest, workflow, committed kept check or dependency.
The cross-OS reading now reaches the six served tasks (counter, flight-booker, timer, crud, cells, load): a 48-row table of the driver's answer digests for snapshot (every part), lint and actions, both layout modes, at boot and after one click, type or advance step, read the same on linux x86_64, windows x86_64 and macOS aarch64 in CI. Before, only the two committed Counter kept checks were read across the three OSes.
The generic families (sans-serif, serif, monospace, system-ui, an unavailable named family, cursive, fantasy) were measured to lay out in the stand's one bundled font, DejaVu Sans, by fontique's fallback to the only font loaded rather than by per-family registration; cursive and fantasy are registered for nothing yet read sans-serif's extent, so nothing is deferred.
Both new checks are silent and hold no shot (the print and screenshot census reads 0); the fonts-unavailable refusal is kept unchanged; CI ran 17 of 17 checks green with both checks passing on the three OSes and in the stand-repeat leg.
@task_file n=1 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: a stand check, the cross-OS reading of the headless stand: holds the six served tasks as the session host holds them, runs each call through the session's run and compares the driver's answer digests of snapshot parts, lint and actions to one committed table on every CI OS; silent, no shot, no kept check file
@task_file n=2 path=tests/blitz-tests/tests/stand_bundled_font_families.rs
what: a stand check, the generic font families on the stand's bundled font: boots a fixture page with the stand's options and reads each family's text extent against sans-serif in both layout modes; silent, no shot
@task_item n=1 part=touches
decision/checks-silent (Silent checks and shared modules): two new stand checks join the checks it rules over; both print nothing and the census of prints and screenshots reads 0 - the rule holds, a guard it may name.
@task_item n=2 part=touches
contract/kept-check/answer-digest (kept check file, answer digest): the driver's answer digest now also has a stand-check reader outside any kept check file - a committed 48-row table compared on linux, windows and macOS; the recording caller of the digest is kept unchanged.
@task_item n=3 part=touches
decision/headless-stand-no-sink (Headless stand runs with no sink): the new checks boot the headless stand through the session host, the Cells and load boots and the fixtures boot with the stand's options and install no sink - the rule holds over a wider reach.
@task_item n=4 part=touches
decision/hosted-call-through-session-run (Hosted call runs through the session's run alone): the cross-OS check runs every call through the session's run before reading its digest - a new caller under that rule, unchanged.
@task_item n=5 part=touches
decision/driver-library-installs-no-subscriber and decision/driver-emits-no-event: the new check drives the driver library in-process and installs no subscriber - true, unchanged, a possible guard.
@task_item n=6 part=touches
contract/refusal-cause/fonts-unavailable (Refusal cause names, fonts-unavailable): kept unchanged; the bundled-font stand still refuses where fonts are unavailable, its existing stand checks green unchanged.
@task_item n=7 part=touches
contract/command-answer/shot and decision/shot-image-leaves-only-in-answer: the cross-OS reading compares no shot and holds no screenshot - its digests cover snapshot, lint and actions only; the rules hold.
@task_item n=8 part=touches
decision/advance-accepted-every-session (advance accepted on every session): the cross-OS step uses advance on the timed tasks along with click and type, never press - a reader of that rule, unchanged.
@task_item n=9 part=touches
decision/actions-read-changes-nothing and contract/command-span/snapshot-part: the check reads actions and every snapshot part (Cells' six) at boot and after a step and compares the reading - true, unchanged.
@task_item n=10 part=touches
decision/ci-platform-github-workflows and the CI legs: the test leg, the windows and macOS matrix legs and the stand-repeat leg (three rounds by its stand_ glob) now run the two new checks; no workflow, step or verdict shape changed and no artifact was added.
@task_item n=11 part=touches
decision/committed-kept-checks-clean and decision/committed-checks-hold-no-private-data: no kept check file was added; the committed table holds only task, moment, verb, part and a 16-hex digest, no page content - the rules hold.
@task_fact n=1 slug=3-observability-harness-contract
fact: The driver's answer digests of snapshot (every part), lint and actions for the six served tasks, in both layout modes at boot and after one step, read the same on linux x86_64, windows x86_64 and macOS aarch64, held by a silent stand check against one committed table, with no shot and no kept check file.
@task_item n=1 part=untouched
Telemetry strategy and the tracing crate rows (engine tracing feature, escher crates tracing ungated, telemetry via tracing): no tracing call, feature or crate changed.
@task_item n=2 part=untouched
The command and tool_call span contracts and their field lists, and the closed span line format: no span, field or level changed.
@task_item n=3 part=untouched
escher's telemetry sink, its log line layout, scrub set, target allowlist, resource keys and RUST_LOG handling, and the no-OTel-export decision: untouched.
@task_item n=4 part=untouched
Metric coverage and the metrics and telemetry-backend absences, and the WPT-scores-upstream rows: no metric, exporter or backend added.
@task_item n=5 part=untouched
Error capture and reporting rows (panic hook, script error surface, window error event, WPT report messages): untouched.
@task_item n=6 part=untouched
The agent-run and cold-agent harness rows (events, verdict fields, counted verbs, transcripts, cleanup): no harness script or verdict changed.
@task_item n=7 part=untouched
escher-session roles, stdout, stderr, exit codes, host wire, error object, coverage line and the recording file, recording directory and session state directory rows: no session binary or recording behavior changed.
@task_item n=8 part=untouched
PII scrubbing rows over select lists, optgroup labels, mutation buffers, page events and the state registry: no production code reached them.
