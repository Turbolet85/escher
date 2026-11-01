# Classified — arch · existing-scopes.b1

## Answers
existing-scopes#1.1 · no · accesskit_xplat's platform adapters are not reached; the a11y target is proven through the document's tree, not an adapter
existing-scopes#1.2 · no · debug_timer is not reached by any named file or rule
existing-scopes#1.3 · no · blitz-traits' modules are not reached by any named file or rule
existing-scopes#1.4 · yes · lib.rs is edited (orphan comment) and the crate root's `pub use clock` stays; the module list names clock, mutator, scrolling, stylo and layout, where the taken code and the clock invariant land
existing-scopes#1.5 · no · cssom.rs and resolved_style.rs are not reached and no rule names getComputedStyle or CSSOM editing
existing-scopes#1.6 · no · form.rs, iframe.rs and query_selector.rs are not reached and no named rule covers them
existing-scopes#1.7 · yes · document.rs hit testing: hit_with_scrollbar answering the select list first, hit_inner and the focus fixup under inert, and the CSSOM View geometry (absolute bounds, get_client_bounding_rect) that the hit stops keep
existing-scopes#1.8 · yes · accessibility.rs builds the tree that the a11y target's standing checks and accessibility-tree identity read through the Dioxus bridge
existing-scopes#1.9 · yes · events/focus and keyboard carry the focus fixup under inert (merge surface), the tree focus falling back to the window, and Enter and Space activation proven by stand_keyboard
existing-scopes#1.10 · yes · scrolling.rs is a conflict surface holding the clock surface that survives the resolution and the visible-region reader that the hit stops and scroll_into_view depend on
existing-scopes#1.11 · yes · stylo.rs is a conflict surface carrying our `size` hint, which a multiple select's height reads
existing-scopes#1.12 · no · tree.rs, traversal.rs and url.rs are not reached and no named rule covers versioned storage or traversal
existing-scopes#1.13 · yes · construct.rs, inline/build.rs and writing_mode.rs are conflict surfaces; writing_mode.rs holds the bounds the hit stops and scrolled-box bounds keep, and both layout modes must stay identical through the layout and damage changes
existing-scopes#1.14 · yes · node.rs is a conflict surface holding the key-press panic at position_in_containing_block (node.rs:1775) and Node::absolute_position; select.rs and select_list.rs hold the select model and open list the chunk must keep unchanged
existing-scopes#1.15 · no · blitz-html's TreeSink parsing is not reached and no named rule covers it
existing-scopes#1.16 · no · blitz-net fetching is not reached and no named rule covers it
existing-scopes#1.17 · yes · text.rs is a conflict surface holding our `stroke_text_as` seam; render/select_list paints the open list last, and the font-embolden witness runs through blitz-paint's feature
existing-scopes#1.18 · yes · bootstrap.rs and events.rs are conflict surfaces; the document_may_load gate on fetch, import and script src, the bootstrap globals and report_js_error must keep their behaviour
existing-scopes#1.19 · no · blitz-shell's event module test and modules are not reached; the winit pin is unchanged by the default take and no named rule covers the shell's redraw wake
existing-scopes#1.20 · no · the blitz crate's re-export list is not reached and no named rule covers it
existing-scopes#1.21 · no · stylo_taffy's modules are not reached; the pin set binds Cargo manifests, not this crate's modules
existing-scopes#1.22 · no · dioxus-native's module list is not reached and no named rule covers it
existing-scopes#1.23 · yes · dioxus_document.rs is a conflict surface holding the bridge's stable ids, the snapshot family, unkeyed_actionable and the accessibility_tree override that a resolution must keep
existing-scopes#1.24 · yes · contrast.rs is re-read for the None conditions of text_contrast; the harness's one VirtualClock is the clock invariant the take must honour, and screenshot pixels are re-measured
existing-scopes#1.25 · yes · format.rs is re-read: the sink's target allowlist and SAFE_FIELDS and CONTENT_FIELDS must admit only engine and escher_* targets over the taken delta
