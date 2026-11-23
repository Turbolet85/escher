@task_prose part=session
The session added one stand check, stand_canvas_shot, that proves a widget painted from Rust appears as pixels in a harness shot and in a driver shot (the whole shot and the element crop), in both layout modes, per package and in the workspace build.
It turned on blitz-paint's custom-widget feature in the headless test harness, so blitz-paint's widget pass is compiled in every per-package selection that reaches the harness (blitz-test-harness, escher-driver, seven_guis).
It added peniko as a blitz-tests dev-dependency: one lock line and no new package. No public symbol, driver verb, argument, result field, refusal cause, session error, exit code, log field or span was added, and no source under packages changed.
It measured the check red before the fix and green after, with a control that holds. A failure message now carries the layout mode, the row and the pixel RGBA it read. CI#45125989655 on the pre-CI commit was green, 17 of 17 checks.
Retired: nothing.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_shot.rs
what: the new stand check (a stand_* integration test target of blitz-tests, read by stand-repeat and the run stand selection) that is the witness that an app widget painted from Rust shows in a harness shot and in a driver shot whole and cropped, run in both layout modes; it declares the shared session test module and decodes the PNG of a shot to read pixels
@task_file n=2 path=packages/blitz-test-harness/Cargo.toml
what: the manifest of the dedicated headless test harness, holding its dependencies and feature set; its blitz-paint dependency now names custom-widget
@task_file n=3 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests manifest, holding its test targets and dev-dependencies (its difference from upstream); it gains peniko as a dev-dependency
@task_file n=4 path=Cargo.lock
what: the workspace lock file that every locked cargo leg resolves against; one line added, peniko in the blitz-tests entry, no package added
@task_item n=1 part=touches
Rows that say the stand checks run in both layout modes (stand checks in both layout modes, layout assertions in both layout modes): the new canvas shot check runs both modes and belongs beside them.
@task_item n=2 part=touches
Rows on stand-repeat (stand checks repeated three rounds, zero-failure budget for stand checks, every requirement witnessed inside stand-repeat, a file joining a leg keeps its legs): one more stand_* target joins stand-repeat through the existing glob. Any row cell that states the count of stand_* targets is now one higher.
@task_item n=3 part=touches
The agent-run stand selection contracts (agent-run run selection stand, agent-run selections run stand): one more stand_* target is selected. Any cell that holds the count of stand readers moves by one.
@task_item n=4 part=touches
One test target per file or per behaviour (one blitz-tests target per file, one test target per file, one integration test target per behaviour, blitz-tests manifest holds one test target): the session adds one new blitz-tests target file for one behaviour.
@task_item n=5 part=touches
The blitz-tests manifest's difference from upstream: the manifest gains a peniko dev-dependency.
@task_item n=6 part=touches
Shared session module rows (session checks share one module, shared test modules are no target): the new check declares the shared session module, so any stated count of its declarers moves by one.
@task_item n=7 part=touches
Headless screenshot from the harness and the harness screenshot contracts (screenshot, screenshot_rect): a harness shot now paints an app's custom widget in every build selection, not only where the workspace unified the feature.
@task_item n=8 part=touches
Driver shot rows (screenshot pixels in band, driver session screenshot, escher_driver Session screenshot): a driver shot now carries a widget's pixels, whole and element-cropped. This is checked in process only, with no new field or carrier.
@task_item n=9 part=touches
PNG decoding in checks and pixel tests on a CPU buffer: the new check decodes the shot PNG and reads RGBA pixels at stated points.
@task_item n=10 part=touches
Failure-message conventions (test failures print no argument value, call assertions carry a row index only, no ids or names in stand logs): the new check's failure message carries the layout mode, the row and the pixel RGBA it read. That RGBA is a measured value, not page text or a control value. This is a possible exception.
@task_item n=11 part=touches
No screenshot step in committed checks, and no shot in cross-OS byte comparisons: the new check reads shots inside a Rust stand test. It is not a committed recorded check and does not compare bytes across OSes. A picker should confirm that these rows still hold.
@task_item n=12 part=touches
Tests controlled red-before-green: the check was fired red before the fix (the inside pixel read white), and its step control holds on the harness case.
@task_item n=13 part=touches
Stand checks under a bundled font with no skip: the new stand check runs on the stand's pinned font like its siblings.
@task_item n=14 part=touches
Per-package green is not workspace green, and workspace tested in the fast CI leg: the harness feature now holds in the per-package selections (the gate reads 3), and the check is green both per package and in the fast workspace leg.
@task_item n=15 part=touches
Cargo legs run locked, and the dependency audit: the lock gained one line, every gate ran with locked resolution, and the audit stayed green.
@task_item n=16 part=touches
Dedicated headless test harness and harness-driven engine tests: the harness's feature set changed, so blitz-paint's widget pass is now part of every harness build.
@task_item n=17 part=touches
Select list painted by the scene painter alone: a custom widget is now painted by blitz-paint's widget pass inside a shot, a second paint path that sits beside it.
@task_item n=18 part=touches
Test data is code and the stand boots offline: the painted widget and its component are defined in code inside the check, with no fixture file and no network.
@task_fact n=1 slug=test-strategy
fact: The canvas shot stand check is the witness that a widget painted from Rust appears in a harness shot and in a driver shot (whole and element-cropped), in both layout modes, per package and in the workspace build.
@task_fact n=2 slug=ci-integration
fact: The headless test harness turns on blitz-paint's custom-widget feature itself, so a shot paints an app's widget in every per-package selection that reaches the harness, not only in the workspace build.
@task_fact n=3 slug=session-lifecycle
fact: An app's canvas content crosses to a caller only as pixels in a screenshot. It is checked in process only, with no new carrier, field, exit, cause, session error, log field or span. No served app holds a canvas yet.
@task_fact n=4 slug=unit-test-strategy
fact: A stand check's failure message may carry the RGBA pixel it read, a measured number, besides the layout mode and the row. It carries no page text or control value.
@task_item n=1 part=untouched
Handler coverage rows about canvas handlers (canvas handlers not counted, handler coverage limits on canvas handlers): these are Circle Drawer's canvas handlers, another sense of canvas. Handler coverage did not change.
@task_item n=2 part=untouched
Driver refusal causes, session errors, session wire, exit codes or status, and the command span: no cause, error, wire form, exit or span was added or moved.
@task_item n=3 part=untouched
MCP role, escher-session command line and CLI contracts (tools/list, help, snapshot, actions, graph, record): no verb, argument or result field changed.
@task_item n=4 part=untouched
Cold-agent rows (events, verdict, isolation, live run, wrong-call bar): no cold-agent run or rule moved.
@task_item n=5 part=untouched
WPT rows (runner CLI, outputs, status set, pinned commit, report-only conformance): no WPT change.
@task_item n=6 part=untouched
Session recording, committed recorded checks and the recording script: nothing was recorded or replayed anew.
@task_item n=7 part=untouched
CI workflow setup (actions pinned to SHAs, permissions, secrets, caches, trigger branches, apt-install script, CI leg script exit codes, artifacts): no script, leg or job changed.
@task_item n=8 part=untouched
Coverage floors, mutation baseline, complexity ceiling and oversize-file rows: no product source changed and no floor or threshold moved.
@task_item n=9 part=untouched
The windowed end-to-end absence and the mocking-library absence: both still hold, because the check is headless and uses no mock.
@task_item n=10 part=untouched
Virtual clock, advance, settle and timer rows: the check drives no time.
