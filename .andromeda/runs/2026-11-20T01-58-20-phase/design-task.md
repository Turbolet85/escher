@task_prose part=chunk
Moves the stand checks' repeated setup (the boot of a stand task or fixture, the act through a driver session, the read of an answer: refusal, diff, caused, snapshot text) into shared test helpers the checks call, so one copy stands where several do now.
First family folded is the act-error-text check's clone shared with the act-errors and order-served checks; further clone families among the stand checks are chosen from the audit tool's report at HEAD.
Changes test code only and no behaviour: every check keeps its one target a file, its test functions, names, assertions and layout modes.
Re-measures the duplication reading before and after with the code audit's own tool and recipe, against the 0.3.0 readings 3.80 % to 3.81 % to 3.89 %.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of the error text a refused driver act answers with — edits: its repeated boot, act and answer-read setup (the first clone family)
@task_file n=2 path=tests/blitz-tests/tests/stand_act_errors.rs mark=edits
what: the stand check of driver acts refused on a stand task — edits: its repeated boot, act and answer-read setup (clone of the first family)
@task_file n=3 path=tests/blitz-tests/tests/stand_order_served.rs mark=edits
what: the stand check of the order task served through a driver session — edits: its repeated boot, act and answer-read setup (clone of the first family)
@task_file n=4 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the shared test helpers module many blitz test targets include — edits: the new shared stand-check helpers for boot, act and answer read
@task_file n=5 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared helpers the driver-session stand checks include — edits: the driver session act and answer-read helpers, where P3 places them here
@task_file n=6 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared stand fixtures module the checks include — edits: the fixture boot helpers, where P3 places them here
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target pin check, which must still pass with every target unchanged
@task_item n=1 part=needs
The restructure's invariant: the boot of a stand task or fixture, the act through a driver session and the read of an answer (refusal, diff, caused, snapshot text) move from the stand checks into shared helpers, and each stand check keeps the same test functions under the same names, the same assertions on the same readings in the same layout modes; the stand checks themselves, the target pin and the stand-repeat leg hold it.
@task_item n=2 part=needs
Where a moved unit may live: the shared helpers live under the blitz-tests common test directory; which shared module takes which part follows which modules each check family already includes, read at P3.
@task_item n=3 part=needs
The one-target-per-file decision of 2026-10-10 stands: no test file merged, split or renamed, no test target added or removed, the stand-repeat leg's named targets unchanged.
@task_item n=4 part=needs
A helper moves setup and reading code only, never an assertion's subject or bound; no check is weakened, skipped or ignored.
@task_item n=5 part=needs
Security rule over refusals: a refusal holds nothing of the call, so no id, name, value or typed text is newly written into a test name or an assertion message by a helper or a check.
@task_item n=6 part=needs
A shared helper that non-stand targets also include keeps every such target compiling with no new dead-code warning under the clippy leg, following the module's existing dead-code allowance convention where it has one.
@task_item n=7 part=needs
Test code only: no product source, engine, driver, harness or stand application change, no new crate or dependency, the workspace manifest and lockfile unchanged.
@task_item n=8 part=needs
The duplication measure: jscpd 5.4.0 run in Rust format over packages, apps, tests, examples and the WPT runner at default min-tokens 50 with ignore files respected, summarized as the 0.3.0 Epoch 6 audit record does — percent to two places, duplicated and total lines, clones, the src, test and mixed split by a tests directory segment, and the top ten — stated before and after.
@task_item n=9 part=needs
The readings it moves against: 3.80 % then 3.81 % then 3.89 %, test pairs 266 then 291 then 307 with src pairs flat; which earlier reading counts as back under, and whether whole percent or the test split, is settled at P4.
@task_item n=10 part=needs
The founder is away: a fork of his is written to the run's founder-forks record and the least-change option stands meanwhile.
@task_item n=11 part=needs
The codebase-health capability is shared with two other working entries; this chunk advances its duplication leg alone and makes no capability claim at P5.
@task_item n=12 part=needs
The prior wrap commit's CI verdict, in progress at take-up, is re-read before the plan's review.
@task_item n=1 part=not_needed
Color palette rows (engine colours, select list colours, colour scheme recascade, colour tokens): the chunk changes no colour and paints nothing new.
@task_item n=2 part=not_needed
Spacing and depth strategy rows (presentational spacing hints, select list placement, stacking, paint order and hit-testing): engine layout and paint behaviour the chunk does not change.
@task_item n=3 part=not_needed
Motion rows on scrollbar fades, smooth scroll, wheel cancel and select list scrolling or opening: engine animation behaviour no stand-check helper alters.
@task_item n=4 part=not_needed
Iconography rows (cursor icons, cursor resolution, favicon): no cursor or icon behaviour is reached.
@task_item n=5 part=not_needed
The web-spa surface and WASM font context rows: the chunk reaches no WASM page or canvas.
@task_item n=6 part=not_needed
WPT runner output and result colour rows: the WPT runner's terminal output is not touched.
@task_item n=7 part=not_needed
Single select list events and select accessibility rows: engine select behaviour the chunk neither changes nor newly asserts.
@task_item n=8 part=not_needed
Shell provider window controls, JS window dimensions, dioxus-native configuration and MountedData-during-render rows: integration behaviour outside test setup code.
@task_item n=9 part=not_needed
Brand identity, border radius, anti-patterns, self-validation and design decisions log areas: they carry no recorded intent the chunk could reach.
