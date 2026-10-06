# security-plan — amendments

One entry per amendment to `security-plan.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-fork-ci-reached — signing and PR-token jobs repository-guarded; CI legs locked
**Section:** §Threat Model Summary (CI workflow triggered by pull requests) · §Authentication & Authorization (RBAC — the publish job row) · §Dependency Security (Pinning · Supply chain integrity — Signed artifacts, Lockfile verification) · §Logging & Monitoring (CI publish)
**Change:** was the publish job using "Signed Builds" only on main or `ci-test` branches — a ref-only gate — and post-results running for successful `pull_request` WPT runs; now `release-cli` and `post-results` (and the WPT jobs) carry `github.repository == 'DioxusLabs/blitz'`, so no fork ref — the fork's `main`, `ci-test/*`, `build/**` — reaches the environment, its secrets, a signed artifact or the PR-writing token; publish-build trace logging is upstream-only too. `--locked` now covers every ci.yml cargo leg (the leg script and the matrix) beside the flake and `dx bundle`, `examples/wasm_hello` excepted (no `Cargo.lock`). ci.yml references no `secrets.`, pinned by a unit test.
**Why:** the fork holds no secrets or environments, and a ref-only `if:` re-arms signing on the fork's own `main` and its existing `ci-test/sign-android-builds` branch; a repository guard keeps upstream byte-equal.
**Kept:** the write / `always()`-remove pairs for the signing key and keystore are unchanged.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `ci.yml`, `wpt.yml` or `publish-browser.yml` lines
**Change:** 23 citations re-pointed — `publish-browser.yml` from line 37 on +1 and `wpt.yml` from line 26 on +1 (the repository guards), `ci.yml` by a range map over the rewritten file; no claim text changed by the re-point.
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-ci-gate-legs — cargo-deny audit leg, SHA-pinned ci.yml, a read-only ci.yml token
**Section:** §Authentication & Authorization (RBAC — the ci.yml row) · §Dependency Security (Audit tool · Pinning · Update policy · CI integration · NOT YET MEASURED) · §Supply chain integrity (Lockfile verification) · §Bootstrap phases (`dep-audit-tooling-install` · `dep-security-ci-gate`)
**Change:**
- was "a workflow-level permissions block in ci.yml is observed absent"; now ci.yml declares `permissions: contents: read`, no job-level grant — every ci.yml `GITHUB_TOKEN` read-only, pinned by a unit test;
- was "Audit tool: observed absent" / "CI integration: no audit step"; now cargo-deny 0.20.2 runs `cargo deny --locked check advisories` as ci.yml's `audit` job on every push, slow tier, uncached, configured by the root `deny.toml`: `[graph]` the six ci.yml platforms with `all-features = true`; per-ID ignores only with a written reason, no blanket allow, `unmaintained`/`unsound` at their defaults;
- one ignore: RUSTSEC-2026-0192 (ttf-parser 0.25.1, unmaintained, no patched release), reached only through the exact winit beta pin — a bounded deferral the audit leg re-reads every push; RUSTSEC-2026-0285 fixed by the lockfile update rustls 0.23.43 → 0.23.45 (no build graph reaches rustls);
- the audit's reach is cargo-deny's resolved graph, not the lockfile: 0.20.2 prunes `http-cache` (blitz-net's `cache` feature) and `ravif`, so RUSTSEC-2024-0436 (paste 1.0.15) and RUSTSEC-2026-0186 (memmap2 0.5.10) never reach the gate;
- Pinning: was `awalsh128/cache-apt-pkgs-action` at `@latest` in ci.yml; now every ci.yml `uses:` pinned to a 40-hex SHA; the upstream-only publish-browser and wpt workflows keep `@latest`; ci.yml installs cargo-deny 0.20.2 and cargo-llvm-cov 0.9.1 through `taiki-e/install-action`;
- `--locked` covers the new audit, a11y and coverage legs; the NOT YET MEASURED note narrows to the critical-CVE response SLA; the Update-policy search is stated inline; both bootstrap keys read discharged.
**Why:** the CI gate legs chunk (cargo-deny chosen over cargo-audit at its P4 fork — cargo-audit fails on vulnerabilities only, leaving unmaintained and unsound findings unowned).
**Kept:** the guarded workflows' `permissions` rows and their `@latest` references are unchanged (upstream-only; a ref-only edit would re-arm them on the fork).
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

## 2026-10-06-telemetry-bootstrap — escher's log sink scrubs; logging-redaction-wire discharged for it; RUST_LOG read
**Section:** Data Protection (At rest) · Bootstrap phases · Secret Management (Environment values read) · Logging & Monitoring (Log format and backends) · every section citing `Cargo.toml` or `tests/blitz-tests/Cargo.toml` lines
**Change:**
- `logging-redaction-wire`: was recorded absent; now discharged for escher's own sink (`escher_telemetry::init`'s allowlist scrub in `seven_guis_native`), still open for the upstream `fmt::init()` stdout subscribers and the WPT runner's `env_logger`.
- Logging & Monitoring gains escher's sink: stderr only; engine targets (`blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer`, `js_console`) print only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`; `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload` redacted at any target; its reach is that sink only — the std panic hook's raw message and `log.file` host paths stay as-is; the examples' `println!`-only bullet names `seven_guis_native`'s sink.
- Data Protection: the s05 redaction-absent search stands, qualified with escher's sink.
- Environment values read gains `RUST_LOG` (not secret).
- 3 `file:line` citations re-pointed by the chunk's measured line shifts.
**Why:** the telemetry bootstrap chunk wired the scrub at escher's subscriber; the opt-in OTel export (egress + the `OTEL_EXPORTER_OTLP_HEADERS` credential path) was deferred at P4 by the overseer delegate under the founder's standing delegation of technical forks, provisional on the founder's word, and adds no surface here.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/

## 2026-10-06-headless-stand — `disabled` row: parsed for focus, presence for state and clicks
**Section:** §Input Validation → Markup attributes (`disabled`)
**Change:** was "Parsed as a boolean value; elements with it ignore pointer selection and click default actions"; now parsed as a bool for focusability only, while its presence alone — `disabled="false"` included — sets the DISABLED element state and makes the element ignore pointer selection and click default actions.
**Why:** the headless stand chunk measured `disabled="false"` matching `:disabled`; the row conflated the two readers.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/
