# Classified — a11y · cognitive-accessibility.b1

## Answers
cognitive-accessibility#1.1 · no · the error and 404 pages' text sits in apps/browser assets, which the chunk does not reach
cognitive-accessibility#1.2 · yes · the forward focus sequence `stand_keyboard` asserts per lean task through `focus_next_node` against the shared `tab_order` and `ENABLED_TAB_ORDER`, the chunk's SC 2.4.3 witness, and the snapshot's `focused` after one move; the invalid-date cue is out of the entry
cognitive-accessibility#2.1 · no · the history time labels sit in apps/browser/src/browser_history.rs, which the chunk does not reach
cognitive-accessibility#2.2 · no · the tab tooltip sits in apps/browser assets and apps/browser/src/tab_strip.rs, which the chunk does not reach
cognitive-accessibility#2.3 · no · the status bar text sits in apps/browser/src/status_bar.rs, which the chunk does not reach
cognitive-accessibility#2.4 · no · the seven_guis cards sit in examples/seven_guis/src/app.rs, which the chunk does not reach
cognitive-accessibility#2.5 · yes · the load page's `load-fetch` click reveals `load-done` (display none until the stylesheet lands) after the 200 ms stand-in answer, which the load-page keyboard, Tab-order and contrast rows must drive and wait out
