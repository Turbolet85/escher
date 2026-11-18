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
leaf-text: `BoundingRect`, the public crate-root `TRACING_COMPILED`
source: ctr-gr7trxoq «contract/nodes-widgets/try-layout-data · blitz-dom nodes and widgets · Node::try_layout_data — Public and read-only: a node's layout data, None before the first layout.»
replace-with: `BoundingRect`, the public read-only `Node::try_layout_data` (a node's layout data, `None` before the first layout), the public crate-root `TRACING_COMPILED`
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=11 class=omission
leaf-text: keep the trait's do-nothing default
source: ctr-f6uhjel3 «contract/stylesheet-handler/document-url · blitz-dom StylesheetHandler · document_url — StylesheetHandler carries a public document_url, the URL of the document whose linked sheet it loads, used to stamp that»
replace-with: keep the trait's do-nothing default; `StylesheetHandler` carries a public `document_url`, the URL of the document whose linked sheet it loads, used to stamp that sheet's `@import` and `@font-face` requests
