# Cascade dispositions — 2026-10-06-stand-test-contract

**The search.** `cascade.py sweep` over `cascade-patterns.toml` (baseline `913f5918`, the pre-CI parent), nine
patterns, each derived from a claim this pass retired or a token it introduced:
- `5cmd` `5-command` · `cmdlist` `boot, status, cleanup or logs|boot / run / status / cleanup / logs` · `statusendp`
  `status endpoint` · `pidfile` `PID file` · `bootstrap` `test-data bootstrap` — test-plan §3's retired
  "no boot/status/cleanup/logs command … status endpoint shape, PID file … was gathered" marker, by its nouns and its
  command list;
- `jsonschema` `JSON log schema|log JSON schema|JSON-line log` · `logfileloc` `log file location|log-file sink` —
  obs-plan §3/§6's markers ("a JSON log schema, log file location" · "a log JSON schema, a log-file sink");
- `ci23` `runs 23 tests|23 tests` — test-plan §4's retired CI-scripts count;
- `agentrun` `agent-run` — every site naming the new contract or claiming it absent.
Dropped: `notrendered` (`not rendered|scripts are generated`) — its control never fired over the masters (the phrase
lives only in a leaf); controlled by hand: the leaf `.claude/rules/verification-harness.md:14` carries it and is
re-derived below. Every remaining pattern's control fired (test-plan.md:100 · :114 · obs-plan.md:83 · a11y-plan.md:57).

**Not looked for:** the regression test's retired focus-order wording — it lived in a test doc, not a master
(`grep -F 'focus order'` over the seven masters: 0 hits at report time).

## Rows

Masters (`new` = this pass's own text):
- test-plan.md:100 `5cmd` (edited) — the new block's heading "the 5-command discipline" — amended, true.
- test-plan.md:102 `statusendp` / `pidfile` / `agentrun` — the new Invocation bullet stating there is NO PID file or
  status endpoint — amended, true.
- test-plan.md:103-110, :128 `agentrun` — the new contract bullets and the §4 `test_agent_run.py` bullet — amended.
- test-plan.md:113 `5cmd` / `bootstrap` (edited) — the narrowed marker, test-data bootstrap only — amended, true.
- test-plan.md `ci23` — 0 rows: the retired "runs 23 tests" is gone; the new :127 text reads "runs 37 tests — the 23 of
  the files above and …" (the `23 tests` regex does not match "23 of") — no change.
- obs-plan.md:83 `5cmd` / `agentrun` — the new §3 harness paragraph — amended.
- obs-plan.md:144-147, :285, :312 `agentrun` — the new §6 format block, §8 reach bullet, §9 table row — amended.
- obs-plan.md:149 `jsonschema` / `logfileloc` (edited) — §6's marker, now scoped "for escher's own sink" — amended, true.
- architecture.md:139 `agentrun` ×6 @c2685…3354 (3535-char line) — the new CI-contracts clause — amended (window read
  at the offsets: all six inside the inserted clause).
- architecture.md:147 `agentrun` ×5 @c1444…1878 (2147-char line) — the new Filesystem clause — amended.
- security-plan.md:66 · :362 `agentrun` — the new §Input Validation row and §Logging & Monitoring bullet — amended.
- layout-templates.md:71 `agentrun` — the new §Surface: cli bullet — amended.
- a11y-plan.md:57 `agentrun` (standing) — "agent-runnable invariants", a different word sharing the token — no change.

Judgment bases (`base` — propose→approve channel, never a cascade edit):
- drift-base.md:87-88 `5cmd` / `statusendp` — D-tests-obs-harness's invariant "the 5-command harness / status shape /
  log format stays consistent" and check "harness, status endpoint, or log format" — still true as a detector of a
  measured contract; no rule quotes retired wording — no change, nothing proposed.

Curation homes: 0 rows.

Leaves (re-derived in step 3; a row never stands in for the recompute):
- .claude/rules/verification-harness.md:3, :13, :14 — "The 5-command contract — NOT YET MEASURED … scripts/agent-run.{sh,ps1}
  is not rendered" — stale → re-derived.
- .claude/docs/tests-summary.md:15, :17 — "5-command discipline … NOT YET MEASURED … no `scripts/agent-run.*` exists yet";
  "Status endpoint · PID file · JSON-line logs · tempdir: NOT YET MEASURED" — stale → re-derived.
- .claude/docs/obs-summary.md:16 — "JSON log schema · log sink path · rotation · heartbeat · status endpoint: NOT YET
  MEASURED" — partially stale (the harness log) → re-derived.

Step-3 leaf set (the DAG table + provenance): architecture → CLAUDE.md `GENERATED:setup:*`, docs/commands.md
(§Standard Contracts), docs/workflow.md, docs/gotchas.md, docs/conventions.md (provenance; §Conventions not amended);
test-plan → docs/tests-summary.md, rules/testing.md, rules/verification-harness.md, CLAUDE.md warnings; obs-plan →
docs/obs-summary.md, rules/observability.md; security-plan → docs/security-summary.md, rules/security.md;
layout-templates → docs/design-summary.md.
