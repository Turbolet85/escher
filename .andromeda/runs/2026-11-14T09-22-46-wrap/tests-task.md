@task_prose part=session
The session added two stand checks and changed no production code, manifest, CI step, committed check or dependency.
One stand check reads the six served tasks (counter, flight-booker, timer, crud, cells, load) in both layout modes, at boot and after one acting step by click, type or advance. It compares each reading's driver answer digest (snapshot of every part, lint, actions) to one committed 48-row table, with no shot.
CI read the one table the same on linux x86_64, windows x86_64 and macOS aarch64. Before this, only the two committed Counter checks were read across the three OSes.
The other stand check reads seven font families (sans-serif, serif, monospace, system-ui, an unavailable named family, cursive, fantasy) on the stand's bundled DejaVu Sans in both layout modes. Every family reads the sans-serif text extent.
Measured: the families read the bundled font by fallback to the collection's only font, not by registration. Removing one generic family's registration is a vacuous control there. The red control is a font context holding other fonts.
fonts-unavailable is kept unchanged, nothing is deferred, and nothing is retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the stand check of cross-OS readings: a table-driven integration test target that compares the six served tasks' driver answer digests to one committed table, in both layout modes, with no shot
@task_file n=2 path=tests/blitz-tests/tests/stand_bundled_font_families.rs
what: the stand check of the generic font families on the bundled font: an integration test target over a shared fixture page, asserting each family's text extent equals sans-serif's, in both layout modes
@task_item n=1 part=touches
decision/no-shot-in-cross-os-byte-checks: cross-OS readings are now also compared by driver answer digests in a stand check, not in a kept check file, and that check holds no shot. The rule takes the new cross-OS readings check as a guard.
@task_item n=2 part=touches
decision/cross-os-checks-hold-no-shot: a second statement of the same rule. The new cross-OS digest comparison holds no shot, so the rule stays true and it may take the same guard.
@task_item n=3 part=touches
decision/no-screenshot-in-cross-os-checks and decision/no-screenshot-in-recorded-check-scripts: these are about the committed recorded checks. The cross-OS reading now reaches all six served tasks through a stand check, not only the Counter's committed checks. Check whether any wording says the committed checks are the only cross-OS reading.
@task_item n=4 part=touches
decision/stand-checks-bundled-font: gains the families check as a guard. The generic families are read on the bundled font.
@task_item n=5 part=touches
decision/stand-checks-bundled-font-no-skip, decision/stand-pinned-real-font and decision/font-metric-tests-never-skip: both new stand checks run on the pinned bundled font with no skip, and the families check asserts a non-zero extent first.
@task_item n=6 part=touches
decision/font-assertions-on-system-fonts: its exception (stand checks measure the same on any runner) is now measured on three OSes by the cross-OS readings check. It may take that guard.
@task_item n=7 part=touches
decision/native-platform-ci-matrix: the windows and macOS test jobs now read the six served tasks' digest table, not only the Counter.
@task_item n=8 part=touches
decision/stand-repeat-three-rounds, decision/zero-failure-stand-repeat and decision/stand-checks-zero-failure-budget: the two new stand checks join the three repeated rounds through the stand prefix, and each round was green. The pinned count of named targets is unmoved.
@task_item n=9 part=touches
decision/one-test-target-per-file, decision/blitz-tests-one-target-per-file and decision/one-test-target-per-behaviour: two new auto-discovered integration test targets, one behaviour each. There was no manifest edit, so decision/blitz-tests-single-test-target and decision/blitz-tests-manifest-difference hold.
@task_item n=10 part=touches
decision/stand-checks-in-both-layout-modes and decision/layout-tests-in-both-layout-modes: both new checks read each case with incremental layout off and on.
@task_item n=11 part=touches
decision/table-tests-count-their-rows: the cross-OS readings table has 48 rows, and its first test asserts the row count and that the six served tasks are covered.
@task_item n=12 part=touches
decision/call-assertions-carry-row-index and decision/assertions-print-no-argument-value: a table mismatch fails with the row's fixed label only (layout mode, task, moment, verb, part). It prints no read value.
@task_item n=13 part=touches
contract/stand-boot/boot-cells, contract/stand-boot/boot-load and contract/stand-boot/options: they gain the cross-OS readings check as a caller, which holds Cells and the load page through them. Their contract is unchanged.
@task_item n=14 part=touches
decision/session-checks-share-one-module, decision/shared-fixtures-in-one-module, decision/shared-fixture-pages and decision/shared-test-modules-not-targets: the new checks reuse the shared session hold and the shared fixture boot, and add no new shared module.
@task_item n=15 part=touches
decision/test-data-in-code and decision/test-data-is-code: the expected digests are 16-hex literals in the check's own table, measured once on the dev host. Nothing is seeded.
@task_item n=16 part=touches
decision/no-sleep-or-wall-clock-in-checks, decision/no-wall-time-in-session-checks and decision/one-virtual-clock: the acting step moves time by advance on the virtual clock alone, never by press or wall time.
@task_item n=17 part=touches
decision/no-ids-or-names-in-stand-logs and decision/session-writes-no-screen-content: both new checks are silent (the print census reads 0).
@task_item n=18 part=touches
decision/mutation-controlled-tests: the families check's planned control (one generic's registration removed) did not hold, because a one-font collection reaches its font for any generic. The red control used instead switches the stand to the host's system fonts. This is an exception to how a red-before-green control is chosen.
@task_item n=19 part=touches
decision/joining-file-keeps-its-legs and decision/workspace-tests-in-fast-ci-leg: the new files run in the test leg, the matrix test jobs and stand-repeat through the workspace build. The fast leg was green.
@task_item n=20 part=touches
decision/driver-instance-verbs-tested-by-stand and contract/driver-session/run: the cross-OS check reads snapshot (every part), lint and actions through the session run path and the driver's answer digest. These are new integration readings of the verbs, with the contract unchanged.
@task_fact n=1 slug=test-strategy
fact: The six served tasks (counter, flight-booker, timer, crud, cells, load) are read on every CI OS (linux x86_64, windows x86_64, macOS aarch64) by one stand check. It compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file.
@task_fact n=2 slug=test-strategy
fact: In both layout modes, every generic font family (sans-serif, serif, monospace, system-ui, cursive, fantasy) and an unavailable named family read the bundled font's sans-serif text extent in a stand check. The reason is fallback to the font context's only font, not a registration per family.
@task_fact n=3 slug=unit-test-strategy
fact: On a font context that holds one font, a control that removes one generic family's registration is vacuous, because the fallback reaches that font anyway. The red control for a font-family reading is a font context that holds other fonts.
@task_item n=1 part=untouched
WPT runner, conformance, reftest and pinned-commit rows: no WPT input or runner changed.
@task_item n=2 part=untouched
Cold-agent and agent-run contracts, events, verdicts, exit codes and selections: no harness script or verdict shape changed.
@task_item n=3 part=untouched
Session wire, session host, session errors, session state directory and recording contracts, and the escher-session command line, MCP role and exit codes: no production code changed.
@task_item n=4 part=untouched
CI workflow rows (actions pinned to SHAs, caches, triggers, package installs, secrets, permissions, artifacts, clippy, dependency audit, MSRV): no workflow, CI script or manifest changed.
@task_item n=5 part=untouched
Coverage floors, the coverage leg, mutation runs and the complexity ceiling: no floor, threshold or gate value moved.
@task_item n=6 part=untouched
Handler coverage rows and the canvas handler limit: no coverage role or listener walk changed.
@task_item n=7 part=untouched
The fonts-unavailable refusal and shot-without-fonts rows: kept unchanged, and their checks are green as before. Canvas text stays a later reader of the same font context, with no canvas renderer added.
@task_item n=8 part=untouched
Committed recorded check files and their recording script, byte-equality and private-data rows: no committed check changed.
@task_item n=9 part=untouched
Lint defect, snapshot node, contrast and harness input or inspection contracts: read by the new check, not changed.
@task_item n=10 part=untouched
The absence rows for windowed end-to-end tests and a mocking library: the session adds neither.
