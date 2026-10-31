# Classified — arch · design-philosophy.b1

## Answers
design-philosophy#1.1 · yes · the overlay is painted by blitz-paint through PaintScene for every renderer and lives in the engine, so the layered crate split and the resolved-styles paint assumption bind the overlay's paint.
design-philosophy#2.1 · yes · the list's redraw requests and the clock reach the DOM only through DocumentConfig and the ShellProvider, so the injected providers and their no-op defaults bind the new calls.
design-philosophy#3.1 · yes · Enter, Space, arrows and typeahead on a single select, the focus order and aria-expanded are browser behaviour written against the HTML, WAI-ARIA and WCAG texts, so the spec-matching binds.
design-philosophy#4.1 · yes · the open state references DOM nodes over versioned NodeIds, is cleared or retargeted before a removed node is freed, and open and close invalidation runs through doc.mutate() and damage propagation.
design-philosophy#5.1 · yes · the new stand rows, the harness's screenshot, text_contrast and settle that never move time, and synthesized input through the real dispatch pipeline bind this headless-harness rule.
