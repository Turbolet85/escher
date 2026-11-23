@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=77
last: - 2026-11-22: Any cargo run on the shared `target/` without 
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=37 class=omission
leaf-text: and refuses a document whose `can_paint_text` reads false with nothing painted.
source: dec-nqizbumm «decision/app-widget-shows-in-shots · App widget painted from Rust shows in shots — A widget an app paints from Rust shows in a harness shot and in a driver shot, whole and cropped to its element, in both layout»
replace-with: and refuses a document whose `can_paint_text` reads false with nothing painted; a widget an app paints from Rust shows in a harness shot and in a driver shot, whole and cropped to its element, in both layout modes, under each per-package test selection and in the workspace build (`stand_canvas_shot`).
@leaf_stands n=1 leaf=.claude/rules/testing.md line=15
because: The pass only added the app-widget-shows-in-shots decision, which retires no stand check and says nothing of which checks share `tests/common` or `tests/session_common`, so the line's stand and session-module lists contradict no row.
@leaf_stands n=2 leaf=.claude/rules/testing.md line=41
because: No row of the pass touches the agent-run selections, and `run stand` still runs the `stand_*` files, which a new `stand_canvas_shot` joins by its name alone.
@leaf_stands n=3 leaf=.claude/rules/testing.md line=48
because: The added row states no flakiness budget, coverage floor or stand-repeat count, so the line's measured-gap statements still match the rows.
@leaf_stands n=4 leaf=.claude/rules/verification-harness.md line=15
because: The added decision changes nothing in the agent-run 5-command contract the line states.
@leaf_stands n=5 leaf=.claude/rules/verification-harness.md line=17
because: The added decision touches neither the `{name}` grammar nor the empty `run stand` rule.
@leaf_stands n=6 leaf=.claude/rules/verification-harness.md line=21
because: The added decision says nothing of the fixtures bootstrap or of which checks read the shared fixtures module, so the line's selections contradict no row.
