# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · the html5ever, skrifa, svgtypes and winit pin couplings and the taffy and parley registry pins live in Cargo.toml, a merge surface a take can move
established-decisions#2.1 · yes · the dev profile's debuginfo setting and the named profiles live in Cargo.toml:208-250, a merge surface a take can move, so the Not needed list does not drop it
established-decisions#3.1 · yes · the one workspace version and the in-repo = pins live in Cargo.toml:34-60, a merge surface a take can move, so the Not needed list does not drop it
established-decisions#4.1 · yes · blitz-test-harness's screenshot renderer and its opaque-white composite read the screenshot pixels the chunk re-measures; the paint_bench, wpt and rdme selections are not reached
established-decisions#5.1 · yes · the blitz-shell and dioxus-native default lists sit in their Cargo.toml merge surfaces, which the clipboard move edits
established-decisions#6.1 · yes · the incremental default, CSS media type, no-quirks mode and Stylo prefs sit in document.rs (merge surface), which the both-modes pipeline re-run reads
established-decisions#7.1 · yes · the render-blocking, iframe-depth and import-depth caps and the image-cache rule sit in mutator.rs (conflict surface) and document.rs (merge surface)
established-decisions#8.1 · yes · the CSSOM 1/64px snap at document.rs:3119-3124 (merge surface) binds, and the stylo_taffy conversions it names feed the stand bounds the chunk re-measures
established-decisions#9.1 · yes · the unsupported-feature gates in stylo.rs (merge surface) and the pseudo-element string-only and anonymous-table rules in construct.rs (conflict surface) bind
established-decisions#10.1 · yes · the scroll_into_view, client-rect and visible-region rules in scrolling.rs and node/node.rs (conflict surfaces) and the scrolled-box bounds in writing_mode.rs bind
established-decisions#11.1 · yes · the inert, contain-paint hit-stop and absolute-position rules sit in node/node.rs (conflict surface), the password caret click in events/pointer.rs (merge surface), and the client rect in scrolling.rs and writing_mode.rs; the chunk re-reads their merge surfaces
established-decisions#11.2 · yes · the document clock is the invariant the chunk re-reads in clock.rs, and its reads in scrolling.rs (conflict surface) and document.rs and events/pointer.rs (merge surfaces) bind
established-decisions#11.3 · yes · the select model's one choice path apply_select_choice runs through mutator.rs and blitz-vibey-script runtime.rs (conflict surfaces) and document.rs, events/pointer.rs, element.rs and blitz-traits events.rs (merge surfaces)
established-decisions#12.1 · yes · the overlay-scrollbar hit in events/pointer.rs (merge surface) binds, and stand_cells_served, which the chunk re-runs, witnesses it
established-decisions#13.1 · yes · construct.rs (conflict surface) gives text inputs, checkboxes, radios and range inputs their control data at construction, and node/text.rs (merge surface) sets the editor size
established-decisions#14.1 · no · layout/replaced.rs is not a reached file, and no scope behaviour or re-measured value sizes a replaced box by aspect-ratio, so it is a re-check every change owes
established-decisions#15.1 · yes · the presence reading of disabled in node/element.rs and events/pointer.rs and the Dioxus removal mapping in dioxus_document.rs (merge surfaces) bind
established-decisions#16.1 · yes · the inert hit, tree and focus rules in node/node.rs (conflict surface) bind, and inert_subtree.rs is a witness the chunk re-runs
established-decisions#16.2 · yes · the focus-within rules in document.rs (merge surface) and mutator.rs (conflict surface) bind
established-decisions#16.3 · yes · the select CHECKED-state rules in mutator.rs (conflict surface) and element.rs (merge surface) bind
established-decisions#17.1 · yes · the hit test's skip of a node dropped since the last resolve sits in node/node.rs (conflict surface), the hit-test contract the Needs state, and binds
established-decisions#18.1 · yes · element.rs (merge surface) holds the template fragment at :89-90 and the feature-gated hit tests at :353-354 and :1043-1069 that bind; the parser and config.rs parts are not reached
established-decisions#19.1 · yes · window.rs, lib.rs and blitz-traits events.rs (merge surfaces) hold the windowing, event-loop and winit-free IME rules that bind; the accesskit_xplat part is not reached
established-decisions#20.1 · yes · the re-measured screenshot pixels read blitz-paint's border, mask-isolation and layer output, and document.rs (merge surface) carries layout_as_text_input, which the password mask lays out through
