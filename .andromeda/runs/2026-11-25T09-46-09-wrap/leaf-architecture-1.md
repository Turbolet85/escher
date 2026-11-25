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
leaf-text: name and region as a `BoundingRect`, the engine never calling it
source: ctr-jwsqrnv6 «contract/nodes-widgets/virtual-nodes · blitz-dom nodes and widgets · Widget::virtual_nodes and VirtualNodeDeclaration — Behind custom-widget, a defaulted method returning an empty list through which a widget»
replace-with: name, region as a `BoundingRect` and a disabled flag, the engine never calling it
@leaf_edit n=2 leaf=.claude/docs/services/blitz-shell.md line=18 class=omission
leaf-text: so a wrapper's override reaches the platform tree;
source: dec-ypqm4aep «decision/declaration-change-marks-nothing · A virtual node declaration change marks nothing — A change of what a canvas declares writes no changed-set mark and sends no invalidation signal, so a windowed»
replace-with: so a wrapper's override reaches the platform tree, while a change of a canvas's declared virtual nodes marks no changed-set entry and so reaches the adapter only at its next rebuild;
