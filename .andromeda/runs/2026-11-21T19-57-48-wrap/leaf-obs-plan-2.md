@leaf_group group=obs-plan-2
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: `SelectListRow.label` holds page text in a public read type and is never logged; a label leaves only as a painted pixel and the snapshot's existing `name`.
source: dec-56o4ca6s «decision/select-chosen-set-never-logged · Select chosen set never logged — A select's chosen state never leaves as a log line, span or field: a multiple select's chosen set leaves only as form entries»
replace-with: `SelectListRow.label` holds page text in a public read type and is never logged; a label leaves only as a painted pixel and the snapshot's existing `name`. A select's chosen state never leaves as a log line, span or field: a multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, and a single list box's chosen row and row labels only in the snapshot, the diff and the actions answer.
