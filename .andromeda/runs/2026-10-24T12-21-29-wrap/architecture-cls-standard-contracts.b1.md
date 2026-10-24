# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · stale · the take's config.rs edits after line 13 moved its citations (31-74, 37-41, 76-95)
standard-contracts#2.1 · yes · stale · the take's config.rs edits after line 13 moved the clock field's citation (71-73)
standard-contracts#2.2 · no · select_value, its writers and their document.rs citations are not touched by the take
standard-contracts#3.1 · yes · add-to · set_attribute gains a class arm and remove_attribute clears classes, keeping the new ElementData::classes in sync
standard-contracts#4.1 · no · the event driver, keyboard, pointer and focus code are unchanged, and the dropped-node hit skip changes no target this names
standard-contracts#5.1 · yes · stale · citations into node.rs (1424-1554, 1740-1761) and stylo.rs (63, 187-229) moved, and the hit walk now skips nodes dropped since the last resolve
standard-contracts#6.1 · no · the transform-aware client rect and its document.rs citations are unchanged, and no hunk edits it
standard-contracts#6.2 · no · text_extent.rs and blitz-dom's lib.rs are untouched, and the reader's contract holds
standard-contracts#7.1 · yes · stale · citations into layout/construct.rs (44-62), layout/paint_tree.rs (107-203) and layout/mod.rs (608-866, 868-941) moved, and compute_replaced_layout now coerces natural height through aspect-ratio
standard-contracts#8.1 · yes · stale · citations into node.rs (885-911, 1788-1992) and element.rs (341, 348-376, 635-657, 880-888) moved, and Node::out_of_flow and ElementData::classes are new pub fields
standard-contracts#9.1 · yes · stale · the take's element.rs edits after line 45 moved the is_disabled citation (497-501)
standard-contracts#9.2 · no · the select module and its node/mod.rs:10 and select.rs citations are unchanged, and no select rule is touched
standard-contracts#10.1 · no · the provider traits and blitz-dom's html.rs are untouched by the take
standard-contracts#11.1 · no · blitz-html and blitz-net are untouched by the take, and their citations hold
standard-contracts#12.1 · no · blitz-shell, blitz-paint and the password path are untouched, and the Parallel launch edits sit in apps and dioxus-native, which this entity does not describe
