@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (CI on the build branch, legs through ci-leg.sh) every linux leg runs `bash .github/scripts/ci-leg.sh {leg}`, the same command the host runs, every cargo leg `--locked` except `examples/wasm_hello`; WPT runs upstream-only, the fork runs it on the host; local pre-push gate is `ci-leg.sh fast`. This sets up the rows decision/ci-legs-host-reproducible, decision/cargo-legs-locked and decision/wpt-run-on-host-on-fork that the Taffy move must keep. It is also why the wasm leg's taffy resolution has to be read separately: its lock is outside `--locked`.

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (local baseline re-measured under line-tables-only debuginfo) moved the decision/local-baseline reference to the dev profile with `debug = "line-tables-only"`, workspace default features, `--locked`, no opt-level rewrite, cold/warm defined. The "measured before planned" trial should compare the git-Taffy tree's gates against this baseline form.

@history_item n=3 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `audit` leg (`cargo deny check advisories`), made the `doc` leg `cargo doc --workspace --no-deps --locked` and added the `coverage` leg. Records the trap that `cargo llvm-cov report` without `--workspace` exits 0 with an empty table. These are the audit and doc legs the scope says a git Taffy must still pass.

@history_item n=4 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — (root-manifest citations re-pointed after the line-61 insert) a wrap missed re-pointing root `Cargo.toml` citations after an insert shifted them. Trap for this chunk: editing the root manifest's `taffy` line can move cited `Cargo.toml` lines (e.g. the `[profile.dev]` stanza), so the citation sweep has to cover them.

@history_item n=5 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the stand checks under the bundled DejaVu Sans with system fonts off and no skip path, `stand::{boot, boot_timer}` with `stand::options(incremental)`, and HarnessOptions' `font_ctx`/`incremental`. Origin of decision/stand-checks-bundled-font(-no-skip), decision/stand-pinned-real-font and the contract/stand-boot/* rows, which pin layout-dependent readings that a Taffy layout change can move.

@history_item n=6 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — measured the agent-run five-command contract: `boot` builds every blitz-tests test binary `--locked --no-run`, `run stand` is an empty run when no `stand_*.rs` exists, selections `stand`/`all`/`{name}`, exit grammar. This is the contract/agent-run-verbs/* and agent-run-selection/* rows the chunk's gate runs go through.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — added "Engine features by runner": `cargo test --workspace` builds blitz-dom with features the per-package build lacks. Rule: a green per-package run is not the workspace leg's green (decision/per-package-green-not-workspace-green). The Taffy trial has to read the workspace leg itself, not infer it from per-package runs.

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (CI-scripts 78; `tests/all.rs` declared and never built) added `BlitzTestsTargetsTest`, which pins that the blitz-tests manifest leaves `autotests` on and holds one `[[test]]` `all` with `test = false`. It turns a later sync red if the sync takes upstream's manifest lines. Bears on decision/one-test-target-per-file, blitz-tests-single-test-target and blitz-tests-manifest-difference whenever upstream's "Bump Taffy" commits touch manifests.

@history_item n=9 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (readings on the merged tree) re-counted the workspace after a merge and attributed every moved row to a named upstream file. Also: the workspace build turns on `writing-mode`, which compiles a different body of the bounds reader `physical_unrounded_geometry`, so the same stand check runs over different code under each runner. This is the precedent for re-reading tallies after an upstream take and for decision/bounds-reader-bodies-each-checked.

@history_item n=10 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (test inventory gains upstream's tests) brought blitz-vibey-script's tests/selection.rs and the blitz-dom feature additions. Relevant to decision/vibey-selection-tests-shape-real-text, whose selection tests shape real text and so depend on layout.

@history_item n=11 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — (three engine checks; first conformance reading) each of the bounds reader's two bodies got its own check, and its mutation controls read red only under their own build. The first WPT conformance reading was taken on the fork's dev host (`css/cssom-view` at the pinned commit) as report-only. This is the precedent for decision/fork-wpt-reading-report-only and decision/wpt-conformance-report-only, which apply to any layout move such as a new Taffy.

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (harness and session contracts gain the shot) added `screenshot`/`screenshot_rect`, `ScreenshotError::{FontsUnavailable, EmptyRect}`, painting at the harness's own time over opaque white with no font refused. This is the source of the contract/harness-screenshot/* rows and decision/frames-compared-whole. Those frames are layout-dependent, so a Taffy move can repaint them.

@history_item n=13 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure flakiness budget with the `stand-repeat` leg. It widened the audit, clippy and coverage legs. §8 found that the font test skip came from a fixture and not from missing fonts (decision/font-metric-tests-never-skip). Origin of decision/stand-checks-zero-failure-budget, zero-failure-stand-repeat and clippy-legs, which the Taffy move must keep green.

@history_item n=14 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — re-read the per-OS tallies on CI and named the reading that had been skipped. Rule for later syncs: a wrap that does not re-read the tallies leaves the next wrap to attribute the gap. Applies to this chunk's Taffy-chain move.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness's time became a `VirtualClock`: `pump`, `tick` and `advance_time` move it, and `screenshot` and `settle` never do. `stand_virtual_clock` reads the double click (ratified 2026-10-18). This is the basis of contract/harness-core/pump and the session-lifecycle row decision/two-clicks-without-advance-double-click.

@history_item n=16 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (§2–§10 counts and harness readings) `layout_rect`, `layout_rect_of` and `center_of` read a scrolled box where it stands, where they used to subtract its own scroll offset. This is decision/scrolled-box-reads-where-it-stands, guarded by `scrolled_box_absolute_position`, and a layout reading a Taffy move could disturb.

@history_item n=17 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (test counts; the shot's same-host limit) a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577). So a check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step. This is the source of decision/no-screenshot-in-cross-os-checks and decision/shot-png-deterministic-per-host.

@history_item n=18 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — measured the test-data bootstrap: nothing is seeded, the fixtures compile into each target at `agent-run.sh boot`, and there is no `run fixtures` selection. This is the source of contract/agent-run-selections/all.

@history_item n=19 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — set the a11y leg's membership to fourteen `--test` words: the tier-0 witnesses and five tree-reading checks (decision/a11y-leg-membership). It also records the tally trap: strip ANSI with `(\x1b|\^\[)\[[0-9;]*m`, because the log spells ESC as `^[`.

@history_item n=20 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (served pages' boots and checks) added `stand::boot_cells` and `stand::boot_load`, the latter with `LoadNet` in place of the options' `net_provider`. These are the contract/stand-boot/boot-cells and boot-load rows.

@history_item n=21 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (six served slugs; snapshot line sizes) re-measured the hosted snapshot line sizes (832 · 1,403 · 1,368 · 2,208 B) after a stand change moved them. Precedent: snapshot byte figures depend on layout and bounds, and must be re-measured when layout moves, which a Taffy move does.

@history_item n=22 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (flow-classification file an exception to both layout modes) upstream's `pseudo_element_flow_classification` asserted neither layout mode, and `incremental_oracle` was named as the layout-identity witness. This is the precedent for how decision/layout-tests-in-both-layout-modes and decision/incremental-differential-oracle treat layout code an upstream take brings in.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (per-OS and local tallies re-read after the take) +1 result line and +8 passed, every move attributed to the take by name. This is the template for the tally re-read this chunk owes after the Taffy chain.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (every cargo leg `--locked` but `fmt` too) the exceptions to decision/cargo-legs-locked are `fmt` and `examples/wasm_hello`, which has no root-locked lock. The scope requires `--locked` to stay on every leg, and wasm_hello's taffy resolution to be read on its own.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`pseudo_element_flow_classification` runs both layout modes) ended that upstream exception by looping `for incremental in [false, true]`, with `incremental_oracle` still the witness. Relevant to decision/layout-tests-in-both-layout-modes.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the macOS-only red and the font-embolden local witness) CI went red on `Test (macos)` alone through `apple-font-embolden`. `cargo` stops at the first failing target, so a red job's later targets go unread. The local witness of a paint change under the feature is `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`. A layout move can show up on macOS only.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (stand-repeat leg, seventeen named words and 51 files) `stand-repeat` runs the `stand_*.rs` glob plus a fixed list of named `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`. This is the current form of decision/stand-repeat-three-rounds.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the list leaves unmeasured) the select list paints through `paint_scene` alone and is driven headlessly by the harness, the session and the binary. This is the source of decision/select-list-painted-by-paint-scene.

@history_item n=29 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `embolden` step of `fast`) `fast` became fmt → clippy → test → ci-scripts → embolden, the embolden leg pinned last by `test_fast_leg_ends_on_the_font_embolden_witness`. This is the source of decision/local-pre-push-gate, decision/embolden-leg-local-only and decision/embolden-local-witness: the "fast leg" the trial measures includes it.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (key-press panic retired and its check) a key press on a focused control whose anonymous block was dropped panicked at `position_in_containing_block`. The fix sets `layout_parent` to `None` in `deallocate_anonymous_block`, guarded by `key_press_after_container_lost_its_box` in both layout modes. This is decision/key-after-dropped-anonymous-block, a layout-tree invariant next to Taffy that the move has to keep.

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (upstream's commits with no test here) upstream layout commits (table damage, sub-pixel computed sizes, culling bounds) arrived with no test of their own; the workspace and `incremental_oracle` were their witness. This is the precedent for the Taffy "Bump Taffy" chain, which likely brings layout changes with no fork test.

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the workspace tally and `run all`) per-OS and local tallies were re-read after an upstream take, each move attributed (+2 lines, +63 passed). Another template for this chunk's tally re-read.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (red Dependency audit and its green re-read) the audit went red with `Cargo.lock` and `deny.toml` unchanged, because advisories were published after the base's green audit. It was fixed by a semver-compatible update and an ignore by ID. Relevant to decision/dependency-audit-advisories-and-licenses: a git Taffy has to pass the audit leg, and an audit red can come from outside the move.

@history_item n=34 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (per-OS CI tallies re-read, attributing a tally move) in Actions job logs the stderr `Running …` lines and the stdout `test result:` lines do not interleave in order. So a tally move has to be attributed by grepping the test's own name, together with the ANSI strip. This is the rule for reading this chunk's CI tallies.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (stand checks' shared modules gain the graph's drives) `GRAPH_DRIVES` in `tests/common/mod.rs` drives every stand task's conditional write once, so no stand write reads `unconfirmed`. This is the source of decision/graph-drives-confirm-every-write.

@history_item n=36 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (graph verb held in session and over MCP) introduced the contract/driver-verbs/graph reading, equal to `DioxusDocument::graph` on every served screen in both layout modes. The (Cells screen and graph answer, measured) entry records that the Cells screen measures 53,859 B in 6 parts of `SNAPSHOT_TEXT_BUDGET`. These are layout-sensitive figures that a Taffy move may shift.

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (stand_actions; measured actionable counts) introduced contract/driver-verbs/actions, whose causes depend on layout (`off-screen`, `covered`). It also introduced `ACTIONABLE_AT_BOOT` and the Cells answer (106,088 B, 461 cells `off-screen`). These are layout-derived pins that the Taffy move's stand checks would turn red if geometry shifts.

@history_item n=38 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (5. Integration Test Strategy) `~ decision/layout-tests-in-both-layout-modes` gained the exception: upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions. Applies to any upstream test the Taffy chain brings in.

@history_item n=39 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (9. CI Integration) `+ decision/blitz-dom-requests-through-stamping-constructor`: a crate-level clippy `disallowed-methods` config makes the clippy leg fail on any other request construction. Upstream blitz-dom has no such config, so it is a merge surface for the chain's blitz-side code edits.

@history_item n=40 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — (3. Test Harness Contract) `~ contract/harness-construction/from-vdom`: `from_vdom` first provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context where the root holds none. This is the current promise of a picked row that the stand boots rest on.

@history_item n=41 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — (2. Test Strategy) `+ decision/cross-os-readings-by-answer-digest`: the six served tasks are compared by answer digest (snapshot, lint, actions) against one committed table on every CI OS. `~ decision/stand-checks-bundled-font` got the guard `stand_bundled_font_families`. A Taffy layout change that moves bounds changes those digests, so the committed table would need re-recording, which counts as a behaviour change.

@history_item n=42 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — (5. Integration Test Strategy) `+ decision/start-in-state-reads-as-by-hand-path`: a session started in a named fixture state must read, in both layout modes, as that state reached by hand from boot. A picked row that the move must keep.
