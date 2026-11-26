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
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: region as a `BoundingRect` and a disabled flag, the engine never calling it
source: ctr-jwsqrnv6 «Behind custom-widget, a defaulted method returning an empty list through which a widget declares regions, each a VirtualNodeDeclaration of key, AccessKit Role, name, region as a BoundingRect, a disabled flag»
replace-with: region as a `BoundingRect`, a disabled flag and an optional selected state, the engine never calling it
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: the widget still hearing every pointer event first)
source: dec-ymxetbsf «A focused custom-widget element does not trap Tab: the widget hears the KeyDown first, then the engine's Tab default action moves focus forward, Shift+Tab backward, so a widget cannot keep Tab for itself.»
replace-with: the widget still hearing every pointer event first; a focused custom-widget element does not trap Tab — the widget hears the KeyDown first, then the engine's Tab default action moves focus forward, Shift+Tab backward — a direct engine change for every document behind `custom-widget`, flagged upstreamable)
