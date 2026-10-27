# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the re-read stand-at-boot lint check measures its findings at the pinned viewport, the bundled font and the default UA sheet (DocumentConfig viewport, font_ctx, ua_stylesheets), and Session::lint refuses on can_paint_text before it reads
standard-contracts#2.1 · no · names the document's clock (scrollbar fade, double-click window, pan and fling timing, smooth-scroll frames), which neither re-read check reads: the boot findings and the chosen-state reading read no time, and the README names the virtual clock by name only
standard-contracts#2.2 · yes · the chosen state the re-read flight-type check asserts is held by the choice writers settle_select_choice, set_select_choice and set_option_chosen (the CHECKED write), which the snapshot's selected reading carries
standard-contracts#3.1 · yes · the builder's option selected read (its chosen state, true or false, with no aria-selected read) is what the re-read flight-type check asserts; the rest of the entity (names, descriptions, shell refresh) is not asserted by either check
standard-contracts#4.1 · no · neither re-read check asserts an event dispatch: the boot findings dispatch no event and the flight-type check asserts a snapshot state, so the default actions and the input and change dispatch are not read
standard-contracts#5.1 · no · scrolling, selection, traversal and hit testing are not among the lint's inputs (its engine readers are text_extent and text_contrast), and neither re-read check scrolls, selects or hits
standard-contracts#6.1 · yes · the lint's findings carry the snapshot's bounds, which read get_client_bounding_rect, so the re-read boot findings (including the flight booker's option overlap) measure this reader's untransformed client rects
standard-contracts#6.2 · yes · Session::lint reads text_extent for its truncation check on the held screen, so the boot findings the re-read stand-at-boot lint check asserts pass through this reader
standard-contracts#6.3 · no · the dropped-node skip in the hit walk is read by no lint defect (the lint has no hit test), and neither re-read check clicks or hits
standard-contracts#7.1 · no · the layout pass's trait surface and the stylo_taffy exports are not read by the lint or either re-read check; the boot geometry is read through the client-rect reader in 6.1
standard-contracts#8.1 · no · node-data variants, serialization and the CSSOM View accessors are not among the lint's or the re-read checks' readers
standard-contracts#9.1 · no · is_disabled gates focus, the pointer and the activation keys, none of which the lint or either re-read check reaches; the snapshot's enabled reads the disabled attribute, not this method
standard-contracts#9.2 · yes · the chosen-state readers is_single_select, is_chosen, chosen_option and settled carry the exactly-one-chosen reading the re-read flight-type check asserts on the flight-type select
standard-contracts#9.3 · no · the out-of-flow flag feeds layout placement and the style reading, which neither re-read check names, and the lint reads no out-of-flow state
standard-contracts#9.4 · no · the class atoms feed Stylo's selector matching of the author sheet, which neither re-read check names; the lint reads computed colour and geometry, not the class storage
standard-contracts#10.1 · no · the provider traits (fetch, is_noop, NetHandler::bytes) are not named by the lint or either re-read check; the load page's one stylesheet is answered by the in-process stand-in LoadNet
standard-contracts#11.1 · no · the blitz-html parser and the blitz-net Provider are not named by the lint or either re-read check, and the stand's load page fetches through its in-process stand-in, not blitz-net
