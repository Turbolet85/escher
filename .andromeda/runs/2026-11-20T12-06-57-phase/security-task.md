@task_prose part=chunk
Ends the four browser mutants the 0.3.0 boundary audit read as missed at the browser's screenshot capture module (render_scene's returned (u32, u32) size replaced by (0, 0), (0, 1), (1, 0), (1, 1)): each is either caught by a browser test or kept by the founder's word.
The likely way is a direct unit test in the capture module's own test module that calls render_scene for two inputs of different viewport size and asserts the returned size; the other way, a fork for the founder, removes the unused return value.
Re-derives the mutants' coordinate at HEAD (render_scene declared at line 81, the mutant span at 86:5 stands) and re-measures that no caller reads the returned size.
Any cargo-mutants run, even of these four alone, is planned as an operator entry or a fork for the founder, never as a gate that fires by itself.
@task_file n=1 path=apps/browser/src/capture.rs mark=edits
what: the browser app's screenshot capture component, which renders a document's scene to pixels and saves a screenshot — edits: its own test module (a new unit test of render_scene), or render_scene's return value, as the fork rules
@task_item n=1 part=needs
render_scene's returned size: a new unit test calls it directly with a document whose viewport is wider and taller than 1 px, and with a second document of another size, and asserts the size it returns follows the input.
@task_item n=2 part=needs
The alternative the founder may choose: render_scene returns nothing, so no value mutants are generated for it; this edits upstream browser app code and adds to the fork's upstream divergence.
@task_item n=3 part=needs
What the browser paints or saves as a screenshot stays as it is: the capture path's pixels, the screenshot encoding and its saved file are untouched by the test or the return-value change.
@task_item n=4 part=needs
The new test builds only what render_scene takes (a document with a set viewport, a paint scene) inside the browser crate under its default features; it adds no dependency, no feature and no change to the dependency set or lockfile.
@task_item n=5 part=needs
The mutation tier runs on the dev host only, never in CI, and only on the founder's word; the plan adds no CI job, no local gate and no script that starts cargo-mutants by itself.
@task_item n=6 part=needs
The chunk adds no new carrier, verb, log field, span, environment read or listener: the new test installs no tracing subscriber, reads no environment variable and opens no socket.
@task_item n=7 part=needs
The new test touches no network: the document it renders is built in memory, with no outbound fetch and no file or remote URL load.
@task_item n=8 part=needs
The new test writes no screenshot or other file to disk; any pixels it renders stay in memory and are never logged.
@task_standing n=1
convention: each new test holds no secret or credential and reads no environment beyond what the test harness already reads
home: secret-management
@task_standing n=2
convention: each new test keeps screen content out of every log line it may cause
home: logging-monitoring
@task_item n=1 part=not_needed
Rows on the driver session socket, its request grammar, replies, refusals and exit codes: the chunk changes no driver verb, socket or session command.
@task_item n=2 part=not_needed
Rows on the MCP server, its tools, results, errors and stdio transport: the chunk touches no MCP surface.
@task_item n=3 part=not_needed
Rows on the stable element id, the snapshot text, the graph and actions answers and accessibility names: the chunk reads no element identity or accessibility tree.
@task_item n=4 part=not_needed
Rows on select lists, range inputs, password masking and other form-control behaviour: the chunk changes no control's input handling or painting.
@task_item n=5 part=not_needed
Rows on recorded checks, kept check files, recording directories and coverage answers: the chunk records and replays nothing.
@task_item n=6 part=not_needed
Rows on cold-agent runs, agent-run and apt-install scripts and the build-directory sweep: the chunk runs none of those tools.
@task_item n=7 part=not_needed
Rows on CI tokens, publish jobs, release signing and upstream-only publishing: the chunk adds no CI job and publishes nothing.
@task_item n=8 part=not_needed
Rows on authentication and authorization: there is none in the app, and the chunk adds none.
@task_item n=9 part=not_needed
Rows on browser history, cookie store, HTTP cache and their storage: the chunk does not touch stored browsing data.
@task_item n=10 part=not_needed
Rows on WPT runner workflows, results and quarantine entries: the chunk runs no WPT test.
