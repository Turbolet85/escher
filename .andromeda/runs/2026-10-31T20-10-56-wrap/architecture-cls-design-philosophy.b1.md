# Classified — architecture · design-philosophy.b1

## Answers
design-philosophy#1.1 · yes · stale · the insertion at blitz-paint render.rs line 7 moves the cited render.rs:171-175 citation, and the list is painted after the root in paint_scene
design-philosophy#2.1 · yes · stale · the insertions in document.rs (at lines 10-11 and 343-344, among others) move the cited BaseDocument::new fallback at document.rs:479-490
design-philosophy#3.1 · yes · stale · the insertion at keyboard.rs lines 32-34 moves the cited implicit form submission line keyboard.rs:556
design-philosophy#4.1 · yes · contradict · a row of an open list scrolls by its own write to the list's scroll_y and returns outside the shared scrolling primitive, so one primitive no longer covers every scroll
design-philosophy#5.1 · no · the cited harness, WPT runner, paint bench, browser and tests/blitz-tests lib.rs are not reached, and the new checks are headless integration tests in the same tests/ directory
