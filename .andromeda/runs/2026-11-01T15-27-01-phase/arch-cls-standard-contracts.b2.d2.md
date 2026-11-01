# Classified — arch · standard-contracts.b2

## Answers
standard-contracts#8.1 · yes · the NodeData and SpecialElementData enums, incl. RangeInput, sit in node.rs, a predicted conflict surface, so the resolution may move them
standard-contracts#9.1 · yes · is_disabled is the activation-key and focusability gate that the a11y target re-proves through stand_keyboard (Enter and Space)
standard-contracts#9.2 · yes · row_disabled is the select list's disabled-row reading beside option_disabled, which the scope names, inside the select list merge surface re-read row by row
standard-contracts#9.3 · yes · the select model's rules (OptionReading, Typeahead, readers) are the select model the scope keeps unchanged through any resolution
standard-contracts#9.4 · yes · apply_select_choice (the one write path) and option_disabled are named by the scope; select_cursor sits in document.rs
standard-contracts#9.5 · yes · SelectListView and SelectListRow are the open list's read types and colours, and the open list is kept unchanged by the scope
standard-contracts#9.6 · yes · SelectList.fresh is part of the open list the scope keeps unchanged, its click-open against a :focus restyle
standard-contracts#9.7 · yes · the open list's row cap, placement and width are the open list the scope keeps unchanged
standard-contracts#9.8 · yes · Node::out_of_flow and its LayoutPassState reader sit in node.rs and layout/mod.rs, predicted conflict surfaces
standard-contracts#9.9 · yes · ElementData::classes and its set_attribute class arm sit in mutator.rs and stylo.rs, both conflict surfaces
standard-contracts#9.10 · yes · flush_open_state, the open arm in mutator.rs and Stylo's :open read in stylo.rs are conflict surfaces, and the range arm of mutator.rs is named by the scope
standard-contracts#10.1 · yes · NetProvider::fetch and is_noop carry the critical-resource gate in resolve and settle, which the scope names through f6edb9701
standard-contracts#11.1 · yes · blitz-net's fetch result and ProviderError carry the failed-stylesheet outcome that the scope's boundary read (f6edb9701) names
standard-contracts#12.1 · yes · paint_scene calls paint_select_list last, and the password mask through paint changes is named by the scope
standard-contracts#12.2 · yes · paint_select_list paints the open list, which the scope keeps painted last and unchanged
standard-contracts#12.3 · yes · stroke_text_as with RunPaint synthesize_bold is the text.rs seam the scope names, witnessed by the font-embolden run
standard-contracts#13.1 · no · BlitzShellProvider's request_redraw routing is not named by any scope unit, and blitz-shell is not reached
standard-contracts#14.1 · yes · the harness's text_contrast None conditions are re-read, and settle, screenshot and input helpers are what the stand checks drive
standard-contracts#15.1 · yes · the harness's one VirtualClock and tick and advance_time carry the document clock invariant the scope names
standard-contracts#16.1 · yes · seven_guis stand boot and options carry the stand checks in both layout modes and stand_replay that the chunk re-asserts
standard-contracts#17.1 · yes · DejaVu Sans is the family the stand checks and the font-embolden run are measured at, so moved pixels and bounds depend on it
standard-contracts#18.1 · yes · boot_cells mounts the Cells task in cells.rs, a conflict surface, and states the snapshot text budget a moved number may shift
standard-contracts#18.2 · no · boot_load's load page is booted by no scoped check, and the critical-resource gate it exercises is carried by standard-contracts#10.1 and the boundary read
standard-contracts#18.3 · no · LoadNet is the load page's private stand-in, booted by no scoped check, and no scope unit names its answer or delay
standard-contracts#19.1 · yes · the sink's target allowlist and scrub, re-read in format.rs for every new tracing or log call site, are stated in this entity
