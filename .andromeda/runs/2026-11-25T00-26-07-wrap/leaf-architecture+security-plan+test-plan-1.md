@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=11 class=omission
leaf-text: — and `escher-telemetry` (the workspace's), whose scoped capture default
source: dec-jqo2g7ng «escher-driver takes the workspace's skrifa to read a glyph's outline bounds for the canvas lint. · why: skrifa is already in the graph through blitz-dom and blitz-paint, so no package is added.»
replace-with: — and, since 2026-11-24-canvas-lint, `skrifa` (the workspace's, already in the graph through blitz-dom and blitz-paint, so no package is added), which reads a glyph's outline bounds for the canvas lint — and `escher-telemetry` (the workspace's), whose scoped capture default
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=17 class=omission
leaf-text: per defect it reads, in document order, then `Defect::ALL` order:
source: ctr-saozb5cg «in document order, then defect order, a canvas element's `truncated` read once per label in paint order.»
replace-with: per defect it reads, in document order, then `Defect::ALL` order, a canvas element's `truncated` read once per label in paint order:
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=17 class=omission
leaf-text: `truncated` reads blitz-dom's `BaseDocument::text_extent` (its one non-test caller) on a node that is no text control,
source: ctr-s5ximkln «a node clipped or off-screen that holds text also reads `truncated`; a canvas element reads `truncated` once per label whose ink box does not lie inside its region.»
replace-with: `truncated` reads blitz-dom's `BaseDocument::text_extent` (its one non-test caller) on a node that is no text control, and a canvas element reads `truncated` once per label whose ink box (the union of its glyphs' outline bounds) does not lie inside its region (the widget's content box cut by each pushed clip, layer or mask path's bounding box, mapped through its transform, and by the element's clip region),
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=17 class=omission
leaf-text: hold no `NodeId`, page text, colour or value
source: dec-i6hdc3hr «A lint finding holds ids, closed defect words and numbers only, never page text, a canvas label's text, colour or a control value, and no log site, print, span or field carries a finding.»
replace-with: hold no `NodeId`, page text, canvas label's text, colour or value
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=24 class=omission
leaf-text: its limits — geometry only, no hit, right-to-left and vertical text read as left-to-right lines)
source: ctr-ualaeppi «No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines, a canvas widget's labels read by their ink boxes; dispatches nothing.»
replace-with: its limits — geometry only, no hit, right-to-left and vertical text read as left-to-right lines, a canvas widget's labels read by their ink boxes)
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=6 class=omission
leaf-text: the closed defect words and numbers, no page text, colour, control value or `NodeId` (a ratified boundary widening)
source: dec-i6hdc3hr «A lint finding holds ids, closed defect words and numbers only, never page text, a canvas label's text, colour or a control value, and no log site, print, span or field carries a finding.»
replace-with: the closed defect words and numbers, no page text, canvas label's text, colour, control value or `NodeId` (a ratified boundary widening)
@leaf_edit n=7 leaf=.claude/docs/services/escher-driver.md line=32 class=omission
leaf-text: the closed defect words and numbers, no page text, colour, control value or `NodeId` — with no new carrier
source: dec-lyyw3eam «A canvas label its widget paints crosses the lint's ratified carriers only as numbers, its ink box and its visible region inside a truncated finding, never the label's text, a glyph id or font bytes.»
replace-with: the closed defect words and numbers, no page text, colour, control value or `NodeId`, a canvas label crossing (since 2026-11-24-canvas-lint) only as numbers, its ink box and its visible region inside a truncated finding, never the label's text, a glyph id or font bytes — with no new carrier
@leaf_edit n=8 leaf=.claude/docs/services/escher-driver.md line=60 class=omission
leaf-text: findings follow document order, then the defect order
source: dec-eeyz3rvo «the session lint's readings over a held instance are tested by integration checks, not unit tests; the canvas label reader, which reads recorded paint commands and no held instance, is unit-tested.»
replace-with: findings follow document order, then the defect order, a canvas element's `truncated` once per label in paint order, and, since 2026-11-24-canvas-lint, the canvas label reader, which reads recorded paint commands and no held instance, unit-tested
