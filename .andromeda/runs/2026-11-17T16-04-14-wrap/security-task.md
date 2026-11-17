@task_prose part=session
Five upstream commits were cherry-picked from upstream blitz main, pinned at e5305a4c1, ahead of Epoch 6. They bring an SVG image fetch, scroll clamping at layout time and on subtree removal, and an image fetch moved into one document-level function built on the stamping request constructor.
The engine moved to AnyRender 0.15 as one coupled dependency set from the registry: anyrender_vello_hybrid was replaced by anyrender_vello_gpu, vello moved to 0.12 and usvg to 0.50, zip went from 2.4.2 to 9.6.1, and skrifa 0.49 and read-fonts 0.46 now sit beside 0.44 and 0.41. The lock grew from 988 to 998 packages, and Taffy's pinned rev is still the only git source.
The renderer feature vello-hybrid, and the apps' and examples' feature hybrid, were renamed vello-gpu.
deny.toml dropped the four zip advisory ignores (RUSTSEC-2026-0367, 0368, 0369 and 0429), taking the ignores from 9 to 5. The audit reads advisories ok and licenses ok.
The CI job test-features-default now runs the SHA-pinned free-disk-space action and two df -h steps before and after the test leg, so two jobs now free disk instead of one. The CI-script guard was renamed and a second guard added. The step measures disk use and does not fix the disk reds.
The founder ruled that the SVG fetch stands as measured, image and feImage href and not use. The founder also ruled that the upstream-sync dependency-set rule is amended in its standing form, not as a one-time exception.
@task_file n=1 path=Cargo.lock mark=merge
what: the locked dependency graph, the lockfile the audit leg and locked builds read
@task_file n=2 path=Cargo.toml
what: the root workspace manifest, holding the workspace-inherited dependency versions and the engine dependency pins (the AnyRender set)
@task_file n=3 path=deny.toml
what: the cargo-deny configuration, holding the advisory ignores by ID and the licence gate
@task_file n=4 path=.github/workflows/ci.yml
what: the CI workflow, holding the job matrix, the SHA-pinned actions, the free-disk step and the dependency audit gate
@task_file n=5 path=.github/scripts/test_ci_workflows.py
what: the CI-script tests guarding the workflow, including the free-disk guards
@task_file n=6 path=.github/scripts/ci-leg.sh
what: the CI leg runner, holding the fast, audit, wasm and other legs
@task_file n=7 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document, now holding the image fetch for sub-resources, built through the stamping request constructor
@task_file n=8 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator, holding the image load that now also reads an SVG image or feImage href, and the removal of a subtree
@task_file n=9 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's sub-resource request side, holding the stamping request constructor
@task_file n=10 path=packages/blitz-dom/src/node/element.rs mark=merge
what: the element data, holding the markup attribute readers, now including an SVG element's href or xlink:href reader
@task_file n=11 path=packages/blitz-dom/src/util.rs mark=merge
what: the engine helpers, holding the SVG parse for an SVG image and for inline or top-level svg, now given the image cache
@task_file n=12 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the scroll state, holding the programmatic scroll clamp and now the scroll-range clamp and reset per node
@task_file n=13 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the layout pass, whose final layout now clamps each node's scroll offset
@task_file n=14 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the layout tree construction
@task_file n=15 path=packages/blitz-dom/src/resolve.rs mark=merge
what: the style and layout resolve step, now clamping the viewport scroll and scroll animation
@task_file n=16 path=packages/blitz-dom/Cargo.toml mark=merge
what: the engine crate manifest, its svg feature now taking png
@task_file n=17 path=packages/blitz-paint/src/text.rs mark=merge
what: the text painter, including the password input painted as bullets
@task_file n=18 path=packages/blitz-paint/src/render.rs
what: the paint pass over the document, ported to AnyRender 0.15
@task_file n=19 path=packages/blitz-paint/src/render/form_controls.rs
what: the form-control painter, ported to AnyRender 0.15
@task_file n=20 path=packages/blitz-paint/src/render/select_list.rs
what: the painter for the open select list drawn over the page, ported to AnyRender 0.15
@task_file n=21 path=packages/blitz-paint/src/filters.rs
what: the CSS filter conversion for paint
@task_file n=22 path=packages/blitz-paint/src/debug_overlay.rs
what: the paint debug overlay
@task_file n=23 path=packages/dioxus-native/Cargo.toml
what: the Dioxus native manifest, its public renderer feature renamed from vello-hybrid to vello-gpu
@task_file n=24 path=packages/dioxus-native/src/dioxus_renderer.rs
what: the Dioxus native renderer selection, now aliasing the Vello GPU window renderer
@task_file n=25 path=packages/dioxus-native/src/lib.rs
what: the Dioxus native launch surface and its renderer options
@task_file n=26 path=apps/browser/Cargo.toml
what: the browser app manifest, its renderer feature renamed to vello-gpu
@task_file n=27 path=apps/browser/src/capture.rs
what: the browser app's screen capture
@task_file n=28 path=apps/readme/Cargo.toml
what: the rdme app manifest, its renderer feature renamed to vello-gpu
@task_file n=29 path=justfile
what: the developer recipes, the hybrid recipe renamed vello-gpu
@task_file n=30 path=examples/paint_bench.rs
what: the paint benchmark example and its backend list
@task_file n=31 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the check that file URLs load only for file documents and that every engine request carries its document as initiator, now covering the SVG image load
@task_file n=32 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of scrolling a list row into view, the off-screen refusal and the click
@task_file n=33 path=tests/blitz-tests/tests/fragment_navigation.rs mark=merge
what: the fragment navigation checks, now including a scroller clamped after a viewport resize
@task_file n=34 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT commit
@task_file n=35 path=wpt/runner/src/main.rs mark=merge
what: the WPT runner
@task_file n=36 path=wpt/runner/src/test_runners/ref_test.rs mark=merge
what: the WPT runner's reference-test runner
@task_file n=37 path=wpt/runner/src/test_runners/mod.rs mark=merge
what: the WPT runner's test-runner dispatch
@task_file n=38 path=wpt/runner/src/test_runners/crash_test.rs
what: the WPT runner's crash-test runner
@task_file n=39 path=docs/parley.md mark=merge
what: the parley text-layout notes
@task_file n=40 path=HOWTO_WASM.md
what: the wasm build guide, its renderer feature renamed to vello-gpu
@task_item n=1 part=touches
Upstream sync takes the dependency set whole: the founder amended it to a standing form. This sync's set added 7 packages to the lock and took 2 out (988 to 998), and skrifa, read-fonts and harfrust each now sit in two versions.
@task_item n=2 part=touches
Runner disk freed only for the repeated stand job is now stale. test-features-default frees disk too and reads df -h before and after its test leg. The guard test_only_stand_repeat_frees_disk was renamed test_free_disk_only_on_test_and_stand_repeat, and test_test_job_reads_its_disk was added.
@task_item n=3 part=touches
Boundary widening rule: the SVG image and feImage href load is a new sub-resource kind that arrived by upstream merge, ratified by the founder as measured. The use href is not fetched.
@task_item n=4 part=touches
One stamping constructor for sub-resource requests: the image fetch moved into a document-level function that builds every image request, now including the SVG image and feImage href, through that constructor.
@task_item n=5 part=touches
File URLs load only for file documents, and file URLs load only from file documents: the SVG image load is now among the refused file loads from a remote document. The guard local_files_for_local_documents now covers it, and the request carries its document as initiator.
@task_item n=6 part=touches
Sub-resource requests are GETs, and outbound HTTP goes through blitz-net: the new SVG image fetch goes the same way.
@task_item n=7 part=touches
Advisory ignores by ID only, and audit ignores per ID with a written reason: four zip ignores were dropped, so the ignores went from 9 to 5. zip 9.6.1 holds the fixes, and no ignore was added for 0545 or 0548.
@task_item n=8 part=touches
Dependency audit CI gate, and cargo-deny as the audit tool: the audit leg reads advisories ok and licenses ok on the new lock.
@task_item n=9 part=touches
No git dependencies in the root manifest, and no git dependency: Taffy's pinned rev is still the only git source after the AnyRender move.
@task_item n=10 part=touches
Engine dependency pins: the pinned renderer set moved to anyrender 0.15, anyrender_vello_gpu 0.12, vello 0.12, vello_cpu and vello_gpu 0.4, usvg 0.50 and wgpu_context 0.11. anyrender_vello_hybrid is gone.
@task_item n=11 part=touches
Workspace-inherited dependency versions: the inherited key anyrender_vello_hybrid was renamed anyrender_vello_gpu across the members.
@task_item n=12 part=touches
CI actions pinned to commit SHAs: free-disk-space was newly used on test-features-default, pinned at its v1.3.1 commit.
@task_item n=13 part=touches
Programmatic scroll clamped and missing node ignored: beside it, scroll offsets are now also clamped at final layout, on viewport changes and on scroll animation, and reset when a subtree is removed.
@task_item n=14 part=touches
The off-screen refusal cause: after a delete shrinks the CRUD list, the first row is no longer refused as off-screen. stand_act_scroll was re-pinned to the clamped reading, with the offset at 0, the row centre inside the list's box and the click accepted.
@task_item n=15 part=touches
png encodes a screenshot, and png as the shot encoder: png now also joins the engine's svg feature to encode raster image data handed to the SVG parse.
@task_item n=16 part=touches
Inline SVG serialization and the inline-SVG serialization output: the SVG parse for an inline or top-level svg now takes the document URL and the image cache. The serializer's rule is unchanged, but the parse beside it moved.
@task_item n=17 part=touches
Engine log targets keep only allowlisted fields, and the network tracing log and resource-load failure log contracts: two of the three image-fetch tracing info lines moved into the document-level fetch. No log site was added (3 moved).
@task_item n=18 part=touches
Password input painted as bullets or masked, and the select list painted over the page: these painters were ported to AnyRender 0.15 with anti-aliasing on every fill, stroke and glyph draw. The pixel checks are green.
@task_item n=19 part=touches
Locked builds: the lock was re-resolved by cargo update -w as one coupled set, and every leg still builds locked.
@task_item n=20 part=touches
Name freed and taken: the renderer feature names vello-hybrid and hybrid are freed, and vello-gpu is taken across dioxus-native, the apps, the examples and the just recipe.
@task_fact n=1 slug=input-validation
fact: An SVG image or feImage element's href (unprefixed href, else xlink:href) is fetched as an external image sub-resource through the stamping constructor, carrying its document as initiator and under the file-URL refusal. A use href is never fetched. This is a boundary widening that arrived by upstream merge and was ratified by the founder.
@task_fact n=2 slug=input-validation
fact: Each node's scroll offset is clamped to its scroll range at final layout, and the viewport scroll and any scroll animation are clamped on device change. A removed subtree's scroll state is reset.
@task_fact n=3 slug=dependency-security
fact: The default-features test job frees runner disk with the SHA-pinned free-disk-space action and reads df -h before and after its test leg, into the step log only, with no upload. The readings measure disk use and do not fix it.
@task_item n=1 part=untouched
Authentication and authorization, the CI token scopes and the publish guards: no auth path, token or publish job moved.
@task_item n=2 part=untouched
Secret management, the CI secrets and RUST_LOG as the only environment read: no environment variable or secret was added.
@task_item n=3 part=untouched
Data protection: the history store, cookies, HTTP cache, TLS and kept check files are unchanged.
@task_item n=4 part=untouched
The driver session socket, the MCP server and their contracts (request lines, replies, bounds, protocol errors, exit codes): no IPC method, endpoint, port or socket was added.
@task_item n=5 part=untouched
The stable element id, snapshot text, graph, actions and layout answers and their crossings: no carrier or answer field moved.
@task_item n=6 part=untouched
The cold-agent pipe, the recordings and the agent-run harness: their validation, privacy and logging rows are unchanged.
@task_item n=7 part=untouched
Script fetch and JavaScript rules (GET and HEAD only, file and data URLs only, JavaScript opt-in): no script path moved.
@task_item n=8 part=untouched
Escher's own telemetry sink and the escher_ target scrubbing: no escher_ target was added.
