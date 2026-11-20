@task_prose part=chunk
Ends the four browser mutants the 0.3.0 Epoch 6 boundary audit read as missed: render_scene's returned (u32, u32) size replaced by (0, 0), (0, 1), (1, 0) and (1, 1), at the coordinate re-derived at HEAD as capture.rs:86:5 (render_scene declared at line 81).
Either a direct unit test in the browser capture module's own test module calls render_scene and asserts the returned size for two inputs of different viewport size, or the unused return value is removed, or the four are kept on the founder's word; the choice is a fork put to the founder.
Re-measures at HEAD that no caller reads render_scene's returned size, and tests the directive's hypothesis that a two-input size assertion catches all four.
Any cargo-mutants run, even of these four alone, is planned as an operator entry or a fork for the founder, never as a gate that fires on its own.
@task_file n=1 path=apps/browser/src/capture.rs mark=edits
what: the browser's capture module, which renders a document's scene for a screenshot or an anyrender scene capture — edits: render_scene (its return value, if the fork removes it) and the module's own #[cfg(test)] test module
@task_item n=1 part=needs
render_scene's returned (u32, u32) size: the value the four mutants replace; one input whose viewport is wider and taller than 1 px tells all four constants apart from the real size, and a second input of a different size shows the value follows the input.
@task_item n=2 part=needs
The four surviving mutants as the audit's survivors list holds them (metric met-mjjioowg, cargo-mutants 27.1.0, the mutation:browser run with nextest): each must end caught by a browser test or kept by the founder's word.
@task_item n=3 part=needs
The re-derived coordinate: capture.rs:86:5 is the first statement of render_scene's body, the span cargo-mutants gives a function-value mutant; one upstream commit since the audit moved no line at or above 86.
@task_item n=4 part=needs
render_scene's two callers, the screenshot capture and the anyrender scene capture, both discard its returned size, and the screenshot capture resolves the same size itself before calling; so a test catches the four only by calling render_scene directly and asserting what it returns.
@task_item n=5 part=needs
A new direct unit test must build what render_scene takes, a document with a set viewport and a paint-scene sink, inside the browser crate's own test module under its default features; whether it can is checked at P3.
@task_item n=6 part=needs
The fork for the founder: a test that pins an unused value, removing the return value (render_scene returning nothing, after which no value mutants are generated), or keeping the four on his word; removal edits upstream browser code the fork does not develop and adds to the upstream divergence.
@task_item n=7 part=needs
Any cargo-mutants run starts only on the founder's word, carried as an operator entry or a fork put to him; the mutation tier runs on the dev host only, never in CI.
@task_item n=8 part=needs
The browser crate's test run must stay green with the new test in it, and every existing check stays green.
@task_item n=9 part=needs
The chunk adds no carrier, verb, log field, span, env read or listener, and changes nothing the browser paints or saves; a new test emits no telemetry of its own.
@task_item n=10 part=needs
Whether the chunk can claim the codebase-health capability's last leg, given the next boundary audit has not run, is left to P4's claim lean.
@task_standing n=1
convention: each new test is a silent check, printing nothing on success and installing no telemetry sink of its own
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
The escher-driver command and tool_call span contracts and their field names: the chunk opens and changes no span.
@task_item n=2 part=not_needed
The cold-agent pipe's events, counted verbs and verdict fields, and the agent-run harness events: the chunk runs and changes neither harness.
@task_item n=3 part=not_needed
The escher-session roles, commands, exit codes, error object, coverage line and host wire: the chunk touches no session binary.
@task_item n=4 part=not_needed
The answer contracts of the driver's verbs (actions, layout, graph, snapshot, shot) and the select and page-event contracts: the chunk changes no answer or page behaviour.
@task_item n=5 part=not_needed
The script error surface, window error event, window.onerror and script error source labels: the chunk touches no script error path.
@task_item n=6 part=not_needed
WPT report messages and WPT score publishing: the chunk runs and changes no WPT.
@task_item n=7 part=not_needed
The recording file, recording and session-state directories, session lifecycle and their refusals: the chunk records nothing.
@task_item n=8 part=not_needed
The build-directory sweep line and the package-install diagnostic line: the chunk changes neither script.
