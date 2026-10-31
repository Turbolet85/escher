# Classified — architecture · design-philosophy.b1

## Answers
design-philosophy#1.1 · yes · stale · blitz-paint's render.rs gains the select_list module near line 7, moving the cited lines 171-175; the crate split and the no-new-crate claim still hold
design-philosophy#2.1 · yes · stale · document.rs gains a select_list field and edits at lines 10-11 and 761-1089, moving the cited BaseDocument::new lines 479-490; the provider fallbacks still hold
design-philosophy#3.1 · yes · stale · keyboard.rs gains lines at 32-34, moving the implicit-submission citation at 556, and Enter on a single select now opens a list rather than doing nothing
design-philosophy#4.1 · yes · contradict · scroll_into_view on a row of an open list writes the list's own scroll_y and returns, an exception to the one shared scroll primitive cited at scrolling.rs:125-129
design-philosophy#5.1 · no · the cited harness, tests/blitz-tests lib.rs, paint_bench and browser files are unreached and the headless, phase-split and integration-test claims still hold
