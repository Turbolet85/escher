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
leaf-text: and a click on any other element still clearing focus;
source: dec-svdas4ye «except: a select focuses through its own arm, and a list box's row focuses its select; a click on a custom-widget target runs no click walk and clears no focus»
replace-with: and a click on any other element still clearing focus, a click on a custom-widget target running no click walk and clearing no focus;
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: the default ignoring it)
source: dec-sr3didn3 «On a custom-widget target the engine, behind the custom-widget feature, synthesises the click on a main-button pointerup, contextmenu on a secondary one and a double click on a second click»
replace-with: the default ignoring it; on a custom-widget target the engine, behind the `custom-widget` feature, synthesises the click on a main-button pointerup, `contextmenu` on a secondary one and a double click on a second click, after the widget's own handling and the focus set, with no click walk and no focus clear — a direct engine change for every document behind `custom-widget`, ratified and flagged upstreamable, the widget still hearing every pointer event first)
@leaf_edit n=3 leaf=.claude/docs/services/blitz-paint.md line=30 class=tree
leaf-text: (gradient.rs, css_box.rs, layers.rs)
source: tree «packages/blitz-paint/src/kurbo_css/css_box.rs»
replace-with: (gradient.rs, kurbo_css/css_box.rs, layers.rs)
