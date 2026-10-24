# Classified — arch · design-philosophy.b1

## Answers
design-philosophy#1.1 · yes · the merged blitz-dom, blitz-paint and dioxus-native-dom code and the manifest merge must keep each crate separately usable, blitz-dom headless and blitz-paint painting through PaintScene
design-philosophy#2.1 · yes · the new DocumentConfig.clock is an embedder-supplied service that falls back to WallClock when absent, the same no-op-default injection the config's providers use
design-philosophy#3.1 · yes · the merged accessibility, pointer, keyboard, select, range-input and layout code implements behaviour cited to HTML, WAI-ARIA and CSS specs, which the take must keep
design-philosophy#4.1 · yes · the merged layout must keep incremental and non-incremental layout identical, versioned node ids must stop resolving once dropped, and upstream's out-of-flow flag on Node must fit the rule that Node carries only tree structure
design-philosophy#5.1 · yes · the stand and blitz-tests checks run headless one target per file, with upstream's hit_test_dropped_nodes and stale_node_mapping added as targets and the harness's VirtualClock time honoured
