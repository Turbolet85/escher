@task_prose part=chunk
Moves the stand checks' repeated setup (booting a stand task or fixture, acting through a driver session, and reading an answer: refusal, diff, caused, snapshot text) into shared helpers under tests/blitz-tests/tests/common, so one copy replaces several. The checks call those helpers.
The first family to fold is the 29-line clone in the stand act-error-text check, also found in the stand act-errors and stand order-served checks. Later families are chosen from the audit tool's clone report at HEAD.
Changes test code only. Every check stays one target per file and proves what it proved before.
Re-measures the duplication reading with the code audit's own jscpd recipe before and after, against the 0.3.0 readings 3.80 % -> 3.81 % -> 3.89 % (test pairs 266 -> 291 -> 307).
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: blitz-tests stand check, one test target, for the text of refusals and errors that acting on the stand returns — edits: the repeated boot, act and answer-read setup (the first clone family) that is replaced by calls to the shared helpers
@task_file n=2 path=tests/blitz-tests/tests/stand_act_errors.rs mark=edits
what: blitz-tests stand check, one test target, for errors when acting on the stand through a driver session — edits: the cloned boot, act and answer-read setup that is replaced by calls to the shared helpers
@task_file n=3 path=tests/blitz-tests/tests/stand_order_served.rs mark=edits
what: blitz-tests stand check, one test target, for the order in which the stand serves acts — edits: the cloned boot, act and answer-read setup that is replaced by calls to the shared helpers
@task_file n=4 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: shared blitz-tests test module (not a target of its own) that many test targets include; the home of the new stand-check helpers — edits: the added boot, act and answer-read helpers
@task_file n=5 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: shared blitz-tests module (not a target of its own) holding the driver-session setup that session and stand checks include — edits: the session helpers the stand checks share, if P3 places part of the helpers here
@task_file n=6 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: shared blitz-tests fixtures module (not a target of its own) that states fixtures once — edits: the fixture-boot helpers, if P3 places part of the helpers here
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target pin, the CI script test that pins the list of blitz-tests test targets; re-run to show that no target was added, removed or renamed
@task_item n=1 part=needs
Restructure invariant. The repeated setup units move into shared helpers: booting a stand task or fixture, acting through a driver session, and reading an answer (refusal, diff, caused, snapshot text). Each stand check keeps the same test functions under the same names, the same assertions on the same readings, and the same layout modes. The existing stand checks hold this, run both directly and by the stand-repeat leg's named targets.
@task_item n=2 part=needs
One target per file in blitz-tests (the 2026-10-10 decision stands). No test file is merged, split or renamed, and no test target entry is added or removed. The blitz-tests target pin and the stand-repeat leg's named targets stay unchanged.
@task_item n=3 part=needs
The shared helpers live in non-target shared test modules in subdirectories of the blitz-tests tests directory. Stand-check helpers belong in non-target modules, and shared fixtures are stated once. Each check reaches a helper by including the module, not as a target of its own.
@task_item n=4 part=needs
A helper moves only setup and reading code. It never moves the subject or the bound of an assertion. No test is weakened, skipped or marked ignored. Where a fixture's precondition is asserted first, it stays first in the check.
@task_item n=5 part=needs
A refusal holds nothing of the call. No id, name, value or typed text is newly written into a test name or an assertion message, so nothing a check asserts on leaves the repo boundary differently.
@task_item n=6 part=needs
Shared modules included by several targets. A helper that one target includes but does not use must raise no new dead-code warning under the clippy leg, and every non-stand target that includes the module must still compile. Lint allows stay local to the item that needs them.
@task_item n=7 part=needs
The duplication measure is the code audit's own tool and recipe: jscpd 5.4.0 run through mise with node 26.8.2, Rust format, default 50 min-tokens, .gitignore respected, over packages, apps, tests, examples and the WPT runner. It is summarised as pct to two places, duplicated and total lines, clones, the src / test / mixed split by a tests directory segment, and the top ten. The reading is stated before and after.
@task_item n=8 part=needs
The target the reading moves against is the earlier audit reading (3.80 % or 3.81 %, as whole pct or as the test split's pairs and lines, which P4 settles). The growth to reverse is test-to-test clone pairs.
@task_item n=9 part=needs
Test code only. No product source, engine, driver, test harness or stand changes. No new crate or dependency, and the workspace manifest and lockfile stay unchanged.
@task_item n=10 part=needs
Stand checks run their scenarios in both layout modes. A shared boot or act helper must keep each check running in every layout mode it ran in before.
@task_item n=11 part=needs
Where moved setup opens a telemetry capture, the capture still opens before the check's own recorder. The helper keeps that order.
@task_item n=12 part=needs
The founder is away. A fork that needs his call is written to founder-forks.md in the run directory, and the least-change option stands meanwhile.
@task_standing n=1
convention: Each new shared test module and helper carries module and item docs that state what it is for
home: conventions
@task_standing n=2
convention: Each expect or unwrap in new helper code carries a justification
home: conventions
@task_standing n=3
convention: An assertion or row-reading helper that panics on a check's behalf is marked track-caller, so a failure points at the calling check
home: conventions
@task_standing n=4
convention: New test code passes the formatting and clippy gate, with any clippy allow kept local to its item
home: conventions
@task_standing n=5
convention: Test names stay descriptive, and integration test files keep one behavior per file
home: conventions
@task_item n=1 part=not_needed
Engine layout, style, paint, scrolling, hit-testing and select-list decisions and contracts. The chunk changes no engine code, and the checks' readings of these stay as they are.
@task_item n=2 part=not_needed
The WPT runner, its outputs, result kinds and statuses, and the WPT diff-to-PR script. The chunk does not touch WPT.
@task_item n=3 part=not_needed
The browser app, its history store, about pages, persistence and publish workflow. No browser code is reached.
@task_item n=4 part=not_needed
The MCP role, MCP server, cold-agent and agent-run scripts with their verdict fields and exit codes. The chunk adds no carrier, verb or script.
@task_item n=5 part=not_needed
Name registers for crates, binaries, servers and environment variables. The chunk adds no crate, binary or variable.
@task_item n=6 part=not_needed
Stack choices of crates and tools (Stylo, Taffy, Parley, Boa, Dioxus, wgpu and the like). No dependency is added or changed.
@task_item n=7 part=not_needed
Occupied resources: listeners, the recording directory, the session state directory, allocators. The chunk opens no new resource.
@task_item n=8 part=not_needed
Upstream sync, merge-base and cherry-pick decisions. This chunk takes nothing from another tree.
@task_item n=9 part=not_needed
Telemetry bootstrap, scrub and log-line contracts. The chunk adds no log line and changes no subscriber.
