# Fan-out results — 2026-10-05-fork-ci-reached

Seven Explore doc-agents, one batch, prompt per `amendment-flow.md` §Fan-out. Returns arrived as plain YAML (no HTML
entities; `&&` literal); each was preamble-free except a trailing `#` commentary block, stripped (its substance noted
per doc). Proposal `change` text is condensed below to its claim; the applied body text is re-derived from the report
(Apply step 1), never pasted.

## Verdicts

- architecture — 12 proposals (D-arch-resources 6 · D-arch-decisions 6)
- security-plan — 3 proposals (D-security-auth 3); stripped: D-security-input satisfied (leg allow-list, exit 2),
  D-security-deps no ban list and no dependency added
- design-system — `proposals: []`; stripped: every new surface `tokens n/a`
- layout-templates — `proposals: []`; stripped: no user-facing surface
- test-plan — 10 proposals (D-tests-framework 9 · D-tests-coverage 1); stripped: D-tests-obs-harness no drift
  (§3 ↔ obs §3 both NOT YET MEASURED)
- obs-plan — 7 proposals (D-obs-instrumentation 7, the agent noting §9 staleness filed under the nearest detector);
  stripped: D-obs-stack and D-obs-pii no drift (build output, no user data)
- a11y-plan — `proposals: []`; stripped: no interactive element, schema untouched; a note that §9's "a11y checks
  observed absent" predates this chunk (not a change made here)

## Proposals and dispositions

Checks: 1 playbook · 2 cross-contradiction · 3 intent · 4 absence-needs-evidence · 5 expected-amendments · 6
disproved-claims. Rule A = "Accurate this-chunk addition" (routine, apply).

### architecture
| # | detector | section · basis | claim | disposition |
|---|---|---|---|---|
| A1 | D-arch-resources | §Standard Contracts → CI contracts · :137 | register the `ci-leg.sh {leg}` contract (legs, exit 2, per-leg log, `fast` pre-push gate) and the `ci-log-*` failure artifacts | apply — rule A (the bullet already registers CI-script CLI contracts at this grain, `wpt_diff_to_pr.py`); check 4: :137 read whole (1254 c), lists only wpt_diff_to_pr + WPT artifacts |
| A2 | D-arch-resources | §Standard Contracts → CI contracts · :137 | the `update-results` dispatch is upstream-only (`trigger-archive` guard) | apply — rule A |
| A3 | D-arch-resources | §Occupied Resources → CI infrastructure · :151 | environments / warp runner reached only from `DioxusLabs/blitz`; the fork's Actions cache occupancy (12 entries, ≈9.73 GB of 10 GB) | apply — rule A + plan Expected amendment; occupancy written `as measured at` evidence/operator-pass.md |
| A4 | D-arch-resources | §Occupied Resources → Filesystem · :145 | register `target/ci-logs/{leg}.log` / `matrix-{platform}.log` | apply — rule A (Filesystem enumerates written paths, incl. `./target/dx/…`); check 4: :145 read whole (1258 c), no ci-logs entry |
| A5 | D-arch-resources (dependent-of A3) | §Infrastructure Patterns → Deployment model · :173 | browser bundling upstream-only | apply — rule A |
| A6 | D-arch-resources (dependent-of A3) | §Inherited Defaults (Deployment) · :214 | same, Inherited Defaults line | apply — rule A |
| A7 | D-arch-decisions | §Established Decisions [Build profiles] · :74 | add `[profile.dev] debug = "line-tables-only"` (test inherits) | apply — plan Expected amendment (named change) + rule A |
| A8 | D-arch-decisions | §Stack → Code quality · :67 | gates via `ci-leg.sh`, clippy/doc `--locked`, `fast` pre-push gate | apply — rule A |
| A9 | D-arch-decisions (dependent-of A8) | §Conventions → Formatting and lints · :104 | clippy command `--locked` via the leg | apply — rule A |
| A10 | D-arch-decisions (dependent-of A8) | §Inherited Defaults (Code quality) · :217 | same | apply — rule A |
| A11 | D-arch-decisions | §Stack → CI/CD · :66 | add bash leg runner, `test_ci_workflows.py` and its PyYAML dependency, rust-cache / upload-artifact | apply — rule A; PyYAML is in the report's Schema/config (`ci-scripts` ensures PyYAML) — the report's Dependencies bullet under-stated it ("none added"), corrected in report.md before apply (see below) |
| A12 | D-arch-decisions (dependent-of A8) | §Infrastructure Patterns → CI/CD · :174 | trigger `build/**`, fast/slow `needs`, legs via script, rewrite removed, matrix without linux + `--locked` + log tee, rust-cache save rule, failure logs, WPT/publish upstream-only | apply — plan Expected amendment (named change) |

### security-plan
| # | detector | section · basis | claim | disposition |
|---|---|---|---|---|
| S1 | D-security-auth | §Authentication & Authorization (RBAC row) · :48 | publish job repository-guarded before its ref-keyed "Signed Builds" | apply — plan Expected amendment (named change) |
| S2 | D-security-auth (dependent-of S1) | Signed artifacts bullet · :229 | same restatement | apply — rule A |
| S3 | D-security-auth (dependent-of S1) | §Threat Model Summary (CI PR vector) · :22 | post-results also repository-guarded | apply — rule A |

### test-plan
| # | detector | section · basis | claim | disposition |
|---|---|---|---|---|
| T1 | D-tests-framework | §9 Platform · :264 | PRs + pushes to main / v0.* / build/** | apply — plan Expected amendment |
| T2 | D-tests-framework | §9 Pipeline facts · :266-272 | legs via `ci-leg.sh`, `--locked`, fast/slow, rust-cache rule, 7-day failure logs, `fast` pre-push gate | apply — plan Expected amendment |
| T3 | D-tests-framework (dependent-of T2) | §1 Workspace (CI) · :7 | `cargo test --workspace --locked` via the `test` leg | apply — rule A |
| T4 | D-tests-framework (dependent-of T2) | §1 CI Python scripts · :8 | via the `ci-scripts` leg, PyYAML ensured, 16 tests | apply — rule A |
| T5 | D-tests-framework | §1 Surfaces under test · :31 | matrix windows/macos test, ios/android build, linux dropped, `--locked` | apply — plan Expected amendment |
| T6 | D-tests-framework | §9 WPT workflow · :268 | WPT upstream-only | apply — rule A |
| T7 | D-tests-framework (dependent-of T6) | §1 Web Platform Tests · :32 | same | apply — rule A |
| T8 | D-tests-framework (dependent-of T6) | §6 Drivers — WPT · :155 | WPT run and PR posting upstream-only | apply — rule A |
| T9 | D-tests-framework | §9 Local baseline · :273-277 | re-measured under `debug = "line-tables-only"` | apply — plan Expected amendment; the prior rustdoc reading (exit 101, 3 crates, 9 errors) kept with its own evidence pointer — not re-measured here |
| T10 | D-tests-coverage | §4 What unit tests cover · :110 | add `test_ci_workflows.py` coverage | apply — rule A |

### obs-plan
| # | detector | section · basis | claim | disposition |
|---|---|---|---|---|
| O1 | D-obs-instrumentation | §9 artifact table · :281-284 | per-leg CI log row: failure-only upload, 7 days, unscrubbed | apply — plan Expected amendment |
| O2 | D-obs-instrumentation (dependent-of O1) | §9 NOT YET MEASURED · :288 | narrow to snapshot upload + resource attributes | apply — plan Expected amendment (keeps the marker for the unmeasured part) |
| O3 | D-obs-instrumentation | §9 WPT row · :283 | archive + dispatch upstream-only | apply — rule A |
| O4 | D-obs-instrumentation (dependent-of O3) | §9 `wptscores.json` row · :284 | upstream-only | apply — rule A |
| O5 | D-obs-instrumentation (dependent-of O3) | §5 WPT scores (CI) row · :116 | upstream-only | apply — rule A |
| O6 | D-obs-instrumentation | §9 publish-build log bullet · :286 | upstream-only | apply — rule A |
| O7 | D-obs-instrumentation (dependent-of O6) | §6 CI publish builds · :198 | upstream-only | apply — rule A |

Re-derivation tell (Validate preamble): several `change` lines cite workflow `file:line` locations the report does not
carry (e.g. `ci-leg.sh:55-56`, `ci.yml:44-49`). The FACTS each carries are in the report; the citations are not used —
the orchestrator re-derives every applied citation from the files itself. No proposal rests on a fact outside the report.

## Checks
- 2 cross-contradiction: none — A8/A9/A10 move the same clippy claim one way; A12 and T2 agree.
- 3 intent: the report's deviations are justified (ordering, stricter tests, the stray file, entry 15's `-R`); scope
  record none.
- 5 expected amendments (plan list, 6 entries → arch CI/CD A12 · [Build profiles] A7 · Occupied CI infrastructure A3 ·
  security A&A S1 · test-plan §9 CI Integration T1/T2 + Local baseline T9 + §1 Surfaces T5 · obs §9 O1/O2): all matched.
- 6 disproved claims (3, all plan-only — no master states them): the 6+4 cache forecast → measured 11+1 recorded in
  evidence/operator-pass.md and A3's occupancy; the "far below" warm forecast → measured −46 % recorded in the report
  and evidence; entry 15's bare `gh` → curation (the `-R` sweep hazard, P3).
- Escalations: 0.

## Orchestrator raises beyond the proposals
- Citation re-pointing (cascade step 2, verbatim citations): this chunk moved four cited files — 114 `file:line`
  citations across architecture, security-plan, design-system, test-plan, obs-plan went stale (Cargo.toml ≥ 195 +3 ·
  wpt.yml ≥ 26 +1 · publish-browser.yml ≥ 37 +1 · ci.yml rewritten, 22-range map). 112 re-pointed by a scratchpad
  script (digits-only change verified per master: the digit-stripped text equals HEAD's); the 2 citing the removed
  rewrite (ci.yml:34-35 / :44-45, arch:174) are replaced by A12. 0 hits in `.andromeda/registries/`.
- Restatements no detector proposed, found reading the sites and by the cascade sweep — routine (rule A), applied:
  security-plan.md:230 and :211 (`--locked` now covers the ci.yml legs), security-plan.md:351 (publish-build logging
  upstream-only), architecture.md:143 (the WPT report fetch is the upstream-only `wpt` job's).
- report.md Dependencies bullet corrected: PyYAML (Ubuntu `python3-yaml`, ensured by the `ci-scripts` job) is a new
  CI-script dependency; the bullet read "none added". security-plan's D-security-deps verdict is unchanged by it
  (§Dependency Security records no ban list — the agent's stripped note).
