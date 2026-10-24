# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · the session's manifest work (tokio's lock entry, one blitz-tests target, mimalloc in two packages) names no accesskit pin or member, so the workspace entries stand
a11y-scope-summary#1.2 · no · accesskit_xplat is one of the session's untouched items, so its manifest and its adapter stand and no stand binary builds it
a11y-scope-summary#1.3 · no · accessibility.rs is unchanged and the session names no change to the DOM reads that build_accessibility_tree makes, so its citations stand
a11y-scope-summary#1.4 · no · custom_widget.rs is not among the session's changes, so its TODO hook and its lines stand
a11y-scope-summary#1.5 · no · the session names no blitz-html change, so its accessibility feature forwarding stands
a11y-scope-summary#1.6 · no · blitz-shell's accessibility feature and its View adapter wiring are untouched, since the windowed change lives in dioxus_native::launch
a11y-scope-summary#1.7 · no · the session names no change to blitz's manifest, so its accessibility feature and its forwarding to blitz-shell stand
a11y-scope-summary#1.8 · no · the session names no change to node_id.rs, so NodeId::as_u64 and its interop note stand
a11y-scope-summary#1.9 · yes · contradict · the windowed launch now builds its document with Parallel style threading under PARALLEL_TRAVERSAL_LOCK, so the windowed stand no longer boots as before (smoke green, still no adapter)
a11y-scope-summary#1.10 · no · mimalloc is added as apps/browser's global allocator, which leaves its default feature set and its dioxus-native accessibility wiring as they were
a11y-scope-summary#1.11 · no · the one new target, hit_test_dropped_nodes, names no feature or dev-dependency, so blitz-dom and dioxus-native-dom stay on the accessibility feature and accesskit stays a dev-dependency
a11y-scope-summary#1.12 · no · the session changes no input fixture, so examples/assets/input.html and its focusable divs stand
a11y-scope-summary#2.1 · yes · stale · the new root example examples/multi_doc.rs is one more examples/*.rs file, so the count of 18 moves, and it carries no ARIA, role or tabindex, so the absence still holds
a11y-scope-summary#2.2 · no · the session's engine changes (hit skip, classes, out_of_flow, aspect-ratio height, the windowed lock) add no accessibility, ARIA or screen-reader code to the listed files
a11y-scope-summary#2.3 · no · the session adds no accessibility handling, its changes being the hit skip, classes, out_of_flow, the aspect-ratio height and the windowed lock
a11y-scope-summary#2.4 · no · the WPT runner change is mimalloc as its global allocator, which carries no ARIA, role, keyboard or focus text
