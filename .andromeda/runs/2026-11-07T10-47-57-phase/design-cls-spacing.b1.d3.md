@answer n=1 entity=spacing#1.1 v=no
why: rdme spacing tokens in the readme app stylesheet; the chunk sets no spacing and reads no token
@answer n=2 entity=spacing#2.1 v=no
why: browser chrome padding and gaps; the chunk compares documents, element ids and mounted events, not chrome paint
@answer n=3 entity=spacing#2.2 v=no
why: about:history page padding and list spacing is browser page styling the chunk does not touch
@answer n=4 entity=spacing#2.3 v=no
why: rdme markdown body width and padding; no readme document or its paint is in the chunk's scope
@answer n=5 entity=spacing#2.4 v=no
why: a pointer to the stand card and Home padding held in layout-templates; the entity states no value the chunk's code or tests must honour here
@answer n=6 entity=spacing#2.5 v=yes
why: blitz-dom's default body margin lays out every document, and the ported test's get_client_rect and the probe's re-measured layout and snapshot read it
@answer n=7 entity=spacing#2.6 v=no
why: multiple select row padding; no select element is in the chunk's scope
@answer n=8 entity=spacing#2.7 v=no
why: multiple select size hint and overflow; no select element is in the chunk's scope
@answer n=9 entity=spacing#3.1 v=no
why: hspace and vspace margins on embed, img and object; the chunk's scope names no such element
@answer n=10 entity=spacing#3.2 v=no
why: legacy body margin attributes map only when a body carries them; the chunk's scope names no body margin attribute
@answer n=11 entity=spacing#3.3 v=no
why: overlay scrollbar thumb paint geometry; the chunk's MountedData scroll calls carry offset and size, not thumb thickness or length
@answer n=12 entity=spacing#3.4 v=no
why: open select list row limit and engine constants; no select list is touched by the chunk
@answer n=13 entity=spacing#3.5 v=no
why: select list placement inside the viewport; no select list is touched by the chunk
@answer n=14 entity=spacing#3.6 v=no
why: vertical centering of text inside an input content box moves the text, not the input client rect the ported test reads
@answer n=15 entity=spacing#3.7 v=no
why: outside list marker padding is paint of list items, which the chunk does not change
@answer n=16 entity=spacing#3.8 v=no
why: double text decoration line offset is text paint, which the chunk does not change
@answer n=17 entity=spacing#4.1 v=no
why: example page gaps and padding in the examples assets; no example page is in the chunk's scope
@answer n=18 entity=spacing#4.2 v=no
why: graphite example page variables; no example page is in the chunk's scope
@answer n=19 entity=spacing#4.3 v=no
why: hr example page margins; no example page is in the chunk's scope
@answer n=20 entity=spacing#4.4 v=no
why: servo example page columns and buttons; no example page is in the chunk's scope
@answer n=21 entity=spacing#4.5 v=no
why: Preact TodoMVC and reference page layout; no example page is in the chunk's scope
