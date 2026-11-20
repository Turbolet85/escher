@task_prose part=chunk
Moves the stand checks' repeated setup — booting a stand task or fixture, acting through a driver session, reading an answer (refusal, diff, caused, snapshot text) — into shared test helpers the checks call, so one copy stands where several do now.
First family is the act-error clone shared by three stand checks; further clone families among the stand checks and their shared test modules follow as the duplication tool reads them.
Test code only: no product, engine, driver, harness or stand change, no new crate or dependency, every check one target a file and proving what it proved.
Re-measures the duplication reading before and after by the code audit's own jscpd recipe, and re-reads the last wrap commit's CI verdict before the plan's review.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of the act refusal error texts read through a driver session — edits: the repeated boot, act and answer-read setup of its first clone family
@task_file n=2 path=tests/blitz-tests/tests/stand_act_errors.rs mark=edits
what: the stand check of act refusals through a driver session — edits: its clone of the boot, act and answer-read setup
@task_file n=3 path=tests/blitz-tests/tests/stand_order_served.rs mark=edits
what: the stand check of the served order reading through a driver session — edits: its clone of the boot, act and answer-read setup
@task_file n=4 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the shared test helper module many blitz-tests targets include — edits: the new shared boot, act and answer-read helpers
@task_file n=5 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared driver-session test helper module the session and stand checks include — edits: the shared session act and answer-read helpers, where a family already includes it
@task_file n=6 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared stand fixture module of the blitz-tests checks — edits: the shared fixture boot helpers, where a family already includes it
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target pin check — one target a file, the target list unchanged
@task_item n=1 part=needs
The restructure's invariant: the stand checks' boot of a stand task or fixture, their act through a driver session, and their read of an answer (refusal, diff, caused, snapshot text) move into shared helpers; every check keeps the same test functions under the same names, the same assertions on the same readings in the same layout modes, no assertion's subject or bound moved into a helper, no test weakened, skipped or ignored — held by the stand checks themselves running unchanged.
@task_item n=2 part=needs
A refusal holds nothing of the call: a helper that reads a refusal answer, or builds a test name or assertion message from a diff, caused, snapshot-text or refusal reading, writes no id, name, value or typed text into it that was not there before.
@task_item n=3 part=needs
Caller input is never echoed: shared helpers that act through a driver session (type, key, click with ids and text) pass the call content to the session only and print or format none of it into output a check leaves behind.
@task_item n=4 part=needs
Masked control values stay masked in what a helper reads: snapshot text and session replies for password and file inputs carry the masked stand-in, and a helper reading them asserts on nothing past it.
@task_item n=5 part=needs
One target a file: no test file merged, split or renamed, no test target added or removed; the blitz-tests target pin and the stand-repeat leg's named targets stay as they are.
@task_item n=6 part=needs
No new crate or dependency, manifest and lockfile unchanged: the blitz-tests crate stays unpublished, its test harness takes no tracing and no style dependency, and builds stay locked.
@task_item n=7 part=needs
A shared helper added to a module that non-stand targets also include keeps every including target compiling with no new dead-code warning under the clippy leg, by the module's existing dead-code allowance convention.
@task_item n=8 part=needs
The measure the chunk is read by: the code audit's jscpd recipe (pinned node and jscpd version, rust format, default min-tokens, gitignore respected) summarized as pct to two places, duplicated and total lines, clones, the src/test/mixed split and the top ten — stated before and after against the readings 3.80, 3.81 and 3.89 percent and test pairs 266, 291, 307.
@task_item n=9 part=needs
Helpers that boot a driver session take the session state directory as a parameter, keep it owner-only and within its path-length bound, and clean up only their own files non-recursively, as the moved setup did.
@task_item n=10 part=needs
Helpers reading answers through the session socket keep to its closed request grammar (hello, call, stop) and its reply forms (accepted, refused, oversize, refused malformed or version) as the moved setup did, without new verbs or fields.
@task_standing n=1
convention: every shared test helper reaching a driver session uses an owner-only state directory passed as a parameter and removes only its own files
home: input-validation
@task_standing n=2
convention: every test in the blitz-tests crate keeps the crate unpublished and takes no tracing or style dependency
home: dependency-security
@task_standing n=3
convention: every check's refusal or session-error assertion reads the fixed cause, fault, meaning and remedy fields, never call content
home: error-handling
@task_item n=1 part=not_needed
The browser application's stored data — history database, cookie store, HTTP cache, TLS, clipboard and file dialogs — is product code the chunk does not reach.
@task_item n=2 part=not_needed
Outbound network rules — fetch methods, per-host limits, CORS and same-origin absences, request timeouts, response size limits, the opt-in CDP devtools server — are untouched: the checks run local stands with no network.
@task_item n=3 part=not_needed
CI secrets, publishing, signing material and the release guards are not reached: no workflow, job or secret changes.
@task_item n=4 part=not_needed
The WPT runner, its quarantine entries and its post-results workflow are outside the chunk.
@task_item n=5 part=not_needed
Engine parsing and painting rules — CSS and HTML parsing, select list, range, password painting, iframe and import nesting, font and SVG sub-resources — bind the product code, which the chunk leaves as it is.
@task_item n=6 part=not_needed
The cold-agent run, its recordings, transcripts and wrong-call bar, and the kept-check and recorded-check file formats, are not touched: the chunk folds setup in committed stand checks only.
@task_item n=7 part=not_needed
The MCP server's stdio transport, protocol error codes and tool result bounds are not reached by moving stand-check setup.
