# Classified — tests · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · no · the human-read pages under examples/assets carry no budget and no assertion, and the chunk neither reaches nor re-reads them
test-data-fixtures#2.1 · no · the stand Counter's recorded sessions are read only by stand_replay and seven_guis recording, which the chunk neither edits nor re-reads
test-data-fixtures#3.1 · yes · the incremental_oracle identity the chunk re-reads runs over these inline pages (incremental_oracle.rs:371-420), and the chunk's new checks state their pages as inline HTML of this kind
test-data-fixtures#3.2 · no · the 400-button fixture page is read only by stand_snapshot_parts and stand_act_spans, which the chunk neither reaches nor re-reads
test-data-fixtures#3.3 · yes · the new Cells Tab-order, keyboard-activation and contrast rows boot this screen through stand::boot_cells, whose snapshot runs in five budget parts, so they read it in parts as stand_cells_served does
test-data-fixtures#3.4 · no · the script tests' inline HTML sits in blitz-vibey-script, a surface no debt touches
test-data-fixtures#3.5 · no · the temp-directory files of the script's file-load tests sit in blitz-vibey-script, a surface no debt touches
test-data-fixtures#3.6 · no · the inline literals sit in html_sink.rs and node/element.rs, neither reached nor re-read by the chunk
test-data-fixtures#3.7 · no · the SVG byte literals sit in blitz-dom util.rs, and SVG parsing is outside the scope
test-data-fixtures#3.8 · no · the WPT runner's fuzzy inputs belong to WPT, which the fork does not run
test-data-fixtures#3.9 · no · the WPT diff script's entries belong to WPT, which the fork does not run
test-data-fixtures#3.10 · yes · the pin reads docs/capabilities.md and escher-driver's schema.rs, refusal.rs and error.rs, where the off-screen and covered text moves, and counts the closed NODE_FIELDS, VERBS and causes
test-data-fixtures#3.11 · no · the inline user-agent sheets sit in document.rs, a file the chunk neither reaches nor re-reads
test-data-fixtures#3.12 · yes · the contrast rows and the transform proof are painted and measured at one of these fixed sizes under scale 1.0 and Light, the defaults they read
test-data-fixtures#3.13 · yes · the bundled DejaVu Sans the stand registers for every generic is the family the re-measured contrast rows (stand_contrast and the new Cells and load rows) are measured in
test-data-fixtures#3.14 · no · the background-image injections in background_size and svg_background_size are neither reached nor re-read by the chunk
test-data-fixtures#3.15 · no · the generated benchmark pages of paint_tree_bench are not needed, since no budget is set and no assertion lives there
test-data-fixtures#3.16 · no · the external benchmark page of paint_tree_bench is not needed, since no budget is set and no assertion lives there
test-data-fixtures#3.17 · no · the browser entry helpers belong to apps/browser, which no debt reaches
test-data-fixtures#3.18 · no · the .test URLs belong to the browser tests in apps/browser, which no debt reaches
test-data-fixtures#3.19 · no · the favicon bytes belong to apps/browser, which no debt reaches
test-data-fixtures#3.20 · no · the fetch body belongs to blitz-vibey-script's dom tests, a surface no debt touches
test-data-fixtures#3.21 · yes · the new load-fetch keyboard-activation row acts over this LoadNet and must wait the answer out through the driver's load wait, with no sleep or clock read, as stand_load_page does
test-data-fixtures#4.1 · no · an absence search over a slice the task does not name, which asks the chunk for no fixture file or loader
