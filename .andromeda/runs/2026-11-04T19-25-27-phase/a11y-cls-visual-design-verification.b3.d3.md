@answer n=1 entity=visual-design-verification#8.6 v=yes
why: the probe's row click on the CRUD listbox (named in scope as the write path for selection) lands in a shown overlay scrollbar band right after a scroll, so the probe must advance past the fade before that click
@answer n=2 entity=visual-design-verification#8.7 v=no
why: concerns the open select popup painting over elements and clicks refused as covered; the probe opens no select and the CRUD listbox is not a select list (not_needed item 2)
@answer n=3 entity=visual-design-verification#8.8 v=no
why: paint order of the open select list under mix-blend-mode and isolation; the probe reads no select list and reads no paint
@answer n=4 entity=visual-design-verification#8.9 v=no
why: devtools layout outlines and hover/node highlight overlay settings; the probe enables no devtools overlay and reads no paint or hover highlight
