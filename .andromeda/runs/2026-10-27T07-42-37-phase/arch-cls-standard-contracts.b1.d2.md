# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the changed-set contract over the mutator.rs conflict surface and the document.rs merge surface, and DioxusDocument's accessibility_tree override the Dioxus bridge carries
standard-contracts#2.1 · yes · the document clock invariant the task re-reads in clock.rs and holds over the rewritten scrolling code, every time read going through DocumentConfig.clock
standard-contracts#2.2 · yes · select_value and the crate-private select writers in the document.rs merge surface, the one choice path apply_select_choice that the Needs name
standard-contracts#3.1 · yes · the doc.mutate() methods over the mutator.rs conflict surface, and the accessibility builder's name and state reads that the a11y leg re-proves
standard-contracts#4.1 · yes · the event driver, pointer default actions and button focus through the driver.rs, pointer.rs and blitz-traits events.rs merge surfaces, with the range and Enter and Space arms the Needs name
standard-contracts#5.1 · yes · scroll_into_view and visible_region in the scrolling.rs conflict surface, and the hit walk and Node::absolute_position in the node.rs conflict surface that the scrolling contract names
standard-contracts#6.1 · yes · the client rect under transforms in the document.rs merge surface, and the scrolled-box bounds in the writing_mode.rs conflict surface, both named by the scrolling contract
standard-contracts#6.2 · yes · the text extent's block_offset comes from the inline layout that the inline/build.rs conflict surface builds, and its named check text_extent is re-run while stand_lint re-measures the findings that read it
standard-contracts#6.3 · yes · the hit walk's skip of a dropped node lives in the node.rs conflict surface, and its named check hit_test_dropped_nodes is re-run
standard-contracts#7.1 · yes · LayoutPassState as the one implementer of Taffy's traits, which the Needs name over the construct.rs and inline/build.rs conflicts
standard-contracts#8.1 · yes · SpecialElementData in the element.rs merge surface, with the text-input control data that construct.rs builds at construction, and the node.rs CSSOM View accessors
standard-contracts#9.1 · yes · ElementData::is_disabled in the element.rs merge surface is the one disabled reading that the focus, pointer and activation keys share, which the Needs name
standard-contracts#9.2 · yes · the crate-private select model is the fork's engine change that residual line 36 re-reads against the take, and the select arms the Needs name
standard-contracts#9.3 · yes · Node::out_of_flow and record_out_of_flow live in the node.rs conflict surface, and the style traversal writes the flag that layout reads
standard-contracts#9.4 · yes · ElementData::classes is kept in sync in the mutator.rs conflict surface and element.rs merge surface and read by stylo.rs, all of them conflict or merge surfaces
standard-contracts#10.1 · yes · the ShellProvider clipboard defaults and set_ime_enabled in the shell.rs merge surface, which the shell's clipboard move and winit's IME copy name
standard-contracts#11.1 · no · blitz-html and blitz-net are in no reached file and the scope names no call or behaviour of their API, so the chunk only uses them and they keep their guarantees by their own code
