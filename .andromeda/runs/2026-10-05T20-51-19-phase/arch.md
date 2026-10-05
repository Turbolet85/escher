# arch extract

## Relevance
relevant: the chunk changes the CI/CD pattern, the build profiles and the CI infrastructure registry that arch records (§Infrastructure Patterns → CI/CD, §Established Decisions [Build profiles], §Occupied Resources → CI infrastructure).

## Constraints
- The workflow set, triggers, per-ref concurrency and job roster the chunk reworks are the ones recorded in §Infrastructure Patterns → CI/CD. Per that section, `Swatinem/rust-cache@v2` saves only on `refs/heads/main`, so a cache added for build-branch runs must decide where the cache is saved. Whether a build-branch run ever gets a warm cache under that save rule is research's question.
- A reduced dev/test debuginfo level is a new profile entry next to the named profile set in §Established Decisions [Build profiles] (`production`, `p2`, `small`, `small-panic`, `tiny`, with their per-package opt-level overrides). §Infrastructure Patterns → Deployment model requires `production` to stay the bundle profile. §Infrastructure Patterns → CI/CD says Ubuntu jobs rewrite `opt-level = 2` to `0` before building. Whether that rewrite also hits a new profile stanza is research's question.
- The MSRV job runs only `cargo build`, so dev-dependencies are not required to build at the MSRV (per architecture §Established Decisions [MSRV]). CI guarantees latest stable and keeps the MSRV check (per architecture §Design Philosophy, "Radically modular, embeddable engine"). Whatever the fast/slow split does, it keeps this job's command and toolchain.
- Signing and upstream-publishing infrastructure is registered in two places. §Occupied Resources → CI infrastructure lists the GitHub environments "Signed Builds" and "WPT" and the `warp-ubuntu-latest-arm64-16x` runner with the "warpbuild" cache provider. §Standard Contracts → CI contracts lists the Pages deploy and the `repository-dispatch` to `DioxusLabs/blitz-wpt-results`. Excluding signing-secret jobs and classifying the WPT workflows must reckon with all of these. Whether the fork can reach the warp runner label at all is research's question.
- A new env var, workspace crate or other resource needs an §Occupied Resources registration first. §Occupied Resources → Environment variables currently lists only `WPT_DIR`, `PAINT_TREE_BENCH_HTML` and the CI script's `GITHUB_REPOSITORY`/`PR_NUMBER`/`RUN_URL`/`GITHUB_STEP_SUMMARY`.
- Build-configuration work must not touch the coupled dependency pins (html5ever family ↔ stylo, skrifa ↔ parley/vello, svgtypes ↔ usvg, taffy/parley git revs, exact winit beta), per architecture §Established Decisions [Dependency pinning].
- The fmt and clippy gates stay as recorded in §Conventions (Formatting and lints) and §Inherited Defaults (Code quality). Per §Infrastructure Patterns → CI/CD, the docs job is nominal (bare `cargo doc` over the lib-less root package). Making it real belongs to "CI gate legs", not this chunk.

## Patterns to follow
- Extend the existing caching pattern: `Swatinem/rust-cache@v2` in the matrix test job, plus the `wpt.yml` variant with the "warpbuild" provider (per architecture §Infrastructure Patterns → CI/CD; §Occupied Resources → CI infrastructure).
- Failure artifacts follow the artifact patterns already in use: `actions/upload-artifact@v7` (per architecture §Infrastructure Patterns → Deployment model) and the named WPT artifacts `wpt-report.json.zst` / `wpt-diff` (per architecture §Standard Contracts → CI contracts).
- The local-reproduction surface targets the dev host recorded in §Stack and Technologies (Build environment row): Arch packages in place of CI's `libfontconfig1-dev` and build-time python3, the flake unused, no `rust-toolchain*` file and no `.cargo/config*`. A local leg runs the same command the workflow step runs (per architecture §Infrastructure Patterns → CI/CD).
- The CI-script test leg (`.github/scripts/test_wpt_diff_to_pr.py`, `unittest`) is a host-reproducible leg as it stands (per architecture §Conventions, Tests; §Stack and Technologies, CI/CD row).

## Anti-patterns to avoid
- Adding an env var, `.cargo/config*`, `rust-toolchain*` or workspace crate without an amendment. §Occupied Resources (Environment variables / Names) and §Stack and Technologies (Build environment) record their current state or absence, so each addition is a registration.
- Letting an upstream-targeted step fire from the fork's build branch: the "Signed Builds" environment, the Pages deploy, or the `repository-dispatch` to `DioxusLabs/blitz-wpt-results` (per architecture §Occupied Resources → CI infrastructure; §Standard Contracts → CI contracts).
- Altering `production` or the other named profiles, or bumping one side of a coupled pin, as a side effect of build tuning (per architecture §Established Decisions [Build profiles] / [Dependency pinning]).

## Contract bindings
- arch §Infrastructure Patterns → CI/CD ↔ tests: the fast/slow leg split and the local-reproduction commands bind to the test plan's CI legs and its local baseline. The baseline timings for cold vs warm runs are what a cache or profile change is measured against.
- arch §Occupied Resources → CI infrastructure ("Signed Builds") ↔ security: signing material lives only in GitHub Actions secrets and is removed in an `always()` step. Excluding signing jobs on the fork must keep that pairing intact where the jobs still run.
- arch §Established Decisions [Build profiles] ↔ tests: a dev/test debuginfo change alters every local and CI test build, including the `for incremental in [false, true]` pipeline tests. Debuggability of failure artifacts (backtraces) depends on the debuginfo level chosen.

## Acceptance criteria contributions
- Root `Cargo.toml` changes, if any, are confined to `[profile.*]` entries: the `[workspace.dependencies]` pins are unchanged and `production` is byte-identical (per architecture §Established Decisions [Build profiles] / [Dependency pinning]).
- The MSRV job still runs `cargo build` only, on Rust 1.91 (per architecture §Established Decisions [MSRV]).
- A push to the build branch reaches no "Signed Builds" environment job, Pages deploy or `repository-dispatch` to `DioxusLabs/blitz-wpt-results` (per architecture §Occupied Resources → CI infrastructure; §Standard Contracts → CI contracts).
- Every env var, runner label, environment or crate the chunk introduces appears in an §Occupied Resources amendment at wrap; none is added silently (per architecture §Occupied Resources).
