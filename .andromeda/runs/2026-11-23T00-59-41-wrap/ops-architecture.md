@row_set n=1 row=contract/layout/stylo-taffy-exports cell=promise seen=1 detector=D-arch-resources
value: stylo_taffy exports to_taffy_style, TaffyStyloStyle, StyleFlags, Atom, WritingMode and WritingModeExt; its convert module's float and clear converters take the box's direction as a second argument, an ltr flag.
basis: fct-yxmdkbm5
because: The merge changes the public signatures of stylo_taffy's convert::float and convert::clear to take an ltr flag, so the stylo_taffy export contract must state the converters' new form.
@row_add n=1 row=decision/ime-preedit-composition-underline area=established-decisions fact=F1 detector=none
name: IME preedit painted with a composition underline
rule: While an IME composes, the preedit text in a text input is painted with a composition underline in the text colour, in every document, until the composition is committed.
why: an upstream behaviour change taken, ratified for its take
guard: composing_text_is_underlined_until_committed
basis: fct-yxmdkbm5
because: The merge takes upstream's composition underline (E5) as a behaviour for every document, ratified, and no standing row states it.
@row_add n=2 row=decision/logical-float-clear-by-direction area=established-decisions fact=F2 detector=none
name: Logical float and clear sides resolved by direction
rule: Logical float and clear values, inline-start and inline-end, resolve to left or right by the box's direction, left-to-right or right-to-left.
why: an upstream behaviour change taken, ratified for its take
basis: fct-yxmdkbm5
because: The merge takes upstream's logical float and clear sides (E2) as a layout behaviour for every document, ratified, and no standing row states it.
@row_add n=3 row=decision/block-containment-through-display-contents area=established-decisions fact=F3 detector=none
name: Block containment looks through display contents
rule: Block containment looks through display contents, so an element whose display contents child holds a block counts as containing a block, and a display contents element holding only inlines does not break its parent's inline flow.
why: an upstream behaviour change taken, ratified for its take
guard: block_inside_contents_inside_inline_fills_width
basis: fct-yxmdkbm5
because: The merge takes upstream's display contents fixes (E3, E4) to block containment and box construction for every document, ratified, and no standing row states them.
@row_add n=4 row=contract/page-script-dom-api/domtokenlist-replace area=standard-contracts fact=F4 detector=D-arch-resources
boundary: page-script DOM API
item: DOMTokenList.replace
promise: Page scripts get DOMTokenList replace, so element.classList.replace swaps one class token for another as the DOM standard says.
basis: fct-yxmdkbm5
because: The merge adds DOMTokenList.prototype.replace to the JS bootstrap page scripts see (E1), a new page-script API no standing contract row registers.
