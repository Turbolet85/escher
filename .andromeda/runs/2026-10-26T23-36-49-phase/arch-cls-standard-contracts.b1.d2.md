# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · `BaseDocument::can_paint_text`, the sans-serif font check the lint's fonts-unavailable refusal runs before any read, so the refactored walk keeps that check first
standard-contracts#2.1 · no · the document clock and its readers (scrollbar fade, double-click window, pan, fling, smooth scroll) are not reached: the scope names no timed reading for the lint, its tests or the re-read rows, and the lint moves no time
standard-contracts#2.2 · no · `select_value` is a control-value reader and the findings hold no control value; the select's choice writers and model are not reached
standard-contracts#3.1 · no · the mutator, selector, CSSOM, resolved-style and form APIs and the accessibility builder's name, description and state reads are not reached, the scope adding no DOM mutation and the findings holding no name, description or state; its BoundingRect type is text_extent's return type, carried by standard-contracts#6.2
standard-contracts#4.1 · no · event dispatch and the range, select, keyboard-activation and click default actions are not reached: the lint dispatches no event and the scope names no event rule for the lint or its re-read rows
standard-contracts#5.1 · no · scrolling and hit testing are listed as not reached, and the lint's readers the scope names are text_extent and text_contrast, not the scroll, visible-region or hit readers
standard-contracts#6.1 · no · the transform-aware client rect is geometry the chunk does not change, and the scope names no transformed node among the lint's or the re-read rows' inputs
standard-contracts#6.2 · yes · `BaseDocument::text_extent`, the lint's read-only text reader (its one non-test caller is `Session::lint`): the refactored walk keeps the call non-mutating and handles its `None` for a node with no inline layout data
standard-contracts#6.3 · no · the hit walk's skip of a dropped node is not reached, the scope listing hit testing as not reached and naming no hit reader for the lint
standard-contracts#7.1 · no · the layout pass, Taffy trait implementations, paint-tree and table APIs and stylo_taffy exports are not reached: the chunk edits no layout code and the lint reads the finished snapshot
standard-contracts#8.1 · no · node data kinds, the range input's value reader, serialization, CSSOM View accessors and the Widget trait are not reached: the scope names none of them for the lint walk or the re-read rows
standard-contracts#9.1 · no · `ElementData::is_disabled` is the focus, pointer and activation gate reading, none of which the scope names; the contrast exemption is stand_contrast's rule, which this reading does not state
standard-contracts#9.2 · no · the crate-private select rules module (choice steps, typeahead, option readers) is not reached; the flight booker's pinned overlap reads the snapshot's option bounds, which this module does not compute
standard-contracts#9.3 · no · `Node::out_of_flow` and its crate-private readers are not reached: the scope names no out-of-flow reading for the lint or its re-read rows, and the chunk edits no style or layout code
standard-contracts#9.4 · no · `ElementData::classes` is not reached: the scope names no class reading for the findings or the re-read rows, and the chunk constructs no `ElementData`
standard-contracts#10.1 · no · the provider traits, `DevtoolSettings` and `Viewport` are supplied unchanged by the stand's boot and its load-page stand-in, and the scope names no fetch, shell or viewport reading the chunk must honour
standard-contracts#11.1 · no · the blitz-html and blitz-net APIs are not reached: the stand's boot is unchanged and the chunk's walk and re-read rows call no parser or fetch the scope names
