@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summary
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: running inside the lint call's `command` span)
source: dec-p5uvcuwx «decision/declared-node-read-logs-nothing · The declared-node read logs nothing — The in-process read of a canvas's declared virtual nodes, its validation and its answer have no log site, print, span or field;»
replace-with: running inside the lint call's `command` span; the in-process read of a canvas's declared virtual nodes, its validation and its answer having no log site, print, span or field, a refused declaration holding a closed reason and its index alone)
