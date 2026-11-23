@leaf_group group=architecture+security-plan-3
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=omission
leaf-text: an app's widget painted from Rust (a canvas) paints in a harness or driver shot
source: dec-uii2ewmf «Custom widget painted and pointed in its content box — Behind the custom-widget feature, a custom widget's paint size is its content box times scale, floored at 0, its scene sits at the content origin»
replace-with: an app's widget painted from Rust (a canvas) paints at its content box (its paint size the content box times the scale, floored at 0, its scene at the content origin inside the element's transform, unclipped, its pointer events arriving in its content-box frame while a sub-document's stay in its border-box frame) in a harness or driver shot
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=omission
leaf-text: blitz-test-harness naming blitz-paint's custom-widget feature,
source: ctr-nbk5fquv «contract/nodes-widgets/widget-fonts · blitz-dom nodes and widgets · Widget::fonts — A defaulted method that hands the widget the document's own shared Parley font context before each paint; the default ignores it.»
replace-with: blitz-test-harness naming blitz-paint's custom-widget feature, the widget handed the document's own shared, mutable Parley font context before each paint (`Widget::fonts`, defaulted to ignore it; `BaseDocument::font_ctx` public behind that feature),
