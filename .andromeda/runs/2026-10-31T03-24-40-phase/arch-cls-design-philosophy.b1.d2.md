# Classified — arch · design-philosophy.b1

## Answers
design-philosophy#1.1 · no · a crate-split value; the chunk adds no crate, moves no code across a crate line and bumps no dependency, so nothing it changes needs the split honoured
design-philosophy#2.1 · yes · the redraw check reads redraws requested through the shell provider, so it supplies its own ShellProvider rather than the no-op default, and the load page's LoadNet is the NetProvider its rows run on
design-philosophy#3.1 · yes · the spec and engine semantics the fixes and checks read: CSS Transforms' matrix for the transformed box, WAI-ARIA tree exclusion for inert, HTML-AAM role mapping for a native checkbox's or radio's toggled write, and WebKit/Blink interaction-state teardown as the reference for the focus fixup
design-philosophy#4.1 · yes · one incremental pipeline: incremental_oracle holds incremental and non-incremental layouts identical over any engine fix, and the geometry-generation memo of hoisted positions and stacking-context bounds is what the transform fix must stay inside
design-philosophy#5.1 · yes · the new Tab-order, keyboard-activation and contrast rows and the shell check run on the headless harness, with synthesized input routed through the real event-dispatch pipeline
