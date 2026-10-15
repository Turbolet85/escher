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

## 2026-10-06-upstream-sync-element-identity — upstream main 23354585 merged; normal alignment keyword; sync base recorded
**Section:** §Established Decisions → [CSS approximations] · §Project Intent · every section citing a merged upstream file's lines
**Change:**
- CSS approximations: was "container `align-items`/`justify-items: normal` left unset"; now `align-items`/`justify-items`/`align-self`/`justify-self: normal` map to Taffy's `NORMAL` keyword, which Taffy resolves by the aligned box's layout mode — a container's `auto` or unknown flag too.
- CSS approximations: the `safe` positional content-alignment default of a block container now covers a table cell.
- Project Intent: a new `Upstream sync:` line beside `Origin:` — DioxusLabs/blitz `main` last merged at `2335458530518cdf167c55ce635fae99323e0789` (merge commit `f00b0216`), the next sync's merge base; the `Origin:` line unchanged.
- 113 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585` (11 commits, conflict-free, our changes additive); upstream moved the alignment mapping to Taffy's first-class `normal` keyword and extended `align-content` to table cells. The sync base is where the next epoch's Upstream sync reads its merge base.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-upstream-sync-element-identity — root-manifest citations re-pointed after the line-61 insert
**Section:** §Design Philosophy · §Stack and Technologies · §Established Decisions · §Conventions · §Standard Contracts · §Infrastructure Patterns · §Cross-cutting Patterns · §Inherited Defaults (every root `Cargo.toml` citation past line 60)
**Change:** 43 root `Cargo.toml` citations re-pointed +1 per number past 60 — they read one line low since the `seven_guis` path entry was inserted at `Cargo.toml:61`; each verified against the cited text. The two citations of `Cargo.toml:61` itself, written after the insert, unchanged. No claim text changed.
**Why:** the 2026-10-06-headless-stand wrap did not re-point the root-manifest citations past its insert; the operator chose at this wrap's escalation (2026-10-06) to fix them in this pass rather than carry them.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-stable-element-ids — stable element ids registered on the Dioxus DOM bridge
**Section:** §Standard Contracts → Dioxus DOM bridge · §Existing Scopes → dioxus-native-dom · §Existing Scopes → tests/blitz-tests · the citations below
**Change:**
- Dioxus DOM bridge: `DioxusDocument::element_id(NodeId) -> Option<String>` (`None` for a non-element, stale or detached node; never panics) and `element_ids() -> Vec<(NodeId, String)>` (document pre-order, pairwise distinct), computed on demand from a vdom walk joined with a DOM walk, written to neither the DOM nor the vdom.
- The grammar: (1) author key = the HTML `id` verbatim when non-empty, `/`-free and first in document pre-order; (2) component path = `/`-joined component segments below the app root — dioxus-core's four framework scopes (ROOT · ROOT_SUSPENSE_BOUNDARY · ROOT_ERROR_BOUNDARY · APP) add none, a segment is the last `::` segment of `VComponent.name` with generics stripped, an owner's 2nd+ same-name instance reads `{name}:{k}` — then `{tag}[{key}]` for a Dioxus-keyed template root else `{tag}:{n}` among same-parent, same-owner, same-tag siblings; a same-owner template root appends to its parent's path, another owner's restarts at its chain; the app root's own elements read `/{tag}:{n}…`; (3) document path `/{tag}:{n}…` from the document root. No `NodeId`, `ElementId`, `ScopeId` or pointer in an id.
- A node a re-render removes is detached (parent `None`) and keeps resolving until its ElementId is reassigned; it reads no id.
- dioxus-native-dom's module list gains the crate-private `element_id`; the tests/blitz-tests row gains `stand_element_ids` (one stable element id per element).
- 8 `file:line` citations into `dioxus_document.rs`, `lib.rs` and the four lean-task files re-pointed by the chunk's measured line maps.
**Why:** the chunk built the ids for v010-01; the grammar is recorded as BUILT — the plan's text assumed the base scope is the user root `root`, that `VComponent.name` is a bare name, and a grammar two component instances or a nested same-owner root could collide under; dioxus-core 0.7.10 and the measured ids corrected all three. Later chunks (persistence, a11y identity, snapshot, driver) address elements by this grammar.
**Kept:** the edge where the app root renders a top-level `html` element (it would read `/html:0`, the skeleton's document path) — never on the stand, untested.
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/

## 2026-10-06-id-persistence — stable ids proven persistent; CRUD rows keyed by person; test re-exec spawns registered
**Section:** §Standard Contracts → Dioxus DOM bridge · §Occupied Resources → Process-wide state and threads
**Change:**
- Dioxus DOM bridge: the stable element id now **persists** on the stand, in both layout modes, on each lean task. Every element present before and after a re-render reads the same id. A remount (card → Back → the same card in `app::app`) reads the same id list in order, equal to `stand::boot`'s, on fresh `NodeId`s for every element except the four document-skeleton elements `DioxusDocument::new` builds outside the VirtualDom (`/html:0`, `/html:0/head:0`, `/html:0/body:0`, `main`). Every dropped pre-remount `NodeId` reads `None`. A second process reads the same id lists.
- The lean CRUD row's Dioxus key was the list index `{i}`; it is now the person's model-assigned `u64` (`{person.id}`, fixture people 0–2, Create from 3). The key sits on the `for` item itself with the filter in the iterator, so a row's id `…/div[{person.id}]` follows its person under filter, Create and Delete. A `key:` inside an `if` within a `for` body does not key the list.
- Process-wide state and threads: registers `telemetry_stdout_silent` and `stand_id_persistence`. Each re-executes its own test binary once under `cargo test` (`current_exe()` with `--ignored --exact {child} --nocapture`, output captured; only `telemetry_stdout_silent` removes `RUST_LOG`). Test-only, a fixed argv, no socket or port.
**Why:** v010-02 proves the id an agent holds survives re-render, remount and restart, which Epoch 3 diffs and Epoch 4 act-by-id rest on. The operator chose person keys over index keys at P4. The operator ruled at the P5 review (2026-10-06, given directly) that the test re-exec is not a boundary widening and that both spawns register here. Trap: the html/head/body/#main skeleton is never remounted, so "every node is fresh after a remount" is false for those four.
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/

## 2026-10-06-project-readme — README is escher's front page; stylo_taffy triple licence recorded
**Section:** §Conventions → Licensing exceptions · §Project Intent → Front page
**Change:**
- Licensing exceptions now also lists `stylo_taffy`, which declares `license = "MIT OR Apache-2.0 OR MPL-2.0"`: the workspace's dual licence plus MPL 2.0.
- Project Intent gains a **Front page** bullet. The root `README.md` is escher's own page, not upstream Blitz's. It covers what escher is, what is built and what is planned (stable element ids and the headless 7GUIs stand are built; snapshot, driver, CLI and MCP are planned), its Blitz lineage, the 0.1.0 epochs, the licence and an Author / Contact section. Its one image is the fork's `ci.yml` badge. No build, doc or test step reads it; `rdme` resolves a README at runtime only. At an Upstream sync, an upstream change to `README.md` resolves to escher's version.
**Why:** the chunk replaced upstream's front page wholesale, so later Upstream syncs will meet README conflicts at some syncs; the rule records which side wins. The README states stylo_taffy's MPL 2.0, which no master recorded. Trap: Product type's homepage/repository still name dioxuslabs/blitz; the release-metadata repoint is a separate decision pinned on the route, not this bullet.
**Ref:** .andromeda/runs/2026-10-06T11-42-50-wrap/

## 2026-10-06-accessibility-tree-identity — stable id on the accessibility tree; name sources; shell builds through the document
**Section:** §Standard Contracts → Document core · → Mutation, query and CSSOM · → Dioxus DOM bridge · §Cross-cutting Patterns → Config management · citations into the changed files (re-pointed)
**Change:**
- Document core: the `Document` trait gains, under `accessibility`, a default `accessibility_tree() -> accesskit::TreeUpdate` returning `self.inner().build_accessibility_tree()`; a wrapper may override it, and `DioxusDocument` alone does.
- Mutation, query and CSSOM: `build_accessibility_tree` names a node from three sources — direct text children, an `aria-label` non-empty after trimming (`label`, accname-1.2 §2C), and a `<label>` bound through `label_bound_input_element` (pushed onto the bound `<input>`'s `labelled_by` after the visit). blitz-shell's `AccessibilityState::update_tree` takes `&dyn Document` and calls `doc.accessibility_tree()` from `View::poll` and `View::build_accessibility_tree`; was: built from the inner `BaseDocument`.
- Dioxus DOM bridge: the override sets each element node's AccessKit `author_id` to its stable id (none on `TextRun`, document root or `Window`; no tree id indexes the slab), so the id reaches both the harness's tree and the shell's platform tree, written to neither the DOM nor the vdom. Feature wiring: dioxus-native-dom's `accessibility` = `["blitz-dom/accessibility", "dep:accesskit"]`; dioxus-native's forwards to it; under the workspace's `default-features = false` a crate gets the override only by naming the feature — blitz-tests does, seven_guis does not.
- Config management: was "each dioxus-native feature forwards to the same-named blitz feature"; now `accessibility` also forwards to dioxus-native-dom's.
- 66 citations re-pointed by the measured line map (38 blitz-dom `document.rs`, 15 `dioxus_document.rs`, 6 blitz-shell `window.rs`, 4 blitz-dom `accessibility.rs`, 3 dioxus-native-dom `Cargo.toml`).
**Why:** intent §Findings 1 needs the same id on the accessibility node as in the snapshot and driver; a `DocumentConfig` provider cannot reach the vdom, and an inherent post-pass would never reach the platform tree, so the crossing is a defaulted trait method the shell calls. The feature-forwarding claim was measured false once the override needed dioxus-native-dom's own feature. Trap: a feature-gated item can pass under `cargo test --workspace` and be compiled out under `-p blitz-tests` — check `cargo tree -e features -i`.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/

## 2026-10-06-founder-rulings — the PROVISIONAL widenings ratified: cold-agent crossings, coverage-report upload, falsy-disabled fix
**Section:** §Stack and Technologies → CI/CD row · §Standard Contracts → CI contracts · §Occupied Resources → Outbound hosts · → Process-wide state and threads
**Change:**
- Cold-agent run pipe: the four body clauses (the `claude` client and its stdio stub · the `claude -p` session · the model provider reached through the operator's own Claude Code login · the spawned client and stub processes) were "PROVISIONAL, overseer under the founder's standing delegation, pending the founder's word"; now "ratified by the founder, 2026-10-06". The registered crossings themselves are unchanged.
- The `coverage-report` upload (`target/coverage/`, `if: success()`, kept 7 days, the one fork-CI artifact outside `target/ci-logs/`): no body text changes — the body never carried its provisional status; it is now ratified, no longer PROVISIONAL per the 2026-10-05-ci-gate-legs entry.
- The falsy-`disabled` clearing in dioxus-native-dom: no body text changes; now ratified, no longer PROVISIONAL per the 2026-10-06-headless-stand entry (still upstreamable).
**Why:** the founder ruled on all three at the Epoch 2 boundary, in their own word given at this 0-pending wrap (the founder, 2026-10-06), which supersedes the delegate overseer's provisional answers by rule. Standing rule set by the same ruling: PROVISIONAL items are discharged in one batch at each epoch boundary.
**Ref:** .andromeda/runs/2026-10-06T16-16-18-wrap/

## 2026-10-06-snapshot-model — snapshot model registered on the Dioxus DOM bridge
**Section:** §Standard Contracts → Dioxus DOM bridge · §Existing Scopes → dioxus-native-dom · §Cross-cutting Patterns → Config management · §Conventions → Feature gating
**Change:**
- Dioxus DOM bridge: registers `DioxusDocument::snapshot(&self) -> Snapshot`, read-only, under the `accessibility` feature. `Snapshot { roots }` with `nodes()` and `get(&str)`; `SnapshotNode { id, role: accesskit::Role, name, state, bounds: blitz_dom::BoundingRect, children }`; `NodeState { enabled: Option<bool>, checked: Option<bool>, value: Option<String>, focused: bool }`; all `Debug`, `Clone`, `PartialEq`, re-exported from the crate and through dioxus-native's glob. A node is every element the accessibility tree keeps (an `author_id` node whose element resolves through `get_node`), generic containers included; `Window`, document root and `TextRun`s are not nodes. Each field is an existing reader's: `author_id`, `role()`, `label` else `labelled_by` names, `get_client_bounding_rect`, `disabled` presence on a `can_be_disabled()` element, `checkbox_input_checked()`, the editor text else `value` attribute, the `TreeUpdate`'s `focus`. No field holds a `NodeId`, tree id, `ElementId`, `ScopeId` or pointer; no wire form; no driver, CLI or MCP command exposes it yet.
- The feature clause was "gets the override only by naming the feature"; now "the override and the snapshot model".
- Existing Scopes: the module list gains `snapshot` (crate-private, `accessibility` only); citation `lib.rs:12-16` → `13-19`.
- Config management: was "gates its `accessibility_tree` override"; now "and its `snapshot` module".
- Feature gating: citations re-pointed `lib.rs:33-56` → `38-61`, `47-50` → `52-55`, `5-10` → `5-11`.
**Why:** intent §Findings 2's data model, built where the stable id lives. The operator chose at phase P4 that every kept element is a node. Standing rule: the snapshot derives no role or name of its own, so it cannot disagree with the accessibility tree. Trap: `focused` never reads `get_focussed_node_id`, which falls back to the root element.
**Kept:** the README sentence in §Project Intent still lists the snapshot as planned, because no snapshot command exists until the driver.
**Ref:** .andromeda/runs/2026-10-06T19-14-10-wrap/

## 2026-10-06-id-stability-across-code-edits — the id grammar gains an anchored tier; the bridge gains the actionable-key check
**Section:** §Standard Contracts → Dioxus DOM bridge · → Document core · → Headless stand · §Cross-cutting Patterns → Config management · → Event processing · §Conventions → Feature gating · §Existing Scopes → dioxus-native-dom · → blitz-tests · §Project Intent → Apps and demonstrators
**Change:**
- Grammar: was three tiers (author key · component path · document path); now four — the anchored path `{key}//{segment}` sits second, for an element under an element of its own component instance that reads an author key, the nearest one anchoring. A template root owned by its DOM parent's owner continues the parent's path, which is `{key}/` when the parent reads its key; one owned by another component still restarts at its chain, so an anchor acts inside one component only. An unusable `id` anchors nothing. An anchored path is the only id holding `//`.
- New clause: the id holds across code edits on the stand, with the edits that may still change an unkeyed id named.
- Stand readings: a CRUD row was `TaskShell/Crud/div:0/div:1/div:0/div[{person.id}]`; now its author key `crud-person-{person.id}`. Home's seven cards read `task-card-{slug}`, their children anchored paths.
- Registers `DioxusDocument::unkeyed_actionable() -> Vec<UnkeyedActionable>` under `accessibility`: the three readers (focusable · a closed interactive role list · `data-dioxus-id`), the type's seven fields and `remedy()`, no wire form; it reads empty on the four lean tasks and Home, and 2 · 3 · 676 on the three tasks not yet keyed.
- The `accessibility` feature is stated at three sites to gate the override, the snapshot model and the actionable-key check; the dioxus-native-dom module list opens with `actionable`.
- 21 line citations re-pointed.
**Why:** the founder ratified both rules, the `key//segment` spelling, the one-component reach and the public check at phase (the founder, 2026-10-06). Trap for later chunks: `UnkeyedActionable.node` is a process-local `NodeId`, so the first wire form of the check must not carry it.
**Kept:** anchoring across a component boundary, a bare `key/segment` and a leading `#` were weighed and ruled out by the founder; the component-path tier and every existing id but CRUD's rows are unchanged.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/

## 2026-10-06-snapshot-state-fidelity — the bridge clears 27 falsy boolean attributes; a password's snapshot value is masked
**Section:** §Standard Contracts → Dioxus DOM bridge · §Occupied Resources → Process-wide state and threads · §Conventions → Known gaps · §Cross-cutting Patterns → Event processing · §Existing Scopes → dioxus-native-dom · → blitz-tests
**Change:**
- Falsy clear: was "a falsy `checked` or `disabled` clears the attribute; every other boolean attribute is still written with the literal value `"false"`"; now a falsy value (Bool false, Text `"false"`, Int 0, Float 0.0, None) of one of the 27 names of the module-private `BOOLEAN_ATTRIBUTES` list — the `isBoolAttr` list of Dioxus 0.7.10's web interpreter — clears the attribute, on the dynamic and the static template path alike. A truthy value of a listed name is still written; an attribute outside the list holding `false` is still written as `"false"`. An element rendered `hidden: false` carries no `hidden` attribute, is laid out and is a snapshot node; a truthy `hidden` excludes it.
- Snapshot `value`: a password `input` (type compared ASCII-case-insensitively) reads `MASKED_VALUE` where its reading is non-empty and `""` where it is empty, never the text or its length; every other element reads as before.
- New public item `MASKED_VALUE: &str` (eight U+2022), re-exported with the three snapshot types; `NodeState` keeps its four fields.
- Snapshot `enabled`: stated as the `disabled`-presence reading that decides whether a click acts, with its pinned disagreement — `disabled="false"` and a bare `disabled` read `Some(false)` while the focusability reader takes both as focusable.
- The blitz-tests scope row names `dioxus_falsy_boolean_attrs.rs` and `stand_snapshot_state.rs`.
- 14 line citations re-pointed (`mutation_writer.rs`, `snapshot.rs`), one retired with its clause.
**Why:** the chunk proved the four state readings per control and closed the one that was wrong (a password read in the clear). The mask form — one fixed marker — is the operator's answer at phase (the operator, 2026-10-06). The 27-name clear is PROVISIONAL: answered at phase's question round and recorded provisional on the direction given at its P5 review (2026-10-06), pending the founder's own word; by the founder's standing rule it is discharged with the Epoch 3 boundary batch. Trap for later chunks: the list is a copy — a Dioxus upgrade that changes `isBoolAttr` does not move it.
**Kept:** the web interpreter's `truthy` predicate (anything but `true` is removed) was weighed and left out — only the name list widened, the bridge's own falsy test stays; a fifth `NodeState` field or a changed `value` type to mark a masked value was ruled out; masking in the accessibility tree was ruled out (the tree gives an element no value).
**Ref:** .andromeda/runs/2026-10-06T22-39-16-wrap/

## 2026-10-06-compact-snapshot-serialization — the snapshot's text form and its size budget
**Section:** §Standard Contracts → Dioxus DOM bridge · §Existing Scopes → dioxus-native-dom · §Cross-cutting Patterns → Config management · §Conventions → Feature gating
**Change:**
- New contract item `Snapshot::to_text(&self) -> String` (crate-private module `snapshot_text`, compiled only under `accessibility`): a pure function of the `Snapshot`, one line per node in `nodes()` pre-order, each ended by a line break, two spaces of indent per depth, the empty snapshot the empty string. A line reads the role (its `Debug` spelling, no alias table), the quoted name, `id=` and the quoted id, the state tokens in the fixed order `enabled`/`disabled` · `checked`/`unchecked` · `value=` quoted · `focused`, then `@x,y widthxheight` in `f64`'s `Display` form. Strings are `str`'s `Debug` form: no line break or bare `"` inside a field, an apostrophe written as it is. No `Display` impl, second entry point, parser, JSON form, truncation or paging.
- New public constant `SNAPSHOT_TEXT_BUDGET: usize = 10_000`, bytes of `to_text`'s result — a ceiling a caller compares against, never enforced by `to_text` — re-exported from the crate and through dioxus-native. The lean stand screens read 755 · 1269 · 1259 · 2034 bytes at boot, both layout modes, dev host.
- Was "the snapshot has no wire form"; now it has one serialized form, that text, which leaves the process through the returned value only — no log, event, socket, file or command carries the snapshot or its text, and `to_text`'s callers are tests.
- Snapshot `value`: the mask was a password `input`'s only; now a password or a file `input` reads `MASKED_VALUE` where non-empty, never the text, its length or a file's host path; the DOM attribute and form submission still hold the path.
- The three places that list what `accessibility` gates name the text form and the `snapshot_text` module; the scope row lists the module.
- 6 line citations re-pointed (`snapshot.rs` ×1, `lib.rs` ×5) by the chunk's measured line map.
**Why:** the chunk built the snapshot's first text form and recorded its budget. The returned-value-only crossing is PROVISIONAL: the operator's answer at the chunk's plan (2026-10-06), recorded as provisional on the operator's answer at this wrap's escalation (2026-10-07), pending the founder's word at the Epoch 3 boundary. The quoting is the `Debug` form on the operator's direction (2026-10-06), replacing the plan's `str::escape_debug`, which backslash-escapes an apostrophe. Standing rule for later chunks: the driver, CLI and MCP entries ask again when the text first leaves the process.
**Kept:** "the check has no wire form" — the actionable-key check's sentence in the same contract — stands; this chunk did not touch that check.
**Ref:** .andromeda/runs/2026-10-06T23-58-14-wrap/

## 2026-10-07-change-tracking-and-diff — the changed set, the shell's refresh on change, the snapshot diff
**Section:** §Standard Contracts → Document core · Mutation, query and CSSOM · Dioxus DOM bridge; §Cross-cutting Patterns → Invalidation and state integrity · Config management; §Existing Scopes → dioxus-native-dom; `file:line` citations into five edited files
**Change:**
- Document core: `BaseDocument::has_changes` reads true while the changed set is non-empty, from the first tracked write until the next drain (was: `changed_nodes.is_empty()` over a set the constructor filled and nothing drained); new `BaseDocument::take_changed_nodes(&mut self) -> HashSet<NodeId>` returns the set and leaves it empty, a returned id read through `get_node`; `create_node` no longer marks; `has_changes` has no caller outside tests.
- Invalidation and state integrity: the set is marked by the 15 mutation-flag sites of `DocumentMutator` (in-document nodes only), by `snapshot_node_and` on `FOCUS` or `CHECKED`, and by the text-input `Input` arm — the last two wherever the node is — and not by node creation, hover, active, scroll, resize, a resource load, an animation or layout; engine-touched, a superset of snapshot-changed.
- Mutation contract and Invalidation: on a poll that reported work `View::poll` takes the set (outside the `accessibility` cfg) and, under `accessibility`, calls `update_tree` when it was non-empty (was: "from `View::poll` on `has_changes()`" and "rebuilt on poll when the document changes"); the platform adapter receives the tree on change as well as on its initial request; no windowed witness on the dev host.
- Dioxus DOM bridge: `Snapshot::diff(&self, after: &Snapshot) -> SnapshotDiff`; `SnapshotDiff { added, removed, changed }` with `is_empty()`; `DiffNode { id, parent, role, name, state, bounds }`, the reading after the step, flat; added, removed and changed by id, in pre-order, the first node standing for a duplicated id; not derived from the changed set; no text form or entry point; returned value only. The feature-forwarding sentence, Config management and the Existing Scopes row name `snapshot_diff`.
- 50 of 85 citations into the five edited files re-pointed by the measured line map.
**Why:** one item is PROVISIONAL, pending the founder at the Epoch 3 boundary, in two halves: the engine flag contract, which changes behaviour for every Blitz document (the operator, 2026-10-07, at the chunk's P5 review), and the shell's refresh on change, a boundary widening (the operator, 2026-10-07, at the plan and at this wrap's escalation: record as provisional). The diff's returned-value-only exit carries the snapshot text's provisional answer. Later: the driver's act returns this diff unchanged, and the first command returning it asks the crossing question again.
**Kept:** "the snapshot has one serialized form, the text" stands — a diff is not a serialized form.
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/

## 2026-10-07-audit-corrections — a shared stand-check module, an out-of-line unit-test module, the id walk's helpers
**Section:** §Conventions → Tests; §Existing Scopes → dioxus-native-dom · blitz-tests; §Standard Contracts → Dioxus DOM bridge (`file:line` citations); §Occupied Resources → Process-wide state and threads (one citation)
**Change:**
- Conventions → Tests: unit tests sit in `#[cfg(test)] mod tests` at the end of the source file, with one exception — dioxus-native-dom's bridge tests, in a file of their own, declared at the end of `dioxus_document.rs` as `#[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;`, a child of the module under test (was: no exception stated).
- Conventions → Tests: "a pointer-event builder is repeated per file" stands for the three files it cites; the seven_guis stand checks state the tables and helpers they share once, in `tests/blitz-tests/tests/common/mod.rs`, read through `mod common;`. A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior (was: every integration test file stood alone).
- Existing Scopes → dioxus-native-dom: `dioxus_document` carries one `cfg(test)` child module, `bridge_tests`, six unit tests, declared in `dioxus_document.rs` and not in `lib.rs`.
- Existing Scopes → blitz-tests: the shared module `tests/common/mod.rs` — no test target, no test in it, the one statement of the tables `controls`, `INPUT_NAMES` and `rendered` and of six helpers, read by six stand checks.
- Dioxus DOM bridge: the id grammar is unchanged. The walk's citation now spans `VdomWalk::walk` and the three private helpers split from it (`record_roots`, `walk_component`, `child_chain`). 8 of 34 citations into the ten edited files re-pointed by the measured line map; 26 keep their numbers.
**Why:** the chunk closed four code-audit findings without changing any behaviour: it killed seven surviving mutants with tests placed inside `dioxus_document` (the only place one of them can be observed), brought the id walk and three stand checks under the complexity ceiling, and stated the stand's restated tables and helpers once. Where the test file lives and that the helpers go with the tables were the operator's answers at the chunk's plan (2026-10-07). Standing rule: a table typed by dev-dependencies is shared through a subdirectory module, since the test crate's `[lib]` cannot see them.
**Kept:** per-file repetition elsewhere in `tests/blitz-tests` — only the stand checks share a module.
**Ref:** .andromeda/runs/2026-10-07T03-59-09-wrap/

## 2026-10-07-driver-session — the driver session registered: crate `escher-driver`, its lifecycle socket, the `escher-session` host
**Section:** §Occupied Resources (Network ports and listeners · Filesystem · Process-wide state and threads · Names) · §Standard Contracts (new Driver session bullet · CLIs) · §Established Decisions (new [Driver session]) · §Inherited Defaults (API style · Publishability) · §Infrastructure Patterns → Directory structure · §Existing Scopes (new escher-driver row · seven_guis · blitz-tests) · §Conventions → Tests · §Stack and Technologies → Testing · §Cross-cutting Patterns → Logging and timing
**Change:**
- Listeners: was "none"; now no TCP or UDP port and one local listener — the Unix-domain socket `{state_dir}/session.sock`, mode `0600` in a `0700` caller-named state directory, one bind site, lifecycle messages only (`hello`, `stop`), unix only (`Unsupported` elsewhere). API style: "no network listener" scoped to TCP/UDP.
- Filesystem: the session state directory (socket file alone while up; socket then directory removed at `stop`, non-recursively) and the checks' four directories under `target/tmp/`.
- Process state: re-executed test children 2 → 4 (two host a session socket); `host_binary` spawns the real binary; `escher_telemetry::init` has a second install site, `escher-session`, so both seven_guis binaries stamp `service.name=seven_guis`.
- Names and scopes: crate `escher-driver` (`publish = false`, no features, no binary; depends on blitz-test-harness and dioxus-native-dom with no feature named); binary `escher-session` of seven_guis and its test target `host_binary`; seven_guis' native dependencies now five; five `stand_session_*` checks and a second shared module `tests/session_common/mod.rs`; `mod common;` readers six → eleven.
- Standard Contracts: `Session::start` (label 1-32 bytes of `a-z0-9-`, checked before the caller's boot), `serve`, `start` / `attach` / `stop`, `Hello`, `Started`, the eleven `SessionError` variants with fixed messages, and the crate-private `v1` wire (request ≤ 64 bytes, reply ≤ 128, 2 s I/O bound, 25 ms poll, 5 s stop bound); the `escher-session <task> <state-dir>` CLI, exit `0` · `1` · `2`, stdout empty.
**Why:** the chunk built the driver's session. The socket is a boundary widening, ratified by the founder's own choice among three options (the founder, 2026-10-07, relayed verbatim). The library takes the app's boot from its caller and depends on no app (the operator's review, 2026-10-07). Standing: nothing of the screen crosses the wire — the first request that carries an id, a name, a value, snapshot text or a diff is a new crossing question.
**Kept:** no idle expiry (a host whose client died without `stop` keeps running — the Driver CLI entry answers it); no `dioxus-native-dom` feature named by the library; `start` does not check that the answering pid is its child.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-driver-session — founder's rulings: the Epoch 3 PROVISIONAL items ratified
**Section:** §Standard Contracts → Document core · Paint and shell · Dioxus DOM bridge (snapshot text · snapshot diff) · §Cross-cutting Patterns → Invalidation and state integrity
**Change:**
- The engine's changed-set contract and the shell's refresh of the platform accessibility tree on change: four body clauses read "PROVISIONAL … pending the founder's word at the Epoch 3 boundary"; now "ratified by the founder (2026-10-07)". The windowed witness is still owed and the body says so: no windowed run witnesses the refresh on the dev host.
- The snapshot text and the snapshot diff leave the process through the returned value only: two clauses were PROVISIONAL; now ratified by the founder, 2026-10-07.
- The bridge's falsy clear of its 27 boolean attributes: no body text changes — the body never carried its provisional status; it is now ratified, no longer PROVISIONAL per the 2026-10-06-snapshot-state-fidelity entry.
**Why:** the founder ruled on all three at the Epoch 3 boundary (the founder, 2026-10-07, in the overseer session, relayed verbatim by the overseer and given at this wrap by its operator), superseding the operator's provisional answers by rule. The standing rule that PROVISIONAL items are discharged in one batch at each epoch boundary is now a playbook rule ("Provisional discharge"), appended at this wrap on the operator's direction.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-driver-session — manifest citations re-pointed after the two root-manifest inserts
**Section:** every section citing root `Cargo.toml` (65 sites) · §Stack and Technologies → Testing · §Occupied Resources → Names · §Existing Scopes → seven_guis
**Change:** root `Cargo.toml` gained the member line at `:18` and the `[workspace.dependencies]` entry at `:62`, so every cited line at old `:18`-`:60` moved +1 and at old `:61` or below +2; 62 of architecture's 65 sites re-pointed (`:1` ×2 and `:17` stand), ranges spanning an insert widened (`:2-30` → `:2-31`, `:44-60` → `:45-61`, `:243-254` → `:245-256`). The blitz-tests manifest ranges, stale by one line before this chunk, are re-measured to `:14-25; :27-29; :31-40`. The stand manifest's native block is `:29-35` (was `:29-34`) at both sites; its eight other cited lines sit in the unchanged lines 1-34.
**Why:** a measured re-point: each moved citation was verified by comparing the base file's cited lines with the working tree's lines at the new numbers. Trap for later chunks: any insert into the root manifest's members list or dependency block shifts every citation below it in all seven masters.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-sink-target-allowlist — the sink's target allowlist: a second public prefix set and the drop of every outside-target record
**Section:** §Standard Contracts → Telemetry bootstrap · §Cross-cutting Patterns → Logging and timing · §Existing Scopes → escher-telemetry, seven_guis · §Occupied Resources → Filesystem, Process-wide state and threads, Environment variables · §Conventions → Feature gating
**Change:**
- Telemetry bootstrap: was "a field in `CONTENT_FIELDS` is redacted at any target", every non-engine target printing; now the sink admits two public prefix sets, `ENGINE_TARGET_PREFIXES` and the new `ESCHER_TARGET_PREFIXES` (`escher_`, the fifth public scrub constant): an engine target prints only `SAFE_FIELDS`, an escher target prints every field not in `CONTENT_FIELDS`, and a target of neither set is dropped whole — zero bytes, at every level, WARN and ERROR included, whatever `RUST_LOG` names, a bridged `log` record judged by the target it was logged under. The line shape reads "one per printed event".
- Logging and timing and the escher-telemetry scope row: the sink is described as drop-then-scrub; no filter layer is added, the drop sits in the formatter.
- Process-wide state and threads: the re-executing blitz-tests binaries are three (`telemetry_drop` added, its child run with `RUST_LOG=warn,style=trace,dioxus_core=trace,selectors=trace` and installing the sink); two seven_guis tests spawn the `escher-session` binary (`host_log` added: `crud`, `RUST_LOG=trace`, one draining thread per stream in the test process, `attach` under 60 s, exit under 10 s).
- Filesystem: the seven_guis state directories are `hb-serve`, `hb-refuse` and `hl-trace`.
- seven_guis scope row: two integration-test targets, `host_binary` and `host_log`, and their shared module `tests/common/mod.rs`, no test target.
- Thirteen citations into the sink's two source files and `host_binary.rs` re-pointed to the as-built lines; the `RUST_LOG` and install-line citations are coordinate-only moves.
**Why:** the chunk closed the leak the previous chunk measured: a record the sink has no scrub rule for is no longer printed. The four design leans — the drop in the formatter, the prefix `escher_`, a public constant, every level dropped — were approved by the operator at the plan review (the operator, 2026-10-07). Standing rule for later chunks: a log target is admitted only by one of the two prefix sets, so a new escher crate's target must start with `escher_`, and a third-party WARN or ERROR does not reach a sink-installing binary's stderr.
**Kept:** the registries gain no new resource kind — no socket, port, env var, crate, binary or feature; `RUST_LOG` stays the sink's one env read.
**Ref:** .andromeda/runs/2026-10-07T08-23-43-wrap/

## 2026-10-07-settle-detection — settle is built: `Harness::settle`, `Session::act`, a twelfth `SessionError` variant
**Section:** §Standard Contracts → Test harness · → Driver session (escher-driver) · §Existing Scopes (the blitz-test-harness, escher-driver and blitz-tests rows) · §Conventions → Tests · → Documentation · §Established Decisions → [Driver session] · §Design Philosophy → Headless, measurable, testable
**Change:**
- Test harness: the exports gain `Busy`, `NotSettled`, `SETTLE_PASS_LIMIT` (64) and `Settled`; `Harness::settle() -> Result<Settled, NotSettled>` is stated — one pass is pump's poll then resolve with poll's answer kept; a pass is quiet when poll answered false, the hover node did not change and no load finished; after a quiet pass a request in flight or a pending render-blocking resource is `NotSettled { busy: Loads }` at once, else `Settled { passes, animating }`; 64 passes with none quiet is `Render`, `Loads` or `Layout`, read off the last pass. Settle never waits on a load, neither waits on nor advances a timer or an animation, reads no clock but the harness's, neither reads nor drains the changed set, logs nothing, starts no thread. A fetching `net_provider` is wrapped in a crate-private counter that stores counts only; an offline harness and a wrapped document carry none.
- Driver session: the re-exports gain `Busy` and `Settled` (not `NotSettled`, not the limit); `Session::act(step) -> Result<Settled, SessionError>` runs a step on the held instance, settles it and maps a not-settled instance onto the twelfth variant `NotSettled(Busy)`, whose message appends the class name; the step is not rolled back; nothing `act` reads or returns crosses the socket. Was "Not built: a verb set, settle, act by id …"; now settle is built in process and what is not built is a settle verb or busy-source reply on the socket.
- Existing Scopes: the harness has four modules (`settle`); the driver row names `act` and the two re-exports; the blitz-tests row gains `stand_settle`, and `mod session_common;` is read by four session checks and by `stand_settle` (was "by four of them").
- Conventions → Tests: was "the session checks keep what only they share"; now the checks that hold a `Session` — the session checks and `stand_settle` — share it.
- Citations into `harness.rs`, the harness's `lib.rs` and the driver's `lib.rs`, `session.rs` and `error.rs` re-pointed to the lines measured after the chunk.
**Why:** the chunk built the settled step the driver's later entries return after. Rules it sets: settle reads no clock, so it reports a load and never waits; time on a held instance is the caller's to move; the outcome names a class only; a settle verb on the socket is a new crossing question.
**Kept:** no quiescence method on the `Document` trait and no change to what `poll` returns; no input helper settles.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/

## 2026-10-07-command-and-refusal-schema — the driver's command and refusal schema is built, in process
**Section:** §Standard Contracts → Driver session · §Existing Scopes → escher-driver
**Change:**
- Driver session: the crate re-exports 32 names (was 10): the session's 8, `Busy` and `Settled`, and the schema's 22. The contract now states the schema: `VERBS`, five verbs in order — `snapshot` (result `text`), `click` (`id`), `type` (`id`, `text`), `press` (`key`, optional `shift`), `advance` (`ms`) — with `VerbSpec` · `ArgSpec` · `FieldSpec` and `verb(name)`; `ArgKind` `Id` (1 to 1024 bytes) · `Text` (0 to 4096) · `Key` (twelve `KEY_NAMES`) · `Flag` · `Milliseconds` (1 to 60000); the acting result `settled` · `busy` · `added` · `removed` · `changed`, `advance` adding `advanced_ms`; `validate(call: &Call) -> Result<Command, Refusal>`, which receives no `Session` and answers once per call in a fixed order (verb, then the call's arguments as passed, then the verb's arguments in schema order); `Command`, `Key`, `Call`, `ArgValue`; `Refusal` over `Cause`, closed at eight (`unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable`), and `Fault` (`Missing` · `Unnamed` · `Repeated` · `WrongKind` · `OutOfBound`), none with a `String` field.
- "Not built" was "a verb set, a settle verb or busy-source reply on the socket, …"; now "anything that runs a `Command`, a verb, a settle verb or busy-source reply on the socket, …". The verb set is stated and validated, executed nowhere; `validate` returns `unknown-verb` and `malformed` only.
- Existing Scopes: eight private modules (was five) — `command`, `refusal`, `schema` added, under no `cfg` gate.
**Why:** the chunk built the one table every later driver surface reads. Rules for later chunks: a later entry adds its verbs to `VERBS`, never a second table; a not-settled step is a result, not a refusal; `Command`, `Call` and `ArgValue` print an id and typed text under `Debug`, so nothing fields them; the crate root's `Command` is the schema's while `start` takes a `std::process::Command`.
**Kept:** `SessionError` stays twelve variants and the wire stays `hello` and `stop`; the three "no driver, CLI or MCP command exposes … yet" clauses of the Dioxus DOM bridge contract stand, since nothing runs a command.
**Ref:** .andromeda/runs/2026-10-07T12-34-00-wrap/

## 2026-10-07-act-by-id — a call runs on a held instance: `Session::run`, `Outcome`, `Session::with_time`
**Section:** §Standard Contracts → Driver session · Test harness · Headless stand · Dioxus DOM bridge · CLIs · §Established Decisions → [Driver session] · §Occupied Resources → Names · Filesystem · Process-wide state and threads · §Existing Scopes (escher-driver, blitz-tests) · §Conventions → Tests
**Change:**
- Driver session: the crate re-exports 33 names (was 32; `Outcome`). `Session::run(&Call) -> Result<Outcome, Refusal>` is the one entry point that runs a call, in process: `validate` first; `click` and `type` resolve their id through `element_ids()` and `get_node` (`not-found` otherwise; no id or `NodeId` kept between calls); `press` acts on what holds focus, one macOS-only arm also dispatching `deleteBackward:`; `advance` hands `ms` to the caller's step (`time-unavailable` without one) and reports the lesser of the step's return and `ms` — the time delivered to the app, not what a counter inside it gained; each acting verb is one settled step returning the diff. `Outcome` is `Screen` · `Acted` · `Advanced`; a not-quiet instance is a result. `Session::with_time` carries the caller's time step. Was "executed nowhere" and "not wired"; `stale`, `disabled`, `covered`, `off-screen` are still returned by nothing. "Not built" loses "anything that runs a `Command`" and "act by id".
- Test harness: exports `Key`, `Modifiers`; the input helper `apple_keybinding`. Headless stand: `TIMER_TICK_MS` (100), `timer_step`. CLIs: `escher-session` hands the timer's session that step.
- escher-driver names `accessibility` on dioxus-native-dom (was: no feature), so both seven_guis binaries compile its accessibility modules, with no platform adapter. Nine private modules (was eight; `execute`, with the crate's one `cfg` gate).
- Dioxus DOM bridge: the snapshot text and the diff are returned by driver commands in process, to the caller of `run`; `to_text` and `diff` each have one caller outside the tests.
- blitz-tests: six `stand_act_*` files (17 tests); `mod common;` read by 14 stand checks (was eleven), `mod session_common;` by eleven (was five). Ten citations re-pointed.
**Why:** the chunk built the executor. In process, nothing on the socket: the operator's answer at the plan's forks (the operator, 2026-10-07); the meaning of `advanced_ms` is the overseer's technical answer (2026-10-07). Rules it sets: nothing public runs a `Command` that did not come from `validate`; nothing resolved outlives a call; nothing fields an `Outcome` (`Debug` prints ids, names, values); the first command carrying a call or an outcome out of the process is a new crossing question.
**Kept:** the returned-value-only rule for a snapshot's text and a diff, with its ratification (the founder, 2026-10-07); the `unkeyed_actionable` clause; the wire; no range interaction model — a route entry owns it.
**Ref:** .andromeda/runs/2026-10-07T14-22-35-wrap/

## 2026-10-07-refusal-detection — the executor detects the screen-level refusals; a sixth verb `scroll`; the engine's into-view scroll widened
**Section:** §Standard Contracts → Driver session · Scrolling, selection, tree · Test harness · Dioxus DOM bridge · §Established Decisions → [Scrolling and input] · §Occupied Resources → Filesystem · Process-wide state and threads (citations only) · §Existing Scopes (escher-driver, blitz-dom: scrolling and selection, blitz-tests) · §Conventions → Tests
**Change:**
- Driver session: `VERBS` lists six (was five; `scroll`, one `id`), with `Command::Scroll` and `Outcome::Scrolled` carrying `in_view`; five acting verbs (was four). All eight causes are returned by code, six by the executor (was four, two). `click`, `type` and `scroll` resolve their id on the screen's snapshot: `stale` when the session's record holds it, else `not-found` (was `not-found` alone). A `click` or a `type` on a target that cannot take it is refused in the order `disabled` · `off-screen` · `covered` (was: it runs). The session keeps a record of id text, `SeenIds`, at most `MAX_SEEN_IDS` 4096 ids — a count; an id's length is unbounded (was "keeps no id"). The click point is the centre of the snapshot's `bounds` plus the viewport's scroll (was the layout border box's centre). `off-screen`'s meaning and remedy are reworded.
- Scrolling: `scroll_into_view` scrolls every scrolling box on the target's containing-block chain, innermost first, then the viewport; a nested box at once, the requested behaviour for the viewport. New reader `visible_region`. The hit walk reaches a row scrolled out of its box (measured, unfixed).
- [Scrolling and input]: "programmatic scrolls clamp to one scroller" now holds for `scroll_to` and `scroll_by`; `scroll_into_view` writes every box on the chain.
- Dioxus DOM bridge: `bounds` is viewport-relative except for a box that is itself scrolled, which reads shifted by its own scroll offset (measured, unfixed).
- Test harness: the input helper `scroll_into_view`. Scopes and conventions: nine `stand_act_*` files, 27 tests (was six, 17); `scroll_into_view_nested`; fourteen readers of `session_common`. Nineteen citations re-pointed.
**Why:** the chunk built the detection its route entry asked for. Two widenings are ratified by the founder (2026-10-07), each his own choice relayed verbatim by the overseer and confirmed by the operator at this wrap's escalation: the sixth verb, and the engine method widened for every document, not a second method. The record is the operator's decision at the plan's forks. Rules it sets: a refused call runs nothing; visibility is read from geometry, never from a hit; a verb joins `VERBS`, never a second table.
**Kept:** the bounds misreading, the hit walk and the unbounded id length are stated as measured and owned on the route, not fixed here; `scroll` is never refused for failing to reach its target.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/

## 2026-10-07-driver-command-spans — the driver's command span; `tracing` its third dependency
**Section:** §Established Decisions → [Driver session] · §Standard Contracts → Driver session · §Occupied Resources → Names, Process-wide state and threads, Environment variables (one citation) · §Conventions → Feature gating, Tests · §Inherited Defaults → Optional capabilities · §Existing Scopes (escher-driver, blitz-tests)
**Change:**
- escher-driver depends on three crates: blitz-test-harness, dioxus-native-dom and `tracing` (`{ workspace = true }`, no cargo feature, no `[features]` table). Was "blitz-test-harness and dioxus-native-dom only … no `tracing` dependency, span or event". `Cargo.lock` gained one line (`"tracing",` in the escher-driver entry) and no package; "`Cargo.lock` is byte-identical" now reads as the act-by-id chunk's measurement.
- Every call handed to `Session::run` leaves one `tracing` span: target `escher_driver`, name `command`, level INFO, built with `tracing::info_span!` (no `#[instrument]`), entered before `validate` and held until `run` returns. Eight fields, each recorded only where it applies: `verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`. `snapshot` records `verb` alone; a refused call `verb` and `cause` only. Never fielded: `in_view`, `advanced_ms`, the screen text or its length, the label, the record of ids, any argument, any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`. Was "nothing in the crate prints, logs or fields any of the four"; now the crate prints nothing, fields none of the four and fields those eight values. `run`'s signature is unchanged; its former body is the private `Session::execute`.
- `escher_driver` is a registered log target. escher-driver joins escher-telemetry under the ungated form.
- Four blitz-tests binaries re-execute themselves (was three): `stand_act_spans` runs two ignored children, `RUST_LOG=info` set on one and removed from the other, each installing the sink over an in-memory capture.
- Counts: `stand_act_*` 10 files, 29 tests and two ignored children (was 9, 27); `session_common` 15 readers (was 14); `mod common;` 15 stand checks (was 14).
- Citations into `execute.rs`, the driver's `lib.rs` and `Cargo.toml` and the telemetry `lib.rs` re-pointed.
**Why:** the chunk built the span the route entry owed. The ungated dependency was the operator's answer at the plan forks and carries no mark.
**Kept:** "the key types its `press` needs come through the harness's re-exports" — still true; the third dependency is `tracing`, not a key crate. The crate still installs no subscriber and reads no env var and no clock.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/

## 2026-10-07-driver-command-spans — the sink prints a closed span; the diff's lengths reach a log — both PROVISIONAL
**Section:** §Standard Contracts → Telemetry bootstrap, Dioxus DOM bridge · §Cross-cutting Patterns → Logging and timing · §Existing Scopes (escher-telemetry)
**Change:**
- Telemetry bootstrap: the sink has two record classes (was one line per printed event). The second, PROVISIONAL, is one line per closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited: the event line's prefix, `span={the span's name}` (`span` a reserved field name), the span's own fields in the order recorded, then the layer's `message`, `time.busy`, `time.idle`. Every pair, `span` included, is judged by the rule that judges an event's field under the span's target; an outside target's span writes no byte; the value of a `CONTENT_FIELDS` field is not stored; every other span field's value is held in process memory until the span closes. An event's line is unchanged, inside a span included. No public item added or changed.
- Logging and timing: the layer is stated once in the private `sink_layer`, with `with_span_events(FmtSpan::CLOSE)` and the crate-private field formatter `SpanFields`. Existing Scopes: module format also holds `SpanFields`.
- Dioxus DOM bridge: beside the founder-ratified "a diff leaves the process through the returned value only — no log, event, socket or file carries it", which is not reworded, the body now states, PROVISIONAL, that the three list lengths of a diff the driver returns (`added` · `removed` · `changed`, counts only) are recorded on the command span and printed by the sink at `info` or below. The snapshot text's clause is untouched: no text and no length of it is fielded.
- Citations into `format.rs` and the telemetry `lib.rs` re-pointed.
**Why:** a new record class in the sink is a boundary widening answered by the operator at the plan forks, not by the founder. Both marks were kept by the operator at this wrap — the first at its invocation, the second at its escalation halt — and wait for the founder's batch at the Epoch 4 boundary. Neither is discharged by a later chunk's wrap on its own judgement.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/

## 2026-10-09-founder-rulings — the driver-command-spans PROVISIONAL records ratified: the sink's closed-span line, the diff's sizes in a log
**Section:** §Standard Contracts → Telemetry bootstrap, Dioxus DOM bridge · §Cross-cutting Patterns → Logging and timing
**Change:**
- Telemetry bootstrap: the sink's second record class — one line per closed span of an admitted target — was "PROVISIONAL: a boundary widening on the operator's answer … not yet the founder's word", per the entry `2026-10-07-driver-command-spans — the sink prints a closed span; the diff's lengths reach a log — both PROVISIONAL`; now "a boundary widening, ratified by the founder (2026-10-09) as built". The line's shape and the rule that judges its fields are unchanged.
- Logging and timing: the `sink_layer` clause read "(PROVISIONAL, §Standard Contracts → Telemetry bootstrap)"; now "(ratified by the founder, 2026-10-09 — §Standard Contracts → Telemetry bootstrap)".
- Dioxus DOM bridge: the clause beside "a diff leaves the process through the returned value only — no log, event, socket or file carries it" was "PROVISIONAL: stated beside the ratified sentence, which is not reworded"; now "ratified by the founder, 2026-10-09": the sentence stands and gains the clarification that a diff's content never reaches a log while its three sizes — the counts of added, removed and changed — may.
**Why:** the founder ruled on both records at the Epoch 4 boundary (the founder, 2026-10-09, relayed verbatim by the overseer as his chosen option labels; the English wording of each ruling is the overseer's), which supersedes the operator's provisional answers by rule. Standing rule for later chunks: a count of a diff's nodes may reach a log, nothing of its content may.
**Kept:** the same day the founder ruled that an in-process driver call returning the snapshot text and the diff is not a boundary widening — it is the returned value his rule allows. No mark waited on that answer, so no clause is reworded for it.
**Ref:** .andromeda/runs/2026-10-09T19-17-00-wrap/

## 2026-10-09-audit-corrections-agent-surfaces — fork CI's package install is one bounded, retried script
**Section:** §Stack and Technologies → CI/CD · §Standard Contracts → CI contracts · §Infrastructure Patterns → CI/CD
**Change:**
- §Standard Contracts → CI contracts gains the package-install contract: `bash .github/scripts/apt-install.sh [--attempts N] [--bound SECONDS] [--pause SECONDS] <package>...`, the one statement of fork CI's package install — up to 3 attempts, each `apt-get update` then `apt-get install -y` under `sudo timeout --kill-after=10 120`; a failed call (the bound's expiry included) prints one stderr line `apt-install: attempt {n} of {N} failed in {update|install} (exit {status})`, pauses 10 s, and the next attempt starts from `update`; exits `0` · `1` every attempt failed · `2` usage, never 124 or 137; arguments checked before any command runs (package name `^[a-z0-9][a-z0-9+.-]+$`, option value `^(0|[1-9][0-9]{0,5})$`, `--attempts` and `--bound` at least 1, at least one package); constants `ATTEMPTS=3`, `BOUND=120`, `PAUSE=10`, `GRACE=10`; no environment variable, no file written; called by nine `run:` steps of ci.yml, by no leg of `ci-leg.sh`, never on the dev host; pinned by `InstallScriptTest` and `test_o_package_installs_are_bounded`. Named limit: a bound firing inside `apt-get install` can leave dpkg interrupted.
- §Infrastructure Patterns → CI/CD: was "Ubuntu jobs install `libfontconfig1-dev`"; now eight named jobs install it through that script, every package install of ci.yml is bounded and retried, no line calls `apt-get`, and the matrix job's `Install apt deps` action step carries `timeout-minutes: 10`, the file's one step timeout, with no job-level timeout.
- §Stack and Technologies → CI/CD: the row names the install script beside the leg runner (the runner image's `sudo`, GNU `timeout` and `apt-get`; no package or action added), and the `ci-scripts` job's `python3-yaml` install reads as a guarded step, reached only where `import yaml` fails.
**Why:** the install step hung two jobs of one CI run for about half an hour each, and nothing bounded it. One script was the operator's choice over a loop in each step or a third-party retry action, so the numbers are stated once and the retry is provable on the dev host. In the chunk's CI run eight jobs ran the script and the guarded call was not reached — a green run shows the script installs, never that the bound fires.
**Kept:** the leg contract's own sentence and its pin citation; no `ci-leg.sh` leg, env var, listener or crate is added, so §Occupied Resources takes no entry.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — the second upstream sync: the merge base, Parley's pin form and the merged dependency set
**Section:** §Project Intent → the `Upstream sync:` line · §Stack and Technologies → Text and fonts, Accessibility, Markdown app (rdme), CI/CD · §Established Decisions → [Dependency pinning] · §Inherited Defaults → Framework · §Conventions → Licensing exceptions
**Change:**
- Upstream sync line: was merge base `23354585`, merge commit `f00b0216`, 2026-10-06; now DioxusLabs/blitz `main` at `7832c177`, merge commit `9462a7e4`, 2026-10-10 — the next sync's merge base.
- Parley: was "from git at a pinned rev" (Stack), "taffy and parley are git dependencies pinned by `rev`" ([Dependency pinning]) and "taffy and parley at pinned git revs" (Inherited Defaults); now parley is a registry version, "0.12", and taffy the one git dependency pinned by `rev`.
- Text and fonts gains ICU4X: icu_casemap, icu_locale_core, icu_properties and icu_segmenter "2.3" with writeable "0.6" — icu_casemap and writeable optional behind blitz-dom's default `text-transform-icu` feature, the other three unconditional dependencies of blitz-dom.
- Accessibility: accesskit_unix was 0.23.0, now 0.24.0; accesskit_android was 0.8.0, now 0.9.0. Markdown app: comrak was 0.55, now 0.56.
- CI/CD: the Python helper scripts now include upstream's `wpt_area_changes.py` with its test file and escher's `test_blitz_tests_targets.py`, which uses the stdlib `tomllib` — the `ci-scripts` leg needs Python 3.11 or later, no package added; the runner's Python version was not read.
- Licensing exceptions: `wasm_hello` was "has no license field"; now it takes its licence from the workspace.
**Why:** the merge of upstream's 61 commits took upstream's dependency set whole, with no `cargo update`. The sync line is what the next sync measures its merge base from. The coupling rule of [Dependency pinning] is unchanged: parley, skrifa and vello still move together, now through a registry version.
**Kept:** the Taffy rev is spelled in no master, so its bump amends nothing; accesskit "0.25" is still the manifest's string, the lock's patch bump stated nowhere.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — `text-transform-icu` in four default lists, `writing-mode` opt-in, the `text-indent` sentence marked not established
**Section:** §Established Decisions → [Default features], [Document defaults], [Unsupported features] · §Occupied Resources → Names
**Change:**
- [Default features]: blitz-dom's default list was svg, woff, accessibility, system-fonts, file-input, custom-widget; it now ends with `text-transform-icu`. blitz's was `net`, `accessibility`, `tracing`; dioxus-native's and dioxus-native-dom's lists likewise gain `text-transform-icu`. A new sentence: `writing-mode`, CSS vertical writing modes, is in no default list — `stylo_taffy`'s is an empty feature, blitz-dom's forwards to it, dioxus-native-dom's and blitz's forward to blitz-dom's, and dioxus-native's to blitz-dom's and dioxus-native-dom's.
- [Document defaults]: the Stylo prefs list gains `layout.writing-mode.enabled`, set only under the `writing-mode` feature.
- [Unsupported features]: was "`text-indent` `hanging`/`each-line` do not work because their parsing is cfg'd out in Stylo"; now the two flags are read from the style and handed to Parley, and that they do not work is `recorded, not established`.
- Names: blitz's feature names were `net`, `accessibility`, `tracing`, `scrollbars`; now also `text-transform-icu` and `writing-mode`.
**Why:** upstream's features, arrived by the merge. The `text-indent` sentence quoted an upstream code comment that the merge removed; the code passes both flags as before, stylo is unmoved at 0.22.0, and the chunk took no reading of either flag — the operator, 2026-10-10, through the question dialog of this wrap, kept the sentence with its status stated instead of dropping it. A wrap that later measures either flag restates it as measured.
**Kept:** Names lists only the feature names it already listed crates for — blitz-dom's, stylo_taffy's and the Dioxus crates' feature names were never in it and are not added. The by-runner reading of `writing-mode` is test-plan §9's.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — Taffy's tree traits on `LayoutPassState`; the bounds reader renamed; upstream's script and layout additions
**Section:** §Standard Contracts → Layout, Dioxus DOM bridge, JavaScript (blitz-vibey-script) · §Existing Scopes → blitz-dom: layout, blitz-vibey-script, stylo_taffy, wpt
**Change:**
- Layout: was "`BaseDocument` implements Taffy's `TraversePartialTree` … `RoundTree`, `PrintTree`"; now the crate-private `LayoutPassState<'doc>`, which wraps the document for one layout pass and derefs to `BaseDocument`, implements the nine tree traits, `BaseDocument` implements none of them, and `PrintTree` is implemented for `TaffyDebugTree`. `stylo_taffy`'s export list was `to_taffy_style`, `TaffyStyloStyle`, `StyleFlags`, `Atom`; it gains `to_taffy_style_in`, `WritingMode` and, from a public `writing_mode` module, `WritingModeExt`.
- Dioxus DOM bridge, the `bounds` clause: was "the reader positions a node through `unrounded_absolute_position(0, 0)`"; now through the crate-private `BaseDocument::physical_unrounded_geometry`, which has two bodies — one without the `writing-mode` feature, one with it — each summing `location − scroll_offset` over the containing-block chain, the node's own offset included. The consequence the clause records, a scrolled box's `bounds` shifted by its own offset, stands.
- JavaScript: a script reaches the document's text selection through `document.getSelection` (`setBaseAndExtent`, `removeAllRanges`, `toString`), written through `BaseDocument::set_text_selection` with no `mutate()` call — read from the code at the chunk's research, not traced at run time — and reads `innerText` / `outerText` through read-only getters; escher uses none.
- Existing Scopes: blitz-dom layout gains `text_transform` and `writing_mode` (the second compiled only under the feature); blitz-vibey-script gains module `inner_text`, dom submodule `selection` and `tests/selection.rs`; stylo_taffy gains `writing_mode`; wpt gains `test_variants`.
**Why:** upstream moved the tree traits off the document and removed `Node::unrounded_absolute_position`; escher's one edit to upstream-owned code moved `visible_region`'s call onto the new reader, signature kept. The Selection API and the getters are realizations on the script-to-DOM crossing already registered, as the first sync's accessors were recorded — no read-only channel gains a write.
**Kept:** methods of an already-registered type and functions inside an already-registered module (`TaffyStyloStyle::new_in`, `Node::layout_style_in`, `full_width` and the like) are not registered: the contract lists crate-root exports. The route's own citation of the removed reader is route-resolve's.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — blitz-tests keeps one test target per file; upstream's `all` target stands unbuilt, pinned by a fourth test of ours in the `ci-scripts` leg
**Section:** §Conventions → Tests · §Standard Contracts → CI contracts
**Change:**
- Tests: a new clause — blitz-tests keeps one test target per file directly under `tests/`; its `[package]` sets no `autotests`; the one file there that is declared and never built is upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, held at `test = false`; a standing two-line difference from upstream's manifest, pinned by `BlitzTestsTargetsTest`.
- CI contracts: a new clause — the blitz-tests target contract (`[package]` does not set `autotests` to false; exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false`) is pinned by `test_blitz_tests_targets.py`, one test, run by the `ci-scripts` leg, which discovers six test files. The CLI list of `wpt_diff_to_pr.py` was `diff_file`, `--repo`, `--pr`, `--run-url`, `--dry-run`; it gains `--areas`, the per-area subtest changes upstream's `wpt_area_changes.py` computes from the two reports.
**Why:** upstream moved to one test binary (`autotests = false` with a single `all` target); the operator, 2026-10-10, by question dialog at the plan, kept every file its own target: under upstream's shape `--test {name}` names no target, `mod common;` and `mod session_common;` do not resolve from a module, the re-executing checks select a child by a bare `--exact` name that would match nothing and still exit 0, and the telemetry checks need a process each. Taking upstream's table whole and switching it off keeps the difference at two lines for the next sync. A sync that drops the `test = false` line, or takes `autotests = false`, turns the pin red.
**Kept:** `wpt_area_changes.py`'s function names are not registered — the contract lists scripts and their CLI.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-driver-cli — the session socket carries a call and its answer; a hosted session expires
**Section:** §Established Decisions → [Driver session] · §Occupied Resources → Network ports and listeners · Filesystem · Process-wide state and threads · §Inherited Defaults → API style
**Change:**
- [Driver session]: was "lifecycle messages only … nothing of the screen on the wire"; now the socket carries the lifecycle and a driver call with its answer — an element id and typed text in; ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out; no `NodeId`. The `std`-only socket, no third-party IPC, no port and the three-crate dependency set stand.
- [Driver session]: was "a session has no idle expiry"; now `serve` takes one — `IDLE_EXPIRY`, 1,800 s, for a host a command line starts — and a host with no request answered for that long ends exactly as on `stop`, every answered request starting the count again.
- Network ports and listeners: the one listener serves three requests at wire `v2` — `hello`, `stop`, `call`; a request line at most 16,384 bytes, an answer at most 1,048,576. Still no second listener and no port.
- Filesystem: a state directory is named by `--session <dir>` and removed at `stop` or at the idle expiry; the checks' own directories are `ss-life`, `ss-idle`, `ss-quiet`, `hb-*`, `hl-trace`, `cc-*`, `cf-*`, `ht-timer`.
- Process-wide: the `escher_driver` span is now also written by an `escher-session` host, one closed-span line per hosted call; the host is started by hand or by the binary's own `start` command, which spawns the running binary again through `std::env::current_exe()` with its three streams closed and neither kills nor reaps it, and it ends on `stop` or its expiry; `stand_session_lifecycle` re-runs two ignored host children; five seven_guis targets spawn the binary.
- API style: the socket is no longer named a lifecycle socket.
**Why:** the chunk built the command line, and a command leaves the process only by crossing the socket. The crossing is a second widening of the 2026-10-07 boundary, ratified by the founder (2026-10-10), by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Сокет несёт команды» (the English wording is the overseer's), and confirmed not PROVISIONAL by the operator at the plan's review; the idle expiry is the founder's answer, the same day, to the orphaned-host question. From here an id, a name and a value leave the process by design: the next surface that carries them, MCP, asks again.
**Kept:** no log carries any of it — a diff's three sizes may, its content never.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — Driver session contract: wire `v2`, `serve` with an idle expiry, the client's `call`, thirteen errors, 40 re-exports
**Section:** §Standard Contracts → Driver session (the re-exports, `serve`, the lifecycle, `SessionError`, the wire, the non-unix arm, the Not built list)
**Change:**
- Re-exports: was 33 names; now 40 — the session's eleven (`IDLE_EXPIRY`, `call` and `Answer` joined), the schema's 25 (`SessionCommand`, `validate_session`, `Level` joined), `Outcome`, `command_line`, and the harness's two.
- `serve(state_dir, session, idle_expiry)`: a third parameter; a non-blocking accept loop polled every 25 ms; a `call` request is run through `Session::run` and nothing else; every answered request — a refused or malformed one included — counts toward `served` and starts the idle count again, a connection that breaks before its request is whole does neither.
- The lifecycle: `Hello` gains `idle_expiry_s`; `call(state_dir, &Call) -> Result<Answer, SessionError>` with `Answer { accepted, json }` validates first, so a call the schema refuses reaches no session and every call sent fits the request bound; an answer is read under 30 s per read.
- `SessionError`: was twelve variants; now thirteen — `AnswerTooLarge`, read from an `oversize` reply, the call having run — with `kind()`, one fixed kebab-case word per variant, and `to_json()`.
- The wire: was `v1`, `hello` and `stop`, 64-byte requests; now `v2` with `call v2 <verb> [<name>=<kind>:<value>]…` (kinds `t` · `n` · `f`, strict percent-escapes), requests at most 16,384 bytes, lifecycle replies at most 128 (the widest `hello` measures 122, was 87), answers `ok v2 accepted` · `refused` · `oversize` bounded at 1,048,576 bytes on the JSON line.
- Non-unix: `call` joins the functions that return `Unsupported`.
- Not built: now a settle verb or busy-source reply of its own on the socket, and an MCP tool; a verb on the socket, CLI JSON and an idle expiry left the list.
**Why:** the contract is the crate's public surface, and the chunk changed each of these. `client::call` validating first was decided at implement: the plan did not say what a call over the request bound does, and checking first means none is ever sent.
**Kept:** `Session::start`, `act`, `with_time`, `attach`, `stop` and `start` read as they did; `start` still does not check that the answering pid is its child.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — Driver session contract: nine verbs in two levels, the written forms, `command_line`; the `escher-session` CLI
**Section:** §Standard Contracts → Driver session (the schema, `validate`, the written forms, the command line) · §Standard Contracts → CLIs
**Change:**
- The schema: was "held in process — no item of it is reachable from the socket"; now a call reaches it from outside the process too — a host decodes a `call` request and hands it to `run`; the client and the command line validate before anything is sent.
- `VERBS`: was six verbs; now nine in two levels (`Level::Instance` · `Level::Session`) — the six, then `start` (`app`; answers `label`, `pid`, `idle_expiry_s`), `status` (adds `served`), `stop` (answers `stopped`); `VerbSpec` carries `level`. `ArgKind` gains `Name` (1 to 32 bytes of `a-z`, `0-9`, `-`), the sixth; `FieldKind` gains `Count`, the seventh.
- `validate` refuses a session-level verb as `unknown-verb` before its arguments are read; `validate_session(&Call) -> Result<SessionCommand, Refusal>` checks the three session-level rows by the same rules; `SessionCommand` is `Start { app }` · `Status` · `Stop`. `Command` stays six variants.
- Written forms, new: `Outcome::to_json`, `Refusal::to_json`, `SessionError::to_json` — one line of JSON each, keyed by the schema's own words, hand-written, output only; the node, refusal and error shapes are stated.
- `command_line(args, apps, boot) -> ExitCode`, new: the reader's rules in their fixed order, the flag spelling, four endings (status `0` accepted · `1` refused · `2` usage · `3` session error) and the host role's quiet end, no ending writing back caller input, and `start` spawning the running binary as `serve <app> --session <dir>`.
- "The crate prints nothing" narrowed: it writes a command's answer on its command line; the executor, session, schema, validation and JSON writer print nothing and read no clock or environment.
- CLIs: was `escher-session <task> <state-dir>`, exits 0 · 1 · 2, nothing on stdout; now `escher-session <verb> [arguments] --session <dir>` over the nine verbs, the host role `serve <task> --session <dir>`, one line of JSON on stdout, exits 0 · 1 · 2 · 3; the two-argument form reads refused, `unknown-verb`.
**Why:** v010-12 — every driver command from the command line. The three session-level verbs and the sixth kind widen the validated verb set, ratified by the founder (2026-10-10) with the crossing; the JSON is written by hand on the operator's answer at the plan's forks (2026-10-10), the question re-opened at the MCP surface.
**Kept:** one verb table: the usage line and the argv reader read `VERBS`, and no second list exists.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — `type` replaces; the record refuses a long id; two engine limits measured and said by the tool; the snapshot text and the diff leave in an answer
**Section:** §Standard Contracts → Driver session (the executor, the record of ids, the cause texts, the span) · §Standard Contracts → Dioxus DOM bridge (the snapshot text, the diff)
**Change:**
- `type`: was "clicks … and types"; now it replaces — where the snapshot reads a `value` for the target (an empty text input reads one) the driver presses the engine's select-all, `a` with Control, Super on macOS, then types, and with an empty text deletes the selection, so an empty text clears. Until this chunk the text was appended after the old value (measured), not inserted inside it.
- The executor's `cfg` gates: was one, the macOS backward-delete arm; now two, the second choosing the select-all's modifier.
- The record of ids: was "nothing bounds a recorded id's length … not measured"; now an id longer than `MAX_ID_BYTES` is not recorded, held by a unit test.
- A click naming a scrolled box: was a hypothesis "read from the code, not measured on a click"; now measured in both layout modes — it lands off the box's centre by the scroll offset (shift 76 on the stand at 14 Creates) or is refused `off-screen` while the box is in view.
- `covered`: a control before a scrolling box in the document, lying where a scrolled-out row extends, is refused `covered`; one after the box is not; no control of the stand's CRUD reads it.
- Four fixed texts amended so the tool says both limits and what `type` does: `covered`'s meaning, the help of `changed`, of `snapshot`'s `text` and of `type`. The cause set stays eight.
- The span: a session-level verb handed to `run` records its table word beside `unknown-verb`.
- Dioxus DOM bridge: was "no CLI or MCP command exists and nothing of it crosses the socket", the text and a diff leaving "through the returned value only"; now a hosted answer carries the text and a diff — which the driver, not dioxus-native-dom, writes as JSON — over the socket to stdout; the hosted snapshot line measures 832 to 2,187 bytes on the four tasks. No log, event or file carries either.
**Why:** `type` replacing is the founder's answer (2026-10-10, relayed by the overseer); the record's rule is the operator's at the plan's forks. Both engine limits were measured and left unfixed on the founder's answer — measure and state — and the operator's review added that the tool itself must say them, since an agent learns the tool from the tool. The operator's 2026-10-07 answer that a call runs in process is superseded by the ratified crossing.
**Kept:** both engine defects stand in the engine, pinned by `stand_act_scroll` and `stand_act_obstructed`; each fix keeps a route owner.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — scope rows after the command line: eleven driver modules, five seven_guis targets, eleven driver-action checks; a shell in one test target; the front page
**Section:** §Existing Scopes → escher-driver · seven_guis · blitz-tests · §Conventions → Tests · §Stack and Technologies → Testing · §Project Intent → Front page
**Change:**
- escher-driver: was nine private modules, an in-process definition and executor, a lifecycle line protocol; now eleven — `json` (the written forms) and `cli` (`command_line`) joined — `client` gains `call`, `host` runs hosted calls and ends on a stop or its idle expiry, `wire` is the line protocol of the lifecycle and of a call and its answer, `command` gains `SessionCommand` and `validate_session`, and `execute` carries two macOS `cfg` gates. Dependencies and the one named feature unchanged.
- seven_guis: `escher-session` is the driver's command line and, in its host role, the session host; was two integration-test targets, now five — `cli_commands`, `cli_flow` and `host_timer` joined — with `tests/flows/`, two POSIX `sh` scripts that are no target.
- blitz-tests: was ten `stand_act_*` files, 29 tests, fifteen readers of `session_common`, 15 readers of `common`; now eleven files (`stand_act_filled`), 37 tests, sixteen and 17 readers; the row names what the new tests cover and the `snapshot` call builder.
- Conventions → Tests: ten driver-action checks becomes eleven.
- Testing: seven_guis' unix-gated `cli_flow` runs two `sh` scripts under plain `sh` against the built binary — a shell is that one target's requirement; no dependency, manifest line or test framework was added.
- Front page: the README now also says how the command line is run and names the MCP surface, the driver's self-description and the screenshot as still to come.
**Why:** each row restated a count or a name the chunk moved. The reader counts were measured at this wrap, since the report gives the new file but not which shared modules it declares.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — a scrolled box reads its own bounds, and a hit stops where a box clips by overflow
**Section:** §Standard Contracts → Dioxus DOM bridge (the `bounds` clause) · §Standard Contracts → Scrolling, selection, tree · §Established Decisions → Scrolling and input
**Change:**
- Dioxus DOM bridge: was "true of every element but a box that is itself scrolled … `bounds` read shifted by its own offset … unfixed and owned by a route entry"; now `bounds` is viewport-relative for every element. `BaseDocument::physical_unrounded_geometry` subtracts the scroll offset of every box on the containing-block chain above the node and not the node's own, in both of its bodies: a scrolled box reads the same rect before and after its content is scrolled, a scrolled box inside one moves by the outer offset alone, and a scroll's diff does not name the box. `offset_rect` and `inline_fragment_rects` apply their origin box's offset themselves and answer as before. No check reads a scrolled box under a vertical writing mode.
- Scrolling, selection, tree: was "a clipping box's own scroll offset is cancelled where that box is positioned" and "the hit walk passes a point outside a scrolling box to that box's scrolled-out children … unfixed — the hit walk is not edited"; now `visible_region` places each clipping box where the reader puts it, with no add-back, and reads as before, and `Node::hit` / `hit_inner` test a node's own area where the box stands and stop at the padding box of a node that clips by `overflow` — either axis not `visible`, the root element excepted, the two axes not told apart.
- Same contract, four readings stated and owned by route entries: the walk does not stop at a `contain: paint` box with a `visible` `overflow` (measured), and an image's, a sub-document's and a text input's own box are not measured; a one-axis clip stops the hit at the whole padding box while `visible_region` narrows the clipped axis only; no check reads a box both transformed and scrolled; `Node::absolute_position` is unedited — a box at (40, 50) scrolled by (70, 120) reads (−30, −70) — and the harness's `layout_rect`, `layout_rect_of` and `center_of` read with it.
- Established Decisions: gains both behaviours as engine changes for every document, flagged upstreamable against upstream `main` at 7832c177, no upstream pull request open.
**Why:** the founder ruled (2026-10-10, by question dialog in the overseer session, relayed by the overseer) that both defects are fixed in the engine, each a direct change, ahead of the MCP surface; the operator's answers of that day set how far each reaches — the shared reader, paint's overflow half, the second reader measured and left.
**Kept:** `Node::absolute_position` as it reads, and the hit not stopped at `contain: paint` — both by the operator's answer, each owned on the route.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — the driver's three texts restated, and the checks that pinned the two limits
**Section:** §Standard Contracts → Driver session · §Standard Contracts → Test harness (the `scroll_into_view` helper) · §Existing Scopes → blitz-tests
**Change:**
- Driver session, the texts: was "`covered`'s meaning now ends "a hit reaches content scrolled out of a scrolling box…"" and "the help of the result field `changed` and of `snapshot`'s `text` state the scrolled-box bounds limit"; now `covered`'s meaning ends with the sentence that a hit reaches content clipped by `contain: paint`, the two helps read as they did before 2026-10-10-driver-cli, and `type`'s help stands. The cause set is eight, in its order; the driver's logic is unchanged and no file of it states either fixed limit.
- Driver session, the readings: was "The `covered` limit as measured … the engine is unchanged, the reading is pinned by `stand_act_obstructed`" and "those `bounds` read shifted by the box's own scroll offset … lands off its centre … pinned by `stand_act_scroll`"; now a control lying where a scrolled-out row extends, before the box or after it, is clicked as itself and no element reads focused after the click, and a `click` or a `type` naming a scrolled box lands at its centre — on the stand `crud-list` reads the same four figures after its content is scrolled, at 12 Creates and at 14. One reading is still off: content clipped by `contain: paint` beside a `visible` `overflow`.
- Test harness: was "the driver's `scroll` is its one caller"; now `stand_actionable_keys` calls `Harness::scroll_into_view` too, before each raw click on a Home card. The helper still has no check of its own.
- blitz-tests: was "the two measured engine limits pinned as they read"; now the two fixed readings, and the row names `scrolled_box_client_rect` (6 tests), `hit_clipped_at_scrolling_box` (9) and `scrolled_box_absolute_position` (1).
**Why:** the four texts were amended at 2026-10-10-driver-cli so that the tool said two measured limits where an agent reads it; both limits are now fixed in the engine, so three texts say what is true again and one limit, the `contain: paint` one, is still said by `covered`'s meaning. The second caller of the helper was found by the cascade sweep, not by a detector.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-snapshot-states — the snapshot model reads selected, pressed and invalid; the driver's node carries them
**Section:** §Standard Contracts → Dioxus DOM bridge · §Standard Contracts → Driver session
**Change:**
- Dioxus DOM bridge: was "`NodeState` carries `enabled`, `checked`, `value` and `focused`"; now seven fields in the order `enabled` · `checked` · `value` · `selected: Option<bool>` · `pressed: Option<bool>` · `invalid: bool` · `focused`, no new public type. The three are the accessibility node's own, read at the one construction site and never parsed from the element: `is_selected()`; `toggled()` with only `Toggled::True` reading `Some(true)`; `invalid().is_some()` — a boolean model with two limits, a mixed toggle reading not pressed and a grammar or spelling error reading invalid, the node keeping the full value. On the stand each reading equals its accessibility node's.
- `Snapshot::to_text`: the fixed token order gains `selected` / `unselected` · `pressed` / `unpressed` · `invalid` (only when true) after `value=` and before `focused`.
- Measured sizes: the boot text was Counter 755 · FlightBooker 1269 · Timer 1259 · Crud 2034 bytes, now 755 · 1287 · 1259 · 2049; the hosted `snapshot` line was 832 · 1,375 · 1,368 · 2,187, now 832 · 1,393 · 1,368 · 2,202.
- Driver session: `NODE_FIELDS` was nine words (`[&str; 9]`), now twelve — `selected` · `pressed` · `invalid` between `value` and `focused`. In JSON, `parent`, `enabled`, `checked`, `value`, `selected` and `pressed` are left out when absent (was the first four); `invalid` is always written.
- The help of `changed` and of `snapshot`'s `text` were quoted as the words that stood before 2026-10-10-driver-cli; both are restated and quoted as they now read — each names the state words and says the two limits. The crate docs gain one sentence saying the same.
**Why:** the founder brought the three states into 0.1.0 as their own chunk (2026-10-10): a snapshot reads which control is selected, pressed or invalid from what the app states in its markup, and an agent learns the words and the limits from the tool.
**Kept:** a boolean model, not a tri-state `pressed` or a kind on `invalid`, and no help per node key — the operator's answers at the chunk's plan forks; the per-key help is owned on the route.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-snapshot-states — the accessibility builder reads three state attributes; the adapter's third class; a third direct engine change
**Section:** §Standard Contracts → Mutation, query and CSSOM · §Established Decisions → Scrolling and input
**Change:**
- Mutation, query and CSSOM: the builder's contract named its three name sources only; it now also states three state reads on every element, whatever its role and with no role consulted, each value trimmed of ASCII whitespace and compared exactly in lower case — `aria-selected` `true` / `false` → `selected`; `aria-pressed` `true` · `false` · `mixed` → `toggled` `True` · `False` · `Mixed`; `aria-invalid` `true` · `grammar` · `spelling` → `invalid`, an absent, empty or `false` value nothing, any other value `Invalid::True`. The lower-case match is a choice: `TRUE` reads unrecognized. `aria-checked` is not read; nothing else writes `toggled`. No log site, no panic path.
- Same contract, the shell clause: the platform adapter received "the same content classes (ids as `author_id`, accessible names)"; it now also receives a third class, states — `selected`, `toggled`, `invalid`, closed enums and booleans, never an attribute's value as typed. What an assistive technology reads of them is unmeasured; the windowed witness is owned by a route entry.
- Established Decisions: was two direct engine changes for every document, "both flagged upstreamable"; a third stands beside them, in the builder's own file, flagged upstreamable against the same upstream `main` at 7832c177 (`accessibility.rs` +57 −1 against it, +26 −0 before), no upstream pull request open.
**Why:** ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Ратифицировать»; confirmed by him at this wrap's card, relayed verbatim the same way as his chosen option label «Подтвердить всё»; the English wording is the overseer's): the builder writes the three states on the node — a widening of the adapter crossing by a new author-derived class. A later attribute read keeps this shape: a closed set, no text carried out.
**Kept:** `aria-checked` unread and no role table in the engine — the operator's answers at the chunk's plan forks; an ARIA checkbox's reading is owned on the route.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-snapshot-states — scope rows: twelve driver-action checks, seventeen readers, a third accessibility check
**Section:** §Conventions → Tests · §Existing Scopes → blitz-tests
**Change:**
- Conventions → Tests: was "the eleven driver-action checks `stand_act_*`"; now twelve.
- blitz-tests: was "the eleven files … 37 tests and two ignored children"; now twelve files, `stand_act_states` the twelfth, 40 tests and two ignored children, the covered behaviours gaining a step that returns the node whose pressed, selected or invalid reading it changed. `mod session_common;` was read by sixteen checks, now seventeen; `mod common;` stays at 17.
- blitz-tests, accessibility coverage: gains `accessibility_states` — the builder's three state attributes on a parsed page, every value of each grammar, on a role that supports the attribute and one that does not, `aria-checked` setting none, 4 tests in both layout modes. It runs in the workspace test leg and per package; the `a11y` CI leg stays its three named files.
**Why:** the chunk added two check files and no manifest line; each count is a measured reading of the tree.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-mcp-surface — the MCP server and its `mcp` role; the MCP crossing ratified
**Section:** §Standard Contracts → Driver session (escher-driver) · CLIs (escher-session) · Dioxus DOM bridge · §Established Decisions → [Driver session] · §Occupied Resources → Network ports and listeners · Names · Process-wide state and threads · §Project Intent → Front page
**Change:**
- Driver session: re-exports 40 → 41 (`serve_mcp`); was "an MCP tool" on the Not built list, now built. `serve_mcp(input, output, apps, boot, idle_expiry)` speaks newline-delimited JSON-RPC 2.0 in `2025-06-18` · `2025-03-26` · `2024-11-05` (else answers `2025-06-18`), `initialize` · `ping` · `tools/list` (the nine `VERBS`) · `tools/call`, every other method `-32601`, notifications run nothing; bounds 65,536-byte request line, the `id` echoed only as an i64 or a string ≤ 128 bytes, a result ≤ 1,048,576 bytes; fixed protocol errors with no `data`; results `{content, isError}` — `snapshot` the screen text, else the JSON line, no `structuredContent`; idle expiry checked on arrival. `command_line`'s boot `FnOnce` → `FnMut`; the reader takes `mcp` alone as a role; the role's end writes nothing on stdout (0 / 3); usage gains `; mcp role: mcp`. Was "writes one thing", now two: command answers and MCP replies. A live client sent `server/discover` first, then asked `2025-11-25`.
- Dioxus DOM bridge: was "no MCP command exists" (twice); the snapshot text and an acting call's diff now also reach an MCP client.
- [Driver session]: the calls also cross over the `mcp` role's stdio — no listener — with the three bounds above.
- Occupied Resources: the stdio channel (still one listener, no port); the MCP server name `escher`; the role one process, one session, no thread, a third reach of `init`; seven_guis targets five → seven (`mcp_server`, `mcp_log`, shared `Speaker`).
- Front page: the README now says how an MCP client drives the apps; "the MCP surface … still to come" retired.
**Why:** a boundary widening, ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Ратифицировать как есть»; his confirm given at this wrap's Validate, relayed verbatim by the overseer as his chosen option label «Подтвердить всё», as the operator's answer; the English wording is the overseer's) — recorded ratified, no PROVISIONAL mark. The next surface that carries a call or a screen's content is a new crossing question.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-10-mcp-surface — `serde_json` a reader only; the `tool_call` span; twelve modules, four crates
**Section:** §Stack and Technologies (Media) · §Established Decisions → [Driver session] · §Conventions (Feature wiring) · §Standard Contracts → Driver session (the written forms, the span) · §Occupied Resources → Names (escher-driver) · Logging · §Cross-cutting Patterns (Logging) · §Inherited Defaults (Optional capabilities) · §Existing Scopes (escher-driver · seven_guis · blitz-tests)
**Change:**
- `serde_json "1"` a root workspace entry (1.0.151, already in the graph through dioxus-html), escher-driver's fourth crate: was "three crates", now four; it reads MCP requests and writes nothing — was "no serialization crate entered … re-opened at the MCP surface", now the written forms stay hand-written, `json`'s `string` / `Object` `pub(crate)`; the lock gained one line and no package.
- A second span, `tool_call` (target `escher_driver`, INFO, `tool` and `ending` only), one per MCP `tools/call`, the parent of `command` by nesting; was "its one command span" in Conventions, Inherited Defaults, the Logging bullet and Cross-cutting, now two spans; the `mcp` role's stderr reads 5 `tool_call` + 3 `command` lines over one CRUD flow at `info`, 0 needles at `trace`.
- Existing Scopes: eleven → twelve private modules (`mcp`); the dependency list gains `serde_json`; seven_guis' `escher-session` gains the `mcp` role and its targets five → seven; blitz-tests gains `stand_mcp_tools`.
**Why:** the founder chose `serde_json` as a reader only (2026-10-10, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «serde_json, только чтение»; the English wording is the overseer's) and the `tool_call` span with `tool` and `ending` («Span с tool и ending», relayed verbatim the same way) — a boundary widening (a new log field), ratified by him and confirmed at this wrap's Validate, relayed verbatim by the overseer as his chosen option label «Подтвердить всё», as the operator's answer. Standing rule: no span field holds what a call supplied or a screen reads.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-12-self-description — the self-description: `help`, `describe`, the tables, `differs`, four revisions
**Section:** §Standard Contracts → Driver session (escher-driver) · Dioxus DOM bridge (the diff) · CLIs
**Change:**
- Driver session: was 41 re-exports, now 45 (`describe`, `NodeFieldSpec`, `ExitStatus`, `EXIT_STATUSES`; the schema's 25 → 28). Was nine verbs in two levels, now ten in three: `Level::Tool` with `Level::name()`, and the row `HELP` — no argument, seven result fields `verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`. `FieldKind` gains `Words` · `Entries` (nine). Was `NODE_FIELDS: [&str; 12]`, now `[NodeFieldSpec; 13]` `{ name, help }`, with `differs` after `bounds` on a changed node only; the help states each reading, the boolean model's two limits among them. `EXIT_STATUSES: [ExitStatus; 4]` is read by the command line's statuses. `describe()` writes the self-description from the tables alone. `SessionError` gains `remedy()` over the crate-private `SESSION_ERRORS: [ErrorSpec; 13]`: `dead` and `answer-too-large` share the earlier row's remedy, and `StateDirTooLong` states 94 bytes on Linux, 90 elsewhere. `SessionCommand` gains `Help`; `validate_session` admits it and refuses an argument `malformed`, while `validate` refuses it `unknown-verb`. The written forms: was `{"error":{"kind","message"}}`, now with `remedy`; a changed node carries `differs`; `json::list` is `pub(crate)`. The command line gains the `help` ending (0 · 1 · 2, no session reached). The MCP server: was three revisions, else `2025-06-18`; now four, else `LATEST_VERSION` `2025-11-25`. It lists ten tools whose descriptions carry their result fields, and answers `help` itself — no `command` child, counted in `served`. `help` joins `command.verb` and `tool_call.tool`, with no field added. "Not built" loses the self-description.
- Dioxus DOM bridge: `DiffNode` gains `differs: Vec<Reading>` (`Reading::ALL` order, filled where `changed` is decided, empty when added); `pub enum Reading` (eleven) is re-exported, `pub use` 15 → 16; the diff's JSON restatement gains `differs`.
- CLIs: was the nine of `VERBS`, now ten, `help` needing no `--session`; the statuses are read from `EXIT_STATUSES`.
**Why:** the chunk built the self-description on the founder's answers 1A and 2A; the revision is the operator's own answer at the plan forks. Every served text is the table's fixed `'static` text, the one place it is stated.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-self-description — fixed program text on the ratified carriers, ratified; scopes, front page, readers
**Section:** §Established Decisions → [Driver session] · §Occupied Resources (the `escher-session mcp` row) · §Project Intent → Front page · §Existing Scopes → escher-driver · dioxus-native-dom · blitz-tests · §Conventions → Tests
**Change:**
- [Driver session] and Occupied Resources: the command line's stdout and the `mcp` role's stdio also carry the program's own fixed `'static` table text — the `help` self-description, a session error's remedy, each tool's description with its result fields. Text from a page, a call or a screen, a new carrier or a new exit still widens. `differs` crosses as a closed set (rule of 2026-10-10). The role serves `help` and speaks `2025-11-25` by default. Still no listener, socket or port; no dependency added; `serde_json` a reader only.
- Front page: was "the nine driver commands" with the self-description to come; now ten, how to ask `help` on both surfaces, only the screenshot to come.
- Existing Scopes: escher-driver was twelve private modules, now thirteen (`describe`); schema in three levels with node-key help and `EXIT_STATUSES`; error with its table; cli's `help` ending; mcp's `help` tool and four revisions. dioxus-native-dom's snapshot_diff re-exports `Reading`. blitz-tests gains `stand_mcp_masked` and `stand_mcp_tools`' remedy and `help` checks.
- Readers of `tests/session_common/mod.rs`: was seventeen, now eighteen — `stand_mcp_tools` since 2026-10-10-mcp-surface (Existing Scopes and Conventions → Tests).
**Why:** a boundary widening ratified by the founder (2026-10-12, «Внутри, с правилом» and «help — десятый глагол», by question dialog in the overseer session, relayed verbatim by the overseer; his confirm of the wrap's answer card, «Подтвердить всё», the same channel) — recorded ratified, no PROVISIONAL mark. The playbook gained the rule his answer set. The readers' count was disproved by measurement (18 files declare the module).
**Kept:** `stand_diff` is not named in the blitz-tests row, which never listed it; the diff contract cites it.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-headless-screenshot — the `screenshot` verb, `fonts-unavailable`, `Outcome::Shot` and the MCP image block
**Section:** §Standard Contracts → Driver session (escher-driver) · CLIs
**Change:**
- Was ten verbs, six asked of the instance; now eleven, seven: `screenshot` after `scroll` — one optional `id` (`ArgKind::Id`), result `width` · `height` (counts) and `png` (`FieldKind::Png`, base64 text), all always present; its help states two limits (an element shot is cropped to what can be seen; a thumb painted within 0.7 s of a scroll fades by wall time where overlay scrollbars are painted). `FieldKind` nine → ten. `Command` gains `Screenshot { id: Option<String> }`; `describe()` counts 11 verbs and 9 causes.
- Was `Cause` closed at eight, the executor returning six; now nine, seven: `fonts-unavailable` last, from `screenshot` alone, checked before anything is painted.
- `Outcome` gains `Shot { width, height, png: Vec<u8> }`. `screenshot` dispatches nothing and is no step; given an `id` it resolves it as `click` does, crops the frame to the element's visible part rounded outward (private `crop`), a crop of no pixel refused `off-screen`; a font context with no `sans-serif` family (`can_paint_text` false) is refused `fonts-unavailable`; the frame is the harness's, written as a PNG by the private `image::png`; a shot changes nothing. On the stand a whole shot is an 800 × 600 PNG equal to the harness's raster.
- Written forms gain the shot's line `{"width","height","png"}`, its base64 hand-written (`image::base64`, padded) — no base64 crate. Was every MCP result one text block bounded by its text; now an accepted `screenshot` is an `image` block then a text block (the line without `png`), a refused one one text block, and a result holding an image is bounded by its whole written form (`ToolResult::bounded`). `tools/list` eleven.
- The `command` span keeps its eight fields: a shot records `verb` (and `cause`), never the image, its size or dimensions; `tool_call.tool` reads `screenshot`.
- CLIs: `escher-session screenshot [--id]`.
**Why:** the chunk built the headless screenshot; the image in band is the founder's fork 1, ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's answer card confirmed the same way, 2026-10-13).
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-12-headless-screenshot — the harness's screenshot, the password mask, the image in band, `png`
**Section:** §Standard Contracts → Test harness · Document core (blitz-dom) · Paint and shell · §Established Decisions → [Renderer backends] · [Painting] · [Driver session] · §Occupied Resources → Network ports and listeners · Names
**Change:**
- Test harness: exports gain `PixelRect`, `Raster`, `ScreenshotError::{FontsUnavailable, EmptyRect}`; `Harness::screenshot` / `screenshot_rect` resolve at the harness's own time (no tick, no pump), paint the viewport at its scale and device size into `VelloCpuImageRenderer` and composite over opaque white (each channel + (255 − alpha)); a rect is clipped, an empty one `EmptyRect`; `can_paint_text` false is `FontsUnavailable` with nothing painted; no print, panic or log; three more dependencies — `blitz-paint` (default features off), `anyrender`, `anyrender_vello_cpu` (no `multithreading`).
- Document core: `BaseDocument` gains two public read-only methods, `can_paint_text` (a `sans-serif` family resolves) and `layout_as_text_input` (a text laid out as a text input lays out, one line; its one caller the mask).
- Paint and shell: a password `input` paints its value as bullets in every document and renderer through blitz-paint, windowed included; equal-length values paint identical frames; the caret at its bullet in the input's caret colour; the field scrolls on the bullets. A click inside a filled field still places the caret by the real glyph widths — not measured, owned by a CARRY on "Stand requirement sweep".
- [Renderer backends]: the harness composites white after paint where the example fills before it (no `kurbo` / `peniko` among its dependencies).
- [Painting]: the mask is a direct engine change for every document; before it the engine painted a password's characters, confirmed on a frame; merge surface `render.rs` and `document.rs`.
- [Driver session]: was four dependencies; now five, `png` (no package entered `Cargo.lock`). A shot's pixels cross in band — base64 in the socket's and stdout's answer line, an MCP `image` block — the bounds unchanged (largest whole-screen answer 22,404 bytes, 2.1 %).
- Network ports and listeners: both carriers take a shot out; still one listener, no port. Names: escher-driver depends on `png`.
**Why:** the image in band is the founder's fork 1 and the engine mask his fork 2, each ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer; the card confirmed the same way, 2026-10-13); the caret's owner is this wrap's route placement.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-12-headless-screenshot — the scopes, the tests and the stack: `image`, three test targets, `png`
**Section:** §Existing Scopes → escher-driver · blitz-test-harness · blitz-tests · §Conventions → Tests · §Occupied Resources → Process-wide state and threads · §Stack and Technologies → Testing · Media
**Change:**
- escher-driver: was thirteen private modules, now fourteen — `image` (the PNG encoder and the hand-written padded base64); execute holds `Outcome::Shot` and the crop, json the shot's line, mcp the image block; was four dependencies, now `png` the fifth.
- blitz-test-harness: gains the private module `screenshot`, its three types re-exported.
- blitz-tests: a thirteenth driver-action file, `stand_act_screenshot`; `stand_screenshot_repeat` (one state's shot byte-identical in two child runs and the parent, 12 shots); the engine check `password_painted_masked`. Was `session_common` read by eighteen; now twenty (+ `stand_act_screenshot`, `stand_screenshot_repeat`), holding the two `screenshot` builders and the `shot` reader.
- Conventions → Tests: was the twelve `stand_act_*` and `stand_mcp_tools`; now the thirteen and `stand_screenshot_repeat` too.
- Process-wide state: was four blitz-tests binaries re-executing their test binary; now five — `stand_screenshot_repeat` runs it twice, each child printing its shots for the parent's assertions only.
- Testing: blitz-tests' dev-dependencies gain `png`. Media: `png` (0.18.1) also taken by escher-driver and blitz-tests, no package entering `Cargo.lock`; no `base64` crate.
**Why:** the chunk's code: one more private module, one harness module, three new test targets and a dependency already in the graph.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the upstream sync: taffy from the registry, the beta.3 pins, the pin and the front page
**Section:** §Stack and Technologies → Workspace package row · Layout row · Testing row · §Established Decisions → [Dependency pinning] · §Inherited Defaults → Framework · Publishability · §Project Intent → `Upstream sync:` line · Front page
**Change:**
- [Dependency pinning]: was "taffy is a git dependency pinned by `rev`"; now taffy is a registry version, "0.15" (resolved 0.15.0, its eight features unchanged), and no git dependency is left in the manifest. The coupled pins, the winit exact beta and the commented-out patch blocks stand.
- Layout row: was taffy from git at a pinned rev; now taffy "0.15" from crates.io with the same eight features. Inherited Defaults → Framework: engine pins `=0.3.0-beta.3`, taffy "0.15" from the registry.
- Workspace package row: version and the nine in-repo `=` pins move `0.3.0-beta.2` → `0.3.0-beta.3`.
- Testing row: blitz-tests' dev-dependencies gain `cursor-icon` (the workspace's), for upstream's `pointer_leave` check's hand-written `ShellProvider`.
- Publishability: blitz-vibey-script sets `publish = false` and joins the exceptions to the published `packages/` crates.
- `Upstream sync:` was last merged 7832c177 (merge 9462a7e4, 2026-10-10); now 434e06615d5f47a3df92c0f94e916df088fdcf4d (merge a6c24d24, 2026-10-13), the next sync's merge base.
- Front page: the page says eleven driver commands as tools, and the screenshot is no longer named as still to come.
**Why:** the merge took upstream's pin set whole — the plan's lean, with no third-party package moved and the audit green; the [Dependency pinning] retirement was named in the plan's reviewed Expected amendments and confirmed by the founder at this wrap's card (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's). Trap for the next sync: the CLAUDE.md coupled-pins warning no longer names a taffy git rev — taffy now moves with the registry like parley.
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the bridge drops a removed node; the default sheet from the config; the 28-name falsy predicate; `PointerLeave`
**Section:** §Established Decisions → [DOM semantics] · [Dioxus integration] · [HTML parsing] · §Standard Contracts → Document core · Events · Dioxus DOM bridge · §Existing Scopes → blitz-tests row
**Change:**
- [DOM semantics]: was "`node_id_mapping` entries are intentionally not cleared on removal so dioxus-core can reuse detached nodes"; now the Dioxus bridge drops a removed node with its descendants and clears their `element_id_mapping` / `node_id_mapping` entries (upstream #1185), so a re-render-removed node no longer resolves.
- Dioxus DOM bridge: was "a node a re-render removes is DETACHED (parent `None`) and keeps resolving until its ElementId is reassigned"; now it is dropped by the crate-private `remove_and_drop_node` / `replace_and_drop_node` and reads no id — `get_node` `None`, `element_id` `None`, no pair in `element_ids()`.
- Dioxus DOM bridge, the falsy clear: was one of the 27 names of the module-private `BOOLEAN_ATTRIBUTES` list; now one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set — `inert` added, a framework-written `inert: false` now cleared. `BOOLEAN_ATTRIBUTES` retired; `mutation_writer.rs` is byte-identical to upstream's at `434e0661`. The falsy values, both attribute paths and the `hidden` consequence are unchanged.
- blitz-tests row: `dioxus_falsy_boolean_attrs` still reads the 27 names of its own list; whether a check reads a falsy `inert` is not measured.
- [Dioxus integration]: was "`DEFAULT_CSS` always added to a `DioxusDocument`"; now `DioxusDocument::new` adds no sheet, and the default user-agent sheet arrives once through `DocumentConfig::default()` (upstream #1189). [HTML parsing]: was "`HtmlDocument` appends `DEFAULT_CSS` if absent"; now it uses the configured sheets as given.
- Document core: `DocumentConfig.ua_stylesheets` is a `Vec<UserAgentStylesheet>` (was `Option<Vec<String>>`), its hand-written `Default` the one `UserAgentStylesheet::default_sheet()` (a `LazyLock` parse of `DEFAULT_CSS`), and a config that sets it is used as given.
- Events: `UiEvent` gains `PointerLeave` (`DomEventKind::PointerLeave`, `EventDriver::handle_pointer_leave`), clearing hover, in a hosted sub-document too, and firing the leave events.
**Why:** upstream's behaviour, merged and measured green; the [DOM semantics] retirement was named in the plan's reviewed Expected amendments and confirmed by the founder with the reconcile (2026-10-13, «Подтвердить (Рекомендую)», relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's).
**Kept:** the `add_`/`remove_user_agent_stylesheet` methods and `synthetic_form_event` are not registered in the contract — the contract names neither kind, and escher uses neither.
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the founder's word respelled to the verbatim form; a doubled word
**Section:** §Established Decisions (the accessibility builder's states; escher-driver: the socket's crossing, `serde_json`, the `tool_call` span, the MCP crossing) · §Standard Contracts → Accessibility tree · Driver session (`serde_json`, the MCP bounds, the `tool_call` span) · §Occupied Resources (the socket widened to calls, the MCP crossing) · §Existing Scopes (escher-driver's json module)
**Change:**
- Eleven sites where the body's «relayed by the overseer» copies one of the four respelled entries now read "relayed verbatim by the overseer as his chosen option label …; the English wording is the overseer's", naming the entry's label:
  - «Сокет несёт команды» — the socket's second widening;
  - «Ратифицировать» — the adapter's three states;
  - «Ратифицировать как есть» — the MCP crossing and its bounds;
  - «serde_json, только чтение» and «Span с tool и ending».
- Five short spellings stay, each copying an entry that kept its short spelling or names no label: the bounds-and-hit ruling ×3, `type` replacing, and the orphaned-host answer.
- The `json` module: "the hand-written written forms" → "the hand-written forms".
**Why:** the body follows the entry it copies, so the provenance spelling matches the sidecar's verbatim form; no claim, ratification or PROVISIONAL mark moved. The lists were confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить списки (Рекомендую)»; the English wording is the overseer's), and he added playbook rule A on the same answer («Добавить правило (Рекомендую)»), so the class is routine from here.
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-stand-contrast-harness — the harness's contrast reader and its sixth module; the stand contrast check
**Section:** §Standard Contracts → Test harness · §Existing Scopes (blitz-test-harness row · blitz-tests row)
**Change:**
- Test harness: blitz-test-harness also exports `TextContrast` and `contrast_ratio` from its private module `contrast`, re-exported at the crate root. `contrast_ratio(a, b) -> f32` is the WCAG 2.x ratio `(L1 + 0.05) / (L2 + 0.05)`, lighter over darker, `L` from the `color` crate's relative luminance, alpha ignored — blitz-paint's in-crate helper restated, blitz-paint untouched. `TextContrast { text, background, ratio, font_size_px, font_weight }` carries `is_large_text()` (at least 24 px, or 14 pt at weight 700 or more) and `required_ratio()` (3.0 large, else 4.5). `Harness::text_contrast(&self, NodeId) -> Option<TextContrast>` reads and changes nothing (no mutate, input, pump or settle; every node by `get_node`): the text-bearing element (a text input itself, else the parent of the first non-whitespace text node), its computed colour, size and weight, over the background composited source-over to the first opaque ancestor, else the root-else-`body` canvas over opaque white; `None` for no text, a node without primary styles, or `opacity` < 1, a filter or a `background-image` layer on the walk, the image read through `resolved_style_value` with no `style` crate. The harness gains `color` (workspace, already in the graph; no package enters the lock) and still takes no `tracing`.
- Existing Scopes: blitz-test-harness lists the private `contrast`, six modules in all (was five); blitz-tests' `mod common;` readers 17 → 18, and the row names `stand_contrast`, 2 tests in both layout modes — 18 rows (pass 9 · fail 5 · exempt-inactive 3 · no-text 1, exempt exactly where `enabled` reads `Some(false)`) and the painted witness within 1 per channel; it records and gates nothing.
**Why:** the chunk built the SC 1.4.3 instrument as a harness reader; the harness contract and the scope rows carry every public item and module the crate holds.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/

## 2026-10-13-range-input-interaction — the range interaction model, the fifth direct engine change
**Section:** §Established Decisions → [Scrolling and input] · [Layout] · §Standard Contracts → Events · §Cross-cutting Patterns → Invalidation and state integrity
**Change:**
- [Scrolling and input]: a fifth direct engine change for every document, beside the three the decision counts and the password paint under [Painting].
  - An `input` whose `type` reads `range` (ASCII case-insensitive) holds its current value apart from its `value` attribute.
  - The rules follow the WHATWG range state in `node/range.rs`: `min` 0, `max` 100 and `step` 1 by default; a `max` below `min` reads `min`; a step ≤ 0 or unparsable reads 1; `any` sets no grid; the step base is `min`, else the `value` attribute, else 0; the default is the midpoint; every value is sanitized to `[min, max]` and the nearest grid value, the greater on a tie.
  - Focused and not `disabled` (presence): the arrows step it, `Home`/`End` take it to its minimum/maximum, other keys do nothing.
  - A main-button pointerdown sets the value by the hit's fraction of the content box; a click focuses and moves nothing.
  - One `input` per change, none for a no-change move.
  - A `value` write re-seeds the value and a `min`/`max`/`step` write or removal re-sanitizes it, a departure from the spec's dirty-value flag.
  - Form submission sends the current value.
  - Flagged upstreamable against 434e0661, no pull request open. Merge surface: seven upstream files plus `node/range.rs`.
  - Was: no key or click moved it, and a click did not focus it.
- [Layout]: a range input gets `RangeInputData` at construction. It is seeded only when none is held and cleared off `type="range"`, with no early return.
- Events: the range `input` dispatch goes through `BaseDocument::apply_range_move`, and the click focuses through `generate_focus_events`. No handler signature changes.
- Cross-cutting: the changed set is written at four kinds of site, was three. The fourth is `apply_range_move`, which marks a range input whose value changed and nothing for a no-change move.
**Why:** a boundary widening and an engine change for every document. It was ratified by the founder by question dialog in the overseer session, each answer relayed verbatim by the overseer as his chosen option label:
- his ruling of 2026-10-07 «Пусть будет а»;
- his three fork answers of 2026-10-13 «Отдельное текущее значение», «Стрелки + Home/End (Рекомендую)» and «Клик даёт фокус (Рекомендую)»;
- his confirm of this wrap's answer card «Подтвердить (Рекомендую)», which also covers the three consequences the operator confirmed at the review.

The re-seed is the first answer's stated cost, so a Dioxus re-render keeps control. The merge surface is owned by a CARRY on "Quality gates".
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/

## 2026-10-13-range-input-interaction — the range input's types, its snapshot value and its checks
**Section:** §Standard Contracts → Nodes and widgets · Dioxus DOM bridge · §Existing Scopes → blitz-dom: node · blitz-tests
**Change:**
- Nodes and widgets:
  - `SpecialElementData` lists `RangeInput`, a source break for an external exhaustive match (the enum is not `#[non_exhaustive]`; none in the workspace outside blitz-dom), beside `SpecialElementType::RangeInput` (`#[non_exhaustive]`, no reader).
  - The public `RangeInputData` (`value() -> f64`) is exported from `blitz_dom::node`, not re-exported from the crate root.
  - The public `ElementData` accessors are `range_input_data()`, `range_input_data_mut()` and `range_input_value() -> Option<String>`.
  - `RangeRules`, `RangeMove` and four range helpers stay crate-internal.
- Dioxus DOM bridge: the snapshot's `value` reads the editor text, else a range input's current value `range_input_value()`, else the `value` attribute. Was: the editor text, else the `value` attribute.
- §Existing Scopes, blitz-dom node: the module list adds range input data, the private module `range`.
- §Existing Scopes, blitz-tests:
  - `range_input_interaction` (6 tests, both layout modes) is registered.
  - `stand_act_range` now asserts that the driver moves the slider by click and key, its changes named by the diff on the slider (`value`) and its display (`name`). Was: "the range input's measured reading", a pinned no-op.
**Why:** the chunk added these public symbols, the module and the check, and moved the snapshot's range reading. The `stand_act_range` restatement rides the answer-card group the founder confirmed (2026-10-13, «Подтвердить (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label).
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/

## 2026-10-13-stand-keyboard-harness — keyboard activation and a click's focus, the sixth and seventh direct engine changes
**Section:** §Established Decisions → [Scrolling and input] · §Standard Contracts → Events · §Standard Contracts → Driver session
**Change:**
- [Scrolling and input]: three more direct engine changes for every document, the sixth to the eighth.
  - Keyboard activation. A pressed `Enter` on the focused node, after the text-input and range arms, dispatches a Click when the node has no `disabled` attribute (presence) and is a `button`, a `summary`, an `input` of type `submit`/`reset`/`button`, or an `a` with `href`. A released Space dispatches the same Click through the new `handle_key_up` for a `button`, a `summary`, or an `input` of type `submit`/`reset`/`button`/`checkbox`/`radio`. A link takes no Space; a text or range input takes neither key there. The Click is queued, so the app's handlers run before `handle_click`. Enter and Space now reach form submission, link navigation and `summary` toggling in every document, remote pages included.
  - A click focuses a button. `handle_click`'s submit arm became a button arm (a `button` of any type, an `input` of type `submit`/`reset`/`button`): it focuses through `generate_focus_events` / `set_focus_to`, submits a submit button's form owner and ends the walk matched. Was: a submit button submitted without breaking the walk and every other button fell through to the unmatched clear of focus. A `disabled` attribute still stops the walk first; a click on any other element still clears focus.
  - Focusability by `disabled` presence (the next entry).
  - Flagged upstreamable against 434e0661, no pull request open. Merge surface: `events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs`, `node/element.rs`.
- Events: `DomEventData::KeyUp` has its first default action, routed to the crate-internal `handle_key_up`; Enter's Click comes from `handle_key_or_input_event`; both through `dispatch_activation_click`. No signature changes.
- Driver session: after an accepted click on a control lying where a scrolled-out row extends, the clicked button reads `focused`. Was: no element reads `focused`.
**Why:** engine changes for every document, and keyboard activation is a boundary widening, never routine. The founder answered each fork himself by question dialog in the overseer session, each answer relayed verbatim by the overseer as his chosen option label: «Клавиши нажимают (Рекомендую)» (keyboard activation), «Клик даёт фокус (Рекомендую)» (a click focuses a button) and «По наличию (Рекомендую)» (`disabled` by presence), 2026-10-13. He confirmed on this wrap's answer card, 2026-10-14, «Подтвердить (Рекомендую)», that these proposals lie inside those answers. The merge surface is owned by a CARRY on "Quality gates".
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/

## 2026-10-13-stand-keyboard-harness — `disabled` by presence, no `inert`, and the keyboard check
**Section:** §Established Decisions → [DOM semantics] · [Unsupported features] · §Standard Contracts → Dioxus DOM bridge · §Existing Scopes → blitz-tests
**Change:**
- [DOM semantics]: `disabled` is read one way, by presence. `ElementData::flush_is_focussable` reads `has_attr(disabled)`, so a bare, a `"false"` and a `"true"` `disabled` on any element remove focusability, as the element state, the click target and the activation keys already key on it. The Tab walk, autofocus and `DioxusDocument::unkeyed_actionable` move with it; the recompute on a `disabled` write is unchanged. Was: focusability parsed a boolean through `attr_parsed`, and `disabled="false"` and a bare `disabled` stayed focusable. Known limit, pinned by `stand_keyboard`: a `<div disabled tabindex=0>` is not focusable, unlike in browsers; its narrowing to `button`, `input`, `select`, `textarea` is owned by a CARRY on "Stand requirement sweep".
- [Unsupported features]: `inert` is not implemented. A truthy `inert` takes no focusability away; whether an inert subtree takes a hit is not measured. A fix is an engine change for every document, owned by a CARRY on "Stand a11y assertions".
- Dioxus DOM bridge: the snapshot `enabled` note — a `disabled="false"` and a bare `disabled` read `Some(false)` and are not focusable either. Was: "while the engine's focusability reader still takes both as focusable". The `inert: false` clause: cleared, and the engine honours no `inert` either way. Was: "it no longer makes content inert".
- §Existing Scopes:
  - `dioxus_falsy_boolean_attrs` reads 28 names, `inert` the 28th. Was 27, the falsy `inert` not measured.
  - The shared module states `tab_order` (CRUD six stops) and `focused` as well, and holds seven helpers. It is read by 19 stand checks, `stand_keyboard` the nineteenth (was 18).
  - The new target `stand_keyboard` is listed: 11 tests in both layout modes.
  - The twelve driver-action files hold 41 tests (was 40), `stand_act_keys` adding Enter and Space through the driver.
**Why:** `disabled` by presence is an engine change for every document. The founder answered the fork himself, «По наличию (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label; the `<div>` limit is named on the operator's condition. The `inert` reading corrects a claim about code the chunk did not edit to its measured limit, the fix carried. The counts are this chunk's own.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/

## 2026-10-14-stand-a11y-assertions — the a11y leg at six files, the stand checks restated, two owners re-homed
**Section:** §Standard Contracts → CI contracts · §Standard Contracts → Mutation, query and CSSOM · §Established Decisions → [Unsupported features] · §Existing Scopes (blitz-tests)
**Change:**
- CI contracts: the `a11y` leg runs six integration tests — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` — with its `LegScriptTest` pin; was three. No new job, required status check or `ci.yml` change.
- Existing Scopes: `stand_contrast` 5 tests (was 2), record 14 · 0 · 3 · 1 after the accent moved one step darker (was 9 · 5 · 3 · 1), `TEXT_ROWS` 14 and `STATE_ROWS` 19, in the `a11y` leg (was "records and gates nothing"); `stand_keyboard` 14 tests (was 11) with the enabled-state walks, every control reached by Tab and Enter/Space on the later-enabled CRUD buttons; the shared module gains `ENABLED_TAB_ORDER` and `REGION_NAMES`; `accessibility_states` is "not by the `a11y` leg, which runs six" (was "the leg stays its three named files").
- [Unsupported features]: the `inert` fix's owner is a `.andromeda/residuals.md` line (target: next), `stand_keyboard::the_engine_honours_no_inert` its pin; was a CARRY on "Stand a11y assertions".
- Mutation, query and CSSOM: the AT witness's owner is a residuals line (target: next), its headless halves — the `&dyn Document` tree and the refresh on a Tab and an Enter — proven on the stand; was "a route entry".
**Why:** the leg widened on the founder's F4 «Расширить задание a11y (Рекомендую)»; the re-homes are his F3 «Оставить, перенести (Рекомендую)» and F1 «Пока нет, перенести (Рекомендую)», each by question dialog in the overseer session, relayed by the overseer as his chosen option label, placed as residuals lines by the operator's P5 word. No resource, crate or dependency added.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/

## 2026-10-14-stand-requirement-sweep — the hit's own-box, one-axis and transformed-and-scrolled readings measured; `covered`'s second limit
**Section:** §Standard Contracts → Scrolling, selection, tree · Driver session
**Change:**
- Scrolling, selection, tree: the hit's answer at an element's own box is measured, both layout modes and both builds — an `img` answers the `img`, a block inside an `iframe`'s `srcdoc` sub-document answers the `iframe` (the parent's walk does not enter it, pinned as it reads), a filled text `input` answers the `input`; was "not measured". A driver `click` on a target beside a box clipped on one axis is refused `covered`, the instance unchanged; neither reader moves. A box both transformed and scrolled is hit where its rows show; was "no check reads a box that is both transformed and scrolled". `BaseDocument::get_client_bounding_rect` reads a transformed box at its layout position, not where the transform moves it — a divergence from CSSOM View's `getBoundingClientRect`, measured and pinned as a fixture condition, not fixed, owned by a `.andromeda/residuals.md` line (target: next). "Each of these four readings is owned by a route entry" is retired: the `contain: paint` stop and `Node::absolute_position` are each owned by a residuals line, the other three measured.
- Driver session: `covered`'s quoted meaning gains its second limit — a box clipped on one axis stops a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows — in every `covered` refusal's line and in `describe()`'s causes on both surfaces; the crate docs state two readings off, was one; "the one reading still off" is now two, `stand_act_obstructed` holding both. The remedy, the nine causes and their names are unchanged.
**Why:** measured at this chunk. The deferrals are the founder's C4 «Отложить (Рекомендую)» and C14 «Отложить на следующую версию (Рекомендую)», each placed as a residuals line by the operator's word on the P5 card, and his C5 «Ничего, замерить и записать (Рекомендую)» leaves both readers as they read — each by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label; the client-rect line rests on the overseer's directive relayed by the operator at this wrap. `covered`'s text is the program's own fixed text inside the ratified carriers — not a widening.
**Ref:** .andromeda/runs/2026-10-14T22-00-51-wrap/

## 2026-10-14-stand-requirement-sweep — every stand task keyed; the password click caret measured; three new checks
**Section:** §Standard Contracts → Dioxus DOM bridge · Paint and shell · §Established Decisions (the `disabled` row) · §Existing Scopes → blitz-tests · §Conventions → Tests
**Change:**
- Dioxus DOM bridge: `unkeyed_actionable()` returns nothing on all seven stand tasks, Circle Drawer's dialog open included; was "Temperature Converter 2 · Circle Drawer 3 · Cells 676 on the three tasks not yet keyed". Nine author ids named: `temp-celsius`, `temp-fahrenheit`, `circle-undo`, `circle-redo`, `circle-canvas`, `circle-diameter`, `circle-close`, `cell-{A..Z}{0..25}`, `cell-input`. A scrolled box under a vertical writing mode is read, under both builds; was "no check reads … owned by a route entry".
- Paint and shell: the password click caret is measured — index 5 after eight narrow characters, 2 after eight wide — a known 0.1.0 limit whose fix is owned by a `.andromeda/residuals.md` line (target: next); was "not measured by any check", owned by a CARRY on "Stand requirement sweep".
- `disabled` row: the narrowing's owner is a residuals line; was a CARRY on that entry.
- Existing Scopes: `stand_requirement_sweep` 2 (`Session::run` alone on each lean task; `WITNESSES`, 16 rows), `stand_non_lean_tasks` 3, `password_caret_click` 1; `range_input_interaction` 6 → 8, `scrolled_box_client_rect` 6 → 7, `hit_clipped_at_scrolling_box` 9 → 13, `stand_act_obstructed` +1, so the twelve listed `stand_act_*` read 42, was 41; `common/mod.rs` readers 19 → 21 and `session_common/mod.rs` 20 → 21, the Conventions reader list naming the sweep.
**Why:** measured at this chunk. The owners rest on the founder's C11 «Замерить сейчас, исправить потом (Рекомендую)» and C15 «Оставить по наличию, отложить (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option labels, placed as residuals lines by the operator's word on the P5 card; the keys on the operator's "Key all three; C8 deferred" at P4.
**Ref:** .andromeda/runs/2026-10-14T22-00-51-wrap/

## 2026-10-14-quality-gates — the quality gates: legs, the new job, the install repair, the code-quality row
**Section:** §Stack and Technologies → CI/CD, Code quality · §Conventions → Formatting and lints · §Standard Contracts → CI contracts · §Occupied Resources → CI infrastructure · §Infrastructure Patterns → CI/CD · §Inherited Defaults → Code quality
**Change:**
- The leg list gains `stand-repeat` (not a fast leg): three rounds of every `stand_*.rs` target, then seven_guis' seven process checks; the first red command ends the leg with its status, no retry; three green rounds print `stand-repeat: 3 of 3 rounds green`.
- `audit` is `cargo deny --locked --workspace --all-features check advisories licenses` (was `cargo deny check advisories`).
- `clippy` runs a second line, only when the workspace line is green: upstream's `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`.
- `coverage` ends with `coverage_floor.py` over escher-driver 97 % · dioxus-native-dom 83 % · blitz-test-harness 90 % (read 98.34 · 84.25 · 91.69 %). Was "with no threshold".
- New contract `coverage_floor.py LCOV CRATE_DIR=PERCENT …`: exit 0 / 1 / 2, prints crate directories and counts only. The ci-scripts leg discovers seven test files (was six).
- `apt-install.sh`: every attempt after the first runs a bounded `dpkg --configure -a` first, in phase `repair`; the first runs no `dpkg`. Ten calls (was nine), `InstallScriptTest` eight (was five), `dpkg` among the runner tools. Was "dpkg interrupted, which the script does not handle".
- CI/CD: the job `Stand checks repeated`; thirteen linux leg jobs (was twelve); nine `libfontconfig1-dev` jobs (was eight); `a11y` and `stand-repeat` restore `workspace-test` and never save.
**Why:** this chunk's legs and job, on the founder's «3 крейта, на 1 пункт ниже (Рекомендую)», «Повторное задание, ноль сбоев (Рекомендую)», «Проверка лицензий (Рекомендую)», «Clippy blitz-dom, как в основном репозитории (Рекомендую)» and «Аудит всего workspace (Рекомендую)», relayed verbatim by the overseer as his chosen option labels; the repair on the operator's answer «Add a bounded repair» at P4.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/

## 2026-10-14-quality-gates — the test-only listener, the manifests, the merge surface read, the pinned driver claims
**Section:** §Occupied Resources → Network ports and listeners, Filesystem, Environment variables, Names · §Project Intent → Product type, Upstream sync · §Established Decisions → [Scrolling and input] · §Standard Contracts → Driver session
**Change:**
- Network ports and listeners: the session socket is the one served listener site. Beside it, one test-only bind: escher-driver's unix-only `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout` binds a `UnixListener` in `eab-{pid}` under the system temp directory, never answers, removes it — no crossing. Filesystem gains that directory; Environment variables gain `TMPDIR`, through `std::env::temp_dir()`, test code only.
- Names and Product type: the workspace package's and debug_timer's homepage and repository read `https://github.com/Turbolet85/escher` (was `https://github.com/dioxuslabs/blitz`, kept as the as-adopted reading).
- [Scrolling and input]: the range model's and keyboard activation's merge surfaces read at upstream `e72650a6`, untouched since `434e0661`. Was "owned by a CARRY on the route entry Quality gates".
- Upstream sync: the merge base stays `434e0661`; upstream `main` read at `e72650a6`, three commits past it (#1198, #1212, #1211), not merged.
- Driver session: the 30 s per-read answer bound and "a write to a closed pipe is dropped, not a panic" each pinned by a check; `busy` read on a real not-settled `command` line. Was "covered by its unit test only".
**Why:** measured at this chunk. The test-only bind registered on the operator's answer at this wrap's escalation, «Register test-only»: never served, reached by no caller, so no crossing and not the founder's — registered as `PAINT_TREE_BENCH_HTML` and `CARGO_TARGET_TMPDIR` are.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/
