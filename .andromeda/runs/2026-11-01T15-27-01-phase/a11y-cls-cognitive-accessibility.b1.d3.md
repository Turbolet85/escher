# Classified — a11y · cognitive-accessibility.b1

## Answers
cognitive-accessibility#1.1 · no · the "Failed to load page" and "404 Not found" strings sit in apps/browser assets, which the chunk does not reach, and no named check reads them
cognitive-accessibility#1.2 · yes · the stated Tab orders in the stand checks' shared tab_order tables (flight booker's four stops, flight-return-date joining ENABLED_TAB_ORDER) are re-read by stand_keyboard in both layout modes; the aria-invalid state on the two date fields is what the named snapshot checks read
cognitive-accessibility#2.1 · no · history relative time labels are in apps/browser/src/browser_history.rs, which the chunk does not reach and no named check reads
cognitive-accessibility#2.2 · no · the tab hover tooltip is in apps/browser (browser.css, tab_strip.rs), which the chunk does not reach and no named check reads
cognitive-accessibility#2.3 · no · the status bar's hovered link target and loading message are in apps/browser/src/status_bar.rs, which the chunk does not reach and no named check reads
cognitive-accessibility#2.4 · no · seven_guis card descriptions and tags are in examples/seven_guis/src/app.rs, which the chunk does not reach and no named check reads
cognitive-accessibility#2.5 · yes · stand_load_page is named in the scope beside stand_keyboard and the load page's contrast ratio is re-measured; the load-done paragraph hidden until Load is clicked is the behaviour that check asserts
