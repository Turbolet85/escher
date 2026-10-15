# arch extract

## Relevance
partial — the chunk lands no crate, contract or resource of its own; arch governs where its gates live (the `ci-leg.sh` leg contract, the CI/CD job shape, the CI-installed tools), what the "driver crates" set may be drawn from, the registered paths and env vars it must not widen silently, and the recorded merge surfaces CARRYs 11–12 own.

## Constraints
- A gate is a leg of `.github/scripts/ci-leg.sh`: one command shared by the dev host and the CI job, every cargo leg `--locked`, output tee'd to `target/ci-logs/{leg}.log`, the exit taken from the leg command, exit 2 on an unknown leg. The `coverage` leg's contract is `cargo llvm-cov` lcov to `target/coverage/lcov.info` followed by its per-file report. A coverage floor or a repeat runner extends this contract and does not bypass it. The contract is pinned by `test_ci_workflows.py`, so a changed leg moves that pin (per arch §Standard Contracts → CI contracts).
- Fork CI's job shape is fixed (per arch §Infrastructure Patterns → CI/CD):
  - `fmt`, `clippy`, `test-features-default` and `ci-scripts` carry no `needs`, and every other job `needs` all four.
  - Permissions are set once at workflow level as `contents: read`, with no job-level grant.
  - Every `uses:` is SHA-pinned with its ref kept as a comment.
  - Each leg job uploads its own failure log.
  - The `coverage` job carries no cache step and alone uploads `coverage-report`.
  - The matrix tests windows and macos with `--all --tests`, only builds ios and android, and tees its output to `target/ci-logs/matrix-{platform}.log`. Linux is tested by the fast `test` leg.

  A new job, or a changed one, must keep this shape.
- The CI-installed tools are `cargo-deny@0.20.2` and `cargo-llvm-cov@0.9.1` (with `llvm-tools-preview`). Code quality records the audit as `cargo deny --locked check advisories` over the root `deny.toml`, and coverage as a report "with no threshold". A floor or an audit-reach mechanism either works within these pinned tools or is a recorded tool addition (per arch §Stack and Technologies → CI/CD and → Code quality rows).
- The "driver crates" set may be drawn only from existing workspace members. The members are listed under `packages/`. `escher-driver` is recorded as depending on `blitz-test-harness`, `dioxus-native-dom`, `tracing`, `serde_json` and `png`. That dependency record is the arch-side input to naming the set (per arch §Existing Scopes; per arch §Infrastructure Patterns → Directory structure).
- No new env var beyond the registered `RUST_LOG`, `WPT_DIR`, `PAINT_TREE_BENCH_HTML` and the CI script's `GITHUB_*` / `PR_NUMBER` / `RUN_URL`. No new workspace crate. No new artifact path beyond `target/ci-logs/` and `target/coverage/` without an §Occupied Resources registration (per arch §Occupied Resources → Environment variables · → Names · → Filesystem).
- The fork's Actions cache has a 10 GB per-repository budget with LRU eviction. It was already measured over that budget once, and `audit` and `coverage` carry no cache step. Any job a flakiness or coverage gate adds is read against that budget (per arch §Occupied Resources → CI infrastructure).
- Release metadata has three recorded readings:
  - The root manifest's `homepage` and `repository` are recorded at `Cargo.toml:38-39` as `https://github.com/dioxuslabs/blitz` (per arch §Project Intent → Product type).
  - Workspace-package metadata is inherited by member crates (per arch §Stack and Technologies → Workspace package row).
  - `debug_timer` declares its own `homepage` and `repository` with the same upstream URL (per arch §Occupied Resources → Names).

  Whether CARRY 2 reaches `debug_timer` too is P4's question.

## Patterns to follow
- An existing leg can be widened without a new job, without a required status check and without a `ci.yml` change. The `a11y` leg is the precedent: it went from three to six tests on the founder's F4 and was pinned by `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`. That is the arch-side form of "widen an existing gate", as opposed to "add a new gate" (per arch §Standard Contracts → CI contracts).
- An unhandled case is recorded as a named limit inside its contract. The `apt-install.sh` contract states that "a bound that fires inside `apt-get install` can leave dpkg interrupted, which the script does not handle and no run has shown". The nine `run:` call sites are recorded with the guarded ninth unreached. CARRY 5 reads against these two records (per arch §Standard Contracts → CI contracts; per arch §Infrastructure Patterns → CI/CD).
- A gate value enters the plan with its evidence, in the form "as measured at escher-0.1.0/chunks/{chunk}/evidence/{file}". The CI/CD and Code quality rows are built that way. A floor or budget is recorded beside the run it was read on (per arch §Infrastructure Patterns → CI/CD).
- The two layout modes are one pipeline, and pipeline checks run in both. A stand check's repeat behaviour is read across `for incremental in [false, true]`, never in one mode alone (per arch §Design Philosophy → One incremental pipeline, safe identities).
- The upstream merge surfaces are recorded per change against upstream `main` at `434e0661`:
  - range: `node/element.rs`, `node/mod.rs`, `layout/construct.rs`, `mutator.rs`, `events/keyboard.rs`, `events/pointer.rs`, `form.rs` and the new `node/range.rs`;
  - keyboard activation: `events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs` and `node/element.rs`.

  Both are owned by CARRYs on this entry. The merge base is recorded under Upstream sync. CARRYs 11–12 restate these files and read the conflict risk against upstream's current `main` (per arch §Established Decisions → [Scrolling and input]; per arch §Project Intent → Upstream sync).

## Anti-patterns to avoid
- Never bump one side of a coupled pin, add a git dependency or drop `--locked` while closing the audit reach. The pin couplings are html5ever family ↔ stylo web_atoms, skrifa ↔ parley/vello, svgtypes ↔ usvg, and winit's exact beta (per arch §Established Decisions → [Dependency pinning]).
- Never add an action `uses:` that is not SHA-pinned, or a `dtolnay/rust-toolchain` step that does not name its `toolchain` (per arch §Infrastructure Patterns → CI/CD).
- Never add a new listener, port, env var or workspace crate silently. Each one is an §Occupied Resources registration first (per arch §Occupied Resources → Network ports and listeners · → Environment variables).

## Contract bindings
- The arch CI contracts bind to test-plan's gates. Arch §Standard Contracts → CI contracts (the leg grammar, the `coverage` leg) ↔ test-plan §9 (coverage leg) / §10 (gate values), with both pinned by `.github/scripts/test_ci_workflows.py`.
- The arch audit gate binds to security-plan's dependency rules. Arch §Infrastructure Patterns → Build system (the cargo-deny advisory gate over `deny.toml`) ↔ security-plan §Dependencies, which covers the audit's reach and the open licence-gate question (CARRYs 1 and 6).
- The arch driver-session contract binds to obs-plan. Arch §Standard Contracts → Driver session (the `command` span's `busy` field, recorded as "covered by its unit test only") ↔ obs-plan, which covers span field readings (CARRY 4). It also binds to test-plan through `stand_settle`'s not-settled fixtures (per arch §Standard Contracts → Test harness: `SETTLE_PASS_LIMIT`, `Busy`).
- The arch command-line contract binds to test-plan's named checks. Arch §Standard Contracts → Driver session (the command line drops a write to a closed pipe without panicking, and the client reads under a 30 s bound per read) ↔ test-plan's named checks (CARRY 10).

## Acceptance criteria contributions
- The coverage floor is enforced inside the `coverage` leg of `.github/scripts/ci-leg.sh`, run identically on the dev host and in CI, `--locked`, and logged to `target/ci-logs/coverage.log`. The leg's change is reflected in `test_ci_workflows.py`, and `ci-scripts` stays green (per arch §Standard Contracts → CI contracts).
- Any CI job added or changed keeps:
  - the fast/slow `needs` split;
  - workflow-level `contents: read` with no job-level grant;
  - SHA-pinned `uses:`;
  - a per-leg failure-log upload.

  These are asserted by the `ci-scripts` leg (per arch §Infrastructure Patterns → CI/CD).
- The chunk adds no env var, workspace crate, listener or artifact path outside the registered set. Pass check: a diff of the root `Cargo.toml` `[workspace] members` and of the source env reads shows nothing new (per arch §Occupied Resources → Environment variables · → Names · → Filesystem).
- The dependency-pin couplings and the lockfile policy hold after the audit-reach change. `Cargo.lock` is committed, no git dependency appears, and every cargo leg stays `--locked` (per arch §Established Decisions → [Dependency pinning]).
