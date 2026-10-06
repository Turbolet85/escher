# Cascade dispositions — 2026-10-06-cold-agent-run-pipe

Sweep: `cascade.py sweep` over `cascade-patterns.toml` (baseline d73df1a4, the pre-CI parent), listing
`sweep-listing.txt`. This pass's amendments are 17 additions and 1 retirement (T2: the CI-scripts count "37 tests …
CI run 37409303977"). Patterns: the retired count (`ci37`, `run37`); the sibling contract every addition lands beside
(`agentrun`); the "nothing crosses" claims the widened crossings could stale (`secretread`, `noport`, `outbound`).
Dropped: `credpath` (`(?i)credential path`) — its control never fired on the pre-pass masters, so no master stated
the phrase to stale; controlled by hand: `grep -ci 'credential path'` over the seven pre-pass masters → 0.

## ci37 — 1 row
- `.claude/docs/tests-summary.md:26` leaf "37 CI-script tests" → **re-derived** (step 3; now 64). The one master site
  (test-plan §4, line 127 pre-pass) was amended (T2); 0 master rows remain.
## run37 — 0 rows (control fired at test-plan.md:127 pre-pass) — the retired CI-run citation stands nowhere.

## agentrun — 55 rows (new 12 · standing 22 · leaf 21)
- **new (12):** arch 66, 139 (the +offsets), 145, 147 (the +offsets), 148, 172 · security 258, 366 · test 121, 122, 138,
  140 · obs 85, 295 — this pass's own text; the token matches inside the marker `cold-agent-run-pipe` or names
  `target/agent-run/` surviving cold-agent's cleanup → **no change** (true).
- **standing, sibling sites (21):** arch 139 (agent-run CI contract) · arch 147 (agent-run state area) · security 66
  (agent-run.sh Input Validation row) · security 365 (agent-run logging bullet) · layout 71 (agent-run.sh Primary
  screens) · test 102–110 (§3 agent-run block, 9 rows) · test 139 (§4 agent-run contract tests) · obs 83 (§3 agent-run
  log) · obs 146, 148, 149 (§6 agent-run format) · obs 294 (§8 agent-run scrub bullet) · obs 322 (§9 agent-run row) →
  **no change**: each is a true claim about agent-run, unchanged; each now has its cold-agent sibling beside it (A1, A2,
  R2, R3, L1, T3, T1, O1, O2, O4, O3). None claims agent-run is the only harness: each row scanned at its offsets
  for `(?i)\b(only|sole|single|the one|exclusive|no other)\b` — 23 matches, every one scoped to agent-run's own
  state or behaviour, or to this pass's text; the one global claim, arch 139 @2498 "the one fork-CI artifact outside
  `target/ci-logs/`" (`coverage-report`), stays true — `target/cold-agent/` is uploaded by no CI job.
- **standing, false match (1):** a11y 57 "agent-runnable invariants" → **no change**.
- **leaf (21):** CLAUDE.md 65, 85 · rules/observability.md 25 · rules/testing.md 40 · rules/verification-harness.md 3,
  13, 14, 17, 18 · docs/commands.md 33–36 · docs/design-summary.md 28 · docs/obs-summary.md 16, 31, 43 ·
  docs/security-summary.md 17, 27 · docs/tests-summary.md 15, 26 → **re-derived** in step 3 where the leaf derives from
  an amended section (adding the cold-agent sibling); verification-harness.md is scoped to `scripts/agent-run.*` and
  stays agent-run's (its paths do not reach the cold-agent scripts) — no change.

## secretread — 4 rows
- security 260 "secret reads in source are observed absent" → **no change** (true: the pipe and stub read no env var,
  census 0).
- test 138 `no secrets.` (ci.yml pin) → **no change** (true: CI references no secret).
- leaf rules/security.md:8 "Source reads no secret" · docs/security-summary.md:19 → **re-derived** (step 3; still true,
  the credential path recorded beside it).

## noport — 8 rows
- arch 144 "Network ports and listeners: none" → **no change** (true: the client's local socket is the client's, no
  escher code binds — recorded under Process-wide state).
- security 181 "No served API surface exists" → **no change** (true).
- new security 258 · test 115 → this pass's text, true.
- leaf CLAUDE.md:42 · rules/security.md:17 · docs/security-summary.md:7, 36 → **re-derived** (step 3).

## outbound — 16 rows
- arch 145 (Outbound hosts) → **amended** (A4).
- arch 114, 209 · test 64, 309 ("regression") → **no change** (token inside "regression").
- security 181, 185, 200, 201 (outbound HTTP controls) → **no change** (true: they govern blitz-net and the examples;
  the pipe makes no request of its own).
- registries obs-plan bootstrap key file (OTel egress deferral) → **no change** (true, unrelated deferral).
- curation docs/session-learnings.md:12 ("regression") → **no change** (token inside "regression"; preserve-verbatim).
- leaf rules/observability.md:35 · docs/obs-summary.md:15 · docs/security-summary.md:16, 35 · rules/testing.md:21 →
  re-derived in step 3 only where their source section was amended; otherwise no change.

Step 3 leaf set: CLAUDE.md `GENERATED:setup:*` (overview directory tree, warnings) · docs/commands.md · docs/stack.md ·
docs/tests-summary.md · docs/obs-summary.md · docs/security-summary.md · docs/design-summary.md · rules/security.md ·
rules/observability.md · rules/testing.md — enumerated by provenance below at step 3.

## Step 3 — leaves re-derived (provenance: "Distilled from" / "Mirrors" / "From" / "Source:" headers + CLAUDE.md GENERATED blocks)
- CLAUDE.md `overview` (a `scripts/` key-directory line, from arch Directory structure) · `pointer-table` (a cold-agent
  pipe row) · `warnings` (the secrets line gains the dev-host operator login).
- docs/commands.md (§Standard Contracts: a Cold-agent run pipe section; the ci-scripts tests list) · docs/stack.md
  (§Stack CI/CD: the host-only agent client) · docs/tests-summary.md (§3 pipe bullet; 37 → 64) · docs/obs-summary.md
  (§3 log bullet, §9 artifacts, §8 PII) · docs/security-summary.md (CLI input, the pipe's crossings, two data classes) ·
  docs/design-summary.md (cli surface).
- rules/security.md (Secrets, Surfaces) · rules/observability.md (the pipe's events) · rules/testing.md (running the
  pipe) · rules/verification-harness.md (§3 source: a cold-agent section + its path globs).
- No change: docs/gotchas.md, docs/conventions.md, docs/services/*, rules/a11y.md, docs/a11y-summary.md — none derives
  from an amended section. Session Additions / USER blocks untouched.
