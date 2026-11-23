@task_prose part=session
The session added one headless stand check that holds a table of twelve screenshot answer digests for three canvas-only fixtures (a solid fill, an antialiased edge, canvas-painted glyphs), in the whole and element shot forms and in both layout modes.
It measured that a canvas-only shot's bytes are equal on CI's linux x86_64, windows x86_64 and macOS aarch64 hosts: all twelve rows held on all three hosts in one green CI run.
It edited the capability statement by hand: the shot-bytes limit that a shot's PNG bytes differ between linux x86_64 and macOS aarch64 now applies only to a shot holding document text, and a canvas-only shot is stated as equal on the three hosts, with the unmeasured canvas conditions listed.
The founder ratified byte equality for canvas-only shots across hosts; the no-shot cross-OS rule keeps standing for page text shots.
No product symbol, verb, argument, field, refusal cause, exit code, log field, span, sink setting, CI script or dependency moved; no PNG was committed and no kept or committed check changed; nothing was retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: a new headless stand check of the driver's screenshot answer: a committed table of answer digests for canvas-only shots, run with no sink and printing nothing, read across the CI hosts
@task_file n=2 path=docs/capabilities.md
what: the capability statement, updated by hand: its shot-bytes limit and its list of unmeasured conditions
@task_item n=1 part=touches
decision/capability-statement-updated-by-hand: the capability statement was edited by hand this session, scoping its shot-bytes limit to shots holding document text and adding a canvas-only exception.
@task_item n=2 part=touches
contract/command-answer/shot: the shot answer's answer digest is now read by a committed stand check as a cross-host constant for canvas-only content; any row that says a shot's bytes hold on one host only, or differ by OS without qualification, is now stale for a canvas-only shot.
@task_item n=3 part=touches
decision/shot-image-leaves-only-in-answer: the new check reads the shot answer and keeps only its digest; no image is written or committed, so the rule holds but gains a check that relies on it.
@task_item n=4 part=touches
decision/headless-stand-no-sink: one more headless stand check runs with no sink and prints nothing.
@task_item n=5 part=touches
decision/checks-silent: the new check and the shared session helpers it reads print nothing; a temporary print used to read the digests was removed before commit.
@task_item n=6 part=touches
decision/committed-checks-hold-no-private-data and decision/committed-kept-checks-clean: a committed check now holds a table of shot digests of fixture canvases, no page content, no image and no private data.
@task_item n=7 part=touches
decision/open-list-shot-carries-labels and decision/shot-image-leaves-only-in-answer, indirectly: the canvas text fixture paints glyphs as pixels on a canvas, so its content crosses only as pixels inside the shot answer, unchanged.
@task_item n=8 part=touches
decision/ci-platform-github-workflows: the cross-host reading was taken from the linux, windows and macOS test jobs of the GitHub workflow; no workflow or leg script line moved, the repeat leg takes the new check by its name prefix.
@task_item n=9 part=touches
contract/command-span/screenshot and contract/cold-agent-counted-verbs/screenshot: the screenshot verb is exercised by the new check; its span and counting are unchanged.
@task_fact n=1 slug=3-observability-harness-contract
fact: The screenshot answer's digest for a canvas-only shot is byte-equal on linux x86_64, windows x86_64 and macOS aarch64, held by a committed digest table; a shot holding document text still differs between linux x86_64 and macOS aarch64.
@task_item n=1 part=untouched
Telemetry strategy, tracing features and the sink: no tracing call, subscriber, sink target, RUST_LOG handling or resource key moved.
@task_item n=2 part=untouched
Span and trace coverage: the command and tool_call spans, their fields and endings, and the refusal cause names are unchanged.
@task_item n=3 part=untouched
Metric coverage and the absences of metrics, exporters, OTel and an error-reporting service stand as they were.
@task_item n=4 part=untouched
Log line format, scrubbing by field name, escaping and the content-named set: no log field or scrub rule moved.
@task_item n=5 part=untouched
Cold-agent and agent-run harness events, verdict fields and counted verbs other than the screenshot exercise: no harness changed.
@task_item n=6 part=untouched
Error capture: panic hooks, script error labels, the window error event and the WPT report are not reached.
@task_item n=7 part=untouched
Session lifecycle, recording directory and recording file, exit codes, host wire and session state directory: unchanged.
@task_item n=8 part=untouched
CI artifact uploads and the build-directory sweep line: no artifact, upload or sweep changed.
