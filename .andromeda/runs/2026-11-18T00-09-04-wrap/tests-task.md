@task_prose part=session
The session added tests only: no product code, CI script, agent-run, manifest, dependency or schema changed.
The stand requirement sweep gained a 0.3.0 table of 16 rows (v030-01 to v030-17 bar 15) naming each requirement's witness test, and a test that every witness runs inside the stand-repeat leg; v030-02, v030-03 and v030-16 name no screen witness.
The sweep's leg-membership check now reads the stand-repeat arm of the CI leg script alone, not the whole script; the named stand-repeat targets went from 10 to 17, the arm's words in its order.
A new blitz-tests target checks in both layout modes that graph, actions and layout answers, a kept check and every explored path's check hold a typed password and a file input's path zero times, and that the file input's value reads the mask over MCP.
The browser's document loader gained five unit tests pinning its document config fields (3 to 8 tests).
Measured: agent-run run stand 332 passed and run all 879 passed, 0 failed; stand-repeat 3 of 3 rounds green; coverage floors held, escher-driver 97.68, dioxus-native-dom 90.79, blitz-test-harness 95.70 per cent on CI; three verified ledger refs found naming renamed tests.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: edited — the stand requirement sweep, the blitz-tests target that checks every release requirement names a witness test run inside the stand-repeat leg, with its witness tables and the named stand-repeat targets list
@task_file n=2 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: added — a new blitz-tests stand check that the masks hold on the graph, actions and layout answers, on kept and explored-path recorded checks, and on the file input's value over the MCP role
@task_file n=3 path=apps/browser/src/document_loader.rs
what: edited inside its inline unit test module alone — the browser's document loader, which builds the document config for a loaded page
@task_item n=1 part=touches
The rule that the stand requirement sweep's leg check reads the CI leg script: it now reads the stand-repeat arm alone, a named target counting only where the arm runs it and a seven_guis target only on the arm's seven_guis line.
@task_item n=2 part=touches
The named stand-repeat targets list rows (stand checks repeated three rounds, the stand-repeat leg membership, the named blitz-tests targets): the list grew from 10 to 17 names; any row holding the count ten is stale.
@task_item n=3 part=touches
Capability claims name their checks: every 0.3.0 requirement now names its witness test by a table pinned to the test's current function name, and three verified ledger refs name renamed tests.
@task_item n=4 part=touches
Verb-count and test-rename sweeps: a renamed test is caught by the table's function-name assert; a ledger ref naming a test is no proof the function still bears that name.
@task_item n=5 part=touches
Table-driven tests assert their row count: the new 0.3.0 table holds 16 rows and its test asserts them.
@task_item n=6 part=touches
A typed password never leaves the instance and typed passwords never returned: a new check holds them on graph, actions and layout answers, kept checks and every explored path's check.
@task_item n=7 part=touches
Graph answer holds no value and actions answer holds no value: newly witnessed with a typed password and a file input path present.
@task_item n=8 part=touches
The driver JSON masked value contract item: the file input's value reads the mask over the MCP role, now witnessed.
@task_item n=9 part=touches
Mask on every carrier, the guard of the decision naming the stand mask checks: a new stand check joins stand_act_filled and stand_mcp_masked as its witness.
@task_item n=10 part=touches
Stand checks run in both layout modes and layout tests in both layout modes: the new mask check runs each test in both modes.
@task_item n=11 part=touches
One test target per file, one blitz-tests target per file, and the blitz-tests manifest holding one test target: a new auto-discovered target was added with no manifest change.
@task_item n=12 part=touches
Session checks share one module and shared fixtures in one module: the new target reads the shared session module and declares the shared fixtures module; its masked-inputs fixture page is local with one reader.
@task_item n=13 part=touches
Socket session checks unix-gated and recording checks unix-gated: the new kept-check and explored-path mask tests are gated to unix.
@task_item n=14 part=touches
Session recording exploration checks: every explored path's kept check is read for masked values, with the password typed before exploration.
@task_item n=15 part=touches
MCP tested as a child and in process: the file input mask is read through the in-process MCP serve path.
@task_item n=16 part=touches
Unit tests in inline test modules and unit tests in their source file: the browser's document loader test module grew from 3 to 8 tests.
@task_item n=17 part=touches
Test failures print no argument value and call assertions carry a row index only: the new checks' messages name a layout mode and an index alone, printing nothing of the screen.
@task_item n=18 part=touches
Agent-run selections run stand and run all, and any row holding agent-run Proof readings: run stand now 332 passed, 0 failed, 10 ignored; run all 879 passed, 0 failed, 15 ignored.
@task_item n=19 part=touches
Per-crate coverage floors and the coverage leg gated by the floors: all three crates held their floors; any row holding the coverage figures is restated by escher-driver 97.68, dioxus-native-dom 90.79, blitz-test-harness 95.70 per cent.
@task_item n=20 part=touches
Stand checks repeated three rounds and the zero-failure budget for stand checks: stand-repeat read 3 of 3 rounds green with the new target inside the arm's stand glob.
@task_item n=21 part=touches
Tests controlled red-before-green: each new check and the guard entry carried a control that held.
@task_fact n=1 slug=ci-integration
fact: The stand requirement sweep checks leg membership against the stand-repeat arm of the CI leg script alone: a named target counts only where the arm runs it, a stand file only where the arm holds the stand glob, a seven_guis target only on the arm's seven_guis line, and the named stand-repeat list holds the arm's seventeen words in its order.
@task_fact n=2 slug=test-strategy
fact: Every 0.3.0 requirement names its witness test inside stand-repeat in a table pinned by each test's function name, so a renamed witness reads red; v030-02, v030-03 and v030-16 alone name no witness, having no screen.
@task_fact n=3 slug=integration-test-strategy
fact: The masks are held on every answer carrier in both layout modes: graph, actions and layout answers, a kept check and every explored path's check hold a typed password and a file input's path zero times, the kept type step reads the stand-in mask, and the file input's value reads the mask over MCP.
@task_fact n=4 slug=integration-test-strategy
fact: The mask check states a caused item outside the mask by ratified exception: no served task logs or panics, so no hosted acting call returns a caused item, and the caused reading is the in-process one.
@task_item n=1 part=untouched
WPT conformance, the WPT runner, its CLI, outputs and status set: no WPT test or runner changed.
@task_item n=2 part=untouched
Cold-agent pipe rows, events, verdict, exit codes and isolation: nothing of the cold agent changed.
@task_item n=3 part=untouched
Session wire, session errors, session host, the escher-session command line and exit codes, the MCP role's protocol and tools list: no product code changed, only tests read them.
@task_item n=4 part=untouched
Driver refusal causes, lint defects and findings, snapshot node and budget contracts: none moved.
@task_item n=5 part=untouched
Harness construction, core, input, inspection, settle, contrast and screenshot contracts: the test harness is unchanged.
@task_item n=6 part=untouched
CI workflow rows: actions pinned to SHAs, secrets, permissions, caches, package installs, apt-install and coverage floor script exit codes, CI artifacts, clippy, dependency audit, MSRV: no CI script, workflow or leg changed.
@task_item n=7 part=untouched
Agent-run verbs, events, exit codes, state and invocation: the agent-run script is unchanged; only its counts were read.
@task_item n=8 part=untouched
Mutation testing, the complexity ceiling and the keyboard_compat coverage exclusion: not run or changed.
@task_item n=9 part=untouched
Mocking discipline, virtual clock, advance and timer rows: no stand-in, clock or timer changed.
@task_item n=10 part=untouched
The Coverage report job's attempt-1 red on a corrupt profile, green on re-run: no test-plan row; a recurrence watch outside the master.
