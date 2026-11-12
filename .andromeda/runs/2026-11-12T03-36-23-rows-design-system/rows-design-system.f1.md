@row_add n=1 row=decision/driver-scroll-instant area=motion-calibrated-to-expression-level-unrecorded rec=1,2
name: Driver scroll is instant
rule: The driver's scroll is instant in every scrolling box and in the viewport, made inside one settled step.
why: Ratified.
@row_add n=2 row=absence/reduced-motion-handling area=motion-calibrated-to-expression-level-unrecorded rec=3,4
capability: Reduced-motion handling
@row_add n=3 row=decision/cursor-resolution area=iconography rec=5,6
name: Cursor resolution order
rule: The cursor is taken from the CSS cursor keyword, else Text for text inputs, Pointer inside links, Text over selectable text, and Default otherwise, a pointer over no box included, so the shell keeps a visible arrow there.
why: Taken from upstream's #1220, ratified.
