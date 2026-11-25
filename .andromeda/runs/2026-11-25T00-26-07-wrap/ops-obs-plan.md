@row_set n=1 row=decision/canvas-read-opens-no-span cell=rule seen=1 detector=D-obs-instrumentation
value: The driver's in-process canvas read opens no span of its own, as the layout read does not; reached by the canvas verb it runs inside that call's command span, and the lint's canvas label reading opens no span and logs nothing, running inside the lint call's command span.
basis: fct-nolgh3rx, fct-mbmku3j6
because: The lint now reads each canvas widget's painted labels, and the report carries that this new reading opens no span and logs nothing by decision, so the row naming the canvas read's span must cover the lint's reading too.
@row_add n=1 row=decision/canvas-lint-label-crosses-as-numbers area=pii-scrubbing-compliance fact=F1 detector=D-obs-pii
name: Canvas lint label crosses as numbers only
rule: A canvas label read by the lint crosses only as numbers, its ink box and its visible region inside a truncated finding; its text and glyph ids never leave the process and are never logged.
guard: the_lint_verb_answers_the_canvas_findings
basis: fct-mbmku3j6, fct-fmp5esit, fct-xff2nsjd
because: The lint now reads labels a canvas widget paints, and the report asserts that only the label's box crosses in a truncated finding with no text or glyph id, a crossing no obs-plan row states yet.
