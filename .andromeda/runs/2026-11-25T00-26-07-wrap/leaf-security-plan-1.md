@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=23 class=omission
leaf-text: closed defect words and numbers only (no `NodeId`, page text, colour or control value)
source: dec-i6hdc3hr «Lint findings hold no page content — A lint finding holds ids, closed defect words and numbers only, never page text, a canvas label's text, colour or a control value, and no log site, print, span or field carries a finding.»
replace-with: closed defect words and numbers only (no `NodeId`, page text, canvas label's text, colour or control value; a canvas label its widget paints crossing only as numbers, its ink box and its visible region inside a truncated finding, never a glyph id or font bytes)
