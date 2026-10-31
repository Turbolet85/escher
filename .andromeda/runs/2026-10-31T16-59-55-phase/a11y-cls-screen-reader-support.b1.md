# Classified — a11y · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · yes · an option with no `label` attribute is named by its text, built as this TextRun labelling in accessibility.rs, and the open list's options read that name
screen-reader-support#1.2 · no · aria-label naming of the flight type select is unchanged, since the chunk reads that select only for its options and value
screen-reader-support#1.3 · no · a `<label>` binding names fx-pick on stand_act_select's own fixture, which the chunk does not re-read or move
screen-reader-support#1.4 · yes · the select's options as ListBoxOption nodes under display none, selected from chosen state, named by label or text, and a closed select's options reading the select's client rect, which are the open list's and the snapshot's option reading
screen-reader-support#1.5 · yes · a multiple select's rows reading their own client rects stays as built, which the chunk's snapshot bounds change must keep
screen-reader-support#1.6 · yes · a multiple select's Input and Change entries through multiple_select_values stay as built, while the chunk's write path and the multiple row click fire those events on the same select
screen-reader-support#2.1 · yes · every option element node carries its element id as author_id, and the stand's controls table records select#flight-type, which the chunk's new row moves
screen-reader-support#3.1 · yes · the snapshot reads an option's role and name from the tree, its selected and focused readings from the tree, its enabled and value from the element, and snapshot.rs edits the options' bounds
screen-reader-support#3.2 · no · native checkbox and radio toggled reading on checkable roles, and the chunk adds no checkable control
screen-reader-support#4.1 · yes · the actionable-key rule, where unkeyed_actionable returns nothing on the stand tasks, so options made actionable through the driver on select#flight-type fall under it
screen-reader-support#4.2 · no · cells-grid's aria-label and no-role rule, Cells being outside the chunk
screen-reader-support#4.3 · no · the load page's keyed reading and its rows, which the chunk does not move
screen-reader-support#5.1 · yes · the flight booker's mode reads by selected on its two options with no aria-pressed, and aria-activedescendant stays unread under accessibility_active_descendant_unread, which bounds any tree focus on an option
screen-reader-support#6.1 · yes · element nodes carry their tag name, which identifies the option and select for the hidden-content carve-out and the option reading
screen-reader-support#6.2 · yes · tree focus is written only for a built node and otherwise the window, which the focus agreement at each step with a list open depends on
screen-reader-support#6.3 · no · a role-mapping test's stated goal, which states no rule the chunk's code or tests must honour
screen-reader-support#7.1 · no · accesskit_xplat routing to one EventHandler, a platform adapter the chunk does not build or change
screen-reader-support#8.1 · no · AccessKit adapter creation in blitz-shell, and the chunk builds no adapter
screen-reader-support#8.2 · no · feature gating of screen-reader integration in blitz-shell and the Dioxus crates, unchanged since the chunk adds no adapter
screen-reader-support#9.1 · no · lang and xml:lang element language, and the list adds no document language
screen-reader-support#9.2 · no · dir attribute bidi isolation in default.css, a direction rule outside the list's defaults
screen-reader-support#9.3 · no · html lang attributes of eleven fixtures, fixture markup the chunk does not edit
screen-reader-support#9.4 · no · lang="en" on example pages, fixture markup the chunk does not edit
screen-reader-support#10.1 · no · absence of html lang in other fixtures, fixture markup the chunk does not edit
screen-reader-support#11.1 · no · sr-only tooltip in the GitHub fixture, a fixture the chunk does not reach
