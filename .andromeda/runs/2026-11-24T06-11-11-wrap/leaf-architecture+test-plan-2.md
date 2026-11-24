@leaf_group group=architecture+test-plan-2
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary (
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=18 class=omission
leaf-text: Otherwise they click the centre of the element's snapshot `bounds` plus the viewport's scroll
source: dec-dvtoe7w5 «decision/canvas-placement-at-click-centre · Canvas placement only at the click centre — A driver click lands at the centre of its target element, so an agent places content on a canvas only at the canvas's centre»
replace-with: Otherwise they click the centre of the element's snapshot `bounds` plus the viewport's scroll (a measured limit: an agent places content on a canvas only at the canvas's centre, no verb landing a pointer elsewhere)
