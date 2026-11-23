@history_item n=1 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — added `+ decision/app-widget-shows-in-shots`: a Rust-painted widget shows in harness and driver shots, whole and cropped, both layout modes, per package and in the workspace build (guard `stand_canvas_shot`). This chunk's canvas fixture and its cross-host check extend this row. Its fixture boot is the one to copy.

@history_item n=2 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (test counts, the shot's same-host limit) — restated `stand_screenshot_repeat`'s byte-identity as true on one host only. A shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577, `Test (macos)`), so no check compared byte for byte across OSes may hold a `screenshot` step, a committed check's digest included. This is the rule the chunk measures and may amend, through `decision/no-screenshot-in-cross-os-checks`, `decision/no-shot-in-cross-os-byte-checks` and `decision/cross-os-checks-hold-no-shot`. Its basis is a whole-stand shot from before the bundled fonts.

@history_item n=3 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the kept answer, the replay and the committed checks) — a kept answer holds no shot `png`. The committed checks under `tests/blitz-tests/checks/` are replayed by `stand_replay` and kept fresh by seven_guis `recording`. The shot path's kept form is covered on linux and macOS only, since recording is unix only. This bounds the chunk: the kept check keeps no pixels, and the committed Counter checks stay free of a `screenshot` step unless the founder amends that too.

@history_item n=4 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — added `+ decision/cross-os-readings-by-answer-digest`: one stand check compares answer digests (snapshot, lint, actions) to one committed table on every CI OS, with no shot. It also guarded `decision/stand-checks-bundled-font` with `stand_bundled_font_families`. This is the precedent and template for a check "equal to one committed table" on every CI OS, and the font basis that may have changed the no-shot rule's premise.

@history_item n=5 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (2. Test Strategy) — the cross-OS digest row moved from six to seven served tasks. `decision/count-and-rename-sweeps` gained its exception: a count-worded test name pinned as a witness string is kept, and names are searched before any rename. Both bind a new cross-host table check and any rename of the no-shot rows' guards.

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the counts and the new checks) — the harness paints with no display, and frames are compared whole: the PNG equals the raster and is byte-identical across processes under both builds. A no-font shot is refused, never skipped. The harness takes blitz-paint without default features. This is the origin of `decision/frames-compared-whole`, `decision/headless-screenshot` and `decision/shot-png-deterministic-per-host`, which a cross-host reading would extend past one host.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the harness and session contracts gain the shot) — set `screenshot` / `screenshot_rect`: harness time, `VelloCpuImageRenderer`, white composite, `FontsUnavailable` and `EmptyRect`, and no print, panic or log. A shot's line holds no image byte, and its PNG crosses in band as base64 under the unchanged bound (founder fork 1). This is the frame of the chunk's boundary: no new verb, field or carrier, and pixels cross only as they already do.

@history_item n=8 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (§1, §3, §4, §5, §6, §8) — the harness's time is a `VirtualClock` that `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move (ratified 2026-10-18). A shot taken across hosts rests on this determinism: no tick and no pump at shot time.

@history_item n=9 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the clock checks counted) — the overlay fade reads the document's clock, so a shot after a scroll repeats until `advance` moves it (`stand_virtual_clock`). This is a trap for a cross-host shot taken after an acting step: time-dependent paint must be pinned by the virtual clock.

@history_item n=10 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (3. Test Harness Contract: 6 rows) — added `Harness::from_component_alone`, `vdom_alone`, `stand::boot_alone` and `decision/component-alone-props-as-maker`. The chunk's canvas is a stand fixture booted alone, as `stand_canvas_shot` boots its own.

@history_item n=11 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (3. Test Harness Contract: 1 row, 2 of 2) — added `decision/component-alone-root-context-before-build`: a root context added after the build reaches no render. This binds the new fixture's boot.

@history_item n=12 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (2. Test Strategy) — added `decision/pinned-viewport-read-from-document`: a component alone sizes its root to its content, so the pinned viewport is read from the document. This matters for a whole-screen canvas shot's dimensions, which must be equal on every host.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the macOS-only red and `blitz-paint/font-embolden`'s local witness) — CI#41233234207 was red on `Test (macos)` alone through `apple-font-embolden`: paint differs on macOS by a feature no linux gate built. `cargo` stops at the first failing target, so a red job's later targets go unread. This is a direct cause of a cross-host pixel difference for any glyph in a canvas shot, and a trap for reading the matrix logs.

@history_item n=14 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` now ends with `embolden` (`--features blitz-paint/font-embolden --test select_popup_painted`), pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. This is the local witness of macOS paint, and the route for reproducing a macOS-only glyph difference on the linux dev host.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`select_popup_painted`) — the select list is read on frames against engine constants. A closed select's frame is byte-equal before opening and after a close. The header's weight under font-embolden is pinned by a local witness. This is the precedent for frame-level pixel pins whose glyph rows move by host feature.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the list leaves unmeasured) — the list paints through `paint_scene` alone and is driven headlessly by harness, session and binary (row `decision/select-list-painted-by-paint-scene`). This is the paint path a canvas shot shares on every host.

@history_item n=17 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (4. Unit Test Strategy) — `decision/expected-value-pinned-as-literal` gained its exception: a pixel row that font-embolden moves is pinned as a stated band, with its reason in a comment, where the test crate cannot see the feature, and wholly-inside assertions are kept exact. This is the precedent if the founder picks a tolerance over byte equality.

@history_item n=18 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (4. Unit Test Strategy) — added `+ decision/expected-value-pinned-as-literal`: a check pins its measured value as a literal, never derived from the constant its control mutates. A committed cross-host table or tolerance must be measured literals.

@history_item n=19 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (4. Unit Test Strategy) — `decision/mutation-controlled-tests` now reads that a case the control leaves green is widened until it reds. The new cross-host check needs a named red control.

@history_item n=20 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (4. Unit Test Strategy) — `decision/mutation-controlled-tests` gained the bound-loosening clause: a control removing a bound loosens it to twice the budget. This applies if the check holds a tolerance bound.

@history_item n=21 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (CI on the build branch) — CI runs on PRs and on pushes to main, v0.* and build/**. The matrix tests windows and macos and only builds ios and android. Linux is the `test` leg's, through `ci-leg.sh`, and the fast/slow split gates every other job. This is the basis of `decision/native-platform-ci-matrix`, `decision/ci-trigger-branches` and `decision/ci-fast-slow-split`: the only route to macOS and Windows readings, as the scope's premises state.

@history_item n=22 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (9. CI Integration: 2 rows, skia) — added `decision/matrix-skia-from-verified-archive` and `decision/skia-witness-reads-by-content`: every matrix row fetches and verifies its skia archive, with a witness step after test. This sits on the matrix job whose `cargo test --all --tests` will run the new check on macOS and Windows.

@history_item n=23 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (9. CI Integration: 2 rows, hosted runner) — added `decision/hosted-runner-cancel-hand-rerun` (no automatic re-run, read-only token) and `decision/ci-flag-rebuild-limit`. The measurement runs depend on hosted runners, and a cancelled run is answered by a hand re-run.

@history_item n=24 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (9. CI Integration: 1 row) — `decision/hosted-runner-cancel-hand-rerun` changed from provisional to standing on the founder's own word.

@history_item n=25 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — new "Engine features by runner": `cargo test --workspace` builds blitz-dom with `tracing` on, while per-package builds do not. A green per-package run is not the workspace leg's green (row `decision/per-package-green-not-workspace-green`). The matrix's `cargo test --all --tests` and the per-package builds differ in features, so a shot must be read under each.

@history_item n=26 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (readings on the merged tree) — the workspace build resolves blitz-dom with `writing-mode` on in that build only, which selects one of two bounds-reader bodies. The same stand check runs over different code per runner. This is a cause of geometry, and so pixel, differences between builds rather than hosts.

@history_item n=27 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (three engine checks) — each of the bounds reader's two bodies holds a check, with mutation controls red under its own build only. This is the origin of `decision/bounds-reader-bodies-each-checked`.

@history_item n=28 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`tests/all.rs` declared and never built) — `BlitzTestsTargetsTest` pins one target per file, with upstream's `all` at `test = false`. The new stand check is its own auto-discovered target with no manifest line (rows `decision/one-test-target-per-file`, `decision/blitz-tests-manifest-difference`).

@history_item n=29 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (bridge tests, the shared stand module) — `tests/blitz-tests` holds one test-target file per behaviour plus shared modules that are no target, and the stand tables live in `tests/common/mod.rs`. This is the basis of `decision/shared-test-modules-not-targets` and `decision/shared-stand-tables-in-one-module`.

@history_item n=30 marker=2026-11-20-stand-test-setup-shared
**2026-11-20-stand-test-setup-shared** — `decision/shared-stand-tables-in-one-module` gained its exception: a helper whose body or failure message differs stays in its check. This governs where a cross-host table or helper for canvas shots lives.

@history_item n=31 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `tests/fixtures/mod.rs` was added: a page read by more than one check is stated once, and a page with one reader stays in its file. The module holds no print, env read, file, socket or `blitz_net`. This applies if the canvas fixture is shared between `stand_canvas_*` checks (rows `decision/shared-fixtures-in-one-module`, `decision/shared-fixture-pages`).

@history_item n=32 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (7. Test Data & Fixtures: `stand-boots-offline`) — `decision/stand-boots-offline` names its exceptions (LoadNet, FailingNet). A new canvas boot must keep no network provider.

@history_item n=33 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, in the package-alone build too. The skip covers pre-existing tests only. This is the basis of `decision/stand-checks-bundled-font`, `decision/stand-pinned-real-font` and the font-assertions exception, and the font a canvas glyph uses on every host.

@history_item n=34 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the zero-failure flakiness budget (stand-repeat, three rounds, no retry). It also found that no `document.rs` font test skips: the skip was the fixture, not missing system fonts. This is the basis of `decision/stand-checks-zero-failure-budget`, `decision/zero-failure-stand-repeat` and `decision/font-metric-tests-never-skip`. A `stand_*` canvas check joins stand-repeat by its prefix.

@history_item n=35 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the `hover_state_tests` run on the two-argument fixture with no skip. This is the guard of `decision/font-metric-tests-never-skip`.

@history_item n=36 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` runs ten named targets) — stand-repeat runs the `stand_*.rs` glob plus literal named `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`. A non-`stand_` check would need a named word and would move the count the capability pin holds.

@history_item n=37 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (2. Test Strategy) — added `decision/requirement-witness-inside-stand-repeat`: each requirement names its witness inside stand-repeat, pinned by function name. v040-09's cross-host leg would be witnessed there once the next entry claims it.

@history_item n=38 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — each file joining a leg keeps its legs (row `decision/joining-file-keeps-its-legs`). A per-job tally strips ANSI with `(\x1b|\^\[)\[[0-9;]*m`, because the log spells ESC as `^[`. This is a trap when reading the macOS and Windows job logs for the measurement.

@history_item n=39 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the per-OS CI tallies re-read) — in a GitHub Actions job log, the stderr `Running …` lines and the stdout `test result:` lines do not interleave in order. A tally move is attributed by grepping the test's own name. This is a trap for reading per-host shot readings out of CI logs.

@history_item n=40 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — trap: a headless key press differs by platform (the editor's Backspace arm is compiled out on macOS, green on Linux and red on macOS). The cross-host check's acting step should use clicks and typed characters.

@history_item n=41 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording is `cfg(unix)` item by item, and an off-unix `Unsupported` check runs on windows alone. A cross-host check that keeps a check file cannot run on Windows, which constrains how the Windows reading is taken.

@history_item n=42 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Integration Test Strategy) — `decision/layout-tests-in-both-layout-modes` gained the upstream-target exception. The new check must run both layout modes, as the scope requires.

@history_item n=43 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — `decision/no-sleep-or-wall-clock-in-checks` gained its wall-time exceptions (`stand_load_wait`). A canvas shot check holds no sleep and no wall-clock read and moves time by `advance` alone.

@history_item n=44 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (2. Test Strategy) — `decision/png-decoding-in-checks` now reads "decodes or encodes" through the png dev-dependency, while seven_guis uses a hand base64 reader. This is how the check reads a PNG's bytes against its decoded RGBA raster to tell the encoder from the rasterizer.

@history_item n=45 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (2. Test Strategy) — added `decision/engine-items-asserted-on-tracing-build`: a build-conditional assertion rule. This is a precedent if a canvas reading must differ between the workspace and per-package builds.

@history_item n=46 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (2. Test Strategy) — added `decision/count-and-rename-sweeps` after a missed numeric pin and a missed renamed witness. Amending or retiring the no-shot rows (guards `stand_session_recording`, `stand_screenshot_repeat`, `the_committed_checks_are_what_the_binary_records`) needs a sweep for their names held as strings.

@history_item n=47 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (2. Test Strategy) — the count sweep was widened to a CI leg's file count held as a literal in CI-script tests. Adding a `stand_*` file moves the stand-repeat and `test`-leg file counts.

@history_item n=48 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `test_capability_statement.py` pins `docs/capabilities.md`. Its claim map names each claim's check, and a renamed or moved check changes it by hand. A changed no-shot or cross-OS claim in the statement must be re-stated there.

@history_item n=49 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (1. Test Scope Summary, `capability-statement-pinned`) — the pin now also holds the a11y leg's file count and stand-repeat's named-target count against `ci-leg.sh`. A leg membership change reddens ci-scripts until the page re-states it.

@history_item n=50 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (1. Test Scope Summary, `capabilities-doc-pinned-to-schema`) — the schema counts and the leg counts are held to their two sources (the schema's tables, the CI leg script).

@history_item n=51 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (2. Test Strategy) — `decision/capability-check-names-held-to-schema` was restated with the leg-count pins. A check that moves between legs changes the claim section by hand.

@history_item n=52 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — the agent-run five-command contract: `run stand` selects `stand_*.rs` by prefix, and with none present it is an empty run that exits 1. Events never carry captured test output. This is the basis of `contract/agent-run-selection/stand`, `contract/agent-run-verbs/*` and `decision/test-output-never-reaches-events`. A `stand_*` canvas check is picked up with no script change.

@history_item n=53 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** (3. Test Harness Contract: 4 rows) — `contract/agent-run-verbs/run` now reads `-p blitz-tests` under stand, all and a file name, and `-p seven_guis --test cli_flow` under session. This is the current run promise the stand selection rests on.

@history_item n=54 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** (10. Quality Gates: 2 rows) — added `decision/oversize-test-targets-kept`: escher's integration targets over 800 code lines are kept whole, one file and one target. This applies if `stand_canvas_shot` grows by the cross-host check rather than a new file.
