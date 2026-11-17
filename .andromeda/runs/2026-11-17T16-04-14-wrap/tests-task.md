@task_prose part=session
The session took five commits from upstream blitz main (pinned at e5305a4c1) by cherry-pick, ahead of Epoch 6, and left 5 of upstream's 10 out; the left-out delta now reads 102, 97 of them carried.
It ported the paint and the examples to AnyRender 0.15 (anyrender_vello_cpu 0.19, vello 0.12, usvg 0.50), replaced anyrender_vello_hybrid with anyrender_vello_gpu, renamed the renderer feature vello-hybrid and hybrid to vello-gpu across dioxus-native, the apps, the examples, the wasm leg and the justfile recipe, and bumped zip 2.4.2 to 9.6.1, dropping the four zip advisory ignores (9 to 5).
Upstream's take brings a scroll offset clamped at layout time (a box whose content shrinks reads its offset back in range) and a fetch of SVG image and feImage href as a stamped engine sub-resource request; a use href is not fetched.
The stand scroll check was re-pinned: after a delete the CRUD list's offset reads 0 and its first row is no longer refused off-screen but clicked, in both layout modes; the local-files check now holds the SVG image load among the refused file loads; upstream's percentage-height scroller clamp test joined.
CI's default-features test job gained the SHA-pinned free-disk step and two df -h readings before and after the test leg, in the step log only; the guard test_only_stand_repeat_frees_disk was renamed test_free_disk_only_on_test_and_stand_repeat and test_test_job_reads_its_disk added (CI-script test methods 37 to 38).
Tallies: fast 1547 passed 0 failed 15 ignored; run all 873 to 874; run stand 327 0 10 unchanged; per-OS linux 1547 0 15, windows 1468 0 10, macOS 1543 0 15; stand repeat three rounds 1449 0 30; a first CI red was two jobs no runner acquired, re-run and green.
@task_file n=1 path=.github/workflows/ci.yml
what: the CI workflow, its jobs: the default-features test job (now with the free-disk step and two disk readings), the stand-repeat job and the rest
@task_file n=2 path=.github/scripts/test_ci_workflows.py
what: the CI Python script unit tests under unittest, the guards over the workflow, among them the free-disk guard renamed and the new disk-reading guard
@task_file n=3 path=.github/scripts/ci-leg.sh
what: the CI leg script every leg runs through; its wasm leg now builds the vello-gpu feature
@task_file n=4 path=deny.toml
what: the dependency audit's advisory and license configuration; four zip advisory ignores dropped
@task_file n=5 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of scroll into view and acting on a row past the list, re-pinned to the clamped offset in both layout modes
@task_file n=6 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the integration check that file: URLs load only from a file: document and every engine request carries its document as initiator; now holds the SVG image load
@task_file n=7 path=tests/blitz-tests/tests/fragment_navigation.rs mark=merge
what: the integration checks of fragment navigation and scrolling; upstream's percentage-height scroller clamp test added
@task_file n=8 path=Cargo.toml
what: the workspace manifest, its dependency pins: the AnyRender set, vello, usvg, zip
@task_file n=9 path=Cargo.lock mark=merge
what: the workspace lockfile the locked cargo legs read; 988 to 998 packages, the one git source Taffy's
@task_file n=10 path=justfile
what: the developer recipes; the hybrid recipe renamed vello-gpu
@task_file n=11 path=wpt/WPT_COMMIT mark=merge
what: the recorded WPT commit the conformance runner is pinned to
@task_file n=12 path=wpt/runner/src/main.rs mark=merge
what: the WPT conformance runner's command line
@task_file n=13 path=wpt/runner/src/test_runners/mod.rs mark=merge
what: the WPT runner's test runners
@task_file n=14 path=wpt/runner/src/test_runners/ref_test.rs mark=merge
what: the WPT reftest runner, painting on the CPU renderer
@task_file n=15 path=wpt/runner/src/test_runners/crash_test.rs
what: the WPT crash test runner
@task_file n=16 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document; the image fetch moved here, built through the one stamping request constructor
@task_file n=17 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator, its image load arm, now reading an SVG image and feImage href; removed subtrees reset scroll state
@task_file n=18 path=packages/blitz-dom/src/net.rs mark=merge
what: blitz-dom's network side, the sub-resource load kinds
@task_file n=19 path=packages/blitz-dom/src/node/element.rs mark=merge
what: the element node data; reads an SVG element's href or xlink:href
@task_file n=20 path=packages/blitz-dom/src/util.rs mark=merge
what: blitz-dom utilities, the SVG parse now given the document's image cache
@task_file n=21 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the engine's scroll state: scroll range, offset clamp, viewport and scroll-animation clamp
@task_file n=22 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine layout pass, where the final layout now clamps each node's scroll offset
@task_file n=23 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the layout tree construction
@task_file n=24 path=packages/blitz-dom/src/resolve.rs mark=merge
what: the style and layout resolve pass, clamping viewport scroll and scroll animation
@task_file n=25 path=packages/blitz-dom/Cargo.toml mark=merge
what: blitz-dom's manifest, its svg feature now with png
@task_file n=26 path=packages/blitz-paint/src/text.rs mark=merge
what: the text painter, glyph drawing and the font embolden paint
@task_file n=27 path=packages/blitz-paint/src/render.rs
what: the scene painter on AnyRender 0.15, every fill and stroke now with anti-aliasing stated
@task_file n=28 path=packages/blitz-paint/src/render/box_shadow.rs
what: the box-shadow painter, drawn in AnyRender 0.15's form
@task_file n=29 path=packages/blitz-paint/src/render/select_list.rs
what: the select list painted by the scene painter
@task_file n=30 path=packages/blitz-paint/src/render/form_controls.rs
what: the form control painter
@task_file n=31 path=packages/blitz-paint/src/render/background.rs
what: the background painter
@task_file n=32 path=packages/blitz-paint/src/render/border.rs
what: the border painter
@task_file n=33 path=packages/blitz-paint/src/filters.rs
what: the CSS filter conversion for paint
@task_file n=34 path=packages/blitz-paint/src/debug_overlay.rs
what: the paint debug overlay
@task_file n=35 path=packages/dioxus-native/Cargo.toml
what: dioxus-native's manifest, its public renderer feature renamed from vello-hybrid to vello-gpu
@task_file n=36 path=packages/dioxus-native/src/dioxus_renderer.rs
what: dioxus-native's renderer choice, the GPU renderer now VelloGpuWindowRenderer
@task_file n=37 path=packages/dioxus-native/src/lib.rs
what: dioxus-native's launch surface and its renderer feature gates
@task_file n=38 path=examples/seven_guis/Cargo.toml
what: the seven_guis example app's manifest, the stand app, its feature hybrid renamed vello-gpu
@task_file n=39 path=examples/screenshot.rs
what: the screenshot example on the CPU renderer
@task_file n=40 path=apps/browser/src/capture.rs
what: the browser app's capture to an image
@task_item n=1 part=touches
decision/only-stand-repeat-frees-disk is stale: the free-disk step now stands on the default-features test job as well as stand-repeat, and its guard was renamed from test_only_stand_repeat_frees_disk to test_free_disk_only_on_test_and_stand_repeat, with test_test_job_reads_its_disk added beside it.
@task_item n=2 part=touches
decision/dependency-audit-advisories-and-licenses: the four zip advisory ignores are gone with zip 9.6.1, the ignores 9 to 5, audit reading advisories ok, licenses ok; any row holding the ignore count or list moves.
@task_item n=3 part=touches
decision/ci-actions-pinned-to-sha and decision/actions-pinned-to-commit-sha: a further action, free-disk-space v1.3.1, joins the test job pinned to its commit SHA; the convention holds, its instance list grows.
@task_item n=4 part=touches
decision/wpt-pinned-commit and the WPT runner rows (decision/wpt-conformance-runner, contract/wpt-runner-cli items): the recorded WPT commit moved and the runner's files took upstream's picks and the paint port.
@task_item n=5 part=touches
decision/scrolled-box-reads-where-it-stands and contract/harness-input/scroll-into-view: a box's scroll offset is now clamped at layout time, so after content shrinks the box reads its offset back in range (the CRUD list reads 0 after a delete).
@task_item n=6 part=touches
contract/refusal-cause/off-screen and contract/lint-defects/off-screen, indirectly through the scroll clamp: the stand check's first row after a delete is no longer refused off-screen but clicked; a row pinning the unclamped reading is stale.
@task_item n=7 part=touches
decision/file-urls-only-for-file-documents: SVG image and feImage href loads are now among the engine's loads, and a file: href from a remote document is refused like the rest; the local-files check names the SVG image.
@task_item n=8 part=touches
decision/blitz-dom-requests-through-stamping-constructor: the image fetch moved into the document and builds its request through the stamping constructor, now for a new kind, the SVG image and feImage href; controlled red-before-green.
@task_item n=9 part=touches
decision/harness-counts-fetching-providers-only, decision/network-stand-ins-counted and contract/harness-settle/busy-loads, indirectly: a document with an SVG image or feImage href now issues loads the settle waits on.
@task_item n=10 part=touches
decision/pixel-tests-on-cpu-buffer, decision/pixel-tests-cpu-buffer and decision/headless-screenshot with the harness-screenshot contracts, indirectly: the CPU renderer moved to anyrender_vello_cpu 0.19 and every fill, stroke and glyph draw now states anti-aliasing; the six tasks' shots differ by a few hundred to 2618 pixels, max delta 30.
@task_item n=11 part=touches
decision/box-shadow-chromium-expectations, indirectly: the box-shadow painter now draws in AnyRender 0.15's form (outset unclipped, zero offset and spread); the expectations stood green.
@task_item n=12 part=touches
decision/select-list-painted-by-paint-scene and decision/embolden-local-witness, indirectly: the select list painter and the text painter were ported to AnyRender 0.15.
@task_item n=13 part=touches
decision/buffered-render-on-dioxus-07-bridge and decision/buffered-render-unborrowed-document, indirectly: dioxus-native's GPU renderer is now VelloGpuWindowRenderer under the feature vello-gpu; the name vello-hybrid is freed.
@task_item n=14 part=touches
decision/tally-move-attributed-by-test-name and decision/workspace-tests-in-fast-ci-leg: fast reads 1547 passed 0 failed 15 ignored, run all 873 to 874, the moves named by test; a row holding a tally moves.
@task_item n=15 part=touches
decision/native-platform-ci-matrix: the per-OS tallies on the pushed commit read linux 1547 0 15, windows 1468 0 10, macOS 1543 0 15, android and iOS build only; any row holding them moves.
@task_item n=16 part=touches
decision/stand-repeat-three-rounds, decision/zero-failure-stand-repeat and decision/stand-checks-zero-failure-budget: stand repeated three rounds read 1449 0 30, no non-ok line; a row holding that count moves.
@task_item n=17 part=touches
decision/ci-scripts-unit-tested and decision/python-script-tests-under-unittest: the CI-script test methods 37 to 38, Ran 120.
@task_item n=18 part=touches
decision/failure-logs-uploaded and contract/ci-artifacts/leg-log: the test job's two df -h readings stand in the step log only, no artifact uploaded.
@task_item n=19 part=touches
decision/stand-checks-in-both-layout-modes, decision/expected-value-pinned-as-literal and decision/mutation-controlled-tests: the stand scroll re-pin holds three measured literals reading the same in both layout modes, and both controls (scroll-clamp, svg-image-fetch) held red then green.
@task_item n=20 part=touches
decision/upstream-buffered-render-test-kept-as-written and decision/upstream-mounted-flush-test-left-out: the upstream sync moved its merge base to e5305a4c1, 5 taken, 5 left out; these rows stand only if they name no count of the left-out delta.
@task_item n=21 part=touches
decision/msrv-1-91 and decision/cargo-legs-locked: the msrv and wasm legs exit 0 on the new dependency set, the lock read locked; the wasm leg builds vello-gpu.
@task_item n=22 part=touches
decision/regression-tests-named-for-bug and decision/one-test-target-per-file: upstream's percentage_height_scroller_clamps_after_viewport_resize joined an existing target, no new target.
@task_fact n=1 slug=ci-integration
fact: The default-features test job reads its disk with df -h before the test leg and after it (if always), the readings in the step log only, guarded by test_test_job_reads_its_disk; the first reading showed the leg using 45G and the free-disk step freeing 24G.
@task_fact n=2 slug=ci-integration
fact: A CI job no runner acquired (cancelled, no step run) is re-run, not fixed; a red on anything but a missing runner stops the pass.
@task_fact n=3 slug=integration-test-strategy
fact: An SVG image or feImage href is fetched as an engine sub-resource request carrying its document as initiator and refused as file: from a remote document; a use href is not fetched; the local-files check holds it, controlled.
@task_fact n=4 slug=3-test-harness-contract
fact: A box's scroll offset is clamped into its scroll range at layout time, so a box whose content shrinks reads its offset back in range; the stand scroll check reads the CRUD list at 0 after a delete and clicks its first row, controlled.
@task_item n=1 part=untouched
The session wire, session host, session errors and session state directory contracts: no protocol, bound, socket or error moved.
@task_item n=2 part=untouched
The MCP server and MCP role contracts, tools list, protocol revision and tool results: no MCP surface changed.
@task_item n=3 part=untouched
The cold-agent pipe rows, its events, verdict, exit codes, isolation and wrong-call bar: untouched.
@task_item n=4 part=untouched
The agent-run verbs, selections, events and exit codes: run stand and run all passed as before, no contract moved.
@task_item n=5 part=untouched
The virtual clock, advance, timer ticks and settle-never-advances-time rows: no time behaviour moved.
@task_item n=6 part=untouched
The typed password, masking and snapshot text budget rows: no driver verb or answer changed.
@task_item n=7 part=untouched
Handler coverage, recorded checks, replay and the committed checks' recording script: no recording or coverage line moved.
@task_item n=8 part=untouched
The mutation testing, complexity ceiling and keyboard_compat coverage exclusion rows: no floor, ceiling or exclusion moved.
@task_item n=9 part=untouched
The driver refusal cause set and order, the escher-session command line and exit codes: no cause added, removed or reordered.
