# Materialization Plan — escher (setup-project checkpoint)

_Phase 0 output. Phases 1–6 read ONLY this file. Re-run (the upgrade) on an adopted project._

- **Project:** escher · version workspace `escher-0.1.0/` · branch `build/escher-0.1.0` · HEAD `d606e718`
- **Development Style:** agent-driven (arch §Cross-cutting Patterns · §Inherited Defaults)
- **Primary language / stack fragment:** Rust (Cargo workspace, edition 2024, rust-version 1.91.0) · `rust`
- **Host:** `upgrade.py host` prints `host: linux` → the host leaf is `.claude/rules/host-linux.md`
- **Upstreams read:** input (48 lines) · architecture (267 lines, whole, body — `registry.py contracts` exit 3 `NOT MIGRATED`, §Infrastructure Patterns read in the body) · security-plan (405) · design-system (437) · layout-templates (99) · test-plan (348) · obs-plan (348) · a11y-plan (346) · master-route (25 records, 25 complete). U35 key files read: test-plan §3 `bootstrap-phases-derive-for-route-setup-project.md` and `session-lifecycle.md`; obs-plan §3 and a11y-plan §3 one each, `bootstrap-phases-derive-for-route-setup-project.md`; security-plan carries no keyed-contract section (`n/a`).
- **Route cursor (Setup 5b):** `records 25 · complete 25 · pending 0 · gated 0` · `half-promote 0 of 25` · next `working-route.md:64 · Refusal detection`.

## Upgrade

5b HEAD: `d606e71811fee708e3a44142e62f14ffdf223d32`

5b path set (all expected-transient bookkeeping):
```
 M .andromeda/friction-log.ndjson
 M .andromeda/runs/2026-10-07T14-22-35-wrap/evolve-2026-10-07-act-by-id.json
 M .claude/session-handoff.md
```

`upgrade.py detect --root .` — verbatim (the tool clips long fact columns with `…`):
```
upgrade v1.6 · 814083ff
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · behind · setup · .claude/rules/host-{os}.md · absent — setup P2 renders host-linux.md
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh carries `ensure_fresh_artifa…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 15 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: test K, obs K, a11y K · n/a: infra K, test L, obs L, a11y L, se…
U36 · ok · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summary…
upgrade: for setup 1 (U04) · awaiting a door 0 · noted 0 · INDETERMINATE 0 · 16 detectors of 40 registry entries
```

Acting rows: **U04** (setup, `behind` — the host leaf is ABSENT, not rendered for another host) → Phase 2 step 2, the tool's render; no re-seed, so no `host-reseed.json`, no step 8a sort and nothing of U04 at P7.5. Nothing else for P7.5 (U11 · U12 `ok`). No hand row awaits a door; no noted drift (U09 · U10 · U36 `ok`); no `INDETERMINATE`.
U03's clipped fact, read in full from health check 11 at this session's start: `behind py 0 · sql 0 · cookbook 0` — the triple is current.
U04 dry run (`upgrade.py apply --root . --id U04 --dry-run`): `would render .claude/rules/host-linux.md — the linux sections of the host template, 2857 B, an empty ## Session Additions`.

## Adoption

- `Adopted at 0f60502ea724ef703b34220b3847bb3483f70b5f · push target refs/remotes/origin/build/escher-0.1.0 · secret scan not measured`
- arch §Project Intent reads `Origin: adopted at 0f60502ea724ef703b34220b3847bb3483f70b5f`.
- Master sections still reading NOT YET MEASURED / NO RECORDED INTENT / observed absent (nothing renders from a template default; every count below is a ceiling):
  - security-plan: application threat model; HSTS / TLS versions / certificate pinning; key management / backups / retention / anonymization; a critical-CVE SLA; SBOM / base image scanning / licence compliance; development secret storage beyond the `claude` login, secret scanning, rotation, access auditing; security-event logging, retention, access controls, tamper evidence — NOT YET MEASURED. Compliance Controls, Security Anti-Patterns, Decisions Log — NO RECORDED INTENT. Application auth, CORS / CSP, a request timeout, a response size cap, automated dependency updates, error-reporting integration, monitoring — observed absent.
  - design-system: Brand Identity, Depth chosen approach, Radius personality, Motion expression level, Anti-Patterns, Self-Validation, Decisions Log — NO RECORDED INTENT. A project-wide colour token set, type scale, spacing scale, elevation scale, radius scale; cli and web-spa navigation and platform notes — NOT YET MEASURED.
  - layout-templates: every surface's tooling context / expression level / signature placement — NOT YET MEASURED. Decisions Log — NO RECORDED INTENT.
  - test-plan: §1 tier justification / critical paths / coverage triggers; §3 the test-data bootstrap mechanism of the 5-command contract; §6 windowed or browser-driven E2E; §10 a quality gate, coverage threshold, flakiness or performance budget — NOT YET MEASURED. §11 Anti-Patterns, §12 Decisions Log — NO RECORDED INTENT.
  - obs-plan: §1 telemetry-surface table / must-trace paths / triggers; §2 naming conventions; §3 product mode / snapshot integration / trace context / heartbeat / a JSON schema or log-file location for escher's sink; §6 log JSON schema / file sink / rotation; §9 snapshot upload / CI resource attributes — NOT YET MEASURED. §10 SLO, §11 Anti-Patterns, §12 Decisions Log — NO RECORDED INTENT. An OTel SDK or backend, spans, metric emitters, an error-reporting client — observed absent.
  - a11y-plan: §1 critical paths / triggers; §2 a stated strategy; §3 a violation schema / WCAG mapping of the existing tests / a screen-reader test pattern; §5 skip links / modal focus traps / route-change focus restoration; §6 token contrast pairs and focus-ring, target-size and readability tokens; §7 live regions / heading hierarchy / per-surface screen-reader specs; §8 timeouts / plain language / redundant entry / accessible authentication — NOT YET MEASURED. §10 SLO, §11 Anti-Patterns, §12 Decisions Log — NO RECORDED INTENT. Contrast checks — observed absent.
- **test-plan §3 is measured for the 5-command contract** (built by the completed Foundation chunk `2026-10-06-stand-test-contract`; `scripts/agent-run.{sh,ps1}` exist) — the Agent harness row is `preserved`, not `none`.

## Tier 1 — CLAUDE.md (re-derived; the cascade has kept it current — 1 point change)

Every `GENERATED:setup:*` block was re-derived against the masters as read in this Phase 0 (the last wrap's cascade re-derived them at 2026-10-07T15:02Z and no master has changed since). Blocks that come out byte-identical are left untouched; the one change is an anchored Edit. Backup: `.claude/backup/CLAUDE.md.pre-setup-2026-10-07T15-11-18` (md5 `98dfaad87f183f520f9909396c1646ab`).

- **Overview** — unchanged. Arch §Project Intent paragraph; stack one-liner; key dirs **6** (`packages/` · `tests/blitz-tests/` · `examples/` · `apps/` · `wpt/runner/` · `scripts/`) — the template says "up to 5"; the sixth (`scripts/`, cascade-added and kept by the 2026-10-06 re-run) is kept and named on the card.
- **Modules (17)** — unchanged: blitz-dom · blitz-traits · blitz-html · blitz-net · blitz-paint · blitz-shell · blitz · blitz-vibey-script · stylo_taffy · accesskit_xplat · debug_timer · dioxus-native-dom · dioxus-native · blitz-test-harness · escher-telemetry · escher-driver · seven_guis (arch §Infrastructure Patterns directory tree + §Existing Scopes — every `packages/` crate plus the stand; two past the template's "5-15 typical"). Each entry's text re-checked against arch §Standard Contracts (Document core · Test harness · Headless stand · Telemetry bootstrap · Driver session · Dioxus DOM bridge) — no clause contradicted.
- **Universal warnings (10)** — unchanged; each re-checked against its master (no plan carries an anti-pattern section — all drawn from measured facts; severity security > a11y > obs > tests > arch):
  1. [security] secrets only in GitHub Actions secrets/vars + the operator's `claude` login on the dev host; source reads none (security-plan §Secret Management)
  2. [security] no network port; the one listener is the driver session's owner-only Unix-domain socket, lifecycle messages only; a new listener / port / env var / workspace crate is an arch §Occupied Resources registration (arch §Occupied Resources · security-plan §API Security)
  3. [a11y] target SC 2.1.1 · 1.4.3 · 2.4.3; `accessibility` a default feature (a11y-plan §1)
  4. [obs] engine telemetry is `tracing` behind each crate's `tracing` feature with a no-op fallback; escher binaries install `escher_telemetry::init`, stderr only (arch §Conventions Feature gating · §Cross-cutting Logging; obs-plan §2 · §3)
  5. [obs] escher's sink admits two target families and drops the rest; upstream `fmt::init()` and the WPT `env_logger` stay unscrubbed — no new user-content log fields (obs-plan §8)
  6. [tests] incremental and non-incremental layout identical; `for incremental in [false, true]` (arch §Design Philosophy · §Conventions Tests)
  7. [arch] DOM mutations through `DocumentMutator` (arch §Conventions DOM API patterns)
  8. [arch] `NodeId` versioned slot id; stale index panics (arch §Standard Contracts · §Established Decisions Node identity)
  9. [arch] coupled dependency pins (arch §Established Decisions Dependency pinning)
  10. [arch] not done until `ci-leg.sh fast` and `ci-leg.sh doc` pass (arch §Stack Code quality)
- **Rejected (audit):** every actionable element reads an author key, checked by `unkeyed_actionable` (path-scoped to Dioxus UI code → `a11y.md`); the Dioxus crates are taken `default-features = false`, so `accessibility` must be named (→ `services/dioxus-native-dom.md`); `disabled` is keyed two ways — presence for state, parsed bool for focusability (→ `a11y.md` / `gotchas.md`); `StyleThreading::Parallel` panics with two documents resolving concurrently (blitz-dom-scoped → `gotchas.md`); `blitz-net` reads `file:` URLs with no path restriction and has no size cap or timeout (already in the always-loaded `security.md`); recursion caps iframe 10 / `@import` 16 (→ `security.md`); a `Refusal` holds nothing a call supplied and an `Outcome` is never logged or fielded (escher-driver-scoped, already in `security.md` §Surfaces); no driver command waits on a load (→ `services/escher-driver.md`); the headless stand and the session library install no telemetry subscriber (→ `observability.md`); `unsafe` carries `// SAFETY:` (→ `conventions.md`); deps declared once in `[workspace.dependencies]` (→ `conventions.md`); a stable id never holds a `NodeId` / `ElementId` / `ScopeId` / pointer (dioxus-native-dom-scoped → its service note); host shell mechanics (the tool's `grep` function, the Bash guards) — not a project invariant, the host leaf's subject (Tier 2).
- **Pointer table (28 rows)** — unchanged. The test, obs and a11y rows carry their U35 registry notes; the architecture row keeps `§Infrastructure Patterns / §Occupied Resources` with no registry note (not migrated); no row bakes a version-workspace path. Three past the template's "10-25 typical" — each added by a wrap's cascade for a contract a chunk built.
- **Workflow (5 commands)** — unchanged: `cargo build --workspace` · `bash .github/scripts/ci-leg.sh fast` · `cargo test -p blitz-tests --test {name}` + `bash scripts/agent-run.sh boot` / `run stand|all|{name}` · `bash .github/scripts/ci-leg.sh {leg}` · `just seven_guis`.
- **Architecture** — unchanged (2 paragraphs, arch §Design Philosophy + the as-built capability list against §Standard Contracts).
- **@imports** — `@.claude/session-handoff.md` only (U01 `ok`).
- **Deeper topics** — services line unchanged (9 notes, all present). **Rules line: the one change** — it names the rule files that exist after Phase 2, so it gains the host leaf, which loads on every turn like `security.md`:
  - old: `- \`security.md\` (always loaded) · \`testing.md\` · \`observability.md\` · \`a11y.md\` · \`verification-harness.md\``
  - new: `- \`security.md\` · \`host-linux.md\` (both always loaded) · \`testing.md\` · \`observability.md\` · \`a11y.md\` · \`verification-harness.md\``
- **USER:session-learnings** — preserved verbatim (2 bullets, 0.9 KB, 0 over 600 B). Setup edits no `USER:*` bullet.
- Expected size: 138 lines (no line added or removed).

## Tier 2 — .claude/rules/ (dispositions)

| File | Disposition |
|---|---|
| `security.md` (always loaded, 4.5 KB) | preserve |
| `testing.md` | preserve |
| `observability.md` | preserve |
| `a11y.md` | preserve |
| `verification-harness.md` | preserve |
| `host-linux.md` (always loaded) | **ABSENT → the tool renders it**: `upgrade.py apply --root . --id U04 --run-dir {run_dir}` — the linux sections of `rules-templates/host.md`, 2857 B, an empty `## Session Additions`. Never a hand render. |

The always-loaded rule total goes from 1 file · 4.5 KB to 2 files · about 7.3 KB on every turn.
Not planned (unchanged from the first run): `migrations` (only browser-persistence's rusqlite_migration — upstream app), `api` (no served API; the one listener is a lifecycle socket), `events` (no event bus), `frontend` (no TS / web frontend; a WASM canvas). No leaf drift note: no preserved leaf reads `noted`.

## Tier 3 — .claude/docs/ (dispositions)

- Core 5 — `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md`: **preserve** (U09 `ok`).
- Summaries 5 — `security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md`: **preserve** (U36 `ok`).
- Services, existing 9 — `blitz-dom` · `blitz-traits` · `blitz-paint` · `blitz-shell` · `blitz-test-harness` · `dioxus-native-dom` · `escher-telemetry` · `escher-driver` · `seven_guis`: **preserve**. No module was added to the workspace since the last note was written (escher-driver's exists), so nothing is ABSENT and nothing is written.
- `session-learnings.md` — present; untouched (wrap territory).

## Agent harness

- agent-driven; `scripts/agent-run.sh` (228 lines) and `scripts/agent-run.ps1` (4 lines, pass-through) present → **preserved**, no write. test-plan §3 is derived FROM the script (it cites the script's own lines), so the plans' 5-command params equal the script's — no param drift, no backup. U10 `ok`. `verification-harness.md` present → preserve.

## Hooks (.claude/settings.json)

- Formatter `rustfmt "$f"` on `*.rs` · linter none at write time (clippy is a gate) · type-checker none (matrix Rust rows). `rustfmt.toml` present (U05 `ok`) — no formatter config written, no reflow step.
- Re-run rule: U02 reads `ok` — `write current · bash current · PostToolUse on the stdin prologue` — so the three setup-rendered entries are the matrix's current forms and a re-render is byte-equal: **no write, no backup**. `env` (`PYTHONUTF8` · `PYTHONIOENCODING`) present. No user-managed entry exists in the file.
- Host: `jq` on PATH (`/usr/bin/jq`) · `rustfmt` on PATH (`~/.cargo/bin/rustfmt`).

## Code-graph pipeline

- Planes: rust (root `Cargo.toml`); no ts plane. `scripts/{code-graph.py, code-graph-views.sql, scip_pb2.py, requirements.txt, code-graph-cookbook.md}` all present → preserve; currency `py 0 · sql 0 · cookbook 0` → no drift, no backup, no proposal. `rust-analyzer` on PATH.
- Graph state: `.refresh-done` = `rust ok 61s 6530/35926`; `tree.db.commit` = `d606e718…` (= HEAD, stamped by the last wrap). Scripts were not seeded by this run, so no build is fired and P9 stamps nothing.

## Code reviewer

- `.claude/agents/code-reviewer.md` (Rust, 72 lines) present → **preserve**.

## Gitignore / gitattributes

- `.gitignore`: U07 `ok` — every base ignore decided by the root file at the root and at depth 2 → nothing appended.
- `.gitattributes`: U06 `ok` — carries `* text=auto eol=lf` → present, unchanged.

## Seeds (Phase 6)

- `.andromeda/state.yaml` (schema 3, session_count 29) · `.claude/session-handoff.md` · `.andromeda/drift-base.md` · `.andromeda/playbook.md` · `.claude/docs/session-learnings.md` — all present → nothing seeded.

## Planned writes (the manifest's set)

1. `.claude/backup/CLAUDE.md.pre-setup-2026-10-07T15-11-18` (backup, gitignored — written at Setup step 5)
2. `CLAUDE.md` — the Deeper Topics rules line (one anchored Edit)
3. `.claude/rules/host-linux.md` — new, by `upgrade.py apply --id U04` (U04)

## Consistency check

- Every rule file that exists after Phase 2 is listed in Deeper Topics (6 names: the 5 present + the host leaf); the services line names exactly the 9 files that exist; the agent-harness row and `verification-harness.md` agree (script present, rule present); the pointer table cites no baked version path; @imports = handoff only, and the file exists; the three registry files the pointer notes name exist under `.andromeda/registries/`; every setup-class `behind` row (U04) has a planned write and nothing else is `behind`; the host leaf is written by the tool alone and has no `paths:` frontmatter, so Phase 2's frontmatter validation reads 4 of 4 on the path-scoped files, as health check 4 does.
