@task_prose part=chunk
Moves the stand checks' repeated setup (the boot of a stand task or fixture, the act through a driver session, the read of an answer: refusal, diff, caused, snapshot text) into shared test helpers the checks call, so one copy stands where several do now.
The first family folded is the stand act-error-text clone also cloned in the stand act-errors and order-served checks; further families are the clone pairs the audit tool reads among the stand checks and their shared modules.
Every check keeps one target a file and proves what it proved: same test functions, names, assertions, readings and layout modes.
Re-measures the code duplication reading before and after with the code audit's own tool and recipe, against the 0.3.0 readings 3.80 -> 3.81 -> 3.89 %.
Test code only: no product source, engine, driver, harness or stand change, no new crate or dependency, manifests unchanged.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of refusal error text acted through a driver session — edits: its repeated boot, act and answer-read setup, first family
@task_file n=2 path=tests/blitz-tests/tests/stand_act_errors.rs mark=edits
what: the stand check of acting-call refusals through a driver session — edits: its cloned boot, act and answer-read setup
@task_file n=3 path=tests/blitz-tests/tests/stand_order_served.rs mark=edits
what: the stand check of the served order task acted through a driver session — edits: its cloned boot, act and answer-read setup
@task_file n=4 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the shared test module of the blitz-tests checks, no target of its own, included by many targets — edits: the shared stand helpers for boot, act and answer read
@task_file n=5 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the one shared module the session checks share, no target of its own — edits: shared driver-session act and answer-read helpers, where P3 reads they belong
@task_file n=6 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the one shared fixtures module, the fixture pages stated once, no target of its own — edits: shared stand fixture boot helpers, where P3 reads they belong
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target pin check, which pins the blitz-tests targets one a file and must read unchanged
@task_item n=1 part=needs
The restructure's invariant: the stand checks' boot of a stand task or fixture, act through a driver session and read of an answer (refusal, diff, caused, snapshot text) move from each stand check into shared helpers, and each check keeps the same test functions under the same names, the same assertions on the same readings in the same layout modes; the existing stand checks themselves, run in the fast test leg and the stand-repeat leg, hold it.
@task_item n=2 part=needs
One test target per file stands: no blitz-tests test file merged, split or renamed, no test target added or removed, the blitz-tests manifest's targets and the stand-repeat leg's named targets unchanged.
@task_item n=3 part=needs
Where a shared helper may live: under the blitz-tests shared test modules, which are no target; session-check helpers in the one module the session checks share, fixture helpers in the one shared fixtures module, stand tables in the one shared stand-tables module.
@task_item n=4 part=needs
A helper moves setup and reading code only, never an assertion's subject or bound; no test is weakened, skipped or ignored, and no expected value moves out of the check that asserts it.
@task_item n=5 part=needs
Nothing a check asserts on leaves the repo boundary differently: no id, name, value or typed text newly written into a test name or an assertion message; refusals hold nothing of the call, test failures print no argument value, call assertions carry a row index only, a typed password never leaves.
@task_item n=6 part=needs
A shared helper that a non-stand target also includes keeps that target compiling with no new dead-code warning under the clippy leg over the workspace.
@task_item n=7 part=needs
The duplication reading stated before and after by the code audit's own jscpd recipe and summary (pct to 2 places, duplicated and total lines, clones, the src / test / mixed split, the top ten), read against the audit's metric rows: 3.80, 3.81, 3.89 % with test pairs 266, 291, 307 and src pairs 217, 223, 223.
@task_item n=8 part=needs
Stand checks run in both layout modes: a shared boot or act helper keeps each check exercised in every layout mode it ran in.
@task_item n=9 part=needs
Stand checks run under the bundled pinned real font with no skip: a shared boot helper keeps the bundled font and adds no font-absent skip path.
@task_item n=10 part=needs
Stand checks hold a zero-failure budget over three stand-repeat rounds: the folded checks still pass every round.
@task_item n=11 part=needs
Stand timers and time in the folded checks stay driven by delivered ticks and virtual time through advance; a shared helper adds no sleep or wall-clock read.
@task_item n=12 part=needs
Test code only: no product source, engine, driver, harness or stand change, no new crate or dependency, the workspace manifest and lockfile unchanged.
@task_standing n=1
convention: a new shared test helper module is included by the checks as a module and is never a test target of its own
home: test-strategy
@task_standing n=2
convention: test data a helper builds is stated in code, nothing seeded, and fixture pages are stated once in the shared fixtures module
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: a helper's failure output prints no argument value and holds no id, name or typed text
home: unit-test-strategy
@task_item n=1 part=not_needed
WPT conformance runner rows (its CLI, outputs, status set, pinned commit, harness timeout, Thai font): the chunk touches no WPT suite.
@task_item n=2 part=not_needed
Cold-agent pipe and agent-run rows (events, verbs, exit codes, verdict, selections, state): the chunk changes neither script nor their checks.
@task_item n=3 part=not_needed
escher-session command-line, MCP role and exit-code contracts, and the session wire, socket and state-directory contracts: the chunk changes no session binary or protocol, only how in-process stand checks are set up.
@task_item n=4 part=not_needed
CI package-install, action-pinning, cache, trigger-branch, secrets, permissions and artifact rows: the chunk adds or edits no CI job or workflow.
@task_item n=5 part=not_needed
Coverage floors per crate, mutation testing and mutation baseline rows: the chunk adds no test whose mutation control is owed and moves no product line.
@task_item n=6 part=not_needed
Lint defect and lint finding contracts, text contrast reader and WCAG ratio rows: no lint or contrast code is touched.
@task_item n=7 part=not_needed
Pixel, screenshot PNG and box-shadow expectation rows: the folded stand checks' readings are text answers, and no screenshot or paint expectation moves.
@task_item n=8 part=not_needed
Upstream buffered-render, mounted-flush and Dioxus bridge rows: the chunk touches no bridge test.
@task_item n=9 part=not_needed
Recorded-check freshness and byte-equal recording rows, handler coverage limits: the chunk records and replays no committed check.
