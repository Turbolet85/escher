@task_prose part=session
The session added test code alone to the browser crate: two unit tests over the browser's scene capture, which return the rendered size at the viewport (800 x 600 at scale 1.0) and at a HiDPI scale (1280 x 720 at scale 2.0).
The tests sit out of line in a sibling _tests file, declared as a cfg(test) module by path; no public, crate-visible or IPC surface, dependency, schema, config, log, span or field changed.
It measured: the four render_scene mutants that survived the 0.3.0 Epoch 6 mutation audit now read caught, under four hand controls and under a cargo-mutants run fired on the founder's word (4 mutants tested, 4 caught, run at 2 jobs on the operator's cap).
A first cargo-mutants run with an unanchored filter also tested capture_anyrender_scene and read it missed; it lies outside the founder's word and is carried as a residual and a next-version incubator line, with no work on it here.
CI ran green on the pushed commit, 17 of 17 checks, its 2634 s wall past the gate's 2400 s wait; the raw cargo-mutants output was moved, not deleted, into the gitignored build directory.
Nothing was retired.
@task_file n=1 path=apps/browser/src/capture.rs
what: the browser's scene capture: renders a document's scene at the viewport or the physical size and captures the anyrender scene behind the browser's screenshot; edited to append the declaration of its cfg(test) unit-test module
@task_file n=2 path=apps/browser/src/capture_tests.rs
what: new; the browser capture's unit tests, out of line: a document builder and two tests pinning the rendered size at the viewport and at a HiDPI scale
@task_item n=1 part=touches
Indirect: any rule over the browser's screenshot or shot capture path (the shot answer, the shot image leaving only in its answer) is reached only by test code that pins the capture's rendered size; the capture's behaviour, output and logging are unchanged.
@task_item n=2 part=touches
A row holding the browser crate's unit-test count or a workspace test tally: it moves by two named tests, capture::tests::render_scene_returns_the_viewport_size and capture::tests::render_scene_returns_the_physical_size_at_a_hidpi_scale; the browser's nextest run under the mutants profile reads 45 passed.
@task_item n=3 part=touches
A row holding mutation-survivor counts or a mutation quality gate for the browser: the 0.3.0 audit's four render_scene survivors now read caught; one further mutant, capture_anyrender_scene replaced with nothing, read missed outside the founder's word and stands as a residual.
@task_item n=4 part=touches
A row holding the fork CI's wait bound or its measured wall time: CI read green at 17 of 17 checks with a 2634 s wall, past the 2400 s wait sized from the last measured 2253 s; the first read ended in progress and was re-fired unchanged.
@task_item n=5 part=touches
Rows over local state kept local and the build-directory sweep: raw cargo-mutants output (debug logs, log directories, outcomes) now stands under the gitignored build directory as target/mutants-raw-2026-11-20 and target/mutants-tmp, left for the operator; nothing was uploaded or committed from it.
@task_item n=6 part=touches
Rows that host paths or raw tool logs stay out of committed records: hygiene refused seven raw files for host paths and they were moved out of the committed evidence, keeping only the caught, missed, mutant list, diff, lock, timeout and unviable records.
@task_item n=1 part=untouched
Telemetry strategy and the tracing crate's use: no tracing call, span, event, feature gate or subscriber was added or changed in any crate.
@task_item n=2 part=untouched
The escher-driver command and tool_call spans, their fields, the refusal cause names and tool_call endings: no driver code changed.
@task_item n=3 part=untouched
escher's telemetry sink, its log line format, scrub set, target allowlist, RUST_LOG handling and the panic hook: untouched.
@task_item n=4 part=untouched
The agent-run and cold-agent harnesses, their events, verdict fields, counted verbs, kept checks and recording directories: untouched.
@task_item n=5 part=untouched
escher-session's roles, stdout, stderr, exit codes, host wire, coverage line and the actions, layout, graph and snapshot answers: untouched.
@task_item n=6 part=untouched
PII scrubbing rules, script error capture and the window error surface, WPT reporting and publishing: untouched.
@task_item n=7 part=untouched
The absences of a telemetry backend, metric emission, metrics exporters and an error-reporting service: still absent; no dependency was added.
