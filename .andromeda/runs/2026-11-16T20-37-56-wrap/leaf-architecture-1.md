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
leaf-text: `BoundingRect`, the layout reason's
source: ctr-3kblfyru «contract/blitz-dom-crate-root/tracing-compiled · blitz-dom crate root · TRACING_COMPILED — Public: true when the engine's tracing feature is compiled in.»
replace-with: `BoundingRect`, the public crate-root `TRACING_COMPILED` (true when the engine's `tracing` feature is compiled in), the layout reason's
