@leaf_group group=obs-plan-1
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=43 class=omission
leaf-text: `SelectListRow.label` holds page text in a public read type and is never logged;
source: dec-x7gxy2pt «Dropped data (dropped file paths, dragged text and the contents of a file a Dioxus handler reads) is never logged: the OS drop source, the drag dispatch and the dropped-file reader write it to no log line»
replace-with: Dropped data — dropped file paths, dragged text and the contents of a file a Dioxus handler reads — is never logged: the OS drop source, the drag dispatch and the dropped-file reader write it to no log line, span or field. `SelectListRow.label` holds page text in a public read type and is never logged;
