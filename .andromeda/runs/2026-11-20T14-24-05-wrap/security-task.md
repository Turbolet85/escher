@task_prose part=session
The session added test code only: two unit tests in the browser app's capture module, in a cfg(test) child module whose tests sit out of line in a sibling file with the _tests suffix.
The tests check that the private scene renderer returns the viewport size (800 x 600 at scale 1.0) and the physical size at a HiDPI scale (1280 x 720 at scale 2.0).
They catch the four surviving render_scene mutants of the 0.3.0 Epoch 6 mutation audit, under hand controls and under a cargo-mutants run fired on the founder's word, on the dev host with the nextest mutants profile.
No public, crate-visible or IPC surface, dependency, manifest, config, schema, CI or gate surface changed; no capability was added or removed and nothing was retired.
A cargo-mutants run found one mutant outside the founder's word that no test catches (the capture_anyrender_scene function replaced with an empty body). It is carried as a residual for the next version, with no work done on it.
CI ran green on the pre-CI commit (17 of 17 checks), and the locked fast leg ran green locally before the push.
@task_file n=1 path=apps/browser/src/capture.rs
what: the browser app's screen capture module, which renders the document's scene at the viewport size and HiDPI scale for a screenshot; edited, with only a cfg(test) test-module declaration appended
@task_file n=2 path=apps/browser/src/capture_tests.rs
what: the browser app's capture unit tests, test-only code that checks the scene renderer's returned viewport and physical sizes; new file
@task_item n=1 part=touches
Rules over screenshot capture and in-band screen pixels (the screenshot carried in band, the screen pixels in band, the screenshot answer as png, png as the shot encoder): the capture component they rule over gained tests only, with no behaviour change, so they are re-read and left standing.
@task_item n=2 part=touches
Locked builds: the new tests and the mutation runs all ran with --locked, and no lockfile or manifest moved, so the rule holds as it stands.
@task_item n=3 part=touches
Workspace-inherited dependency versions, engine dependency pins and no-git-dependency rules: the tests use only existing browser dependencies (blitz-html, blitz-dom, blitz-traits, anyrender at the pinned 0.15.0), and none was added.
@task_item n=4 part=touches
Per-crate line coverage floors: the browser crate gained two unit tests, which can only raise its line coverage. This touches a row only where a cell holds the browser crate's floor or its measured value.
@task_item n=5 part=touches
Dependency audit CI gate, licence gate, the CI matrix that runs every job, and running the audit leg locally before a push: CI read green 17 of 17 on the pushed commit and the locked fast leg ran locally first; no gate's threshold or job changed.
@task_item n=6 part=touches
Test harness rules (the test harness taking no tracing and no style): the new tests drive the browser's scene renderer with a null scene painter and add no tracing or style dependency.
@task_item n=1 part=untouched
Driver session socket, MCP server and escher-session CLI rows (verbs, flags, exit codes, request and reply grammar, bounds, idle expiry): no wire, flag, verb or reply changed.
@task_item n=2 part=untouched
Stable element id, snapshot text, graph and actions answers, masked values and control-state rows: no answer shape or masking rule changed.
@task_item n=3 part=untouched
Input validation of page content (HTML, CSS, select lists, range inputs, focus, accessibility names): no engine or widget behaviour changed.
@task_item n=4 part=untouched
Outbound networking, script fetch, CORS, same-origin, TLS, cookies, HTTP cache and history storage rows: nothing in networking or storage moved.
@task_item n=5 part=untouched
Secret management, CI secrets, signing material and publish-job guards: no workflow, secret or release step changed.
@task_item n=6 part=untouched
Logging and telemetry sinks, span lines and log-target allowlists: no log, span or sink changed.
@task_item n=7 part=untouched
Error handling, refusal causes and session error tables: no error path or refusal was added or changed.
@task_item n=8 part=untouched
Cold-agent, recorded-check, kept-check and coverage-answer rows: no agent run, recording or kept check was touched.
