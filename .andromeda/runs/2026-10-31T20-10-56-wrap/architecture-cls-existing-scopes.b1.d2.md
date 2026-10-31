# Classified — architecture · existing-scopes.b1

## Answers
existing-scopes#1.1 · no · packages/accesskit_xplat is not among the reached files or the changed set
existing-scopes#1.2 · no · packages/debug_timer is not among the reached files or the changed set
existing-scopes#1.3 · no · packages/blitz-traits is not among the reached files; its module list is unchanged
existing-scopes#1.4 · yes · stale · document.rs is reached and a merge surface (hit_inner's upstream conflict note), so its cfg(test) child-module citations at 4102-4108 can move past the 761-1089 insertions; lib.rs is reached and gains crate-root re-exports, so the module list at 37-87 can move
existing-scopes#1.5 · no · cssom.rs and resolved_style.rs are not reached; their claims hold on unchanged files
existing-scopes#1.6 · no · form.rs, iframe.rs and query_selector.rs are not reached; their claims hold on unchanged files
existing-scopes#1.7 · yes · stale · document.rs is reached and a merge surface; hit testing now answers an open list first through hit_with_scrollbar, and the cited hit, hover, focus, scrollbar and geometry ranges can move past the 761-1089 insertions
existing-scopes#1.8 · yes · stale · accessibility.rs is reached and the list's aria state and its absence from the tree change what the tree-building reading covers; the cited range 10-279 can move; debug.rs is not reached
existing-scopes#1.9 · no · events/mod.rs, which lists the submodules, is not reached and no submodule was added; keyboard.rs and pointer.rs gain behaviour, not names
existing-scopes#1.10 · yes · stale · scrolling.rs is reached: into-view scrolling now has an exception for a row of an open list, scroll_into_view writes the list's scroll_y and returns, and visible_region answers the list's scrollport for a row; the cited 655-796 range can move
existing-scopes#1.11 · no · stylo.rs:1 still names Servo styling participation and its line-1 citation is unmoved (the change's edits are at stylo.rs:1124-1139); the other stylo files are not reached
existing-scopes#1.12 · no · tree.rs, traversal.rs and url.rs are not reached
existing-scopes#1.13 · no · the list is no layout box and no layout module is reached
existing-scopes#1.14 · yes · add-to · F27: node::select_list holds the list's state and geometry with six unit tests, beside the crate-private select rules; node/mod.rs, which lists the node submodules at 3-16, gains its select_list line
existing-scopes#1.15 · no · blitz-html is not reached
existing-scopes#1.16 · no · blitz-net is not reached
existing-scopes#1.17 · yes · add-to · F27: render::select_list paints the list as a new per-concern module under render, which render.rs (line 7) declares; blitz-paint's render.rs and text.rs are reached
existing-scopes#1.18 · no · blitz-vibey-script is not reached
existing-scopes#1.19 · no · blitz-shell is absent from the changed files
existing-scopes#1.20 · no · packages/blitz/src/lib.rs is not reached and its re-exports are unchanged
existing-scopes#1.21 · no · stylo_taffy is not reached and no layout box moved
existing-scopes#1.22 · no · packages/dioxus-native is absent from the changed files
existing-scopes#1.23 · yes · add-to · F18: dioxus-native-dom gains the public re-exported is_select beside closed_select_of and is_multiple_select in the snapshot re-exports; snapshot.rs and lib.rs are reached
existing-scopes#1.24 · no · packages/blitz-test-harness was not edited and no reached file is in it
existing-scopes#1.25 · no · escher-telemetry is not reached and no log field or sink moved
