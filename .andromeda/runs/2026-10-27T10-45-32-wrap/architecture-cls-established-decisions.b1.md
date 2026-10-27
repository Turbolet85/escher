# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · its pins sit in Cargo.toml and the example manifests, all byte-identical to the base, so no dependency moved
established-decisions#2.1 · no · the build profiles sit in Cargo.toml, which the session left byte-identical
established-decisions#3.1 · yes · stale · publish-browser.yml is a merge surface reached whole, and its lines 21-26 (the prepatch rule) are not shown holding on the changed file
established-decisions#4.1 · no · none of the renderer files it cites is reached, and the WPT runner moved only its upstream pin
established-decisions#5.1 · no · its default features sit in crate manifests, all byte-identical to the base
established-decisions#6.1 · yes · stale · dom/element.rs (merge surface) moved its code into the dom/element module tree, so its 1072-1073 citation moved, and stylo.rs (merge surface) is reached whole
established-decisions#7.1 · yes · stale · mutator.rs (conflict surface) gained lines at 368-371 and 520-523, so its cited lines 1296-1397 have moved
established-decisions#8.1 · no · every file it cites (document.rs, resolved_style.rs, stylo_to_parley.rs, stylo_taffy) is unreached, and Stylo 0.22.0 and Taffy 0.15 stay
established-decisions#9.1 · yes · stale · stylo.rs and construct.rs (merge surfaces) moved their cited lines, and the pseudo-element wording is to be checked against the ::before/::after flow vote while :open stays off the always-false list
established-decisions#10.1 · yes · stale · its take readings at f9cbb2756 move to ebacd68bd, its merge-surface owner moves to Epoch 3, and mutator.rs (conflict surface) moved its cited lines 392-394 and 533-540
established-decisions#11.1 · yes · stale · its merge-surface owner, residual line 36, moves to Epoch 3's «Upstream sync ahead of the UI graph»
established-decisions#11.2 · yes · stale · its clock merge-surface owner, residual line 36, moves to Epoch 3's «Upstream sync ahead of the UI graph»
established-decisions#11.3 · yes · stale · its select-model merge-surface owner, residual line 36, moves to Epoch 3's «Upstream sync ahead of the UI graph»
established-decisions#12.1 · no · its hazard sits in events/pointer.rs and the virtual clock, neither reached, and the scroll and hit readers were re-read unchanged
established-decisions#13.1 · yes · add-to · F2, the ::before/::after box vote in classify_flow_children, joins the layout construction rules, and construct.rs (merge surface) moved its cited lines 417 and 926-944
established-decisions#14.1 · no · its only cited file, layout/replaced.rs, is unreached and the aspect-ratio rule is unchanged
established-decisions#15.1 · yes · add-to · F1, :open matching a details or dialog that carries open through ElementState::OPEN, joins the element-state reads beside DISABLED and ENABLED, and node/element.rs and dioxus_document.rs (merge and conflict surfaces) are reached
established-decisions#16.1 · no · its inert rules sit in node.rs, accessibility.rs and inert_subtree.rs, none reached, and its no-mutator-hook reading holds, since the only new mutator arms flush open
established-decisions#16.2 · yes · stale · mutator.rs (conflict surface) gained lines at 368-371 and 520-523, so its cited lines 767-869 have moved
established-decisions#16.3 · yes · stale · node/element.rs and mutator.rs (merge and conflict surfaces) are reached whole, and mutator.rs gained lines at 368-371 and 520-523, so its cited select lines moved
established-decisions#17.1 · no · its hit-test skip sits in node and paint-tree files the session did not reach, and none of the take's behaviour changes is a hit-test change
established-decisions#18.1 · yes · stale · node/element.rs (merge surface) is reached whole, so its citations at 89-90, 353-354 and 1053-1079 are not shown holding
established-decisions#19.1 · yes · add-to · F3, IME enabled at construction for a focused input, and F4, the windowed shell's redraw requests travelling through its event loop, join the shell decisions, and lib.rs and window.rs are merge surfaces
established-decisions#20.1 · yes · stale · its merge-surface owner moves to Epoch 3's «Upstream sync ahead of the UI graph»
