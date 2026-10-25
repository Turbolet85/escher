# Task — obs-plan · 2026-10-25-capability-statement

## The session
- Added `docs/capabilities.md`, a 304-line markdown statement of what escher 0.2.0 can do, its stated limits, its accessibility reach, what an MCP client carries onward and where each claim is proven. It adds no runtime surface.
- Added `.github/scripts/test_capability_statement.py`, a CI-script pin of four tests. It checks that the statement names every verb and every refusal cause as a code span, states the verb, cause, session-error, node-key and exit-status counts in words as read from the schema's tables, and that README.md says «the twelve driver commands as tools». It reads files by path from the repository root, writes nothing and reads no env var.
- Replaced one line of `README.md` (line 48): eleven driver commands as MCP tools became twelve, and the line now links `docs/capabilities.md`.
- No Rust symbol, verb, argument, field, cause, session error, exit, carrier, log field or span was added or changed (`guard: clean`). No crate, dependency, schema, config, leg, job or workflow changed either.
- Measured: the `ci-scripts` leg now runs 104 tests over eight test files (it was 100 over seven). The workspace tally is unchanged at 206 lines · 1105 passed · 0 failed · 12 ignored. CI#40184674449 is green, 17/17.
- The statement restates escher's observability surface: the two spans `command` and `tool_call` with closed-word, bool and count fields, stderr only, no driver line at the default `warn`. It also names the scrub's limits: the raw panic message, `log.file` on a bridged `js_console` record, the dropped third-party WARN or ERROR, and the unscrubbed upstream apps and WPT runner.

## Reached files
- docs/capabilities.md
- .github/scripts/test_capability_statement.py
- README.md
- ../viola/viola-0.2.0-incubator/research/escher-capabilities.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/counts.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/pin-controls.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/viola-pointer.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/operator-pass.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/operator-confirm.md

## Touches
- Any count of the CI-script tests or test files (`ci-scripts` leg, `python3 -m unittest discover -s .github/scripts`): now 104 tests in eight files, up from 100 in seven. A dated measurement that names 100 stays as it is.
- Any inventory of the files under `.github/scripts/`, or of the tests CI's «Test CI scripts» job runs: `test_capability_statement.py` joins them. No leg, job, workflow or `ci-leg.sh` line changed.
- Any silence census or silence group over test files (log-coverage's absence group): the new pin is a test file that reads by path, writes nothing and reads no env var. It adds no log site, span, field, log field or env read.
- The env-read set stated for source and tests: unchanged. The pin reads no env var.
- The escher-driver span set (`command` with its eight fields, `tool_call` with `tool` and `ending`): unchanged. The statement restates it and agrees with it.
- The default `warn` level and the stderr-only sink: unchanged. The statement restates both.
- The scrub's stated limits under PII scrubbing (the raw panic message past the chained hook, `log.file` on a bridged `js_console` record, the dropped third-party WARN or ERROR, the unscrubbed upstream `fmt::init()` apps and WPT `env_logger`): unchanged, and now restated in `docs/capabilities.md`.
- Any count of the MCP tools or driver verbs (for example in the cold-agent pipe text or the escher-driver emitter): `VERBS` holds twelve. The statement says the 2026-10-15 cold-agent run was at eleven tools and has not been re-run since `lint` was added.
- Any count of refusal causes: eight (`CAUSES: [Cause; 8]`). The masters' «nine» is disproved by measurement.
- What an MCP result carries, as against what a log carries: a result carries ids, names, masked values, snapshot text and diffs, a shot as an `image` block, and a lint's findings. Text that names what crosses over MCP stdio should not drop diffs or lint findings. No log line or span field carries any of it.
- Any owner disposition of the windowed assistive-technology witness, where obs-plan names one: its residuals line now reads `re-carried:0.2.0`, no longer «(target: next)».
- Any directory inventory that names `docs/`: escher's own `capabilities.md` now stands beside upstream's three files.
- Workspace test tallies (206 · 1105 · 0 · 12): re-read and unmoved. No Rust test was added.
- CI artifact handling: no artifact was added or changed, and no new file under `target/` or uploaded by a leg.
- Committed-text hygiene: the committed statement and pin hold no credential, token, auth header or host home path (entry 4: 0). This bears on any rule about what committed files may carry.

## New facts
- F1 · log-coverage · The capability-statement pin `.github/scripts/test_capability_statement.py` is silent: it reads `docs/capabilities.md`, `README.md` and the escher-driver schema, refusal and error sources by path, writes nothing and reads no env var. Its four tests add no log site, span, field, log field or env read.
- F2 · log-coverage · `docs/capabilities.md` is the published statement of escher's observability reach: the `command` and `tool_call` spans, stderr only, no driver line at the default `warn`, and the scrub's four limits. The pin checks none of these observability claims, so a change to the span set or the scrub's limits must also be made in that document by hand.

## Untouched
- Engine and upstream log sites and the values they log as-is (blitz-dom, blitz-net, blitz-html, dioxus-native, the browser app): no engine or app code changed.
- escher-telemetry's subscriber, line format, `EnvFilter` default, target allowlist and scrub sets: the crate is unchanged. The statement only restates them.
- Panic hooks and error-capture classes, the wpt runner's hook included: nothing changed.
- Metric coverage, in-process timings, `debug_timer` and the log-times features: nothing changed.
- Feature wiring of the `tracing` cargo features across crates: no manifest changed.
- The agent-run harness log and the cold-agent pipe log, with their event formats and state areas: the scripts are unchanged, and the cold-agent run was not re-run.
- Session recording, kept checks and the two committed Counter checks: no recording code or check file changed.
- The standing silence lines for earlier chunks' stand and engine checks: no test target under `tests/` or `examples/` was added or edited.
- The hygiene respell of a phase run-dir control record: an operator-pass fix to a run dir, outside the masters' scope, with no telemetry bearing.
- Publish builds, the WPT score publishing and the apt-install diagnostic: no workflow or CI script other than the new test changed.
