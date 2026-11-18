@task_prose part=session
The session rewrote the capability statement for Escher 0.3.0: what it can do, its stated limits, its accessibility reach and where each claim is proven.
The accessibility reach now states the a11y leg runs eighteen files, re-read as 18 · 163 · 0 · 0 on CI, eleven of them also in stand-repeat, and names the hidden-focus SC 2.4.3 gap with its owner carried at 0.3.0.
A new ci-scripts check pins the a11y leg's file count and stand-repeat's seventeen named targets against the capability statement, so a change of either leg's membership reddens ci-scripts until the statement is re-stated.
The README's status reads 0.3.0 built, its cold-agent run to come; deny.toml's advisory owner clauses now name residual rows; four harness leaf pages had stale counts removed.
No product code changed: no role, state, name, key, focus, contrast or snapshot behaviour moved, and no carrier, verb, cause or field was added.
@task_file n=1 path=docs/capabilities.md
what: the capability statement, the page stating what Escher can do, its limits, its accessibility reach and where each claim is proven
@task_file n=2 path=README.md
what: the project README and its status section per version
@task_file n=3 path=deny.toml
what: the dependency audit config, its ignored advisories and their owner clauses
@task_file n=4 path=.github/scripts/test_capability_statement.py
what: the capability statement's pin, the ci-scripts check holding the statement's counts to their sources
@task_file n=5 path=.claude/rules/a11y.md
what: the accessibility rule page for agents, stating the a11y leg and its checks
@task_file n=6 path=.claude/docs/a11y-summary.md
what: the accessibility summary page, the a11y checks and their leg
@task_file n=7 path=.claude/docs/tests-summary.md
what: the tests summary page, the test legs and the stand checks they run
@task_file n=8 path=.claude/docs/services/seven_guis.md
what: the seven_guis service page, the stand's tasks and their checks
@task_item n=1 part=touches
decision/capability-statement-reach-by-hand: its why says the capability statement's pin reads none of the accessibility reach's claims; the pin now reads the reach's a11y leg file count, the rest stays by hand.
@task_item n=2 part=touches
The a11y CI leg and the accessibility test leg rows (decision/a11y-ci-leg, decision/a11y-test-leg, decision/a11y-ci-gate): the leg now runs eighteen files, read as 18 · 163 · 0 · 0, and its file count is now held by a ci-scripts check; any cell stating the leg's file count or result line moves.
@task_item n=3 part=touches
decision/stand-checks-repeated-three-rounds: stand-repeat now runs the stand files and seventeen named blitz-tests targets, select_popup and select_popup_painted among them, read 273 · 1467 · 0 · 30; its named-target count is now pinned against the capability statement.
@task_item n=4 part=touches
Rows saying the select list checks or the popup stand check are not among the a11y leg's files (decision/select-list-coverage-headless and the select-list rows that cite their leg): since the a11y re-read the leg runs select_popup, select_popup_painted and stand_act_popup.
@task_item n=5 part=touches
The hidden-focus limit (decision/inert-subtree-excluded, decision/focus-fixup-reads-inert-alone, decision/removed-focused-node-drops-focus): the capability statement now states as a limit that a focused control that no longer shows keeps focus, the SC 2.4.3 gap, with its residual owner carried at 0.3.0, target next.
@task_item n=6 part=touches
decision/buffered-dioxus-render: the capability statement now states the buffered render's same-render limit as a 0.3.0 limit, asserted by no check, owned by a residual row.
@task_item n=7 part=touches
Rows stating stand checks run on the four lean tasks (the stand checks and their tab-order and expectation tables): the capability statement now describes the stand as six served tasks and one component alone; the lean-set claim itself stays true.
@task_item n=8 part=touches
The accessibility reach rows (absence/assistive-technology-reach, decision/accessibility-reach-tier-0, decision/stand-builds-no-platform-adapter): the capability statement's reach section was rewritten for 0.3.0; check each row still matches the reach it states.
@task_fact n=1 slug=ci-integration
fact: A change of the a11y leg's file membership or of stand-repeat's named targets reddens the ci-scripts leg until the capability statement re-states the count in words.
@task_item n=1 part=untouched
ARIA role mapping, ARIA property reads and the accessibility tree state contracts: no role, state or property read changed.
@task_item n=2 part=untouched
Keyboard contracts (Cells grid, CRUD list, range input, select keys, keyboard activation, Tab order): no key behaviour changed.
@task_item n=3 part=untouched
Contrast, the lint's contrast finding, colours, scrollbar and viewport scheme or zoom rows: nothing visual changed.
@task_item n=4 part=untouched
Snapshot, snapshot text form, snapshot diff, actions answer, driver refusal and screenshot contracts: no carrier, verb, cause or field changed.
@task_item n=5 part=untouched
Accessible name precedence, password masking, AccessKit adapter and accessibility feature gating decisions: no product code or feature flag moved.
@task_item n=6 part=untouched
Cognitive accessibility and the invalid-state rows: untouched.
