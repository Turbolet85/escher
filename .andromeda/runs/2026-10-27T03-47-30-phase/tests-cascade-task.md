# Task — tests · 2026-10-27-readme-and-records

## The chunk
A documentation and records chunk with no code, test, engine, driver, CI or schema change: no verb, field, carrier, log site, span, check or count moves.
It rewrites `README.md` to show 0.1.0 and 0.2.0 done and verified (what 0.2.0 can do, its cold-agent result) and 0.3.0 planned in its six stages, the project named «Escher» in prose while crate, binary and path names stay lowercase.
It restates the capabilities statement's measured-instance paragraph to the 2026-10-25 cold-agent live run: six apps, twelve tools, 66 calls, 0 wrong, a screenshot and a lint called.
It re-points two verified ledger refs to the tests' current names, moves four design-system stand rows into architecture and layout-templates as an expected wrap amendment, and settles the incubator ignore to the repository's own `.gitignore`.
It re-reads the capability statement's pin (kept green) and the two renamed test functions where they stand; it adds and runs no new check.

## Reached files
- README.md · edits §Status (0.2.0 done and verified, 0.3.0 planned with its stages), the "Later:" line, "What Escher is", the driver paragraph and "Try it"
- docs/capabilities.md · edits the measured-instance paragraph of the cold-agent run (lines 288-291) and any other line naming the 2026-10-15 run as the latest
- .github/scripts/test_capability_statement.py · re-reads the capability statement's pin (verb and cause names as code spans, the five counts in words, README's count of the driver commands as tools)
- escher-0.1.0/verification-matrix.json · edits the v010-17 verified ref
- escher-0.2.0/verification-matrix.json · edits the v020-05 verified ref
- tests/blitz-tests/tests/stand_snapshot_state.rs · re-reads the test `exactly_the_chosen_flight_type_reads_selected` standing under that name
- tests/blitz-tests/tests/stand_lint.rs · re-reads the test `the_stand_at_boot_reads_its_measured_findings` standing under that name
- .andromeda/design-system.md · edits the accent row, the success row, the cards item and the stand's DejaVu Sans item (moved out, as a wrap amendment)
- .andromeda/architecture.md · edits the section each moved stand row lands in (as a wrap amendment)
- .andromeda/layout-templates.md · edits the section each moved stand row lands in (as a wrap amendment)
- .gitignore · re-reads the incubator ignore line 39, witnessed by `git check-ignore -v`
- .git/info/exclude · edits its two trailing incubator lines (local, untracked)

## Needs
- The capability statement's pin, `test_capability_statement.py` in the fast `ci-scripts` leg: every verb and refusal cause named as a code span in `docs/capabilities.md`, the verb, cause, session-error, node-key and exit-status counts stated in words, and README.md's prose holding the phrase that counts the twelve driver commands as tools — the rewritten README and statement must keep all of it green.
- The README's MCP tool count follows the driver schema's `VERBS` (twelve) through that pin; the chunk moves no count.
- The cold-agent live run's recorded proof (test-plan §3 Cold-agent run pipe, Proof): `passed`, `isolated` with the twelve `mcp__escher__*` tools, six apps met, 66 tool calls and 0 wrong against the 5 % bar, `counts_agree`, one lint, one screenshot, one advance, two part reads, six recorded sessions kept — the figures the statement and README state must be read from that record and the run's report.
- The cold-agent pipe's state rule: the transcript, `check.json`, `server.log` and the `record/` kept checks are never printed, copied or committed — the README and statement cite verdict counts, booleans and closed words only, no transcript, reply or check text.
- The run's crossing to the model provider (ids, names and values in snapshot texts, paged parts and diffs, a lint's findings and one screenshot) is stated as the record holds it, nothing wider.
- The renamed stand checks: `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected` and `stand_lint::the_stand_at_boot_reads_its_measured_findings` must stand in their files under those names before the ledger refs name them.
- What `stand_lint` pins at boot (the counter, the timer, CRUD and the load page 0 findings, the flight booker one `overlap`) bounds any README sentence about the lint on the stand.
- The verified ledger refs of a closed version's verification matrix are written through the ledger tool, not by hand; the refs name a test target and function as the matrix records them.
- The stand's bundled DejaVu Sans with system fonts off (the headless-stand checks' font, no skip path) is the fact the moved DejaVu Sans item carries; its new home must state it as the test plan does.
- The accent row's measured contrast (white on `#3a5ce5`, 5.47) and the success row are the colours `stand_contrast`'s `TEXT_ROWS` and `STATE_ROWS` assert; the move changes no colour, so that record does not move.
- A screenshot claim in the README states a shot as byte-identical across processes on one host only; a shot's PNG bytes differ between linux and macOS.
- The recorded-check replay: a session recorded as a replayable check, the two committed Counter checks under `tests/blitz-tests/checks/` replayed by `stand_replay` and kept fresh by seven_guis `recording` — the README names it only as the statement states it.
- The command-line and MCP forms the README's "Try it" shows (`escher-session` verbs, `--session`, `start {task} [--record]`, the `mcp` role, `cargo build -p seven_guis --bin escher-session`) must match what `cli_commands`, `cli_flow` and `mcp_server` prove.
- The six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`) are what the stand boots and the cold-agent run drives; the README names no app the stand does not serve.
- The done gate: `bash .github/scripts/ci-leg.sh fast` (fmt · clippy · workspace tests · CI scripts) and `bash .github/scripts/ci-leg.sh doc` pass on the changed tree.
- The incubator ignore's acceptance is witnessed by `git check-ignore -v` naming `.gitignore:39`, with the `escher-0.2.0-incubator/` line (`.gitignore:37`) untouched.

## Standing conventions
- (none)

## Not needed
- The coverage floors, the workspace line coverage reading and `coverage_floor.py`: no crate's source changes.
- The mutation tier and the cognitive-complexity ceiling: no function is written or changed.
- The flakiness budget and the `stand-repeat` leg's membership: no stand check, named target or process check is added, renamed or removed by this chunk.
- The agent-run contract and its `run stand` reader count: no `stand_*` file or shared reader changes.
- The blitz-tests manifest pin (`BlitzTestsTargetsTest`) and the one-target-per-file layout: no test target is added.
- The cold-agent pipe's contract tests (`test_cold_agent.py`), its verdict fields and its exit grammar: the pipe is not changed and the live run is not re-fired.
- The incremental oracle and the `for incremental in [false, true]` rule: no pipeline test is written.
- The headless `Harness` API, settle semantics, screenshot and contrast readers: used by no new code here.
- The WPT runner, its scoring and its upstream-only workflows.
- The browser app's, blitz-vibey-script's and blitz-dom's unit tests.
- The CI workflow invariants (`test_ci_workflows.py`), caches, package installs and the per-OS tallies: ci.yml is not touched.
- Test fixtures, seeds and the held network stand-ins (`LoadNet`, `HeldNet`).
- Mocking and stubbing discipline: no fake or stub is written.
- The telemetry capture checks and the scrub sets: no log line or span is added.
