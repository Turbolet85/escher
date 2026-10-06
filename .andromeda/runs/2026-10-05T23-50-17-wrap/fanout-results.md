# Fan-out results — 2026-10-05-ci-gate-legs (wrap 2026-10-05T23-50-17)

7 Explore doc-agents, one batch, prompt from `amendment-flow.md` §Fan-out sent verbatim; detector scoping
arch 2 · security 3 · design 1 · layout 1 · test 3 · obs 3 · a11y 2 = 15 = the drift-base's 15 single-doc entries.
Contracts lines: test-plan · obs-plan · a11y-plan (`registry.py contracts` exit 0); architecture `NOT MIGRATED`
(line dropped); security-plan · design-system · layout-templates `n/a` (line dropped). Entity probe over every
return: `&lt;` `&gt;` `&amp;` `&quot;` → entities=0.

## Verdicts
- **architecture** — 14 proposals (D-arch-resources 6 · D-arch-decisions 8).
- **security-plan** — 9 proposals (D-security-auth 1 · D-security-deps 8: primary + 7 dependents); stripped: a
  trailing `# D-security-input: no drift …` comment.
- **design-system** — `proposals: []`; stripped: a commentary block (D-design-tokens no drift; notes item 9's 2
  stale `ci.yml:` citations as the wrap's re-point task). Raw twin `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`; stripped: a commentary line (no UI surface). Raw twin.
- **test-plan** — 14 proposals (D-tests-coverage 2 · D-tests-framework 12); stripped: a trailing
  D-tests-obs-harness no-drift comment + "Line numbers for the re-pointed citations were read from the current
  .github/workflows/ci.yml, .github/scripts/ci-leg.sh and .github/scripts/test_ci_workflows.py".
- **obs-plan** — `proposals: []`; stripped: commentary (no drift under the three detectors; flags expected
  amendments 7 and 9). Raw twin.
- **a11y-plan** — `proposals: []`; stripped: commentary (no drift; flags a11y-plan :321, :77 and the
  `a11y-ci-gate-wire` key as expected amendment 8's sites). Raw twin.

## Parsed proposals + dispositions

### architecture
| # | detector | section | change (abridged) | disposition |
|---|---|---|---|---|
| A1 | D-arch-resources | §Standard Contracts → CI contracts (:137) | leg list + `audit` `a11y` `coverage`, doc workspace-wide; coverage job's `coverage-report` upload; test_ci_workflows new pins | **apply** (check 1: *Accurate this-chunk addition*; expected 1) — the `coverage-report` artifact clause rides **E1** |
| A2 | D-arch-resources (dep. A1) | §Infrastructure Patterns → CI/CD (:174) | 14 jobs, twelve linux leg jobs, doc job apt step, workflow `permissions` | **apply** (check 1; expected 1); artifact clause → **E1** |
| A3 | D-arch-resources | §Occupied Resources → Filesystem (:145) | register `target/coverage/` + root `deny.toml` | `target/coverage/` **apply** with **E1** (an artifact path — the registry enumerates them: `target/ci-logs/`); `deny.toml` half **reject** (check 1: *Registry over-reach* — an individual config file; the cargo-deny config is recorded under Build system, A12) |
| A4 | D-arch-resources | §Occupied Resources → CI infrastructure (:151) | `workspace-test` shared key, a11y restore-only, audit/coverage cacheless, 10.72 GB re-measure | **apply** (check 1; expected 2) |
| A5 | D-arch-resources (dep. A1) | §Infrastructure Patterns → CI/CD (:174) | rust-cache "every compiling job" qualifier | **apply** (check 1) |
| A6 | D-arch-resources (dep. A1) | §Stack and Technologies → CI/CD (:66) | rust-cache qualifier | **apply**, merged with A7 (same cell) |
| A7 | D-arch-decisions | §Stack and Technologies → CI/CD (:66) | SHA pins replace "tag-pinned"; cargo-deny / cargo-llvm-cov / install-action / llvm-tools-preview | **apply** (check 1; expected 1) — the "publish and wpt remain tag-pinned" clause is not carried by the report → dropped from the applied text (re-derived) |
| A8 | D-arch-decisions (dep. A7) | §Standard Contracts → CI contracts (:137) | upload-artifact by SHA pin | **apply** |
| A9 | D-arch-decisions (dep. A7) | §Infrastructure Patterns → CI/CD (:174) | rust-cache by SHA pin; test_j/test_k | **apply** |
| A10 | D-arch-decisions | §Stack and Technologies → Code quality (:67) | real workspace rustdoc gate; audit + coverage legs; Role column | **apply** (check 1; expected 3); the coverage % is not carried into arch (test-plan §10 owns readings) |
| A11 | D-arch-decisions (dep. A10) | §Conventions → Formatting and lints (:104) | rustdoc `-D warnings` holds the workspace | **apply** |
| A12 | D-arch-decisions (dep. A10) | §Infrastructure Patterns → Build system (:155) | rustdoc gate reaches library crates; cargo-deny gate + `deny.toml` | **apply** (expected 1) |
| A13 | D-arch-decisions (dep. A10) | §Infrastructure Patterns → CI/CD (:174) | docs job a real workspace gate (+ apt step) | **apply** (expected 1) |
| A14 | D-arch-decisions (dep. A10) | §Inherited Defaults → Code quality (:217) | real rustdoc gate; audit + coverage legs | **apply** (expected 3) |

### security-plan
| # | detector | section | change (abridged) | disposition |
|---|---|---|---|---|
| S1 | D-security-auth | §Authentication & Authorization → RBAC table, ci.yml row (:49) | workflow `permissions: contents: read`, no job grant (test_l) | **apply** (check 1: *Accurate this-chunk addition*; expected 5) |
| S2 | D-security-deps | §Dependency Security → Pinning (:215) | cargo-deny / cargo-llvm-cov installs; ci.yml all SHA-pinned; `@latest` only in guarded workflows; rustls bump | **apply** (check 1: *Accurate this-chunk addition* — no ban exists to violate; the detector's `escalate` severity is its default for a banned dependency, and none is banned; expected 4 names the change) |
| S3 | D-security-deps (dep. S2) | §Dependency Security → Audit tool (:207) | cargo-deny 0.20.2 + deny.toml, one per-ID ignore; reach = resolved graph | **apply** the tool + config; the REACH clause (paste / memmap2 unreached) → **E2** (who owns the gap) |
| S4 | D-security-deps (dep. S2) | §Dependency Security → CI integration (:221) | `audit` job runs `cargo deny --locked check advisories` | **apply** |
| S5 | D-security-deps (dep. S2) | §Dependency Security → Update policy (:219) | dependabot search restated inline | **apply** (keeps the claim's evidence when S3 rewrites :207) |
| S6 | D-security-deps (dep. S2) | §Dependency Security NOT YET MEASURED note (:225) | narrowed to the CVE-response SLA | **apply** |
| S7 | D-security-deps (dep. S2) | §Supply chain integrity → Lockfile verification (:230) | `--locked` enumeration + audit/a11y/coverage; doc workspace-wide | **apply** |
| S8 | D-security-deps (dep. S2) | §Bootstrap phases → `dep-audit-tooling-install` (:240) | discharged | **apply** (expected 4) |
| S9 | D-security-deps (dep. S2) | §Bootstrap phases → `dep-security-ci-gate` (:242) | discharged | **apply** (expected 4) |

### test-plan
| # | detector | section | change (abridged) | disposition |
|---|---|---|---|---|
| T1 | D-tests-coverage | §4 → CI workflows and leg script (:111) | the 23-test unittest description + new invariants; cite `test_ci_workflows.py:80-264` | claim **apply**; its citation numbers are the **re-derivation tell** (read from source, not the report) → citation re-derived by the orchestrator under check 5 (expected 9) |
| T2 | D-tests-coverage (dep. T1) | §9 Pipeline facts → Cache (:271) | SHA-pinned rust-cache, coverage/audit cacheless, `workspace-test` shared, a11y restore-only | claim **apply**; ci.yml line numbers → orchestrator re-point (tell) |
| T3 | D-tests-framework | §9 Observed absent → Coverage (:288) + new Coverage pipeline fact | coverage tooling present (cargo-llvm-cov 0.9.1, lcov + report, no threshold, `coverage-report`) | **apply** (expected 6); `coverage-report` clause rides **E1**; line numbers → orchestrator |
| T4 | D-tests-framework (dep. T3) | §3 → Bootstrap phases key `coverage-tooling-install` (registry file :3) | discharged | **apply** (expected 6) |
| T5 | D-tests-framework | §9 Pipeline facts → Legs (:269) | twelve linux legs incl. audit / a11y / coverage; doc workspace-wide | claim **apply**; line numbers → orchestrator |
| T6 | D-tests-framework | §9 Pipeline facts → Local baseline (:284) | workspace rustdoc exit 0, no collision | **apply** (expected 6) |
| T7 | D-tests-framework | §10 Quality Gates & Coverage Targets (:294) | first coverage reading 53.23 % host / 53.25 % CI, 151 files, no threshold | **apply** (check 1: *Accurate this-chunk addition*; the marker kept for the unmeasured part) |
| T8–T14 | D-tests-framework (dep. T3) | §1 / §2 / §9 citation re-points (:7, :8, :31, :56, :270, :272, :273) | line-number re-points only | **reject** (re-derivation tell: every number was read from the current sources, which the report does not carry) → **re-raised by the orchestrator** under check 5 (expected 9), numbers re-read by the orchestrator |

## Check 5 — Expected amendments (plan list = coverage floor)
| entry | proposals | disposition |
|---|---|---|
| 1 arch CI/CD / Build system | A1 A2 A5 A9 A12 A13 | covered |
| 2 arch Occupied → CI infrastructure | A4 | covered |
| 3 arch Code quality (Stack + Inherited Defaults) | A10 A11 A14 | covered |
| 4 security Dependency Security + 2 keys | S2–S9 | covered (reach → E2) |
| 5 security Auth & Authz | S1 | covered |
| 6 test-plan §9 + §3 key | T1–T7 | covered |
| 7 obs-plan §9 `coverage-report` row | none | **raised by the orchestrator** → **E1** (Boundary widening — never routine, escalates whatever the plan says) |
| 8 a11y-plan §9 + §3 key | none | **raised by the orchestrator, routine** (check 1: *Accurate this-chunk addition*; report Harness): a11y-plan :321, :77, `registries/contracts/a11y-plan/bootstrap-phases-…md:4` |
| 9 stale `file:line` citations into ci.yml / ci-leg.sh / test_ci_workflows.py | T8–T14 (rejected) | **raised by the orchestrator, routine** (the 2026-10-05-fork-ci-reached precedent): every master's citation into the three files re-read and re-pointed — sweep in `cascade-dispositions.md` |

## Check 6 — Spec claims disproved (report)
1. plan step 5 "all-features … every graph a user can build" → S3 + **E2**.
2. plan step 4 `report --summary-only` → no master states it (`llvm-cov` 1 hit, test-plan :288 — the search pattern of the observed-absent line, retired by T3); plan-only → **curation** (sweep hazard).
3. plan prose operator-entry numbering → plan-only, no master → **curation**.
4. arch "≈ 9.73 GB" → A4.
5. shared-key replaces the job-id part → holds; nothing to dispose.

## Check 2 / 3 / 4
- Cross-contradiction: none — A6/A7 edit one cell in the same direction (merged); T1/T2 and A5 state the same cache rule.
- Intent-consistency: the five report deviations are justified (the plan's own step-6 requirement; the CARRY's docs job made to run; a measured tool behaviour ×2; an explicit form of the plan's own "on success"); scope record empty (`gate.py scope` clean).
- Absence-needs-evidence: S5 / S6 / T7 retain their "observed absent / not measured" halves with their searches; no new absence claim enters without one.

## Escalations
- **E1 — Boundary widening:** the `coverage-report` artifact, the first fork-CI upload outside `target/ci-logs/`
  (obs-plan §9 row; arch CI contracts / CI/CD / Filesystem clauses A1-A3; test-plan T3 clause). The plan records
  the overseer's P5 word under the founder's standing delegation — a `delegate` answer, PROVISIONAL; a boundary
  widening is ratified only by the operator's own word (`gate-contract.md` §Scope, *Whose word*).
- **E2 — Audit reach gap:** cargo-deny's resolved graph leaves RUSTSEC-2024-0436 (paste) and RUSTSEC-2026-0186
  (memmap2) ungated; no route entry owns closing it.

## Resolutions (HALT cleared at this wrap)
- **E1** — applied PROVISIONAL: the overseer judged the upload technical (public OSS line counts, no secret, the
  repository's own Actions store, 7 days) and ratified it under the founder's standing delegation of technical
  decisions, the founder's words relayed verbatim by overseer (2026-10-05, the overseer session) — authority
  `delegate overseer, 2026-10-05`; NOT the founder's own word on this upload. A boundary widening is ratified only by
  the operator's own word: the record stays provisional, on the founder's morning list (overseer handoff FOR
  DISCUSSION 4); his later word supersedes it. Clauses applied: obs-plan §9 row; arch CI contracts / CI/CD /
  Filesystem; test-plan §9 Failure logs + Coverage. Each sidecar entry names the authority.
- **E2** — `CARRY` on the Epoch 6 "Quality gates" route entry (P5 of this wrap), on the overseer's word (the
  recommended option); security-plan states the reach truthfully.

## Tally
37 proposals: 30 applied (A3 half-rejected — `deny.toml` registry over-reach), 7 rejected (T8–T14, re-derivation
tell) and re-raised by the orchestrator; 3 orchestrator raises (obs row under E1, a11y-plan §3/§9 + key, citation
re-point across 5 masters); 2 escalations, both resolved. Open escalations: 0.
