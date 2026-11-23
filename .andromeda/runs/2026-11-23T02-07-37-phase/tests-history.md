@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (CI on the build branch) made the workspace tests the fast `test` leg (`cargo test --workspace --locked`), with every cargo leg run through `ci-leg.sh` and `--locked`. That is the one CI leg whose workspace build unifies `blitz-paint/custom-widget` on. A canvas check that passes only there is not proof in the per-package builds this chunk must cover (rows decision/workspace-tests-in-fast-ci-leg, decision/cargo-legs-locked).

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — set up the stand checks under the bundled DejaVu Sans with system fonts off, with no skip path, `stand::options(incremental)` and `HarnessOptions` gaining `font_ctx` and `incremental`. A canvas stand check uses the same boot options and the same unconditional font (rows decision/stand-checks-bundled-font*, decision/stand-pinned-real-font).

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — was the first time one check passed per package and failed in the workspace leg because engine features differ by runner. It set the rule that a green per-package run is not the workspace leg's green, and that sink captures are counted by target. This is the same build-selection split the scope measures for `custom-widget` (rows decision/per-package-green-not-workspace-green, decision/sink-capture-counted-by-target).

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (two entries) The first pinned one test target per file through `BlitzTestsTargetsTest`, with upstream's `all.rs` left unbuilt, so a new canvas check is its own auto-discovered target. The second recorded a second engine feature (`writing-mode`) that is on in the workspace build only and changes which code a stand check runs. It is a direct precedent for the per-selection reading of `custom-widget` (rows decision/one-test-target-per-behaviour, decision/blitz-tests-one-target-per-file, decision/blitz-tests-manifest-difference).

@history_item n=5 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — required that each of the two feature-chosen bodies of the bounds reader hold a check of its own, with each mutation control red under its own build only. A canvas fix that depends on a feature cfg needs a witness under each build selection (row decision/bounds-reader-bodies-each-checked).

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (two entries) Built the shot that this chunk extends.
- The harness takes blitz-paint without default features, which is why `custom-widget` is off per package.
- `screenshot` / `screenshot_rect` go through `VelloCpuImageRenderer` over opaque white at the harness's own time. A font-less document is `FontsUnavailable` with nothing painted.
- Frames are compared whole, and shots are equal under both builds.
- Its checks are `stand_act_screenshot`, `stand_screenshot_repeat`, `password_painted_masked` and `stand_mcp_tools`.
- The PNG crosses the socket in band, on the founder's fork 1.

This is the screenshot contract the scope says must stand (rows contract/harness-screenshot/*, decision/headless-screenshot, decision/frames-compared-whole, decision/screenshot-in-band, contract/command-span/screenshot).

@history_item n=7 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set a zero-failure flakiness budget on the stand checks (three rounds in stand-repeat, no retry) and coverage floors per crate. Any new canvas stand check or named target runs under that budget (rows decision/stand-checks-zero-failure-budget, decision/zero-failure-stand-repeat).

@history_item n=8 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (two entries, ratified at 2026-10-18-engine-debts)
- Made the harness time a `VirtualClock` that `screenshot` and `screenshot_rect` never move.
- Fixed eight refusal causes in order, with `fonts-unavailable` the eighth.
- Moved the overlay scrollbar fade onto the document clock, so a shot after a scroll repeats until an `advance`.

A canvas shot must paint at the harness's own time and add no cause (rows decision/harness-virtual-clock, decision/eight-refusal-causes-in-order, decision/scrollbar-fade-on-document-clock).

@history_item n=9 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (two entries)
- Kept answers record a shot without `png`. The shot path's kept form is covered by `stand_session_recording` on linux and macOS.
- Measured that a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577). Since then a cross-OS byte-compared check, including a committed check's digest, holds no `screenshot` step, and `stand_screenshot_repeat` is byte-identity on one host.

This is why canvas byte equality across hosts is out of scope here (rows decision/no-screenshot-in-cross-os-checks, decision/shot-png-deterministic-per-host, decision/checks-hold-no-screenshot-step).

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (relevant entries) `select_popup_painted` reads engine-painted pixels on a frame's `Raster` against constants, in both layout modes, as its own non-`stand_` target named in stand-repeat. It is the precedent for a painted-content check. The same chunk had a macOS-only red through `apple-font-embolden` that no local gate built. That red is the trap of a paint feature unified on one build selection only (the font-embolden local witness).

@history_item n=11 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (relevant entries)
- Made the font-embolden witness the `embolden` step of `fast`, so a feature-gated paint path got a local gate. This is the model for gating a feature cfg that per-package builds leave off.
- Brought `mutator.rs`'s `custom_widget_replays_initial_attributes`, the existing `CustomWidget` unit test.
- Recorded `mix-blend-mode` and the inline-box paint skip as painted on no frame. That is the gap pattern a canvas shot check closes.

@history_item n=12 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — ported upstream's buffered render onto the Dioxus 0.7 bridge, so a task polled during render reaches the document unborrowed, pinned by `dioxus_runtime_probe`. Its limit is that a node created in the same render is not yet in the document. This matters if a canvas widget is handed in through the Dioxus mutation writer (row decision/buffered-render-on-dioxus-07-bridge).

@history_item n=13 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — `actions` names `fonts-unavailable` for `screenshot` with nothing painted, and `check_target` reads `can_paint_text` after the crop and before painting. A canvas-only document with no text still has to keep this refusal order (rows contract/driver-refusals/fonts-unavailable, decision/shot-refused-without-fonts).

@history_item n=14 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — stated as a handler-coverage limit that a handler inside a canvas is not counted, deferred to 0.4.0 (the canvas plans), and mapped that limit in the capability statement. The canvas work inherits that deferral. This chunk must not claim it discharged.

@history_item n=15 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — widened png-decoding-in-checks: a blitz-tests check decodes or encodes a PNG through the png dev-dependency, and seven_guis reads PNGs with a hand base64 reader. A canvas shot check reads pixels by these means (row decision/png-decoding-in-checks).

@history_item n=16 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — added cross-OS readings of the served tasks by answer digest (snapshot, lint, actions) with no shot, and gave the bundled-font stand decision its guard `stand_bundled_font_families`. A canvas on the stand must leave those digests unmoved, or re-record them under a ruling (rows decision/cross-os-readings-by-answer-digest, decision/stand-checks-bundled-font).

@history_item n=17 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added the rule that a control of a read that must change nothing needs pending work. A shot "changes nothing" read on a settled screen is otherwise no control. This bears on keeping the shot's no-change contract under the operator's red-first directive (row decision/no-change-read-control-needs-pending-work).

@history_item n=18 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — added expected-value-pinned-as-literal: a check never derives its expectation from the constant its control mutates. 2026-11-20-select-list-and-selection-debts later added the embolden pixel-band exception. Canvas pixel expectations are pinned as measured literals (row decision/expected-value-pinned-as-literal).

@history_item n=19 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — added the doc-leg gate: a chunk that edits a public doc comment lists `ci-leg.sh doc` in its local gate block, because the fast leg runs no rustdoc. Editing the `Widget`, harness or `screenshot` doc comments triggers it. The same chunk also widened stand-boots-offline to a second networked stand boot (rows decision/doc-leg-gates-public-doc-edits, decision/stand-boots-offline).

@history_item n=20 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — (relevant entries)
- Added `stand::boot_alone` and `Harness::from_component_alone`, with the root context provided before the build.
- The pinned viewport is read from the document, never from the root element's bounds.
- harness-graph-tests-enable-accessibility turns a gated feature on through a dev-dependency for the test build only.

That last item is the precedent for how a test build can enable a feature, such as `custom-widget`, that per-package builds leave off (rows contract/stand-boot/boot-alone, decision/pinned-viewport-read-from-document, decision/harness-graph-tests-enable-accessibility).

@history_item n=21 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — (two entries)
- Every release requirement names its witness inside stand-repeat, in a table pinned by function name. v040-09's witness must enter it.
- The typed-password and file-path masks were widened across every answer and pixels. A canvas must not become a path that leaks typed text as pixels.

(rows decision/requirement-witness-inside-stand-repeat, decision/typed-password-never-returned)

@history_item n=22 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — made stand-repeat run named non-`stand_` targets as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds` and counted by the capability statement. A canvas target without the `stand_` prefix only enters the leg by being named there (rows decision/stand-repeat-three-rounds, decision/joining-file-keeps-its-legs).

@history_item n=23 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added count-and-rename-sweeps after a numeric pin and a renamed witness were missed. 2026-11-18-served-error-page and 2026-11-21-engine-debts extended the rule to keep count-worded witness names and to cover CI-leg file counts. Any moved served-task, leg or target count from a canvas page or task must be swept (row decision/count-and-rename-sweeps).

@history_item n=24 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — is the last precedent for the stand gaining a served page: `stand::boot_order`, six served tasks became seven across the cross-OS digest table, the cold-agent pipe and `start`, plus the sweep-convention exception. If the canvas fork adds a stand page or task, the same count sites move (rows contract/stand-boot/boot-order, decision/cross-os-readings-by-answer-digest).

@history_item n=25 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — pinned the capability statement to the schema and to the CI leg script's a11y and stand-repeat counts (`CapabilityStatementTest`). This followed 2026-10-25-capability-statement's claims-to-checks map. Claiming v040-09 or adding a named stand-repeat target moves that pinned page (rows decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-check-names-held-to-schema, decision/capability-claims-name-their-checks).

@history_item n=26 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — keeps the workspace, blitz-tests, embolden and seven_guis feature resolutions each a build of its own, and never merges them. A fix that switches `custom-widget` on across build selections must respect the stated resolutions and the dev-host target sweep (rows decision/build-directory-held-by-sweep, decision/local-baseline).

@history_item n=27 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** — kept upstream-authored files over 800 code lines, and functions over the complexity ceiling, as upstream wrote them. Escher's additions to blitz-paint's `lib.rs` / `render.rs` should stay out of those upstream bodies (row decision/upstream-oversize-files-kept).

@history_item n=28 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — sharpened mutation-controlled-tests: a case left green is widened until the control turns it red, and a removed bound is loosened to twice the budget. 2026-11-14-auto-exploration had added the bound clause. This is the rule behind the operator's "each new control shown with its red reading" (row decision/mutation-controlled-tests).
