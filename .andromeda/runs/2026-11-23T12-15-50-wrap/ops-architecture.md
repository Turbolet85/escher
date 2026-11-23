@row_set n=1 row=decision/forwarded-event-coordinates cell=rule seen=1 fact=F2 detector=none
value: Sub-document and custom-widget events are forwarded with adjusted coordinates: a custom widget hears pointer events in its content-box frame, the position plus its left and top border and padding in CSS px, untransformed, while a sub-document hears them from its border-box origin.
basis: fct-wvvcibgm
because: The chunk moves a custom widget's pointer frame to its content box while the sub-document arm stays at the border-box origin, so the row that says only "adjusted coordinates" must state each frame.
@row_add n=1 row=decision/custom-widget-content-box-paint area=established-decisions fact=F1 detector=none
name: Custom widget painted at its content box
rule: A custom widget's paint size is its content box times the scale, the border box less border and padding floored at 0, and its scene is appended at the content origin inside the element's transform, unclipped.
why: answers upstream issue 1578, a direct engine change flagged upstreamable
guard: stand_canvas_placement
basis: fct-wvvcibgm
because: The chunk fixes where and at what size a custom widget paints, a placement decision future widget work must respect, held by no standing row.
@row_add n=2 row=contract/nodes-widgets/widget-fonts area=standard-contracts fact=F3 detector=D-arch-resources
boundary: blitz-dom nodes and widgets
item: Widget::fonts
promise: A defaulted method that hands the widget the document's own shared Parley font context before each paint; the default ignores it.
basis: fct-wvvcibgm
because: The chunk adds a public defaulted method to the Widget trait, a new item on that boundary that the registry does not yet hold.
@row_add n=3 row=contract/base-document/font-ctx area=standard-contracts fact=F4 detector=D-arch-resources
boundary: BaseDocument
item: font_ctx
promise: Public behind the custom-widget feature: returns a clone of the handle to the document's shared font context, so code holding it shares and can mutate the engine's own font context.
basis: fct-wvvcibgm
because: The chunk adds a public BaseDocument reader that hands out the engine's shared font context, a new API the BaseDocument contracts do not register.
