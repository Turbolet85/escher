@row_add n=1 row=decision/custom-widget-text-from-document-fonts area=typography fact=F1 detector=none
name: Custom widget text shaped from the document's fonts
rule: A custom widget is handed the document's own shared font context before each paint, so text it draws is shaped from the same fonts as the document's text, the bundled-font route included.
why: So canvas text matches the document's text; the widget handoff is flagged upstreamable, a difference from upstream.
guard: stand_canvas_text
except: A widget that ignores the handed font context keeps its own fonts.
basis: fct-wvvcibgm
because: The chunk adds the font-context handoff to custom widgets, a contract the typography section does not yet state.
@row_add n=2 row=decision/custom-widget-content-box area=spacing fact=F2 detector=none
name: Custom widget painted and pointed in its content box
rule: A custom widget paints and hears pointer events in its content box: its paint size is the border box less border and padding times scale, its scene sits at the content origin inside the element's transform, unclipped, and pointer positions are offset by its left and top border plus padding.
why: Answers upstream issue #1578, where a widget painted at its border box; a difference from upstream.
guard: stand_canvas_placement, stand_canvas_pointer_frame
basis: fct-wvvcibgm
because: The chunk moves a custom widget's paint size, scene origin and pointer frame to its content box, a spacing decision no row of the section states.
