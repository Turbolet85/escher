@task_prose part=chunk
Moves the stand checks' repeated setup — the boot of a stand task or fixture, the act through a driver session, the read of an answer (refusal, diff, caused, snapshot text) — into shared helpers under the blitz-tests shared test modules, so one copy stands where several do now.
First family folded: the 29-line clone shared by the act-error-text, act-errors and order-served stand checks; further clone families among the stand checks and their shared modules as the audit's tool reads them at HEAD.
Test code only: no product, engine, driver, harness or stand change, no new crate or dependency, no test file merged, split or renamed, no test target added or removed.
Re-measures the code duplication before and after with the code audit's own jscpd recipe, against the 3.80 % to 3.81 % to 3.89 % readings whose growth was test-to-test.
Re-reads the blitz-tests target pin and the stand-repeat leg's named targets, both unchanged.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: a stand check of the error text a refused act answers with, one test target — edits: its boot, act and answer-read setup, replaced by calls to the shared helpers
@task_file n=2 path=tests/blitz-tests/tests/stand_act_errors.rs mark=edits
what: a stand check of the refusals an act through a driver session answers, one test target — edits: its boot, act and answer-read setup, replaced by calls to the shared helpers
@task_file n=3 path=tests/blitz-tests/tests/stand_order_served.rs mark=edits
what: a stand check of the order the stand serves its answers in, one test target — edits: its boot, act and answer-read setup, replaced by calls to the shared helpers
@task_file n=4 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the shared module the blitz-tests check targets include, stand and non-stand alike — edits: the new shared stand setup helpers (boot, act, answer read)
@task_file n=5 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared module of driver-session helpers the stand checks include — edits: the session act and answer-read helpers, where a family already includes it
@task_file n=6 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared module of stand fixtures and tasks the stand checks boot — edits: the fixture boot helpers, where a family already includes it
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target pin, the check that the set of test targets is the one pinned
@task_item n=1 part=needs
The restructure's invariant: the boot, act and answer-read setup moved out of the stand checks into shared helpers does what it did — the same stand task or fixture booted, the same act through the driver session, the same refusal, diff, caused and snapshot-text readings handed back — and every stand check keeps the same test functions under the same names, the same assertions on the same readings in the same layout modes, held by those stand checks themselves; none weakened, skipped or ignored.
@task_item n=2 part=needs
The duplication reading is stated before and after by the code audit's own tool and recipe: jscpd 5.4.0 at default min-tokens over packages, apps, tests, examples and the WPT runner, summarized to pct at two places, duplicated and total lines, clones, the src, test and mixed split by a tests directory segment, and the top ten.
@task_item n=3 part=needs
The helpers live in the blitz-tests shared test modules and every check stays one target a file: no test file merged, split or renamed, no test target added or removed, the target pin and the stand-repeat leg's named targets unchanged.
@task_item n=4 part=needs
A helper moves setup and reading code only, never an assertion's subject or bound; the assertion stays in the check that makes it.
@task_item n=5 part=needs
Nothing a check asserts on leaves the repo boundary differently: a helper writes no id, name, value or typed text into a test name or an assertion or panic message — a refusal holds nothing of the call, and a failure says the kind and a count, not the content read.
@task_item n=6 part=needs
The shared helpers and the checks that call them write nothing to stdout or stderr of their own and install no tracing subscriber or sink; a boot helper starts the headless stand as the checks did, with no sink.
@task_item n=7 part=needs
A boot helper that starts a session or stand child passes the same environment the check passed, so a check that reads an empty stderr still runs its child with RUST_LOG unset.
@task_item n=8 part=needs
A helper in the shared module that non-stand targets also include keeps each of those targets compiling with no new dead-code or unused warning under the clippy leg.
@task_item n=9 part=needs
Test code only: no product source, engine, driver, harness or stand change, no new crate or dependency, and the workspace manifest and lockfile unchanged.
@task_standing n=1
convention: every check and shared test module is silent — it prints nothing to stdout or stderr and installs no subscriber
home: 3-observability-harness-contract
@task_standing n=2
convention: every check's failure message names a kind and a count and echoes nothing the caller typed or the answer held
home: 3-observability-harness-contract
@task_standing n=3
convention: committed checks and their helpers hold no private data or secret
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Telemetry strategy rows — which crate carries tracing, engine tracing behind a feature, escher crates taking tracing ungated: the chunk adds no event or span and changes no crate's dependencies.
@task_item n=2 part=not_needed
Metric coverage rows and the absences of metric emission, metrics exporters, telemetry backends and OTel export: the chunk adds no counter, gauge, histogram or exporter.
@task_item n=3 part=not_needed
Error capture and reporting rows — script error source labels, the window error event, window.onerror, WPT report messages, the panic hook and the WASM panic hook: the chunk changes no engine or telemetry error path.
@task_item n=4 part=not_needed
PII scrubbing rows over the product sink and engine — scrubbing by field name, the escher-telemetry scrub set, engine targets' safe fields, select list labels and chosen sets, the mutation buffer: the chunk changes no sink, no engine log site and no answer the product writes.
@task_item n=5 part=not_needed
Log coverage contracts of the product's own output — escher's sink log line, the closed span line, escher-session exit codes, commands, host wire and coverage role: the chunk changes no product output, only how the checks set up the reading of it.
