# Classified — arch · existing-scopes.b1

## Answers
existing-scopes#1.1 · no · the platform AccessKit adapter, which the headless lint and stand_lint never reach
existing-scopes#1.2 · no · an opt-in phase timer that neither the lint nor the refactor uses
existing-scopes#1.3 · no · a module list; the no-NodeId rule for findings binds the driver's own Finding type, and NodeId's versioned-slot rule sits in 1.12
existing-scopes#1.4 · yes · names text_extent, the module of the BaseDocument::text_extent reader the lint's per-node walk calls, which must stay read-only
existing-scopes#1.5 · no · the JS-facing getComputedStyle, CSS.supports and style editing, none of which the chunk calls; the lint's contrast reads go through text_contrast (1.24)
existing-scopes#1.6 · no · form submission, iframe loading and query selectors are off the lint's path
existing-scopes#1.7 · yes · CSSOM View geometry gives the rects the snapshot's bounds read, which the overlap, clipped, off-screen and zero-size findings and stand_lint's pinned rows measure
existing-scopes#1.8 · yes · the snapshot is built from the accessibility tree this builder writes, so the lint's node order, names and the flight booker's select-option nodes behind its overlap come from it
existing-scopes#1.9 · no · a module list; the lint dispatches no event and reads no focus, and the stand_lint Creates are test steps this entity does not state
existing-scopes#1.10 · no · the lint runs no scroll and reads no visible region, its readers being the snapshot, text_extent and text_contrast
existing-scopes#1.11 · yes · Stylo's computed styles feed text_contrast, Device management sets the viewport the off-screen findings measure against, and the Stylo-to-Parley conversion sets the family and size each text box is measured at
existing-scopes#1.12 · yes · versioned node storage means a stale id must not be indexed, so the walk keeps text_extent's non-panicking None path, and traversal's inline-root collection gates what text_extent reads
existing-scopes#1.13 · yes · the inline layout pass builds the text box text_extent reads and box layout gives the measured bounds, and stand_lint asserts both layout modes, so incremental and non-incremental layout must agree
existing-scopes#1.14 · no · a module list; the computed styles and text the lint reads arrive through text_contrast and the inline layout (1.11, 1.13), and the chunk calls none of this module's API
existing-scopes#1.15 · no · the stand's screens reach the DOM through the Dioxus renderer, not this TreeSink, so the pinned rows read no parsed page
existing-scopes#1.16 · no · the stand's one load-page fetch is answered by its in-process LoadNet stand-in, and the chunk reaches no network
existing-scopes#1.17 · no · the lint paints nothing and reads layout and computed styles, not pixels, and no shot is taken
existing-scopes#1.18 · no · the Boa script runtime and its DOM bindings, which the chunk does not reach and the Dioxus stand does not use
existing-scopes#1.19 · no · windowing and the AccessKit adapter wiring, which the headless stand and the lint never reach
existing-scopes#1.20 · no · a re-export umbrella that the chunk names no module of
existing-scopes#1.21 · no · a module list; the geometry the findings measure reaches the lint through the geometry and layout entities (1.7, 1.13), and the chunk calls no conversion here
existing-scopes#1.22 · no · the windowed renderer, while the headless stand_lint boots through dioxus-native-dom
existing-scopes#1.23 · yes · the snapshot model behind DioxusDocument::snapshot is the input the lint walks for stable ids, closed words and bounds, and it compiles only under accessibility, a gate the lint's code must keep
existing-scopes#1.24 · yes · Harness::text_contrast is the contrast reader the lint's contrast defect calls under stand_contrast's exempt rule and must leave the harness's VirtualClock unmoved; the screenshot module is not reached
existing-scopes#1.25 · no · the sink's allowlist, scrub and span printing are unchanged, the chunk adds no log site, print, span or field, and Session::run, not the lint's walk, records the command span
