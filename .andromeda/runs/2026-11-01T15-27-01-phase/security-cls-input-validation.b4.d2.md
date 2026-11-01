# Classified — security · input-validation.b4

## Answers
input-validation#1.71 · yes · the range arms in the conflict surface mutator.rs (value re-seed, min/max/step re-sanitize, lines 365-398 and 541-548) bind, as the Needs keep the range arm within its rules
input-validation#1.72 · no · keyboard.rs and pointer.rs are not reached and no Need names the Enter or Space Click dispatch; the key-press panic is a separate recorded path
input-validation#1.73 · no · the checkbox and radio change arms sit in pointer.rs, not a reached file, and no Need names them
input-validation#1.74 · yes · the single select's keys and typeahead are the select list's keys the Needs keep unchanged, and its selectedness rules sit in the conflict surface mutator.rs (lines 1262-1295)
input-validation#1.75 · no · the list's state field and reconcile are neither keys nor painted labels, the two behaviours the Needs keep; document.rs is not reached
input-validation#1.76 · yes · an open list's keys (press_open_list_key, Tab's choose-and-close, the single write path apply_select_choice) are the select list's keys the Needs keep, read row by row
input-validation#1.77 · no · the pointer press and outside-close behaviour is neither keys nor painted labels, and no Need names it; the skipped select_popup row is keyed to the panic, not to this
input-validation#1.78 · yes · the open list's painted box and rows (finite box, no panic on hostile size or label length, MAX_ROWS 20 with scroll) are the select list's painted labels the Needs keep unchanged
input-validation#1.79 · yes · the multiple select's keys (press_multiple_select_key) and clicks are the select list's keys, read row by row on the surface
input-validation#1.80 · yes · the optgroup arm at mutator.rs:1253-1257, a conflict surface, and the group header painted in an open list are the select list's painted labels
input-validation#1.81 · yes · multiple_select_values in the conflict surface dioxus_document.rs (lines 31-45) carries the chosen set into the input and change values
input-validation#1.82 · no · the driver's type verb path (execute.rs focus_select and the schema help) is verb schema the chunk does not edit; the select keys it reaches are carried by 1.74 and 1.79
input-validation#1.83 · no · the public readers closed_select_of, select_of and is_select are neither keys nor painted labels, and their files are not reached
input-validation#1.84 · yes · SelectListRow.label holds page text that is never logged, a log-content rule the chunk's sink re-read against the taken delta must witness
input-validation#1.85 · yes · the password and file value mask in the snapshot, diff, driver and MCP carriers, and its bullets painted through text.rs, are the masking Need on the conflicted text and snapshot paths
input-validation#1.86 · no · aspect-ratio clamping in layout/replaced.rs is neither a conflict nor a merge surface and no Need names it
input-validation#1.87 · no · the inline box height clamp sits in layout/inline/layout.rs, not a reached file, and no Need names it
input-validation#1.88 · no · text_extent is a read-only reader used by lint, which the chunk does not change, and no Need names it
input-validation#1.89 · yes · visible_region sits in scrolling.rs, a conflict surface, and the Needs keep scrolled-box bounds as recorded through its resolution
input-validation#1.90 · yes · culling of non-finite geometry before paint is the paint-value rule the Needs apply to the new page-CSS paint properties, and the culling bounds ae774d7f4 and e42ee1f25 move sit in the conflict files
input-validation#1.91 · yes · paint_select_list's non-finite box and scrollport-only rows are the select list's painted box and labels the Needs keep unchanged
input-validation#1.92 · yes · Node::hit_inner's non-invertible answer sits in node.rs, a conflict surface, and the Needs require the 0980cce5f hit_inner resolution to keep it
input-validation#1.93 · yes · the percentage overflow clamp is the overflow-clamped paint-value rule the Needs name for the new page-CSS paint properties
input-validation#1.94 · no · radius scaling is not among the paint-value rules the Needs name (non-finite culling, overflow clamping), and css_box.rs is not reached
input-validation#1.95 · no · the hidden-input paint skip sits in blitz-paint render.rs, not a reached file, and no Need names it
