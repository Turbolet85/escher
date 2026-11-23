@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/services/blitz-test-harness.md line=11 class=omission
leaf-text: `blitz-paint` (`default-features = false`, so no `svg` and no `scrollbars` in a per-package build)
source: dec-rlsleth2 «Harness turns on blitz-paint's custom-widget feature — blitz-test-harness names blitz-paint's custom-widget feature, so a harness or driver shot paints an app's widget in every build selection reaching the harness.»
replace-with: `blitz-paint` (`default-features = false` with its `custom-widget` feature named, so a harness or driver shot paints an app's widget in every build selection reaching the harness, and no `svg` and no `scrollbars` in a per-package build)
@leaf_edit n=2 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: --test stand_masks_on_new_answers` (`stand_masks_on_new_answers`:
source: dec-nqizbumm «App widget painted from Rust shows in shots — A widget an app paints from Rust shows in a harness shot and in a driver shot, whole and cropped to its element, in both layout modes, under each per-package test selection and in the workspace build.»
replace-with: --test stand_masks_on_new_answers --test stand_canvas_shot` (`stand_canvas_shot`: a widget an app paints from Rust showing in a harness shot and in a driver shot, whole and cropped to its element, both layout modes, under each per-package test selection and in the workspace build, its content crossing the carriers only as pixels; `stand_masks_on_new_answers`:
@leaf_stands n=1 leaf=.claude/docs/commands.md line=52
because: The `custom-widget` here is in blitz-dom's feature list for upstream's own clippy line, which no changed row touches; the new row dec-rlsleth2 concerns only blitz-test-harness naming blitz-paint's custom-widget feature.
@leaf_stands n=2 leaf=.claude/docs/commands.md line=14
because: The stand-repeat leg runs every `stand_*.rs` by its prefix, so the new guard `stand_canvas_shot` is picked up without a change to the line, and no changed row alters the leg or its named targets.
@leaf_stands n=3 leaf=.claude/docs/commands.md line=62
because: The line covers the driver's screen-level refusals, scroll, range input and select engine checks, none of which the new canvas or custom-widget rows bear on.
@leaf_stands n=4 leaf=.claude/docs/commands.md line=63
because: The line covers engine-fact targets for client rects, hits, the virtual clock and text extent, none of which the new canvas-as-pixels or app-widget-in-shots rows bear on.
@leaf_stands n=5 leaf=.claude/docs/services/blitz-test-harness.md line=15
because: Its "canvas" is the CSS canvas background (the root element's, else body's) the contrast reader composites onto, not an app-painted canvas, so dec-vd5ewvhr leaves it true.
