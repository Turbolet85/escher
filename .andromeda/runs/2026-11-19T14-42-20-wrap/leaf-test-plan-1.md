@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=29 class=omission
leaf-text: `embolden` (the `blitz-paint/font-embolden` witness, a leg with no CI job of its own, macOS exercising the feature there)
source: dec-c5zu7n7g «The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate; on a dev host with CI unset, a green fast leg ends with the target sweep»
replace-with: `embolden` (the `blitz-paint/font-embolden` witness, a leg with no CI job of its own, macOS exercising the feature there), the fast legs stopping at the first red, and on a dev host with CI unset a green fast leg ending with the target sweep, whose refusal or failure never reddens the leg, a red leg calling none
