# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · can_paint_text and layout_as_text_input are named Needs of the document core contract, the password caret witnessed by the password checks
standard-contracts#2.1 · yes · DocumentConfig.clock and the VirtualClock that tick, advance_time and the driver's advance move are named Needs, witnessed by the virtual clock checks
standard-contracts#2.2 · yes · select_value is the public reader the snapshot's select value and the select witnesses rest on, a named Need
standard-contracts#3.1 · yes · the accessibility builder's name and state reads (aria-labelledby, aria-selected, aria-pressed, aria-checked, aria-invalid) are what the Dioxus snapshot reads and the stand snapshot checks prove, a named Need
standard-contracts#4.1 · yes · the event dispatch behind the range-input, select and keyboard-activation witnesses (input then change, range default actions, Enter and released Space as a queued Click) is a named Need
standard-contracts#5.1 · yes · scroll_into_view through every scrolling box, visible_region and the hit stopping at a clipping box's padding box are named Needs for the text-extent checks and the covered refusal
standard-contracts#6.1 · yes · get_client_bounding_rect reading an element where its transforms move it is a named Need, and the snapshot's bounds and the lint's readings read through it
standard-contracts#6.2 · yes · text_extent and its stated limits (RTL and vertical text as LTR lines, a transformed box at its client rect) are asserted by the chunk's new checks, and packages/blitz-dom/src/text_extent.rs is a reached file
standard-contracts#6.3 · no · the walk's skip of a child dropped since the last resolve is named by no Need, and node.rs and paint_tree.rs, where its rule lives, are not reached files
standard-contracts#7.1 · no · LayoutPassState, the Taffy traits and the table, replaced and paint-tree API are layout's crate-private surface, named by no Need, table layout is Not-needed, and no reached file holds layout
standard-contracts#8.1 · yes · RangeInputData is named by the nodes-and-widgets Needs behind the range-input witnesses, and ElementData's range_input_value is the snapshot's reading of a range input
standard-contracts#9.1 · yes · ElementData::is_disabled is a named Need, the one reading focusability, the pointer gates and the activation keys share
standard-contracts#9.2 · yes · the select model's rules (the next, previous, first and last steps and the 1 s typeahead on the document's clock) are the select witnesses' named Need
standard-contracts#9.3 · no · the recorded out-of-flow flag is layout's internal input, read only by LayoutPassState and unreached callers, and no Need names it or its write path
standard-contracts#9.4 · no · the classes cache and its set_attribute arm are styling's own sync, named by no Need, and element.rs, mutator.rs and stylo.rs are not reached files
standard-contracts#10.1 · yes · the Viewport's pinned 800x600 Light window and the held network stand-in HeldNet, which stands in for a NetProvider, are named by the offline stand and fixture boot
standard-contracts#11.1 · no · HTML parsing and blitz-net networking are excluded by the Not-needed list, the stand and fixtures boot offline, and blitz-html and blitz-net are not reached files
