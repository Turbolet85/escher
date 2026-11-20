@task_prose part=chunk
Ends the four browser mutants the 0.3.0 boundary audit read as missed: render_scene's returned (u32, u32) size replaced by (0, 0), (0, 1), (1, 0) and (1, 1), coordinate re-derived at HEAD as capture.rs:86:5.
They are either caught by a browser unit test that calls render_scene directly and asserts the size it returns for two inputs of different size, or ended by removing the unused return value, or kept on the founder's word — a fork put to the founder.
Re-measured at HEAD: no caller reads the returned size, so only a direct call can tell the four apart from the real value.
Any cargo-mutants run, even of these four alone, starts only on the founder's word, planned as an operator entry or a fork, never as a gate that fires by itself.
@task_file n=1 path=apps/browser/src/capture.rs mark=edits
what: the browser app's screenshot capture component, which paints a document's scene and resolves its render size — edits: the render_scene function's return value (only if the fork rules removal) and the file's own inline #[cfg(test)] unit test module
@task_item n=1 part=needs
The returned size of render_scene: a new unit test asserts it for one input whose viewport is wider and taller than 1 px and for a second input of a different size, so each of the four constant mutants (both dimensions at most 1) fails it and the value is shown to follow the input.
@task_item n=2 part=needs
The new test lives in the capture file's own inline test module and reaches render_scene there, without widening the function's visibility or adding a seam for the test.
@task_item n=3 part=needs
The test builds what render_scene takes inside the browser crate under its default features: a document with a set viewport and scale, and a scene painter written by hand in the test, with no mocking library and its data stated in code.
@task_item n=4 part=needs
The expected sizes are pinned as literal values in the test, not recomputed by the code under test or by the same size resolution the capture path uses.
@task_item n=5 part=needs
The test is controlled red-before-green: it is shown to fail when the returned size is replaced by each of the four constants, and to pass on the real code.
@task_item n=6 part=needs
The mutation tier: a cargo-mutants run under nextest of the browser package, even of these four alone, is started only on the founder's word, as an operator entry or a fork put to him, on the dev host only and never in CI; the plan carries no gate that runs it by itself.
@task_item n=7 part=needs
The mutation baseline is read per unit: the audit's survivors list for the browser holds these four, and the chunk's outcome is each one read caught by the test or kept by the founder's word.
@task_item n=8 part=needs
The fork between a test that pins an unused return value, removing that value from upstream browser code (adding to the upstream divergence of an app the fork does not develop), and keeping the four on the founder's word is the founder's to rule.
@task_item n=9 part=needs
No change to what the browser paints or saves, and no new carrier, verb, log field, span, env read or listener: the capture path's behaviour stays as it is whichever way the fork goes.
@task_item n=10 part=needs
The browser crate's own test run is the check that holds the new test green; a per-package green is not read as the workspace green.
@task_standing n=1
convention: a new unit test sits in an inline test module in its source file
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: a new unit test pins its expected value as a literal, is controlled red-before-green, and its failure prints no argument value
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=3
convention: a new test uses hand-written stand-ins, never a mocking library
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=4
convention: a new test's data is stated in code, nothing seeded
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: a regression test documents and is named for what it guards
home: 2-test-strategy · test-strategy
@task_item n=1 part=not_needed
The session host, session wire, session errors, state directory and recording contracts: the chunk opens no session and touches no socket or recording.
@task_item n=2 part=not_needed
The escher-session command line, MCP role, exit codes, stdout and stderr contracts: no command-line or MCP surface is touched.
@task_item n=3 part=not_needed
Driver verbs, refusal causes, snapshot, lint defects and handler coverage rows: the chunk adds no verb and drives no app through the driver.
@task_item n=4 part=not_needed
The agent-run and cold-agent contracts and decisions (events, verbs, verdicts, exit codes, isolation): no agent-run or cold-agent harness is reached.
@task_item n=5 part=not_needed
The seven_guis headless stand, stand log, stand-repeat and committed recorded checks: the chunk boots no stand and records no check.
@task_item n=6 part=not_needed
The WPT runner, its CLI, outputs, status set and conformance decisions: no web-platform test is run or changed.
@task_item n=7 part=not_needed
The escher harness's construction, settle, input, inspection and virtual clock contracts: the new test calls the browser's capture function directly, not through the harness.
@task_item n=8 part=not_needed
CI package-install scripts, their exit codes, coverage-floor script exit codes, action pinning, secrets and cache decisions: no CI job, workflow or script is added or changed.
@task_item n=9 part=not_needed
Telemetry, log line and span contracts of the driver and session host: the scope adds no log field, span or log line.
