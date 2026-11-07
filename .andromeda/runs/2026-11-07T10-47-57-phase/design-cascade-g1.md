@carried n=1 label=design-system/motion fact=fct-vt52hpbo
why: once scroll_to is reachable from render-polled tasks, the buffered render must leave the into-view scroll rule exactly as it stands.
quote: A programmatic into-view scroll never animates a nested scrolling box: `BaseDocument::scroll_into_view` writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked — the document holds one scroll animation at a time — and the requested behaviour, `Smooth` included, applies to the viewport alone
@carried n=1 run=2 label=design-system fact=fct-tnwqvola
quote: A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, whatever behaviour was asked, and the requested behaviour, `Smooth` included, applies to the viewport alone
@carried n=2 label=design-system/motion fact=fct-vb77s7g2
why: the MountedData scroll calls the scope names must keep supporting both behaviours when called from render, the buffer changing neither.
quote: programmatic scrolling in dioxus-native-dom supports `Smooth` and `Instant` (packages/dioxus-native-dom/src/events.rs:246-249; packages/dioxus-native-dom/src/events.rs:285-288)
@carried n=3 label=design-system/motion fact=fct-wr73fuj5
why: a Smooth scroll_to or scroll reached during render must still run the engine's own scroll animation, unchanged by the buffered render.
quote: Smooth scrolls run 300 ms on a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:112-122; packages/blitz-dom/src/scrolling.rs:432-433)
@carried n=4 label=design-system/motion fact=fct-z2ugu37c
why: the stand renders through the buffer and its stand_* checks are re-asserted, so the Cells grid's synchronous Instant scroll_to must keep bringing its cell into view with no animation.
quote: The stand's Cells grid brings its current cell into view with no animation: its arrow keys' handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously (tests/blitz-tests/tests/stand_keyboard.rs:293-335; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
@binding n=1
**Motion ↔ the scope's MountedData calls**: scroll_to and scroll reachable during render must reach the same engine behaviour as from a handler — Smooth and Instant supported (design-system/motion fct-vb77s7g2), Smooth animating 300 ms in the viewport (design-system/motion fct-wr73fuj5), nested boxes written at once (design-system/motion fct-vt52hpbo; design-system fct-tnwqvola) — and the Cells grid's Instant scroll_to kept (design-system/motion fct-z2ugu37c).
@binding n=2
**Surfaces ↔ the engine change**: every Dioxus document renders through the buffer — the windowed launch (design-system fct-kdmcvdbg) and the wasm canvas (design-system fct-x3p4qjpb) — with the document, element ids and mounted events unchanged.
