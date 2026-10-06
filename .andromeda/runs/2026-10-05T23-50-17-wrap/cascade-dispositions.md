# Cascade dispositions — 2026-10-05-ci-gate-legs (wrap 2026-10-05T23-50-17)

## Step 1 — bodies applied
- architecture.md: 15 anchored edits (Stack → CI/CD, Code quality · Conventions → Formatting and lints · Standard
  Contracts → CI contracts · Occupied Resources → Filesystem, CI infrastructure · Infrastructure Patterns → Build
  system, CI/CD · Inherited Defaults → Code quality).
- security-plan.md: 9 (Auth & Authz RBAC row · Dependency Security: Audit tool, Pinning/CI tooling, Update policy,
  CI integration, NOT-YET-MEASURED note · Supply chain → Lockfile verification · Bootstrap phases ×2).
- test-plan.md: 10 (§4 CI workflows and leg script · §9 Legs, Cache, Failure logs, Local baseline, Coverage (was
  Observed absent) · §10) + registry key `test-plan/bootstrap-phases-…md` (`coverage-tooling-install`).
- obs-plan.md: 1 (§9 artifact table: `Coverage report` row).
- a11y-plan.md: 2 (§3 CI integration line :77 · §9) + registry key `a11y-plan/bootstrap-phases-…md`
  (`a11y-ci-gate-wire`). `registry.py check` → 0 defects for test-plan and a11y-plan.
- Citation re-point (expected amendment 9, raised by the orchestrator): one script over the seven masters + every
  `.andromeda/registries/**` file, the ci.yml map measured base→chunk (+4 from base 14 · +6 from 59 · +8 from 78 · +9
  from 80 · +11 from 98 · +13 from 197 · +14 from 200 · +91 from 208), ci-leg.sh +7 from base 33, two span overrides
  where the cited span grew (`ci-leg.sh:20-32` → `20-38`; `test_ci_workflows.py:61-177` → `80-264`); 28 lines / 70
  citations re-pointed (architecture 11 lines · security-plan 4 · design-system 2 · test-plan 10 · obs-plan 1;
  layout-templates, a11y-plan, registries 0). Spot-checked against the base: ci.yml 24-26 / 217-218 / 237-238 read
  identical at 28-30 / 308-309 / 328-329.

## Step 2 — sweep (`cascade.py sweep`, `cascade-patterns.toml`, 17 patterns, every control fired)
Patterns cover each retired claim's tokens AND its mechanism phrasing: `nine linux` · `16 tests` · `tag-pinned` ·
`rust-cache@v2` · `upload-artifact@v7` · `bare \`cargo doc|cargo doc --locked` · `no library crate|nominal doc|
documents only the|documenting only the` · `exit 101|9 errors` · `coverage tooling` · audit-absent phrasings ·
`permissions block … absent` · `(accessibility|a11y) checks in CI` · `@latest` · `9.73|9 728 732 229|≈ 97 %` ·
`every compiling` · `reached the lockfile` · `only path the CI|target/ci-logs/ alone|only … target/ci-logs`.
Dropped: `CI gate legs` (its control never fires over the masters) — swept by hand below.

Rows (11 master/registry · 9 leaf), each dispositioned:
| row | disposition |
|---|---|
| architecture.md:173 `upload-artifact@v7` (standing) | no change — a true claim about `publish-browser.yml` (upstream-only, untouched); window read: "uploaded unzipped with `actions/upload-artifact@v7` (.github/workflows/publish-browser.yml:176-182)" |
| test-plan registry key :3 `coverage tooling` (edited) | amended this pass — the new text states the tooling present |
| security-plan.md:215 `@latest` (new) | amended this pass — the remaining `@latest` refs are the upstream-only workflows' (true) |
| architecture.md:66, :174 (@c1829), test-plan.md:111, :271 `every compiling` (edited) | amended this pass — each carries the `but coverage` qualifier; no intra-line duplicate of the retired universal |
| architecture.md:137 (@c2222), :145 · obs-plan.md:285 `only … target/ci-logs` | no change — each speaks of the FAILURE logs (still `target/ci-logs/` only); the success-only `coverage-report` is stated beside each (arch :137, :145) or in its own row (obs :286) |
| `.claude/docs/commands.md:40`, `stack.md:39` (×2 patterns), `stack.md:41`, `tests-summary.md:38` | leaf → re-derived (step 3) |
| `.claude/rules/testing.md:44`, `.claude/rules/security.md:20` | leaf → re-derived (step 3) |
Zero-row patterns (control fired): `nine linux` · `16 tests` · `tag-pinned` · `rust-cache@v2` · `permissions block` ·
`a11y checks in CI` · `9.73…` · `reached the lockfile` — each retired claim gone from every master and registry file.

Hand sweep (the dropped pattern + phrasings): `CI gate legs|red at baseline|rustdoc.{0,40}red|cargo-audit|coverage
tooling|accessibility checks in CI|a11y checks|tag-pinned|nine linux|16 tests|9\.73|permissions\` block|no audit` over
`CLAUDE.md` and `.claude/` (handoff excluded — P6 rewrites it) → 11 hits, all leaves: CLAUDE.md:49 (GENERATED
warnings) · a11y-summary.md:31 · rules/security.md:20 · tests-summary.md:35, :38 · commands.md:40 · rules/a11y.md:38 ·
workflow.md:39 · security-summary.md:19, :32 · rules/testing.md:44 → all re-derived. Curation homes (CLAUDE.md
`USER:session-learnings`, each rule's `## Session Additions`, `docs/session-learnings.md`): 0 hits. Judgment bases
(`playbook.md`, `drift-base.md`): 0 rows on any pattern.

## Step 3 — leaves re-derived (recomputed from the amended masters)
CLAUDE.md `GENERATED:setup:warnings` (the done-gate line: `ci-leg.sh doc` replaces the "red at baseline" clause) and
`GENERATED:setup:workflow` (the leg list) · `.claude/docs/stack.md` (Code quality, CI) · `commands.md` (docs gate +
the three gate legs) · `workflow.md` · `tests-summary.md` (Coverage, Format/lint/docs rows) · `security-summary.md`
(CI line; the "not yet measured" owner list now names the audit-reach gap → "Quality gates") · `a11y-summary.md`
(bootstrap item 4) · `obs-summary.md` (CI artifacts) · `.claude/rules/security.md`, `testing.md`, `a11y.md` (their
generated bodies; `## Session Additions` untouched). Re-sweep of the same hand patterns after the rewrite: 0 hits.
CLAUDE.md overview / modules / pointer-table / architecture blocks: structurally re-read — they state no CI-leg,
cache, pin or rustdoc fact; unchanged. Lateral binds: test-plan §3 ↔ obs-plan §3 and a11y ↔ obs schema untouched by
this pass.
