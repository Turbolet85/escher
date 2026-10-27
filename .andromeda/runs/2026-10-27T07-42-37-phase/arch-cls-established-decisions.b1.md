# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · the pin set (Cargo.toml:74-141 and the commented patch block at 308-337) lies in the Cargo.toml merge surface, the dependency-pinning Need
established-decisions#2.1 · yes · the `dev` debuginfo line (Cargo.toml:208-209) lies in the Cargo.toml merge surface, so a merge can move it; the named profiles below it are the task's not-needed part
established-decisions#3.1 · yes · the workspace version and in-repo `=` pins (Cargo.toml:34-60) lie in the Cargo.toml merge surface, the lines the manifests re-read for a members change
established-decisions#4.1 · yes · the screenshot pixels the chunk re-measures read the harness's VelloCpuImageRenderer and its opaque-white composite, the re-measure rule the Needs name
established-decisions#5.1 · yes · the clipboard default in blitz-shell's and dioxus-native's Cargo.toml default lists, both merge surfaces, is the default-features and clipboard-move Need
established-decisions#6.1 · yes · the Stylo prefs, the incremental default and the media and quirks defaults (document.rs:448-505) lie in the document.rs merge surface
established-decisions#7.1 · yes · the render-blocking, iframe-depth, srcdoc and image-cache rules at mutator.rs:1288-1389 (conflict surface) and document.rs:380-387, 1389-1405 (merge surface)
established-decisions#8.1 · yes · the CSSOM 1/64px snap at document.rs:3119-3124 lies in the document.rs merge surface
established-decisions#9.1 · yes · the unsupported-feature and pseudo-state rules at stylo.rs (merge surface) and the pseudo-element content and anonymous table rules at construct.rs:590-672 (conflict surface)
established-decisions#10.1 · yes · the scrolling contract, hit stops and client rect over scrolling.rs, writing_mode.rs and node.rs (conflict surfaces) and document.rs and pointer.rs (merge surfaces), the Needs' scrolling and hit-test contracts
established-decisions#11.1 · yes · the seven direct engine fixes the Needs re-read per row: the password caret at pointer.rs:582-648 and the hit and client-rect rules over node.rs and document.rs
established-decisions#11.2 · yes · the document clock invariant the Needs name, over scrolling.rs (conflict surface) and document.rs and pointer.rs (merge surfaces), with clock.rs re-read
established-decisions#11.3 · yes · the select model the task names unchanged through the conflicts: element.rs, pointer.rs and document.rs (merge surfaces) and mutator.rs (conflict surface)
established-decisions#12.1 · yes · the overlay-scrollbar hit rule at events/pointer.rs (merge surface) and the stand_cells_served check the chunk re-runs
established-decisions#13.1 · yes · the construction rules at construct.rs:427, 432-449 and 929-947 (conflict surface) and the editor size at node/text.rs:85-92 (merge surface), the Needs' box-construction contract
established-decisions#14.1 · no · the aspect-ratio rule lives in layout/replaced.rs, which the chunk does not reach, and no re-measured stand value reads a replaced element
established-decisions#15.1 · yes · the disabled-by-presence reading at element.rs (merge surface, 480-518 and 733), with pointer.rs and dioxus_document.rs (merge surfaces), holds the focus and activation rules
established-decisions#16.1 · yes · the inert rule at node.rs (conflict surface, 563-580 and 1437-1446), the hit pass-through and accessibility exclusion the Needs name
established-decisions#16.2 · yes · set_focus_to and clear_focus in document.rs (merge surface) and the focus carry in mutator.rs (conflict surface), whose FOCUS_WITHIN writes the changed-set Need names
established-decisions#16.3 · yes · the CHECKED seed at element.rs (merge surface, 472-478) and the select settling in mutator.rs (conflict surface, 1025-1249), the select model the task names
established-decisions#17.1 · yes · the hit test that skips a dropped node is the hit-test contract the Needs name, over the node.rs conflict surface
established-decisions#18.1 · yes · the template fragment and feature-gated hit testing at element.rs (merge surface, 89-90, 353-354, 1043-1069)
established-decisions#19.1 · yes · window.rs and lib.rs (merge surfaces) and blitz-traits events.rs (merge surface, the IME event copy) hold the shell and windowing rules the Needs name
established-decisions#20.1 · yes · the password-mask engine change is a fork engine change whose merge surfaces, render.rs as the entity names and document.rs, the chunk re-reads
