# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · Cargo.toml is not edited, and Cargo.lock moved only pulldown-cmark, alone, so taffy's resolved 0.15.0 holds
established-decisions#2.1 · no · its profiles live in Cargo.toml, which the session does not edit
established-decisions#3.1 · no · its release statements live in Cargo.toml and publish-browser.yml, neither of which the session edits
established-decisions#4.1 · no · none of its cited renderer, harness, wpt and app files is reached
established-decisions#5.1 · no · its feature lists live in unedited Cargo.toml files, and blitz-shell's new cfg(test) module adds no feature
established-decisions#6.1 · yes · stale · document.rs is reached, so its cited lines 450–507 and 1382 are re-pointed, and no default is among the session's changes
established-decisions#7.1 · yes · stale · mutator.rs past :325 and document.rs are reached, so the cited lines 382–389, 1334–1435 and 1419–1435 are re-pointed
established-decisions#8.1 · yes · stale · document.rs is reached, so the cited lines 3153–3158 are re-pointed
established-decisions#9.1 · no · its stylo, stylo_to_kurbo, stylo_taffy and layout files are not reached, and no unsupported-feature rule is among the session's changes
established-decisions#10.1 · yes · contradict · a multiple select's option click focuses the select (F7), so "a click on any other element still clears focus" narrows, and the builder's toggle also reads a native checkbox's checked state
established-decisions#11.1 · yes · add-to · F25: the eight this chunk adds join the tally, flagged upstreamable with nothing sent, and upstream 0980cce5f's hit_inner hunk conflicts with the non-invertible answer for Epoch 3's sync
established-decisions#11.2 · yes · stale · its merge surface names document.rs, scrolling.rs and events/pointer.rs, all reached, so the file-level reading is re-read; the clock itself is unchanged
established-decisions#11.3 · yes · contradict · a multiple select's keys and pointer (F6, F7) and an optgroup's options are now modelled, so "not modelled" ends and the option-click refusal narrows to a closed single select (F14)
established-decisions#11.4 · no · its six items are cross-references judged in their own decisions, and the six stay uncounted in the direct tally
established-decisions#12.1 · yes · stale · events/pointer.rs is reached, so the hit_with_scrollbar hazard is re-read, and stand_cells_served moves from 4 to 5
established-decisions#13.1 · no · no layout module is edited, and the new default-sheet rows are style rules, not a layout rule
established-decisions#14.1 · no · layout/replaced.rs is not reached, and no layout module is edited
established-decisions#14.2 · yes · contradict · pseudo_element_flow_classification's three tests now run in both layout modes, so "its test file not asserting both layout modes" no longer holds
established-decisions#14.3 · yes · stale · dioxus_document.rs is reached, so the cited test lines 741–775 are re-pointed
established-decisions#15.1 · yes · add-to · the new select_cursor field of BaseDocument is one more interaction state the clear-or-retarget rule for removed nodes must read against
established-decisions#16.1 · yes · contradict · the focus-fixup limit ends: flush clears focus on a node that reads is_inert (F3), firing no blur, and inert_subtree's limit lines retire
established-decisions#16.2 · yes · stale · document.rs and mutator.rs are reached, so the cited lines (document 1055–1945, mutator 775–877) re-point, and flush now calls clear_focus from a new site
established-decisions#16.3 · yes · contradict · optgroup's options are listed in tree order (F12), a multiple select's selected write leaves its other options alone, and the user-agent sheet gains optgroup and multiple rows (F19)
established-decisions#17.1 · yes · stale · the hit path in node.rs and document.rs is reached (hit_inner's non-invertible answer), so the dropped-node skip is re-read
established-decisions#17.2 · yes · contradict · open_state.rs is a new four-test target, so ":open" no longer has no test of its own, and its mutator.rs citations move
