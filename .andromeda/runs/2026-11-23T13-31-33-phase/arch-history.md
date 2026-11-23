@history_item n=1 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the replay API, its targets, and the shot's same-host limit) — restated `stand_screenshot_repeat`'s byte-identity as holding on one host only: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no `screenshot` step. This is the origin and the stated basis of `decision/no-screenshot-in-cross-os-byte-check`, the rule this chunk measures and may amend; that basis was a whole-stand shot read before the bundled fonts.

@history_item n=2 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest, and the committed checks) — a kept check keeps each answer whole but a shot's without its pixels (`{"width":…,"height":…}`). The two committed Counter checks (`tests/blitz-tests/checks/counter-cli.json`, `counter-mcp.json`) hold no `screenshot` step and are kept fresh byte for byte by seven_guis `recording`. These are `decision/committed-stand-checks`, `contract/kept-check/answer` and `decision/kept-check-masking`. The scope keeps them free of a shot unless the founder's word also amends that rule.

@history_item n=3 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the harness's screenshot, the password mask, the image in band, `png`) — built `Harness::screenshot` / `screenshot_rect`: resolve at the harness's time, paint through blitz-paint into `VelloCpuImageRenderer` (no `multithreading`), composite over opaque white after paint (no `kurbo` / `peniko`), `FontsUnavailable` / `EmptyRect`. It made `png` the driver's encoder, with no package entering the lock, and a shot's pixels cross in band within the unchanged bounds. This is the raster path whose bytes and RGBA this chunk measures per host: `decision/headless-screenshot-cpu-renderer`, `decision/harness-composites-over-white`, `decision/harness-screenshot-white-after-paint`, `decision/png-encodes-shots`, `decision/screenshot-in-band`.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the `screenshot` verb, `fonts-unavailable`, `Outcome::Shot` and the MCP image block) — the `screenshot` verb with one optional `id`, its result `width` · `height` · `png`, the crop and the `fonts-unavailable` refusal, and the hand-written padded base64 (no base64 crate). MCP carries a shot as an image block, and the `command` span records `verb` alone, never the image. The scope's boundary is that a shot crosses only as it already does, so no new verb, field, cause or span.

@history_item n=5 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the scopes, the tests and the stack) — added `stand_screenshot_repeat`: one state's shot is byte-identical across two child runs and the parent, 12 shots, on one host. This is the existing same-host byte-equality precedent and the check form a cross-host canvas check would extend. It also added the private `image` module (PNG encoder plus base64), and `png` 0.18.1 became a blitz-tests dev-dependency so tests can decode a shot.

@history_item n=6 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs; Test harness) — moved the overlay scrollbar fade onto the session's clock, so two shots of one state with no `advance` between them read the same pixels. The harness's `VirtualClock` is never moved by `screenshot`. Those are `decision/screenshot-reads-session-clock`, `decision/one-virtual-clock` and `contract/test-harness/virtual-clock`. A pixel comparison must hold the clock still, or wall-time fades come back.

@history_item n=7 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder) — ratified the one-virtual-clock marks, the shot's fade on the session's clock among them. Shot determinism within a host is therefore founder-ratified.

@history_item n=8 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — added `decision/stand-readings-same-on-every-ci-os`: the served tasks read the same snapshot, lint and actions digests on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. Its guard is `stand_cross_os_readings`. It restated `decision/stand-typography-dejavu-sans`: one bundled DejaVu Sans, system fonts off, every family by fallback, guarded by `stand_bundled_font_families`. This is the cross-OS check this chunk's canvas shot check sits beside, and the font premise behind the hypothesis that glyph coverage may now match across hosts.

@history_item n=9 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Standard Contracts 1 of 2 and 2 of 2) — the cross-OS readings row moved from six to seven served tasks with the order page. The stand boot list gained the order page, beside the component-alone boot. This is the current form of `decision/stand-readings-same-on-every-ci-os` and `contract/headless-stand/boot`.

@history_item n=10 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — built `seven_guis::stand`: a pinned 800 × 600 · scale 1.0 · Light viewport, bundled DejaVu Sans with system fonts off, offline options, no telemetry, env read or `blitz_net`. The bundled bytes are compiled on native too. These are `decision/stand-pinned-environment`, `decision/stand-uses-bundled-font` and `contract/headless-stand/no-telemetry`. Every stand fixture this chunk boots inherits that environment.

@history_item n=11 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand's typography in the Headless stand contract, F2 ratified) — ratified that the stand's typography is DejaVu Sans, stated in the stand contract and not on design-system's surface. A canvas text shot's glyphs rest on that one font.

@history_item n=12 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — added `decision/canvas-crosses-as-pixels`, ratified for all carriers (F1): a canvas's app-painted content crosses only as pixels in a screenshot, with no new carrier, verb, field, exit, cause, error, log field or span. It also added `decision/harness-paints-custom-widgets`: blitz-test-harness names blitz-paint's `custom-widget`, guard `stand_canvas_shot`. This is the immediate predecessor whose shot this chunk measures across hosts. The scope's no-new-surface boundary is this decision.

@history_item n=13 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** (Established Decisions, both entries) — added `decision/custom-widget-content-box-paint`: paint size is the content box times the scale, appended at the content origin inside the transform, unclipped, guard `stand_canvas_placement`. It was then marked ratified as a direct engine change (F1), upstreamable for issue 1578. A canvas shot's geometry and its antialiased edges come from this placement.

@history_item n=14 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** (Standard Contracts) — added `contract/nodes-widgets/widget-fonts` (`Widget::fonts`, the document's shared Parley context handed over before each paint) and `contract/base-document/font-ctx` (F2). Canvas text paints through the stand's one bundled font, which this chunk's glyph-coverage reading across hosts depends on. This is `stand_canvas_text`'s fixture form.

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `blitz-paint/font-embolden` is exercised on CI's macOS leg and built locally only by the `embolden` step of `fast`. Trap: macOS builds paint text with a different feature set than linux, which is a cross-host pixel-difference source separate from the SIMD hypothesis. Any text in a canvas shot must be read under it.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (no local gate builds `font-embolden`) — a local gate read green over a paint defect only macOS showed, and CI's first run went red on `Test (macos)` alone. This is the precedent that a paint reading can differ on macOS and is reached only through CI, as this chunk's measurement is.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers, a boundary widening) — the open select list's labels cross as painted pixels in a `screenshot`, ratified as a boundary widening with no carrier, field or span added. This is the precedent and source of `decision/screen-pixels-in-band`, beside which canvas pixels cross.

@history_item n=18 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — `stand_contrast`'s painted witness is held "within 1 per channel" against the computed colours. This is the existing per-channel tolerance precedent in the stand checks, relevant to the founder's TOLERANCE option.

@history_item n=19 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (fork CI on the build branch through one leg script) — CI triggers on `build/**`. fmt, clippy, test-features-default and ci-scripts carry no `needs`, and every other job, the matrix included, needs all four. The matrix tests windows and macos, and linux is tested by the fast `test` leg. These are `decision/ci-fast-slow-split`, `decision/linux-tested-by-fast-leg` and `decision/github-actions-ci`. The measurement reaches macOS and Windows only through this path, after the fast set.

@history_item n=20 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added the `stand-repeat` leg: three rounds of every `stand_*` target, first red ends it, no retry, printing `stand-repeat: 3 of 3 rounds green`. These are `decision/stand-repeat-zero-failure-budget` and `contract/ci-leg-legs/stand-repeat`. A new `stand_` canvas check joins it by prefix and must be flake-free under the zero-failure budget.

@history_item n=21 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` names further targets as literal `--test` words after the `stand_*` glob, pinned by `test_stand_repeat_leg_runs_three_rounds`. A canvas shot check without the `stand_` prefix would need a named entry there.

@history_item n=22 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (`stand-repeat`'s membership held jointly) — trap: `stand_requirement_sweep`'s leg check reads `ci-leg.sh` whole by `contains`, so a target dropped from `stand-repeat` alone still reads inside while another leg names it. Membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`.

@history_item n=23 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests keeps one test target per file) — blitz-tests keeps one target per file directly under `tests/`, with upstream's `tests/all.rs` held at `test = false`, pinned by `BlitzTestsTargetsTest`. Re-executing checks select a child by `--exact`. These are `decision/blitz-tests-target-per-file`, `decision/blitz-tests-one-target-per-file` and `decision/blitz-tests-single-test-target`. A new canvas cross-host check is its own auto-discovered target.

@history_item n=24 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Conventions) — `decision/both-layout-modes` gained its exception: upstream-taken targets are kept as taken, and escher's own targets run both layout modes. The scope's measurement and check run in both modes under this convention.

@history_item n=25 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the headless stand serves Cells and a load page) — the stand boots Cells and a load page. Every boot is offline but the load page's, whose `.test` URL `LoadNet` answers in process after 200 ms. These are `decision/stand-offline`, `contract/headless-stand/offline-boots` and `decision/stand-load-page-in-process-url`. The canvas fixture boot stays offline.

@history_item n=26 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (both Standard Contracts entries) — added `boot_alone` (one component alone at the stand's viewport and font) to `contract/headless-stand/boot`, and the absence `absence/component-alone-on-carriers`. The scope's canvas fixture is "booted alone" like `stand_canvas_shot`'s, reached in process only.

@history_item n=27 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — `Session::reach` / `start_in` reach a named fixture state in process. `absence/fixture-state-on-carriers` holds no fixture state on any carrier. A recorded session started in a state replays from boot (`contract/driver-recording/kept-check`). The canvas fixture must not reach the carriers either.

@history_item n=28 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` style traversals serialised) — no headless document (harness, stand or driver) sets `style_threading`, so Sequential stays the default. These are `decision/headless-documents-default-threading` and `decision/headless-documents-sequential-style`. Keeping threading fixed is one of the determinism premises for a cross-host shot.

@history_item n=29 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Stack and Technologies; Conventions) — the AnyRender 0.15 set replaced vello-hybrid with vello-gpu in `decision/anyrender-rendering-abstraction` and `decision/window-renderer-chosen-at-compile-time`, and seven_guis' default renderer feature became `vello-gpu` (`decision/in-repo-default-features-off`). This is the current renderer set around the CPU shot renderer.

@history_item n=30 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Stack and Technologies) — Parley moved 0.12 → 0.13 (`decision/parley-text-layout`) and Stylo 0.22 → 0.24. These are the text-layout versions a canvas text shot's glyph positions rest on at HEAD, unlike the reading CI#39225375577 was taken on.

@history_item n=31 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Stack and Technologies) — Taffy now comes from upstream's git source pinned to one revision (`decision/taffy-layout`). Layout geometry for the canvas fixture rests on this pin.

@history_item n=32 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Established Decisions) — added `decision/default-font-fallback`: when no listed family is available, the default font is used for text and font metrics. On the stand that means the bundled font, which keeps glyphs host-independent.

@history_item n=33 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Established Decisions, font-face) — `decision/font-face-css-descriptors` now registers `@font-face` under family, weight, style and width (`font-stretch`). This bears on font matching for any fixture text.

@history_item n=34 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`text-transform-icu` in four default lists) — blitz-dom's default feature list holds `custom-widget` among svg, woff, accessibility, system-fonts and file-input, fixed per crate in its manifest (`decision/per-crate-default-features`, `decision/feature-gated-widgets`). The harness names blitz-paint's `custom-widget` against these lists.

@history_item n=35 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`CustomWidget`'s attribute replay) — `ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed`, behind `custom-widget`. This is custom-widget behaviour the canvas fixture's object element (`decision/custom-widget-object`, `contract/custom-widget-embed/object-data`) goes through.
