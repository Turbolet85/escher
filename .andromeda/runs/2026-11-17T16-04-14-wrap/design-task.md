@task_prose part=session
The session took five commits from upstream Blitz main (pin e5305a4c1) and ported the engine's paint to AnyRender 0.15 as one coupled dependency set (anyrender 0.15, vello 0.12, usvg 0.50, zip 9.6.1, the GPU renderer crate anyrender_vello_gpu replacing anyrender_vello_hybrid).
The GPU renderer name changed everywhere: the dioxus-native feature vello-hybrid and the apps' and examples' feature hybrid are now vello-gpu, and the justfile recipe hybrid is now vello-gpu; the hybrid names are freed.
Every paint fill, stroke and glyph draw now passes anti-aliasing explicitly, and box shadows are drawn in AnyRender 0.15's outset-unclipped form; the six tasks' screenshots differ from the previous take by a few hundred to a few thousand pixels, max channel delta 30, with no colour or token moved.
Upstream's picks add a scroll clamp at layout: a scroll offset is clamped to its node's scroll range when layout settles, after a viewport resize and when a subtree is removed; after a delete shrinks the CRUD list, the list's offset reads 0 and its first row lies inside the list's box, so the stand's scroll test was re-pinned.
Upstream's picks also fetch external images named by an SVG image or feImage href as stamped sub-resources, and the use element's href is not fetched; skrifa 0.49, read-fonts 0.46 and harfrust 0.15 join beside 0.44, 0.41 and 0.12.
CI's default-features test job now frees disk and reads df before and after its leg; four zip advisory ignores were dropped; every gate is green, and CI is green on the second attempt after a runner shortage.
@task_file n=1 path=Cargo.toml
what: the workspace manifest, holding the rendering dependency set (anyrender, vello, usvg, the GPU renderer crate key) and zip
@task_file n=2 path=Cargo.lock mark=merge
what: the lockfile, which pins every resolved package version, including the font stack (skrifa, read-fonts, harfrust)
@task_file n=3 path=deny.toml
what: the dependency audit's advisory ignore list and licence policy
@task_file n=4 path=packages/blitz-dom/Cargo.toml mark=merge
what: the engine DOM crate's manifest and its svg feature
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document, which now holds the image fetch built through the stamped request constructor
@task_file n=6 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the DOM mutator: image loading for img and the SVG image and feImage href, and the cleanup of a removed subtree's scroll state
@task_file n=7 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the engine's scrolling: node scroll ranges, offset clamping, viewport scroll and scroll animation clamping
@task_file n=8 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the layout pass's final-layout step, where a node's scroll offset is now clamped
@task_file n=9 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout tree construction from the DOM, including inline and top-level svg elements
@task_file n=10 path=packages/blitz-dom/src/resolve.rs mark=merge
what: style resolution and the device-change flush, which now clamps the viewport scroll and scroll animations
@task_file n=11 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's resource fetch handlers for the document's sub-resources
@task_file n=12 path=packages/blitz-dom/src/node/element.rs mark=merge
what: the element data of a DOM node, now reading an SVG element's href or xlink:href
@task_file n=13 path=packages/blitz-dom/src/util.rs mark=merge
what: the engine's SVG parsing, which now resolves an inline svg's image children through the image cache
@task_file n=14 path=packages/blitz-paint/src/text.rs mark=merge
what: the text painter, which draws glyph runs, decorations and text selection
@task_file n=15 path=packages/blitz-paint/src/render.rs
what: the engine's scene painter, which walks the layout tree and paints each box, its clips and its stacking-context layers
@task_file n=16 path=packages/blitz-paint/src/render/background.rs
what: background colour, gradient and image painting
@task_file n=17 path=packages/blitz-paint/src/render/border.rs
what: border and outline painting
@task_file n=18 path=packages/blitz-paint/src/render/box_shadow.rs
what: box shadow painting
@task_file n=19 path=packages/blitz-paint/src/render/form_controls.rs
what: the painting of form controls: checkboxes, radio buttons and the select box
@task_file n=20 path=packages/blitz-paint/src/render/select_list.rs
what: the painting of an open select list overlay, its rows and its engine colour constants
@task_file n=21 path=packages/blitz-paint/src/filters.rs
what: the conversion of CSS filters into paint filters
@task_file n=22 path=packages/blitz-paint/src/debug_overlay.rs
what: the debug overlay that paints layout box outlines
@task_file n=23 path=packages/dioxus-native/Cargo.toml
what: the dioxus-native integration's manifest and its public renderer features, now vello-gpu in place of vello-hybrid
@task_file n=24 path=packages/dioxus-native/src/dioxus_renderer.rs
what: the dioxus-native window renderer selection and its renderer options, now aliasing the Vello GPU renderer
@task_file n=25 path=packages/dioxus-native/src/lib.rs
what: the dioxus-native launch entry and configuration
@task_file n=26 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand's scroll-into-view test over the CRUD list, now pinned on the clamped offset after a delete
@task_file n=27 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the test that engine requests carry their document as initiator and local files are refused, now with the SVG image load
@task_file n=28 path=tests/blitz-tests/tests/fragment_navigation.rs mark=merge
what: fragment navigation and scroller tests, now with a percentage-height scroller clamped after a viewport resize
@task_file n=29 path=.github/workflows/ci.yml
what: the CI workflow: its jobs, the free-disk step and the disk readings
@task_file n=30 path=.github/scripts/test_ci_workflows.py
what: the unit checks pinning the CI workflow's shape, including which jobs free disk
@task_file n=31 path=.github/scripts/ci-leg.sh
what: the CI leg runner, whose wasm leg now builds with the vello-gpu feature
@task_file n=32 path=justfile
what: the developer recipes, with the GPU renderer recipe renamed vello-gpu
@task_file n=33 path=wpt/WPT_COMMIT mark=merge
what: the pinned web-platform-tests commit
@task_file n=34 path=wpt/runner/src/main.rs mark=merge
what: the WPT runner's entry and its result output
@task_file n=35 path=wpt/runner/src/test_runners/mod.rs mark=merge
what: the WPT runner's test-kind dispatch
@task_file n=36 path=wpt/runner/src/test_runners/ref_test.rs mark=merge
what: the WPT runner's reference test, rendering and comparing two pages' pixels
@task_file n=37 path=wpt/runner/src/test_runners/crash_test.rs
what: the WPT runner's crash test, which renders a page and checks it does not crash
@task_file n=38 path=apps/browser/src/capture.rs
what: the browser app's screenshot capture
@task_file n=39 path=examples/screenshot.rs
what: the screenshot example rendering a page to an image
@task_file n=40 path=examples/wasm_hello/src/lib.rs
what: the WASM hello example rendering into a canvas, now naming the vello GPU backend
@task_item n=1 part=touches
Every row naming the GPU renderer as hybrid or vello-hybrid, a renderer feature, a backend choice or a recipe: the name is now vello-gpu, and hybrid is freed.
@task_item n=2 part=touches
The dioxus-native configuration contracts (base_color, the window compositing alpha mode, the font context): the renderer options they live on now alias the Vello GPU renderer's options; check that each still holds as written.
@task_item n=3 part=touches
Buffered render then replayed under one borrow: the dioxus-native window renderer it rules over was reworked for the GPU renderer rename.
@task_item n=4 part=touches
Engine colours hardcoded, and select list colours as engine constants: the painters holding those constants now use AnyRender 0.15's call form with explicit anti-aliasing, and no colour value moved.
@task_item n=5 part=touches
Select list painted after the root render, the open select list answering hits first, and the select chosen row's Chromium pair: the select list and form-control painters were ported to the new paint API.
@task_item n=6 part=touches
Stacking-context root groups paint and hit-testing, positioned auto z-index paint level, and hit-testing stopping at a paint clip: the scene painter that applies them was ported to AnyRender 0.15.
@task_item n=7 part=touches
Screenshot never paints with text skipped, and the screenshot verb's contracts, including a row of an open select list: the text painter and the capture paths changed, and the six tasks' screenshots differ by a few hundred to a few thousand pixels, max delta 30.
@task_item n=8 part=touches
The CRUD task's contract and the into-view scroll decisions (driver scroll is instant, and the two rows on into-view scroll in nested boxes): after a delete shrinks the list, its offset is now clamped to 0 and the first row lies inside the list's box, so a row's position after a delete reads differently.
@task_item n=9 part=touches
Smooth scroll-behavior and the wheel cancel, plus wheel and key scrolling of the open select list: scroll animations and the viewport scroll are now clamped to the scroll range on a device change and at layout.
@task_item n=10 part=touches
Overlay scrollbar fade timings and a shown overlay scrollbar taking a thumb drag: a removed subtree's scroll state is now reset, and offsets are clamped at final layout.
@task_item n=11 part=touches
Tests built with system fonts, the test engine built with system fonts, and the unavailable-family and generic-family fallbacks: the font stack now holds two versions each of skrifa, read-fonts and harfrust (0.44 with 0.49, 0.41 with 0.46, 0.12 with 0.15).
@task_item n=12 part=touches
Bundled font context on WASM, the WASM font-context contract and the web-spa overview's canvas: the WASM pages and the wasm leg now build with the vello-gpu feature.
@task_item n=13 part=touches
The WPT runner output and result-colour contracts: the runner's entry, test dispatch, ref test and crash test changed with the upstream picks and the paint port, and the pinned WPT commit moved.
@task_item n=14 part=touches
Mobile targets built, not tested: CI's per-OS runs still build android and iOS only; the macOS and iOS jobs were cancelled for lack of a runner and green on re-run.
@task_item n=15 part=touches
Any row whose cell names the CI jobs that free disk, or names the guard test_only_stand_repeat_frees_disk: that guard is renamed test_free_disk_only_on_test_and_stand_repeat, the free-disk step now stands on the default-features test job beside stand-repeat, and test_test_job_reads_its_disk is added.
@task_item n=16 part=touches
Any row naming an SVG image load, an image fetch or the engine's sub-resource kinds: external images named by an SVG image or feImage href are now fetched as stamped requests, and the use element's href is not.
@task_fact n=1 slug=motion
fact: The engine clamps an element's scroll offset to its scroll range when layout settles, after a viewport resize and when content shrinks; after a delete empties space in the CRUD list, its offset reads 0 in both layout modes.
@task_fact n=2 slug=surface
fact: The dioxus-native GPU renderer is named vello-gpu, the feature, the renderer options and the recipe alike; the name hybrid is no longer used for a renderer.
@task_fact n=3 slug=surface
fact: Every fill, stroke and glyph draw in the engine's paint is anti-aliased, set explicitly on each call.
@task_fact n=4 slug=new
fact: An inline or top-level svg renders its image and feImage children's external images, loaded through the document's image cache; a use element's href is not fetched.
@task_item n=1 part=untouched
Brand identity, the anti-patterns, the self-validation protocol and the design decisions log: no recorded intent was given or moved.
@task_item n=2 part=untouched
Colour, type, spacing, elevation and radius tokens and their absences: no token, colour value or scale was added or moved.
@task_item n=3 part=untouched
The escher-session commands, exit statuses, exit codes, stderr, stdout, mcp role and session-error contracts, and the actions, coverage and UI graph lines: no verb, answer or status changed.
@task_item n=4 part=untouched
The select list's events and the select's snapshot and accessibility-tree contracts, and which selects open a list: only its paint call form changed.
@task_item n=5 part=untouched
The shell provider contracts, the JS window object's contracts, MountedData during a render, and cursor and favicon resolution: none was reached.
@task_item n=6 part=untouched
The driver's virtual clock: advance, click, press and type, the load wait and settle, and animation driven by the virtual clock are not changed.
@task_item n=7 part=untouched
Presentational hints (font element, table cellspacing, body margins), lang setting the text locale, root-relative units and the color-scheme recascade: no style rule changed.
