# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · no dependency, manifest or lock moved; Cargo.toml, Cargo.lock and deny.toml are unchanged and no cited file is reached
established-decisions#2.1 · no · no build profile moved; Cargo.toml:208-250 is unreached and unchanged
established-decisions#3.1 · no · no version or release tooling moved; the workspace version and publish workflow are unreached
established-decisions#4.1 · no · no renderer selection moved; the cited examples, wpt, rdme, dioxus-native and blitz-test-harness files are unreached
established-decisions#5.1 · no · no default feature list or manifest moved; the feature names stand and the list's thumb keeps the existing scrollbars gate
established-decisions#6.1 · yes · stale · document.rs is reached and its cited lines 453-468, 510 and 1718 moved past the session's document.rs insertions at 10-11, 343-344 and 761-1089; the defaults themselves hold
established-decisions#7.1 · yes · stale · document.rs:385-392 and 1755-1771 moved past the session's document.rs insertions, and resolve.rs, reached whole, gains reconcile_select_list at 133-135; the render-blocking rule itself holds
established-decisions#8.1 · yes · stale · document.rs:3492-3497 moved past the session's document.rs insertion at 2404-2406, and the file is reached whole; the approximations themselves hold
established-decisions#9.1 · yes · stale · stylo.rs:1228-1234 moved past the session's stylo.rs insertion at 1124-1139, and the file is reached whole; the unsupported-feature readings hold
established-decisions#10.1 · yes · contradict · scroll_into_view on a row of an open list writes only the list's scroll_y and a wheel over an open list scrolls the list, so the every-box chain and the chain-to-viewport rule no longer hold for a list row
established-decisions#11.1 · yes · stale · pointer.rs:657-723 moved past the session's pointer.rs insertions at 595-656, and the hit path now asks the open list first through hit_with_scrollbar; the seven changes themselves hold
established-decisions#11.2 · yes · stale · the tally of direct engine changes gains the open select list, hit_inner was not edited so its conflict note with upstream 0980cce5f is unmoved, and the merge-surface line stands
established-decisions#11.3 · yes · stale · the clock's merge-surface files document.rs, resolve.rs, scrolling.rs and events/pointer.rs are all reached whole, so the clock reading cannot be ruled out
established-decisions#11.4 · yes · contradict · Enter and Space now open a single select's list, which the engine paints over the page with a close on Escape, outside clicks, scroll or disable (F1, F2, F3, F5, F6 F8 included), so Enter and Space open nothing, no popup is painted and size is unread on a single select now end; apply_select_choice stays the one path and an option of a closed select stays refused covered
established-decisions#11.5 · yes · stale · keyboard.rs:285-468 moved past the session's keyboard.rs insertions at 3, 7 and 32-34, and the file is reached whole; the multiple select's keys hold
established-decisions#11.6 · yes · stale · pointer.rs:1121-1162 moved past the session's pointer.rs insertions at 220 and 595-656, and the file is reached whole; the multiple select's pointer rule holds
established-decisions#11.7 · no · the six upstream behaviours are :open (unmoved, F17), the flow vote, the windowed shell's redraw route, the file: refusal and the accessors; blitz-shell, blitz-vibey-script and layout are unedited
established-decisions#12.1 · yes · add-to · a list row is never a thumb drag, since hit_with_scrollbar answers the open list first; the overlay thumb hazard itself stands
established-decisions#13.1 · no · the list is no layout box and no layout module moved; the cited layout and node files are unreached
established-decisions#14.1 · no · the replaced-element aspect ratio sits in layout/replaced.rs, which the session did not reach
established-decisions#14.2 · no · the pseudo-element flow votes sit in layout/construct.rs and its test, neither reached
established-decisions#14.3 · no · the IME hook sits in layout/construct.rs and dioxus_document.rs, neither reached
established-decisions#15.1 · yes · stale · pointer.rs:333, 472, 832 and keyboard.rs:332, 349 moved past the session's insertions in those files, and the disabled reading holds with the list's Enter and Space gates added
established-decisions#16.1 · yes · stale · accessibility.rs, reached whole, carries the inert reading at 283 and 287-300, which may have moved, and the hit path now asks the open list before hit_inner
established-decisions#16.2 · no · the focus fixup's call and its no-event clear sit in mutator.rs and inert_subtree.rs, neither reached; the list's close on inert runs at resolve, not in the fixup
