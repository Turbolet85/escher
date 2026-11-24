@leaf_group group=obs-plan-2
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing
@leaf_edit n=1 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: A new field carrying user content takes a name in the content set — or is not logged.
source: dec-x7gxy2pt «decision/dropped-data-never-logged · Dropped data never logged — Dropped data (dropped file paths, dragged text and the contents of a file a Dioxus handler reads) is never logged: the OS drop source»
replace-with: Dropped data (dropped file paths, dragged text and the contents of a file a Dioxus handler reads) is never logged: the OS drop source, the drag dispatch and the dropped-file reader write it to no log line, span or field. A new field carrying user content takes a name in the content set — or is not logged.
