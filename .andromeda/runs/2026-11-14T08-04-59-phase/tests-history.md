@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the origin of the bundled font. The stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, in the package-alone build too. The font skip and the `system-fonts` dependency now cover only the pre-existing tests. HarnessOptions went from six fields to eight (`font_ctx`, `incremental`), `stand::options` builds the pinned literal, and the Font payload row adds `seven_guis::DEJAVU_SANS`. This is the register this chunk extends from the stand to every run path (rows stand-checks-bundled-font, stand-pinned-real-font, font-tests-skip-without-font, font-assertions-on-system-fonts).

@history_item n=2 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (counts and strategy) — frames are compared whole: the PNG equals the raster and is byte-identical across processes, under both builds. A no-font shot is refused, never skipped. blitz-dom gains `font_reading_tests`, `stand_act_spans` reads `fonts-unavailable`, and `stand_mcp_tools` serves the causes. The harness takes blitz-paint without default features. These are the guards behind shot-refused-without-fonts and frames-compared-whole, which this chunk must keep while it changes the default font source.

@history_item n=3 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (harness and session contracts) — Harness Core gains `screenshot` and `ScreenshotError::FontsUnavailable`: a document with no font is refused with nothing painted, and nothing is printed, panicked or logged. In session-start, a shot with no font is refused `fonts-unavailable`. These are the contract rows harness-screenshot/fonts-unavailable and driver-session/screenshot. They stay true only if some path can still build a font context that can paint no text.

@history_item n=4 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the refusal causes went from nine to eight, with `fonts-unavailable` the eighth (`the_eight_causes_are_named_in_order`), and seven are served over MCP. This chunk's boundary leaves `fonts-unavailable`'s text and its place unchanged (rows eight-refusal-causes-in-order, refusal-causes/cause-set, mcp-causes-served-with-remedy).

@history_item n=5 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the eight causes in order, `fonts-unavailable` the eighth. Changing that order or wording would need the founder again.

@history_item n=6 marker=2026-10-12-self-description
**2026-10-12-self-description** — each cause's `remedy` is served and pinned in `cli_commands`, `stand_mcp_tools` and json.rs. `fonts-unavailable`'s remedy text is fixed. Under this chunk's boundary that text stays unchanged even if bundling makes the refusal harder to reach (rows cli-refusal/remedy, mcp-causes-served-with-remedy).

@history_item n=7 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (counts and same-host limit) — `stand_screenshot_repeat`'s byte-identity was restated to hold on one host only: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577). So no check compared byte for byte across OSes may hold a `screenshot` step, a committed check's digest included. This is the precedent and constraint behind this chunk's rule that hosts are compared by readings and never by shot bytes.

@history_item n=8 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (kept answers) — committed checks under `tests/blitz-tests/checks/` are replayed by `stand_replay` and kept fresh by seven_guis `recording`. Recording is unix only, so the shot path's kept form is covered on linux and macOS alone. This bears on how this chunk can hold a cross-OS comparison as "a committed reading each OS replays, or a kept check": Windows cannot record.

@history_item n=9 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — a check that records is `cfg(unix)`-gated item by item, beside an off-unix `Unsupported` check that only the windows job runs. This is a trap for a cross-OS proof built on recordings: the Windows runner replays but never records (rows recording-checks-unix-gated, socket-session-checks-unix-gated).

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — checks driving the MCP stdio child through `Speaker` are not unix-gated and run on windows. Only `mcp_server`'s CLI comparison is gated. These checks are the one in-process/child route that reaches all three CI OSes, which matters for this chunk's proof across Linux, macOS and Windows.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (committed checks re-recorded) — the two committed checks were re-recorded through `record_checks.sh`, and their byte sizes are no longer stated. This is a precedent: when a change moves the readings (bounds, text widths), the committed checks must be re-recorded byte-equal (rows recorded-checks-kept-fresh, checks-byte-equal-to-recording).

@history_item n=12 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — §8 records that no `document.rs` font test skips. The skip came from the one-argument `qual_name!` fixture, not from missing system fonts (row font-metric-tests-never-skip, guard hover_state_tests). The same entry added `font_face_sniffed_source` 2, the per-OS tallies record, the coverage floors (blitz-test-harness 90 %) and the zero-failure stand-repeat budget. A harness default change answers to all of these.

@history_item n=13 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the `hover_state_tests` gain a third test on the two-argument fixture, with no precondition and no skip. The font-metric tests that rest on the host's fonts are named here. A bundled default for harness-built documents moves what they measure.

@history_item n=14 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — new §9 bullet, "Engine features by runner": `cargo test --workspace` and the per-package build compile blitz-dom with different features. The same check read green per package and red in the workspace leg, so a per-package green is not the workspace green (row per-package-green-not-workspace-green). `system-fonts` also differs by runner, which is the very source of host font variance this chunk removes.

@history_item n=15 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (readings on the merged tree) — the workspace build resolves blitz-dom with 16 features and the per-package build with 10, `writing-mode` on in the first only, so the same stand check runs over a different bounds reader. A reading compared across OSes and runners must hold under both builds.

@history_item n=16 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`BlitzTestsTargetsTest`) — the blitz-tests manifest keeps one target per file, with `all` held at `test = false` and pinned by `BlitzTestsTargetsTest`. A new cross-OS check this chunk adds is auto-discovered with no manifest line (rows one-test-target-per-behaviour, blitz-tests-single-test-target).

@history_item n=17 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (engine checks) — each of the bounds reader's two bodies (`writing-mode` on or off) holds a check of its own, its mutation control red under its own build only (row bounds-reader-bodies-each-checked). Bounds are among the readings this chunk compares across OSes.

@history_item n=18 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — trap for a cross-OS proof: a headless key press is not the same on every platform. The editor's Backspace arm is compiled out on macOS, so a deleting key read green on Linux and red on the macOS leg. A state compared across OSes must be reached by clicks and typed characters.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (macOS-only red) — CI#41233234207 went red on `Test (macos)` alone through `apple-font-embolden`, and no local gate built `blitz-paint/font-embolden`. The local witness is `--features blitz-paint/font-embolden --test select_popup_painted`, and `cargo` stops at the first failing target. This is a precedent of a font-dependent reading differing on one OS only.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`select_popup_painted`) — the header weight under `blitz-paint/font-embolden` points to the local witness. The check is a named `--test` of `stand-repeat`. This painted font reading is per-OS by feature, so a bundled-font change must keep it green on macOS.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step) — `fast` now ends with an `embolden` step pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. It is the local witness of a paint or font change under the macOS feature, and the gate this chunk's font change must pass locally.

@history_item n=22 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the shared `tests/fixtures/mod.rs` boots pages with `stand::options` and holds no print, `tracing` call, env read, file, socket, listener or `blitz_net` (row shared-fixtures-in-one-module). The scope names "the shared fixture pages" as a run path to bundle. The font must reach them through the options, not through a file read in the module.

@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (served pages) — `stand::boot_load` replaces only `net_provider` in its options and keeps every other field, and `stand::boot_cells` boots Cells (rows stand-boot/boot-load, stand-boot/boot-cells, stand-boots-offline). These are the boot paths the scope requires to read the bundled font. The `font_ctx` field must survive each one.

@history_item n=24 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `Harness::from_vdom` now provides `SeededRandom::from_seed(DEFAULT_SEED)` where the vdom's root holds none, and a root context the builder gave stands. This is the direct precedent for this chunk's default: a run-wide determinism default the harness supplies when the caller names none (here, `HarnessOptions::font_ctx` being `None`), restated in the construction contract row.

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (clock checks) — §7 records that `into_config` puts the harness's clock in `DocumentConfig.clock` and that a wrapped document keeps its own. This is the precedent for how a harness option reaches the document config, and how a wrapped document is exempt. `font_ctx` takes the same route, and `wrap` builds without pumping (row harness-construction/wrap).

@history_item n=26 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — the `file:` rule widened: an engine sub-resource load (stylesheet, `@import`, font, image, iframe) through blitz-net reads no local file for a non-`file:` document, guarded by `local_files_for_local_documents`. This chunk's boundary leaves which font a remote page or the browser reads unchanged. A bundled font must come from an in-tree asset compiled in, not through a `file:` load.

@history_item n=27 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — `stand_lint` asserts `fonts-unavailable`, and blitz-dom's `text_extent` reads `BaseDocument::text_extent`. Lint findings and text extents are among the readings this chunk compares across OSes, and both are font-measured.

@history_item n=28 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (asserted limits) — `text_extent` 6 asserts its three stated limits, and `stand_lint` 7 asserts that a clipped or off-screen text node reads `truncated`. These font-measured readings and their pinned limits move if the font source changes.

@history_item n=29 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` named targets) — `stand-repeat` runs the `stand_*.rs` glob plus a fixed list of named `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`. A new cross-OS check without the `stand_` prefix enters that leg only by being added to the list and the pin (rows stand-repeat-three-rounds, zero-failure-stand-repeat).

@history_item n=30 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (legs reaching a new target) — this entry is the pattern for recording which legs a new target reaches (the workspace `test`, `run all`, `stand-repeat`, `a11y`). A cross-OS witness must be placed in a leg that runs on all three OSes, that is, the `test` matrix, and not `stand-repeat` (linux only).

@history_item n=31 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `SNAPSHOT_TEXT_BUDGET` (10,000 bytes) and `stand_snapshot_text`'s rule that the same text reads across two calls, both layout modes and a second boot (row snapshot-text-budget). The snapshot text is the first of the readings this chunk compares across OSes.

@history_item n=32 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `stand_snapshot_parts` reads a screen over the budget in parts, and `--part` was added. A cross-OS comparison of a large screen (Cells) reads it in parts, each within the budget (rows snapshot-text-budget guard, escher-session-command-line/snapshot).

@history_item n=33 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (Cells' bytes corrected) — the Cells screen measured 47,089 bytes, not the stale 47,949, with the rule that a plan quoting an old reading re-reads it. The Cells screen's bytes and parts are font- and bounds-dependent readings this chunk may move or compare.

@history_item n=34 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (`stand_actions`) — the font-dependent tests record that the `actions` read names `fonts-unavailable` for `screenshot` with nothing painted, `check_target` reading `can_paint_text` after the crop and before painting. This is a current reader of `fonts-unavailable` that must keep working once fonts are bundled (rows driver-verbs/actions, contract/driver-refusals/fonts-unavailable).

@history_item n=35 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§1 coverage) — `password_caret_click` was restated from caret indices "by the real characters' widths" to the bullets. It is a pre-existing font-dependent check whose readings depend on which font measures the text. It is affected if harness-built documents default to the bundled font.

@history_item n=36 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `decision/layout-tests-in-both-layout-modes` gains its exception: upstream targets taken in a sync that run one layout mode stand as named exceptions. A new cross-OS check must assert both layout modes (row stand-checks-in-both-layout-modes).

@history_item n=37 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (per-OS tallies) — the per-OS tallies were re-read on CI, with a rule: a GitHub Actions log interleaves `Running …` and `test result:` out of order, so a tally move is attributed by grepping the test's own name. This applies when reading this chunk's cross-OS witness on the macOS and Windows jobs.

@history_item n=38 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — a per-job CI tally strips ANSI codes with `(\x1b|\^\[)\[[0-9;]*m`, because the log spells ESC as the literal `^[`. This is a trap when reading per-OS test results for the cross-OS proof.
