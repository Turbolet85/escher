# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the config's `ua_stylesheets` used as given with the engine's default sheet through `DocumentConfig::default()` (Colours), and the changed-set rule (`has_changes`, `take_changed_nodes`) that a list's open and close invalidation relies on
standard-contracts#2.1 · yes · `DocumentConfig.clock` is the one time source for reduced motion and typeahead, and no `Instant::now()` may stand in blitz-dom outside `clock.rs`
standard-contracts#2.2 · yes · `select_value` is read by the bridge and `apply_select_choice`, the choice writers stay crate-private, and `select_typeahead` is the single select's typeahead state
standard-contracts#2.3 · yes · `select_values`, `select_of`, `closed_select_of`, `is_multiple_select` and `clear_select_typeahead` are the named public readers the chunk and the driver use
standard-contracts#2.4 · no · the mutator's record of a `multiple` write concerns multiple selects, which the chunk keeps as built and never writes; `mutator.rs` is not reached
standard-contracts#3.1 · yes · the tree's name sources (an option's `label`, an `optgroup` as `Role::Group`) name the group and option rows, and the `doc.mutate()` methods are how the overlay opens and closes
standard-contracts#3.2 · yes · `accessibility.rs` is reached whole, and the tree's focus write over built nodes binds the open list's focus under SC 2.1.1 and 2.4.3
standard-contracts#3.3 · yes · the Needs name "a native control's state writes": a select, a native control under a non-checkable role, writes no `toggled` and reads no author attribute
standard-contracts#4.1 · yes · `input` then `change` order on a select's choice, the queued Click for Enter and released Space before the default action, and the focus event order are all named by Needs
standard-contracts#4.2 · no · the checkbox and radio arms of `handle_click` are not the select's choice path; the chunk changes neither, and the select's `change` dispatch is covered by #4.1
standard-contracts#5.1 · yes · `scroll_into_view`, `visible_region` and the hit walk's padding-box stop under the overlay are named readings of an open list's option
standard-contracts#6.1 · yes · `get_client_bounding_rect` under transforms is the driver's reading of an option in an open list
standard-contracts#6.2 · yes · `visible_region` under transforms is the driver's in-view reading of an option, named in Needs
standard-contracts#6.3 · no · `text_extent`'s one caller is lint's other check; the chunk edits only lint's `overlaps` exclusion and reads or changes no text extent
standard-contracts#6.4 · yes · the hit walk skips a node dropped since the last resolve, a hit rule the overlay's hit testing must keep
standard-contracts#6.5 · yes · a hit under the overlay takes nothing under a non-invertible transform, as Needs state
standard-contracts#7.1 · no · the layout pass wrappers, Taffy traits, table types and `stylo_taffy` are not reached, and the overlay is painted after layout without calling this API
standard-contracts#8.1 · no · the chunk adds no node kind, special element variant or widget; the select stays an element
standard-contracts#9.1 · yes · `is_disabled` is the one disabled reading that the list's rows and the group's options, and the pointer and key gates, must read
standard-contracts#9.2 · yes · the crate-private `node::select` rules (`SelectStep`, `Typeahead`, `TYPEAHEAD_TIMEOUT` and the readers) hold the single select's keys and typeahead with no public type
standard-contracts#9.3 · yes · `SelectCursor`, `RowGesture`, `option_disabled`, `stepped` and the `select_cursor` field are the rows and gestures the chunk drives, with no public type
standard-contracts#9.4 · no · `out_of_flow` is a layout-side recorded flag that no Needs names and `node.rs` is not reached
standard-contracts#9.5 · no · the `classes` atoms are named by no Needs and by no reached unit
