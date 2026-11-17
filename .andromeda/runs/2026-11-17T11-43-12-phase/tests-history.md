@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — CI on PRs and main/v0.*/build/**, every linux leg through `ci-leg.sh`, the fast/slow split (fmt, clippy, test-features-default, ci-scripts gate the rest), the matrix testing windows/macos and only building ios/android, WPT and its posting upstream-only behind a repository guard. These are the rows a sync must keep green (decision/ci-trigger-branches, decision/ci-fast-slow-split, decision/ci-legs-host-reproducible, decision/native-platform-ci-matrix, decision/wpt-upstream-only, decision/upstream-repository-guard). Upstream's WPT-only commits (`e8b370a25`, `585c1869c`) land against that guard.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the audit, a11y and coverage legs. Trap: `cargo llvm-cov report` without `--workspace` exits 0 with an empty table. This is the base of the audit leg this sync must keep green, and of the coverage floors (decision/per-crate-coverage-floors).

@history_item n=3 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first upstream-sync precedent: the workspace re-counted after the merge with each +N attributed to a named upstream test, and the `file:line` citations into merged upstream files re-pointed by the merge's line map with no claim text changed. A sibling entry fixed root-manifest citations a wrap had left stale. This is the shape this take's citation sweep and tally re-count follow.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` (a sink-installing host at `trace`, both streams drained by threads, `attach` never `start`) and `telemetry_drop` (third-party target dropped under a naming RUST_LOG). Trap: a check that must fail red on a leaking sink cannot use `start` on a piped host. This is the standing capture the scope's "sink allowlist re-checked" re-reads at 0 needles (decision/sink-drops-third-party-targets, decision/drain-host-pipes-while-running, decision/host-stderr-no-screen-content).

@history_item n=5 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — "Engine features by runner": `cargo test --workspace` compiles blitz-dom's `tracing` call sites in, while `-p blitz-tests` and `-p seven_guis` builds do not. Rule: count a sink capture by target, never in total; a per-package green is not the workspace leg's green (decision/sink-capture-counted-by-target, decision/per-package-green-not-workspace-green). A take that adds engine `tracing` sites must be re-read under both builds.

@history_item n=6 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** (the install invariant) — `test_o_package_installs_are_bounded`: no step runs `apt-get` directly, installs go through `apt-install.sh`, and the one action step carries `timeout-minutes`. `InstallScriptTest` added. Any CI workflow edit for the disk reds (a free-disk step on `Test [default features]`) must stay inside this invariant (decision/bounded-package-installs, decision/ci-package-installs-bounded).

@history_item n=7 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`BlitzTestsTargetsTest`) — the fork keeps one blitz-tests target per file: `autotests` stays on, and upstream's single `all` target is carried at `test = false`, a two-line difference pinned by `test_blitz_tests_targets.py`. That test needs Python 3.11+ (`tomllib`). A take of upstream's manifest lines turns it red. Upstream's new test files enter as their own targets, with `mod` lines added to the unbuilt `all.rs` (decision/blitz-tests-one-target-per-file, decision/one-test-target-per-file, decision/blitz-tests-single-test-target, decision/blitz-tests-manifest-difference).

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (the inventory and the readings on the merged tree) — the precedent for recording upstream's arrived tests by name and count, with arrivals that bring no test recorded as absences, not failed gates. Every moved workspace count is attributed to a named upstream file. A second engine-feature split by runner, `writing-mode` (workspace only), selects which body of the bounds reader is compiled. Per-OS tallies were left unattributed, owned by a route entry. This take's re-read of the tallies must close that kind of gap.

@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the scrolled-box bounds reader and the hit walk fixed in the engine. Three engine checks, green under both builds because each build compiles a different reader body: `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box`, `scrolled_box_absolute_position`. Upstream's `e5305a4c1` (scroll offsets clamped at layout rounding) touches exactly this surface, so these checks and decision/bounds-reader-bodies-each-checked are what a resolution must hold.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the harness paints shots through the CPU image renderer, composited over white. Frames are compared whole, byte-identical across runs, and a no-font shot is refused, never skipped. The harness takes blitz-paint without default features. The AnyRender 0.15 move, canvas backgrounds and shear direction may move these pixels, and a moved reading is re-measured, never loosened (decision/headless-screenshot, decision/frames-compared-whole, decision/shot-refused-without-fonts, contract/harness-screenshot/*).

@history_item n=11 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — a merge took five upstream targets, each its own target, with `all.rs` gaining `mod` lines and staying unbuilt. It also took the bridge's six unit tests, bumped the CI-scripts and workspace tallies, and noted that the bridge clears by upstream's 28-name `is_bool_attr`. This is the precedent for counting upstream's new targets and unit tests at a take.

@history_item n=12 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the per-crate coverage floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90), the zero-failure stand-repeat budget (three rounds, no retry), and the first per-OS test tallies record. These are the gates the take re-reads (decision/per-crate-coverage-floors, decision/zero-failure-stand-repeat, decision/stand-checks-zero-failure-budget, decision/stand-repeat-three-rounds).

@history_item n=13 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA checks; the tallies after the merge) — upstream's checks were counted as their own targets, run by the `test` leg and not the `a11y` leg. The measured directory count corrected a stale figure. The per-OS tallies were re-read on the final HEAD's green run, every moved count explained by name. This is the method the scope's "per-OS tallies re-read" follows.

@history_item n=14 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the `keyboard_compat.rs` exclusion; `NamedKey` and test-that) — the coverage leg excludes upstream's temporary `keyboard_compat.rs` by filename until escher takes upstream's retirement of it (`286874cc1`, the Dioxus 0.8 move, still out of scope here). That sync also took keyboard-types 0.8 and test-that 0.6, reviewed by hand. The decision/per-crate-coverage-floors except clause stands while `286874cc1` stays left out.

@history_item n=15 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the per-OS tallies were moved straight to the sync's green run, with the skipped earlier reading named. Standing rule for later syncs: a wrap that does not re-read the tallies leaves the next one to attribute the gap.

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — one virtual clock drives engine time: the scrollbar fade, double-click timing and smooth scroll read the document's clock. `advance` moves it by exactly `ms`, and `screenshot`, `text_contrast` and `settle` never move it. This is the clock invariant the scope requires every conflict resolution to keep, guarded by `virtual_clock_engine` and `stand_virtual_clock` (decision/one-virtual-clock, decision/harness-virtual-clock, decision/scrollbar-fade-on-document-clock).

@history_item n=17 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the shot's same-host limit) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a committed check holds no screenshot step and byte-identity is asserted on one host only. Paint-moving upstream commits (AnyRender 0.15, backgrounds, shear) are re-read per host under this limit (decision/no-screenshot-step-in-committed-checks, decision/shot-png-deterministic-per-host).

@history_item n=18 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — upstream's `focus_within` and `focus_redraw` were taken, each its own target, run by the `test` leg and `run all` but not by `a11y` or `run stand`. The per-OS tallies were re-read and the earlier chunks' unwritten moves attributed per target from the job logs. A report's claim was disproved at Validate.

@history_item n=19 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — upstream's `hit_test_dropped_nodes` came in as its own target, read red with its engine hunks reversed. Stated gaps were left with no check (`PARALLEL_TRAVERSAL_LOCK`, out-of-flow helpers). A stale directory count was corrected. The `keyboard_compat.rs` exclusion was re-read: it lifts only with `286874cc1`, and its owner is the left-out-delta line. This is the precedent for this take's left-out table and its re-read reasons.

@history_item n=20 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` runs the `stand_*.rs` glob plus a fixed list of named `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`. Upstream targets taken in a sync are in neither list unless added, which bears on where new upstream files run (decision/stand-repeat-three-rounds).

@history_item n=21 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg names its fourteen `--test` words (tier-0 witnesses plus five tree-reading checks), pinned by one `assertIn` per word. Per-job tallies strip ANSI written both as ESC and as the literal `^[`. This is the a11y leg the scope re-reads green, and decision/ci-log-ansi-strip for the per-OS re-read (decision/a11y-leg-membership).

@history_item n=22 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host complexity ceiling) — a ceiling of 15, read on the dev host only, test functions counted, no CI leg, with engine functions over it left as they are. The except clause on decision/complexity-ceiling-host-only came later (n=48) and is why an upstream edit to a `handle_click` arm needs a hand re-apply.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `pseudo_element_flow_classification` target and its legs; the both-layout-modes exception) — the new upstream target runs only in the workspace `test` leg and `run all`, not in stand-repeat, `run stand` or a11y. It first stood as a named exception to the both-layout-modes rule, with `incremental_oracle` as the layout-identity witness. This is the precedent for placing new upstream targets and their layout-mode exceptions (decision/layout-tests-in-both-layout-modes, decision/incremental-differential-oracle).

@history_item n=24 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (tallies, script tests, browser tab, citations, agent-run Proof) — a take's per-OS and local workspace tallies were re-read, each move named (the new target's line plus tests added in existing lines). Script tests for `file:`-load narrowing were added, including a test-only `TMPDIR` read for files on disk. The upstream browser app's `tab.rs` test was counted. Stretched citation ranges were re-pointed. `run all` was re-counted with no script change. Together these are the full checklist of a take's test-plan touch points.

@history_item n=25 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (gaps: `:open` untested, IME test, an upstream test absent, the redraw route, vibey-script's inline-test absence, the `keyboard_compat` owner) — engine behaviours a take brought with no test were recorded as gaps, not fixed. `task_polled_during_render_can_access_document` was left out with `286874cc1`. The precedent for recording untested arrivals such as root SVG background, canvas-only backgrounds, shear direction and SVG external images.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the free-disk step of `Stand checks repeated`, and the red before it) — CI#41190771843 at `42eed99a6` went red on `Stand checks repeated` with linker bus errors and `Free space left: 5 MB`. A pinned `jlumbroso/free-disk-space` step was added after the toolchain and before rust-cache. Its cure is unproven: that runner had 86 GB free before the step, the failed run has no `df` line, and the job stood alone on the older image `20261018.96.1`. This is the measured base of the operator's disk-red hypothesis that this chunk must give a line to.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand-repeat` fifteen targets; `--locked` but `fmt`; `pseudo_element_flow_classification` both modes; tallies) — the named stand-repeat list grew. Every cargo leg is `--locked` except `fmt` and `examples/wasm_hello`. The upstream flow-classification file was edited to run both layout modes, ending its exception. The per-OS tallies were re-read at the final HEAD. These are the leg lists and counts this take re-reads.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (engine behaviours the take must keep: `visible_region` through transforms, non-invertible transform hit, `:open` via `open_state`, engine-only `change`, the chosen-row colour pair, `inert` focus fixup, native `toggled`) — escher-owned engine behaviours with their checks. A take touching scrolling, transforms or paint must keep them (decision/open-pseudo-class-follows-attribute, decision/change-event-from-engine-only, decision/chosen-select-row-colours).

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the macOS-only red and `font-embolden`'s local witness; a red job's later targets) — `Test (macos)` went red alone through `apple-font-embolden`, with no local gate building the feature. Also, `cargo` stops at the first failing target, so a red job's later targets go unread. This matters because upstream's shear and paint changes (`d02c094c7`) can move paint under the embolden feature (decision/embolden-local-witness, decision/embolden-leg-local-only).

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the per-OS and stand-repeat tallies; the key-press panic) — tallies re-read with every move named, the free-disk record of CI#41192433140 kept as that run's. A key-press panic past a dropped anonymous block was found and its owner placed on the next upstream sync. It was retired there (n=33), which is why decision/key-after-dropped-anonymous-block stands.

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait) — upstream's failed-stylesheet fix stopped at the harness's `CountedHandler`, because `NetHandler::error`'s do-nothing default compiled silently through the wrapper, so sessions read `NotSettled(Loads)` for good. Trap rule: a take that extends a trait sweeps that trait's implementers. `7b8712cba`'s new SVG image loads go through the same counter (decision/failed-load-ends-settle-wait, contract/harness-settle/failed-load, decision/network-stand-ins-counted).

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`; the matrix job's free-disk step gone) — `fast` ends on the embolden leg, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. `matrix_test` has no free-disk step, and `test_only_stand_repeat_frees_disk` pins that only stand-repeat frees disk. Adding a free-disk step to `Test [default features]` would turn this pin red, so it has to be amended in step (decision/only-stand-repeat-frees-disk, decision/local-pre-push-gate).

@history_item n=33 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — `deallocate_anonymous_block` clears each child's `layout_parent`. `key_press_after_container_lost_its_box` runs in both modes in the `test` leg only, and `select_popup` presses in every row. This fix is part of the merge surface (`cry-blhcfqeb`) that this take re-reads row by row (decision/key-after-dropped-anonymous-block).

@history_item n=34 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (tallies and `run all`; citations re-pointed; CI scripts' count; coverage readings; the `keyboard_compat` exclusion) — the most recent take before this one. Per-OS tallies were re-read with +2 lines and +63 passed attributed, citations re-pointed, `ci-scripts` 110 tests, and coverage re-read (two crates inferred from counts, not the lcov). The take left `286874cc1` out (36 of 57 taken), so the exclusion stands. This is the template for this take's wrap.

@history_item n=35 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (upstream tests taken: `url_search_params`, `render_blocking_stylesheet`, blitz-net's unit tests with test-only loopback binds, blitz-dom's restyle tests) — upstream's arriving tests were each recorded with their fixtures, and the blitz-net "no test" claim was retired. This is the precedent for recording this take's tests.

@history_item n=36 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (blending and the inline box's paint skip read on no frame; upstream commits with no test; `text_contrast` past a blend) — paint behaviours a take brought (`mix-blend-mode`, the inline box skip) were recorded as no-frame gaps with an owner. Commits with no test have the workspace and `incremental_oracle` as their witness. `text_contrast` reads `None` past a blend. This take's root SVG background, canvas-only background propagation and shear commits fall in the same class (contract/harness-contrast/blend-mode).

@history_item n=37 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the red Dependency audit and its green re-read) — the audit went red on an unchanged `Cargo.lock` when new advisories appeared (RUSTSEC-2026-0433, -0429). The fix was a semver-compatible update plus an ignore by ID, with the mechanism pointed to security-plan. The scope requires the audit green after a take that moves AnyRender to the registry and changes the dependency graph (decision/dependency-audit-advisories-and-licenses).

@history_item n=38 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard; listeners and a held signal's readers) — `dioxus_runtime_probe`, 10 tests, reads public Dioxus 0.7.10 API alone. A bridge listener attribute outlives its listener, so listeners are read from the vdom walk. Dioxus stays pinned at 0.7.3 in this take, and these pins must hold (decision/listeners-read-from-vdom-walk, decision/one-actionable-rule).

@history_item n=39 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — upstream `286874cc1`'s buffered render was ported onto the 0.7 bridge. The probe's two `…_spawned_acts_*` pins are the fix's discriminating witness, and upstream's `task_polled_during_render_can_access_document` is kept as written but is insensitive on 0.7. `element_removed_before_its_mounted_event_is_flushed` stays out until the Dioxus 0.8 move. This is the merge surface carried as `cry-mnnoitsu` and `cry-o26emj4y` (decision/buffered-render-on-dioxus-07-bridge, decision/buffered-render-unborrowed-document, decision/upstream-buffered-render-test-kept-as-written, decision/upstream-mounted-flush-test-left-out).

@history_item n=40 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the per-OS CI tallies re-read, and attributing a tally move) — in a GitHub Actions job log the stderr `Running` lines and stdout `test result:` lines do not interleave in order. A tally move is therefore attributed by grepping the test's own name; attributing by position once gave 246 passed to `url_search_params`. This is the method rule for this chunk's tally re-read (decision/tally-move-attributed-by-test-name).

@history_item n=41 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (task state through the fork's registry; the unconfirmed flag's limit) — the stand registers signals through `use_graph_state` with no dioxus-core change. A declared write left unrun after its event reads `unconfirmed`, and `GRAPH_DRIVES` drives each conditional write once. These are rows of dioxus-native-dom behaviour a take must keep (decision/graph-signal-registration, decision/unrun-declared-write-reads-unconfirmed, decision/graph-drives-confirm-every-write).

@history_item n=42 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows: decisions, contracts, absences and overviews, each verified against the code. Every later amendment names what it changed by row handle (`+`/`~`/`−`). This take's test-plan changes are written as row amendments in this form.

@history_item n=43 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (`ci-actions-pinned-to-sha` widened; guard moved to `WorkflowPinTest`) — the SHA pin check now reads every workflow, and the pin and toolchain assertions moved out of `CiWorkflowTest` into `WorkflowPinTest`. Any new action step (a free-disk step on `Test [default features]`) is held by `WorkflowPinTest` (decision/ci-actions-pinned-to-sha, decision/actions-pinned-to-commit-sha).

@history_item n=44 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (`layout-tests-in-both-layout-modes` except) — upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions; six of seven did at that take. This is the rule this take applies to upstream's new and edited test files (decision/layout-tests-in-both-layout-modes).

@history_item n=45 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — the `file:` rule was extended to blitz-net sub-resource loads, guarded by `local_files_for_local_documents`. The clippy leg enforces blitz-dom's one stamping request constructor. A real blitz-net provider may sit inside a hand-written stand-in. `7b8712cba`'s external SVG images are a new sub-resource load that must go through `stamped_request` and the `file:` refusal (decision/file-urls-only-for-file-documents, decision/blitz-dom-requests-through-stamping-constructor, decision/real-provider-inside-stand-in).

@history_item n=46 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — the `file:` rule was widened again: the browser app drops a `file:` favicon or link navigation silently, and user loads are not gated. This is the current rule text a new SVG image load from a remote page must satisfy (decision/file-urls-only-for-file-documents).

@history_item n=47 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — added the cross-OS readings of the six served tasks by answer digest (snapshot parts, lint, actions) against one committed table, guarded by `stand_cross_os_readings`, and gave the bundled-font stand decision the guard `stand_bundled_font_families`. Text and paint changes such as italic and oblique shear can move these digests and readings on any CI OS (decision/cross-os-readings-by-answer-digest, decision/stand-checks-bundled-font).

@history_item n=48 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — `~ decision/complexity-ceiling-host-only` except: the engine's click handler is held below the ceiling, upstream's arms included, at the price of a hand re-apply when an upstream sync edits one of its arms. The `handle_click` arm-functions carry (`cry-hvl3d4gp`) asks P3 to read whether a taken commit edits `events/pointer.rs`. It also added `contract/harness-inspect/focused`.

@history_item n=49 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `+ decision/engine-items-asserted-on-tracing-build` (engine caused items asserted only on a tracing build) and `+ decision/expected-value-pinned-as-literal`. A take that adds engine `tracing` call sites moves what the tracing build reads, and moved numbers are pinned as measured literals.

@history_item n=50 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `+ decision/doc-leg-gates-public-doc-edits`: the fast leg runs no rustdoc, so a chunk that edits a public doc comment lists the doc leg in its gate block. The scope names the `doc` leg among its proofs. The same entry widened `decision/stand-boots-offline` and `decision/stand-load-page-fixed-answer` to the FailingNet page.

@history_item n=51 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `+ decision/stated-tolerance-read-inclusively`, `+ contract/session-layout/changes-nothing`, `+ decision/layout-reason-words-each-read`. Layout numbers that the scroll-clamp commit (`e5305a4c1`) or paint changes move are re-measured against these readings, never loosened.

@history_item n=52 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/session-run/layout-answer` (`layout` equal to the in-process read on every node of the six screens in both modes). `~ contract/session-layout/changes-nothing` now covers the verb too. `+ decision/count-and-rename-sweeps` came from a missed numeric pin and a missed renamed witness. This is the latest layout surface a take's layout changes must keep green, and the sweep convention for any count moved by taken tests.

@history_item n=53 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — `+ contract/stand-boot/boot-alone`, `+ decision/pinned-viewport-read-from-document`, `+ decision/harness-graph-tests-enable-accessibility`, `+ decision/timer-context-read-by-delivered-ticks`, plus the component-alone harness constructor rows. This is the immediately preceding chunk: its boot, viewport and per-package accessibility-feature rows are on the tree being synced.

@history_item n=54 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — acting steps wait out a load in flight up to `LOAD_WAIT` (5 s wall), settling every 10 ms and moving no virtual time, guarded by `stand_load_wait`. External SVG image loads (`7b8712cba`) add loads that settle and the driver wait must count and end on (decision/load-wait-bound, contract/driver-session/load-wait).

@history_item n=55 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle beside pump: a quiet pass, `NotSettled { busy: Loads }` at once with a load in flight, the `SETTLE_PASS_LIMIT` of 64, and no clock or time moved. Only a fetching `net_provider` is wrapped in the load counter. This is the settle contract that new upstream loads and layout-rounding changes must keep (contract/harness-settle/*, decision/harness-counts-fetching-providers-only).

@history_item n=56 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§1 coverage) — the password caret placed by the bullets, `contain: paint` stopping the hit, a scrolled box's absolute position where it stands, and a transformed box's client rect, later ratified by the founder. These engine readings live on the scrolling and paint surface `e5305a4c1` touches (decision/password-caret-placed-by-mask, decision/scrolled-box-reads-where-it-stands).
