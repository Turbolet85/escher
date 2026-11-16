@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=44 class=omission
leaf-text: rather than removing it so the mutated run stays bounded;
source: dec-wdwf7q3c «decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply, so a read»
replace-with: rather than removing it so the mutated run stays bounded, and a control of a read that must change nothing giving the read pending work that a pass would apply, so a read that ran a pass would be seen;
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: and process telemetry (stdout silent, scrub, panic hook, idempotent init — one process per file).
source: dec-opk4kyum «decision/layout-reason-words-each-read · Every layout reader word read on a fixture — Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, from tables that»
replace-with: the layout reader (`layout_reason`: every placing word and every size cause read on a fixture node in both layout modes, from tables that count their rows; a layout read changing nothing — no change pending, the snapshot, focus, diff and clock unchanged, an open list staying open and delivered Timer ticks unapplied; a tolerance stated as to 1/64 px read inclusively) and process telemetry (stdout silent, scrub, panic hook, idempotent init — one process per file).
