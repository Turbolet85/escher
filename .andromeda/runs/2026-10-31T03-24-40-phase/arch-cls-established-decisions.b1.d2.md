# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · the chunk bumps no dependency, so a fix must work inside the pinned Taffy 0.15, Parley 0.12 and Stylo-matched markup5ever set
established-decisions#2.1 · no · build profiles are Not needed, and the chunk adds no profile or debug setting
established-decisions#3.1 · no · release versioning is Not needed by this chunk
established-decisions#4.1 · no · renderer backends and the harness screenshot are outside the scope, which names no screenshot, paint_bench or renderer for the rows
established-decisions#5.1 · no · feature defaults are not named by the scope, and the chunk adds or moves no feature
established-decisions#6.1 · yes · headless documents default to ColorScheme::Light, the scheme that the pinned 800x600 Light viewport and the contrast rows run in
established-decisions#7.1 · no · render-blocking stylesheets, link, @import and image loading are not named, resource-loading caps and iframe nesting are Not needed, and the load page's fetch is the in-process stand-in
established-decisions#8.1 · yes · CSSOM geometry snapped to 1/64px is the precision of the client-rect reading that a visible_region fix must stay consistent with and that the transformed_client_rect pin reads
established-decisions#9.1 · yes · 3D transforms dropped and CSS offset unsupported are the limits the pan/zoom fixture stays inside, and :focus-within held on ancestors is what a focus fixup acts on
established-decisions#10.1 · yes · visible_region's scrolling neighbours, the client-rect reader, Node::absolute_position, scroll_into_view and the hit's padding-box stop, bind, and so does the keyboard activation the rows run
established-decisions#11.1 · yes · the record of seven direct engine changes that a fix joins, including the client rect read where transforms move it, the toggle read per role, and the aria-activedescendant and no-area notes stated in covered's meaning
established-decisions#11.2 · no · standing S9 · new engine code reads time only through DocumentConfig.clock, a convention the task lists
established-decisions#11.3 · yes · the select model the chunk limits and re-reads, apply_select_choice, its keys, typeahead and click, and the stated limits on multiple, optgroup, the unpainted popup and the unclicked option
established-decisions#11.4 · yes · :open, the ::before/::after flow vote and the windowed shell's redraw route are three upstream behaviours the chunk adds checks for, kept outside escher's direct-change record
established-decisions#12.1 · yes · a driver click in a just-scrolled box needs an advance past the overlay fade, and hit_with_scrollbar in the reached events/ holds the hazard
established-decisions#13.1 · yes · out-of-flow boxes hit by their containing block and hoisted offsets computed once per geometry generation are what the transform pan/zoom fixture exercises
established-decisions#14.1 · no · replaced-element aspect-ratio sizing is not named, and no replaced element is in the scope
established-decisions#14.2 · yes · the ::before/::after flow vote is what pseudo_element_flow_classification asserts, and the chunk edits its run to both layout modes
established-decisions#14.3 · no · IME enabling at editor construction is not named by the scope, whose focus fixup acts on events, a redraw, :focus-within and interaction state
established-decisions#15.1 · yes · ElementData::is_disabled, the one disabled reading gating Enter and Space activation, and transforms stored in device-pixel space with the translation scaled once
established-decisions#16.1 · yes · inert honoured by presence and out of hit, focus and the accessibility tree, with the focus fixup not built and its residual line, which the inert_subtree pin and a fixup act on
established-decisions#16.2 · yes · set_focus_to and clear_focus hold :focus-within on ancestors and request a redraw, and their FOCUS_WITHIN writes mark no changed-set entry, which a focus fixup must keep
established-decisions#16.3 · yes · a select's choice held as CHECKED state and settled by selectedness in node/select.rs, with the mutator's select records and the builder's option rules, which apply_select_choice must keep
established-decisions#17.1 · yes · the hit that the covered and off-screen refusals reach skips a node dropped since the last resolve, a rule the padding-box stop and option hit must keep
established-decisions#17.2 · yes · :open matches a details or dialog carrying open, an upstream change with no test of its own, which the chunk's :open check pins as a fork rather than a re-pin
