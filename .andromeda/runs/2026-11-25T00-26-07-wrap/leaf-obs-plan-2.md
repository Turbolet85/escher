@leaf_group group=obs-plan-2
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/rules/observability.md line=24 class=omission
leaf-text: nor the driver's canvas read, which opens no span of its own and, reached by the `canvas` verb, runs inside that call's `command` span,
source: dec-nwhdhtlo «reached by the canvas verb it runs inside that call's command span, and the lint's canvas label reading opens no span and logs nothing, running inside the lint call's command span.»
replace-with: nor the driver's canvas read, which opens no span of its own and, reached by the `canvas` verb, runs inside that call's `command` span, nor the lint's canvas label reading, which opens no span and logs nothing, running inside the `lint` call's `command` span,
