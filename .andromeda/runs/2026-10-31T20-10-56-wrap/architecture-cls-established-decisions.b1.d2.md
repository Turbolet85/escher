# Classified — architecture · established-decisions.b1

## Answers

established-decisions#1.1 · no · no manifest moved (Cargo.toml, Cargo.lock and deny.toml unchanged) and no reached file is cited
established-decisions#2.1 · no · no build profile moved, Cargo.toml is unchanged and no reached file is cited
established-decisions#3.1 · no · no version or manifest moved and no reached file is cited
established-decisions#4.1 · no · no renderer selection moved; the list paints in the shared paint_scene and no backend, example or harness file is reached
established-decisions#5.1 · no · no manifest or default-feature list moved; the list's thumb stays under the scrollbars feature, still off by default
established-decisions#6.1 · yes · stale · document.rs and stylo.rs are reached whole and the list adds a BaseDocument field and a resolve step, so the defaults cited at document.rs:453-510, 1718 and stylo.rs:257-263 are not re-read on the changed files
established-decisions#7.1 · yes · stale · resolve.rs is reached whole and the list's reconcile step moves into resolve, so the render-blocking early return at resolve.rs:52-63 and the document.rs:385-392 and 1755-1771 citations are not re-read
established-decisions#8.1 · yes · stale · document.rs is reached whole and its CSSOM geometry snap at 3492-3497 is not re-read against the list's row bounds read by the snapshot and CSSOM
established-decisions#9.1 · yes · stale · stylo.rs is reached whole and its pseudo-class and unsupported-feature citations (133, 213, 460-508, 1228-1234) are not re-read against the list's restyle hold
established-decisions#10.1 · yes · contradict · scroll_into_view writing every scrolling box on the target's chain now has an exception for a row of an open list, and a wheel over an open list scrolls the list while a wheel elsewhere closes it first (F2)
established-decisions#11.1 · yes · stale · pointer.rs (password caret at 657-723), scrolling.rs and accessibility.rs (toggle) are reached whole and the list edits them, so the seven ratified rules are not re-read on the changed files
established-decisions#11.2 · yes · add-to · F1 and F8: the list is one more direct engine change for every document, its sixteen-file merge surface a line owned by Epoch 3's sync; F4's rejected overlay hit in hit_inner sits beside the unedited hit_inner conflict note with upstream 0980cce5f
established-decisions#11.3 · yes · stale · document.rs, resolve.rs, scrolling.rs and events/pointer.rs, four files of the clock's merge surface, are reached whole and edited by the list, so that surface reading is not re-measured; the list reads no engine time
established-decisions#11.4 · yes · contradict · F1-F3 open, close and choose a single select's list by click, Enter, Space, Escape, outside press, Tab and wheel, so the Enter and Space open-nothing and no-popup limits no longer hold; F5-F8 add the list's limits, the multiple size hint, the group label paint and the merge surface
established-decisions#11.5 · no · the multiple select's keys and the unmodelled Ctrl+A stay as they were on the changed keyboard.rs, per the select-model re-read, and no list or open-state rule reaches a multiple select
established-decisions#11.6 · no · the select-model re-read keeps apply_select_choice as the one path and a multiple select opens no list, so click_select_row's plain, Ctrl, Shift and disabled-row rules stand
established-decisions#11.7 · no · :open and flush_open_state are unmoved (the list writes set_expanded, not :open), the windowed redraw route and the file: refusal lie in unreached files
established-decisions#12.1 · yes · add-to · F5: the list's thumb is painted and not draggable and a list row is never a thumb drag, while the overlay-scrollbar hazard for boxes stands
established-decisions#13.1 · no · the list is no layout box and no layout module, Taffy trait or table path moved, per the dec-layout re-read
established-decisions#14.1 · no · the list is no layout box and no replaced-element aspect-ratio path moved; replaced.rs is unreached
established-decisions#14.2 · no · the list is no layout box and no flow-classification path moved; construct.rs is unreached
established-decisions#14.3 · no · no IME or editor-construction path moved; construct.rs and dioxus_document.rs are unreached
established-decisions#15.1 · yes · add-to · F3: a disabled row changes nothing and leaves the list open, so row_disabled joins the disabled readings; F2: disabled on the select closes an open list, a reader beyond the form-control activation gates
established-decisions#16.1 · yes · add-to · F2: inert on the select, or on an ancestor, closes an open list, a new place where inert's presence is honoured beside the Tab walk, hit and accessibility rules
established-decisions#16.2 · no · the focus fixup lives in mutator.rs, which is not reached, and no list change touches clear_focus's inert case
