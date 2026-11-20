@task_prose part=chunk
Moves the stand checks' repeated setup — booting a stand task or fixture, acting through a driver session, reading an answer (refusal, diff, caused, snapshot text) — into shared test helpers the checks call, so one copy stands where several do now.
The first family folded is the act-error clone shared by the act-error-text, act-errors and order-served stand checks; further clone families among the stand checks are folded as the audit's tool reports them at HEAD.
Test code only: every check stays one target a file and proves what it proved; no product, driver, harness or stand source changes.
Re-measures the duplication reading before and after with the code audit's own tool and recipe, against the 3.80 → 3.81 → 3.89 % rise that was test-to-test.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of act-error refusal text through a driver session — edits: its boot, act and answer-read setup, replaced by calls to the shared helpers
@task_file n=2 path=tests/blitz-tests/tests/stand_act_errors.rs mark=edits
what: the stand check of act errors through a driver session — edits: its cloned boot, act and answer-read setup, replaced by calls to the shared helpers
@task_file n=3 path=tests/blitz-tests/tests/stand_order_served.rs mark=edits
what: the stand check of the served order task through a driver session — edits: its cloned boot, act and answer-read setup, replaced by calls to the shared helpers
@task_file n=4 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the shared test-helper module many blitz-tests targets include — edits: the new stand boot, act and answer-read helpers
@task_file n=5 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared driver-session test-helper module the session-driving stand checks include — edits: whichever stand session helpers the families already reach through it
@task_file n=6 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared stand fixture module the checks include — edits: whichever fixture boot helpers the families already reach through it
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target pin — one target a file, its list of targets unchanged
@task_item n=1 part=needs
The restructure's invariant: the stand checks' boot of a stand task or fixture, their act through a driver session and their read of an answer (refusal, diff, caused, snapshot text) move into shared helpers, and every check keeps the same test functions under the same names, the same assertions on the same readings, in the same layout modes; the stand checks themselves and the blitz-tests target pin hold it.
@task_item n=2 part=needs
One target a file stands (the 2026-10-10 decision): no test file merged, split or renamed, no test target added or removed, the stand-repeat leg's named targets unchanged; checks reach the helpers by including the shared test modules, never by becoming one target.
@task_item n=3 part=needs
A helper carries setup and reading code only, never an assertion's subject or bound; no test weakened, skipped or ignored.
@task_item n=4 part=needs
A shared helper that a non-stand target also includes keeps that target compiling with no new dead-code warning under the clippy leg.
@task_item n=5 part=needs
No id, name, value or typed text newly written into a test name or an assertion message: caller input is never written back, and a refusal holds nothing of the call.
@task_item n=6 part=needs
The boot helper boots the stand the way the checks did: the session roles' shared stand boot, a held instance reading as a fresh boot, the headless stand's pinned viewport.
@task_item n=7 part=needs
The act helper reaches stand controls by their stable element ids and the served tasks (the order task among them) as the driver session serves them.
@task_item n=8 part=needs
The answer-read helpers read the escher-session answers the checks assert on — act answers, refusals, snapshot answer and its part, the off-screen and unknown-verb refusals — in the shape the session surface carries them.
@task_item n=9 part=needs
Test code only: no product source, engine, driver, harness or stand source change; no new crate or dependency; the workspace manifest and lockfile unchanged.
@task_item n=10 part=needs
The duplication measure: jscpd 5.4.0 by the audit's recipe (rust format, default min-tokens, ignore file respected, over packages, apps, tests, examples and the wpt runner), summarized as pct to two places, duplicated and total lines, clones, the src / test / mixed split by a tests directory segment, and the top ten — stated before at this chunk's base and after.
@task_item n=11 part=needs
The reading it moves against: 3.80 → 3.81 → 3.89 %, test pairs 266 → 291 → 307 with src pairs flat; the after reading back under an earlier reading, which one and in what form settled at P4.
@task_item n=12 part=needs
The founder is away: a fork of his goes to the run's founder-forks record and the least-change option stands meanwhile.
@task_standing n=1
convention: every stand check addresses a stand control by its stable element id, never by text or position
home: surface-desktop-native
@task_standing n=2
convention: every check of a session refusal or answer holds nothing of the caller's input in its name or message
home: surface-cli
@task_item n=1 part=not_needed
The shell provider's window chrome controls (drag, maximize, minimize, close, decorations): the chunk touches no native window code.
@task_item n=2 part=not_needed
The stand's visual and document decisions — accent colour, fixed document skeleton, arbitrary index templates — the stand source is untouched.
@task_item n=3 part=not_needed
The agent-run and cold-agent pipe command lines and their decisions (pipe builds nothing, prompt names no id, drives through the MCP server): no script or agent pipe changes.
@task_item n=4 part=not_needed
The multi_doc and paint_bench command lines: benchmark tools the stand checks do not reach.
@task_item n=5 part=not_needed
The MCP server's stdio transport, protocol revisions, tools list and result bound: the helpers drive the session as the checks did, not the MCP server.
@task_item n=6 part=not_needed
The session recording contracts and the replayable-recording decision: no check of this chunk records a session.
@task_item n=7 part=not_needed
The mobile-native and web-spa surfaces: not measured and not reached by test-only code.
