# Classified — tests · test-scope-summary.b4

## Answers
test-scope-summary#20.1 · yes · visible_region_transformed rules over BaseDocument::visible_region in scrolling.rs, a conflict surface the take can move
test-scope-summary#21.1 · yes · checkable change event dispatch (input then change) sits on the dioxus-native-dom document-event path, reached through dioxus_document.rs, a conflict surface, and the scope names the bridge and document events
test-scope-summary#22.1 · yes · no change on text input or range is the same document-event dispatch through dioxus_document.rs, a conflict surface
test-scope-summary#23.1 · yes · driver reaching a control in a scrolling box under a zoomed world relies on scrolled-box bounds and visible region in scrolling.rs, a conflict surface, and the scope names scrolled_box_absolute_position
test-scope-summary#24.1 · yes · stand_act_popup drives the select list the scope names as a contract (select model and its list), and select_popup.rs is edited
test-scope-summary#25.1 · yes · the open list's rows reading no overlap under stand_lint depend on the row geometry that construct.rs and inline/build.rs conflict resolutions move, and on the select list the scope names
test-scope-summary#26.1 · yes · native checkbox and radio writing toggled under a checkable role is an accessibility_* tree-identity proof the a11y leg runs under WCAG 4.1.2 scope and the accessibility_tree override
test-scope-summary#27.1 · yes · inert_subtree is a re-read standing proof the a11y leg names, and its focus-on-inert rule must hold on the synced engine
test-scope-summary#28.1 · yes · select_popup is named in Needs as a select-model contract, and its skipped key-press row in select_popup.rs is the one the founder fork restores
test-scope-summary#29.1 · yes · select_popup_painted is named in Needs as a select-model contract and is the font-embolden witness the local gate joins
test-scope-summary#30.1 · yes · stand_keyboard on Cells and the load page is the named focus-order and activation proof, and Cells is a conflict surface in cells.rs
test-scope-summary#31.1 · yes · stand_contrast rows on Cells and the load page are named contrast proofs, and the .cell.current outline rests on cells.rs and default.css, both conflict surfaces
test-scope-summary#32.1 · no · the helper structure of stand_requirement_sweep.rs is a test file the chunk neither reaches nor re-reads; its leg-text reading is carried by test-scope-summary#33.1
test-scope-summary#33.1 · yes · the sweep reads .github/scripts/ci-leg.sh by contains, and the chunk edits ci-leg.sh for the font-embolden gate, so the stand-repeat membership of select_interaction, select_popup and select_popup_painted must still hold
test-scope-summary#34.1 · yes · stand_cells_served rests on cells.rs, a conflict surface, and states the 47,949-byte snapshot and 5-part Cells screen that a moved engine re-measures; stand_load_page is the named keyboard and contrast proof
test-scope-summary#34.2 · yes · stand_snapshot's select ComboBox and two ListBoxOptions bind the dioxus-native-dom snapshot the scope names, and host_log is a named 0-needle capture check
test-scope-summary#34.3 · no · the stand module's boot-option and load-in-flight unit tests sit in examples/seven_guis/src/stand.rs, which the chunk does not reach, and the take adds no listener or provider of ours
test-scope-summary#34.4 · yes · the lint's select-option skip reads visible_region (scrolling.rs, a conflict surface) and the select box, and the scope names the select model and its list
test-scope-summary#35.1 · yes · the host_log and mcp_log checks read 0 needles at trace, which the scope names among the standing capture checks, and the hosted open list must still match the in-process shot
test-scope-summary#36.1 · yes · stand_cells_served's keyboard walk is a stand_keyboard proof the scope names, over the Cells code in cells.rs, a conflict surface
test-scope-summary#37.1 · no · the WPT runner's own unit tests are upstream-only and out of this chunk's scope under Not needed
