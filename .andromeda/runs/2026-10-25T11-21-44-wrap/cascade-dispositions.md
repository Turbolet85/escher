# Cascade dispositions — 2026-10-25-capability-statement (wrap P2, cascade step 2)

The search: `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.16), baseline e2eb01b4 (the parent of
the oldest pre-CI commit e7e71221), fired after the pass's last amendment (the eight replacements of `replace-1.md`).
Eight patterns, each over the seven masters' bodies and sets, the registry files, the curation homes, the judgment bases
and the leaves — the amended claims' old wording AND their mechanism:
- `causes9` — the cause set stated as nine; `exec7` — the executor's share stated as seven (the mechanism: «by the executor»)
- `atowner` — an AT-witness / adapter owner phrased «(target: next)» (within 300 chars, no `;`), and the new `re-carried:0.2.0`
- `readme12` — README's MCP count stated eleven; the Front page's «no build, doc or test step»
- `ciscripts` — the ci-scripts leg at 100 tests / seven files
- `carried` — the socket's and the MCP server's carried lists in their old words
- `carriedset` — every «ids, (accessible) names, (control) values» carried-set list (the mechanism, worded otherwise)
- `docstree` — the Directory structure fence rows around the new `docs/` row
Not looked for: a new fact's own name tokens (`capabilities.md`, `test_capability_statement`, `CapabilityStatementTest`) —
their controls cannot fire on the pre-pass text (P1's `docsdir` control unfired), so the new text's sites are the
apply's own, listed in `fanout-results.md`.

total (8 patterns) · 63 rows over 28 files
per class · new 13/8 · standing 29/15 · leaf 21/9 · curation 0/0 · base 0/0

## Rows
causes9 · 0 rows (control fired) — no master states nine causes after the pass
exec7 · 0 rows (control fired) — no master states the executor's share as seven after the pass
readme12 · 0 rows (control fired) — no master states eleven tools or «no build, doc or test step» after the pass

atowner:
- standard-contracts.md:27 · input-validation.md:194 · input-validation.md:217 · test-plan.md:48 · test-scope-summary.md:98 · a11y-plan.md:9 · :27 · :35 · a11y-scope-summary.md:17 · a11y-strategy.md:9 · :21 — new → amended (this pass)
- a11y-strategy.md:20 — standing → no change: the native checked-state fix's owner «(target: next)», another residual, true as stated; «adapter» within the window only
- .claude/rules/a11y.md:25 · :27 · .claude/rules/security.md:17 · .claude/docs/a11y-summary.md:21 · :26 · .claude/docs/services/blitz-shell.md:18 — leaf → step 3
ciscripts:
- unit-test-strategy.md:30 — standing edited → no change: the dated 2026-10-15 reading («`Ran 100 tests`»), now the series' second entry beside the newest at :29
carried:
- established-decisions.md:162 — standing → no change: its continuation (@6777) names a lint's findings on those carriers and at rest
- security-plan.md:25 · :27 · :65 — amended (:25 the local-IPC vector, :27 the local-stdio vector, :65 the bounded replies)
- threat-model-summary.md:26 — standing edited → amended: the socket row's lint clause on its continuation line
- .claude/rules/security.md:21 — leaf → step 3
carriedset:
- established-decisions.md:162 · occupied-resources.md:8 · :9 — standing → no change: each block's continuation names a lint's findings (@6777 · @4113 · @2269)
- established-decisions.md:172 · security-plan.md:25 · :27 · :65 · data-protection.md:8 · input-validation.md:38 · :65 · :70 · threat-model-summary.md:26 · :35 · obs-plan-detail/ci-integration.md:16 · registries/contracts/test-plan/session-lifecycle.md:7 — amended: each carried set, live or at rest, now names a lint's findings (the visual lint's crossing onto every ratified carrier, input-validation.md:54)
- occupied-resources.md:32 · data-protection.md:14 · test-data-fixtures.md:14 · obs-plan-detail/ci-integration.md:18 — standing → no change: the two committed Counter checks, which hold no `lint` step (`grep -c '"lint"'` 0 in each)
- test-plan.md:263 · integration-test-strategy.md:16 — standing → no change: a log line holds none of them, a negative claim
- obs-plan.md:262 — standing → no change: the 2026-10-15 cold-agent transcript's carriage, a run before the `lint` verb
- CLAUDE.md:37 · :44 ×2 · :110 · .claude/rules/observability.md:32 · .claude/rules/security.md:21 ×2 · :22 · :23 · .claude/docs/obs-summary.md:32 ×2 · :43 · .claude/docs/security-summary.md:20 · :32 · .claude/docs/services/escher-driver.md:6 · :32 · :48 — leaf → step 3
docstree:
- architecture.md:241 · :243 — standing → no change: the fence rows beside the new `docs/` row (its control)

Every amended line re-read for an intra-line duplicate of the retired wording: none (the `edited` rows above are the amended lines, their swept text the kept part).

## Leaves
`cascade.py leaves` (pass mode; `--add` the step-2 leaf rows' nine leaves): 11 groups · 24 leaves · 513893 B · unmapped 1. One comparer per group (batch `leaf`, `VERDICT: 11 of 11 pass`); `leaves --apply` total · blocks 28 · applied 27 · held 1.
- a11y-plan · pass · findings 6 · applied 6 (a11y-summary.md:21, :26 and rules/a11y.md:25, :27 the AT owner, stale; a11y-summary.md:8 and rules/a11y.md:19 the published reach and its by-hand sync, omission) · by hand 0 · declined 0
- architecture-1 · pass · findings 1 · applied 1 (services/blitz-shell.md:18 the AT owner) · by hand 0 · declined 0
- architecture-2 · pass · findings 0
- architecture+obs-plan · pass · findings 1 · applied 1 (services/escher-telemetry.md:31 the statement restating the scrub's limits) · by hand 0 · declined 0
- architecture+security-plan+test-plan-1 · pass · findings 1 · applied 1 (services/escher-driver.md:35 the pin reading `VERBS` and the schema's tables) · by hand 0 · declined 0
- architecture+security-plan+test-plan+obs-plan+a11y-plan · pass · findings 3 · applied 3 (CLAUDE.md:110 a lint's findings at rest; :88 and :80 the pin and the statement in the pointer table) · by hand 0 · declined 0
- architecture+test-plan · pass · findings 1 · applied 1 (commands.md:31 the pin among the CI-script tests) · by hand 0 · declined 0
- obs-plan · pass · findings 4 · applied 4 (obs-summary.md:32, :43 a lint's findings at rest; :49 and rules/observability.md:24 the statement's restatement and its by-hand sync) · by hand 0 · declined 0
- security-plan · pass · findings 7 · applied 7 (security-summary.md:20, :22, :32 and rules/security.md:21 ×2, :22 a lint's findings in the carried sets; rules/security.md:17 the AT owner) · by hand 0 · declined 0
- test-plan-1 · pass · findings 2 · applied 2 (tests-summary.md:29 the 104-in-eight tally, stale; the pin, omission) · by hand 0 · declined 0
- test-plan-2 · pass · findings 2 · applied 1 (rules/testing.md:21 the claim-to-check section and its by-hand sync) · by hand 0 · declined 1 (rules/testing.md:17 — the pin's own test design is detail its master holds, test-plan §4 and §7; no reader of the testing rule acts against a source without it)
- unmapped .claude/rules/host-linux.md (host) — the host's own statements, no source states them: no comparer, no step-3 edit
