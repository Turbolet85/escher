@history_item n=1 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** (Established Decisions: 2 rows) — added `decision/canvas-crosses-as-pixels` (a canvas's app-painted content crosses only as pixels in a shot, with no new carrier, verb, field, exit, cause, session error, log field or span; ratified, F1 «Ратифицировать для всех каналов») and `decision/harness-paints-custom-widgets` (blitz-test-harness names blitz-paint's `custom-widget`, guard `stand_canvas_shot`). This chunk's direct predecessor: canvas text must stay inside that pixel-only crossing (the scope's settled F3), and `stand_canvas_shot` is the standing widget-paint reading the placement fix must keep green.

@history_item n=2 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`CustomWidget`'s attribute replay) — recorded that `ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget` (upstream #1322, `a26c6ddeb`), an upstream engine change ratified for its take. The precedent of a change to upstream's `Widget` trait behaviour; the new defaulted font-context method on the same trait is a fork divergence, flagged upstreamable, on the same merge surface (`packages/blitz-dom/src/node/custom_widget.rs`, `node/element.rs`).

@history_item n=3 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the culling bounds' port; blending painted) — `render_element`'s border box and culling were re-based on the scaled size with `location` and `box_position` kept for the transform; a viewport-wide blend layer is pushed by `render_element` and popped last, so the select list paints past it; the blend has no frame witness. Bears on the placement fix in `packages/blitz-paint/src/render.rs`: the widget's scene is appended inside the element's transform beside these layer pushes, and the fix must keep `M · T(content origin)` consistent with this geometry.

@history_item n=4 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six upstream behaviour changes for every document, ratified for this take) — created what is now `decision/upstream-behaviour-changes-ratified-per-take`: an upstream behaviour change for every document is taken only ratified for its take and is not counted among escher's direct engine changes. The placement fix (upstream issue #1578 open, no upstream fix) is escher's own direct engine change, counted under `decision/direct-engine-changes-flagged-upstreamable`, not an upstream take.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes of this chunk) — eight ratified direct engine changes for every document, each flagged upstreamable and nothing sent, their merge surface a line owned by the next upstream sync. The pattern this chunk follows: the paint placement, the pointer-event frame and the font-context accessor are engine changes for every document behind `custom-widget`, ratified by F1/F2, flagged upstreamable with a merge-surface owner named.

@history_item n=6 marker=2026-11-22-epoch-2-corrective
**2026-11-22-epoch-2-corrective** — added `decision/fork-additions-in-own-functions`: a fork addition to an upstream function over the cognitive-complexity ceiling goes into a function of escher's own with one call in upstream's body (founder's «Выносить наши добавки в хелперы»). Applies directly to edits inside `process_custom_widget_node`, `draw_custom_widget` and `handle_dom_event`'s `custom-widget` arm.

@history_item n=7 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host cognitive-complexity reading) — added the dev-host `rust-code-analysis-cli` reading of each function against the ceiling of 15, never in CI (`decision/cognitive-complexity-ceiling`). The measure the fork-addition rule reads when the chunk edits upstream paint and event functions.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the harness's screenshot, the password mask, the image in band, `png`) — the harness screenshot paints through blitz-paint into `VelloCpuImageRenderer` and composites over white after paint; `BaseDocument` gained two public read-only methods (`can_paint_text`, `layout_as_text_input`); the password mask landed as a direct engine change for every document with merge surface `render.rs` and `document.rs`; the shot crosses in band. Precedent for adding a public `BaseDocument` accessor (here `font_ctx()`) and for a ratified paint change in `render.rs`; `can_paint_text` reads the same font context a widget will now receive.

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the `screenshot` verb, `fonts-unavailable`, `Outcome::Shot`) — the `screenshot` verb, an element shot cropped to its visible part, and `fonts-unavailable` refused before anything is painted when no `sans-serif` family resolves. The shot contract the scope says must stand (800 × 600 over white, `fonts-unavailable` first, nothing changed by a shot).

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the scopes, the tests and the stack: `image`, three test targets, `png`) — `png` (already in the graph) taken by escher-driver and blitz-tests with no package entering `Cargo.lock`; `stand_screenshot_repeat` byte-identity. Precedent for this chunk's `parley` as a blitz-tests dev-dependency already in the lock, and for shot byte-identity on one host.

@history_item n=11 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — `HarnessOptions.font_ctx` → `DocumentConfig.font_ctx`; the stand's `font_ctx()` loads the bundled DejaVu Sans with system fonts off; the crate-root `DEJAVU_SANS` shared with the wasm entry. The bundled bytes (`examples/seven_guis/assets/DejaVuSans.woff2`) both F2 routes shape from, and the font context the document — and now its widgets — read.

@history_item n=12 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's typography in the Headless stand contract, PROVISIONAL) — the stand's typography is DejaVu Sans, stated in the Headless stand contract rather than on design-system's product surface; ratified at 2026-10-27-upstream-sync-engine-debts (F2). The row `decision/stand-typography-dejavu-sans` that canvas text from the bundled fonts must read the same as the document's text.

@history_item n=13 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (Standard Contracts: 2 rows, then its guard) — `decision/stand-typography-dejavu-sans` restated: every stand boot loads the one bundled font with system fonts off, every generic and unavailable named family falling back to it, guard `stand_bundled_font_families`; added `decision/stand-readings-same-on-every-ci-os` (compared with no screenshot). A widget shaping from the document's font context inherits this fallback; cross-OS shot equality stays out of this chunk.

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the replay API, its targets, and the shot's same-host limit) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared across OSes holds no `screenshot` step; `stand_screenshot_repeat` restated as one-host. Trap for this chunk's canvas-text checks: pixel readings are same-host; cross-host canvas equality is «Canvas shot across hosts».

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest, and the committed checks) — a kept check keeps each answer whole, masked, a shot's without its pixels; the committed Counter checks hold no `screenshot` step. Rows `decision/kept-check-masking` / `contract/kept-check/answer`: canvas text, crossing only as pixels, never reaches a kept check.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers, a boundary widening) — a shot of an open select list carries option labels as painted pixels, ratified as a boundary widening, with no new carrier, key, field or span. The precedent for page text crossing only as pixels that `decision/screen-pixels-in-band` and the canvas crossing extend.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's paint and a group's header) — `paint_scene` paints the open list last; the carve-out to «paint assumes resolved layout» (a row's text laid out at paint, the `decision/paint-to-paint-scene` except); `stroke_text_as` with `RunPaint`, a synthesized bold under `font-embolden`. Precedent for text shaped at paint time, which canvas text drawn in a widget's `paint` also is, and the embolden trap for any text a paint change draws.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (no local gate builds `font-embolden`) — a local gate read green over a paint defect only CI's macOS leg showed; `--features blitz-paint/font-embolden` named the local witness. Closed by 2026-11-01-upstream-sync-ui-graph's `embolden` step of `fast`; a blitz-paint change must read green under that feature locally.

@history_item n=19 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (Stack and Technologies; Standard Contracts) — `decision/local-pre-push-gate` and `contract/ci-leg-legs/fast` gained the dev-host target sweep after a green verdict; `embolden` the last leg of `fast`. The gate this chunk's push runs.

@history_item n=20 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — the `doc` leg runs `cargo doc --workspace --no-deps` under `-D warnings` over every library crate. The scope's two new public items (`Widget`'s defaulted method, `BaseDocument::font_ctx()`) need `///` docs, and the `doc` leg joins the local gate.

@history_item n=21 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added `stand-repeat` (three rounds, zero failures) and clippy's second line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets`. New code behind `custom-widget` in blitz-dom must pass that feature-subset clippy, and new stand checks run under the zero-failure budget.

@history_item n=22 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (Standard Contracts: 4 rows) — added `decision/skia-legs-verify-archive-first`: the clippy, test, build, msrv, doc, coverage and a11y legs verify skia's archive first. Applies to every leg this chunk's gates run.

@history_item n=23 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`text-transform-icu` in four default lists) — blitz-dom's default feature list holds `custom-widget`, so the placement, pointer-frame and font-context changes reach every default build of blitz-dom, remote pages included. Grounds the scope's «engine change for every document behind `custom-widget`» and `decision/feature-gated-widgets`.

@history_item n=24 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (the second upstream sync: Parley's pin form) — parley moved to a registry version with the coupling rule «parley, skrifa and vello move together»; later restated by 2026-11-17-upstream-sync-ahead-of-polish-and-ship as `decision/skrifa-matches-parley` and bumped to Parley 0.13 by 2026-11-19-upstream-sync-ahead-of-the-debts (`decision/parley-text-layout`). A check's widget shaping with `parley` must take the workspace's one parley (`{ workspace = true }`), no second version.

@history_item n=25 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Stack and Technologies: 2 rows; Established Decisions) — `decision/anyrender-rendering-abstraction` backends renamed vello-hybrid → vello-gpu; `decision/skrifa-matches-parley` replaced the two font-crate rows, a second skrifa standing beside parley's from the renderer set. The renderer and font-crate rows a widget's scene and its shaped glyph runs pass through.

@history_item n=26 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Stack and Technologies: 4 rows) — `decision/parley-text-layout` now Parley 0.13; Stylo 0.24. The text stack canvas text is shaped with.

@history_item n=27 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Cross-cutting Patterns: 4 rows; Conventions) — added `decision/events-queued-on-document`, retiring `decision/dispatch-event-callback`: every escher event path queues through the document. The pointer-frame change in `packages/blitz-dom/src/events/mod.rs`'s `custom-widget` arm sits on this queued model.

@history_item n=28 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (`PointerLeave`) — `UiEvent` gained `PointerLeave`, clearing hover in a hosted sub-document too (row `contract/ui-event/pointer-leave`). One of the variants `map_dom_event_to_ui_event` maps for a widget; F1 option C moves the widget's pointer coordinates into the content-box frame while a sub-document's keep the border-box origin.

@history_item n=29 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the geometry readers) — `Node::absolute_position` restated: adds the node's own location and subtracts only its containing blocks' scroll (ratified, guard `scrolled_box_absolute_position`); rows `decision/absolute-position-ancestor-scroll-only`, `contract/scroll-hit/absolute-position`. The widget's pointer origin is read through `node.absolute_position(0.0, 0.0)` (`events/mod.rs:134`); the content-box offset (border + padding) is added on top of this reading.

@history_item n=30 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** (Standard Contracts: 3 rows) — `contract/scroll-hit/hit-clip`: a hit stopped at a clipping box answers in that box's content-box frame, the frame of the final self hit and the text hits. The engine's existing content-box frame convention that a widget's pointer events (F1 C) and its paint now align with.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (visible_region through transforms) — `visible_region` reads each clipping box through its transforms via `transformed_bounds` (`decision/visible-region-through-transforms`, ratified engine change). The transform handling an element shot's crop of a transformed widget reads; the scope keeps a widget's event coordinates untransformed, as today.

@history_item n=32 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the `Document` trait gained a defaulted method (`accessibility_tree()`) that a wrapper may override, chosen as the crossing because a provider could not reach the vdom; trap: a feature-gated item can pass under `cargo test --workspace` and be compiled out under `-p blitz-tests` — check `cargo tree -e features -i`. Precedent for F2's new defaulted method on upstream's `Widget` trait behind `custom-widget`, and the feature-unification trap for a check reaching it.

@history_item n=33 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests keeps one test target per file) — blitz-tests keeps one test target per file under `tests/`, no `autotests = false`, pinned by `BlitzTestsTargetsTest`. A new canvas-text or placement check is its own target file.

@history_item n=34 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Conventions: 1 row) — `decision/both-layout-modes` gained its exception: upstream-taken targets kept as taken may run one mode; escher's own targets run both. Every check this chunk adds runs both layout modes (`decision/scenarios-both-layout-modes`).

@history_item n=35 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` style traversals serialised; stack, conventions) — headless documents of the harness, stand and driver never set `Parallel` (`decision/headless-documents-sequential-style`); the WPT runner's direct `mimalloc` named the one exception to `decision/workspace-dependencies`. The stand boot this chunk's checks use stays sequential, and blitz-tests' `parley` dev-dependency takes `{ workspace = true }`.

@history_item n=36 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Established Decisions: 1 row) — `decision/no-git-dependencies` gained its except: the taffy crate alone may take a git source. The scope's `--locked` legs and «no package enters the lock» rest on it.

@history_item n=37 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Test harness and Headless stand) — the harness holds one `VirtualClock` its documents share; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (`decision/one-virtual-clock`, `contract/test-harness/virtual-clock`). A shot of a widget paints at the harness's own time; canvas-text checks advance nothing.

@history_item n=38 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs) — `Cause` closed at eight, `time-unavailable` retired (`decision/refusal-causes-closed-at-eight`). The scope adds no cause; `fonts-unavailable` stays the shot's one font refusal.

@history_item n=39 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Standard Contracts: 4 rows (2 of 2)) — `contract/headless-stand/boot` lists the four lean tasks, Cells, a load page, the order page and one component alone (`boot_alone`, from 2026-11-17-a-component-in-isolation); `decision/stand-readings-same-on-every-ci-os` seven served tasks. The stand boots a canvas check mounts in; its served tasks' readings must not move.

@history_item n=40 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (Standard Contracts: 9 rows) — `contract/base-document/layout-reason` and the layout-reason types landed (in process at first, crossing every carrier from 2026-11-17-layout-on-the-carriers, `decision/layout-answer-crossing`, whose answer carries a `canvas` placing field). The layout read reports the widget's box; the paint-placement fix moves where the scene lands, not layout, so these readings should not move.
