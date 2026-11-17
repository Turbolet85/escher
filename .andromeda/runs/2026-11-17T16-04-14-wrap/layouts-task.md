@task_prose part=session
The session merged five upstream engine picks past the last sync point and ported the paint layer to AnyRender 0.15, bumping the renderer, vello, usvg and zip dependencies as one coupled set; skrifa 0.49 and read-fonts 0.46 now stand beside 0.44 and 0.41.
It renamed the native GPU renderer feature vello-hybrid and hybrid to vello-gpu across the native crate, the apps, the examples, the wasm leg and the justfile recipe; the hybrid name is freed and paint_bench's backend list now reads vello, cpu, gpu.
The engine now clamps each scroller's offset to its scroll range at layout rounding and after a viewport resize, and resets a removed subtree's scroll state; the CRUD list after a delete reads offset 0 with its first row inside the list's box, so that row is no longer refused off-screen.
The engine now fetches the href of SVG image and feImage elements as a stamped sub-resource request (not use); a remote document cannot reach a local file through it.
CI's default-features test job gained the free-disk step and two disk readings; four zip advisory ignores were retired; the six stand tasks' screenshots differ from the previous take by a few hundred to a few thousand pixels, max delta 30, no token moved.
Files outside the list below that the change also reached: the parley notes, the WPT runner and its commit pin, the readme app, the remaining examples' manifests and pages, the wasm guide, and two harness pages for the stack and the seven_guis service.
@task_file n=1 path=Cargo.toml
what: the workspace manifest — the renderer dependency set (anyrender, vello, usvg, the GPU renderer key)
@task_file n=2 path=Cargo.lock
what: the locked dependency graph — the font stack (skrifa, read-fonts, harfrust) and the renderer crates
@task_file n=3 path=deny.toml
what: the dependency audit's advisory ignore list
@task_file n=4 path=packages/blitz-dom/src/scrolling.rs
what: the engine's scroll state — scroll ranges, offset clamping, into-view scrolling and the viewport scroll
@task_file n=5 path=packages/blitz-dom/src/layout/mod.rs
what: the engine's final layout pass — where each scroller's offset is now clamped after layout rounding
@task_file n=6 path=packages/blitz-dom/src/layout/construct.rs
what: the engine's layout tree construction for elements, including inline and top-level svg
@task_file n=7 path=packages/blitz-dom/src/resolve.rs
what: the engine's resolve pass — the viewport and scroll animation clamp after a viewport change
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: the document mutator — image loads (now also SVG image and feImage href) and removed-subtree cleanup
@task_file n=9 path=packages/blitz-dom/src/document.rs
what: the engine document — the image fetch through the stamped request constructor
@task_file n=10 path=packages/blitz-dom/src/node/element.rs
what: the element data — an SVG element's href reading
@task_file n=11 path=packages/blitz-dom/src/util.rs
what: the engine's SVG parsing helper, now given the document's image cache
@task_file n=12 path=packages/blitz-dom/src/net.rs
what: the engine's network provider glue for document sub-resources
@task_file n=13 path=packages/blitz-dom/Cargo.toml
what: the engine crate's manifest — its svg feature
@task_file n=14 path=packages/blitz-paint/src/text.rs
what: the text painter — glyph drawing for every text run of the stand
@task_file n=15 path=packages/blitz-paint/src/render.rs
what: the paint pass over the layout tree
@task_file n=16 path=packages/blitz-paint/src/render/form_controls.rs
what: the painter of form controls — inputs, checkboxes, radios, the select
@task_file n=17 path=packages/blitz-paint/src/render/select_list.rs
what: the painter of an open select's option list
@task_file n=18 path=packages/blitz-paint/src/render/box_shadow.rs
what: the box-shadow painter
@task_file n=19 path=packages/blitz-paint/src/render/border.rs
what: the border painter
@task_file n=20 path=packages/blitz-paint/src/render/background.rs
what: the background painter
@task_file n=21 path=packages/blitz-paint/src/filters.rs
what: the CSS filter conversion for paint
@task_file n=22 path=packages/blitz-paint/src/debug_overlay.rs
what: the layout debug overlay painter
@task_file n=23 path=packages/dioxus-native/Cargo.toml
what: the native Dioxus renderer crate's public cargo features — the GPU renderer feature now vello-gpu
@task_file n=24 path=packages/dioxus-native/src/lib.rs
what: the native Dioxus launch entry — the window renderer chosen per feature
@task_file n=25 path=packages/dioxus-native/src/dioxus_renderer.rs
what: the native Dioxus window renderer aliases and options
@task_file n=26 path=apps/browser/Cargo.toml
what: the browser app's manifest and renderer feature
@task_file n=27 path=apps/browser/src/capture.rs
what: the browser app's screenshot capture
@task_file n=28 path=examples/seven_guis/Cargo.toml
what: the seven_guis stand's manifest and renderer feature
@task_file n=29 path=examples/seven_guis/index.html
what: the seven_guis stand's web page skeleton
@task_file n=30 path=examples/paint_bench.rs
what: the paint_bench command line — its arguments and backend list
@task_file n=31 path=examples/screenshot.rs
what: the screenshot example's paint of a document
@task_file n=32 path=justfile
what: the developer recipes — the GPU renderer recipe now vello-gpu
@task_file n=33 path=.github/workflows/ci.yml
what: the CI workflow — the default-features test job's free-disk step and disk readings
@task_file n=34 path=.github/scripts/ci-leg.sh
what: the CI leg runner — the wasm leg's renderer feature
@task_file n=35 path=.github/scripts/test_ci_workflows.py
what: the CI-script checks — the free-disk guards
@task_file n=36 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of a CRUD row past the list — refused, scrolled into view, then clicked; now the clamped reading after a delete
@task_file n=37 path=tests/blitz-tests/tests/fragment_navigation.rs
what: the engine checks of fragment navigation and scroller clamping after a viewport resize
@task_file n=38 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the check that a remote document's loads carry it as initiator and cannot reach local files
@task_file n=39 path=.claude/docs/commands.md
what: the harness page of commands — the renderer recipes and the paint_bench backend list
@task_file n=40 path=docs/architecture.mmd
what: the architecture diagram — the renderer backends
@task_item n=1 part=touches
contract/escher-session-refusals/off-screen — after a delete shrinks the CRUD list, its first row is no longer refused off-screen; the stand check's pinned unclamped reading was retired and re-pinned to the clamped one.
@task_item n=2 part=touches
contract/stand-element-ids/crud-list and contract/stand-element-ids/crud-delete — after a delete the list's scroll offset now reads 0 and its first row's centre lies inside the list's box, both layout modes.
@task_item n=3 part=touches
decision/crud-list-keyboard-region — the CRUD list's scroll behaviour changed under it (offset clamped at layout).
@task_item n=4 part=touches
decision/scroll-into-view-moves-every-scroller — every scroller's offset is now clamped to its range at layout rounding and after a viewport resize; a removed subtree's scroll state is reset.
@task_item n=5 part=touches
decision/visible-region-through-clipping-boxes — the visible region reads clamped scroll offsets now, indirectly through the scroll state change.
@task_item n=6 part=touches
decision/headless-stand-pinned-viewport — the engine now clamps scrollers after a viewport resize; indirect, through the viewport scroll clamp.
@task_item n=7 part=touches
decision/held-instance-reads-as-fresh-boot — removed subtrees' scroll state is now reset; indirect, a held instance's scroll readings after a delete.
@task_item n=8 part=touches
contract/escher-session-layout/answer and contract/escher-session-snapshot/answer — scroll offsets and row boxes they report after a delete now read clamped.
@task_item n=9 part=touches
contract/escher-session-served-tasks/crud — the served CRUD task's list reads clamped after a delete.
@task_item n=10 part=touches
contract/paint-bench-cli/backend — the backend list hybrid became gpu (vello_gpu accepted), taken whole from upstream.
@task_item n=11 part=touches
contract/paint-bench-cli/arguments — the paint_bench command line is now upstream's file whole.
@task_item n=12 part=touches
contract/escher-session-screenshot/answer — the paint port to AnyRender 0.15 (anti-aliasing on every fill, stroke and glyph run, the new box-shadow form) moves the six tasks' pixels by 427 to 2618 pixels, max delta 30.
@task_item n=13 part=touches
decision/stand-accent-colour — the paint layer under it was ported; no token or colour moved.
@task_item n=14 part=touches
decision/open-list-rows-own-bounds and decision/flight-type-select-boot-reading — the select and its open option list painter were ported; indirect through form control paint.
@task_item n=15 part=touches
contract/escher-session-host/log-service-name — the windowed smoke now runs on the vello-gpu feature and still logs service.name=seven_guis.
@task_item n=16 part=touches
contract/escher-session-refusals/fonts-unavailable — the font stack now holds skrifa 0.49 and read-fonts 0.46 beside 0.44 and 0.41; indirect through the font dependency set.
@task_item n=17 part=touches
decision/load-page-only-network — the engine now fetches SVG image and feImage href through the document's network provider as a stamped request; a remote document cannot reach a local file through it.
@task_item n=18 part=touches
decision/fixed-document-skeleton-main-mount — the stand's web page skeleton changed for the renderer feature rename; indirect.
@task_item n=19 part=touches
contract/shell-provider-window-chrome rows — the native window renderer behind them is now the vello-gpu renderer alias; indirect through the renamed renderer.
@task_item n=20 part=touches
decision/session-roles-share-stand-boot and decision/cold-agent-pipe-builds-nothing — any row naming the hybrid feature in the stand's build is stale; the feature is vello-gpu now.
@task_item n=21 part=touches
contract/agent-run-cli/stdout — run all now reads 874 passed (873 before), run stand 327 · 0 · 10 unchanged; a touch only where a row's cell holds that count.
@task_fact n=1 slug=surface-desktop-native
fact: After a delete shrinks the CRUD list's content, its scroll offset reads 0 and the first row's centre (208.5, 128.3) lies inside the list's box (24.0, 112.8, 393.0, 532.0), in both layout modes, and that row is clicked without an off-screen refusal.
@task_fact n=2 slug=surface-desktop-native
fact: Every scroller's offset is clamped to its scroll range at layout rounding and after a viewport resize, and a removed subtree's scroll state is reset, so no reading holds an offset past the content.
@task_fact n=3 slug=surface-desktop-native
fact: The native GPU renderer feature is named vello-gpu across the stand, the apps and the examples; the names vello-hybrid and hybrid are freed.
@task_item n=1 part=untouched
surface-mobile-native — no mobile layout code was reached; android and iOS only build.
@task_item n=2 part=untouched
surface-web-spa — no navigation, header, footer or IA moved; the wasm leg only renamed its renderer feature.
@task_item n=3 part=untouched
The stand stable element ids other than the CRUD list and delete — no id was added, renamed or retired.
@task_item n=4 part=untouched
Stand names, roles, states and ids as attributes alone, and stand lints clean at boot — no attribute or lint changed.
@task_item n=5 part=untouched
The MCP server contracts (stdio, tools list, protocol revisions, result bound, end) and the cold agent's MCP and prompt decisions — not reached.
@task_item n=6 part=untouched
The cold-agent pipe's command line, exit codes, record dir and Windows pass-through, and the agent-run usage, exit and Windows pass-through rows — no change of form.
@task_item n=7 part=untouched
The escher-session command line, exit codes, help, actions, graph, lint, coverage and recording contracts — not reached.
@task_item n=8 part=untouched
The multi_doc command line — not reached.
@task_item n=9 part=untouched
absence/arbitrary-index-html-templates — no template capability added.
@task_item n=10 part=untouched
decisions-log — no recorded intent moved; the session ratified no layout decision of its own beyond the clamped reading.
