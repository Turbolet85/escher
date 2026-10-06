# architecture — amendments

One entry per amendment to `architecture.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-as-built-baseline — measured dev-host build environment
**Section:** §Stack and Technologies (Build environment row)
**Change:** the row now states that the flake's rust-bin "1.90.0" pin sits below the workspace `rust-version` "1.91.0" its own comment says to keep in sync with, and carries the dev-host reading as measured at the chunk's `evidence/baseline.md` (Omarchy 4.0.4, Arch-based, 2026-10-05): the flake is not used; host stable rustc/cargo 1.99.0; no `rust-toolchain*` file, no `.cargo/config*`; Arch packages fontconfig 2.18.3, openssl 3.6.4, pkgconf 3.0.7, python 3.14.7 stand in for CI's `libfontconfig1-dev` and build-time python3.
**Why:** the baseline is the first verified dev-host build; host-tool versions are readings on that host, not pins — no lockfile resolves them.
**Ref:** .andromeda/runs/2026-10-05T20-35-18-wrap/

## 2026-10-05-fork-ci-reached — fork CI on the build branch through one leg script
**Section:** §Stack and Technologies (CI/CD row · Code quality row) · §Conventions (Formatting and lints) · §Standard Contracts (CI contracts) · §Occupied Resources (Filesystem) · §Infrastructure Patterns (CI/CD) · §Inherited Defaults (Code quality)
**Change:** was ci.yml on pull_request + push to `main` / `v0.*`, inline cargo commands without `--locked`, an `opt-level = 2` → `0` rewrite before building, a matrix testing windows/macos/linux, rust-cache only on the matrix and saved only on `refs/heads/main`; now:
- trigger adds `build/**`; `fmt`, `clippy`, `test-features-default`, `ci-scripts` carry no `needs` and every other job, the matrix included, `needs` all four;
- each of the nine linux jobs runs `bash .github/scripts/ci-leg.sh {leg}` — legs fmt · clippy · test · ci-scripts · build · msrv (`cargo +1.91 build`) · counter · wasm · doc · fast — every cargo leg `--locked` but `examples/wasm_hello` (no `Cargo.lock`); the script tees merged output to `target/ci-logs/{leg}.log` (truncated per leg), exits with the leg's status, exits 2 on an unknown leg; `fast` (fmt → clippy → test → ci-scripts) is the local pre-push gate; clippy is `cargo clippy --workspace --locked -- -D warnings`, the doc job bare `cargo doc --locked`;
- no opt-level rewrite; the matrix tests windows and macos, builds ios and android, `--locked`, tee'd to `target/ci-logs/matrix-{platform}.log`; linux is tested by the fast `test` leg;
- `Swatinem/rust-cache@v2` on every compiling ci.yml job, saved on `main` and `build/*` (publish and WPT caches still main-only);
- every leg job uploads its own log `if: failure()` as `ci-log-{job id}`, 7 days, only from `target/ci-logs/` — registered as a CI contract and a Filesystem path;
- §Stack adds the bash leg runner, `test_ci_workflows.py` with PyYAML (Ubuntu `python3-yaml`, ensured by the `ci-scripts` job), and the tag-pinned `rust-cache@v2` / `upload-artifact@v7` actions.
**Why:** a push to escher's build branch triggered nothing and every run would have been cold; one script both CI and the host run makes every linux leg host-reproducible by construction, and the fast set gives the common push its verdict first (founder's concern: CI must not rebuild everything each run). The first build-branch run read green, 1255 s cold and 475 s on its warm same-sha re-run.
**Kept:** the docs job stays bare `cargo doc` (the real rustdoc gate, SHA-pinned actions and least-privilege tokens belong to "CI gate legs"); the dead `ubuntu-24.04` Free Disk Space condition is unchanged.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — one dev-profile debuginfo level for host and CI
**Section:** §Established Decisions ([Build profiles])
**Change:** was six named profiles (`profile` … `tiny`) and no `[profile.dev]`; now `[profile.dev] debug = "line-tables-only"` precedes them, inherited by the `test` profile, one level for the dev host and CI; `production` and every other profile unchanged. Its effect is recorded as measured: a blitz-tests binary's debuginfo share 82 % → 55 %, the cold local baseline (build + blitz-tests + workspace tests) 2239.59 s → 161.36 s (dev host, 32 CPUs).
**Why:** debuginfo was 82 % of a test binary's bytes and the cold test-profile compile dominated; the P4 fork chose a workspace stanza over a CI-only `CARGO_PROFILE_DEV_DEBUG` (no new env var, no host/CI divergence) — answered by the overseer under the founder's explicit delegation, relayed verbatim by the operator at the P5 review.
**Kept:** consolidating the 59 blitz-tests binaries was deferred until measured after cache + debuginfo (the same P4 round).
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — upstream-only signing, WPT and dispatch; the fork's cache budget
**Section:** §Occupied Resources (CI infrastructure · Outbound hosts) · §Standard Contracts (CI contracts) · §Infrastructure Patterns (Deployment model · CI/CD) · §Inherited Defaults (Deployment)
**Change:** was "Signed Builds" / "WPT" environments, the warp runner, the Pages deploy, the `update-results` dispatch and browser bundling reached by ref (main / `ci-test`) in this repository; now every job that reaches them — `release-cli`, `wpt`, `trigger-archive`, `post-results` — carries `github.repository == 'DioxusLabs/blitz'`, so no fork ref reaches the environments, their secrets, the Pages deploy, the dispatch or the WPT report fetch; the ref-keyed expressions inside stand, unreachable on the fork. CI infrastructure adds the fork's Actions cache: 12 entries, ≈ 9.73 GB of the 10 GB per-repository budget (LRU eviction) after the first build-branch run — one rust cache per compiling ci.yml job (11) plus one apt cache — as measured, so a lockfile or toolchain change mints new keys and evicts.
**Why:** a ref-only `if:` re-arms signing on the fork's own `main` and its existing `ci-test/sign-android-builds` branch, and the fork holds no secrets or environments; a repository guard keeps upstream's behaviour byte-equal and future upstream merges small (removing the workflows was rejected).
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `Cargo.toml`, `ci.yml`, `wpt.yml` or `publish-browser.yml` lines
**Change:** 64 citations re-pointed to the moved files — `Cargo.toml` from line 195 on +3 (the `[profile.dev]` stanza), `wpt.yml` from line 26 on +1 and `publish-browser.yml` from line 37 on +1 (the repository guards), `ci.yml` by a range map over the rewritten file (e.g. fmt job 135-151, clippy job 153-173, the jobs 30-309); the two citations of the removed opt-level rewrite went with the CI/CD rewrite. No claim text changed by the re-point.
**Why:** this chunk moved the cited lines; a stale `file:line` sends every later reader to the wrong code.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-ci-gate-legs — CI gate legs, SHA-pinned actions, a read-only token, a real rustdoc gate
**Section:** §Stack and Technologies (CI/CD · Code quality) · §Conventions (Formatting and lints) · §Standard Contracts (CI contracts) · §Infrastructure Patterns (Build system · CI/CD) · §Inherited Defaults (Code quality)
**Change:**
- was a docs job running bare `cargo doc --locked`, documenting only the lib-less root package — a nominal doc gate the workspace failed (3 crates, 9 errors); now the `doc` leg runs `cargo doc --workspace --no-deps --locked` under `-D warnings`, green over every workspace library crate, the `browser` bin `blitz` `doc = false` so the lib `blitz` alone writes `target/doc/blitz/`;
- was nine linux leg jobs; now twelve — `ci-leg.sh` adds `audit` (`cargo deny --locked check advisories`, root `deny.toml`), `a11y` (`accessibility_hidden`, `accessibility_roles`, `focusability_updates`) and `coverage` (`cargo llvm-cov` lcov to `target/coverage/lcov.info`, then the per-file report, no threshold); each new job in the slow tier with its failure-only `ci-log-{job id}`; the docs job installs `libfontconfig1-dev`;
- the `coverage` job also uploads `coverage-report` (`target/coverage/`, on success, 7 days) — the one fork-CI artifact outside `target/ci-logs/`;
- was tag-pinned actions; now every ci.yml `uses:` is pinned to a 40-hex commit SHA with its ref as a trailing comment, each `dtolnay/rust-toolchain` step naming its `toolchain`; CI installs cargo-deny 0.20.2 and cargo-llvm-cov 0.9.1 through `taiki-e/install-action`;
- ci.yml declares a workflow-level `permissions: contents: read`, no job-level grant;
- was rust-cache on every compiling job; now every compiling job but `coverage`, `a11y` restore-only on the test job's key.
**Why:** the CI gate legs chunk landed the audit, a11y and coverage legs, the pins and the read-only token, and discharged the as-built baseline's CARRY (a real rustdoc gate). The `coverage-report` upload is a boundary widening recorded PROVISIONAL — delegate overseer, 2026-10-05, under the founder's standing delegation of technical decisions (relayed verbatim by overseer); the founder's own later word supersedes it. A pinned `dtolnay/rust-toolchain` loses the toolchain its `@stable` branch name selected — every pinned step names one.
**Supersedes:** 2026-10-05-as-built-baseline — the rustdoc doc gate reaches no library crate
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

## 2026-10-05-ci-gate-legs — shared test cache, the coverage path, the cache budget re-measured over
**Section:** §Occupied Resources (Filesystem · CI infrastructure) · every section citing `ci.yml`, `ci-leg.sh` or `test_ci_workflows.py` lines
**Change:**
- Filesystem registers the coverage leg's `target/coverage/lcov.info`, uploaded on success as `coverage-report`; `target/ci-logs/` stays the failure artifacts' only path;
- CI infrastructure was "12 entries, ≈ 9.73 GB of the 10 GB budget, one rust cache per compiling ci.yml job (11)", per "2026-10-05-fork-ci-reached — upstream-only signing, WPT and dispatch; the fork's cache budget"; now the test job saves under `shared-key: workspace-test`, which `a11y` restores with `save-if: false`, and `audit` and `coverage` carry no cache; after the first run on a re-keyed lockfile the cache held 12 entries, 10 723 071 252 B — over the budget — the `workspace-test` and clippy entries already evicted while two pre-re-key matrix entries (≈ 2.16 GB) stood;
- 70 citations on 28 lines re-pointed by a measured line map (ci.yml +4 … +91, ci-leg.sh +7 after its doc arm); no claim text changed by the re-point.
**Why:** a `Cargo.lock` change re-mints every rust-cache key and evicts until the old keys age out — over budget, a new cache entry or a lockfile/toolchain change starves the legs that restore last.
**Kept:** `deny.toml` is not registered under Filesystem — a repository config file is below the registry's grain; Build system names it.
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

## 2026-10-06-telemetry-bootstrap — escher-telemetry registered; logging pattern and tracing gating rescoped
**Section:** §Stack and Technologies (Parallelism and misc · Testing) · §Conventions → Feature gating · §Standard Contracts · §Occupied Resources (Process-wide state and threads · Environment variables · Names) · §Infrastructure Patterns → Directory structure · §Cross-cutting Patterns → Logging and timing · Inherited Defaults (Publishability · Optional capabilities) · §Existing Scopes · every section citing `Cargo.toml`, `tests/blitz-tests/Cargo.toml`, `examples/seven_guis/Cargo.toml` or `examples/seven_guis/src/main.rs` lines
**Change:**
- New crate `escher-telemetry` (`publish = false`, no features, workspace member + `[workspace.dependencies]` path entry) in Names, the `packages/` tree, Existing Scopes (modules format, panic under `init`) and Publishability; the published-`packages/` clause now excludes blitz-test-harness and escher-telemetry.
- Stack: `tracing-log "0.2"` beside tracing / tracing-subscriber; blitz-tests dev-deps gain escher-telemetry, tracing, tracing-log.
- `RUST_LOG` registered (escher_telemetry's `EnvFilter`, default `warn`; also the upstream `fmt::init()` installs).
- Process-wide state: in `seven_guis_native` the global subscriber, the `LogTracer` bridge, a chaining panic hook, `OnceLock<ServiceIdentity>` + `Mutex<()>` statics, log targets `escher_telemetry` / `escher_telemetry::panic`; no thread.
- Standard Contracts: the telemetry bootstrap — `init` / `init_with_writer`, `ServiceIdentity` + `service_identity!()`, `InitOutcome`, `InitError::ForeignSubscriber`, the stderr line shape, the allowlist scrub, the panic event.
- Logging and timing: was "`fmt::init()` runs only under the `tracing` feature" as the subscriber pattern; now escher binaries install `escher_telemetry::init` (stderr, identity, scrub, log bridge, chaining hook) and the upstream `fmt::init()` installs stay feature-gated, stdout, unscrubbed.
- Feature gating / Optional capabilities: was "tracing gated per call site" workspace-wide; now the engine and upstream crates' rule, escher-telemetry ungated.
- 62 `file:line` citations re-pointed by the chunk's measured line shifts.
**Why:** the telemetry bootstrap chunk added the crate, its env read and process-wide installs, and adopted it in the stand; the per-call-site gate stays the engine crates' rule.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/

## 2026-10-06-headless-stand — headless stand surface, HarnessOptions fields, seven_guis edges, disabled keyed two ways
**Section:** §Stack and Technologies → Testing · §Established Decisions → DOM semantics · §Conventions → Manifests · §Standard Contracts → Test harness · Headless stand (new bullet) · Dioxus DOM bridge · §Occupied Resources → Names · §Infrastructure Patterns → Deployment model · §Existing Scopes → blitz-tests · seven_guis
**Change:**
- Test harness: `HarnessOptions` has eight public fields — `font_ctx` → `DocumentConfig.font_ctx` and `incremental` → `DocumentConfig.incremental` added, `Default` leaving both `None`.
- New Headless stand contract: native-only `seven_guis::stand` — `LeanTask` (Counter, FlightBooker, Timer, Crud), pinned 800 × 600 · scale 1.0 · Light, `font_ctx()` (bundled DejaVu Sans, system fonts off), `options(incremental)` (offline, `net_provider: None`), `boot` / `boot_timer` (fresh `VirtualDom` per boot, TaskShell via `app::task_in_shell`, a `TimerTicks` context); `app::Task` public, `TaskShell` private; crate-root `DEJAVU_SANS` shared with the wasm entry; `TimerTicks` replaces the timer's 100 ms delay when in context. No telemetry, env read or `blitz_net` in the stand.
- Dioxus DOM bridge: was "falsy `checked` clears"; now a falsy `checked` or `disabled` clears, every other boolean attribute is still written as `"false"`.
- DOM semantics: was "`disabled` read as a parsed boolean"; now focusability parses it as a bool while the DISABLED state (`:disabled`) and the click target key on presence — `disabled="false"` matches `:disabled` yet stays focusable.
- Names / Testing / Existing Scopes: `seven_guis` is a `[workspace.dependencies]` path entry with default features; edges blitz-tests → seven_guis (dev) and seven_guis → blitz-test-harness, blitz-traits (native); native `dioxus-native` features `system-fonts` + `woff`; the `stand` module and the stand checks + falsy-`disabled` test rows.
- Manifests: the `default-features = false` convention gains the seven_guis exception (dioxus-native refuses to compile with no renderer).
- Deployment model: seven_guis' DejaVu bytes are the shared crate-root const, compiled on native too.
**Why:** the headless stand chunk added the stand boot, the harness options it needs and the crate edges; the falsy-`disabled` engine fix is a widening on the delegate overseer's word, 2026-10-06, PROVISIONAL (upstreamable). Trap: a presence read of a Dioxus boolean attribute other than `checked` / `disabled` still reads `"false"` as set.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/

## 2026-10-06-stand-test-contract — the agent-run contract and its state area registered
**Section:** §Standard Contracts → CI contracts · §Occupied Resources → Filesystem
**Change:**
- CI contracts: adds the agent-run test contract — `bash scripts/agent-run.sh {boot | run {stand|all|name} | status | cleanup | logs}` driving `cargo test -p blitz-tests --locked`, exit grammar `0` · `1` failed (an empty run included) · `2` usage, checked before · `3` precondition unmet, JSON-line events encoded by an embedded python3 `json`, `scripts/agent-run.ps1` a pass-through; pinned by `test_agent_run.py` in the existing `ci-scripts` leg; full contract in test-plan §3.
- Filesystem: adds `target/agent-run/` — `status.json`, `events.jsonl`, `run.log` (raw output, unscrubbed like `target/ci-logs/`) — recreated by `boot`, removed by `cleanup`, local only.
**Why:** the stand test contract chunk added a project-authored harness script and its state area; no port, socket, env var, crate or dependency.
**Kept:** "the one fork-CI artifact outside `target/ci-logs/`" stays true — `target/agent-run/` is never uploaded.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/

## 2026-10-06-cold-agent-run-pipe — the cold-agent pipe registered; its crossings PROVISIONAL
**Section:** §Standard Contracts → CI contracts · §Occupied Resources → Filesystem · Process-wide state · Outbound hosts · Names · §Stack and Technologies → CI/CD · §Infrastructure Patterns → Directory structure
**Change:**
- CI contracts: adds the cold-agent run pipe beside agent-run — `bash scripts/cold-agent.sh {run <task> | status | cleanup | logs}`, task `counter`, exit `0` · `1` failed verdict · `2` usage before any precondition · `3` precondition; python3-`json` events `run.start` · `run.end` · `status` · `cleanup`; `run` starts one isolated `claude -p` session from a per-run `mktemp -d` dir with only the stub's MCP tools and writes `verdict.json` (`passed` only on positive evidence); the stdlib stdio MCP stub (`list` · `read` · `press`; `not-found` · `disabled` · `not-pressable` · `malformed`); pinned by `test_cold_agent.py` in the `ci-scripts` leg.
- Filesystem: `target/cold-agent/` (verdict, events, raw transcript, client log, call log, stub state), the per-run session dir, the live run's committed `evidence/live-*`.
- Process-wide state: one spawned `claude` client per run and its stdio stub. Outbound hosts: the model provider via the operator's own Claude Code login, live run only, never CI.
- Names: the three scripts and the MCP server name `stub`. Stack CI/CD: the Claude Code CLI as the host-only agent client (read at 2.1.288 on the dev host). Directory structure: a `scripts/` line.
- Network ports and listeners stays none.
**Why:** the cold-agent run pipe chunk built the cold-agent gate's reachability pipe. The client, its stdio stub, the outbound path and the login are a Boundary widening: answered at P4 and ratified at this wrap by the overseer under the founder's standing delegation, kept PROVISIONAL in the body until the founder's own word.
**Kept:** "the one fork-CI artifact outside `target/ci-logs/`" stays true — `target/cold-agent/` is uploaded by no CI job.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/
