# Task — test-plan · 2026-10-27-readme-and-records

## The session
A documents-and-records chunk. No source, test, CI script, workflow, manifest or lock file changed.
README.md now shows 0.1.0 and 0.2.0 as done and verified and 0.3.0 as planned in six stages. It names 0.2.0's additions and the six task slugs, and its Try it block gains `escher-session lint`. Its pinned phrase about the twelve driver commands as tools moved from README.md:48 to README.md:49.
docs/capabilities.md was restated in three places. The lint at boot now reads 0 findings for the counter, the timer and CRUD, and one `overlap` for the flight booker's `flight-type` select's two options. The host-sink part read is now measured. The run paragraph now describes the 2026-10-25 cold-agent live run, with 66 calls, 0 wrong, the 5 % bar, six apps met and `claude` 2.1.308.
Records were updated: the 0.3.0 ledger rows v030-02 (claimed, gated on the founder's word on F3) and v030-16 (by-construction, implemented); the working-route BLOCKED-ON line; and the master-route pending record. The local `.git/info/exclude` incubator lines were retired, and `.gitignore:39` still ignores the incubator.
Measured: the capability-statement pin was green (4 tests); `ci-leg.sh fast` was green (CI scripts 108 tests) and `doc` was green; CI#40456250076 on 642561729e7c read green, 17/17 checks, wall 1145 s. The smoke run was skipped.

## Reached files
- README.md
- docs/capabilities.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- escher-0.3.0/chunks/2026-10-27-readme-and-records/scope.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/research.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/plan.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/inputs/andromeda-inputs.json
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/records.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/counts.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/operator-pass.md
- .andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh
- .git/info/exclude

## Touches
- §1 CI Python scripts: the capability statement's pin over docs/capabilities.md and README.md was read again and stays green (4 tests). Its count of the driver commands as tools (twelve) and the statement's five counts did not move. The claim stands.
- Any test-plan citation of README.md:48 for the pinned phrase about the twelve driver commands as tools, or for the `docs/capabilities.md` link, now points to README.md:49. The citation sweep found standing sites in architecture only.
- Any test-plan citation of docs/capabilities.md line numbers in or after :108-110, :225-226 or :290-295 may have shifted. Those ranges were restated.
- §3 Cold-agent run pipe, its Proof bullet (the 2026-10-25 live run: passed, 66 tool calls, 0 wrong against the 5 % bar, six apps met, lint 1, screenshot 1, `claude` 2.1.308): the capability statement now states this same run. The proof's figures do not move.
- §3 Drivers per surface, the MCP bullet (a real client in the operator pass, the cold-agent live run over the six served apps under `--record`): it was read again and does not move.
- The host-sink part read (`serve cells` at `trace`, 0 id and 0 name needles under both builds): test-plan.md:268 and integration-test-strategy.md:23 already state it as measured. No change.
- The lint at boot: test-scope-summary.md:124 already states the flight booker's one `overlap` (`flight-type-one-way` × `flight-type-return`) through `stand_lint::the_stand_at_boot_reads_its_measured_findings` (tests/blitz-tests/tests/stand_lint.rs:282-289). No change.
- The stand's accent quoted at test-scope-summary.md:116 and the select's text pair at test-scope-summary.md:115 are contract or test quotations. No change, though design-system's stand rows are leaving it.
- Indirect, through design-system's stand rows moving out (F2, PROVISIONAL): accent, success, shadow, spacing and radius move to layout-templates §Surface: desktop-native → Primary screens, and the bundled DejaVu Sans to architecture §Standard Contracts → Headless stand. A test-plan citation of design-system §Color Palette, §Typography, §Depth Strategy, §Spacing or §Border Radius for a stand value would re-point there.
- The stand's bundled DejaVu Sans with system fonts off (§2 Font-dependent tests, §8 Real dependencies kept, §9 Fonts): these state a test condition, not a design-system row. Their values do not change.
- The `fast` leg reading: green at 108.42 s and 111.06 s on the operator pass, with CI scripts at 108 tests, OK. Check it against any stated count of the CI Python tests or of the `ci-scripts` leg.
- The CI reading on 642561729e7c: CI#40456250076 green, 17/17 checks, wall 1145 s. Check it against any stated count of ci.yml's checks or any stated wall time.
- The `doc` leg read green. The rustdoc gate itself did not move.
- Test inventories (stand checks, session-common readers, fixtures readers, the per-crate unit-test lists, the target pin): no test file was added, removed or renamed in code, so every inventory and count stands.
- The agent-run Proof counts (`run stand` 205 · 0 · 7 over 50 files, as at 2026-10-25-cold-agent-bar) were not read again and are not moved by this chunk.
- The two closed-version ledger refs (escher-0.1.0 v010-17, escher-0.2.0 v020-05) will be re-pointed at P7.3 to the three renamed test names. A test-plan line that quotes those acceptances, the retired pressed pair (`flight-one-way` · `flight-return`) or a pre-rename test name would be stale. The `old-names` pattern found no master site.
- `gate.py hygiene` P1 refused a planted `/home/` literal in a phase run-dir control script. The script was respelled to `/home/<user>/x` and hygiene was run again clean. The test-plan rule that committed transcripts are host-path-masked under hygiene P1 is unmoved.
- The incubator ignore: `.gitignore:39` holds it before and after, and the `.git/info/exclude` lines were retired. The incubator control found no test-plan site.
- Coverage floors, the stand-repeat zero-failure budget, the mutation tier and the complexity ceiling: no source or test changed, so none of their readings moved.
- The smoke run was skipped because no boot path or UI surface changed. No smoke procedure moved.

## New facts
- (none)

## Untouched
- §3 Test Harness Contract's `Harness` API (construction, time, pump, settle, screenshot, contrast, input and inspection helpers). No harness code changed.
- The agent-run contract (invocation, exit grammar, events, state) and its contract tests. `scripts/agent-run.sh` and `test_agent_run.py` are unchanged.
- The cold-agent pipe's invocation, server, judge, verdict fields, events and state. `scripts/cold-agent.sh` and `test_cold_agent.py` are unchanged. Only the capability statement's prose about the run changed.
- The stand log format and the needle readings of the host, the `mcp` role and `stand_act_spans`. No telemetry or driver code changed.
- The §8 network and time stand-ins (`HeldNet`, `LateNet`, `LoadNet`, `TimerTicks`) and the §7 fixtures (`tests/fixtures/mod.rs`, the committed Counter checks under `tests/blitz-tests/checks/`). Both are unchanged.
- §9 CI pipeline structure: legs, the fast/slow split, cache, failure logs, package installs, engine features by runner and the per-OS tallies. ci.yml and ci-leg.sh are unchanged.
- §10 coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %), the mutation tier and the cognitive-complexity ceiling of 15. The last two are 2026-10-26-codebase-health's, not this chunk's.
- WPT runner, WPT scoring and WPT workflow statements. Upstream-only and untouched.
- Browser app, blitz-vibey-script, blitz-paint and blitz-dom unit-test inventories. No code in those crates changed.
- The a11y-leg checks (`stand_keyboard`, `stand_contrast` tables). The stand's colours keep their values; only design-system's record of them moves.
