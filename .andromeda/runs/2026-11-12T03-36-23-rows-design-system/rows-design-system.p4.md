@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=5
@row_none n=3 rec=3 type=7
@row_none n=4 rec=4 type=7
@row_none n=5 rec=5 type=7
@row_none n=6 rec=6 type=7
@row_add n=1 row=decision/cursor-resolution area=iconography rec=7
name: Cursor resolution order
rule: The cursor is taken from the CSS cursor keyword, else Text for text inputs, Pointer inside links, Text over selectable text, and Default otherwise, a pointer over no box included, so the shell keeps a visible arrow there.
why: Taken from upstream's #1220, ratified.
@row_add n=2 row=decision/cursor-icon-crate area=iconography rec=8
name: cursor-icon crate for cursor icons
rule: Cursor icons are cursor-icon crate values, CSS cursor keywords map one-to-one to them, and cursor: none maps to no cursor.
@row_add n=3 row=contract/shell-provider/set-cursor area=iconography rec=8
boundary: ShellProvider
item: set_cursor
promise: Takes an optional CursorIcon; None hides the cursor and resets it to Default.
@row_none n=7 rec=9 type=4
relied: The favicon read returns the href of the first link element whose rel contains icon.
@row_none n=8 rec=10 type=5
@row_none n=9 rec=11 type=5
@row_none n=10 rec=12 type=5
@row_none n=11 rec=13 type=5
@row_none n=12 rec=14 type=5
@row_none n=13 rec=15 type=7
@row_none n=14 rec=16 type=7
@row_none n=15 rec=17 type=5
@row_none n=16 rec=18 type=4
relied: scroll-behavior smooth makes Auto scrolls animate, and a wheel over a scroller cancels an in-progress smooth scroll.
@row_none n=17 rec=19 type=4
relied: A wheel over the open select list scrolls the list, a wheel elsewhere closes it first, and a key brings the current row into view at once.
@row_none n=18 rec=20 type=4
@row_none n=19 rec=21 type=4
@row_add n=4 row=decision/into-view-scroll-nested-boxes-instant area=motion rec=22
name: Into-view scroll is instant in nested boxes
rule: A programmatic into-view scroll writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked; the requested behaviour, Smooth included, applies to the viewport alone.
why: The document holds one scroll animation at a time.
@row_add n=5 row=decision/driver-scroll-instant area=motion rec=22
name: Driver scroll is instant
rule: The driver's scroll is instant in every box and in the viewport, made inside one settled step.
why: Ratified.
@row_none n=20 rec=23 type=4
@row_add n=6 row=decision/overlay-scrollbar-fade-timings area=motion rec=24
name: Overlay scrollbar fade timings
rule: Overlay scrollbars show at full opacity on scroll, stay opaque for 500 ms after last activity, then fade linearly over 200 ms.
why: Documented as Chromium's overlay timings.
@row_none n=21 rec=25 type=4
relied: Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade first.
@row_none n=22 rec=26 type=4
@row_none n=23 rec=27 type=4
relied: A harness settle neither waits on nor advances any animating member and returns Settled with animating reading the document's flag.
@row_add n=7 row=decision/driver-advance-moves-one-clock area=motion rec=28
name: Driver advance moves the one virtual clock
rule: The driver's advance moves the harness virtual clock by exactly its ms and the app's own time together, while click, type and press leave the clock where it was.
why: Ratified.
@row_none n=24 rec=29 type=4
relied: The select list has no motion, so a screen read after a step reads the new state without an advance.
@row_none n=25 rec=30 type=4
relied: While an acting step waits on a load, the harness virtual clock stands still, so no timer or animation runs during the wait.
@row_none n=26 rec=31 type=4
@row_none n=27 rec=32 type=4
@row_none n=28 rec=33 type=5
@row_add n=8 row=absence/reduced-motion-handling area=motion rec=34
capability: Reduced-motion handling
@row_none n=29 rec=35 type=7
@row_none n=30 rec=36 type=5
@row_none n=31 rec=37 type=5
@row_none n=32 rec=38 type=5
@row_none n=33 rec=39 type=5
@row_none n=34 rec=40 type=5
@row_none n=35 rec=41 type=5
@row_none n=36 rec=42 type=5
@row_none n=37 rec=43 type=5
@row_none n=38 rec=44 type=5
@row_none n=39 rec=45 type=S
@row_none n=40 rec=46 type=5
@row_none n=41 rec=47 type=5
@row_none n=42 rec=48 type=4
@row_none n=43 rec=49 type=4
@row_add n=9 row=decision/body-right-bottom-margin-ignored area=spacing rec=50
name: Body rightmargin and bottommargin ignored
rule: The body attributes rightmargin and bottommargin are deliberately ignored, while marginwidth, marginheight, leftmargin and topmargin map to pixel margins.
@row_none n=44 rec=51 type=4
@row_none n=45 rec=52 type=4
@row_none n=46 rec=53 type=4
relied: An open select list is placed inside the viewport, at least its select's width and at most the viewport's, every edge finite.
@row_none n=47 rec=54 type=4
@row_none n=48 rec=55 type=4
