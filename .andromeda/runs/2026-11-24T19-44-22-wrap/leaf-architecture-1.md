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
@leaf_edit n=1 leaf=.claude/docs/services/blitz-paint.md line=14 class=omission
leaf-text: past the blend layers, reaching a shell or renderer through `paint_scene` alone.
source: ctr-3xavymqf «contract/paint/custom-widget-scene · blitz-paint · custom_widget_scene — custom_widget_scene returns one custom widget's scene outside a raster at a given scale, behind the custom-widget feature, beside build_custom_widget_scenes»
replace-with: past the blend layers, reaching a shell or renderer through `paint_scene` alone; and, behind the `custom-widget` feature beside `build_custom_widget_scenes`, `custom_widget_scene`, which returns one custom widget's scene outside a raster at a given scale (an escher addition flagged upstreamable).
