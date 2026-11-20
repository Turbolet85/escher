@task_prose part=session
Documentation-only session: the front page now states 0.3.0 closed and adds a section for 0.4.0, planned in six stages, one bullet per epoch named as the 0.4.0 route names them.
The front page's Later line was restated from 3 items to 7: the demo project and heavy modern HTML (0.5.0), a GPU-painted canvas, a web Canvas 2D context for page scripts, opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, and the next MCP protocol revision.
The 0.1.0, 0.2.0 and 0.3.0 sections of the front page were left as they were; the count of driver commands an MCP client gets as tools still reads fifteen.
The stack document's five versions were re-derived from the existing pins: selectors 0.43.0, skrifa 0.49, usvg 0.50.0, svgtypes 0.16.2; arboard, named in no manifest and absent from the lockfile, was dropped and the windowing row now says clipboard text goes through winit.
No code, crate, dependency, schema, contract, gate or CI leg changed; the capability statement and its test are unchanged; the founder's F1 and F2 stay unanswered and are carried onto the capability statement's 0.4.0 reach.
@task_file n=1 path=README.md
what: the root README, escher's own front page: what each version closed or plans, the planned 0.4.0 stages, the Later line of out-of-version items, and the count of driver commands an MCP client gets as tools
@task_file n=2 path=.claude/docs/stack.md
what: the stack and technologies statement, a harness render landed through the record tool: the workspace's crates and their pinned versions, the windowing row, the toolchain
@task_item n=1 part=touches
The decisions that the root README is escher's own front page (readme-is-escher-own, readme-is-escher-page): the page now carries a 0.4.0 planned section in six stages and a restated Later line; check the rows still describe its shape.
@task_item n=2 part=touches
The root README contract on the count of driver commands an MCP client gets as tools: the count was not moved (still fifteen) but the page it lives on was edited.
@task_item n=3 part=touches
The capability statement decision and its sections contract, indirectly: the front page's 0.4.0 done and its reach are carried onto the capability statement for Epoch 6, with F1 and F2 unanswered; the statement itself did not change.
@task_item n=4 part=touches
The versioning decisions that main is a prepatch one minor ahead (main-is-prepatch, prepatch-version-ahead): the front page now names 0.3.0 closed and 0.4.0 planned.
@task_item n=5 part=touches
The Stack and Technologies overview and its stack decisions: the stack statement re-derived its versions from the pins; a row naming a stale version of selectors, skrifa, usvg or svgtypes is now stale.
@task_item n=6 part=touches
skrifa follows parley (skrifa-matches-parley): the stack statement now reads skrifa 0.49.
@task_item n=7 part=touches
svgtypes version follows usvg (svgtypes-matches-usvg) and usvg for SVG parsing (usvg-svg-parsing): the stack statement now reads usvg 0.50.0 and svgtypes 0.16.2.
@task_item n=8 part=touches
winit as the window shell (winit-window-shell): the windowing row no longer names arboard; clipboard text goes through winit.
@task_item n=9 part=touches
Current MCP revision not taken up (mcp-current-revision-not-taken) and the MCP server protocol revisions contract: the front page's Later line now names the next MCP protocol revision as out of version.
@task_item n=10 part=touches
Driver graph holds no platform adapter (driver-builds-no-platform-adapter) and the accessibility adapter decisions: the Later line names the platform accessibility adapter witnessed with a real assistive technology as out of version.
@task_item n=11 part=touches
The telemetry decisions (escher-telemetry-sink, telemetry-closed-span-lines): the Later line names opt-in OpenTelemetry export as out of version.
@task_item n=12 part=touches
The scripting and painting decisions a GPU canvas or page-script canvas would reach (wgpu-gpu-layer, single-frame-scripting, boa-for-javascript): the Later line names a GPU-painted canvas and a web Canvas 2D context for page scripts as out of version.
@task_item n=13 part=touches
Dioxus 0.8 move held (dioxus-0-8-move-held) and the cold-agent result stated as measured: any row that says what the front page claims for a version may need to match the restated sections.
@task_fact n=1 slug=project-intent
fact: The front page states each version's status in its own section: closed versions stay as written, the next version is a planned section with one bullet per stage named as its route names them, and a Later line lists the user-facing items out of that version.
@task_fact n=2 slug=stack-and-technologies
fact: No clipboard crate is taken: clipboard text goes through winit, and arboard is in no manifest and not in the lockfile.
@task_item n=1 part=untouched
Engine behaviour rows under Established Decisions: style, layout, paint, hit testing, scrolling, select lists, forms and events; the session changed no code.
@task_item n=2 part=untouched
The driver's carriers and their contracts: verbs, answers, refusal causes, the session socket's wire, the MCP server's methods and bounds, kept checks and recordings.
@task_item n=3 part=untouched
CI and gate rows: ci-leg legs, coverage floors, cargo-deny, fmt and clippy, the mutation tier, the stand-repeat budget and the build-directory sweep; none moved.
@task_item n=4 part=untouched
Occupied Resources and names: no binary, crate, environment variable, listener or directory was taken or freed.
@task_item n=5 part=untouched
CSS and web-platform absences (shadow DOM, container queries, subgrid, 3D transforms and the like) and the WPT runner's contracts.
@task_item n=6 part=untouched
Telemetry contracts (line format, scrub, init, panic hook) and the accessibility tree contracts; the Later line only names future items, it changes no behaviour.
@task_item n=7 part=untouched
Upstream sync decisions and Inherited Defaults; no merge, cherry-pick or rebase was made.
