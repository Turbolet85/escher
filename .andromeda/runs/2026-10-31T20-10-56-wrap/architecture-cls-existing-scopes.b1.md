# Classified — architecture · existing-scopes.b1

## Answers
existing-scopes#1.1 · no · accesskit_xplat is not among the reached files and no crate, dependency or module of it moved
existing-scopes#1.2 · no · debug_timer is not among the reached files and the session touches none of its modules
existing-scopes#1.3 · no · blitz-traits is not among the reached files and none of its modules moved
existing-scopes#1.4 · yes · stale · document.rs and lib.rs are reached whole, so the cited document.rs:4102-4108 and lib.rs:37-87 can have moved past the new insertions
existing-scopes#1.5 · no · cssom.rs and resolved_style.rs are not among the reached files and their cited ranges hold
existing-scopes#1.6 · no · form.rs, iframe.rs and query_selector.rs are not among the reached files and their claims hold
existing-scopes#1.7 · yes · stale · document.rs is reached whole and its cited hit, hover, focus, scrollbar and geometry ranges sit past the insertions at 1408-1410 and 2404-2406, so the citations moved; hit_with_scrollbar now answers an open list first
existing-scopes#1.8 · yes · stale · accessibility.rs is reached whole and its tree-building citation at 10-279 can have moved with the change
existing-scopes#1.9 · no · events/mod.rs is not among the reached files and the submodule list it declares (driver, focus, ime, keyboard, pointer) is unchanged
existing-scopes#1.10 · yes · stale · scrolling.rs is reached whole and its into-view and visible-region reader at 655-796 now answers an open list's row, so the reading and the citation moved
existing-scopes#1.11 · yes · stale · stylo.rs is reached whole and its edited span at 1124-1139 is the open list's restyle hold, so the file's reading can have moved beyond its line-1 citation
existing-scopes#1.12 · no · tree.rs, traversal.rs and url.rs are not among the reached files and their claims hold
existing-scopes#1.13 · no · no layout module is among the reached files and the list is no layout box, so the layout claims hold
existing-scopes#1.14 · yes · add-to · F27: node::select_list, the list's state and geometry with 6 unit tests, joins the node module beside the crate-private select rules, and node/mod.rs is reached so its module lines moved
existing-scopes#1.15 · no · blitz-html is not among the reached files and was not edited
existing-scopes#1.16 · no · blitz-net is not among the reached files and was not edited
existing-scopes#1.17 · yes · add-to · F27: blitz-paint's render::select_list, which paints the list, joins the render module beside the other paint modules
existing-scopes#1.18 · no · blitz-vibey-script is not among the reached files and was not edited
existing-scopes#1.19 · no · blitz-shell is absent from the changed files
existing-scopes#1.20 · no · packages/blitz/src/lib.rs is not among the reached files and its re-exports are unchanged
existing-scopes#1.21 · no · stylo_taffy is not among the reached files and no layout module moved
existing-scopes#1.22 · no · packages/dioxus-native is absent from the changed files
existing-scopes#1.23 · yes · add-to · F18: dioxus-native-dom gains the public re-exported is_select beside closed_select_of and is_multiple_select in the snapshot re-exports, and lib.rs is reached
existing-scopes#1.24 · no · blitz-test-harness was not edited
existing-scopes#1.25 · no · escher-telemetry was not edited and no log field or sink moved
