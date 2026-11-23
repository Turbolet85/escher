@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: a shot's PNG equal to the harness's raster (`stand_act_screenshot`),
source: dec-nqizbumm «decision/app-widget-shows-in-shots · App widget painted from Rust shows in shots — A widget an app paints from Rust shows in a harness shot and in a driver shot, whole and cropped to its element,»
replace-with: a shot's PNG equal to the harness's raster (`stand_act_screenshot`), a widget an app paints from Rust showing in a harness shot and in a driver shot, whole and cropped to its element, in both layout modes, under each per-package test selection and in the workspace build (`stand_canvas_shot`),
@leaf_stands n=1 leaf=.claude/docs/tests-summary.md line=13
because: The line's "canvas" is the contrast reader's root-else-body backdrop, and no test-plan row lost words this pass, whose only change adds decision/app-widget-shows-in-shots, answered by finding n=1 on line 26.
@leaf_stands n=2 leaf=.claude/docs/tests-summary.md line=46
because: No test-plan row lost words this pass and none names the custom-widget feature of the clippy gate, so the line's second clippy invocation is not contradicted by any row.
@leaf_stands n=3 leaf=.claude/docs/tests-summary.md line=16
because: No test-plan row lost words this pass (the only change adds decision/app-widget-shows-in-shots), so the cold-agent pipe's stand claims bear on no changed row.
@leaf_stands n=4 leaf=.claude/docs/tests-summary.md line=18
because: No test-plan row lost words this pass (the only change adds decision/app-widget-shows-in-shots), so the fixture bootstrap and run stand counts bear on no changed row.
@leaf_stands n=5 leaf=.claude/docs/tests-summary.md line=21
because: No test-plan row lost words this pass and no row names session_common, so the stand_* coverage and shared test-module claims bear on no changed row.
@leaf_stands n=6 leaf=.claude/docs/tests-summary.md line=29
because: No test-plan row lost words this pass (the only change adds decision/app-widget-shows-in-shots), so the CI leg and stand-repeat claims bear on no changed row.
@leaf_stands n=7 leaf=.claude/docs/tests-summary.md line=30
because: No test-plan row lost words this pass (the only change adds decision/app-widget-shows-in-shots), so the local baseline and run stand counts bear on no changed row.
@leaf_stands n=8 leaf=.claude/docs/tests-summary.md line=43
because: No test-plan row lost words this pass (the only change adds decision/app-widget-shows-in-shots), so the flakiness budget's stand-repeat claims bear on no changed row.
