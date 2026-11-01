# Classified — architecture · existing-scopes.b1

## Answers
existing-scopes#1.1 · no · accesskit_xplat is not reached, and no file under packages/accesskit_xplat changed
existing-scopes#1.2 · no · debug_timer is not reached by the session
existing-scopes#1.3 · no · the module list in packages/blitz-traits/src/lib.rs (not reached) still holds, since net.rs gained a trait method and no module
existing-scopes#1.4 · yes · stale · document.rs (a merge surface) and lib.rs are reached, so the cited line numbers 37-87 and 4371-4377 move
existing-scopes#1.5 · yes · stale · resolved_style.rs is reached and the sub-pixel width and height reads move its cited lines 161-343
existing-scopes#1.6 · no · form.rs, iframe.rs and query_selector.rs are not reached and their cited readings hold
existing-scopes#1.7 · yes · stale · document.rs, a merge surface reached whole, moves every cited line in its hit-testing, hover and geometry ranges
existing-scopes#1.8 · no · accessibility.rs and debug.rs are not reached
existing-scopes#1.9 · no · the events directory is not reached
existing-scopes#1.10 · no · scrolling.rs and selection.rs are not reached
existing-scopes#1.11 · yes · stale · stylo.rs, a merge surface, is reached and its cited line 1 moves with it
existing-scopes#1.12 · no · tree.rs, traversal.rs and url.rs are not reached
existing-scopes#1.13 · yes · stale · construct.rs and damage.rs are reached whole, so their cited line ranges 393-652, 23-44, 361 and 540-555 move
existing-scopes#1.14 · yes · stale · packages/blitz-dom/src/lib.rs is reached and its cited lines 106-109 move
existing-scopes#1.15 · no · blitz-html is not reached
existing-scopes#1.16 · yes · stale · packages/blitz-net/src/lib.rs, a merge surface, is reached and its cited lines 1-3 move
existing-scopes#1.17 · yes · add-to · F26: packages/blitz-paint/src/blend.rs, a new private module of blitz-paint, joins the module list
existing-scopes#1.18 · yes · add-to · F26: url_search_params, a new bootstrap script and test target of blitz-vibey-script, joins the integration tests list
existing-scopes#1.19 · no · blitz-shell is not reached
existing-scopes#1.20 · no · packages/blitz/src/lib.rs is not reached and its re-exports hold
existing-scopes#1.21 · no · stylo_taffy is not reached and the Taffy version still waits
existing-scopes#1.22 · no · packages/dioxus-native is not reached
existing-scopes#1.23 · yes · stale · dioxus_document.rs, a merge surface, is reached and its cited lines 838-840 move
existing-scopes#1.24 · yes · stale · contrast.rs is reached, its blend-aware None reading moves it, and its cited lines 13-91 move
existing-scopes#1.25 · no · escher-telemetry holds no diff and its cited lines do not move
