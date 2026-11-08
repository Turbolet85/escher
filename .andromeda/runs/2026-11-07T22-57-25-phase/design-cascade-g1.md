@carried n=1 label=design-system/iconography fact=fct-guxvotwt
why: The reacts-to-a-click field must be read from the handlers the vdom walk finds, never from this cursor reading, so a Pointer link with no click handler and a Default-cursor element carrying one each read by their handlers.
quote: The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise — a pointer over no box included, so the shell keeps a visible arrow there
@carried n=1 run=2 label=design-system fact=fct-hvyxlxhq
quote: The cursor is the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise
@carried n=2 label=design-system/motion fact=fct-z2ugu37c
why: Moving the Cells signals onto the registry must keep its arrow keys' handler calling the instant, synchronous scroll into view, and that handler is one the graph names on its node.
quote: The stand's Cells grid brings its current cell into view with no animation: its arrow keys' handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously (tests/blitz-tests/tests/stand_keyboard.rs:293-335; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
@carried n=3 label=design-system/motion fact=fct-e2e2e6vg
why: The stand's Timer state written through the step a session's caller hands `advance` must, once on the registry, still move by whole ticks together with the clock, and the graph read verb, like `click`, `type` and `press`, must leave the clock where it was.
quote: the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (ratified by the founder, 2026-10-18), while `click`, `type` and `press` leave it where it was
@carried n=3 run=2 label=design-system fact=fct-za7xpowz
quote: settle reads animation time only through it, never advancing it; the driver's `advance` moves it by exactly its `ms` and, through the step a session's caller hands it, the app's own time together, while `click`, `type` and `press` leave it where it was.
@binding n=1
**iconography cursor ↔ motion Cells handler**: the reacts-to-a-click field is read from handlers, not from the cursor (design-system/iconography fct-guxvotwt), and the Cells grid's arrow-key handler (design-system/motion fct-z2ugu37c) is a keyboard handler on the registry-moved stand: the field must say a node reacts to a click only for a click handler, so a key handler alone does not make it read true.
@binding n=2
**motion virtual clock ↔ the registry's link 3**: the write the registry wrapper records for the Timer's state comes through the `advance` step (design-system/motion fct-e2e2e6vg), and recording it must not change by how much or when that state moves, while the graph read itself advances no clock (design-system fct-za7xpowz).
