@leaf_group group=obs-plan-2
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/rules/observability.md line=24 class=omission
leaf-text: runs inside that call's `command` span, nor the lint's canvas label reading
source: dec-p5uvcuwx «decision/declared-node-read-logs-nothing · The declared-node read logs nothing — The in-process read of a canvas's declared virtual nodes, its validation and its answer have no log site, print, span or field; a refused declaration holds a closed reason and its index alone.»
replace-with: runs inside that call's `command` span, nor the in-process read of a canvas's declared virtual nodes, its validation and its answer, which have no log site, print, span or field, a refused declaration holding a closed reason and its index alone, nor the lint's canvas label reading
