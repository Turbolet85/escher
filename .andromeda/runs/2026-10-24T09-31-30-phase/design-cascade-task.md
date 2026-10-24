# Task — design · 2026-10-24-upstream-sync-polish-and-ship

## The chunk
Takes upstream DioxusLabs/blitz at its measured sha `f9cbb2756` into the fork. The take is a whole merge, cherry-picked engine commits on the registry pins, or nothing; that choice is made at P4.
Upstream's delta changes engine code: the inline layout module split, class atoms, out-of-flow flags, serialised style traversal, a replaced element's natural height through `aspect-ratio`, skipped animation-set lookups, dropped nodes skipped in hit testing, and Stylo `text-indent` keywords and `word-break: break-word`. Its pin moves are git Taffy, git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3, including a resync of `packages/dioxus-native(-dom)` and an edit to the stand's `cells.rs`.
The chunk also brings the dependency audit back to green with a semver-compatible tokio update, re-reads the per-OS test tallies and the telemetry sink allowlist, and re-reads the merge surfaces of the engine-debts, document-clock and select-model fixes. It sends nothing upstream.
It designs no new visual element. Its design question is whether any painted, laid-out, hit-tested or animated value the plan records moves under the take. A moved number is re-measured and recorded, never loosened.

## Needs
- The seven_guis stand's accent colour and its hover and active steps, and every stand text/background pair `stand_contrast` asserts. A Stylo or Taffy move, or upstream's new CSS keywords, could move a computed colour or a contrast row.
- The engine-painted defaults: the user-agent stylesheet `packages/blitz-dom/assets/default.css` and the select model's rules in it. CARRY 5 re-reads this file against the take.
- Engine text mapping and typography: Parley inline layout (upstream splits the module into `build.rs` and `layout.rs`), `text-indent` keywords and `word-break: break-word` as new computed text values, and the bundled stand font. All are read through the stand checks' bounds and screenshots.
- Font loading rules used by the stand and driver checks: the `system-fonts` feature on blitz-dom for blitz-tests, and `fonts-unavailable` refusing a screenshot or lint with nothing painted. These must keep holding on the merged tree.
- Engine spacing and layout values: box sizes through Taffy and Stylo. A replaced element's natural height now goes through `aspect-ratio`, and out-of-flow-ness is recorded on the node. Stand checks that assert exact bounds, a snapshot's byte size or a screenshot's pixels read these values.
- Engine depth behaviour: hit testing stops at the padding box of a box that clips by `overflow` or `contain: paint`, the root excepted. Upstream's `f102663b8` changes hit testing (`hit_inner`, skipping dropped nodes), on ground our fixes also hold.
- Engine radius handling and engine-observed border and shadow values, wherever a Stylo move could change how a stand control paints. Read through the screenshot and contrast checks only.
- Motion and the animation runtime: settle answers every animating member the same way and advances none, the harness `VirtualClock` drives animation time, the load wait moves no clock, and a programmatic into-view scroll is instant in nested boxes. Upstream's `1c5265a8b` changes animation-set lookups, and `scrolling.rs` (a CARRY 4 clock file) is a predicted conflict.
- Through the clock invariant: no wall-clock read in blitz-dom outside `clock.rs`, re-read on the take's tree, because smooth-scroll, fling, scrollbar fade and double-click timing are motion values read through that clock.
- Cursor rules (the CSS `cursor` keyword, else Text, Pointer or Default, and an arrow over no box). These reach the chunk through `packages/blitz-dom/src/events/pointer.rs`, which upstream's delta touches.
- The desktop-native surface's toolkit: Dioxus apps open through `dioxus_native::launch` on winit with the Vello window renderer and AccessKit. The Dioxus 0.8 move resyncs `packages/dioxus-native` and `packages/dioxus-native-dom`, and the stand and root examples must still launch and build.
- The desktop-native stack and platform notes: the browser app gets `mimalloc` as its global allocator (`apps/browser`), and `flake.nix` moves (upstream's `f9cbb2756`). That file is the desktop-native platform's cited source for the default package. Its citations may need re-pointing if lines move.
- The cli surface's output rules for `escher-session` and its `mcp` role: JSON-only stdout, no colour or ESC byte, no terminal test, and the four exit statuses. These must survive the take unchanged, and the stand's session and MCP checks re-prove them.
- The accessibility-facing visual surface: focus order and Enter/Space activation on the stand (`stand_keyboard`), and the stand's state colours for hover and active. The SC 1.4.3 contrast floor applies through the palette rows.
- The stand's non-lean task `examples/seven_guis/src/tasks/cells.rs`, which upstream edits. Any change to its layout or style values is read by `stand_non_lean_tasks` and `stand_actionable_keys`.
- Example fixture values the plan records for root examples and the stand (colours, sizes, radii in the example CSS), where the take changes an example file. This includes the root example `examples/multi_doc.rs` added with `fb297a38c`.

## Not needed
- Brand identity, anti-patterns, the self-validation protocol and the design decisions log: none of them has recorded intent, and a sync adds none.
- Project-wide token sets, type scale, spacing scale, elevation scale and radius scale: each is recorded as not yet measured, and the chunk measures none of them.
- The mobile-native surface (Android APK, iOS target, the urlbar, the back button, capture): the take moves no mobile-only code, and CI only builds those targets.
- The web-spa surface and its canvas tokens (WASM pages on `#f5f5f5`, a canvas sized from host CSS): the chunk runs no wasm32 page.
- Browser iconography: toolbar SVG icons, tab glyphs, icon sizes, favicons and rdme octicons. `apps/browser` gets only an allocator change.
- Desktop-native component patterns for the browser's titlebar and devtools overlay: they are upstream-only browser chrome that the take does not restyle.
- The cli surface's tokens for the WPT runner's `owo-colors` output: the WPT runner is an upstream-only surface whose conformance and output the chunk does not measure.
- New visual design for any escher surface: the chunk adopts no new upstream API in escher's own code, and it adds no control, colour or state.
- Semantic and status-state colours for stand states the chunk does not reach (none is added or restyled): these bind only as rows `stand_contrast` re-reads, not as values to change.
