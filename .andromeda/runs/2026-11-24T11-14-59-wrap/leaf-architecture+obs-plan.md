@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summary
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=26 class=omission
leaf-text: so the field-name scrub reaches them; no filter layer is added
source: dec-x7gxy2pt «Dropped data (dropped file paths, dragged text and the contents of a file a Dioxus handler reads) is never logged: the OS drop source, the drag dispatch and the dropped-file reader write it to no log line, span or field.»
replace-with: so the field-name scrub reaches them; dropped data (dropped file paths, dragged text and the contents of a file a Dioxus handler reads) is never logged — the OS drop source, the drag dispatch and the dropped-file reader write it to no log line, span or field; no filter layer is added
