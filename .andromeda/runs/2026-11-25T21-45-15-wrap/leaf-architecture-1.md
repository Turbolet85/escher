@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: AccessKit `Role` (AccessKit's own, no escher role set), name,
source: ctr-wragicha «Behind custom-widget, the crate root re-exports AccessKit's Role as VirtualNodeRole, the type a virtual node declaration's role takes, so an app reaches it without naming accesskit.»
replace-with: AccessKit `Role` (AccessKit's own, no escher role set, re-exported from the crate root behind `custom-widget` as `VirtualNodeRole` so an app reaches it without naming accesskit), name,
@leaf_stands n=1 leaf=.claude/docs/services/blitz-dom.md line=15
because: The line's "not counted" speaks of engine changes not counted among escher's direct ones, not of handler coverage, so the canvas handler-coverage rows ctr-rq3jq44s and ctr-clnjioqg and the retired uncounted decision do not bear on it.
@leaf_stands n=2 leaf=.claude/docs/services/blitz-dom.md line=21
because: The line's "not counted among escher's direct ones" counts upstream engine changes, not canvas handlers, so nothing the handler-coverage rows ctr-rq3jq44s and ctr-clnjioqg now say or the retired uncounted decision said bears on it.
