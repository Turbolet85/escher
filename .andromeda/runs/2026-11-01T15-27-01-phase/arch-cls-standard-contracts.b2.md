# Classified — arch · standard-contracts.b2

## Answers
standard-contracts#8.1 · yes · the NodeData and SpecialElementData variant lists sit in node.rs, a predicted conflict surface, so the take can move them
standard-contracts#9.1 · yes · is_disabled is the focusability reading the panic fix must not take, since the focus fixup reads neither disabled nor hidden
standard-contracts#9.2 · yes · row_disabled is the open select list's reading of a disabled row, which the scope keeps unchanged beside option_disabled
standard-contracts#9.3 · yes · the select model's rules and readers (SelectStep, Typeahead, stepped, matched) are the select model the scope keeps unchanged, on the select_popup key path
standard-contracts#9.4 · yes · apply_select_choice as the one write path and the select cursor are the select model the scope keeps unchanged
standard-contracts#9.5 · yes · the open list's read types and painted colours are the list the scope keeps unchanged; the lib.rs re-export is only a re-check
standard-contracts#9.6 · yes · a just-opened list held against a :focus restyle is open-list behaviour the scope keeps, and upstream's narrower restyle invalidation lands beside it
standard-contracts#9.7 · yes · the list's row cap, placement and width are open-list behaviour the scope keeps unchanged under reconcile_select_list
standard-contracts#9.8 · yes · Node::out_of_flow is written by the style traversal in node.rs, a predicted conflict surface
standard-contracts#9.9 · yes · ElementData::classes is kept in sync by mutator.rs set_attribute and read by stylo.rs, both predicted conflict surfaces
standard-contracts#9.10 · yes · flush_open_state's writes in mutator.rs and the :open read in stylo.rs are both predicted conflict surfaces
standard-contracts#10.1 · yes · NetProvider::fetch and is_noop carry the critical-resource gate in resolve and settle, which f6edb9701 moves, and the counted offline harness
standard-contracts#11.1 · no · blitz-html and blitz-net are neither reached nor named by the scope; the failed-stylesheet behaviour the scope names lives in blitz-dom's resolve and settle
standard-contracts#12.1 · yes · a password input stays masked through paint changes, a boundary read the delta brings, and paint_select_list is painted last per the open list's order
standard-contracts#12.2 · yes · paint_select_list is the open list's painting, which the scope keeps unchanged and the select_popup_painted witness paints
standard-contracts#12.3 · yes · stroke_text_as and RunPaint sit in blitz-paint's text.rs, a predicted conflict surface and a named seam
standard-contracts#13.1 · no · BlitzShellProvider's redraw proxy is in blitz-shell, which the chunk does not reach and no scope clause names
standard-contracts#14.1 · yes · Harness::text_contrast's None conditions are re-read in contrast.rs, and the harness's settle, input helpers and exports are the chunk's witnesses
standard-contracts#15.1 · yes · the harness's VirtualClock through DocumentConfig.clock is the document clock invariant the take must keep
standard-contracts#16.1 · yes · the stand's pinned viewport, offline options and boot functions are inputs of the stand checks re-measured in both layout modes
standard-contracts#17.1 · yes · the DejaVu Sans font_ctx is the family every re-measured stand value and the select_popup_painted witness is measured at
standard-contracts#18.1 · yes · boot_cells' Cells screen passes a snapshot-text budget that a moved pin can shift and the stand checks re-measure; cells.rs is a conflict surface
standard-contracts#18.2 · yes · boot_load's load page and load-done gate are the load-in-flight path settle reports, which f6edb9701 moves
standard-contracts#18.3 · yes · LoadNet overrides no is_noop, so the harness counts its one stylesheet fetch as a load in flight through the critical-resource settle path
standard-contracts#19.1 · yes · the sink's target allowlist and scrub in format.rs, a reached file re-read for its allowlist, bind
