@history_item n=1 marker=amd-dhibncze
**amd-dhibncze** — CI moved to the build branch: each linux leg through `.github/scripts/ci-leg.sh`, the matrix testing windows/macos and building ios/android, WPT and its result posting upstream-only behind a repository guard, the fork running WPT on the host. This is where `decision/ci-legs-host-reproducible`, `decision/native-platform-ci-matrix`, `decision/wpt-run-on-host-on-fork` and `decision/ci-trigger-branches` come from. A sync that touches upstream's `wpt.yml`/`ci.yml` must keep the guard and the leg shape.

@history_item n=2 marker=amd-uy5wnu36
**amd-uy5wnu36** — Added the `audit`, `a11y` and `coverage` legs. Trap: `cargo llvm-cov report` without `--workspace` exits 0 with an empty table. This chunk's acceptance re-reads the audit and a11y legs green.

@history_item n=3 marker=amd-sotcjgq6
**amd-sotcjgq6** — The first upstream sync's precedent: re-count the workspace on the merged tree, list upstream's new unit tests by file, and re-point citations into merged files by a measured line map. This chunk is the eleventh take and follows the same pattern.

@history_item n=4 marker=amd-s53lzeiq
**amd-s53lzeiq** — "Engine features by runner": the workspace build compiles blitz-dom's `tracing` call sites, the per-package build does not. Gave rise to `decision/per-package-green-not-workspace-green` and `decision/sink-capture-counted-by-target`. The sink allowlist re-check must count by target and read both builds.

@history_item n=5 marker=amd-s4wow4l3
**amd-s4wow4l3** — `host_log` and `telemetry_drop` cover a host that installs the sink, at `trace`, draining both streams. Origin of `decision/host-stderr-no-screen-content`, `decision/sink-drops-third-party-targets` and `decision/drain-host-pipes-while-running`. The standing capture checks that this chunk's sink re-check holds at 0 needles.

@history_item n=6 marker=amd-lrluctc2
**amd-lrluctc2** — Driver session traps: a sink-installing host at `trace` overruns an undrained pipe; an `#[ignore]` host child blocks a bare `--ignored` run. Background to `decision/sink-host-output-drained` and to `decision/log-silence-read-on-a-sink-host`.

@history_item n=7 marker=amd-u7jdserc
**amd-u7jdserc** — Standing trap for sink checks: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. Applies if the taken delta adds new `tracing` call sites.

@history_item n=8 marker=amd-evt52mzu
**amd-evt52mzu** — The stand log's second record class (closed spans) was ratified by the founder. Basis of `decision/closed-spans-logged-as-lines` and `contract/stand-log/span-line`, which the sink re-check must keep.

@history_item n=9 marker=amd-u7vpyh4t
**amd-u7vpyh4t** — An upstream sync introduced `BlitzTestsTargetsTest`, which pins the blitz-tests manifest at `autotests` on and one `[[test]]` table, `all`, with `test = false`. The fork keeps one target per file. Origin of `decision/blitz-tests-single-test-target`, `decision/one-test-target-per-file` and `decision/blitz-tests-manifest-difference`. A take of upstream's manifest lines turns it red. Upstream's new test files enter `all.rs` as `mod` lines and stay unbuilt.

@history_item n=10 marker=amd-u6pbkyf3
**amd-u6pbkyf3** — Precedent for upstream test arrivals: each new upstream test is counted in the inventory, and an untested arrival is recorded as an absence, not a failed gate (the plan is tier 0). The same treatment applies to this take's SVG, `hr` and `@font-face` arrivals.

@history_item n=11 marker=amd-cqdq6a7d
**amd-cqdq6a7d** — Readings on a merged tree: each moved count is attributed to a named upstream file. The `writing-mode` feature differs by runner and selects the bounds reader's body. The per-OS tallies were left unattributed and handed to a route entry. Precedent for this chunk's "per-OS tallies re-read".

@history_item n=12 marker=amd-pzdy3myb
**amd-pzdy3myb** — Each of the bounds reader's two bodies holds a check, red under its own build only (`decision/bounds-reader-bodies-each-checked`, guard `scrolled_box_client_rect`). Also the first WPT conformance reading on the dev host, report-only (`decision/fork-wpt-reading-report-only`). Any take touching `physical_unrounded_geometry` must stay green under both builds.

@history_item n=13 marker=amd-epgtg3nr
**amd-epgtg3nr** — The scrolled box's bounds and the hit walk were fixed in the engine on the founder's ruling. A raw harness click outside a clipping box no longer lands (`contract/harness-input/clipped-click`), and a check leaning on the old hit is found only by running the stand gates. The hit stops and bounds are among the intents that conflict resolutions must keep.

@history_item n=14 marker=amd-3kuwk3el
**amd-3kuwk3el** — Engine debts: a `contain: paint` box stops the hit, a scrolled box reads where it stands, a transformed box's client rect follows its transform, and `password_caret_click` places the caret by the mask. These back `decision/scrolled-box-reads-where-it-stands` and `decision/password-caret-placed-by-mask`. The engine behaviours a `hit_inner` / bounds conflict must keep.

@history_item n=15 marker=amd-ur5xsokp
**amd-ur5xsokp** — `layout_rect`, `layout_rect_of` and `center_of` read a scrolled box where it stands (`contract/harness-inspect/layout-rect`, `decision/scrolled-box-reads-where-it-stands`). This must survive any upstream hunk in the harness or layout readers.

@history_item n=16 marker=amd-fqdymewc
**amd-fqdymewc** — `hit_non_invertible_transform`: `Node::hit_inner` answers `None` for a subtree under a non-invertible transform. The merge surface `0980cce5f`'s `hit_inner` hunk conflicts with this. If the take touches `hit_inner`, the carry on the inline-root coordinate applies.

@history_item n=17 marker=amd-w5bh6ujd
**amd-w5bh6ujd** — 2026-10-13 sync precedent: five upstream targets each its own target, `all.rs` gaining `mod` lines and staying unbuilt, "no `all` executable" read at run time. The bridge's `is_bool_attr` changed and the CI-scripts tally moved. Pattern for counting this take's new test files.

@history_item n=18 marker=amd-n5xqwtgz
**amd-n5xqwtgz** — Quality gates: coverage floors, the zero-failure flakiness budget and the `stand-repeat` leg, the per-OS tallies record opened, and the font test skip disproved. Origin of `decision/stand-checks-zero-failure-budget`, `decision/zero-failure-stand-repeat`, `decision/stand-repeat-three-rounds` and `decision/font-metric-tests-never-skip`. A sync must keep every one of these green.

@history_item n=19 marker=amd-vgun7xw2
**amd-vgun7xw2** — 2026-10-17 sync: upstream's ARIA checks merged as their own targets, asserted with test-that's `verify_that!`. The blitz-tests directory count was found one short and corrected by measuring. Precedent: re-measure the target count, never trust the stated one.

@history_item n=20 marker=amd-ywv2ddmq
**amd-ywv2ddmq** — 2026-10-17 sync: the workspace, per-OS and agent-run tallies were re-read on the final HEAD's green run, with every moved count explained by name. This is the method this chunk's "per-OS tallies re-read" follows.

@history_item n=21 marker=amd-xpqvmmzd
**amd-xpqvmmzd** — The coverage leg excludes `keyboard_compat.rs` by filename (upstream's keyboard-types bridge) instead of lowering a floor. Its lifting is tied to taking `286874cc1`.

@history_item n=22 marker=amd-6ah7rxit
**amd-6ah7rxit** — keyboard-types 0.8: the harness re-exports `NamedKey`, and test-that 0.6.0 is named as the merged checks' assertion crate. This is the except of `decision/rust-tests-on-built-in-harness`. Dependency bumps a take brings are reviewed by hand.

@history_item n=23 marker=amd-jef7zjyx
**amd-jef7zjyx** — 2026-10-18 sync rule: tallies go from the masters' recorded reading straight to the new run, with any skipped reading named. A wrap that does not re-read the tallies leaves the next one to attribute the gap. Binds this chunk's tally re-read.

@history_item n=24 marker=amd-knnha4kj
**amd-knnha4kj** — 2026-10-19 sync: upstream's `focus_within` and `focus_redraw` became their own targets outside the a11y leg and `run stand`. The two earlier chunks' unwritten tallies were attributed per target from the job logs.

@history_item n=25 marker=amd-siqngma2
**amd-siqngma2** — A shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a byte-compared cross-OS check holds no `screenshot` step. Basis of `decision/no-screenshot-in-cross-os-checks` and `decision/shot-png-deterministic-per-host`. The take's paint changes (SVG clipping, layers) must keep this.

@history_item n=26 marker=amd-db7lxiqh
**amd-db7lxiqh** — The headless screenshot: frames are compared whole, a shot with no font is refused, never skipped, and the harness takes blitz-paint without default features. Origin of `decision/frames-compared-whole`, `decision/shot-refused-without-fonts` and `decision/headless-screenshot`. The take's blitz-paint commits (`d176ac4e6`, `beaa796f4`) land on this.

@history_item n=27 marker=amd-rrbdvino
**amd-rrbdvino** — The harness `screenshot` / `screenshot_rect` contract: at the harness's own time, white composite, `EmptyRect`, `FontsUnavailable`, no print, panic or log. These are the `contract/harness-screenshot/*` rows, kept through paint hunks.

@history_item n=28 marker=amd-rv3qyhnh
**amd-rv3qyhnh** — 2026-10-24 sync: upstream's `hit_test_dropped_nodes` came as its own target, read red with the engine hunks reversed, and the stated gaps were recorded (`PARALLEL_TRAVERSAL_LOCK`, out-of-flow helpers). Precedent for a take's engine change arriving with its upstream check.

@history_item n=29 marker=amd-hayhbwsb
**amd-hayhbwsb** — The `keyboard_compat.rs` exclusion stands until escher takes upstream's retirement of it in `286874cc1` (the Dioxus 0.8 move, out of this chunk's scope). This take leaves both regex lines in place.

@history_item n=30 marker=amd-d4acxs6q
**amd-d4acxs6q** — The a11y leg is fourteen `--test` words (`decision/a11y-leg-membership`), and each file joining a leg keeps its legs (`decision/joining-file-keeps-its-legs`). A per-job tally strips ANSI as `(\x1b|\^\[)\[[0-9;]*m` (`decision/ci-log-ansi-strip`). Needed for this chunk's a11y leg and tally re-read.

@history_item n=31 marker=amd-yptdayxo
**amd-yptdayxo** — The dev-host cognitive-complexity ceiling of 15: no CI leg runs it, and engine functions over it are left alone. Origin of `decision/complexity-ceiling-host-only`.

@history_item n=32 marker=amd-nsqxmuql
**amd-nsqxmuql** — `decision/complexity-ceiling-host-only` gained its except: the engine's click handler is held below the ceiling with upstream's arms split into `click_*` functions. An upstream commit editing a `handle_click` arm needs a hand re-apply. This is the carry `cry-lrzp6e2g` that this take must read.

@history_item n=33 marker=amd-ce7fplwt
**amd-ce7fplwt** — 2026-10-27 sync: upstream's new `pseudo_element_flow_classification` reaches only the workspace `test` leg and `run all`, not `stand-repeat`, `run stand` or a11y. The "legs reaching" record is written for each upstream target a take adds.

@history_item n=34 marker=amd-simkhsnt
**amd-simkhsnt** — 2026-10-27 sync: the per-OS and local tallies were re-read after the take, +1 line and +8 passed, every move named (new target, script tests, IME test, tab test). Template for this chunk's tally attribution.

@history_item n=35 marker=amd-uxfxcfgt
**amd-uxfxcfgt** — An upstream target asserting one layout mode was recorded as an exception to the both-modes rule, with `incremental_oracle` as the layout-identity witness. Precursor of the except on `decision/layout-tests-in-both-layout-modes`.

@history_item n=36 marker=amd-wb5mdgi4
**amd-wb5mdgi4** — `pseudo_element_flow_classification` was later edited to run both layout modes, ending that exception. Contrasts with the later rule (amd-gkbc5lsj) that upstream targets taken in a sync stay as written.

@history_item n=37 marker=amd-gkbc5lsj
**amd-gkbc5lsj** — `decision/layout-tests-in-both-layout-modes` except: upstream integration targets taken in a sync are kept as written, and those running one layout mode stand as named exceptions. This binds how this take's new and edited upstream test files are handled.

@history_item n=38 marker=amd-6q2sqo5s
**amd-6q2sqo5s** — The take added `file:`-load narrowing tests (`remote_document_cannot_load_file_urls`, `file_document_can_load_sibling_file_urls`) and CSS accessor tests to `tests/dom.rs`. These are the first guards of `decision/file-urls-only-for-file-documents`.

@history_item n=39 marker=amd-a42cryfq
**amd-a42cryfq** — Citations were re-pointed past a take's added lines, and a stretched range can silently take in new tests, which left a count false. Trap for this take's citation sweep.

@history_item n=40 marker=amd-6znlx4zi
**amd-6znlx4zi** — Each sync re-reads the `keyboard_compat` exclusion through the gate entry (`grep -c` over `ci-leg.sh` reading 2), with its owner moved forward to the next sync.

@history_item n=41 marker=amd-cp6hvbjs
**amd-cp6hvbjs** — The take's `file:`-load tests write files under `std::env::temp_dir()`, a test-only `TMPDIR` read. A new env read in the taken delta must be checked against the sink and env-read sweep.

@history_item n=42 marker=amd-rf4xxwvm
**amd-rf4xxwvm** — Upstream gave `apps/browser/src/tab.rs` a test module, the browser's sixth tested file. This is the file carrying the unjustified `expect_used` allow (carry `cry-eyudppev`), which this chunk handles only if the take touches the browser app.

@history_item n=43 marker=amd-tcrj7hll
**amd-tcrj7hll** — `:open` from an upstream take was recorded as untested on every document. Origin of the gap closed by `open_state` (`decision/open-pseudo-class-follows-attribute`).

@history_item n=44 marker=amd-hw6eolsp
**amd-hw6eolsp** — `open_state` (4, both modes) closes the `:open` gap. It is the guard of `decision/open-pseudo-class-follows-attribute` and is named in `stand-repeat`.

@history_item n=45 marker=amd-wbtnfb6d
**amd-wbtnfb6d** — Upstream's `task_polled_during_render_can_access_document` was recorded absent because `286874cc1` was not taken. Start of the history behind `decision/upstream-buffered-render-test-kept-as-written`.

@history_item n=46 marker=amd-fcjcosus
**amd-fcjcosus** — The windowed shell's redraw route from a take was recorded with no check of its own. Closed later by blitz-shell's first unit test (amd-26updt75).

@history_item n=47 marker=amd-26updt75
**amd-26updt75** — `CountedWakes` and blitz-shell's first unit test read the redraw route. An upstream edit to `blitz-shell/src/event.rs` must keep it green.

@history_item n=48 marker=amd-zucqymgd
**amd-zucqymgd** — An absence that holds while the searched file count is stale: upstream's module split added files with no test module. Precedent for restating inventory searches after a take reorganises modules.

@history_item n=49 marker=amd-gka5mudf
**amd-gka5mudf** — The agent-run Proof gets its next entry after a take: `run all` picks up new upstream targets with no script change, and `run stand` stays unmoved.

@history_item n=50 marker=amd-tcnuzkmh
**amd-tcnuzkmh** — Every cargo leg is `--locked` except `fmt` and `examples/wasm_hello` (`decision/cargo-legs-locked`). The audit and MSRV legs with a git Taffy are read under `--locked`.

@history_item n=51 marker=amd-w6i3vy4y
**amd-w6i3vy4y** — `stand-repeat` names fixed `--test` words pinned by `test_stand_repeat_leg_runs_three_rounds` (fifteen at that point). A new upstream target joins no leg list unless named.

@history_item n=52 marker=amd-kube42bu
**amd-kube42bu** — `stand-repeat` grew to seventeen named words (`select_popup`, `select_popup_painted`), and `pseudo_element_flow_classification` is none of them. This is the current list that upstream targets are measured against.

@history_item n=53 marker=amd-jwvcqtlb
**amd-jwvcqtlb** — `Stand checks repeated` went red on disk space (linker bus errors). The free-disk step was added with its cure unproven. Trap: a red CI job on disk space is no test failure.

@history_item n=54 marker=amd-c3jjjsfi
**amd-c3jjjsfi** — The matrix job holds no free-disk step, pinned by `test_only_stand_repeat_frees_disk`. An upstream `ci.yml` edit must keep this.

@history_item n=55 marker=amd-23w5gd5l
**amd-23w5gd5l** — A macOS-only red through `apple-font-embolden`. `cargo` stops at the first failing target, so later targets go unread. Origin of `decision/embolden-leg-local-only`. Relevant to this take's `@font-face` / fontique and paint commits.

@history_item n=56 marker=amd-yx45xaju
**amd-yx45xaju** — `fast` now ends on the `embolden` step, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (`decision/embolden-local-witness`, `decision/local-pre-push-gate`). This chunk's `fast` gate includes it.

@history_item n=57 marker=amd-ceviithy
**amd-ceviithy** — A key press panicked at `position_in_containing_block` for a focused control whose anonymous block was dropped. Its owner was a CARRY on the UI-graph sync.

@history_item n=58 marker=amd-2ljhblkm
**amd-2ljhblkm** — The panic was fixed in `BaseDocument::deallocate_anonymous_block` with the check `key_press_after_container_lost_its_box` (`decision/key-after-dropped-anonymous-block`). This is the merge surface flagged in carry `cry-oglnc26y` that this take re-reads.

@history_item n=59 marker=amd-jvq3mywt
**amd-jvq3mywt** — Upstream's failed-stylesheet fix stopped at `CountedHandler`, because a trait default compiled silently through the wrapper. The forward fixed it (`decision/failed-load-ends-settle-wait`, `contract/harness-settle/failed-load`). Rule: a take that extends a trait sweeps that trait's implementers.

@history_item n=60 marker=amd-bjdcpa2t
**amd-bjdcpa2t** — UI-graph sync tallies on CI#41333172440: +2 lines and +63 passed per test job, linux equal to `target/ci-logs/test.log` and the other jobs by arithmetic. Template and prior reading for this chunk's re-read.

@history_item n=61 marker=amd-duti3zma
**amd-duti3zma** — Line citations were re-pointed after the take across every detail file. The same sweep is owed here.

@history_item n=62 marker=amd-6o3k2s3y
**amd-6o3k2s3y** — blitz-net's unit tests arrived with test-only loopback binds. The `file:` read through `Url::to_file_path` still has no test of its own.

@history_item n=63 marker=amd-vrpzudrd
**amd-vrpzudrd** — The UI-graph sync left `286874cc1` out, so the `keyboard_compat` exclusion stands. The left-out delta's owner moved to the determinism sync and then to this one (`cry-qrse4pmp`).

@history_item n=64 marker=amd-xkpz46v3
**amd-xkpz46v3** — Blending and the inline box's paint skip were read on no frame, owned by a CARRY on the determinism sync. Relevant because this take brings more paint changes (SVG layers, clipping).

@history_item n=65 marker=amd-qbevnqg6
**amd-qbevnqg6** — Upstream commits with no test of their own are recorded with the workspace (and `incremental_oracle`) as their witness. `element_removed_before_its_mounted_event_is_flushed` was left out (`decision/upstream-mounted-flush-test-left-out`).

@history_item n=66 marker=amd-smvshwtp
**amd-smvshwtp** — `text_contrast` reads `None` past a `mix-blend-mode` ancestor after a take made blending paint. Precedent: a paint feature a take adds is checked against the harness readers.

@history_item n=67 marker=amd-dbyszidj
**amd-dbyszidj** — The `ci-scripts` leg was at 110 tests with the free-disk and embolden pins (`decision/ci-scripts-unit-tested`, `decision/python-script-tests-under-unittest`). This is the count the take's CI-script changes move from.

@history_item n=68 marker=amd-hqsdue54
**amd-hqsdue54** — `dioxus_runtime_probe` reaches the `test` leg and `run all` only. It reads Dioxus 0.7 public API with no fork-level change, and it is the guard of the buffered-render rows.

@history_item n=69 marker=amd-gcwvi4dg
**amd-gcwvi4dg** — `Dependency audit` went red on new advisories with `Cargo.lock` unchanged. It was fixed by a semver-compatible update and an ignore by ID. Trap for this chunk's audit leg, including with a git Taffy.

@history_item n=70 marker=amd-3llbasfo
**amd-3llbasfo** — The MountedData hazard's mechanism (the borrow across `render_immediate`) was pinned as `should_panic`, and the port of `286874cc1`'s buffered render was routed as the fix.

@history_item n=71 marker=amd-kmiudnmn
**amd-kmiudnmn** — The probe's two `…_spawned_acts_*` pins are the fix's discriminating witness on Dioxus 0.7, and the ported upstream test is insensitive there (`decision/upstream-buffered-render-test-kept-as-written`). Merge surface `cry-2hvzi4ns`.

@history_item n=72 marker=amd-jmojotdw
**amd-jmojotdw** — The buffered render (`MutationBuffer`, the document unborrowed during render) was ported onto Dioxus 0.7 (`decision/buffered-render-on-dioxus-07-bridge`, `decision/buffered-render-unborrowed-document`). Resolutions in `dioxus_document.rs` must keep it.

@history_item n=73 marker=amd-x4m5gjdg
**amd-x4m5gjdg** — dioxus-native-dom at 83 unit tests, with the ported upstream test, and `mutation_buffer.rs` holding none. This is the count a take's dioxus-native-dom changes move from.

@history_item n=74 marker=amd-6kqtriic
**amd-6kqtriic** — Upstream's mounted-event test stays out until the Dioxus 0.8 move, not with the buffered render. This is carry `cry-h4dkkdhq`, and this chunk keeps it out.

@history_item n=75 marker=amd-mfx3nppa
**amd-mfx3nppa** — The `keyboard_compat.rs` exclusion's owner is re-said as the open CARRY `cry-mjwr45xr` on the determinism sync (residual `res-6ssktw6x`). This chunk re-reads it again.

@history_item n=76 marker=amd-g4rjfxtm
**amd-g4rjfxtm** — dioxus-native-dom coverage 86.71 %, `mutation_buffer.rs` counted inside the 83 % floor. This is the floor reading the take must hold.

@history_item n=77 marker=amd-jy34nxjf
**amd-jy34nxjf** — Per-OS tallies on CI#42371051720 (the last per-OS record written). Rule: a tally move is attributed by grepping the test's own name, never the preceding `Running` line (`decision/tally-move-attributed-by-test-name`). Binds this chunk's tally re-read.

@history_item n=78 marker=amd-lfaykhfb
**amd-lfaykhfb** — Local baseline 223 result lines with the moves from 219 not attributed. Coverage passed with no per-crate figures read. This is a gap the next tally re-read inherits.

@history_item n=79 marker=amd-jgvjxxwn
**amd-jgvjxxwn** — The masters became rows: the plan's text turned into the decision and contract rows this chunk's picked rows come from.

@history_item n=80 marker=amd-ughddg5d
**amd-ughddg5d** — The determinism sync moved the action-pin assertions into `WorkflowPinTest` over every workflow (`decision/actions-pinned-to-commit-sha`). Upstream workflow edits a take brings must stay SHA-pinned.

@history_item n=81 marker=amd-n4ewxvtg
**amd-n4ewxvtg** — The same sync widened `decision/ci-actions-pinned-to-sha` to every workflow. Precedent for a sync widening a CI rule.

@history_item n=82 marker=amd-bn47lomj
**amd-bn47lomj** — `decision/file-urls-only-for-file-documents` was extended to blitz-net sub-resource loads (stylesheet, `@import`, font, image, iframe). This take's `@font-face` change (`f5ea1caf1`) and SVG image paint fall under it.

@history_item n=83 marker=amd-gso2bo7y
**amd-gso2bo7y** — `local_files_for_local_documents` was added as a guard of `decision/file-urls-only-for-file-documents`.

@history_item n=84 marker=amd-6ierb3xc
**amd-6ierb3xc** — `decision/file-urls-only-for-file-documents` now also covers the browser app's `file:` favicon and navigation gates, with user loads ungated. Kept if the take touches `apps/browser`.

@history_item n=85 marker=amd-lde2vbr3
**amd-lde2vbr3** — `decision/blitz-dom-requests-through-stamping-constructor`: blitz-dom's clippy config bans `Request::get` outside `stamped_request`, a new merge surface. This is carry `cry-nnm3rzya`: an upstream sub-resource `Request::get` (e.g. in `f5ea1caf1`) turns the clippy leg red until it is routed through `stamped_request`.

@history_item n=86 marker=amd-qi7vgvwx
**amd-qi7vgvwx** — `decision/real-provider-inside-stand-in`: blitz-net and tokio became blitz-tests dev-dependencies for the `file:` witness. Dependency review applies if the take moves them.

@history_item n=87 marker=amd-efljpsvp
**amd-efljpsvp** — `contract/harness-construction/from-vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)`. Upstream edits to the harness or the Dioxus construction path must keep it.

@history_item n=88 marker=amd-uimtzpsg
**amd-uimtzpsg** — `decision/cross-os-readings-by-answer-digest` (`stand_cross_os_readings`, a committed digest table over six tasks) and the `stand_bundled_font_families` guard. A take that moves layout or text numbers must re-measure the table, never loosen it.

@history_item n=89 marker=amd-nx5bnxd7
**amd-nx5bnxd7** — `contract/harness-inspect/focused` was added: with nothing focused, the reading answers the root html element. A take touching focus must keep it.

@history_item n=90 marker=amd-fnzr6gbj
**amd-fnzr6gbj** — `decision/start-in-state-reads-as-by-hand-path` was added (`stand_start_in_state`). A standing equivalence the take must keep in both modes.

@history_item n=91 marker=amd-ybvlv3ad
**amd-ybvlv3ad** — One virtual clock: `advance` is accepted on every session and moves the clock by exactly `ms`, and the harness clock is never moved by screenshot or settle. Origin of the `advance-*`, `decision/harness-virtual-clock` and `contract/harness-core/*` time rows. This is the "clock invariant" that conflict resolutions keep.

@history_item n=92 marker=amd-6u4kftng
**amd-6u4kftng** — `virtual_clock_engine` and `stand_virtual_clock` were added, `into_config` hands the harness clock to documents, a wrapped document keeps its own (`decision/wrapped-document-keeps-own-clock`), and the scrollbar fade moved to the document clock (`decision/scrollbar-fade-on-document-clock`).

@history_item n=93 marker=amd-oodkokb2
**amd-oodkokb2** — The founder ratified the one-virtual-clock and engine-debts PROVISIONAL batch. This is why those rows read "ratified".

@history_item n=94 marker=amd-jhx4wjqd
**amd-jhx4wjqd** — `settle` was defined: quiet pass, `NotSettled { busy: Loads }` at once, `SETTLE_PASS_LIMIT`, no clock read and no drain (`contract/harness-settle/*`). Trap: the changed set is non-empty on a fresh harness.

@history_item n=95 marker=amd-nlavi4cc
**amd-nlavi4cc** — The shared `fixtures` module and `HeldNet`. Only a fetching provider is wrapped in the load counter (`decision/harness-counts-fetching-providers-only`).

@history_item n=96 marker=amd-sam3o45d
**amd-sam3o45d** — `stand::boot_load` and `LoadNet`: every other stand boot stays offline (`decision/stand-boots-offline`, `decision/stand-load-page-fixed-answer`, the `contract/stand-boot/*` rows).

@history_item n=97 marker=amd-2gzx73ga
**amd-2gzx73ga** — The headless stand runs under the bundled DejaVu Sans with system fonts off and no skip path (`decision/stand-checks-bundled-font`, `decision/stand-pinned-real-font`, `decision/font-assertions-on-system-fonts` except). `@font-face` / fontique changes in the take land against this.

@history_item n=98 marker=amd-ur7tfp4h
**amd-ur7tfp4h** — The bridge tests live in a `cfg(test)` child module of `dioxus_document`, with no widening (`decision/bridge-tests-without-widening`). Trap: a sed or grep address ending at a function's name also selects every item whose name starts with it.

@history_item n=99 marker=amd-xep3xm3z
**amd-xep3xm3z** — `DioxusDocument::dispatch_to_vdom` became the one `cfg(test)` shim, the except of `decision/bridge-tests-without-widening`. A merge resolution in `dioxus_document.rs` must keep it.

@history_item n=100 marker=amd-myqerkgx
**amd-myqerkgx** — Parsed markup is pinned in blitz-tests, not at unit level (`decision/parsed-markup-pinned-with-real-parser`). Trap: the editor's Backspace arm is compiled out on macOS, so a state change is proven with clicks and typed characters.

@history_item n=101 marker=amd-x5hh3g7d
**amd-x5hh3g7d** — `decision/one-actionable-rule`: both actionable readers read current listeners, never the `data-dioxus-id` mark. A take must not move this.

@history_item n=102 marker=amd-poiocitr
**amd-poiocitr** — `stand_graph`: no served id holds `{owner}///{key}` (`decision/reserved-graph-id-form`, `contract/element-id/virtual-form`). The stable ids that conflict resolutions keep.

@history_item n=103 marker=amd-ebg5mftg
**amd-ebg5mftg** — Refusal detection: `stale`, `not-found`, `off-screen` and `covered`, the order of checks, and `scroll_into_view_nested` (`contract/refusal-cause/*`, `contract/session-run/scroll`). Hit and bounds changes in a take feed these causes.

@history_item n=104 marker=amd-gn53juc4
**amd-gn53juc4** — `scroll_into_view` on an open list's row moves the list's `scroll_y` alone (`contract/harness-input/scroll-into-view`).

@history_item n=105 marker=amd-uqr4ympz
**amd-uqr4ympz** — The select model covers `multiple` and `optgroup`, asserted by `select_interaction` and `stand_act_select` (`contract/session-run/type-on-select`). Part of the "select model and its list" that conflict resolutions keep.

@history_item n=106 marker=amd-di2mgndg
**amd-di2mgndg** — The chosen-row pair `#c8c8c8` on `#000000` in `default.css` reads 12.56:1 (`decision/chosen-select-row-colours`). An upstream UA-stylesheet edit must not move it.

@history_item n=107 marker=amd-e42t37s6
**amd-e42t37s6** — The select list retired the old limits: it opens on click, Enter or Space, and `size` and group headers are painted (`decision/select-list-painted-by-paint-scene`). A take's paint or layout hunks in select code must keep `select_popup` and `select_popup_painted` green.

@history_item n=108 marker=amd-ajxz4bno
**amd-ajxz4bno** — The changed set's contract and its tests in blitz-dom `document.rs` (`contract/harness-settle/side-effects` neither reads nor drains it). This is "the changed set's contract" that conflict resolutions keep.

@history_item n=109 marker=amd-wf5dzunp
**amd-wf5dzunp** — Per-OS and stand-repeat tallies on CI#41235503227, with each move named in the chunk's `evidence/ci.md`. Part of the tally series this chunk extends.
