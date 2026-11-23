@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: CSSOM View accessors, `Widget` trait,
source: ctr-nbk5fquv «contract/nodes-widgets/widget-fonts · blitz-dom nodes and widgets · Widget::fonts — A defaulted method that hands the widget the document's own shared Parley font context before each paint; the default ignores it.»
replace-with: CSSOM View accessors, `Widget` trait (its defaulted `Widget::fonts` hands the widget the document's own shared Parley font context before each paint, the default ignoring it),
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: `BoundingRect`, the public read-only `Node::try_layout_data`
source: ctr-mpeklktr «contract/base-document/font-ctx · BaseDocument · font_ctx — Public behind the custom-widget feature: returns a clone of the handle to the document's shared font context, so code holding it shares and can mutate the engine's own font context.»
replace-with: `BoundingRect`, `BaseDocument::font_ctx` (public behind the `custom-widget` feature: a clone of the handle to the document's shared font context, so code holding it shares and can mutate the engine's own font context), the public read-only `Node::try_layout_data`
@leaf_edit n=3 leaf=.claude/docs/services/blitz-paint.md line=24 class=omission
leaf-text: Custom widgets are pre-painted into Scenes keyed by document and node id.
source: dec-o4brf5xz «decision/custom-widget-content-box-paint · Custom widget painted at its content box — A custom widget's paint size is its content box times the scale, the border box less border and padding floored at 0»
replace-with: Custom widgets are pre-painted into Scenes keyed by document and node id; a widget's paint size is its content box times the scale (the border box less border and padding, floored at 0) and its scene is appended at the content origin inside the element's transform, unclipped (ratified, answering upstream issue 1578, a direct engine change flagged upstreamable; `stand_canvas_placement`).
