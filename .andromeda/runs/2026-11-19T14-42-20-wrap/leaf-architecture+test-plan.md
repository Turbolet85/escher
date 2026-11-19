@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=83
last: - Fonts missing in headless tests → `system-fonts` feature / 
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/tests
@leaf_edit n=1 leaf=.claude/docs/commands.md line=13 class=omission
leaf-text: stops at the first red
source: ctr-nwhn7bv7 «After a green verdict, with `CI` unset or empty, it runs the target sweep, its line printed and its exit discarded; a red leg calls no sweep.»
replace-with: stops at the first red; after a green verdict, with `CI` unset or empty, it runs the dev-host target sweep (`scripts/target-sweep.py`, removing stale build units of the debug build directory alone; exit 0 swept or planned · 1 failed · 2 usage · 3 refused; `scripts/target-sweep.ps1` forwards on Windows), its line printed and its exit discarded; a red leg calls no sweep
@leaf_edit n=2 leaf=.claude/docs/commands.md line=14 class=stale
leaf-text: witness, the last step of `fast`, with no CI job)
source: ctr-o7cicfve «has no CI job, and is the last leg of `fast`, before the dev-host target sweep step.»
replace-with: witness, the last leg of `fast`, before its dev-host target sweep, with no CI job)
@leaf_edit n=3 leaf=.claude/docs/commands.md line=29 class=stale
leaf-text: the last step of `ci-leg.sh fast`, with no CI job
source: ctr-o7cicfve «has no CI job, and is the last leg of `fast`, before the dev-host target sweep step.»
replace-with: the last leg of `ci-leg.sh fast`, before its dev-host target sweep, with no CI job
@leaf_edit n=4 leaf=.claude/docs/commands.md line=62 class=stale
leaf-text: as the `embolden` leg, the last step of `fast`, with no CI job
source: ctr-o7cicfve «has no CI job, and is the last leg of `fast`, before the dev-host target sweep step.»
replace-with: as the `embolden` leg, the last leg of `fast`, before its dev-host target sweep, with no CI job
@leaf_edit n=5 leaf=.claude/docs/commands.md line=40 class=omission
leaf-text: A live model call on the operator's own Claude Code login — never in CI
source: dec-cjysdg6e «A version's live cold-agent run is fired once under a 3 USD cap and never re-fired, not across a surfaced stop nor a plan revision; another run needs a new ruling.»
replace-with: A live model call on the operator's own Claude Code login — never in CI; a version's live run is fired once under a 3 USD cap and never re-fired, not across a surfaced stop nor a plan revision, another run needing a new ruling
