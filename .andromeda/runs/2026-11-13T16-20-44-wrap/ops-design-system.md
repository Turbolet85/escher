@row_add n=1 row=decision/unavailable-families-use-default-font area=typography fact=F1 detector=none
name: Unavailable font families fall back to the default font
rule: When none of an element's listed font families is available, its text lays out in the engine's default font, and font-relative units measure against that font.
why: Taken from upstream, ratified as upstream's behaviour.
guard: font_family_fallback
basis: fct-xve6mmf5, fct-g5byr2t5
because: The upstream take adds the default-font fallback, ratified as upstream's, and no typography row says what text lays out in when no listed family is available.
@row_add n=2 row=decision/font-element-face-and-size area=typography fact=F2 detector=none
name: Font element face and size as presentational hints
rule: The font element's face sets the font family when it is a valid family list, and its size maps to a font-size keyword; a size without digits or a face that is not a valid family list is ignored.
why: Taken from upstream, ratified as upstream's behaviour.
guard: font_element_attributes
basis: fct-xve6mmf5, fct-g5byr2t5
because: The upstream take makes the font element's face and size presentational hints, ratified as upstream's, and no typography row holds it.
@row_add n=3 row=decision/table-cell-spacing-presentational-hints area=spacing fact=F4 detector=none
name: Table cellspacing, cellpadding and valign hints
rule: A table's cellspacing and cellpadding attributes and a cell's valign attribute act as presentational hints, and a change of a table's cellpadding restyles the table's descendants.
why: Taken from upstream, ratified as upstream's behaviour.
basis: fct-xve6mmf5, fct-g5byr2t5
because: The upstream take maps cellspacing, cellpadding and valign to presentational hints, ratified as upstream's, and the spacing rows hold only the body margin attributes.
@row_none n=1 fact=F3 type=4
what: How the cursor walk finds an enclosing link (by DOM ancestors) is a fix inside one mechanism; decision/cursor-resolution already says Pointer inside links and Text over selectable text, so the code holds the rest.
@row_none n=2 fact=F5 type=4
what: An inline box of blocks painting no decorations of its own is how the paint walk is built, with no reason stated; the frame test inline_box_decorations_frame and the code hold it.
