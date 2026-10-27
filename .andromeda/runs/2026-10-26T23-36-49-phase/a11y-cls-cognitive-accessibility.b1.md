# Classified — a11y · cognitive-accessibility.b1

## Answers
cognitive-accessibility#1.1 · no · the browser's error and 404 pages (apps/browser/assets) are not reached by the lint, the MCP listing test or the requirement sweep
cognitive-accessibility#1.2 · no · the flight booker's invalid-date cue and focus sequences are the Not-needed item; the lint takes no `invalid` state, and stand_lint's flight-booker reading is taken at boot, before any date is typed
cognitive-accessibility#2.1 · no · the browser history's relative time labels (apps/browser) are not reached
cognitive-accessibility#2.2 · no · the browser tab tooltip (apps/browser) is not reached
cognitive-accessibility#2.3 · no · the browser status bar (apps/browser) is not reached
cognitive-accessibility#2.4 · no · the seven_guis gallery cards (examples/seven_guis/src/app.rs) are not a screen the lint or the sweep reads
cognitive-accessibility#2.5 · yes · the load page's `load-done` is `display: none` and its stylesheet unlanded at boot, the state stand_lint's boot reading of the load page (0 findings, hidden nodes outside the walk) rests on
