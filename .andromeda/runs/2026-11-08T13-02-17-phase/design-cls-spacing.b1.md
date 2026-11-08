@answer n=1 entity=spacing#1.1 v=no
why: the rdme --base-size tokens belong to the readme app's CSS; the driver read paints nothing and sets no spacing
@answer n=2 entity=spacing#2.1 v=no
why: browser urlbar, tab and menu padding; the browser app is not touched by the chunk
@answer n=3 entity=spacing#2.2 v=no
why: about:history padding and list item spacing; the browser app is not touched by the chunk
@answer n=4 entity=spacing#2.3 v=no
why: rdme markdown body max-width and padding; the readme app is not touched by the chunk
@answer n=5 entity=spacing#2.4 v=no
why: a pointer to layout-templates for the stand's card and Home padding; it states no value here and the read reports ids, verbs and causes, not the stand's padding
@answer n=6 entity=spacing#2.5 v=no
why: blitz-dom default body margin is an engine default the scope does not name, and the chunk re-pins no row that reads it
@answer n=7 entity=spacing#2.6 v=no
why: horizontal padding of multiple select rows is paint geometry; the owner-served rows are answered by the select list and the chunk re-measures no row that reads this padding
@answer n=8 entity=spacing#2.7 v=yes
why: select[multiple][size] takes overflow-y auto, a paint-clipping box the scope names in the hit-testing rule; it decides which options of a sized select are off-screen or covered
@answer n=9 entity=spacing#3.1 v=no
why: hspace and vspace margins on embed, img and object; no image or embed verb or cause is in scope
@answer n=10 entity=spacing#3.2 v=no
why: legacy body marginwidth and topmargin attributes; the scope does not name them and the read reaches no body margin
@answer n=11 entity=spacing#3.3 v=no
why: overlay scrollbar thumb geometry is paint; the scope names overflow clipping and select rows, not scrollbar hits, so the covered check does not read it
@answer n=12 entity=spacing#3.4 v=yes
why: an open select list shows at most MAX_ROWS 20 rows and scrolls past them; the off-screen cause for owner-served select rows and the into-view scroll of that list follow this visible-row limit
@answer n=13 entity=spacing#3.5 v=yes
why: place keeps an open select list inside the viewport, so whether an owner-served row is off-screen is read against that placement, and the stand proof of off-screen for select rows relies on it
@answer n=14 entity=spacing#3.6 v=no
why: vertical centering of a single-line text input's text within its content box moves text, not the box that the hit walk and the verbs read
@answer n=15 entity=spacing#3.7 v=no
why: list-marker padding is paint geometry for outside markers; the scope does not name markers
@answer n=16 entity=spacing#3.8 v=no
why: double text decoration line offset is paint; the read reaches no text decoration
@answer n=17 entity=spacing#4.1 v=no
why: gaps and body padding in examples/assets are paint values of example pages; the chunk's new fixture does not reuse them
@answer n=18 entity=spacing#4.2 v=no
why: graphite example root variables are paint values of an example page; no read of the chunk reaches them
@answer n=19 entity=spacing#4.3 v=no
why: hr margin and body padding of an example page; no read of the chunk reaches them
@answer n=20 entity=spacing#4.4 v=no
why: servo.css column gap, container and button padding of an example stylesheet; no read of the chunk reaches them
@answer n=21 entity=spacing#4.5 v=no
why: Preact TodoMVC and reference page max-width and padding of example pages; no read of the chunk reaches them
