@task_prose part=session
The session took five commits of upstream DioxusLabs/blitz main past ea07ee716 (pinned at e5305a4c1, 5 of 10 taken, 102 left out, 97 of them carried) ahead of Epoch 6. The take also brought in the AnyRender 0.15 port, the zip 9.6.1 bump and the rename of the hybrid renderer feature to vello-gpu.
Engine behaviour changes taken from upstream: the image fetch moved into the document and is built through the stamped request constructor. SVG image and feImage href are now fetched as sub-resources (use is not). The scroll offset is clamped at layout rounding and on subtree removal. Paint moved to AnyRender 0.15.
No log site, print, env read or engine request constructor was added. Three blitz-dom image-fetch info lines moved: two into the document's image fetch, while the cache arm's line stays in the mutator.
The fork CI default-features test job now frees disk and writes two df -h readings (before and after the test leg) to its step log, with no upload. One guard was renamed and one was added. The four zip advisory ignores were dropped (9 to 5).
Measured on the pushed commit: CI green on attempt 2 (17 of 17), linux 1547/0/15, windows 1468/0/10, macOS 1543/0/15. The test leg used 45G and the free-disk step freed 24G.
The founder ruled that the two dependency rules are amended in their standing form and that the SVG fetch stands as measured (image and feImage href, not use).
@task_file n=1 path=.github/workflows/ci.yml
what: edited; the fork CI workflow, its Test [default features] job gaining the free-disk step and two df -h disk readings in the step log, no artifact upload added
@task_file n=2 path=.github/scripts/test_ci_workflows.py
what: edited; the CI-script tests guarding the workflow, the free-disk guard renamed to free disk only on the test and stand-repeat jobs, a guard added that the test job reads its disk
@task_file n=3 path=.github/scripts/ci-leg.sh
what: edited; the CI leg script, its wasm leg now building the vello-gpu renderer feature
@task_file n=4 path=Cargo.toml
what: edited; the workspace manifest, the AnyRender 0.15 dependency set and the vello-gpu renderer key replacing vello-hybrid
@task_file n=5 path=Cargo.lock
what: edited; the workspace lockfile, the upstream-sync dependency set (988 to 998 packages, 7 entering, 2 leaving, skrifa 0.49 beside 0.44)
@task_file n=6 path=deny.toml
what: edited; the cargo-deny configuration, its advisory ignore list cut from 9 to 5
@task_file n=7 path=packages/blitz-dom/src/document.rs mark=merge
what: cherry-picked onto; the engine document, now holding the image fetch built through the stamped request constructor and two of the engine's image-fetch info log lines
@task_file n=8 path=packages/blitz-dom/src/mutator.rs mark=merge
what: cherry-picked onto; the engine DOM mutator, its image load now also reading SVG image and feImage href, its cache-arm image info line kept, the scroll state reset on subtree removal
@task_file n=9 path=packages/blitz-dom/src/node/element.rs mark=merge
what: cherry-picked onto; the engine element data, gaining the SVG href read (href, else xlink:href)
@task_file n=10 path=packages/blitz-dom/src/util.rs mark=merge
what: cherry-picked onto; the engine's SVG parsing, now given the document URL and image cache for inline and top-level svg
@task_file n=11 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: cherry-picked onto; the engine's scrolling, node scroll offset clamping and reset and viewport clamping
@task_file n=12 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: cherry-picked onto; the engine layout's final layout, calling the scroll offset clamp
@task_file n=13 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: cherry-picked onto; the engine's layout tree construction
@task_file n=14 path=packages/blitz-dom/src/resolve.rs mark=merge
what: cherry-picked onto; the engine's resolve phases, now clamping viewport scroll and scroll animation
@task_file n=15 path=packages/blitz-dom/src/net.rs mark=merge
what: cherry-picked onto; the engine's network glue for sub-resource loads
@task_file n=16 path=packages/blitz-dom/Cargo.toml mark=merge
what: cherry-picked onto; the engine crate's manifest, png joining its svg feature
@task_file n=17 path=packages/blitz-paint/src/text.rs mark=merge
what: cherry-picked onto and ported; the paint crate's text painting, on AnyRender 0.15
@task_file n=18 path=packages/blitz-paint/src/render.rs
what: edited; the paint crate's renderer, ported to AnyRender 0.15 with anti-aliasing passed on every fill, stroke and glyph draw
@task_file n=19 path=packages/blitz-paint/src/filters.rs
what: edited; the paint crate's CSS filter conversion, now taking the bounds
@task_file n=20 path=packages/blitz-paint/src/debug_overlay.rs
what: edited; the paint crate's debug overlay, ported to AnyRender 0.15
@task_file n=21 path=packages/blitz-paint/src/render/form_controls.rs
what: edited; the paint of form controls, ported to AnyRender 0.15
@task_file n=22 path=packages/blitz-paint/src/render/select_list.rs
what: edited; the paint of the open select list, ported to AnyRender 0.15
@task_file n=23 path=packages/blitz-paint/src/render/box_shadow.rs
what: edited; the paint of box shadows, in AnyRender 0.15's form
@task_file n=24 path=packages/dioxus-native/Cargo.toml
what: edited; dioxus-native's manifest, its public renderer feature vello-hybrid renamed vello-gpu
@task_file n=25 path=packages/dioxus-native/src/lib.rs
what: edited; dioxus-native's launch, its renderer selection by the vello-gpu feature
@task_file n=26 path=packages/dioxus-native/src/dioxus_renderer.rs
what: edited; dioxus-native's window renderer aliases, now the vello GPU window renderer
@task_file n=27 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: edited; the stand's scroll act check, re-pinned so that after a delete the CRUD list's offset reads 0 and its first row is clicked, the off-screen refusal on that row retired
@task_file n=28 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: edited; the check that engine requests carry their document as initiator and that file: loads from non-file documents are refused, now naming the SVG image load
@task_file n=29 path=tests/blitz-tests/tests/fragment_navigation.rs mark=merge
what: cherry-picked onto; the fragment navigation checks, gaining upstream's percentage-height scroller clamp after viewport resize
@task_file n=30 path=wpt/WPT_COMMIT mark=merge
what: cherry-picked onto; the WPT suite's pinned commit
@task_file n=31 path=wpt/runner/src/main.rs mark=merge
what: cherry-picked onto; the WPT runner, its report and figures
@task_file n=32 path=wpt/runner/src/test_runners/ref_test.rs mark=merge
what: cherry-picked onto and ported; the WPT runner's reftest runner
@task_file n=33 path=wpt/runner/src/test_runners/crash_test.rs
what: edited; the WPT runner's crash test runner, ported to AnyRender 0.15
@task_file n=34 path=examples/paint_bench.rs
what: replaced by upstream's file whole; the paint bench example and its printed figures, its backend list now vello, cpu and gpu
@task_file n=35 path=examples/screenshot.rs
what: edited; the screenshot example and its printed timings, ported to AnyRender 0.15
@task_file n=36 path=examples/seven_guis/Cargo.toml
what: edited; the seven_guis example's manifest, the windowed native app's renderer feature renamed vello-gpu
@task_file n=37 path=examples/wasm_hello/src/lib.rs
what: edited; the wasm example's entry, its renderer named vello GPU
@task_file n=38 path=justfile
what: edited; the task recipes, the hybrid recipe renamed vello-gpu
@task_file n=39 path=apps/browser/src/capture.rs
what: edited; the browser app's capture, ported to AnyRender 0.15
@task_file n=40 path=.claude/docs/commands.md
what: edited through the harness land; the harness command reference, renderer feature and paint bench backend names renamed
@task_item n=1 part=touches
The engine's image-fetch info log lines in the blitz-dom target moved: two went into the document's image fetch, and the cache arm's line stayed with its variable renamed. Any row that counts or places engine log sites, or says engine sites write fixed messages and print safe fields only, reads over them. The census of log, print and env sites over the packages is unchanged (3 added, 3 removed).
@task_item n=2 part=touches
The engine tracing rows (gated behind a cargo feature, set by the build's invocation) rule over the moved image-fetch lines. The moved lines are still upstream's tracing::info in the blitz-dom target, with no escher target.
@task_item n=3 part=touches
The rule on scrubbing by field name and escher-telemetry's scrub set url and error entries are reached indirectly. The image-fetch lines that moved carry the fetched url in the blitz-dom target, which the sink target allowlist decides on.
@task_item n=4 part=touches
The rule that no file: loads come from non-file documents gains a new load kind. SVG image and feImage href loads are now engine sub-resource requests, refused for file: from a remote document, and the check now names the SVG image load. use href is not fetched.
@task_item n=5 part=touches
The refusal cause off-screen: the stand's scroll act no longer pins the CRUD list's first row as refused off-screen after a delete. The layout-time clamp puts the offset at 0 and the row inside the list, so the row is clicked.
@task_item n=6 part=touches
The CI rows that place free-disk on the stand-repeat job alone are now stale. The default-features test job frees disk too, and the guard was renamed to free disk only on the test and stand-repeat jobs, with a new guard that the test job reads its disk. Any row whose guard cell names the old only-stand-repeat guard must take the new name.
@task_item n=7 part=touches
Fork CI uploads two artifacts, the per-leg CI log is uploaded on failure, and local state stays local. The workflow gained two df -h steps whose output stays in the step log with no upload, so the artifact count of two holds and should be confirmed.
@task_item n=8 part=touches
The CI platform on GitHub workflows: the workflow changed, gaining a pinned third-party free-disk action (jlumbroso/free-disk-space v1.3.1).
@task_item n=9 part=touches
The package-install diagnostic line contract sits beside a new CI diagnostic. The disk readings before and after the test leg are written to the step log as named steps.
@task_item n=10 part=touches
The renderer feature name moved: dioxus-native's vello-hybrid and the apps' and examples' hybrid feature are now vello-gpu, and the hybrid just recipe is gone. Any row naming the hybrid renderer or its feature, such as the build the windowed seven_guis_native smoke runs on, is stale.
@task_item n=11 part=touches
The escher sink log line service.name is reached by the windowed seven_guis_native smoke, which now runs on vello-gpu and still logs service.name=seven_guis. The telemetry sink installed first in each native main is reached through that app's manifest.
@task_item n=12 part=touches
The metric-coverage overview names blitz-paint's layer counters, the bench, screenshot and the WPT runner's figures. The paint bench was replaced whole by upstream's with the backend list vello, cpu and gpu, the paint crate and screenshot example were ported to AnyRender 0.15, and the WPT runner and its pinned commit moved.
@task_item n=13 part=touches
The rows on WPT scores and WPT results published from upstream only rule over the WPT runner and suite pin that this take moved. Publishing is unchanged.
@task_item n=14 part=touches
The rule on WASM builds' console_error_panic_hook rules over the wasm example and the wasm leg, which now build with the vello-gpu feature.
@task_item n=15 part=touches
The rows on no OTel export and the absences of a telemetry backend, metrics exporter and error-reporting service rule over the dependency set. The lockfile gained 7 packages (anyrender_vello_gpu, const-oid, constant_time_eq, hybrid-array, image_filters_types, typed-path, zeroize_derive) and the AnyRender 0.15 set, with no telemetry crate among them.
@task_item n=16 part=touches
The section 6 log coverage silence line for this take is owed: the upstream take added no log site, print, env read or engine request constructor, and the three image-fetch lines only moved.
@task_item n=17 part=touches
The select list row label never logged and the open list shot carries labels as pixels rule over the open select list's paint, which was ported to AnyRender 0.15. No log site was added there.
@task_fact n=1 slug=6-log-coverage
fact: The upstream sync to e5305a4c1 adds no log site, print, env read or engine request constructor. Of upstream's three blitz-dom image-fetch tracing::info lines, two move into the document's image fetch and the cache arm's stays in the mutator.
@task_fact n=2 slug=7-error-capture-reporting
fact: SVG image and feImage href are fetched as engine sub-resource requests through the stamped constructor, carrying their document as initiator, and a file: href from a non-file document is refused. use href is not fetched.
@task_fact n=3 slug=ci-integration
fact: The fork CI default-features test job frees disk and writes two df -h readings, before and after the test leg (the after one always), to its step log only, with no artifact upload. Free-disk stands on that job and on stand-repeat alone.
@task_item n=1 part=untouched
The driver's command and tool_call spans, their fields, endings and refusal cause names other than off-screen: no driver or MCP code moved.
@task_item n=2 part=untouched
The cold-agent pipe's events, verdict fields, counted verbs, transcript and kept checks: no cold-agent harness changed.
@task_item n=3 part=untouched
The agent-run harness events and raw-log rules: run stand and run all passed with their events unchanged in shape.
@task_item n=4 part=untouched
The escher-session commands, exit codes, host wire, coverage role, error object and recording file and directory rows: escher-session's code did not move.
@task_item n=5 part=untouched
escher's sink line format, stream, span line layout, field order, quoting and resource keys service.name and service.version: escher-telemetry did not change.
@task_item n=6 part=untouched
The script error surface, window error event, window.onerror and script diagnostics drain contracts: no script runtime code moved.
@task_item n=7 part=untouched
The snapshot, graph, actions and layout answer contracts and their PII rules: the driver's answers are unchanged.
@task_item n=8 part=untouched
Sections 10 to 12 (SLO invariants, anti-patterns, the decisions log) hold no recorded intent and gain none.
