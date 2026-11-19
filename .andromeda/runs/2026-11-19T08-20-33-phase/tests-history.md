@history_item n=1 marker=2026-10-05-as-built-baseline
**2026-10-05-as-built-baseline** (amd-k2zs36me, the local baseline reading) — set §9's "Local baseline": the dev profile, workspace default features, `--locked`, no `opt-level` rewrite; cold after `cargo clean`, warm on an immediate re-run; full-debuginfo figures (workspace tests 1632.88 s cold). Matters: this is the measuring method `decision/local-baseline` rests on and the precedent for this chunk's cold-`target/` before/after reading; the chunk's growth figures should be taken under the same conditions or say how they differ.

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (amd-aeuarcbq, baseline re-measured under line-tables-only debuginfo) — the dev profile moved to `debug = "line-tables-only"`, the cold total fell 2239.59 s → 161.36 s and `target/debug` read 32G, the test counts unchanged. Matters: the one prior debuginfo lever already pulled and its measured effect on size; the "less debuginfo for test binaries" lever starts from this profile, and 32G is the only stated earlier footprint of `target/debug`.

@history_item n=3 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (amd-dhibncze, CI legs through ci-leg.sh) — each linux leg runs `bash .github/scripts/ci-leg.sh {leg}`, the same command the host runs; every cargo leg `--locked`; fast/slow split; rust-cache saved on main and build/*; matrix tests windows/macos, builds ios/android; `test_ci_workflows.py` pins. Matters: origin of `decision/ci-legs-host-reproducible`, `decision/ci-fast-slow-split`, `decision/cargo-legs-locked`, `decision/ci-cache-save-branches` — a leg-invocation mend must keep the host and CI running the same leg command and go through the `test_ci_workflows.py` pins.

@history_item n=4 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (amd-uy5wnu36) — added the `audit`, `a11y` and `coverage` legs; coverage runs `cargo llvm-cov --workspace --locked --lcov --output-path target/coverage/lcov.info` then `cargo llvm-cov report --workspace --locked`; the test job's cache under `shared-key: workspace-test`, a11y restore-only. Trap: `cargo llvm-cov report` without `--workspace` exits 0 with an empty table. Matters: the coverage leg is one of the invocations to measure (llvm-cov's own build directory is an open premise), and `decision/shared-test-cache` / `decision/ci-shared-test-cache` tie the a11y and stand-repeat jobs to the test job's build — a mend changing the test leg's build changes what they restore.

@history_item n=5 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (amd-oaaulgav) — the root `Cargo.toml` `[profile.dev]` debuginfo stanza citation re-pointed (199-200; later 201-202 at 2026-10-07-driver-session), after an insert above it had left it one line off. Matters: a mend editing `[profile.*]` moves that citation; re-point it in the same pass.

@history_item n=6 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** (amd-2gzx73ga) — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, "in the package-alone build too", while pre-existing font tests rely on `system-fonts`. Matters: the root of `decision/font-assertions-on-system-fonts` and `decision/stand-pinned-real-font`; the `-p blitz-tests` and `--workspace` builds resolve different font features, one of the feature-set differences this chunk must measure and must not erase by unifying invocations.

@history_item n=7 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** (amd-2xzkhreo) — the agent-run 5-command contract: `boot` builds every blitz-tests test binary (`--locked --no-run`), state in `target/agent-run/{status.json, events.jsonl, run.log}` only, no daemon or env var; `test_agent_run.py` under a cargo shim. Matters: agent-run is one of the measured invocations; its `boot`/`run` cargo calls and its state set (`contract/agent-run-verbs/*`, `contract/agent-run-state/files`) are contract rows a mend may not change silently.

@history_item n=8 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (amd-ur7tfp4h) — the layout rule became "one test-target file per behaviour and one shared module that is no target" (`tests/common/mod.rs`), unit tests inline bar `#[path]` modules. Matters: origin of `decision/one-test-target-per-behaviour` and `decision/shared-test-modules-not-targets`; the "one test binary per leg" lever changes exactly this layout.

@history_item n=9 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (amd-lrluctc2) — session checks write their state directories under `target/tmp/`; traps: an `#[ignore]` host child blocks a bare `--ignored` run; a child's inherited libtest lines are counted by `agent-run.sh` as tests. Matters: `target/tmp/` is a growth source to measure and a sweep must not break running checks; merging targets into one binary changes how ignored host children re-execute their own binary.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (amd-s53lzeiq) — new §9 bullet "Engine features by runner": `cargo test --workspace` builds blitz-dom with `tracing` compiled in, `cargo test -p blitz-tests` and `-p seven_guis` do not; a green per-package run is not the workspace leg's green; a sink capture is counted by target. Matters: the measured fact behind the scope's feature-unification premise (the same test target built under two hashes) and the rows `decision/per-package-green-not-workspace-green`, `decision/sink-capture-counted-by-target`, `decision/engine-items-asserted-on-tracing-build`; a mend that unifies feature sets would change which code each check runs.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (amd-u7vpyh4t) — the fork keeps one target per file by the operator's decision (2026-10-10), carrying upstream's `all` target at `test = false`, pinned by `BlitzTestsTargetsTest` (no `autotests = false`, exactly one `[[test]]` table); 102 `.rs` files, 101 targets then. Matters: the "one test binary per leg" lever reverses a recorded operator decision and reddens `BlitzTestsTargetsTest` (`decision/blitz-tests-one-target-per-file`, `decision/one-test-target-per-file`, `decision/blitz-tests-manifest-difference`); taking it needs a new ruling and the pin changed through `test_blitz_tests_targets.py`.

@history_item n=12 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (amd-u6pbkyf3) — blitz-tests' blitz-dom dev-dependency gained the `autofocus` and `text-transform-icu` features. Matters: a dev-dependency feature set specific to the blitz-tests package, one of the ways its build resolves shared crates differently from other invocations.

@history_item n=13 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (amd-cqdq6a7d) — the workspace build resolves blitz-dom with 16 features and the per-package build with 10, `writing-mode` on in the workspace only, choosing which body of the bounds reader compiles; the workspace's integration-test executables 103 → 107. Matters: a measured instance of the second hash per invocation, with a feature count to compare against; with amd-pzdy3myb it is why each reader body needs its own build.

@history_item n=14 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (amd-pzdy3myb) — each of the bounds reader's two bodies now holds a check, its mutation controls red under its own build only, since a fix in one body reads green under the other build. Matters: origin of `decision/bounds-reader-bodies-each-checked`; dropping or unifying either the per-package or the workspace build would leave one body unchecked.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (amd-db7lxiqh) — the harness takes blitz-paint without default features; shots equal under both builds. Matters: another per-package feature selection to put in the hash map; also part of why the same paint crate may build twice.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (amd-w5bh6ujd) — upstream targets join as their own targets, `all.rs` gaining their `mod` lines and staying unbuilt; executables 123 → 128, none `all`. Matters: the unbuilt `all.rs` is the ready single-binary form upstream uses; the executable count is what the chunk's per-target binary measurement starts from.

@history_item n=17 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (amd-n5xqwtgz) — added the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure `stand-repeat` leg; stand-repeat restore-only. Matters: origin of `decision/per-crate-coverage-floors`, `decision/stand-checks-zero-failure-budget`, `decision/zero-failure-stand-repeat`, `decision/stand-repeat-three-rounds`; a mend must keep stand-repeat's three rounds per target and the floors green.

@history_item n=18 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (amd-xc3pms54) — the cold-agent pipe builds nothing and needs `target/debug/escher-session` (exit 3 if absent); the live run is fired once. Matters: the pipe adds no build of its own to measure, but a sweep of `target/` must not remove the binary it reads; the live run stays off the measurement (`decision/cold-agent-live-run-off-ci`).

@history_item n=19 marker=2026-10-16-file-splits
**2026-10-16-file-splits** (amd-dvdqody5) — unit-test modules moved out of line through `#[path]`, "moved out whole with test paths unchanged". Matters: the precedent for `decision/test-module-split-keeps-paths`; any test move the mend makes keeps test paths and assertions.

@history_item n=20 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** (amd-le75xddb) — the mutation tier recipe: cargo-mutants `--test-tool nextest` under the root `.config/nextest.toml` `mutants` profile, `--baseline=skip`, each unit's test command first read green on the unmutated tree; dev host only, never CI. Matters: one of the invocations to measure, where it writes being an open premise; `decision/mutation-runs-on-dev-host-only`, `decision/mutation-baseline-skipped` and `decision/mutation-runs-under-nextest-profile` bind any change to its profile.

@history_item n=21 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (amd-xpqvmmzd) — both coverage lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`. Matters: a change to the coverage leg's cargo invocation has to keep both regex lines (gate-checked by a count of 2 in `ci-leg.sh`) and `decision/keyboard-compat-coverage-exclusion`.

@history_item n=22 marker=2026-10-19-test-splits
**2026-10-19-test-splits** (amd-h4ip3qlm) — moving test modules into `*_tests.rs` `#[path]` files moved escher-driver's coverage reading (98.33 → 98.07 %); "A later test split should forecast a reading change, not none." Matters: a trap if the mend moves or merges test files; forecast the coverage reading's move against the floors.

@history_item n=23 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** (amd-nlavi4cc) — the test-data bootstrap measured: nothing seeded, fixtures are code compiled into each target by `agent-run.sh boot` (`cargo test --no-run`); no `run fixtures` selection. Matters: origin of `decision/test-data-is-code` and `decision/shared-fixtures-in-one-module`; how boot builds every target is part of the growth to measure.

@history_item n=24 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (amd-cixe5kae) — `stand-repeat` runs the `stand_*.rs` glob, then named targets as literal `--test` words on the same `cargo test -p blitz-tests --locked` line, then seven_guis' checks; pinned by `test_stand_repeat_leg_runs_three_rounds`; selection by the founder's «Список в скрипте». Matters: a per-leg-binary mend has to keep this membership (`decision/stand-repeat-three-rounds`), and the pin reads the `--test` words.

@history_item n=25 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (amd-qutqvdph) — `stand_requirement_sweep` gained a leg check reading `ci-leg.sh` for the glob and every named target. Matters: renaming or merging targets, or changing how the leg names them, reddens this check (`decision/requirement-witness-inside-stand-repeat`).

@history_item n=26 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** (amd-d4acxs6q) — the `a11y` leg names its fourteen `--test` words on one line, each asserted by `assertIn`; each file joining a leg keeps its legs; a per-job tally strips ANSI as `(\x1b|\^\[)\[[0-9;]*m`. Matters: `decision/a11y-leg-membership` and `decision/joining-file-keeps-its-legs` hold the leg's per-target membership; a per-leg binary must keep every file in every leg it ran in.

@history_item n=27 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (amd-472linzq) — `CapabilityStatementTest` pins `docs/capabilities.md`'s names and counts in the `ci-scripts` leg. Matters: the base of the picked capability-statement rows; with the 2026-11-18 entries it means a leg-membership change reaches the statement.

@history_item n=28 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (amd-6adlxvzj) — the 0.2.0 sweep test reads `ci-leg.sh` as one whole text by `contains`, so a target dropped from `stand-repeat` alone reads green while `a11y` still names it; membership held jointly with `test_stand_repeat_leg_runs_three_rounds`. Matters: a trap when restructuring leg invocations; the leg pin, not the sweep, catches a one-leg drop.

@history_item n=29 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (amd-ce7fplwt) — a new auto-discovered target runs in the workspace `test` leg and `run all` with no script line naming it, and in neither `stand-repeat` nor `a11y`. Matters: the per-target auto-discovery that the per-file binaries come from; a mend replacing it needs a rule for how new files join legs.

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-jwvcqtlb) — CI#41190771843 ended red on `Stand checks repeated` with linker bus errors at `Free space left: 5 MB`; a pinned free-disk step was added, its cure unproven. Matters: direct evidence that the build footprint already breaks a CI job; the chunk's growth number and mend bear on it, and `decision/free-disk-on-test-and-stand-repeat` stands beside it.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-tcnuzkmh) — every cargo leg `--locked` but `fmt` and `examples/wasm_hello`. Matters: the exact exception list of `decision/cargo-legs-locked` that the scope says stays invariant.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-23w5gd5l) — a macOS-only red through `apple-font-embolden`; the local witness `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`; cargo stops at the first failing target, so a red job's later targets go unread. Matters: the `embolden` invocation is a separate feature set (one more hash of blitz-tests), and with one binary per leg a failing test hides fewer and more checks differently.

@history_item n=33 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (amd-yx45xaju) — `fast` reads fmt → clippy → test → ci-scripts → embolden, embolden a leg with no CI job, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. Matters: origin of `decision/local-pre-push-gate`, `decision/embolden-leg-local-only`, `decision/embolden-local-witness`; every `fast` run builds blitz-tests under a second feature set, a probable growth mechanism.

@history_item n=34 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (amd-c3jjjsfi) — the matrix job's free-disk step was removed, pinned by `test_only_stand_repeat_frees_disk`. Matters: part of the free-disk history the CI rows rest on; read with amd-xy6zexlx.

@history_item n=35 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (amd-jy34nxjf) — in a GitHub Actions job log the `Running …` and `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name. Matters: `decision/tally-move-attributed-by-test-name`; if the mend merges binaries, the per-target tallies change shape and must be re-read by name.

@history_item n=36 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-brpalyad) — blitz-tests holds 161 `.rs` files, 160 test targets and the unbuilt `all.rs`, the last count written in the history. Matters: the scope's "185 files" premise must be re-counted against this; the chunk's binary count is measured, not copied.

@history_item n=37 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (amd-gkbc5lsj) — `decision/layout-tests-in-both-layout-modes` gained its exception: upstream targets kept as written, those running one mode named exceptions. Matters: the scope's boundary that a moved or merged test keeps its two layout modes rests on this row and its exceptions.

@history_item n=38 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (amd-knbkpfwu) — `decision/mutation-controlled-tests` gained: a control that removes a bound loosens it to twice the budget. Matters: every new check this chunk writes (a sweep, a leg pin) is controlled red-before-green under this row.

@history_item n=39 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** (amd-d7r7mqwr) — agent-run gained the `session` selection running `cargo test -p seven_guis --locked --test cli_flow`; `decision/test-data-is-code` now reads that boot builds blitz-tests alone and `run session`'s cargo call builds seven_guis. Matters: agent-run now builds two packages under separate invocations, both to measure; the rows `contract/agent-run-selection/session` and `contract/agent-run-verbs/run` bind any change.

@history_item n=40 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (amd-rlvpc7s2) — added `decision/engine-items-asserted-on-tracing-build`: a check asserts engine caused items only on a tracing build and runs on both builds. Matters: another check that needs both the tracing (workspace) and non-tracing (per-package) builds to exist, a reason one of the two hashes cannot simply go.

@history_item n=41 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (amd-lde2vbr3) — added `decision/blitz-dom-requests-through-stamping-constructor`: the clippy leg guards blitz-dom's request construction, then runs upstream blitz-dom's no-default-features line. Matters: the clippy leg's second invocation (`-p blitz-dom --no-default-features`) is one of the feature-set builds the scope lists as a possible re-hash.

@history_item n=42 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (amd-yfp5ovdp) — added `decision/count-and-rename-sweeps`: a test rename sweeps the old name held as a string literal (such as a requirement's witness), a count change sweeps numeric pins. Matters: if the mend renames or merges test targets, the witness strings in `stand_requirement_sweep` and the leg pins must be swept.

@history_item n=43 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (amd-b4vvhst7) — added `decision/harness-graph-tests-enable-accessibility`: a per-package test build leaves `accessibility` off, so the harness turns it on through its dev-dependency for the test build. Matters: one more per-package feature difference that gives a crate a test-only hash.

@history_item n=44 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (amd-xy6zexlx) — `decision/only-stand-repeat-frees-disk` replaced by `decision/free-disk-on-test-and-stand-repeat`: the default-features test job frees disk too. Matters: CI disk pressure had spread to the test job; this chunk's mend may let the step go or keep it, and its pin goes through `test_ci_workflows.py`.

@history_item n=45 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (amd-iriujbey) — added `decision/requirement-witness-inside-stand-repeat`: every release requirement names its witness inside the stand-repeat leg, pinned by function name. Matters: a stand-repeat restructure must keep every witness running inside it.

@history_item n=46 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (amd-m7fw5fjq, amd-zn44tekk, amd-aiakktz6) — the capability statement now pins the a11y leg's file count and stand-repeat's named-target count to `ci-leg.sh`. Matters: `decision/capability-statement-pinned`, `decision/capabilities-doc-pinned-to-schema` and `decision/capability-check-names-held-to-schema` mean any change to either leg's membership or named targets reddens `ci-scripts` until `docs/capabilities.md` restates it.

@history_item n=47 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (amd-mmdtifvl) — `decision/count-and-rename-sweeps` gained its exception: a test name holding a count word pinned as a witness string is kept by a count sweep. Matters: the sweep convention a test move or rename in this chunk follows.

@history_item n=48 marker=2026-11-18-cold-agent-live-run
**2026-11-18-cold-agent-live-run** (amd-ac7ti3es) — added `decision/cold-agent-live-run-fired-once`: the live run held to one firing under the 3 USD cap. Matters: measuring the cold-agent pipe's growth uses its offline contract tests or a fake, never a fresh live run.
