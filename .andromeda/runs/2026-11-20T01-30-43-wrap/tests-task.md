@task_prose part=session
A documentation-only session: the public readme now states 0.3.0 closed and adds a 0.4.0 planned section in six stages, one bullet per epoch, after the 0.3.0 section; the 0.1.0 to 0.3.0 sections are unchanged.
The readme's Later line of out-of-version items grew from 3 to 7: the demo project and heavy modern HTML (0.5.0), a GPU-painted canvas, a web Canvas 2D context for page scripts, opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, the next MCP protocol revision.
The stack list re-derived five stale entries to the pins already held: selectors 0.43.0, skrifa 0.49, usvg 0.50.0, svgtypes 0.16.2, and arboard dropped (in no manifest), the windowing row now reading clipboard text through winit.
No code, test, CI workflow, schema, capability statement or its check changed; no dependency moved; nothing retired; the CI-scripts leg ran green (145 tests) and CI read green 17/17.
@task_file n=1 path=README.md
what: the project's public readme: its version status sections (0.1.0, 0.2.0, 0.3.0 closed, 0.4.0 planned in six stages with one bullet per epoch) and the Later line naming what no version yet builds
@task_file n=2 path=.claude/docs/stack.md
what: the stack list, a harness render naming the pinned dependency versions and the role each library plays (windowing, fonts, SVG, CSS selectors, clipboard)
@task_item n=1 part=touches
The capability-statement rows (statement pinned to the driver schema, read from the schema, claims naming their checks, claim names and counts held to the schema): the readme now states what 0.4.0 builds beside the capability statement, which stays unchanged with its check; its 0.4.0 restatement is carried to a later epoch, so a row that says the readme and the statement agree or that the readme carries no planned claim may read stale.
@task_item n=2 part=touches
The verb-count and test-rename sweeps row: the readme's counts moved (epoch bullets 17 to 23, version status headings 3 to 4, Later items 3 to 7) and it still names the fifteen driver commands as tools; a row whose sweep reads a count in the readme is reached.
@task_item n=3 part=touches
The minimum supported Rust version row and any row naming a pinned library version: the stack list was re-derived to selectors 0.43.0, skrifa 0.49, usvg 0.50.0, svgtypes 0.16.2; a row whose cell holds an older version of these is stale.
@task_item n=4 part=touches
Any row naming arboard or a clipboard library: arboard is in no manifest and the clipboard now reads as text through winit.
@task_item n=5 part=touches
Rows on font and SVG painting (pixel tests on a CPU buffer, stand checks on a bundled or pinned real font, font-metric tests, shot PNG determinism) only where a row names the skrifa or usvg version the stack list now states.
@task_item n=6 part=touches
The MCP protocol revision contract items: the readme's Later line now names the next MCP protocol revision as out of version, beside the revision the MCP role serves.
@task_item n=7 part=touches
Telemetry rows (telemetry target allowlist, service.name in stand and session logs): the readme now names opt-in OpenTelemetry export as out of version.
@task_item n=8 part=touches
Accessibility rows (a11y leg membership, harness graph tests turning accessibility on): the readme names the platform accessibility adapter witnessed with a real assistive technology as out of version.
@task_item n=9 part=touches
The windowed end-to-end absence and canvas rows (canvas handlers uncounted by handler coverage): the readme names a GPU-painted canvas and a web Canvas 2D context for page scripts as out of version.
@task_item n=10 part=touches
The CI Python scripts unit-tested row and the CI legs through the leg script: the CI-scripts leg ran green, 145 tests, as a gate of this session; a row holding that test count is reached.
@task_item n=11 part=touches
The build directory held by a sweep row: the operator pass's fast leg ran its stale-hash sweep, which removed 607 units; a row holding a unit count or bound is reached.
@task_item n=1 part=untouched
The session wire, session host, session errors, session state directory and session recording contracts: no protocol, bound or error changed.
@task_item n=2 part=untouched
The driver verbs, refusal causes, driver session and escher-session command-line, exit-code and stderr contracts: no verb, cause, exit code or output line changed.
@task_item n=3 part=untouched
The harness construction, core, input, inspection, settle, screenshot and contrast contracts and the virtual-clock decisions: no harness code changed.
@task_item n=4 part=untouched
The agent-run and cold-agent contracts and decisions: no event, verb, verdict, exit code or bar changed.
@task_item n=5 part=untouched
The WPT runner, status set, pinned commit and report-only decisions: nothing of WPT moved.
@task_item n=6 part=untouched
Coverage floors, mutation testing, complexity ceiling and the zero-failure stand-repeat budget: no gate, floor or budget changed.
@task_item n=7 part=untouched
CI workflow decisions (pinned actions, read-only permissions, no secrets, trigger and cache branches, bounded package installs, fast/slow split): no workflow or CI script changed.
@task_item n=8 part=untouched
Test data, fixtures, mocking and stand-in discipline, lint defects and handler coverage limits: no test or fixture was added or changed.
