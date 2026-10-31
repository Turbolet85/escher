# Classified — tests · test-scope-summary.b4

## Answers

test-scope-summary#17.1 · no · visible_region under transforms lives in scrolling.rs, which the chunk does not reach, and the scope names no transformed clipping box
test-scope-summary#18.1 · no · checkable checkbox and radio change events are not reached or named, and the chunk adds no checkable control change
test-scope-summary#19.1 · no · text input and range input and change events are not named; the keyboard.rs and pointer.rs changes are for select keys and clicks only
test-scope-summary#20.1 · no · driver reach of a control in a scrolling box under a zoomed world is not named by the scope
test-scope-summary#21.1 · yes · the lint's overlaps exclusion of a closed select's options, which lint.rs edits, and the multiple select's two overlapping rows read by stand_lint, whose table reads select#flight-type
test-scope-summary#22.1 · no · a native checkbox's toggled and pressed reading is not the select's expanded state the scope names, and the chunk changes no checkable role mapping
test-scope-summary#23.1 · no · inert focus loss is not named, and the chunk makes no node inert
test-scope-summary#24.1 · no · Cells' Tab, arrow and editor keys and the load page's button keys are not select keys; the stand_keyboard edit is the flight booker's table row
test-scope-summary#25.1 · no · Cells' and the load page's ratios and the cell focus outline are not the list's colours; the chunk's contrast rows are the list option text and select#flight-type
test-scope-summary#26.1 · no · the 0.2.0 sweep helpers are not reached, and the chunk adds no requirement row to them
test-scope-summary#27.1 · no · standing S8 · a stand-repeat named list that a target with no stand_ prefix must join; select_interaction already joins it and the chunk changes no leg site
test-scope-summary#28.1 · no · the Cells and load-page served checks hold no select, so the lint's select-option exclusion moves no finding on them and no hit testing or overlay scrollbar read is named
test-scope-summary#28.2 · yes · the flight booker's flight-type snapshot (ComboBox with its two options at the select's client rect) reads the snapshot bounds that snapshot.rs edits; the host_log half is not the chunk's
test-scope-summary#28.3 · no · the seven_guis stand unit tests (offline options, Cells shell, load click) are not reached, and the chunk adds no boot
test-scope-summary#28.4 · yes · the lint's select-option reading (in_closed_select through closed_select_of, multiple rows linted as usual) is the overlaps exclusion that lint.rs edits
test-scope-summary#29.1 · yes · the driver's twelve key names holding no character bind the chunk's typeahead tests on a single select, which must supply characters through `type` and not `press`
test-scope-summary#30.1 · no · the WPT runner is not reached and sits in the task's Not needed list
