@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=22 class=omission
leaf-text: (no `NodeId`, page text, colour or control value)
source: dec-i6hdc3hr «decision/lint-findings-closed-content · Lint findings hold no page content — A lint finding holds ids, closed defect words and numbers only, never page text, a canvas label's text, colour or a control value»
replace-with: (no `NodeId`, page text, canvas label's text, colour or control value)
@leaf_edit n=2 leaf=.claude/rules/security.md line=21 class=omission
leaf-text: a `lint`'s findings — stable ids, closed defect words and numbers
source: dec-lyyw3eam «A canvas label its widget paints crosses the lint's ratified carriers only as numbers, its ink box and its visible region inside a truncated finding, never the label's text, a glyph id or font bytes.»
replace-with: a `lint`'s findings — stable ids, closed defect words and numbers, a canvas label crossing only as numbers (its ink box and its visible region inside a truncated finding), never the label's text, a glyph id or font bytes
@leaf_edit n=3 leaf=.claude/rules/security.md line=28 class=omission
leaf-text: reaches the harness, adding no package to `Cargo.lock`.
source: dec-kfileyuy «escher-driver takes skrifa, workspace-inherited and already in the graph through blitz-dom and blitz-paint, to read a canvas widget's glyph outline bounds from its own font bytes, adding no package to Cargo.lock.»
replace-with: reaches the harness, adding no package to `Cargo.lock`. escher-driver takes skrifa, workspace-inherited and already in the graph through blitz-dom and blitz-paint, to read a canvas widget's glyph outline bounds from its own font bytes, adding no package to `Cargo.lock` (provisional).
