@task_prose part=session
The session took upstream Blitz main up to e5305a4c1 by cherry-pick: 5 of 10 upstream commits taken, 5 left out, the left-out delta now 102 with 97 carried, the merge base pin moved from ea07ee716.
The take moved the rendering stack to AnyRender 0.15 as one coupled dependency set (anyrender 0.15, anyrender_vello 0.16, anyrender_vello_cpu 0.19, anyrender_skia 0.13, anyrender_svg 0.16, wgpu_context 0.11, vello 0.12, vello_cpu and vello_gpu 0.4, usvg 0.50), replaced anyrender_vello_hybrid by anyrender_vello_gpu, and bumped zip 2.4.2 to 9.6.1; skrifa 0.49, read-fonts 0.46 and harfrust 0.15 now stand in the lock beside 0.44, 0.41 and 0.12, which the founder ruled amends the font-crate rule and the sync dependency-set rule in their standing form.
Four engine behaviour changes arrived: SVG image and feImage href loads (not use) fetched as stamped sub-resources, scroll offsets clamped to the scroll range after layout rounding and on subtree removal, inline SVG resolving its images against the document, and paint calls taking explicit anti-aliasing with the new box-shadow form.
The GPU window renderer feature vello-hybrid (and the apps' and examples' hybrid feature and just recipe) was renamed vello-gpu; the four zip advisory ignores were retired from the audit config (9 to 5 ignores).
CI's default-features test job now frees disk with the same SHA-pinned action stand-repeat uses and reads disk before and after its test leg into the step log; the guard test_only_stand_repeat_frees_disk was renamed test_free_disk_only_on_test_and_stand_repeat and test_test_job_reads_its_disk added.
The stand's CRUD list scroll check was re-pinned to the clamped reading; the founder ruled the SVG widening stands as measured, image and feImage href.
@task_file n=1 path=Cargo.toml
what: the workspace manifest: the shared workspace dependency declarations, the anyrender and vello renderer set and the svg and font crates
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile: the resolved dependency set, its git sources and duplicated crate versions
@task_file n=3 path=deny.toml
what: the cargo-deny dependency and licence audit config with its advisory ignores
@task_file n=4 path=.github/workflows/ci.yml
what: the GitHub Actions CI workflow: its jobs, the SHA-pinned actions, the free-disk step and the default-features test job
@task_file n=5 path=.github/scripts/test_ci_workflows.py
what: the CI workflow contract tests pinning the workflow, including the free-disk guards
@task_file n=6 path=.github/scripts/ci-leg.sh
what: the one CI leg runner, its legs including the wasm leg and its renderer feature
@task_file n=7 path=packages/blitz-dom/Cargo.toml mark=merge
what: the blitz-dom crate manifest and its optional features, the svg feature
@task_file n=8 path=packages/blitz-dom/src/document.rs mark=merge
what: BaseDocument, the core document, now holding the image fetch through the stamping request constructor
@task_file n=9 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout box construction from the DOM
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the layout pass and its final-layout write, where scroll offsets are clamped after rounding
@task_file n=11 path=packages/blitz-dom/src/mutator.rs mark=merge
what: DocumentMutator, the DOM mutation path: image loading, SVG image href loads, subtree removal
@task_file n=12 path=packages/blitz-dom/src/net.rs mark=merge
what: blitz-dom's resource load handlers for images, stylesheets and fonts
@task_file n=13 path=packages/blitz-dom/src/node/element.rs mark=merge
what: ElementData, an element's data and readers, now the SVG href reader
@task_file n=14 path=packages/blitz-dom/src/resolve.rs mark=merge
what: the resolve pass of style and layout, its device-change flush and viewport scroll clamp
@task_file n=15 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the one scrolling primitive: node and viewport scroll offsets, their ranges, clamps and reset
@task_file n=16 path=packages/blitz-dom/src/util.rs mark=merge
what: blitz-dom utilities including SVG parsing into an image through usvg, with embedded images
@task_file n=17 path=packages/blitz-paint/src/text.rs mark=merge
what: blitz-paint text painting, glyph runs and text decoration
@task_file n=18 path=tests/blitz-tests/tests/fragment_navigation.rs mark=merge
what: the blitz-tests integration check of fragment-link and scroller behaviour, now the percentage-height scroller clamp
@task_file n=19 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT suite revision the WPT runner reads
@task_file n=20 path=wpt/runner/src/main.rs mark=merge
what: the WPT runner's entry and test loop
@task_file n=21 path=wpt/runner/src/test_runners/mod.rs mark=merge
what: the WPT runner's choice of test runner by test kind
@task_file n=22 path=wpt/runner/src/test_runners/ref_test.rs mark=merge
what: the WPT reftest runner and its render target
@task_file n=23 path=docs/parley.md mark=merge
what: the upstream document on Parley text and fonts
@task_file n=24 path=wpt/runner/src/test_runners/crash_test.rs
what: the WPT crashtest runner and its render through the paint scene
@task_file n=25 path=packages/blitz-paint/src/render.rs
what: blitz-paint's paint of the tree into the paint scene, its fill, stroke and layer calls
@task_file n=26 path=packages/blitz-paint/src/render/background.rs
what: background and gradient painting
@task_file n=27 path=packages/blitz-paint/src/render/border.rs
what: border and outline painting
@task_file n=28 path=packages/blitz-paint/src/render/box_shadow.rs
what: box-shadow painting
@task_file n=29 path=packages/blitz-paint/src/render/form_controls.rs
what: form control painting, password masking, checkbox, radio and range
@task_file n=30 path=packages/blitz-paint/src/render/select_list.rs
what: the open select list painted inside the scene
@task_file n=31 path=packages/blitz-paint/src/filters.rs
what: CSS filter conversion for the paint scene
@task_file n=32 path=packages/blitz-paint/src/debug_overlay.rs
what: blitz-paint's debug layout overlay
@task_file n=33 path=packages/dioxus-native/Cargo.toml
what: the dioxus-native manifest, its renderer features (vello, vello-cpu, skia, vello-gpu) of which one is required
@task_file n=34 path=packages/dioxus-native/src/dioxus_renderer.rs
what: dioxus-native's window renderer chosen at compile time by feature
@task_file n=35 path=packages/dioxus-native/src/lib.rs
what: dioxus-native's crate root and launch configs
@task_file n=36 path=apps/browser/Cargo.toml
what: the reference browser app manifest and its renderer features
@task_file n=37 path=apps/browser/src/capture.rs
what: the browser's screenshot capture through the paint scene
@task_file n=38 path=examples/paint_bench.rs
what: the paint benchmark example CLI and its backend list (vello, cpu, gpu)
@task_file n=39 path=justfile
what: the developer recipes, the renderer recipe now vello-gpu
@task_file n=40 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of a scroll act on the CRUD list, the clamped reading after a delete
@task_item n=1 part=touches
The upstream sync merge base and cherry-pick decision: the pin moves from ea07ee716 to e5305a4c1, 5 taken, 5 left out of 10, the left-out delta 102 with 97 carried, not 112.
@task_item n=2 part=touches
The skrifa-follows-parley-and-vello rule and its twin font-deps-match-renderers: the lock now holds skrifa 0.49, read-fonts 0.46 and harfrust 0.15 beside 0.44, 0.41 and 0.12; the founder amended the rule in its standing form.
@task_item n=3 part=touches
anyrender as the rendering abstraction and the stack overview: anyrender 0.15 with anyrender_vello 0.16, anyrender_vello_cpu 0.19, anyrender_skia 0.13, anyrender_svg 0.16, vello 0.12, wgpu_context 0.11, taken as one coupled set.
@task_item n=4 part=touches
The window renderer chosen at compile time, dioxus-native requires a renderer feature and a renderer is required: the GPU renderer feature vello-hybrid renamed vello-gpu, anyrender_vello_hybrid replaced by anyrender_vello_gpu; the name vello-hybrid and hybrid freed.
@task_item n=5 part=touches
wgpu as the GPU layer: wgpu_context moved to 0.11 under the vello-gpu renderer.
@task_item n=6 part=touches
usvg for SVG parsing and svgtypes follows usvg: usvg moves to 0.50; the lock still holds one svgtypes.
@task_item n=7 part=touches
Headless screenshots and WPT default to the Vello CPU renderer: anyrender_vello_cpu 0.19 and vello_cpu 0.4 under them; the pixel readings measured equal to the trial.
@task_item n=8 part=touches
The dependency and licence audit, the cargo-deny advisory gate and workspace dependency audit: the four zip advisory ignores RUSTSEC-2026-0367, 0368, 0369, 0429 retired, ignores 9 to 5, zip at 9.6.1.
@task_item n=9 part=touches
No git dependencies: the one git source left is Taffy's pinned revision; AnyRender comes from the registry.
@task_item n=10 part=touches
Only stand-repeat frees disk and disk freed only in the stand-repeat job: stale, the free-disk step now stands on the default-features test job too; the guard named for it renamed test_free_disk_only_on_test_and_stand_repeat.
@task_item n=11 part=touches
The CI workflow contract pinned by tests: a new guard test_test_job_reads_its_disk pins the test job's two disk readings; the CI-script tests 37 to 38 methods.
@task_item n=12 part=touches
Actions pinned to commit SHAs: one more use of the free-disk action, pinned by SHA, in the default-features test job.
@task_item n=13 part=touches
The wasm leg and Trunk builds of browser-target examples: the wasm leg now builds with the vello-gpu feature.
@task_item n=14 part=touches
Upstream behaviour changes ratified per take and the upstream take as engine behaviour changes: four engine behaviour changes in this take need ratifying (SVG href load, layout-time scroll clamp, inline SVG images, explicit anti-aliasing paint calls).
@task_item n=15 part=touches
Engine sub-resource requests carry their document and blitz-dom requests only through the stamping constructor: the SVG image and feImage href load is a new stamped sub-resource kind, fetched through the moved image fetch.
@task_item n=16 part=touches
File URLs only for file documents: an SVG image href to a file URL from a remote document is refused like the other loads.
@task_item n=17 part=touches
Images cached by URL, one fetch: the image fetch moved from the mutator into the document and now serves SVG image href loads too.
@task_item n=18 part=touches
First-party inline SVG parsed into the DOM and inline SVG re-parsed into an image: inline and top-level SVG now resolve their image elements against the document URL and the image cache; an SVG used as an image resolves none.
@task_item n=19 part=touches
One scrolling primitive, programmatic scrolls clamp to one box and overflow hidden scrolls programmatically: scroll offsets are now also clamped to the scroll range after layout rounding, the viewport and a scroll animation clamped, a removed subtree's scroll state reset.
@task_item n=20 part=touches
Paint emits commands to a paint scene: every fill, stroke and glyph draw now names anti-aliasing explicitly, filters convert against a bounds rectangle and box shadows use AnyRender 0.15's outset form.
@task_item n=21 part=touches
png as the shot encoder: png now also joins blitz-dom's svg feature, to encode raster images embedded in SVG.
@task_item n=22 part=touches
The paint_bench example CLI contract: its GPU backend is named gpu (vello_gpu), not hybrid.
@task_item n=23 part=touches
The WPT runner mirrors upstream WPT tooling: the pinned WPT revision and the runner moved with upstream's picks.
@task_item n=24 part=touches
The ci-leg.sh test leg and stand-repeat leg contracts: the test job now frees disk and prints disk before and after the leg in its step log, with no upload.
@task_fact n=1 slug=established-decisions
fact: An SVG image or feImage element's href, unprefixed else xlink:href, is loaded as an engine sub-resource stamped with its document as initiator; a use element's href is not fetched (founder ruling, the measured scope).
@task_fact n=2 slug=established-decisions
fact: After layout rounding each scroll container's offset is clamped to its scroll range in the final-layout write, the viewport and running scroll animation are clamped, and a removed subtree's scroll state is reset; a list whose content shrinks after a delete reads offset 0.
@task_fact n=3 slug=established-decisions
fact: Inline and top-level SVG resolve their embedded image elements against the document URL and the image cache; an SVG loaded as an image resolves no embedded images.
@task_fact n=4 slug=infrastructure-patterns
fact: The default-features test job frees disk with the same SHA-pinned action as stand-repeat and reads disk before and after its test leg into the step log, uploading nothing; no other job frees disk.
@task_fact n=5 slug=stack-and-technologies
fact: The GPU window renderer is anyrender_vello_gpu under the feature vello-gpu across dioxus-native, the apps and examples; no hybrid renderer name remains.
@task_item n=1 part=untouched
Stylo, winit, parley, Dioxus and Taffy versions and the rules over them: none moved, and the Taffy git pin is unchanged.
@task_item n=2 part=untouched
The driver, its verbs, session socket wire, MCP role, kept checks and recording: no verb, argument, carrier or answer field changed.
@task_item n=3 part=untouched
Telemetry: no log site, print, env read or target added; the escher-telemetry sink and scrub rules stand.
@task_item n=4 part=untouched
The accessibility tree and its contracts, stable element ids, the snapshot and diff: untouched by the take.
@task_item n=5 part=untouched
Occupied resources and names: no binary, crate, port, socket, env var or listener added or removed.
@task_item n=6 part=untouched
The browser app's history, persistence and about pages, and the JS runtime with its script fetch: untouched.
@task_item n=7 part=untouched
Coverage floors, mutation tier, MSRV and toolchain policy: no threshold or floor moved.
