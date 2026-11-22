@history_item n=1 marker=amd-gcwvi4dg
**amd-gcwvi4dg** (2026-11-04-dioxus-runtime-probe, the red Dependency audit) — records CI#41911351081 red on `Dependency audit` with `Cargo.lock` and `deny.toml` unchanged: RUSTSEC-2026-0433 on pulldown-cmark, fixed by a semver-compatible update to 0.13.6, and RUSTSEC-2026-0429 on zip 2.4.2, ignored by ID. The why and the local reproduction are left to security-plan §Dependency Security. This is the direct precedent for this chunk: the zip ignore it added is one of the per-ID ignores the chunk must close or state as a wait, and it shows that an advisory published later can redden the audit with no code change.

@history_item n=2 marker=amd-uy5wnu36
**amd-uy5wnu36** (2026-10-05-ci-gate-legs) — added the `audit` leg (`cargo deny check advisories`) beside the a11y and coverage legs. It also added the CI-workflow invariants for SHA pins, toolchain inputs and workflow `permissions`, and the cache-less / restore-only cache split. This is the origin of decision/dependency-audit-advisories-and-licenses, decision/ci-actions-pinned-to-sha and decision/ci-read-only-permissions, which the skia fetch step and any `deny.toml` edit must respect. Trap: `cargo llvm-cov report` without `--workspace` prints an empty table and exits 0.

@history_item n=3 marker=amd-n5xqwtgz
**amd-n5xqwtgz** (2026-10-14-quality-gates) — widened the audit, clippy and coverage legs, took CI to thirteen linux jobs with the `stand-repeat` leg, put the dpkg repair into the ten install steps, and set the CI-scripts count at 94 with `InstallScriptTest` at eight. The audit's widening is what decision/dependency-audit-advisories-and-licenses states: advisories and licenses, the whole locked workspace, all features. Any dependency move in this chunk is judged by that leg.

@history_item n=4 marker=amd-dhibncze
**amd-dhibncze** (2026-10-05-fork-ci-reached) — set up CI on the build branch:
- each linux leg runs through `ci-leg.sh`, the same command the host runs, and every cargo leg is `--locked`;
- four fast jobs gate all the others;
- the matrix tests windows and macos and only builds ios and android, while linux is the `test` leg's;
- WPT runs only upstream behind a repository guard;
- `test_ci_workflows.py` holds the invariants.

This is the source of decision/native-platform-ci-matrix, ci-legs-through-leg-script, fast-jobs-gate-the-rest, upstream-repository-guard and ci-legs-host-reproducible. The cancelled-runner watch (the hosted macOS/iOS job) and any skia CI step sit inside this frame.

@history_item n=5 marker=amd-aeuarcbq
**amd-aeuarcbq** (2026-10-05-fork-ci-reached, local baseline re-measured) — the dev profile carries `debug = "line-tables-only"`. The baseline is defined as the dev profile, workspace default features, `--locked`, no `opt-level` rewrite, cold after a clean and warm on an immediate re-run. This is decision/local-baseline. The `CI=true` recompile carry asks why a CI-flagged fast leg builds under new hashes, so it has to be read against this baseline.

@history_item n=6 marker=amd-3j6rcoyw
**amd-3j6rcoyw** (2026-11-19-build-directory-held) — two §9 rows:
- ~ decision/local-pre-push-gate: on a dev host with CI unset, a green fast leg ends with the target sweep, whose refusal never reddens the leg.
- + decision/target-sweep-safe-around-builds.

The chunk's `CI=true` recompile carry (`cry-sqod24yl`) comes from this chunk's measured limit, and the sweep's CI-unset gate is exactly the `CI` read the carry says no project source makes. Re-verify both.

@history_item n=7 marker=amd-4pvhutjm
**amd-4pvhutjm** (2026-11-19-build-directory-held) — + decision/build-directory-held-by-sweep. Cargo mints a unit hash per feature resolution, lockfile and toolchain epoch, and the workspace, blitz-tests, embolden and seven_guis resolutions each stay their own build. This bears on the `CI=true` recompile carry and on any dependency or lockfile move: a `Cargo.lock` change mints new hashes.

@history_item n=8 marker=amd-77jliqql
**amd-77jliqql** (2026-10-09-audit-corrections-agent-surfaces) — added `test_o_package_installs_are_bounded`: no step's `run` names `apt-get`, installs go through `apt-install.sh` with package names only, the calling jobs are exactly `INSTALL_JOBS`, and the one action install carries `timeout-minutes`. It also added `InstallScriptTest` (3 attempts, a 120 s bound). This is the origin of decision/bounded-package-installs, package-installs-bounded-and-retried and ci-package-installs-bounded. A new skia fetch or install step in `ci.yml` must keep this invariant and update `INSTALL_JOBS` if it calls the script.

@history_item n=9 marker=amd-n4ewxvtg
**amd-n4ewxvtg** (2026-11-13-upstream-sync-determinism-and-exploration) — ~ decision/ci-actions-pinned-to-sha: the rule widened from the CI workflow to every workflow. Any action this chunk adds (a skia fetch step, a retry wrapper) must be SHA-pinned in every workflow.

@history_item n=10 marker=amd-ughddg5d
**amd-ughddg5d** (2026-11-13-upstream-sync-determinism-and-exploration) — ~ decision/actions-pinned-to-commit-sha guard: `CiWorkflowTest` → `WorkflowPinTest`, which reads every workflow and asserts that every Rust toolchain step names its toolchain. A new toolchain or action step is held by `WorkflowPinTest`.

@history_item n=11 marker=amd-xy6zexlx
**amd-xy6zexlx** (2026-11-17-upstream-sync-ahead-of-polish-and-ship) — − decision/only-stand-repeat-frees-disk → + decision/free-disk-on-test-and-stand-repeat: the default-features test job gains the pinned free-disk step, under a renamed guard. This is the current CI disk rule and its guard `test_free_disk_only_on_test_and_stand_repeat`. A skia archive on the runners and any job edit must keep the matrix job free of a free-disk step.

@history_item n=12 marker=amd-jwvcqtlb
**amd-jwvcqtlb** (2026-10-31-engine-debts) — after CI#41190771843 went red with `Free space left: 5 MB`, `Stand checks repeated` gained the free-disk step (`jlumbroso/free-disk-space` pinned). Its cure is recorded as unproven, and the runner image varied between jobs. This is a precedent for a CI debt kept "written and not claimed as the remedy", the same shape a stated wait or a kept watch takes in this chunk.

@history_item n=13 marker=amd-c3jjjsfi
**amd-c3jjjsfi** (2026-11-01-upstream-sync-ui-graph) — removed the matrix job's free-disk step, which never fired because its `if:` named `ubuntu-24.04` while android runs `ubuntu-latest`. It is pinned by `test_only_stand_repeat_frees_disk`, since renamed. Trap: a matrix-row `if:` keyed on the runner label silently skips. This bears on any change this chunk makes to the macOS/iOS matrix rows for the cancelled-runner watch.

@history_item n=14 marker=amd-tcnuzkmh
**amd-tcnuzkmh** (2026-10-31-engine-debts) — decision/cargo-legs-locked now reads every cargo leg `--locked` except `fmt` and `examples/wasm_hello`. A dependency version move in this chunk is a `Cargo.lock`/manifest change that every locked leg reads, so the lock must be committed with it.

@history_item n=15 marker=amd-s53lzeiq
**amd-s53lzeiq** (2026-10-07-driver-command-spans) — added §9 "Engine features by runner": the workspace build compiles blitz-dom with `tracing` and the per-package build does not. Rule: a green per-package run is not the workspace leg's green. This is the source of decision/per-package-green-not-workspace-green. A dependency move must be proven on the workspace `test` leg, not per package.

@history_item n=16 marker=amd-23w5gd5l
**amd-23w5gd5l** (2026-10-31-select-popup) — CI#41233234207 was red on `Test (macos)` alone through `apple-font-embolden`, since no local gate built `blitz-paint/font-embolden`. `cargo test … --features blitz-paint/font-embolden --test select_popup_painted` reproduced macOS's numbers on linux. Also: cargo stops at the first failing target. This is the precedent for OS-only reds and their local witness, and it bears on reading macOS/iOS matrix outcomes (the cancelled-runner watch) and on skia, which also builds per platform.

@history_item n=17 marker=amd-yx45xaju
**amd-yx45xaju** (2026-11-01-upstream-sync-ui-graph) — the `fast` leg reads fmt → clippy → test → ci-scripts → embolden, with `embolden` in `LEGS` and last in `FAST_LEGS`, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. This is the source of decision/local-pre-push-gate and embolden-local-witness. A new local skia checksum check in a leg must keep this order and its pin.

@history_item n=18 marker=amd-dbyszidj
**amd-dbyszidj** (2026-11-01-upstream-sync-ui-graph) — the `ci-scripts` leg reads 110 tests in eight files, with test_ci_workflows.py at 35 and two new pins named. This is the count precedent: new tests for a skia fetch script or for workflow invariants move these counts, and §1, §4 and the leg tally must agree.

@history_item n=19 marker=amd-u7vpyh4t
**amd-u7vpyh4t** (2026-10-10-upstream-sync-agent-surfaces) — added `BlitzTestsTargetsTest`, which reads the manifest with `tomllib`, so the `ci-scripts` leg needs Python 3.11 or later; the CI runner's version was not read. The CI-scripts count moved by file. A CI-script test of a new checksum file or of `deny.toml` that uses `tomllib` inherits this unread runner premise.

@history_item n=20 marker=amd-n2omhba3
**amd-n2omhba3** (2026-11-21-engine-debts) — ~ decision/count-and-rename-sweeps: a change of a CI leg's file count now sweeps the tests, examples and CI-script tests for that count held as a literal. A sweep over the leg script's names had missed one. This binds any leg-membership change this chunk makes in `ci-leg.sh`.

@history_item n=21 marker=amd-m7fw5fjq
**amd-m7fw5fjq** (2026-11-18-capability-statement) — ~ decision/capability-statement-pinned: the pin also holds the a11y leg's file count and stand-repeat's named-target count against `ci-leg.sh`, so a change to either leg reddens ci-scripts until `docs/capabilities.md` re-states it. A `ci-leg.sh` edit in this chunk must re-state the page.

@history_item n=22 marker=amd-aiakktz6
**amd-aiakktz6** (2026-11-18-capability-statement) — ~ decision/capability-check-names-held-to-schema: only the claim section's names and counts are pinned, and a renamed or moved check changes that section by hand. It goes with decision/capability-claims-name-their-checks: a limit this chunk states (an advisory wait, the skia archive) is either pinned by a check or stated as asserted by none.

@history_item n=23 marker=amd-472linzq
**amd-472linzq** (2026-10-25-capability-statement) — first put `docs/capabilities.md` in test scope through `CapabilityStatementTest`, with the claim map naming each claim's check. This is the origin of decision/capability-claims-name-their-checks, which the Codebase-health claim (`v040-08`) and any stated dependency limit in the capabilities document fall under.

@history_item n=24 marker=amd-yptdayxo
**amd-yptdayxo** (2026-10-26-codebase-health) — new Complexity ceiling: cognitive 15, read on the dev host alone with rust-code-analysis-cli 0.0.25 (a host tool no lockfile resolves), test functions counted, in no CI leg. `cognitive.sum` adds nested closures into their parent. This is the origin of decision/complexity-ceiling-host-only, which the `v040-08` boundary-audit reading rests on.

@history_item n=25 marker=amd-nsqxmuql
**amd-nsqxmuql** (2026-11-13-cognitive-load-corrective) — ~ decision/complexity-ceiling-host-only except: the engine's click handler is held below the ceiling, upstream's arms included, at the price of a hand re-apply when a sync edits it. This is the one named exception the `v040-08` audit reading must keep.

@history_item n=26 marker=amd-ihzt4hfv
**amd-ihzt4hfv** (2026-11-20-codebase-health) — ~ decision/complexity-ceiling-host-only why: ratified by the founder's F2, so upstream's functions over the ceiling are kept as upstream wrote them. This is part of the «kept by the founder's word» half of the `v040-08` claim's acceptance.

@history_item n=27 marker=amd-6eogcl6u
**amd-6eogcl6u** (2026-11-20-codebase-health) — + decision/upstream-oversize-files-kept (F1) and + decision/oversize-test-targets-kept (F4, no route entry for the split, which would move the target layout the CI legs read). These are the founder's rulings that the `v040-08` claim cites as kept by his word.

@history_item n=28 marker=amd-rx3se5ar
**amd-rx3se5ar** (2026-11-20-codebase-health) — + decision/upstream-test-clone-pairs-kept (F3): upstream's cdp-server and vibey-script duplicated test pairs are kept, which discharges the duplication watch. This is the third founder ruling behind the `v040-08` claim's acceptance.

@history_item n=29 marker=amd-kmiudnmn
**amd-kmiudnmn** (2026-11-07-mounteddata-buffered-render) — upstream's `task_polled_during_render_can_access_document` is ported as written, passes, and is insensitive to the borrow on Dioxus 0.7. The probe's two `…_spawned_acts_*` pins are the discriminating witness. This is the origin of decision/upstream-buffered-render-test-kept-as-written. A dependency move touching dioxus or `Cargo.lock` must keep the probe pins green under both layout modes.

@history_item n=30 marker=amd-jy34nxjf
**amd-jy34nxjf** (2026-11-07-mounteddata-buffered-render) — per-OS tallies were re-read on CI#42371051720. Rule: in a GitHub Actions job log the stderr `Running …` and stdout `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name after the ANSI strip. Use this when reading this chunk's CI runs, including the cancelled macOS/iOS runs the watch counts.

@history_item n=31 marker=amd-jef7zjyx
**amd-jef7zjyx** (2026-10-18-upstream-sync-determinism-and-traces) — moved the tallies straight to a newer CI run and named the skipped run in the body. Rule for later chunks: a wrap that does not re-read the tallies leaves the next one to attribute the gap. This chunk's take-up CI (CI#44955866206) was in progress, so its reading is owed.

@history_item n=32 marker=amd-d4acxs6q
**amd-d4acxs6q** (2026-10-24-a11y-re-read) — a per-job tally strips ANSI codes with `(\x1b|\^\[)\[[0-9;]*m`, because the log spells ESC as a literal `^[`. This is a measurement trap for reading this chunk's CI job logs.

@history_item n=33 marker=amd-6ah7rxit
**amd-6ah7rxit** (2026-10-17-upstream-sync-engine-debts) — blitz-tests' dev-dependencies test-that and test-that-macro moved 0.5.2 → 0.6.0, reviewed by hand at the bump. This is a precedent for recording a dependency version move in test-plan §3 when a test crate's dependencies move under this chunk's updates.
