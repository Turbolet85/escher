@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (CI on the build branch, legs through ci-leg.sh, upstream-only WPT) — set the CI shape this sync must keep green: legs run through `ci-leg.sh` with every cargo leg `--locked`, the matrix testing windows and macos and building ios and android, and WPT and its result posting held to `DioxusLabs/blitz` by a repository guard. These are the native-platform-ci-matrix, cargo-legs-locked and wpt-run-on-host-on-fork rows. The merge brings upstream workflow and manifest edits, and the lock move needs a `--locked` workspace build.

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (workspace count, wpt/runner unit test, citation re-point; root-manifest citations after the line-61 insert) — the first upstream sync's test-plan precedent: re-count the workspace tally on the merged tree, add arriving upstream unit tests to the inventory, and re-point `file:line` citations into merged upstream files by the merge's line map. Trap: a wrap that did not re-point root `Cargo.toml` citations past an insert left them stale. This merge edits `Cargo.toml` and `Cargo.lock`.

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (engine features by runner) — added "Engine features by runner". The workspace build compiles blitz-dom with `tracing`, the per-package build does not, so a green per-package run is not the workspace leg's green. A sink capture is counted by target, never in total. These are the per-package-green-not-workspace-green and sink-capture-counted-by-target rows. The merged delta's new engine and shell call sites must be read under both builds, and the sink allowlist re-check depends on this.

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`BlitzTestsTargetsTest`; upstream's tests in the inventory; readings on the merged tree) — created the blitz-tests one-target-per-file rule. Upstream's `tests/all.rs` is declared and never built (`test = false`), and `BlitzTestsTargetsTest` turns a sync red if it takes upstream's two manifest lines. Arriving upstream checks with no test were recorded as absences, not failed gates. Upstream's new `flex_br.rs`, `drag_and_drop.rs` and `all.rs` `mod` lines meet exactly this guard (rows one-test-target-per-behaviour, blitz-tests-one-target-per-file, blitz-tests-single-test-target, blitz-tests-manifest-difference). This sync also found that the workspace and per-package builds resolve different blitz-dom features (`writing-mode`).

@history_item n=5 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (the two pinned engine readings read fixed) — fixed the hit walk so that a raw `click` on an element outside the padding box of a box that clips it no longer lands (contract harness-input/clipped-click). It also fixed a scrolled box's bounds. Trap: a check that leaned on the old hit holds none of the words a sweep searches for, so it is found only by running the stand gates. Upstream `f65baeca5` changes one-axis `overflow: clip` paint, so the hit and paint for clipped boxes must be re-measured.

@history_item n=6 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (three engine checks, the re-measured counts, the first conformance reading) — each of the two bounds-reader bodies chosen by `writing-mode` gained its own check, because a fix in one body reads green under the other build (row bounds-reader-bodies-each-checked). The Taffy 0.16 move edits `blitz-dom/src/layout/*` and `stylo_taffy/convert.rs`, so layout and bounds checks must stay green under both builds.

@history_item n=7 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the merge's tests: five upstream targets, the bridge's six unit tests, tallies) — precedent for counting a sync's upstream checks. Five upstream checks, `pointer_leave` among them, each became its own auto-discovered target, with `all.rs` gaining `mod` lines and staying unbuilt. It also logged the bridge's and CI scripts' count moves and re-read the workspace baseline on the merged tree. `22c18e38f` extends `pointer_leave.rs`, so this target's tests move.

@history_item n=8 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the floors and the flakiness budget) — set the per-crate line-coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the stand checks' zero-failure three-round budget (rows per-crate-coverage-floors, stand-checks-zero-failure-budget, zero-failure-stand-repeat). The take adds dioxus-native-dom code, including the dropped-file reader, so the floors must still hold after the merge.

@history_item n=9 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the two merged ARIA checks; tallies; `keyboard_compat.rs` exclusion; `NamedKey` and test-that) — a sync precedent. Merged upstream checks counted as their own targets, `all.rs` `mod` lines, per-OS tallies re-read with every move explained by name, and the coverage leg excluding `keyboard_compat.rs` by filename (row keyboard-compat-excluded-from-coverage). The keyboard-types 0.8 move re-exported `NamedKey`, and merged checks assert with test-that's `verify_that!`. That is relevant to the drag event's `convert_modifiers` adaptation under the Dioxus hold and to the keyboard_compat re-read CARRY.

@history_item n=10 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** (upstream's cursor test counted, tallies re-read) — rule for later syncs: a wrap that does not re-read the tallies leaves the next sync to attribute the gap. The tallies move from the recorded reading straight to the sync's CI run, with any skipped reading named. This sync's verdict must re-read the per-OS tallies.

@history_item n=11 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** (upstream's two focus targets, the directory, the tallies) — upstream targets taken in a sync run in the workspace `test` leg and `run all`, not in the `a11y` leg or `run stand`. Per-OS tallies were attributed per target against the previous run. This is the pattern for `flex_br` and `drag_and_drop`.

@history_item n=12 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`hit_test_dropped_nodes`; the gaps the take leaves; the `keyboard_compat.rs` exclusion re-read) — an upstream check entered as its own target and was read red with its engine hunks reversed. Untested arrivals were stated as gaps. The `keyboard_compat.rs` exclusion lifts only when escher takes upstream's retirement of it (`286874cc1`, Dioxus 0.8), so it stands under this sync's Dioxus hold.

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`pseudo_element_flow_classification` legs; tallies re-read; both-layout-modes exception; script `file:` tests; `keyboard_compat` re-read; untested `:open`; windowed redraw route; upstream test absent; citations) — the fullest sync precedent:
- a new upstream target's legs are recorded (workspace `test` and `run all`; not `stand-repeat`, `run stand` or `a11y`);
- per-OS tallies are re-read with moves attributed by name;
- an upstream file running one layout mode is recorded as an exception to the both-modes rule;
- upstream behaviour with no test is recorded as a gap, and a windowed shell route with no headless check is stated;
- citations past the take's lines are re-pointed.

The drag-and-drop input reaches only windowed documents and the shell's winit drop source, so the same gap-recording pattern applies.

@history_item n=14 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script tests the take added) — recorded the `file:`-load narrowing (`document_may_load`, a `FetchError` for a non-`file:` document) with its tests. This is the origin of the file-urls-only-for-file-documents row. The drop input reads a dropped file through `std::fs` with no path restriction, and that boundary meets this rule.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`pseudo_element_flow_classification` runs both layout modes; every cargo leg `--locked` but `fmt`) — ended the upstream file's single-mode exception by making it loop both modes, with `incremental_oracle` still the layout-identity witness. It also corrected the locked-leg rule: every cargo leg is `--locked` except `fmt` and `examples/wasm_hello` (row cargo-legs-locked). This is relevant to how `flex_br` and `drag_and_drop` treat layout modes and to the lock move's `--locked` build.

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the event-loop stand-in and the redraw route's check) — blitz-shell's first unit test (`CountedWakes`, an inline `cfg(test)` module in `event.rs`) closed the gap where the windowed shell's route had no check. Upstream's `dnd.rs` lands in blitz-shell with a winit drop source and a `url` dependency, so its coverage is read against this precedent.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the macOS-only red and `blitz-paint/font-embolden`'s local witness) — a paint change under `font-embolden` went red on macOS alone. That led to the local `select_popup_painted` witness under the feature. `f65baeca5` changes paint in `blitz-paint/src/render.rs`, so the embolden witness applies.

@history_item n=18 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait) — a sync trap: upstream's failed-stylesheet fix stopped at the harness's `CountedHandler` because `NetHandler::error`'s do-nothing default compiled silently through the wrapper, and no gate read it. Rule: a take that extends a trait sweeps that trait's implementers. This take adds a `DataTransfer` model to blitz-traits and drag events to the driver, so every escher implementer of an extended trait must be swept.

@history_item n=19 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — made `embolden` the last step of the `fast` leg, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (rows embolden-leg-local-only, embolden-local-witness, local-pre-push-gate). This sync's paint change is checked locally by it.

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (tallies and `run all`; `keyboard_compat.rs` through this take; blending and the inline box's paint skip read on no frame; upstream commits with no test here; blitz-dom and blitz-net unit tests from the take; citations) — another full sync precedent. Moves are attributed on all four test jobs, the `keyboard_compat` exclusion stands while `286874cc1` stays out, and upstream paint behaviour with no frame test is recorded as a gap with an owner. That applies to `f65baeca5`'s one-axis clip paint. The left-out mounted-event test is named.

@history_item n=21 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the red Dependency audit and its green re-read) — the audit went red on an unchanged `Cargo.lock` because of newly published advisories. It was fixed by a semver-compatible update and an ignore by ID. This chunk's audit leg must read green with the moved lock (`taffy` 0.16.0 from the registry, `url`).

@history_item n=22 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the fix's witness on Dioxus 0.7; the hazard closed by the buffered render; upstream's mounted-event test stays out; residual owners) — origin of the rows buffered-render-on-dioxus-07-bridge, buffered-render-unborrowed-document, upstream-buffered-render-test-kept-as-written and upstream-mounted-flush-test-left-out:
- upstream `286874cc1`'s buffered render was ported onto Dioxus 0.7;
- `task_polled_during_render_can_access_document` is kept as upstream wrote it;
- `element_removed_before_its_mounted_event_is_flushed` stays out until the Dioxus 0.8 move;
- the `keyboard_compat` exclusion's re-read is owned by the CARRY on the sync entries.

The `events.rs` conflict and the Dioxus hold must keep these as they are.

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit scope and its one test-only shim) — the bridge tests gained one `cfg(test)` shim, `DioxusDocument::dispatch_to_vdom`. No function under test may gain a constructor or `pub` for them (row bridge-tests-without-widening). The take's dioxus-native-dom drag events and file reader land beside the bridge, so tests of them must not widen visibility.

@history_item n=24 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (4. Unit Test Strategy: `ci-actions-pinned-to-sha` and `actions-pinned-to-commit-sha`) — widened the pin check from the CI workflow to every workflow, under the guard `WorkflowPinTest`. Any workflow file the merge touches must keep every `uses:` pinned to a 40-hex SHA.

@history_item n=25 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Integration Test Strategy: `layout-tests-in-both-layout-modes`) — added the named exception: upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions. This applies directly to `flex_br.rs` and `drag_and_drop.rs`.

@history_item n=26 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (`file-urls-only-for-file-documents`; `blitz-dom-requests-through-stamping-constructor`; `real-provider-inside-stand-in`) — widened the `file:` refusal to engine sub-resource loads through blitz-net, added the clippy disallowed-methods gate on blitz-dom's request construction, and allowed blitz-net's real provider inside a hand-written stand-in. The take's drag data and dropped-file reads over `std::fs` are a new local-file path these rows bear on, and the clippy leg still guards blitz-dom's requests.

@history_item n=27 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** (`file-urls-only-for-file-documents`) — the rule gained the browser app's silent `file:` favicon and link refusal, with user loads not gated. The dropped-file reader is a user-initiated local read that this rule's boundary has to place.

@history_item n=28 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (`cross-os-readings-by-answer-digest`; `stand-checks-bundled-font`) — added the cross-OS reading of the served tasks by answer digest against one committed table, and the bundled-font guard `stand_bundled_font_families`. This sync's screenshot and cross-OS readings must stay equal; a moved reading is re-measured and recorded, never loosened.

@history_item n=29 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (5. Integration Test Strategy: `stated-tolerance-read-inclusively`, `session-layout/changes-nothing`, `layout-reason-words-each-read`) — the layout reader's closed words are each held to a fixture in both modes, and a stated 1/64 px tolerance is read inclusively. The take's `CalcHandle` adaptation in `layout_reason.rs` under Taffy 0.16 makes the layout reason and `stand_layout` a surface of this sync.

@history_item n=30 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (5. Integration Test Strategy: `session-run/layout-answer`) — the `layout` verb's answer must equal the in-process layout read on every node of the served screens in both modes. A Taffy-driven change to the reason words or numbers would move this contract.

@history_item n=31 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (`file-urls-only-for-file-documents`; `free-disk-on-test-and-stand-repeat`) — the previous sync's precedent. A take's new engine sub-resource fetch (SVG image and feImage href) widened the `file:` rule, ratified as measured. The free-disk step joined the test job. A boundary the take carries goes to the founder with its rule row amended.

@history_item n=32 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (2. Test Strategy: `cross-os-readings-by-answer-digest`; `count-and-rename-sweeps`) — the cross-OS digest table now reads seven served tasks. A count-worded test name pinned as a witness is kept by a count sweep. If the merge moves any served screen's snapshot, lint or actions digest, the committed table must be re-measured.

@history_item n=33 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** (9. CI Integration: `a11y-leg-membership`) — the a11y leg runs the files that each witness SC 2.1.1 or 2.4.3, a tree reading or a recorded gap; the leg script holds the list. The chunk must keep the a11y leg green, and a new upstream file joins it only by that criterion.

@history_item n=34 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (3. Test Harness Contract: `rust-tests-on-built-in-harness`) — the last sync widened the exception to cover every merged upstream check that returns `TestResult<()>` with `verify_that!`. Upstream's new `flex_br` and `drag_and_drop` tests may use that form.

@history_item n=35 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (2. Test Strategy: `count-and-rename-sweeps`) — a change of a CI leg's file count sweeps the CI-script tests for literal pins of that count. If an upstream file joins a named leg, its pinned counts in `test_ci_workflows.py` and the capability statement must move with it.

@history_item n=36 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (`cross-os-checks-hold-no-shot`, `no-shot-in-cross-os-byte-checks`, `no-screenshot-in-cross-os-checks`, `no-screenshot-step-in-committed-checks`, `canvas-shots-cross-os-by-digest`) — the ratified requirement of byte equality. A canvas-only shot is compared across hosts by answer digest, and a shot holding document text is byte-identical on one host only. This sync's paint change must not move the canvas digests, which the WATCH `cry-womrv2l4` observes.

@history_item n=37 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** (`canvas-text-on-whole-device-pixels`; `canvas-shots-cross-os-by-digest`; `stand-boot/boot-circle-drawer`) — added Circle Drawer's canvas element shot to the cross-OS `SAMPLE` table, held equal only by snapping glyph origins to whole device pixels. It also added the `boot_circle_drawer` stand boot. Paint edits from the merge must keep `stand_canvas_cross_os` byte-equal on all three hosts.

@history_item n=38 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** (`canvas-text-on-whole-device-pixels` ratified) — the row is now standing on the founder's word, with the engine-side remedy left as residual `res-ieayznir`. This chunk extends the cross-OS canvas readings and must not loosen them.
