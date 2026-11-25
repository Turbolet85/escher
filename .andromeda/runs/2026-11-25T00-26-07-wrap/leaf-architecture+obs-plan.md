@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=47
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summar
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: reached by the `canvas` verb inside that call's `command` span)
source: dec-nwhdhtlo «and the lint's canvas label reading opens no span and logs nothing, running inside the lint call's command span.»
replace-with: reached by the `canvas` verb inside that call's `command` span; the lint's canvas label reading likewise opening no span and logging nothing, running inside the lint call's `command` span)
