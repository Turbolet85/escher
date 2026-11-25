@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dire
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/stac
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: `Widget` trait (its defaulted `Widget::fonts` hands the widget the document's own shared Parley font context before each paint, the default ignoring it;
source: ctr-jwsqrnv6 «contract/nodes-widgets/virtual-nodes · blitz-dom nodes and widgets · Widget::virtual_nodes and VirtualNodeDeclaration — Behind custom-widget, a defaulted method returning an empty list through which»
replace-with: `Widget` trait (its defaulted `Widget::fonts` hands the widget the document's own shared Parley font context before each paint, the default ignoring it; behind `custom-widget` its defaulted `Widget::virtual_nodes`, returning an empty list, through which a widget declares regions, each a `VirtualNodeDeclaration` of key, AccessKit `Role` (AccessKit's own, no escher role set), name and region as a `BoundingRect`, the engine never calling it and a declaration change marking no changed-set entry;
