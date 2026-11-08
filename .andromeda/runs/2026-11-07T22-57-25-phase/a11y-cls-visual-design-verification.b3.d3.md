@answer n=1 entity=visual-design-verification#8.6 v=no
why: the scrollbar-band pointer rule governs driver click dispatch after a scroll, and the chunk changes no click or hit behaviour and its graph reads no hit test
@answer n=2 entity=visual-design-verification#8.7 v=no
why: the open list's covered-click refusal and paint-hit reading are hit testing, and the graph's bounds are client rects, not hit results
@answer n=3 entity=visual-design-verification#8.8 v=no
why: the paint order under mix-blend-mode is a paint rule the graph does not read
@answer n=4 entity=visual-design-verification#8.9 v=no
why: devtools layout outlines and hover or node highlight overlays are settings the graph neither reads nor paints
