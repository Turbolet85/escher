# Classified — architecture · standard-contracts.b2

## Answers
standard-contracts#8.1 · yes · add-to · the Widget row at line 82 gains that existing attributes now replay on set_custom_widget, and its custom_widget.rs and element.rs digits move
standard-contracts#9.1 · yes · stale · element.rs is a reached merge surface, so the is_disabled citation at element.rs:516-520 moves
standard-contracts#9.2 · no · row_disabled sits in select_list.rs, which the change does not reach, and its beside-claim holds
standard-contracts#9.3 · no · node::select and node/mod.rs are not reached, so the select module's readers stand
standard-contracts#9.4 · yes · stale · document.rs is a reached merge surface and holds the select_cursor field, and its claim is not shown to hold on the changed file
standard-contracts#9.5 · yes · stale · the lib.rs:106-109 re-export citation moves, lib.rs being reached
standard-contracts#9.6 · yes · stale · the select_popup.rs:1101-1118 citation moves, select_popup.rs being reached
standard-contracts#9.7 · no · select_list.rs:218-263 sits in the unreached select_list module, and the rows, placement and MAX_ROWS hold
standard-contracts#9.8 · yes · add-to · F20 joins beside out_of_flow as the Node flag and public reader, and the node.rs citations move through a reached merge surface
standard-contracts#9.9 · yes · stale · the citations at element.rs, mutator.rs and stylo.rs, all reached merge surfaces, move
standard-contracts#9.10 · yes · stale · the citations at element.rs, mutator.rs and stylo.rs, all reached merge surfaces, move
standard-contracts#10.1 · yes · add-to · F13 joins beside NetHandler::bytes as the new error method with a default that does nothing, and the net.rs and render_blocking_stylesheet.rs digits move
standard-contracts#11.1 · yes · add-to · F13 joins the blitz-net fetch row as its error arm calling NetHandler::error with the waker after the handler, and the lib.rs:279-318 digits move
standard-contracts#12.1 · yes · add-to · F16 joins the paint order, the blend layer pushed in render_element and popped last so paint_select_list still paints last past it, and the lib.rs:44-52 and render.rs:248-249 digits move
standard-contracts#12.2 · no · render/select_list.rs is not reached, and its non-finite, rows and thumb claims hold
standard-contracts#12.3 · no · text.rs is not reached and the font-embolden feature line stays unmoved
standard-contracts#13.1 · no · blitz-shell's BlitzShellProvider and window.rs are not reached
standard-contracts#14.1 · yes · add-to · F19 adds a blended element to text_contrast's None list at line 131, F15 ends a failed load's wait through the CountedHandler error forward at line 135, and the contrast.rs digits move
standard-contracts#15.1 · no · harness.rs is not reached, and neither text_contrast's new None read nor settle's load-report change moves the clock
standard-contracts#16.1 · no · examples/seven_guis holds no diff, so the offline options and the boots hold
standard-contracts#17.1 · no · the seven_guis stand and its Cargo.toml are not reached, so the DejaVu Sans registration holds
standard-contracts#18.1 · no · examples/seven_guis holds no diff, so boot_cells stands
standard-contracts#18.2 · no · examples/seven_guis holds no diff, and the Load wait touch sits at line 227, outside this batch
standard-contracts#18.3 · no · examples/seven_guis holds no diff, and LoadNet implements NetProvider, which the change did not alter
standard-contracts#19.1 · no · packages/escher-telemetry holds no diff, so the bootstrap and scrub set stand
