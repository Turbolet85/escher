@task_prose part=session
The session re-stated the capability statement for 0.3.0: its version, three new sections with their witness lines, the stated limits, the a11y reach and the claim map, with the stale layout-carrier line retired from Not built.
It re-measured from CI#44225111039: the a11y leg runs eighteen files reading 18 · 163 · 0 · 0, eleven of them also in stand-repeat; stand-repeat runs 64 stand files and seventeen named blitz-tests targets reading 273 · 1467 · 0 · 30.
It states the requirement witness tables as WITNESSES_020 nine rows, 49 witnesses, and WITNESSES_030 sixteen rows, 74 witnesses; the earlier 46 was stale at HEAD.
It added a CI-script test pinning the a11y leg's file count (18) and stand-repeat's named-target count (17), read from the CI leg script, as literals and as words on the capability statement, so a change of either leg's membership reddens the ci-scripts leg until the page is re-stated.
The README's 0.3.0 status now reads built, its cold-agent run to come, with 16 of 17 capabilities verified headlessly; the dependency audit's ignore reasons now name residual rows as owners, the six advisory IDs unchanged.
No product code, carrier, verb, cause, field or span changed; CI#44241832579 is green 17/17.
@task_file n=1 path=docs/capabilities.md
what: the capability statement: what Escher can do, its stated limits, a11y reach and the claim map naming each claim's checks
@task_file n=2 path=README.md
what: the project readme and its per-version status section
@task_file n=3 path=deny.toml
what: the dependency audit's configuration, its advisory ignore list and each ignore's owner reason
@task_file n=4 path=.github/scripts/test_capability_statement.py
what: the CI Python script tests in the ci-scripts leg that pin the capability statement to the driver schema and now to the a11y and stand-repeat legs' counts
@task_file n=5 path=.claude/rules/a11y.md
what: the accessibility rule page for contributors, its a11y leg statements
@task_file n=6 path=.claude/docs/a11y-summary.md
what: the accessibility summary page, the a11y leg and its checks
@task_file n=7 path=.claude/docs/tests-summary.md
what: the tests summary page, the test targets, legs and their membership
@task_file n=8 path=.claude/docs/services/seven_guis.md
what: the seven_guis stand's service page, its served tasks and checks
@task_item n=1 part=touches
1-test-scope-summary/decision/capability-statement-pinned: the pinned set now also holds the a11y leg's file count and stand-repeat's named-target count, read from the CI leg script; the row lacks both.
@task_item n=2 part=touches
test-scope-summary/decision/capabilities-doc-pinned-to-schema: the same pin widened beyond the driver schema to the two leg counts.
@task_item n=3 part=touches
test-strategy/decision/capability-check-names-held-to-schema: no longer only the schema's tables hold the claim section; two counts are held to the CI leg script.
@task_item n=4 part=touches
unit-test-strategy/decision/capability-statement-from-schema: the statement is still read from the schema for its schema counts, but its pin now also reads the CI leg script; check its wording stays true.
@task_item n=5 part=touches
ci-integration/decision/a11y-leg-membership: a change of the a11y leg's membership now reddens ci-scripts until the capability statement's file count (eighteen) is re-stated.
@task_item n=6 part=touches
ci-integration/decision/joining-file-keeps-its-legs: a file joining or leaving the a11y leg or stand-repeat's named list now also owes the capability statement's count in words.
@task_item n=7 part=touches
9-ci-integration/decision/stand-repeat-three-rounds and quality-gates-coverage-targets/decision/zero-failure-stand-repeat: stand-repeat's named-target list is now pinned by count (seventeen), alongside the existing name pin in the workflow tests; the measured run read 273 · 1467 · 0 · 30 with three of three rounds green.
@task_item n=8 part=touches
1-test-scope-summary/decision/ci-scripts-unit-tested and 3-test-harness-contract/decision/python-script-tests-under-unittest: the ci-scripts leg gained a unittest test of the capability statement, 121 tests in the leg.
@task_item n=9 part=touches
unit-test-strategy/decision/mutation-controlled-tests: the new test's red-before-green control was read by hand, since the control tool reads no unittest result line and answered INDETERMINATE.
@task_item n=10 part=touches
unit-test-strategy/decision/expected-value-pinned-as-literal: the two leg counts are pinned as literals 17 and 18 and as their stated words.
@task_item n=11 part=touches
test-strategy/decision/capability-claims-name-their-checks: the claim map was re-pointed (eighteen accessibility files, seventeen named targets) and three new sections carry Witnesses lines naming stand_act_errors, stand_act_error_text, stand_seeded_randomness, stand_bundled_font_families and stand_cross_os_readings.
@task_item n=12 part=touches
test-strategy/decision/requirement-witness-inside-stand-repeat: the capability statement now states WITNESSES_030 at sixteen rows and 74 witnesses, and WITNESSES_020 at 49, not 46.
@task_item n=13 part=touches
test-strategy/decision/engine-items-asserted-on-tracing-build: the capability statement now states this as a limit, pinned by stand_act_error_text.
@task_item n=14 part=touches
test-strategy/decision/cross-os-readings-by-answer-digest and test-strategy/decision/stand-checks-bundled-font: the capability statement gained sections naming their stand checks as witnesses.
@task_item n=15 part=touches
ci-integration/decision/dependency-audit-advisories-and-licenses: the audit's ignore reasons now name residual rows as owners instead of a residuals file; the six advisory IDs are unchanged and the audit reads advisories ok, licenses ok.
@task_item n=16 part=touches
ci-integration/decision/ci-log-ansi-strip: the page's leg readings were tallied from ANSI-stripped job logs summing test result lines.
@task_item n=17 part=touches
e2e-test-strategy/decision/buffered-render-unborrowed-document and test-scope-summary/decision/buffered-render-on-dioxus-07-bridge: the capability statement now states the buffered render's same-render limit as asserted by no check, owned by a residual for the next version.
@task_item n=18 part=touches
Rows stating the stand's boots (3-test-harness-contract contract/stand-boot/*, decision/component-alone-*): the capability statement now describes the stand as six served tasks and one component alone; the sweep read these rows as still true.
@task_fact n=1 slug=ci-integration
fact: A file joining or leaving the a11y leg, or a target joining or leaving stand-repeat's named list, re-states the capability statement's count in words in the same change; the ci-scripts leg reddens until it does.
@task_fact n=2 slug=unit-test-strategy
fact: A red-before-green control of a Python unittest test is read by hand from its FAIL and OK lines, since the control tool reads only nextest, libtest and pytest result lines.
@task_fact n=3 slug=test-strategy
fact: The capability statement's requirement witness counts (WITNESSES_020, WITNESSES_030) are stated by hand and read by no check.
@task_fact n=4 slug=test-strategy
fact: No check asserts that an acting verb's caused carries an app's own text as its author wrote it; only its scrub is pinned, by stand_act_error_text.
@task_item n=1 part=untouched
Driver, session, wire and MCP contracts (verbs, refusal causes, exit codes, error sets, stdout and stderr lines): no product code, verb, cause, field or span changed.
@task_item n=2 part=untouched
Harness construction, settle, input, inspection, screenshot and contrast contracts: no harness code changed.
@task_item n=3 part=untouched
Cold-agent and agent-run contracts and decisions: the cold-agent run is still to come and nothing of its pipe changed.
@task_item n=4 part=untouched
WPT runner, status set and conformance decisions: untouched.
@task_item n=5 part=untouched
Coverage floors, mutation baseline and complexity ceiling decisions: no floor, threshold or ceiling moved.
@task_item n=6 part=untouched
Test data, fixtures and mocking decisions (stand-ins, virtual clock, timer ticks, offline stand): untouched.
@task_item n=7 part=untouched
Lint defect, snapshot and graph answer contracts and committed recorded checks: untouched.
@task_item n=8 part=untouched
CI action pinning, cache, trigger branches, package installs and the fast/slow split: no workflow or leg membership changed.
