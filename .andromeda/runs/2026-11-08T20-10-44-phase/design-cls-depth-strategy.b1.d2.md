@answer n=1 entity=depth-strategy#1.1 v=no
why: browser app's z-index values; the chunk sets and reads no z-index
@answer n=2 entity=depth-strategy#1.2 v=no
why: browser menu shadow value; the chunk paints no shadow
@answer n=3 entity=depth-strategy#1.3 v=no
why: points to another page's shadow values; the chunk draws no shadow
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc example's layered shadows; no chunk code or test reads them
@answer n=5 entity=depth-strategy#1.5 v=no
why: example apps' stacking indices; the chunk sets no z-index and reads no stacking order
@answer n=6 entity=depth-strategy#1.6 v=no
why: default dialog backdrop colour; the chunk paints no backdrop
@answer n=7 entity=depth-strategy#2.1 v=no
why: paint pipeline ordering in blitz-dom and blitz-paint, outside the reached files; the chunk builds no stacking context
@answer n=8 entity=depth-strategy#2.2 v=no
why: engine rule for stacking-context roots; the chunk's code and tests neither create nor read a stacking context
@answer n=9 entity=depth-strategy#3.1 v=no
why: hit-test walk the chunk neither calls nor changes; the coverage read dispatches no pointer and sets no hit value
@answer n=10 entity=depth-strategy#3.2 v=no
why: open select list hit answer the chunk neither calls nor reads; the coverage count reads no covered refusal
@answer n=11 entity=depth-strategy#3.3 v=no
why: engine hit rule for degenerate transforms; the chunk sets no transform
@answer n=12 entity=depth-strategy#3.4 v=no
why: clip-region read the chunk does not call; the coverage read takes no visible region
@answer n=13 entity=depth-strategy#4.1 v=no
why: paint-order rule for z-index auto in blitz-paint; the chunk paints nothing
@answer n=14 entity=depth-strategy#4.2 v=no
why: paint order of the engine's open select list; the chunk paints nothing
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow paint rule; the chunk draws no shadow
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow paint rule; the chunk draws no shadow
@answer n=17 entity=depth-strategy#4.5 v=no
why: paint skip for inline boxes holding only block children; the chunk paints no box and reads no paint flag
@answer n=18 entity=depth-strategy#4.6 v=no
why: opacity and filter layer paint; the chunk applies no opacity or filter
@answer n=19 entity=depth-strategy#4.7 v=no
why: mix-blend-mode paint path; the chunk sets no blend mode
@answer n=20 entity=depth-strategy#4.8 v=no
why: names blending's paint test witnesses, which the chunk neither changes nor relies on
@answer n=21 entity=depth-strategy#4.9 v=no
why: select-list paint rule past opacity and clipping ancestors; the chunk neither calls nor witnesses it
@answer n=22 entity=depth-strategy#5.1 v=no
why: example fixture filter and backdrop values; no chunk test uses that fixture
@answer n=23 entity=depth-strategy#5.2 v=no
why: example fixture shadow and z-index values; no chunk test reads them
@answer n=24 entity=depth-strategy#5.3 v=no
why: example fixture shadow values; no chunk test reads them
@answer n=25 entity=depth-strategy#6.1 v=no
why: record of a search over shadow and z-index handling; no chunk code or test states or witnesses it
