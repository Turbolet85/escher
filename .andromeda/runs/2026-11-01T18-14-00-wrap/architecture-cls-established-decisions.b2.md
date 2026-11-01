# Classified — architecture · established-decisions.b2

## Answers
established-decisions#14.1 · no · its rule lives in packages/blitz-dom/src/layout/replaced.rs, which the session does not reach
established-decisions#14.2 · yes · stale · construct.rs is a merge surface, so the citations at lines 878 and 885-890 move
established-decisions#14.3 · yes · stale · construct.rs and dioxus_document.rs are merge surfaces, so the citations at construct.rs 316-320 and dioxus_document.rs 745-779 move
established-decisions#15.1 · yes · stale · element.rs and dioxus_document.rs are merge surfaces, so the citations at element.rs 480-520 and 743 and dioxus_document.rs 518-552 move; the anonymous-block sentence holds
established-decisions#16.1 · yes · stale · node.rs is a merge surface, so the citations at node.rs 574-591 and 1450-1459 move
established-decisions#16.2 · no · mutator.rs is a merge surface, but the re-read on the changed file holds: DocumentMutator::flush still reads is_inert() alone and the focus fixup is unmoved
established-decisions#16.3 · yes · stale · document.rs and mutator.rs are merge surfaces, so the citations at document.rs 1399-1413 and 2244-2313 and mutator.rs 859-949 move; snapshot_node_and is another symbol
established-decisions#16.4 · yes · stale · element.rs, mutator.rs and default.css are merge surfaces, so the citations at element.rs 472-478, mutator.rs 1113-1367 and default.css 77-86 and 155-159 move
established-decisions#16.5 · yes · stale · mutator.rs is a merge surface, so the citation at mutator.rs 1325-1329 moves
established-decisions#16.6 · yes · stale · select_popup.rs is reached whole, so its citations at 801-851 move
established-decisions#16.7 · yes · stale · default.css is a merge surface, so the citations at default.css 161-199 move
established-decisions#16.8 · yes · stale · stylo.rs and default.css are merge surfaces and select_popup.rs is reached, so the citations at stylo.rs 1132-1147, default.css 183-193 and select_popup.rs 991-1028 move
established-decisions#17.1 · yes · stale · the hit test and the paint-tree walk live in reached merge surfaces (blitz-dom node and document, blitz-paint render.rs), and no report shows the claim holding
established-decisions#17.2 · yes · stale · element.rs, mutator.rs and stylo.rs are merge surfaces, so the citations at element.rs 488-489 and 543-549, mutator.rs 356-359 and 502-505 and stylo.rs 513 move
established-decisions#18.1 · yes · stale · element.rs is a merge surface, so the citations at element.rs 89-90, 353-354 and 1056-1082 move
established-decisions#19.1 · no · packages/blitz-shell, packages/accesskit_xplat and blitz-traits events.rs are not reached
established-decisions#20.1 · no · the redraw path lives in packages/blitz-shell, which the session does not reach
established-decisions#21.1 · yes · add-to · F10, render_element's border box and culling on the scaled size, joins the culling clause; render.rs and document.rs, its merge surfaces, took this take
established-decisions#22.1 · yes · stale · runtime/bootstrap.rs is a merge surface, so the citation at bootstrap.rs 56-60 moves
established-decisions#23.1 · yes · stale · dioxus_document.rs is a merge surface, so the citations at 106-110 move
established-decisions#24.1 · no · the hazard lives in dioxus-native-dom events.rs, which is not reached, and the harness change is the failed-load wait with settle_waiting unedited
established-decisions#25.1 · no · nothing under apps/ is reached
established-decisions#26.1 · no · apps/browser/src/tab.rs sits under apps/, which the session does not reach
established-decisions#27.1 · no · wpt/runner/src holds no diff and the quarantine list is named without a count, so the skip claim holds
established-decisions#28.1 · no · packages/escher-driver holds no diff and the manifests and Cargo.lock are unchanged
