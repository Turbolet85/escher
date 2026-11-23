@history_item n=1 marker=amd-dhibncze
**amd-dhibncze** — 2026-10-05-fork-ci-reached: CI triggers on build/**, each linux leg through `ci-leg.sh`, the fast `test` leg `cargo test --workspace --locked`, the matrix testing windows/macos and only building ios/android, and WPT and its PR posting run only on upstream `DioxusLabs/blitz` (repository guard) while the fork runs WPT on the host. This is where decision/ci-trigger-branches, decision/native-platform-ci-matrix, decision/wpt-run-on-host-on-fork and decision/wpt-upstream-only come from. The merge brings upstream's WPT tooling and pin (`120520960`, `cec303355`) and a CI Pages artifact change (`165d698c3`), so the repository guard and the fork's host-only WPT must survive the merge.

@history_item n=2 marker=amd-u7vpyh4t
**amd-u7vpyh4t** — 2026-10-10-upstream-sync-agent-surfaces: added `BlitzTestsTargetsTest` (test_blitz_tests_targets.py). It pins that the blitz-tests manifest leaves `autotests` on and holds exactly one `[[test]]` table, `all` with `test = false`, so a sync that takes upstream's two manifest lines turns red. It also counted upstream's new test_wpt_area_changes.py into the ci-scripts leg. This is the origin of decision/one-test-target-per-file, decision/blitz-tests-single-test-target and decision/blitz-tests-manifest-difference. The scope's upstream `display_contents.rs`, `ime_preedit_underline.rs` and the new `mod` line in `tests/all.rs` fall under this rule.

@history_item n=3 marker=amd-u6pbkyf3
**amd-u6pbkyf3** — 2026-10-10-upstream-sync-agent-surfaces: precedent for how a sync's test arrivals are recorded. Upstream's new checks are counted into the inventory. Arrivals that bring no test are recorded as observed absences, not as failed gates, and none counts as a test of ours. This chunk's ten commits (IME underline, `display: contents`, float keywords, textarea width, `DOMTokenList.replace`) need the same treatment.

@history_item n=4 marker=amd-cqdq6a7d
**amd-cqdq6a7d** — 2026-10-10-upstream-sync-agent-surfaces: on the merged tree, every moved count was restated and attributed to a named upstream file. It also found a second engine-feature split by runner: `writing-mode` is on in the workspace build only and chooses which of the bounds reader's two bodies is compiled. This matters because the `c4328ad5b` (logical float sides) and `display: contents` layout commits touch the layout these readers serve. It is the origin of decision/per-package-green-not-workspace-green.

@history_item n=5 marker=amd-s53lzeiq
**amd-s53lzeiq** — 2026-10-07-driver-command-spans: the workspace build compiles blitz-dom's `tracing` call sites in, and the per-package build does not. A check reading a sink's capture counts by target, and a per-package green is not the workspace leg's green. These are decision/sink-capture-counted-by-target and decision/per-package-green-not-workspace-green, which bind the scope's sink-allowlist re-check of the merged delta's new `tracing`/`log` call sites.

@history_item n=6 marker=amd-pzdy3myb
**amd-pzdy3myb** — 2026-10-10-scrolling-box-bounds-and-hit: first WPT conformance reading on the fork's dev host, report-only. Also added three engine checks per bounds-reader body (decision/bounds-reader-bodies-each-checked). It backs decision/fork-wpt-reading-report-only and decision/wpt-conformance-report-only; upstream's WPT numbers are out of this chunk's scope.

@history_item n=7 marker=amd-w5bh6ujd
**amd-w5bh6ujd** — 2026-10-13-upstream-sync-polish-and-ship: a sync's tests as precedent. Five upstream targets each became their own target, `all.rs` gained their `mod` lines and stayed unbuilt (no `all` executable), and test_wpt_area_changes.py went 4 → 5. All counts were re-read on the merged tree with old figures kept as dated history. It is the template for counting upstream's `display_contents` and `ime_preedit_underline` targets and any WPT-tooling CI-script test change.

@history_item n=8 marker=amd-vgun7xw2
**amd-vgun7xw2** — 2026-10-17-upstream-sync-engine-debts: upstream's merged targets are their own targets, run by the workspace `test` leg and per package, not by the `a11y` leg. The directory count was measured to correct a stale figure that was one short. Precedent for placing new upstream targets in legs (not a11y, not stand-repeat) and for re-measuring the target count rather than trusting it.

@history_item n=9 marker=amd-ywv2ddmq
**amd-ywv2ddmq** — 2026-10-17-upstream-sync-engine-debts: after the merge, the workspace and per-OS tallies were re-read on the final HEAD's green run, every moved count explained by name. Precedent for this chunk's tally re-read.

@history_item n=10 marker=amd-xpqvmmzd
**amd-xpqvmmzd** — 2026-10-17-upstream-sync-engine-debts: both coverage lines exclude `keyboard_compat\.rs$` (upstream's temporary keyboard-types 0.7/0.8 bridge) on the operator's fork answer, with the floors unchanged. This is the origin of decision/keyboard-compat-excluded-from-coverage and decision/keyboard-compat-coverage-exclusion. The scope's CARRY `cry-ikyyzjzj` requires re-reading this exclusion at this sync.

@history_item n=11 marker=amd-6ah7rxit
**amd-6ah7rxit** — 2026-10-17-upstream-sync-engine-debts: the harness re-exports keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey`. test-that 0.6.0 is named as the assertion crate of the merged upstream checks (`verify_that!`, `TestResult<()>`). New upstream tests may use that form under the built-in harness.

@history_item n=12 marker=amd-jef7zjyx
**amd-jef7zjyx** — 2026-10-18-upstream-sync-determinism-and-traces: upstream's test was counted and the tallies re-read on CI. Rule for later syncs: a wrap that does not re-read the tallies leaves the next wrap to attribute the gap. This chunk inherits that the tallies are re-read at the sync.

@history_item n=13 marker=amd-knnha4kj
**amd-knnha4kj** — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness: upstream's two focus targets were each their own target, not in the a11y leg or `run stand`, with `all.rs` gaining `mod` lines. Per-OS tallies were attributed per target, including earlier unwritten moves, and `run all` was re-counted. Precedent for new upstream targets and tally attribution.

@history_item n=14 marker=amd-rv3qyhnh
**amd-rv3qyhnh** — 2026-10-24-upstream-sync-polish-and-ship: upstream's `hit_test_dropped_nodes` was its own target, read red with its engine hunks reversed and green with them. Untested arrivals (the `PARALLEL_TRAVERSAL_LOCK`, out-of-flow helpers) were recorded as stated gaps, and the directory count was corrected from a stale figure. Precedent for a sync's new upstream checks and its gaps.

@history_item n=15 marker=amd-hayhbwsb
**amd-hayhbwsb** — 2026-10-24-upstream-sync-polish-and-ship: the `keyboard_compat.rs` exclusion now holds "until escher takes upstream's retirement of it" (`286874cc1`, Dioxus 0.8.0-alpha.3, not taken). Both regex lines and the floors stand. The Dioxus hold re-read in this chunk decides whether that lifts.

@history_item n=16 marker=amd-ce7fplwt
**amd-ce7fplwt** — 2026-10-27-upstream-sync-engine-debts: records which legs reach upstream's new `pseudo_element_flow_classification` target: the workspace `test` leg and `run all` with no script line naming it, and not stand-repeat, `run stand` or a11y. The directory count went 147 → 148. Template for recording the legs that reach `display_contents` and `ime_preedit_underline`.

@history_item n=17 marker=amd-uxfxcfgt
**amd-uxfxcfgt** — 2026-10-27-upstream-sync-engine-debts: an upstream layout target that asserts neither layout mode was recorded as an exception to the both-modes rule, with `incremental_oracle` the layout-identity witness. Bears on decision/layout-tests-in-both-layout-modes for upstream's new `display_contents.rs`.

@history_item n=18 marker=amd-wb5mdgi4
**amd-wb5mdgi4** — 2026-10-31-engine-debts: the fork later edited `pseudo_element_flow_classification` to run `for incremental in [false, true]`, ending upstream's both-modes exception. This is a counter-precedent to keeping upstream targets as written, and it puts a fork edit into an upstream test file, which is merge surface for later syncs.

@history_item n=19 marker=amd-gkbc5lsj
**amd-gkbc5lsj** — 2026-11-13-upstream-sync-determinism-and-exploration: decision/layout-tests-in-both-layout-modes gained its except. Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions; six of seven upstream targets were so recorded. This is the standing rule for upstream's new `display_contents.rs` and `ime_preedit_underline.rs`.

@history_item n=20 marker=amd-6q2sqo5s
**amd-6q2sqo5s** — 2026-10-27-upstream-sync-engine-debts: the sync's script tests were counted (`tests/dom.rs` 27 → 30), including the `file:`-load narrowing tests. Precedent for counting the `6fd5a3ade` `DOMTokenList.replace` change in blitz-vibey-script, whether or not it brings a test.

@history_item n=21 marker=amd-a42cryfq
**amd-a42cryfq** — 2026-10-27-upstream-sync-engine-debts: cited line ranges in merged upstream files were re-pointed after the take, and three ranges stretched over tests the take added inside them, leaving a count false. This is a trap for this merge's touched files.

@history_item n=22 marker=amd-6znlx4zi
**amd-6znlx4zi** — 2026-10-27-upstream-sync-engine-debts: re-read the `keyboard_compat` exclusion with `286874cc1` left out. The gate entry `test -f …keyboard_compat.rs && grep -c` over `ci-leg.sh` reads 2. The exclusion's owner was moved to the next sync's residual. This is the re-read procedure this sync repeats.

@history_item n=23 marker=amd-wbtnfb6d
**amd-wbtnfb6d** — 2026-10-27-upstream-sync-engine-debts: upstream's `task_polled_during_render_can_access_document` was recorded as absent because its commit `286874cc1` was not taken. This is the first record of an upstream test left out with the Dioxus 0.8 commit.

@history_item n=24 marker=amd-fcjcosus
**amd-fcjcosus** — 2026-10-27-upstream-sync-engine-debts: an untested upstream route (the windowed shell's redraw through its event loop) was recorded as holding no check of its own, a gap recorded rather than fixed. Same treatment for any upstream paint or layout change that arrives without a test.

@history_item n=25 marker=amd-jvq3mywt
**amd-jvq3mywt** — 2026-11-01-upstream-sync-ui-graph: trap. Upstream's failed-stylesheet fix stopped silently at the harness's `CountedHandler` because `NetHandler::error`'s do-nothing default compiled through the wrapper, and no gate read it. Rule: a take that extends a trait sweeps that trait's implementers. Applies to any trait this merge extends.

@history_item n=26 marker=amd-yx45xaju
**amd-yx45xaju** — 2026-11-01-upstream-sync-ui-graph: `fast` ends with the `embolden` leg (`--features blitz-paint/font-embolden --test select_popup_painted`), pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. This is decision/local-pre-push-gate, decision/embolden-leg-local-only and decision/embolden-local-witness. The scope's `7beb59176` IME preedit underline is a blitz-paint change, and the embolden witness is among the proofs that must pass.

@history_item n=27 marker=amd-23w5gd5l
**amd-23w5gd5l** — 2026-10-31-select-popup: CI went red on `Test (macos)` alone through `apple-font-embolden` while every local gate read green. Also, `cargo` stops at the first failing target, so a red job's later targets go unread. This is why the embolden witness exists, and it is a trap for a blitz-paint change such as the IME underline.

@history_item n=28 marker=amd-2ljhblkm
**amd-2ljhblkm** — 2026-11-01-upstream-sync-ui-graph: the fork ended the key-press panic past a dropped anonymous block in the engine (`deallocate_anonymous_block`), adding `key_press_after_container_lost_its_box`, and recorded its legs. This is engine merge surface: upstream's `display: contents` changes (`1fa2034d0`, `719268a9e`) touch anonymous-block and inline wrapping near it. See also amd-ffczzr5t.

@history_item n=29 marker=amd-ffczzr5t
**amd-ffczzr5t** — 2026-11-21-engine-debts: − decision/key-after-dropped-anonymous-block. The post-layout focus fixup now takes focus off a control whose anonymous block was dropped, held by `key_press_after_container_lost_its_box`. This is fork engine behaviour that a conflict in upstream's anonymous-block/`display: contents` layout must keep.

@history_item n=30 marker=amd-bjdcpa2t
**amd-bjdcpa2t** — 2026-11-01-upstream-sync-ui-graph: per-OS tallies on the merge's CI run, each moved line attributed (upstream's new targets plus the chunk's). The local baseline was re-counted and `run all` re-read. Template for this sync's tally record.

@history_item n=31 marker=amd-vrpzudrd
**amd-vrpzudrd** — 2026-11-01-upstream-sync-ui-graph: the `keyboard_compat.rs` exclusion was re-read through that take (thirty-six commits taken, twenty-one left out, `286874cc1` among them), and the owner moved to the next sync. This sync repeats the re-read.

@history_item n=32 marker=amd-xkpz46v3
**amd-xkpz46v3** — 2026-11-01-upstream-sync-ui-graph: upstream paint changes (blend modes, an inline box's paint skip) arrived with unit tests only and no frame test, recorded as gaps with a CARRY owner. Precedent for upstream's `7beb59176` IME preedit underline (blitz-paint) if it comes with no frame test of ours.

@history_item n=33 marker=amd-qbevnqg6
**amd-qbevnqg6** — 2026-11-01-upstream-sync-ui-graph: four upstream commits with no test of their own were recorded with the workspace (`incremental_oracle` among it) as their witness. Upstream's `element_removed_before_its_mounted_event_is_flushed` was left out because it reads 1 mounted event on Dioxus 0.7 where it asserts 0. This is the origin of decision/upstream-mounted-flush-test-left-out, which the Dioxus hold re-read touches.

@history_item n=34 marker=amd-duti3zma
**amd-duti3zma** — 2026-11-01-upstream-sync-ui-graph: line citations into `ci-leg.sh`, `ci.yml`, `test_ci_workflows.py` and merged engine files were re-pointed after the take, with claims unmoved. This is the expected citation sweep for this merge's 21 files.

@history_item n=35 marker=amd-3llbasfo
**amd-3llbasfo** — 2026-11-04-dioxus-runtime-probe: the MountedData hazard was re-measured and pinned by two `should_panic` tests in `dioxus_runtime_probe`. Its fix was routed to porting upstream `286874cc1`'s buffered render onto the 0.7 bridge, with upstream's absent test as witness. This is the history behind decision/buffered-render-on-dioxus-07-bridge.

@history_item n=36 marker=amd-hqsdue54
**amd-hqsdue54** — 2026-11-04-dioxus-runtime-probe: `dioxus_runtime_probe` (10 tests, 8 in both layout modes) reads Dioxus 0.7.10 public API alone. It runs in the workspace `test` leg and `run all`, not stand-repeat, a11y or `run stand`. It is the guard of the buffered-render rows, so a Dioxus pin move or hold is read through it.

@history_item n=37 marker=amd-kmiudnmn
**amd-kmiudnmn** — 2026-11-07-mounteddata-buffered-render: the probe's pins were inverted to `…_spawned_acts_{non_incremental,incremental}`, the fix's discriminating witness on Dioxus 0.7. Upstream's `task_polled_during_render_can_access_document` was ported as written, passing but insensitive to the borrow on 0.7. This is the source of decision/upstream-buffered-render-test-kept-as-written. A conflict in `dioxus_document.rs` must keep both.

@history_item n=38 marker=amd-jmojotdw
**amd-jmojotdw** — 2026-11-07-mounteddata-buffered-render: `initial_build` and `poll` render into the crate-private `MutationBuffer` with the document unborrowed. It is upstream `286874cc1`'s buffered render ported onto the 0.7 bridge, its Dioxus 0.8 move not taken. This is decision/buffered-render-unborrowed-document, the "buffered render" the scope lists among the intents a conflict must keep, and the reason the Dioxus hold matters.

@history_item n=39 marker=amd-6kqtriic
**amd-6kqtriic** — 2026-11-07-mounteddata-buffered-render: `element_removed_before_its_mounted_event_is_flushed` stays out until the Dioxus 0.8 move, owned by residual `res-6ssktw6x` and CARRY `cry-mjwr45xr`. This is decision/upstream-mounted-flush-test-left-out. The scope re-reads the standing left-out set and the Dioxus hold.

@history_item n=40 marker=amd-mfx3nppa
**amd-mfx3nppa** — 2026-11-07-mounteddata-buffered-render: the `keyboard_compat.rs` exclusion's owner was re-said as residual `res-6ssktw6x` absorbed into `v030-03`, its re-read owned by the open sync CARRY. This is the owner chain the scope's CARRY `cry-ikyyzjzj` continues.

@history_item n=41 marker=amd-x4m5gjdg
**amd-x4m5gjdg** — 2026-11-07-mounteddata-buffered-render: dioxus-native-dom has 83 unit tests including upstream's ported test, and `mutation_buffer.rs` holds none. These are counts a conflict in the bridge would move.

@history_item n=42 marker=amd-i3566nqw
**amd-i3566nqw** — 2026-11-21-engine-debts: + decision/render-wait-by-stored-waker. A test task waiting for a later render on the Dioxus bridge leaves its waker to the producer, because a self-waking wait livelocks inside the buffered render. This binds any test of the merged bridge.

@history_item n=43 marker=amd-jy34nxjf
**amd-jy34nxjf** — 2026-11-07-mounteddata-buffered-render: in a GitHub Actions job log, stderr `Running` lines and stdout `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name. This is decision/tally-move-attributed-by-test-name, used for the merge's tally re-read.

@history_item n=44 marker=amd-d4acxs6q
**amd-d4acxs6q** — 2026-10-24-a11y-re-read: the a11y leg names fourteen `--test` words, and its membership is the founder's word. A per-job tally strips ANSI with `(\x1b|\^\[)\[[0-9;]*m`. Upstream focus targets stay outside the leg. Membership precedent for upstream's new targets (decision/a11y-leg-membership) and a tally-reading trap.

@history_item n=45 marker=amd-j45pb4zi
**amd-j45pb4zi** — 2026-11-18-a11y-re-read: ~ decision/a11y-leg-membership. The rule keeps its criterion (an SC 2.1.1 or 2.4.3 witness, a tree reading, or a recorded gap) and drops the list. An upstream target joins the a11y leg only if it meets that criterion; the scope requires the a11y leg green.

@history_item n=46 marker=amd-zn44tekk
**amd-zn44tekk** — 2026-11-18-capability-statement: ~ decision/capabilities-doc-pinned-to-schema. The capability statement's a11y leg file count and stand-repeat named-target count are pinned to the CI leg script (`CapabilityStatementTest`). If the merge or this chunk moves either leg's membership, ci-scripts reads red until the page re-states it.

@history_item n=47 marker=amd-n2omhba3
**amd-n2omhba3** — 2026-11-21-engine-debts: ~ decision/count-and-rename-sweeps. A change of a CI leg's file count sweeps tests, examples and CI-script tests for that count held as a literal, after a missed a11y-leg count pin. Applies if upstream's new targets are placed in a counted leg.

@history_item n=48 marker=amd-tcnuzkmh
**amd-tcnuzkmh** — 2026-10-31-engine-debts: every cargo leg is `--locked` except `fmt` and `examples/wasm_hello` (decision/cargo-legs-locked). The operator's directive that a lock move is measured by a `--locked` workspace build rests on this.

@history_item n=49 marker=amd-gcwvi4dg
**amd-gcwvi4dg** — 2026-11-04-dioxus-runtime-probe: `Dependency audit` went red on an unchanged `Cargo.lock` and `deny.toml` from newly published advisories. It was fixed by a semver-compatible bump and an ID ignore. The scope requires the audit green, and the RUSTSEC ignore CARRY (`cry-rqochxka`) re-points `deny.toml`. This is the precedent for an audit red that the merge did not cause.

@history_item n=50 marker=amd-lde2vbr3
**amd-lde2vbr3** — 2026-11-13-local-files-for-local-documents: + decision/blitz-dom-requests-through-stamping-constructor. A crate-level clippy `disallowed-methods` config makes the clippy leg fail on any blitz-dom request built outside the stamping constructor. This config does not exist upstream and is new merge surface; the scope lists "the stamped sub-resource request" among the intents a conflict keeps.

@history_item n=51 marker=amd-gith26gr
**amd-gith26gr** — 2026-11-17-upstream-sync-ahead-of-polish-and-ship: a sync took an upstream sub-resource load (SVG image/feImage href), and ~ decision/file-urls-only-for-file-documents widened the `file:` refusal to it, ratified as measured. Precedent: any new upstream fetch path in this merge must meet the `file:` refusal.

@history_item n=52 marker=amd-xy6zexlx
**amd-xy6zexlx** — 2026-11-17-upstream-sync-ahead-of-polish-and-ship: − decision/only-stand-repeat-frees-disk, replaced by + decision/free-disk-on-test-and-stand-repeat. A sync put the pinned free-disk step on the default-features test job too. These are CI workflow invariants held by test_ci_workflows.py that a merge of upstream's `ci.yml`/workflow edits must keep.

@history_item n=53 marker=amd-jwvcqtlb
**amd-jwvcqtlb** — 2026-10-31-engine-debts: `Stand checks repeated` went red on linker bus errors at 5 MB of free disk. The free-disk step (`jlumbroso/free-disk-space` pinned by SHA) was added with its cure unproven. This is background for the free-disk rows and for reading a red stand-repeat job on the sync's CI.

@history_item n=54 marker=amd-ughddg5d
**amd-ughddg5d** — 2026-11-13-upstream-sync-determinism-and-exploration: ~ decision/actions-pinned-to-commit-sha. The guard moved to `WorkflowPinTest`, which reads every workflow, and amd-n4ewxvtg widened decision/ci-actions-pinned-to-sha to every workflow. The scope requires CI workflow edits to keep every action pinned to a 40-hex SHA; upstream's `165d698c3` edits a workflow.

@history_item n=55 marker=amd-n4ewxvtg
**amd-n4ewxvtg** — 2026-11-13-upstream-sync-determinism-and-exploration: ~ decision/ci-actions-pinned-to-sha now covers every action of every CI workflow, all four pinned. Upstream's workflow files taken in this merge (WPT tooling, Pages artifact) fall under it.

@history_item n=56 marker=amd-uzjso2t2
**amd-uzjso2t2** — 2026-11-19-upstream-sync-ahead-of-the-debts (the last sync): ~ decision/rust-tests-on-built-in-harness. The except now covers all upstream merged checks returning `TestResult<()>` with `verify_that!`, not two named ones, after upstream's `accessibility_hidden` test arrived. Upstream's new test files may use that form.

@history_item n=57 marker=amd-u73gab6u
**amd-u73gab6u** — 2026-11-21-dependency-and-ci-debts: + decision/matrix-skia-from-verified-archive and + decision/skia-witness-reads-by-content. Each matrix row verifies its recorded skia archive, and the witness reads skia-bindings' build tree by content because the CI Cargo version moves the layout. These must stay green on the sync's fork CI run.

@history_item n=58 marker=amd-jk46lk7u
**amd-jk46lk7u** — 2026-11-21-dependency-and-ci-debts: + decision/hosted-runner-cancel-hand-rerun (a no-runner cancel is answered by a hand re-run, with the CI token kept read-only) and + decision/ci-flag-rebuild-limit (a dev-host leg run with CI set rebuilds the workspace's units; the leg script does not export CI). Both bind how the sync's CI and local legs are read.

@history_item n=59 marker=amd-zoxar2ba
**amd-zoxar2ba** — 2026-11-21-dependency-and-ci-debts: decision/hosted-runner-cancel-hand-rerun changed from provisional to standing on the founder's ratification. This is the rule's current status when a sync CI job is cancelled with no runner.

@history_item n=60 marker=amd-6eogcl6u
**amd-6eogcl6u** — 2026-11-20-codebase-health: + decision/upstream-oversize-files-kept (upstream-authored files over 800 code lines are kept as upstream wrote them, since a split is a standing divergence in the files every sync touches most) and + decision/oversize-test-targets-kept. A merge conflict must not be resolved by splitting an upstream file.

@history_item n=61 marker=amd-rx3se5ar
**amd-rx3se5ar** — 2026-11-20-codebase-health: + decision/upstream-test-clone-pairs-kept. Upstream's duplicated test pairs are kept as upstream wrote them, with the duplication reading not held against them. This applies to upstream tests this merge brings.

@history_item n=62 marker=amd-nsqxmuql
**amd-nsqxmuql** — 2026-11-13-cognitive-load-corrective: ~ decision/complexity-ceiling-host-only except. The engine's click handler is held below the ceiling, upstream's arms included, at the price of a hand re-apply when an upstream sync edits one of its arms. This is a known merge trap if this take touches that handler.

@history_item n=63 marker=amd-ihzt4hfv
**amd-ihzt4hfv** — 2026-11-20-codebase-health: ~ decision/complexity-ceiling-host-only why. The founder ratified that upstream-authored functions over the ceiling are kept as upstream wrote them. A take's over-ceiling upstream functions are not refactored.

@history_item n=64 marker=amd-dpfqyxgg
**amd-dpfqyxgg** — 2026-11-01-upstream-sync-ui-graph: a take moved the coverage readings of two crates, read at the merge's CI run with `coverage floor: 3 of 3`. This is the coverage-floor re-read decision/per-crate-coverage-floors expects after a merge.

@history_item n=65 marker=amd-giaue4qm
**amd-giaue4qm** — 2026-11-01-upstream-sync-ui-graph: upstream's blitz-dom unit tests were counted, including the 39 restyle tests through `check_incremental_restyle` and the inline-flag tests (`is_inline_with_only_block_children` following `display: contents` descendant edits). Upstream's `1fa2034d0`/`719268a9e` `display: contents` changes land on those tests and the incremental oracle (decision/incremental-differential-oracle).
