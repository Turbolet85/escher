@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=71
last: - 2026-11-14: To plant a panicking handler in a Dioxus fixtu
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own 
@leaf_edit n=1 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: so the mutated run stays bounded.
source: dec-wdwf7q3c «decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply»
replace-with: so the mutated run stays bounded; a control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen (a pump on a settled screen is invisible).
