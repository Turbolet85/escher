@row_add n=1 row=decision/canvas-label-lint-crossing area=threat-model-summary detector=none
name: Canvas label crossing in a lint finding
rule: A canvas label its widget paints crosses the lint's ratified carriers only as numbers, its ink box and its visible region inside a truncated finding, never the label's text, a glyph id or font bytes.
guard: stand_canvas_lint the_lint_verb_answers_the_canvas_findings
basis: fct-ya64mcmm, fct-xff2nsjd, fct-fmp5esit, fct-nolgh3rx
because: The lint now reads a canvas label's ink box and answers it as a truncated finding's text and visible numbers, a new path of canvas content that the canvas-content-crossing row, already at its bound, does not name.
@row_add n=2 row=decision/skrifa-glyph-outline-bounds area=dependency-security fact=F1 detector=D-security-deps
name: skrifa reads glyph outline bounds in the driver
rule: escher-driver takes skrifa, workspace-inherited and already in the graph through blitz-dom and blitz-paint, to read a canvas widget's glyph outline bounds from its own font bytes, adding no package to Cargo.lock.
state: provisional
basis: fct-xisnxkeg, fct-bkjckagp
because: The driver takes a new direct dependency, by one lock edge with the audit leg green, and the dependency set states which crate does this role.
@row_set n=1 row=decision/lint-findings-closed-content cell=rule seen=1 detector=D-security-input
value: A lint finding holds ids, closed defect words and numbers only, never page text, a canvas label's text, colour or a control value, and no log site, print, span or field carries a finding.
basis: fct-xff2nsjd, fct-fmp5esit, fct-mbmku3j6
because: A finding's numbers now include a canvas label's ink box while the label's text is asserted never to cross, which the rule must name now that canvas labels are read.
