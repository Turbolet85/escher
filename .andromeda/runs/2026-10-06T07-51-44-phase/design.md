# design extract

## Relevance
partial. The chunk adds no UI surface, token, component or motion. Upstream's merge does touch engine files that the design plan cites as the source of engine-painted defaults (blitz-paint `render.rs`, blitz-dom `node/node.rs` and `node/text.rs`, plus stylo_taffy alignment and the Parley bump that the stand's text rendering depends on). Design's only stake is that those as-built defaults change only as upstream changes them, and never through an escher edit.

## Constraints
- The engine-painted defaults (scrollbar palette, canvas background fallback, form-control and debug-overlay colours) are as-built values in upstream-owned paint code. The sync takes upstream's version of them as it stands and adds no escher retune (per design-system §Color Palette, "Engine-painted defaults" and the canvas-background bullet).
- The stand's text depends on the bundled DejaVu Sans registered for every generic through `build_single_font_ctx` and `HarnessOptions.font_ctx`, with system fonts off. Upstream's Parley bump (`718dcb72` → `e41dfea5`) must leave that loading path working. Whether the bumped Parley changes the `FontContext` and `build_single_font_ctx` surface is research's question (per design-system §Typography "Loading").
- Engine text mapping (Stylo→Parley generics, line-height, spacing, alignment) and the text-input editor size live in blitz-dom files the merge changes (`node/text.rs`, `layout/inline.rs`). Any change to them must come from upstream, not from us (per design-system §Typography "Engine text mapping" and the "Text input editors" row).
- The spacing behaviour baked into the engine, such as vertical centring of single-line inputs and the outside list-marker offset, sits in `node/node.rs` and `render.rs`, which both sides changed. It is carried exactly as merged (per design-system §Spacing "Engine spacing").
- Stacking-context roots, paint order and hit-test order are defined in `node/node.rs` and `render.rs`. The merge's alignment and inline-fragment changes must leave that order intact. Whether upstream's #1062 (inline fragment rects moved onto `Node`) touches this code is research's question (per design-system §Depth Strategy "Engine depth behavior").
- Overlay-scrollbar fade timing and smooth-scroll easing are as-built motion values. The sync carries them unchanged unless upstream changes them (per design-system §Motion "This project's values").

## Patterns to follow
- Treat the design plan as a record of observed values with file:line citations, not as a token set. The merge keeps whatever upstream ships and introduces no tokens (per design-system §Color Palette "Token sets", which records engine colour tokens as observed absent).
- The stand boots at a pinned viewport with its bundled font and no system fonts, so the merge's layout and Parley changes are judged against that fixed text environment (per design-system §Typography "Loading").
- Several plan citations point at line ranges in files the merge rewrites (`render.rs`, `node/node.rs`, `node/text.rs`, `stylo_taffy/convert.rs`). Record the ones that go stale for a later drift pass. Do not fix them in this chunk (per design-system §Color Palette, §Spacing and §Depth Strategy citations).

## Anti-patterns to avoid
- Do not use the sync to retune or tokenize engine-painted colours, type sizes or radii. The plan records no project token set, and any such change would be a new escher edit to upstream-owned code (per design-system §Color Palette "Token sets").
- Do not fix a rendering difference caused by the Parley or Taffy bump by editing a paint or font default on one side only. A gate failure there is a named, minimal, non-additive fix, or a halt (per design-system §Typography "Loading" and "Engine text mapping").

## Contract bindings
- Painted colours are a11y surface (CLAUDE.md Critical Warnings, SC 1.4.3). If upstream changes any engine-painted default colour (scrollbar thumb, focus outline in default.css, form controls), that change binds to the a11y contrast check.
- The stand's bundled-font loading (§Typography "Loading") binds to the tests harness. The `stand_*` checks are the proof that text layout survived the Parley bump.

## Acceptance criteria contributions
- The escher-side edits in the merge commit (its diff against the upstream parent) change no colour, font-size, radius or motion literal in upstream-owned paint or style code (`blitz-paint/src/render*.rs`, `blitz-dom/assets/default.css`, `blitz-dom/src/node/scrollbar.rs`) (per design-system §Color Palette "Engine-painted defaults").
- After the merge, the stand still registers bundled DejaVu Sans through `build_single_font_ctx` and `HarnessOptions.font_ctx` with system fonts off, and `stand_boot`, `stand_counter`, `stand_crud`, `stand_flight_booker` and `stand_timer` pass (per design-system §Typography "Loading").
- Any design-plan citation into a merged file that no longer resolves to the cited behaviour is listed as a drift note in the chunk's records and left unfixed (per design-system §Color Palette / §Spacing / §Depth Strategy citations).
