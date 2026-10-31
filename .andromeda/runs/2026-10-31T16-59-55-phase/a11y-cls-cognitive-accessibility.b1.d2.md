# Classified — a11y · cognitive-accessibility.b1

## Answers
cognitive-accessibility#1.1 · no · the 404 and error pages under apps/browser/assets are outside the reached files, and the chunk changes no error cue
cognitive-accessibility#1.2 · yes · the stand's shared `tab_order` and `ENABLED_TAB_ORDER` hold the flight booker's Tab stops, including `flight-type` (the select's Tab stop), which the scope names as moving with any new row or key step; the invalid-date cue and `aria-invalid` are not changed
cognitive-accessibility#2.1 · no · history relative time labels in apps/browser/src/browser_history.rs are outside the reached files and the chunk adds no orientation cue
cognitive-accessibility#2.2 · no · browser tab tooltips in apps/browser are outside the reached files
cognitive-accessibility#2.3 · no · browser status bar messages in apps/browser are outside the reached files
cognitive-accessibility#2.4 · no · seven_guis card descriptions and tags in examples/seven_guis/src/app.rs are outside the reached files, and the chunk edits no fixture or application markup
cognitive-accessibility#2.5 · no · the stand's served load page (`load-done`, `load-fetch`, stand.rs, stand_load_page.rs) is not reached, and no scope item reads it
