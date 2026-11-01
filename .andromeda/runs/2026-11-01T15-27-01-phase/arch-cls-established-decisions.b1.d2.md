# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · Cargo.toml is reached whole and the take carries upstream's pins as one set: the web_atoms-matched parser crates, taffy 0.15 from the registry, winit `=0.31.0-beta.3`, and no git dependency in the root manifest.
established-decisions#2.1 · yes · Cargo.toml is reached whole and the take's manifest merge can move the dev and named profile block; the dev `line-tables-only` debuginfo governs the blitz-tests binary and the cold baseline the test tallies re-read.
established-decisions#3.1 · yes · Cargo.toml is reached whole; the take must keep the one workspace version and the in-repo `=` pins it carries.
established-decisions#4.1 · yes · the blitz-test-harness screenshot's VelloCpuImageRenderer and opaque-white composite are what the re-measured screenshot pixels read; paint_bench, wpt and rdme are not reached.
established-decisions#5.1 · yes · the `writing-mode` feature, in no default list and forwarded through blitz-dom and the dioxus crates, must be passed in the legs that test writing_mode.rs, a conflict surface whose bounds the take keeps; the other default lists are unread by the scope.
established-decisions#6.1 · yes · incremental layout defaulting to true and the Stylo prefs and no-quirks HTML in stylo.rs and document.rs define the two layout modes the incremental/non-incremental oracle compares after the take.
established-decisions#7.1 · yes · the `<head>` render-blocking gate in mutator.rs and resolve.rs is the critical-resource gate the scope names against f6edb9701 and settle's report of a load in flight.
established-decisions#8.1 · yes · the scrolled-box bounds re-measured by the stand checks read overflow auto as Taffy Scroll, scrollbar_width 0 and the 1/64px CSSOM snap, through get_client_bounding_rect the scope names.
established-decisions#9.1 · yes · the rules live in stylo.rs (Shadow DOM, container queries, visited links, focus pseudo-classes) and construct.rs (pseudo-element content, anonymous table wrappers), both predicted conflict surfaces.
established-decisions#10.1 · yes · scrolling.rs, writing_mode.rs and node.rs are conflict surfaces, and the scope names scroll_into_view, the shared client-rect reader, the padding-box hit stop and the clock surface.
established-decisions#11.1 · yes · the scope names the hit stop, Node::absolute_position, get_client_bounding_rect, the transform-moved client rect, and the inert and disabled reads the take must keep.
established-decisions#11.2 · yes · the scope names the engine-debts merge surface row by row (visible_region, non-invertible hit, focus fixup, focus fallback, toggled, multiple select, optgroup, checkbox and radio change), including 0980cce5f's textual hit_inner conflict.
established-decisions#11.3 · yes · the scope names the document clock invariant through taken code, DocumentConfig.clock and the clock surface in scrolling.rs.
established-decisions#11.4 · yes · the scope names the select model, the one path apply_select_choice, and the select and range arms of mutator.rs, which every resolution keeps.
established-decisions#11.5 · yes · the scope names paint_select_list painted last and the open list's hit answered first; document.rs is in the select-list merge surface the scope re-reads.
established-decisions#11.6 · yes · the close paths on Escape, click, press, wheel, scroll and disable sit in document.rs and pointer.rs, both in the select-list merge surface the scope re-reads row by row.
established-decisions#11.7 · yes · the Tab, Enter, Space and move-key rows in keyboard.rs and pointer.rs are in the select-list merge surface, and select_popup.rs is an edited file whose other rows must keep holding.
established-decisions#11.8 · yes · the rejected builds (list rows as boxes, an overlay hit in hit_inner, list colours in default.css) bind the resolutions of hit_inner and default.css, both conflict surfaces, which must not add them.
established-decisions#11.9 · yes · the highlighted row's unread contrast bears on the text_contrast None conditions and stand_contrast rows the chunk re-reads, and the highlight's absence from the snapshot bears on the dioxus_document.rs resolution.
established-decisions#11.10 · yes · the select list's sixteen-file merge surface is the one the scope names for row-by-row re-read against the take.
established-decisions#11.11 · yes · press_multiple_select_key in keyboard.rs sits in the multiple-select row of the engine-debts merge surface the scope names.
established-decisions#11.12 · yes · click_select_row behind the option arm of handle_click in pointer.rs is the multiple-select pointer row of the engine-debts merge surface the scope names.
established-decisions#11.13 · yes · the upstream behaviours the take carries (the ::before/::after flow and IME in construct.rs and inline/build.rs, the file: script refusal and the CSS accessors in the vibey-script runtime) land in conflict surfaces.
established-decisions#12.1 · yes · hit_with_scrollbar in document.rs answers the open select list first, a rule the scope names, and the overlay-scrollbar thumb-drag hazard governs the driver click the stand checks use.
established-decisions#13.1 · yes · layout/construct.rs is a conflict surface and carries the range input's RangeInputData seeding at construct.rs:417 and 926-944, which the take's construct conflict must keep.
