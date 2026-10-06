# Materialization Plan — escher (setup-project checkpoint)

_Phase 0 output. Phases 1–6 read ONLY this file. Re-run (the upgrade) on an adopted project._

- **Project:** escher · version workspace `escher-0.1.0/` · branch `build/escher-0.1.0` · HEAD `9944dec5`
- **Development Style:** agent-driven (arch §Cross-cutting Patterns · §Inherited Defaults)
- **Primary language / stack fragment:** Rust (Cargo workspace, edition 2024, rust-version 1.91.0) · `rust`
- **Host:** Linux (POSIX) — `host-win32.md` skipped (noted no-op; U04 `n/a`)
- **Upstreams read:** input · architecture (263 lines, body — `registry.py contracts` exit 3 `NOT MIGRATED`, §Infrastructure Patterns read in the body) · security-plan · design-system · layout-templates · test-plan · obs-plan · a11y-plan · master-route (15 records, 15 complete). U35: test-plan §3, obs-plan §3 and a11y-plan §3 each carry ONE keyed contract, `Bootstrap phases (derive for route / setup-project)`, read from `.andromeda/registries/contracts/{plan}/bootstrap-phases-derive-for-route-setup-project.md`; security-plan, design-system and layout-templates carry no keyed-contract section.
- **Route cursor (Setup 5b):** `records 15 · complete 15 · pending 0 · gated 0` · `half-promote 0 of 15` · next `working-route.md:43 · Snapshot state fidelity`.

## Upgrade

5b HEAD: `9944dec540fb5b7549fdc1658d192539451e3d21`

5b path set (all expected-transient bookkeeping):
```
 M .andromeda/friction-log.ndjson
 M .andromeda/runs/2026-10-06T20-55-58-wrap/evolve-2026-10-06-id-stability-across-code-edits.json
 M .claude/session-handoff.md
```

`upgrade.py detect --root .` — verbatim (the tool clips long fact columns with `…`):
```
upgrade v1.5 · eeed2076
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write current · bash leading-cd
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · n/a · setup · .claude/rules/host-win32.md · absent — a POSIX host skips it
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh carries `ensure_fresh_artifa…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 22 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: test K, obs K, a11y K · n/a: infra K, test L, obs L, a11y L, se…
U36 · ok · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summary…
upgrade: for setup 1 (U02) · awaiting a door 0 · noted 0 · INDETERMINATE 0 · 16 detectors of 36 registry entries
```

Acting rows: **U02** (setup, `behind` — the Bash guard is the `leading-cd` prior form) → Phase 5 step 2. Nothing for P7.5 (U11 · U12 `ok`). No hand row awaits a door; no noted drift (U09 · U10 · U36 `ok`); no `INDETERMINATE`.
U03's clipped fact, read in full from health check 11 (read-only, no run dir): `behind py 0 · sql 0 · cookbook 0` — the triple is current.

## Adoption

- `Adopted at 0f60502ea724ef703b34220b3847bb3483f70b5f · push target refs/remotes/origin/build/escher-0.1.0 · secret scan not measured`
- Master sections still reading NOT YET MEASURED / NO RECORDED INTENT / observed absent (nothing renders from a template default; every count below is a ceiling):
  - security-plan: application threat model, HSTS / TLS versions / pinning, key management / backups / retention, CVE SLA, SBOM / image scanning / licence compliance, dev secret storage / secret scanning / rotation, security-event logging — NOT YET MEASURED; Compliance Controls, Security Anti-Patterns, Decisions Log — NO RECORDED INTENT; application auth, CORS / CSP, request timeout and size cap, automated dependency updates — observed absent.
  - design-system: Brand Identity, Depth chosen approach, Radius personality, Motion expression level, Anti-Patterns, Self-Validation, Decisions Log — NO RECORDED INTENT; project-wide colour tokens, type scale, spacing scale, elevation scale, radius scale, cli / web-spa navigation and platform notes — NOT YET MEASURED.
  - layout-templates: every surface's tooling context / expression level / signature — NOT YET MEASURED; Decisions Log — NO RECORDED INTENT.
  - test-plan: §1 tier justification, §3 the test-data bootstrap mechanism of the 5-command contract, §6 windowed E2E, §10 quality gates / thresholds — NOT YET MEASURED; §11 Anti-Patterns, §12 Decisions Log — NO RECORDED INTENT.
  - obs-plan: §1 telemetry surface, §2 naming conventions, §3 product mode / snapshot / trace context / heartbeat / JSON schema / log-file location, §6 log schema / sink / rotation, §9 snapshot upload — NOT YET MEASURED; §10 SLO, §11 Anti-Patterns, §12 Decisions Log — NO RECORDED INTENT; OTel SDK, spans, metric emitters, error-reporting client — observed absent.
  - a11y-plan: critical paths, §2 stated strategy, §3 violation schema / WCAG mapping / screen-reader pattern, §5 skip links / focus traps, §6 token contrast pairs, §7 live regions, §8 cognitive — NOT YET MEASURED; §10 SLO, §11 Anti-Patterns, §12 Decisions Log — NO RECORDED INTENT; contrast checks, keyboard-event / Tab-order navigation tests — observed absent.
- **test-plan §3 is measured for the 5-command contract** (built by the completed Foundation chunk `2026-10-06-stand-test-contract`; `scripts/agent-run.{sh,ps1}` exist) — the Agent harness row is `preserved`, not `none`.

## Tier 1 — CLAUDE.md (re-derived; the cascade has kept it current — 4 point changes)

Every `GENERATED:setup:*` block was re-derived against the masters. Blocks that come out byte-identical are left untouched; the changes are anchored Edits. Backup: `.claude/backup/CLAUDE.md.pre-setup-2026-10-06T21-21-10`.

- **Overview** — unchanged. Arch §Project Intent paragraph; stack one-liner; key dirs **6** (`packages/` · `tests/blitz-tests/` · `examples/` · `apps/` · `wpt/runner/` · `scripts/`) — the template says "up to 5"; the sixth (`scripts/`, the agent-run contract's home, cascade-added) is kept and named on the card.
- **Modules (16)** — unchanged: blitz-dom · blitz-traits · blitz-html · blitz-net · blitz-paint · blitz-shell · blitz · blitz-vibey-script · stylo_taffy · accesskit_xplat · debug_timer · dioxus-native-dom · dioxus-native · blitz-test-harness · escher-telemetry · seven_guis (arch §Infrastructure Patterns directory tree + §Existing Scopes; one past the template's "5-15 typical" — every `packages/` crate plus the stand).
- **Universal warnings (10)** — unchanged; each re-checked against its master (no plan carries an anti-pattern section — all drawn from measured facts; severity security > a11y > obs > tests > arch):
  1. [security] secrets only in GitHub Actions secrets/vars + the operator's `claude` login on the dev host; source reads none (security-plan §Secret Management)
  2. [security] no port or socket bound; a new listener / port / env var / workspace crate is an arch §Occupied Resources registration (arch §Occupied Resources)
  3. [a11y] target SC 2.1.1 · 1.4.3 · 2.4.3; `accessibility` a default feature (a11y-plan §1)
  4. [obs] engine telemetry is `tracing` behind each crate's `tracing` feature with a no-op fallback; escher binaries install `escher_telemetry::init`, stderr only (arch §Conventions Feature gating · §Cross-cutting Logging; obs-plan §2 · §3)
  5. [obs] escher's sink scrubs by allowlist; upstream `fmt::init()` and the WPT `env_logger` stay unscrubbed — no new user-content log fields (obs-plan §8)
  6. [tests] incremental and non-incremental layout identical; `for incremental in [false, true]` (arch §Design Philosophy · §Conventions Tests)
  7. [arch] DOM mutations through `DocumentMutator` (arch §Conventions DOM API patterns)
  8. [arch] `NodeId` versioned slot id; stale index panics (arch §Standard Contracts · §Established Decisions Node identity)
  9. [arch] coupled dependency pins (arch §Established Decisions Dependency pinning)
  10. [arch] not done until `ci-leg.sh fast` and `ci-leg.sh doc` pass (arch §Stack Code quality)
- **Rejected (audit):** every actionable element reads an author key, checked by `unkeyed_actionable` (path-scoped to Dioxus UI code → `a11y.md`); the Dioxus crates are taken `default-features = false`, so `accessibility` must be named (→ `services/dioxus-native-dom.md`); `disabled` is keyed two ways — presence for state, parsed bool for focusability (→ `a11y.md` / `gotchas.md`); `StyleThreading::Parallel` panics with two documents resolving concurrently (blitz-dom-scoped → `gotchas.md`); `blitz-net` reads `file:` URLs with no path restriction and has no size cap or timeout (already in the always-loaded `security.md`); recursion caps iframe 10 / `@import` 16 (→ `security.md`); the headless stand installs no telemetry subscriber (→ `observability.md`); `unsafe` carries `// SAFETY:` (→ `conventions.md`); deps declared once in `[workspace.dependencies]` (→ `conventions.md`); a stable id never holds a `NodeId` / `ElementId` / `ScopeId` / pointer (dioxus-native-dom-scoped → its service note); a new id / snapshot / check crossing needs a security-plan amendment (covered by warning 2's registration rule + `security.md` §Surfaces).
- **Pointer table (24 rows)** — 21 unchanged; **3 rows gain the U35 note the template renders for a migrated plan** (the keyed contract of §3 lives in a registry file, not the body):
  - `Test harness · agent-run contract` → `.andromeda/test-plan.md` §3 (keyed contracts: `.andromeda/registries/test-plan-contracts.toml`) · `scripts/agent-run.sh` (contract tests `.github/scripts/test_agent_run.py`)
  - `Observability pipeline` → `.andromeda/obs-plan.md` §3 (keyed contracts: `.andromeda/registries/obs-plan-contracts.toml`)
  - `WCAG criteria · a11y harness` → `.andromeda/a11y-plan.md` §1 / §3 (keyed contracts: `.andromeda/registries/a11y-plan-contracts.toml`)
  - The architecture row keeps `§Infrastructure Patterns / §Occupied Resources` with no registry note — architecture is not migrated. No row bakes a version-workspace path.
- **Workflow (5 commands)** — unchanged: `cargo build --workspace` · `bash .github/scripts/ci-leg.sh fast` · `cargo test -p blitz-tests --test {name}` + `bash scripts/agent-run.sh boot` / `run stand|all|{name}` · `bash .github/scripts/ci-leg.sh {leg}` · `just seven_guis`.
- **Architecture** — unchanged (2 paragraphs, arch §Design Philosophy + §Standard Contracts Dioxus DOM bridge as built).
- **@imports** — `@.claude/session-handoff.md` only (U01 `ok`).
- **Deeper topics** — services line gains `escher-telemetry` (Tier 3 below); rules line unchanged.
- **USER:session-learnings** — preserved verbatim (1 bullet).
- Expected size: 132 lines (no line added or removed).

## Tier 2 — .claude/rules/ (all existing — dispositions)

| File | Disposition |
|---|---|
| `security.md` (always loaded) | preserve |
| `testing.md` | preserve |
| `observability.md` | preserve |
| `a11y.md` | preserve |
| `verification-harness.md` | preserve |

Not planned (unchanged from the first run): `migrations` (only browser-persistence's rusqlite_migration — upstream app), `api` (no served API), `events` (no event bus), `frontend` (no TS / web frontend; a WASM canvas), `host-win32` (POSIX host). No leaf drift note (U04 `n/a`).

## Tier 3 — .claude/docs/ (dispositions)

- Core 5 — `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md`: **preserve**.
- Summaries 5 — `security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md`: **preserve** (U36 `ok`).
- Services, existing 7 — `blitz-dom` · `blitz-traits` · `blitz-paint` · `blitz-shell` · `blitz-test-harness` · `dioxus-native-dom` · `seven_guis`: **preserve**.
- Services, **ABSENT → write: `services/escher-telemetry.md`**. The first run planned a note for "the crates the 0.1.0 route touches"; escher-telemetry is the one module added since (by `2026-10-06-telemetry-bootstrap`) and the route returns to it (the OTel export CARRY on "Driver command spans"). Rendered from measured facts only:
  - Responsibility: escher's telemetry bootstrap — a process-global stderr `tracing` subscriber with service identity, the allowlist scrub formatter, the `log` bridge and a chaining panic hook. Not its job: OTel export (none in 0.1.0 — founder ruling 2026-10-06, a residual in `.andromeda/residuals.md`), a file sink, a JSON schema, or the agent-run / cold-agent harness logs (harness metadata, not this sink).
  - Publishes: `init(ServiceIdentity) -> Result<InitOutcome, InitError>` · `init_with_writer(ServiceIdentity, W: MakeWriter)` · `ServiceIdentity { name, version }` · `service_identity!()` (the CALLER's `CARGO_PKG_NAME` / `CARGO_PKG_VERSION`) · `InitOutcome::{Installed, AlreadyInstalled}` · `InitError::ForeignSubscriber` (`Display` + `Error`, never a panic). Line shape `{RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…`, one per event, no ANSI, newlines escaped, string values Debug-quoted, a bridged `log` record under its `log.target`.
  - Consumes: `tracing` (ungated), `tracing-subscriber` (env-filter · fmt · registry · std · tracing-log), `tracing-log`; reads `RUST_LOG` through `EnvFilter` (default `warn` when unset or unparsable).
  - Consumers: `seven_guis_native` (`main`, before `dioxus_native::launch`, `Err` → `eprintln!` and continue); blitz-tests (dev-dependency, the four `telemetry_*` files).
  - Conventions: no `[features]` — its startup and panic events are ungated, unlike the engine crates; `publish = false`; stderr only, never stdout. Scrub sets in `format.rs`: `ENGINE_TARGET_PREFIXES` (`blitz` · `dioxus_native` · `stylo_taffy` · `accesskit_xplat` · `debug_timer` · `js_console`) print only `SAFE_FIELDS` (`node_id` · `status` · `waiting_nodes` · `property` · `log.module_path` · `log.file` · `log.line`), every other field — the message included — as `{name}=[redacted]`; `CONTENT_FIELDS` (`url` · `href` · `src` · `html` · `text` · `value` · `attrs` · `path` · `request` · `error` · `panic.payload`) are redacted at any target. No engine call site is edited.
  - Gotchas: process-global — a second `init` changes nothing (`AlreadyInstalled`); another global subscriber or `log` logger → `ForeignSubscriber`; the first install is recorded in a static `OnceLock<ServiceIdentity>` behind a static `Mutex<()>`; it spawns no thread. The chained std panic hook still prints the raw panic message to stderr; the allowlisted `log.file` carries a host path for bridged third-party records at `RUST_LOG=info`. The headless stand (`seven_guis::stand`) installs no subscriber. The scrub's reach is this sink only — the upstream apps' `fmt::init()` (stdout) and the WPT runner's `env_logger` stay unscrubbed.
  - Entry points: `packages/escher-telemetry/src/lib.rs` (`init`, identity, filter) · `src/format.rs` (formatter + scrub sets) · `src/panic.rs` (hook) · `Cargo.toml`.
  - Tests: `cargo test -p escher-telemetry` (5 inline unit tests — three scrub branches, a bridged `log` record, the identity macro) · `cargo test -p blitz-tests --test telemetry_stdout_silent` · `telemetry_scrub` · `telemetry_panic_hook` · `telemetry_init_idempotent` (`telemetry_stdout_silent` re-executes its own binary once, ignored child `child_emits`) · smoke `RUST_LOG=info just seven_guis`.
  - References: `.claude/rules/observability.md` · `.claude/docs/obs-summary.md` · obs-plan §3 / §6 / §8 · arch §Standard Contracts Telemetry bootstrap.
- `session-learnings.md` — present; untouched (wrap territory).

## Agent harness

- agent-driven; `scripts/agent-run.sh` (228 lines) and `scripts/agent-run.ps1` (4 lines, pass-through) present → **preserved**, no write. test-plan §3 is derived FROM the script (it cites the script's own lines), so the plans' 5-command params equal the script's — no param drift, no backup. U10 `ok` (the `ensure_fresh_artifacts` hook is present). `verification-harness.md` present → preserve.

## Hooks (.claude/settings.json)

- Formatter `rustfmt "$f"` on `*.rs` · linter none at write time (clippy is a gate) · type-checker none (matrix Rust rows). `rustfmt.toml` present with `edition = "2024"` (U05 `ok`) — no formatter config written, no reflow step.
- Re-run rule, entry by entry against the matrix: PreToolUse write guard — equal to the matrix form, re-rendered byte-equal; **PreToolUse Bash guard — the `leading-cd` prior form → REPLACED by the matrix's current form** (a `cd` at every top-level position; a target resolving to the session cwd or the project root passes); PostToolUse formatter — equal, re-rendered byte-equal; `env` (`PYTHONUTF8` · `PYTHONIOENCODING`) kept. No user-managed entry exists in the file. Backup first: `.claude/backup/settings.json.pre-setup-2026-10-06T21-21-10`.
- Host: `jq` on PATH (`/usr/bin/jq`) · `rustfmt` on PATH.

## Code-graph pipeline

- Planes: rust (root `Cargo.toml`); no ts plane. `scripts/{code-graph.py, code-graph-views.sql, scip_pb2.py, requirements.txt, code-graph-cookbook.md}` all present → preserve; currency `py 0 · sql 0 · cookbook 0` → no drift, no backup, no proposal.
- Graph state: `.refresh-done` = `rust ok 26s 5807/30688`; `tree.db.commit` = `9944dec5…` (= HEAD, stamped by the last wrap). Scripts were not seeded by this run, so no build is fired.

## Code reviewer

- `.claude/agents/code-reviewer.md` (Rust, 72 lines) present → **preserve**.

## Gitignore / gitattributes

- `.gitignore`: U07 `ok` — every base ignore decided by the root file at the root and at depth 2 → nothing appended.
- `.gitattributes`: U06 `ok` — carries `* text=auto eol=lf` → present, unchanged. Index: 1643 `i/lf` · 13 `i/none` · 11 `i/-text`, 0 `i/crlf` / `i/mixed`.

## Seeds (Phase 6)

- `.andromeda/state.yaml` (schema 3, session_count 18) · `.claude/session-handoff.md` · `.andromeda/drift-base.md` · `.andromeda/playbook.md` · `.claude/docs/session-learnings.md` — all present → nothing seeded.

## Planned writes (the manifest's set)

1. `.claude/backup/CLAUDE.md.pre-setup-2026-10-06T21-21-10` (backup, gitignored)
2. `CLAUDE.md` — 3 pointer rows + the services line
3. `.claude/docs/services/escher-telemetry.md` — new
4. `.claude/backup/settings.json.pre-setup-2026-10-06T21-21-10` (backup, gitignored)
5. `.claude/settings.json` — the Bash guard entry (U02)

## Consistency check

- Every rule file in Tier 2 is listed in Deeper Topics; the services line names exactly the 8 files that will exist; the agent-harness row and `verification-harness.md` agree (script present, rule present); the pointer table cites no baked version path; @imports = handoff only, and the file exists; the three registry files the new pointer notes name exist under `.andromeda/registries/`; every setup-class `behind` row (U02) has a planned write and nothing else is `behind`.
