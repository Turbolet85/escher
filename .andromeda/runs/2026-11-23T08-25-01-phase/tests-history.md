@history_item n=1 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** (amd-5e244nch, §2 Test Strategy) — added `+ decision/app-widget-shows-in-shots`: a Rust-painted widget shows in harness and driver shots, whole and cropped to its element, both layout modes, per package and in the workspace build, guarded by `stand_canvas_shot`. This chunk's placement fix moves what such a widget paints, so it rests on this row and its guard. The scope measured `stand_canvas_shot` green with the fix prototype because that widget has no padding or border. New padding, border and transform cases extend this witness.

@history_item n=2 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (amd-rrbdvino, §3 harness and session contracts) — the harness gained `screenshot` / `screenshot_rect`, `Raster` and `ScreenshotError::{FontsUnavailable, EmptyRect}`. A shot runs at the harness's own time through `VelloCpuImageRenderer`, composited over white, and a document with no font is refused with nothing painted, never printing, panicking or logging. A `screenshot` command line records the verb with no image byte. The PNG crosses in band as padded base64 (the founder's fork 1). The crop is refused `off-screen` and no font gives `fonts-unavailable`. These are the screenshot contract rows the scope says stand unchanged (harness-screenshot/*, command-span/screenshot, screenshot-in-band, driver-session/screenshot, escher-session-command-line/screenshot).

@history_item n=3 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (amd-db7lxiqh, §1/§2/§9 counts) — this is where pixel tests got their rules. The harness paints with no display, frames are compared whole (the PNG equals the raster and is byte-identical across processes under both builds), and a no-font shot is refused, never skipped. The harness takes blitz-paint without default features. It is the precedent for the canvas checks' frame comparisons and their per-package and workspace runs (decision/frames-compared-whole, shot-refused-without-fonts, shot-png-deterministic-per-host).

@history_item n=4 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (amd-siqngma2) — `stand_screenshot_repeat`'s byte-identity was restated as holding on one host only. A shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step. This is the source of no-screenshot-in-cross-os-checks, no-shot-in-cross-os-byte-checks and the guard `the_committed_checks_are_what_the_binary_records`. A canvas-text check must not pin shot bytes across OSes, and cross-host canvas equality is the out-of-scope «Canvas shot across hosts».

@history_item n=5 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (amd-7qex4fkt) — added the committed checks under `tests/blitz-tests/checks/` and `stand_replay`. The shot path's kept form (a shot kept without its `png`) is covered by `stand_session_recording` on linux and macOS. This is the basis of the committed-checks excluded-content row (no screenshot step) and the guard of decision/no-screenshot-in-cross-os-checks.

@history_item n=6 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** (amd-2gzx73ga) — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, in the package-alone build too. The skip covers pre-existing tests only. `HarnessOptions` gained `font_ctx` and `incremental`, `stand::options(incremental)` builds the pinned literal, and `seven_guis::DEJAVU_SANS` became the font payload. Canvas text must come from this same bundled set (decision/stand-checks-bundled-font, stand-pinned-real-font, contract/stand-boot/options). The scope's widget shapes from these `DEJAVU_SANS` bytes, and no font file enters the tree.

@history_item n=7 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (amd-uimtzpsg, §2) — added `+ decision/cross-os-readings-by-answer-digest` (the served tasks read cross-OS by answer digests, no shot) and gave decision/stand-checks-bundled-font its guard `stand_bundled_font_families`: every generic family and an unavailable named family read the bundled sans-serif extent in both layout modes. This chunk draws canvas text from the document's own font context. That is the set this guard holds, and v030-14's acceptance names canvas text as its 0.4.0 reader.

@history_item n=8 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (amd-mmdtifvl) — decision/cross-os-readings-by-answer-digest went from six to seven served tasks. decision/count-and-rename-sweeps gained its exception: a count-worded test name that is pinned as a witness is kept. The scope requires the seven served tasks' readings (`stand_cross_os_readings`) to stand unless a ruling moves one, then re-recorded and never loosened.

@history_item n=9 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (amd-ybvlv3ad) — the harness's time became a `VirtualClock` its documents read. `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (ratified 2026-10-18 by amd-oodkokb2). This is the source of decision/harness-virtual-clock. A canvas shot must stay a pure read: no tick or pump.

@history_item n=10 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (amd-h47hxvhq, §9) — added `+ decision/doc-leg-gates-public-doc-edits`. Public doc comments that linked a crate-private item read green locally and red only in CI's Documentation job. F2 A adds two public items with `///` docs (`BaseDocument::font_ctx()` and the defaulted `Widget` method), so the `doc` leg must join the local gate block.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (amd-s53lzeiq, §9 Engine features by runner) — the same check file read green per package and red in the workspace leg. A green per-package run is not the workspace leg's green, because `cargo test --workspace` builds blitz-dom with features the per-package build lacks. This is the source of decision/per-package-green-not-workspace-green. The engine changes in blitz-paint and blitz-dom events must be read green under both runners.

@history_item n=12 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (amd-cqdq6a7d) — the workspace build resolves blitz-dom with 16 features and the per-package build with 10. `writing-mode` is on in the workspace build only, and it selects which body of `physical_unrounded_geometry` is compiled. The follow-up amd-pzdy3myb found that a fix in one body reads green under the other build. This is a trap for the content-box offset and event-coordinate changes: check which feature-gated code each runner compiles, `custom-widget` included.

@history_item n=13 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (amd-xt3nypar, §4) — added `+ decision/harness-graph-tests-enable-accessibility`: a per-package test build compiles a feature-gated surface out unless the dev-dependency turns the feature on. This is the precedent for the chunk's `custom-widget`-gated items (the `Widget` method and `BaseDocument::font_ctx`) and for the blitz-tests `parley` dev-dependency. The test crate must enable the feature it reads.

@history_item n=14 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (amd-3quv32td, amd-ptvfk25e, §3) — added `+ contract/stand-boot/boot-alone`, `+ contract/harness-construction/from-component-alone`, `+ decision/component-alone-props-as-maker` and `+ decision/component-alone-root-context-before-build` (TimerTicks provided on the root before the build, since a context provided after it reached no render). The picked stand-boot rows rest here. Any canvas fixture booted alone must keep that order.

@history_item n=15 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (amd-z6xuttzw, §4) — added `+ decision/expected-value-pinned-as-literal`: a check never derives its expected value from the constant its control mutates, but pins the measured literal. The content-box size `(100, 60)`, the offsets and the «Hg» pixel count must be pinned as measured literals.

@history_item n=16 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (amd-xnhp3j2e) — `~ decision/expected-value-pinned-as-literal` gained an exception: a pixel-row pin that `font-embolden` moves is pinned as a stated band, with its reason in a comment where the test crate cannot see the feature, and its wholly-inside assertions stay exact. This applies if a canvas-text pixel reading moves under the macOS `apple-font-embolden` build.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-23w5gd5l) — CI#41233234207 went red on `Test (macos)` alone through `apple-font-embolden` while every local gate read green. A paint change under `blitz-paint/font-embolden` has its local witness in a `--features blitz-paint/font-embolden` run. This is a trap for a blitz-paint change that paints text.

@history_item n=18 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (amd-yx45xaju, §9) — `fast` now reads fmt → clippy → test → ci-scripts → embolden. The `embolden` step (`--features blitz-paint/font-embolden --test select_popup_painted`) is the pinned local witness of a paint change under the feature. It bears on decision/local-pre-push-gate and on the blitz-paint edits in `process_custom_widget_node` and `draw_custom_widget`.

@history_item n=19 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (amd-3j6rcoyw, §9) — `~ decision/local-pre-push-gate`: on a dev host with CI unset, a green fast leg ends with the target sweep, whose refusal never reddens the leg. Also added `+ decision/target-sweep-safe-around-builds`. This is the current form of the local gate the chunk lists.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-ylvtajnx) — `select_popup_painted` is the precedent for reading engine paint on a frame's `Raster` against stated constants, in both layout modes, as its own target with no `stand_` prefix, run as a named `--test` of `stand-repeat`. It also records that blitz-paint's `render/select_list.rs` holds no unit test. This is a model for a blitz-paint placement check (decision/pixel-tests-on-cpu-buffer).

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (amd-giaue4qm) — blitz-dom's mutator.rs gained `custom_widget_replays_initial_attributes`, the `CustomWidget` replay test. It is the one existing blitz-dom unit test on the custom-widget path that the scope's F1 cites (security-plan decision/custom-widget-initial-attribute-replay). The chunk's `custom-widget` engine changes sit beside it.

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (amd-xkpz46v3) — recorded as gaps two blitz-paint behaviours that no check reads on a frame: `mix-blend-mode` and `isolation` painting, held by unit tests alone, and `render_element` skipping an inline box's own shadow, background and border. Each got a CARRY owner. This is the precedent that a blitz-paint change in what a box paints is read on a painted frame or recorded as a gap with an owner. It applies to the `draw_custom_widget` placement fix and the unclipped scene (F1 B not taken).

@history_item n=23 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (amd-stldjcwi, §4) — `~ decision/mutation-controlled-tests`: a case the mutation leaves green is widened until it reads red, and a bound-removing control loosens the bound to twice the budget. The scope requires every new check run red first and that red kept in evidence. The earlier form comes from amd-knbkpfwu (2026-11-14-auto-exploration).

@history_item n=24 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (amd-n5xqwtgz) — set the zero-failure flakiness budget on the stand checks (decision/zero-failure-stand-repeat). It also found that no `document.rs` font test skips: the skip came from the one-argument `qual_name!` fixture, not from missing system fonts. This is the origin of decision/font-metric-tests-never-skip. Canvas checks join `stand-repeat` by prefix or by a named word, three rounds with no retry.

@history_item n=25 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** (amd-jef7zjyx, §8) — the font-metric `hover_state_tests` gained a third test on the same two-argument `qual_name!` fixture, with no precondition and no skip. That is the guard of decision/font-metric-tests-never-skip, which canvas text checks must follow (assert the text-size precondition, never skip).

@history_item n=26 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (amd-cixe5kae) — `stand-repeat` runs the `stand_*.rs` glob plus named non-prefixed targets as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`. Later counts were moved by amd-w6i3vy4y and amd-kube42bu. If the chunk adds a placement or text check without the `stand_` prefix, the leg list, its pin and the capability statement's named-target count move together.

@history_item n=27 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (amd-n2omhba3, §2) — `~ decision/count-and-rename-sweeps`: a change of a CI leg's file count or of the driver's verb count sweeps the tests, examples and CI-script tests for literal pins. This applies if a new canvas target joins `stand-repeat` or the a11y leg (pinned in `docs/capabilities.md` via decision/capability-check-names-held-to-schema).

@history_item n=28 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (amd-aiakktz6, §2) — `~ decision/capability-check-names-held-to-schema`: the claim section's names and counts are pinned, the a11y leg's file count and stand-repeat's named-target count to `ci-leg.sh`. A check renamed or moved changes that section by hand. This applies if v040-09's claim or a leg's membership moves.

@history_item n=29 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (amd-u7vpyh4t) — the fork keeps one auto-discovered target per file, with upstream's `all.rs` held at `test = false`, pinned by `BlitzTestsTargetsTest` on the operator's 2026-10-10 decision. This is the source of decision/one-test-target-per-file and blitz-tests-one-target-per-file. New canvas checks are each their own file with no manifest line.

@history_item n=30 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (amd-ur7tfp4h) — `tests/blitz-tests` holds one test-target file per behaviour plus shared modules that are no target (`tests/common/mod.rs`). This is the origin of decision/one-test-target-per-behaviour. Trap: a sed or grep address ending at a function name also selects every item whose name opens with it.

@history_item n=31 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** (amd-nlavi4cc) — added `tests/fixtures/mod.rs` as the third shared module. A page two checks share goes in `fixtures`, and a page with one reader stays in its file. This applies if the canvas placement and text checks share a widget page.

@history_item n=32 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (amd-bcwcmey4, §2) — `~ decision/png-decoding-in-checks`: a blitz-tests check decodes or encodes a PNG through the `png` dev-dependency, while seven_guis reads it with a hand base64 reader. Canvas shot checks decode the same way.

@history_item n=33 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (amd-lde2vbr3, amd-qi7vgvwx) — added `+ decision/blitz-dom-requests-through-stamping-constructor` (a crate-level clippy disallowed-methods config in blitz-dom, so the clippy leg fails on any other request construction) and `+ decision/real-provider-inside-stand-in`. The chunk edits blitz-dom (`document.rs` accessor, `events/mod.rs`), so the clippy leg's blitz-dom config is in play.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-tcnuzkmh, §9) — `decision/cargo-legs-locked`: every cargo leg is `--locked` except `fmt` and `examples/wasm_hello`. The scope requires `--locked` on every leg, and `parley` enters as a dev-dependency already in `Cargo.lock`.

@history_item n=35 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (amd-aeuarcbq) — remeasured the local baseline under `debug = "line-tables-only"`: the dev host, workspace default features, every cargo command locked and no opt-level rewrite. This is the reference of decision/local-baseline that the chunk's gate timings and counts are read against.

@history_item n=36 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (amd-gkbc5lsj, §5) — `~ decision/layout-tests-in-both-layout-modes` gained an exception: upstream targets taken in a sync stay as written, and the one-mode ones stand as named exceptions. Any new placement or text check of ours runs both layout modes (scope: «Both layout modes»). Precedent: amd-wb5mdgi4 (2026-10-31-engine-debts) ended `pseudo_element_flow_classification`'s exception by looping both modes.

@history_item n=37 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** (amd-nsqxmuql, §10) — `~ decision/complexity-ceiling-host-only` gained an exception: the engine's click handler is held below the ceiling with upstream's arms, at the price of a hand re-apply when its arms are edited. The F2 ruling (amd-ihzt4hfv, 2026-11-20-codebase-health) ratified keeping upstream functions over the ceiling. The chunk edits the `custom-widget` arm of blitz-dom's `handle_dom_event` and the blitz-paint widget functions, which these rows govern.

@history_item n=38 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (amd-s4wow4l3) — the three-outcome telemetry target allowlist (engine target safe fields, `escher_` redacted, other targets dropped) proved by `telemetry_drop` and `host_log` reading no id or name at `trace`. This is the origin of decision/telemetry-target-allowlist and no-ids-or-names-in-stand-logs. The scope allows no new log field or span and no content in a log field, and canvas text must not reach a log.

@history_item n=39 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** (amd-evt52mzu) — the stand log's second record class (closed spans as lines) was ratified by the founder. This is the source of decision/closed-spans-logged-as-lines.

@history_item n=40 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** (amd-2xzkhreo) — the agent-run 5-command contract was measured: `run stand` selection by prefix (an empty run is exit 1), `run {name}`, and the events never carrying captured test output. This is the origin of the picked agent-run rows. A new `stand_*` canvas check is picked up by its prefix with no script change.

@history_item n=41 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** (amd-d7r7mqwr) — `~ contract/agent-run-verbs/run`: `run` covers `-p blitz-tests` under stand, all and file names, and `-p seven_guis --test cli_flow` under the `session` selection. `decision/test-data-is-code` was restated. This is the current form of the picked agent-run run and selection rows.

@history_item n=42 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-n3udqhi7) — the `actions` read names `fonts-unavailable` for `screenshot` with nothing painted. `check_target` reads `can_paint_text` after the crop and before painting. This is the basis of contract/driver-refusals/fonts-unavailable. Handing the widget the document's font context must not change when `can_paint_text` reads false, or the refusal order.

@history_item n=43 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (amd-ur5xsokp) — `layout_rect`, `layout_rect_of` and `center_of` read a scrolled box where it stands (amd-epgtg3nr earlier pinned the offset reading). This is the current form of contract/harness-inspect/layout-rect. The placement checks will read a widget's content box against `layout_rect`, which gives the border box.

@history_item n=44 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (amd-jy34nxjf, §9) — the per-OS tallies were re-read. In a GitHub Actions job log, `Running …` lines and `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name. This applies to the chunk's CI re-read of the unread `d43bc0e7f` verdict and its own run.
