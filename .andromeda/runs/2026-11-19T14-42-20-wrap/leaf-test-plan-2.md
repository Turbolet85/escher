@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file ou
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=44 class=omission
leaf-text: (fmt → clippy → test → CI scripts → embolden, stopping at the first red;
source: dec-c5zu7n7g «The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate; on a dev host with CI unset, a green fast leg ends with the target sweep»
replace-with: (fmt → clippy → test → CI scripts → embolden, stopping at the first red; on a dev host with CI unset a green fast leg ends with the target sweep, whose refusal or failure never reddens the leg, and a red leg calls none;
