@history_item n=1 marker=2026-10-05-as-built-baseline
**2026-10-05-as-built-baseline** — measured dev-host build environment: recorded the flake's rust-bin "1.90.0" pin below the workspace `rust-version` "1.91.0", the flake unused on the dev host (host stable 1.99.0, no `rust-toolchain*`, no `.cargo/config*`). Origin of decision/flake-rust-pin-matches-rust-version and decision/nix-flake-dev-shell; the dev-host facts are where the chunk's `CI=true` recompile measurement (no `.cargo/config`, no toolchain file) starts.

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (fork CI through one leg script) — created `ci-leg.sh` and its legs (msrv as `cargo +1.91 build`, every cargo leg `--locked` but wasm_hello), the log tee to `target/ci-logs/{leg}.log`, exit 2 on an unknown leg, `fast` as the local pre-push gate, the fast/slow `needs` split, no opt-level rewrite, linux tested by `fast` and the matrix testing windows/macos and building ios/android, `Swatinem/rust-cache` on every compiling job saved on `main` and `build/*`, failure-only `ci-log-{job id}` uploads. Founder's concern: CI must not rebuild everything each run. Rests under every ci-leg row, decision/ci-fast-slow-split, decision/linux-tested-by-fast-leg, decision/ci-cache-save-refs, decision/rust-cache-on-compiling-jobs; the cancelled-runner watch sits on the matrix jobs this shaped.

@history_item n=3 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (one dev-profile debuginfo level for host and CI) — `[profile.dev] debug = "line-tables-only"`, inherited by `test`, chosen over a CI-only `CARGO_PROFILE_DEV_DEBUG` so the dev host and CI share one level with no env-var divergence. Precedent and constraint for the `CI=true` recompile carry (cry-sqod24yl): the project deliberately keeps no CI-only profile switch, so a CI-driven hash change must come from elsewhere; decision/dev-line-tables-debuginfo, decision/one-debuginfo-level.

@history_item n=4 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (upstream-only signing, WPT and dispatch; the fork's cache budget) — every job reaching Signed Builds/WPT environments, warp runner, Pages deploy or the `update-results` dispatch carries a `DioxusLabs/blitz` repository guard; the fork's Actions cache measured at ≈9.73 GB of 10 GB, so a lockfile or toolchain change mints new keys and evicts. Holds decision/upstream-only-workflows, decision/upstream-ci-resources-upstream-only, contract/wpt-repository-dispatch/update-results; trap for this chunk: any `Cargo.lock` move (rusqlite, rav1e, zip, accesskit) re-keys every rust cache.

@history_item n=5 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `audit` leg (`cargo deny --locked check advisories`, root `deny.toml`), `a11y` and `coverage` legs, pinned every `ci.yml` `uses:` to a 40-hex SHA with its ref as a trailing comment and a named toolchain, installed pinned cargo-deny 0.20.2 / cargo-llvm-cov 0.9.1 through `taiki-e/install-action`, workflow-level `permissions: contents: read`, rust-cache on every compiling job but `coverage`, `a11y` restore-only. Origin of decision/dependency-and-licence-audit, decision/cargo-deny-and-llvm-cov-tools, decision/ci-read-only-permissions, decision/test-cache-restore-only; a new skia fetch step must follow the SHA-pin and read-only rules.

@history_item n=6 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (shared test cache, the coverage path, the cache budget re-measured over) — `workspace-test` shared key restored by `a11y` with `save-if: false`; after a re-keyed lockfile the cache stood over budget (10.72 GB), evicting the legs that restore last; `deny.toml` is not registered under Filesystem (below the registry's grain; Build system names it). Trap for every lock move this chunk makes; and the precedent that `deny.toml` edits land in Build system, not Occupied Resources.

@history_item n=7 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — fork CI's package install became one bounded, retried script `.github/scripts/apt-install.sh` (3 attempts, `BOUND=120`, `GRACE=10`, pause 10 s), no `ci.yml` line calling `apt-get`, pinned by `InstallScriptTest`; the operator chose one script over per-step loops or a third-party retry action. Origin of decision/one-apt-install-script, decision/bounded-package-install, decision/bounded-retried-package-install and the apt-install contracts; a skia CI step that needs a package must go through it and add no action beyond the stated exceptions.

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (merge base, Parley's pin form and the merged dependency set) — took upstream's dependency set whole with no `cargo update`: accesskit_unix 0.23.0 → 0.24.0, accesskit_android 0.9.0, ICU4X, the `ci-scripts` leg needing Python ≥ 3.11; the coupling rule of [Dependency pinning] (parley, skrifa, vello move together) unchanged; accesskit "0.25" the manifest string. Background for the zbus family (reached through accesskit_unix 0.24 ← accesskit_xplat) and its coupling to the AccessKit 0.25 pin.

@history_item n=9 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (taffy from the registry, the beta.3 pins) — workspace version and the nine in-repo `=` pins moved together; the coupled pins, the winit exact beta and the commented-out patch blocks stand; trap for the next sync that the CLAUDE.md coupled-pins warning no longer names a taffy git rev. Rests under decision/workspace-lockstep-versions, decision/lockstep-package-versions, decision/winit-exact-beta-pin — the ttf-parser ignore rides the coupled winit beta.

@history_item n=10 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `audit` became `cargo deny --locked --workspace --all-features check advisories licenses` (founder: «Аудит всего workspace», «Проверка лицензий»), clippy gained upstream's blitz-dom line, `coverage` gained `coverage_floor.py`, `stand-repeat` leg and job added, `apt-install.sh` gained the bounded `dpkg --configure -a` repair before a retry. Exact form of contract/ci-leg-legs/audit, decision/cargo-deny-audit, decision/ci-quality-gates, decision/package-install-repair-before-retry the chunk's advisory work runs against.

@history_item n=11 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — the mutation tier recorded as host-only (cargo-mutants 27.1.0, cargo-nextest under `.config/nextest.toml`'s `mutants` profile), neither tool lockfile-resolved, never in CI. Sits beside decision/cognitive-complexity-ceiling as the dev-host-only code-quality pattern the v040-08 codebase-health claim reads.

@history_item n=12 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the bump crate leaves the workspace) — blitz packages versioned together through the one workspace version and in-repo `=` pins, with no in-repo release tool since upstream deleted `bump` (toml_edit et al. left the lock). Origin of decision/lockstep-package-versions' current wording and absence/in-repo-release-tool.

@history_item n=13 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (keyboard-types 0.8 surfaces) — the `coverage` leg's `--ignore-filename-regex 'keyboard_compat\.rs$'`, pinned by `test_coverage_leg_reaches_the_floor_on_each_crate`, floors unchanged; keyboard-types 0.7 and 0.8 both stand in the graph. Shapes contract/ci-leg-legs/coverage; precedent that a dependency duplicate is tolerated in the graph while an upstream move waits.

@history_item n=14 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (stack, conventions and project intent) — tokio "1.42" resolved 1.54.1 past RUSTSEC-2026-0386 (an advisory closed by a lock-compatible move), mimalloc in the browser's default and the WPT runner's direct `mimalloc` named the one exception to `[workspace.dependencies]`, the flake's rust-bin moved "1.90.0" → "1.91.0" to match `rust-version`. Precedent for advisory closure by a semver-compatible lock move; origin of decision/workspace-dependencies' except and decision/flake-rust-pin-matches-rust-version as standing.

@history_item n=15 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host cognitive-complexity reading) — `rust-code-analysis-cli` 0.0.25 on the dev host alone, never in CI, reading each function against a ceiling of 15; the operator kept 15 as no project-wide convention (85 functions still over it, upstream's among them). Origin of decision/cognitive-complexity-ceiling; the v040-08 claim's acceptance (the founder keeping over-ceiling functions as upstream's) rests on this reading.

@history_item n=16 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the default branch GitHub renders the front page from) — recorded the repository's default branch as `build/escher-0.3.0`, switched on the founder's own word and read back by `gh repo view … defaultBranchRef`. Directly bears on CARRY cry-g3etodij: `.claude/docs/workflow.md:8` still names `build/escher-0.3.0` while GitHub now reads `build/escher-0.4.0`, and no master states the current branch — the landing is the operator's slot, not a code step.

@history_item n=17 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the publish workflow's dioxus-cli) — the upstream-only, repository-guarded publish workflow moved to dioxus-cli `0.8.0-alpha.3` (or a git install at a pinned rev); its sibling entry recorded the take with every manifest, the lock and `deny.toml` byte-identical. Holds decision/publish-workflow-upstream-only and decision/browser-publish-upstream-only — the browser app (and its skia default) builds no fork bundle.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the free-disk steps and what they show) — `Stand checks repeated` gained SHA-pinned `jlumbroso/free-disk-space` after an out-of-disk red, explicitly not shown to be the cure (the red job alone stood on an older runner image); a matrix copy read skipped on every row. Precedent for a CI flake answered by a watch with a stated retire condition rather than a claimed cure — the cancelled-runner watch res-q2tllspz follows the same shape.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (pulldown-cmark resolved at 0.13.5) — RUSTSEC-2026-0414 closed by a semver-compatible move in `Cargo.lock` alone with no coupled pin, `rdme` never setting `ENABLE_WIKILINKS`; the operator's call, not put to the founder. Direct precedent for this chunk's advisory rows: a lock-only compatible fix is the operator's, a non-semver move (rusqlite ≥ 0.41) is not.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the legs that do not run locked) — corrected the locked-legs claim to every cargo leg but `fmt` (`cargo fmt --all --check`) and `examples/wasm_hello`. Current form of decision/cargo-legs-locked and decision/locked-cargo-legs, which any version move here must pass.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (no local gate builds `font-embolden`) — a local gate read green over a defect only macOS CI showed; `--features blitz-paint/font-embolden` named the local witness, owned by a CARRY. Background for decision/font-embolden-local-witness and the lesson that local gates can miss what only a hosted macOS job shows (the cancelled-runner watch's platform).

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` became fmt → clippy → test → ci-scripts → embolden, the `embolden` leg with no CI job, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. The `fast` leg the `CI=true` recompile carry runs; its five steps' build units are what a `CI=true` run re-hashes.

@history_item n=23 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`matrix_test`'s free-disk step) — the skipped matrix free-disk step removed, `test_only_stand_repeat_frees_disk` added; the stand-repeat step's watch read retired at 3 of 3 green, explicitly not as a cure. Precedent for retiring a CI watch by its stated green-run condition (the cancelled-runner watch's «3 green runs» clause).

@history_item n=24 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the advisory fix, pulldown-cmark 0.13.6 and the deny.toml ignore) — pulldown-cmark 0.13.6 past RUSTSEC-2026-0433 by a lock-only move, and `deny.toml`'s ignore list gained RUSTSEC-2026-0429 (zip 2.4.2, `extract` never called), on the operator's fix directive, classed a widening; the Build system item's `deny.toml` citation moved. Direct origin of one of the zip-family ignores this chunk must close or state as a wait; the per-ID ignore with its reason is decision/dependency-and-licence-audit's form.

@history_item n=25 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard re-measured, its fix routed to a port) — residuals now live as `residual` rows in the store (no `.andromeda/residuals.md`); Dioxus 0.7.3 held, Stylo/Taffy pins and no git dependency standing. The chunk's advisory items are residual rows (res-j4x7iiqc, res-kdhnfj5p, res-uqt2yd2y, res-p73cueyv, res-h7acz56c, res-q2tllspz) closed through the store, not a file.

@history_item n=26 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Infrastructure Patterns) — decision/pinned-actions widened: every `uses:` of every workflow pinned to a 40-hex SHA with its ref as a trailing comment, every rust-toolchain step naming its toolchain, the pin check reading every workflow file. Constraint on any new skia fetch step or action the chunk adds to CI.

@history_item n=27 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Established Decisions: no-git-dependencies; Stack: mutation tier) — `~ decision/no-git-dependencies` gained the taffy-only git exception, a git Taffy still passing the lock, audit and MSRV legs; `+ decision/mutation-tier-excludes-demo-shells` ruled two test-less units out of the mutation tier, read by the code audit only from an architecture row. The audit/MSRV legs this chunk keeps green carry the git-Taffy exception; the codebase-health audit reads its exclusions from rows.

@history_item n=28 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** — `~ decision/no-git-dependencies` rule rewritten (every manifest dependency a registry version, taffy the lock's one git source under the exception) and decision/taffy-layout moved to upstream's git source pinned to one revision. Current form of the no-git rule any version move here must respect.

@history_item n=29 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Stack and Technologies; Conventions) — `~ decision/anyrender-rendering-abstraction` and `~ decision/window-renderer-chosen-at-compile-time` moved to AnyRender 0.15 (vello-gpu replacing vello-hybrid, skia and skia-raster-base still window renderers); `~ decision/in-repo-default-features-off` names seven_guis' default `vello-gpu`. The renderer set this chunk's skia archive work and the zip-via-`anyrender_serialize` advisory sit on; per-crate defaults (decision/per-crate-default-features) unchanged for skia.

@history_item n=30 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Established Decisions; Standard Contracts; Occupied Resources) — `− decision/skrifa-matches-parley-vello`, `− decision/font-deps-match-renderers` → `+ decision/skrifa-matches-parley` after AnyRender 0.15 brought a second skrifa/read-fonts version; `− decision/only-stand-repeat-frees-disk` and `− decision/free-disk-space-only-in-stand-repeat` retired as `test-features-default` gained the free-disk step, the fact's home now test-plan. Precedent that the graph tolerates duplicate font crates under a renderer move, and that CI-job disk facts live in test-plan, not architecture.

@history_item n=31 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement`: the pin also holds the a11y leg's file count and stand-repeat's named-target count to the CI leg runner. Any `ci-leg.sh` change this chunk makes that moves those counts reddens the capability-statement pin (decision/capability-statement-matches-schema).

@history_item n=32 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — created `docs/capabilities.md` beside upstream's docs, its five sections and `CapabilityStatementTest` in the `ci-scripts` leg (eight test files then). Origin of decision/capability-statement, decision/capabilities-doc-beside-upstream and contract/capability-statement/sections, the statement where a stated wait or a measured limit of this chunk would surface.

@history_item n=33 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (Occupied Resources) — `~ decision/run-state-areas-local-only` gained the dev-host target sweep (removes stale debug build units only), `+ name/other/ci`, `+ name/other/target-sweep-py`, `+ name/other/target-sweep-ps1`: `CI` became an env name the project reads. Directly under CARRY cry-sqod24yl (`CI=true` recompiles, 48 GB measured at this chunk's evidence): the carry's «project source reading no `CI`» must be re-verified against this registered `CI` read.

@history_item n=34 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (Standard Contracts; Stack and Technologies) — `~ contract/ci-leg-legs/fast` (after a green verdict, with `CI` unset or empty, runs the target sweep, its exit discarded), `~ contract/ci-leg-legs/embolden`, `~ decision/windows-entries-pass-through`, `+ contract/target-sweep-exit-codes/0…3`, `~ decision/local-pre-push-gate` (ends by the sweep, skipped under CI). The `CI` branch inside `ci-leg.sh fast` is the first suspect for the recompile mechanism, and the gate rows a fix must keep.

@history_item n=35 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Stack and Technologies) — Parley 0.13, Stylo 0.24, blitz-net's reqwest on rustls with native-tls vendored on Android (founder's W3), and `~ decision/one-apt-install-script` except gaining the Android job's SHA-pinned `nttld/setup-ndk`. The sync immediately before this chunk: the dependency graph the advisories are read against, and the precedent that an install outside the one script is admitted only as a named except.

@history_item n=36 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Established Decisions) — `~ decision/winit-exact-beta-pin` moved `=0.31.0-beta.3` → `=0.31.0-beta.4`. The coupled pin through which `ttf-parser` 0.25.1 (RUSTSEC-2026-0192) is reached; its ignore can drop only with a winit move.

@history_item n=37 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Project Intent) — `− decision/deferred-stylo-dioxus-bumps` → `+ decision/dioxus-0-8-move-held`; `− decision/upstream-taken-by-cherry-pick` → `+ decision/upstream-sync-by-two-parent-merge`. Dioxus stays 0.7.3: a version move here may not ride a Dioxus 0.8 bump, and the upstream syncs are out of this chunk's scope.

@history_item n=38 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Occupied Resources; Inherited Defaults) — `~ decision/one-local-unix-listener`, `~ decision/session-socket-one-listener`, `~ decision/driver-session-unix-socket` excepts and `~ absence/served-network-port`, `~ absence/network-listener` scoped to default builds for the opt-in devtools CDP server. The boundary the chunk's «no new socket, port, listener» rule holds against; a skia fetch is an outbound supply-chain surface, not a listener.

@history_item n=39 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the scopes, the tests and the stack) — `png` 0.18.1 taken by escher-driver and blitz-tests with no package entering `Cargo.lock`, no base64 crate; the harness paints through `anyrender_vello_cpu` (no skia). Precedent for reusing in-graph crates rather than adding packages, and that the stand's frames come from vello-cpu, so skia's archive touches the browser app and WPT runner, not the driver.
