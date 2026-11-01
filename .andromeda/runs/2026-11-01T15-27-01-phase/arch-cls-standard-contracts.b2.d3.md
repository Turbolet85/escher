# Classified — arch · standard-contracts.b2

## Answers
standard-contracts#8.1 · yes · NodeData and SpecialElementData variant lists sit in node.rs, a predicted conflict surface, and the RangeInput variant is a match the take's resolutions must cover
standard-contracts#9.1 · no · is_disabled is read in element.rs, which the chunk only uses; the scope names the focus fixup as reading neither disabled nor hidden, and no named behaviour carries this reading
standard-contracts#9.2 · yes · row_disabled is the open list's disabled-row reading beside option_disabled, which the scope keeps unchanged, and the select list's merge surface is re-read row by row
standard-contracts#9.3 · yes · the select model's rules, Typeahead, SelectStep and the readers are the select model the scope keeps unchanged by any resolution, and the multiple select and optgroup rows re-read through them
standard-contracts#9.4 · yes · apply_select_choice is the one write path and option_disabled the disabled reading, both named by the scope as unchanged by any resolution
standard-contracts#9.5 · yes · SelectListView and SelectListRow are the open list's public read types and colours, which the list's paint and merge re-read keep
standard-contracts#9.6 · yes · SelectList.fresh holds a just-opened list against a :focus restyle, the path the take's narrower restyle invalidation lands beside
standard-contracts#9.7 · yes · list placement, MAX_ROWS and sizing are the open list's reconcile behaviour, which the scope keeps unchanged
standard-contracts#9.8 · yes · out_of_flow and its recorded flag live in node.rs, a predicted conflict surface, so a resolution there can move these lines
standard-contracts#9.9 · yes · classes is kept by mutator.rs set_attribute and clear_attribute and read by stylo.rs has_class and each_class, both predicted conflict surfaces
standard-contracts#9.10 · yes · flush_open_state is called from mutator.rs set_attribute, beside the range arm the scope names, and read by stylo.rs:510, both predicted conflict surfaces
standard-contracts#10.1 · yes · is_noop and fetch carry the critical-resource gate in resolve and settle's load-in-flight report, which the scope names through f6edb9701; the rest of the provider surface is not named
standard-contracts#11.1 · no · blitz-net and blitz-html are not reached and the scope names none of their API; the failed-stylesheet behaviour the scope names sits in blitz-dom
standard-contracts#12.1 · yes · paint_scene calls paint_select_list after the root render and before the devtools, the painted-last order the scope names, and carries the password mask the boundary reads keep
standard-contracts#12.2 · yes · paint_select_list is the open list's paint, which the scope keeps painted last
standard-contracts#12.3 · yes · stroke_text_as with RunPaint is the seam the scope names in text.rs, a predicted conflict surface
standard-contracts#13.1 · no · blitz-shell and its redraw send are not reached and the scope names no change to them; the winit pin is not tied to this path
standard-contracts#14.1 · yes · text_contrast's None conditions are a re-read the chunk owes in contrast.rs, and settle and screenshot are the witnesses of the stand checks the scope re-asserts
standard-contracts#15.1 · yes · the one VirtualClock every harness document reads is the document clock invariant the scope keeps through taken code
standard-contracts#16.1 · yes · the stand's options and boots are what every stand check re-asserts in both layout modes; options(incremental) and timer_step carry them
standard-contracts#17.1 · yes · DejaVu Sans is the font every stand boot paints with, the default a re-measured snapshot or screenshot reads
standard-contracts#18.1 · yes · boot_cells mounts Task::Cells, whose cells.rs is a predicted conflict surface, and its screen's snapshot budget is a measured value the chunk re-measures
standard-contracts#18.2 · yes · boot_load's counted load page is the load in flight that settle's report and the critical-resource gate the scope names read
standard-contracts#18.3 · yes · LoadNet answers the one stylesheet fetch the counted load in flight depends on, through the settle report the scope names
standard-contracts#19.1 · yes · the target allowlist and scrub in format.rs, which the chunk re-reads against the taken delta, binds; the init and line-shape rules do not
</content>
