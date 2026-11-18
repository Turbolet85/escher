@task_prose part=session
The session is a test-only sweep: it proves every 0.3.0 requirement headless on the stand or a fixture, inside the stand-repeat leg, with no engine, driver, app or CI change.
The stand-requirement sweep gained a 0.3.0 requirement table pinning each of seventeen requirements to a witness test the stand-repeat arm runs (three requirements name no screen witness), and its membership check now reads the stand-repeat arm of the CI leg runner alone; the named stand-repeat targets went from ten to seventeen.
A new blitz-tests target proves password and file input values stay masked in the graph, actions and layout answers, in a kept check and its replay, in every explored path's check, and over MCP, where the file input's value reads the mask; it runs in both layout modes and prints nothing.
The browser's document loader gained five unit tests that each check one field of the document configuration it builds (root shell provider, HTML parser provider, font context, passed abort signal, parallel style); the configuration builder itself is unchanged.
Agent-run counts and coverage figures were re-read; all three per-crate coverage floors held, stand-repeat read 3 of 3 rounds green, and CI went green on its second attempt after a corrupt coverage profile at merge on the first.
Nothing was retired, and no dependency, schema, public symbol, verb, carrier, port or env var was added.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the stand requirement sweep, a blitz-tests integration target that checks every release requirement names a witness test run inside the stand-repeat leg; edited to add the 0.3.0 table and read the stand-repeat arm of the CI leg runner
@task_file n=2 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: a new blitz-tests stand check target proving masked input values (password and file) stay masked on the graph, actions and layout answers, kept checks, explored paths' checks and the MCP role; it has its own local single-reader fixture page
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the reference browser's document loader, which builds the document configuration for each load (shell provider, HTML parser, font context, abort signal, style threading); edited only in its unit-test module
@task_item n=1 part=touches
The masked input values decision (password and file input values masked) and its guard: the guard gains the new mask stand check target, and the file input's masked value is now proven over the MCP role too.
@task_item n=2 part=touches
The kept check masking rule and the kept check file's masked item: a kept check recorded with a typed password holds no masked value and replays green, proven in both layout modes.
@task_item n=3 part=touches
The graph, actions and layout answer crossings (UI graph, actions and layout answers cross the carriers; the graph verb carries no values): each answer is now proven to hold no masked value.
@task_item n=4 part=touches
The explore capability: every explored path's kept check is now proven to hold no masked value when the session starts with a password already typed.
@task_item n=5 part=touches
The acting verb results' caused item: the mask check treats a caused item as outside the mask by the ratified exception, and no hosted acting call returns one because no served task logs or panics.
@task_item n=6 part=touches
The stand-repeat zero-failure budget decisions and the stand-repeat CI leg contract: the requirement sweep now reads that leg's arm as the source of which named targets run, and the named targets went from ten to seventeen; any row holding the count of named targets is stale.
@task_item n=7 part=touches
Integration tests live in blitz-tests and one test target per file: the session adds one new blitz-tests target, auto-discovered with no manifest change.
@task_item n=8 part=touches
The single-reader fixture page stays local convention and the shared fixtures and stand-check shared helpers conventions: the new target keeps its one fixture page local and reads the shared session and fixtures modules.
@task_item n=9 part=touches
Scenarios in both layout modes: every new mask test runs in both the incremental and full layout modes.
@task_item n=10 part=touches
Driver logs no content, no echo of typed input, and raw run output never printed: the new mask check prints nothing and its messages name only a mode and an index.
@task_item n=11 part=touches
The browser document load decisions (a browser document load aborts the previous load; windowed documents style in parallel) and the document configuration contract: the browser's loader now has unit tests pinning that it passes the abort signal and runs style in parallel, along with the shell provider, HTML parser and font context.
@task_item n=12 part=touches
Unit-test placement and unit tests at the end of the source file: the browser loader's five new tests sit in its existing in-file test module.
@task_item n=13 part=touches
Per-crate coverage floors: the floors held at their values; CI read escher-driver 97.68 percent, dioxus-native-dom 90.79 percent and blitz-test-harness 95.70 percent, with no floor moved.
@task_item n=14 part=touches
The CI coverage report and its failure-log artifact: the first CI attempt failed when llvm-profdata merge rejected a corrupt profile after every test passed; an identical re-run was green, no cause was measured, and it is recorded as a candidate watch.
@task_fact n=1 slug=standard-contracts
fact: Every release requirement in a table of the stand requirement sweep names a witness test that the stand-repeat leg's arm runs, or is listed as having no screen witness; witness names are pinned to the current test functions, so a renamed test turns the sweep red.
@task_fact n=2 slug=standard-contracts
fact: Masked password and file input values are proven to stay masked on every 0.3.0 answer: graph, actions and layout answers, kept checks and their replay, each explored path's check, and the file input's value over the MCP role.
@task_item n=1 part=untouched
Engine style, layout, paint, text, scrolling, hit testing and the select list: no engine crate changed outside tests.
@task_item n=2 part=untouched
The driver's verbs, refusal causes, answer JSON shapes, the session socket wire, the MCP server protocol and the command line: no verb, argument, carrier or wire changed.
@task_item n=3 part=untouched
The accessibility tree, ARIA states and the platform adapter: not reached.
@task_item n=4 part=untouched
Telemetry bootstrap, scrubbing, span and line contracts: no telemetry change.
@task_item n=5 part=untouched
The WPT runner, its outputs, result codes and the WPT CI diff tooling: not reached.
@task_item n=6 part=untouched
Stack and dependency decisions, workspace metadata, versioning, publishing and the Nix flake: no dependency, crate or manifest changed.
@task_item n=7 part=untouched
CI workflow structure, the CI leg runner's legs and exit codes, the agent-run and cold-agent script contracts: no script or workflow changed; agent-run counts moved but no row holds them.
@task_item n=8 part=untouched
Names: no binary, crate, env var, server or other name was taken or freed.
@task_item n=9 part=untouched
Browser history, persistence, about pages and the browser chrome UI: only the browser's document loading tests moved.
@task_item n=10 part=untouched
Upstream sync and cherry-pick decisions, the capability statement and the root README: not reached.
