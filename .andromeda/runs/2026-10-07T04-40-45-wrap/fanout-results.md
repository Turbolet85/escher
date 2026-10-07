# Fan-out results — 2026-10-07-upstream-sync-driver-core (wrap 2026-10-07T04-40-45)

Report read by every detector: `escher-0.1.0/chunks/2026-10-07-upstream-sync-driver-core/report.md`.
Batch: 7 Explore doc-agents, one per spec source; 15 detectors over 15 `doc:` names in `drift-base.md` (arch 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2). Keyed-contract renders sent to test-plan, obs-plan and a11y-plan (1 key each); architecture reads `NOT MIGRATED` (line dropped).
Probe on every return: entity decode a no-op, `entities=0`; each parses to `proposals: []` once its trailing comment lines are stripped. A raw twin is kept for each (`.raw-fanout-{doc}.md`) because stripping changed the return.

## Verdicts
- **architecture** — 0 proposals. Stripped: 5 comment lines — both detectors hold (no new resource, dependency or pattern); architecture.md:202's `Upstream sync:` sha equals the measured tip and is the doc's only hit for it; the doc carries none of the test counts.
- **security-plan** — 0 proposals. Stripped: 4 comment lines — no new input surface, no auth/secret change, no dependency; the CI reads are read-only measurements; "Dependency audit=success" is consistent with §Dependency Security as written.
- **design-system** — 0 proposals. Stripped: 5 comment lines — no new UI element, so no `tokens` flag to read.
- **layout-templates** — 0 proposals. Stripped: 3 comment lines — no user-facing surface or region added.
- **test-plan** — 0 proposals. Stripped: 5 comment lines — no new code path; runners are the §9 gate and doc leg; harness untouched on both sides of the §3 bind; test-plan.md:315's 131 · 548 · 0 · 5 and test-plan.md:139's 64 reproduced, not moved.
- **obs-plan** — 0 proposals. Stripped: 24 comment lines — no new operation, logger or log line; evidence/sync.md is a record, not a log sink; the keyed contract unchanged.
- **a11y-plan** — 0 proposals. Stripped: 6 comment lines — no interactive element, no schema change; the a11y leg's three files and the CI job name match the doc; the PROVISIONAL items untouched.

## Validate (orchestrator)
No proposal to disposition. The six checks over the empty set:
1. Playbook — nothing to match.
2. Cross-contradiction — nothing to pair.
3. Intent-consistency — the report matches every plan acceptance criterion. One divergence from the working entry's wording ("upstream/main merged"): nothing was merged because upstream is 0 commits ahead. Justified — the plan the operator reviewed at phase P5 states the measured no-op as its goal and its acceptance, and the master record's desc already reads "measured no-op, no merge". No scope-record line (scope: clean, 0 recorded). No escalation.
4. Absence needs evidence — no absence or caught-ALL claim is applied. The report's two "stays true" readings carry their searches: `grep -n 'Upstream sync:' .andromeda/architecture.md` 1 hit (line 202, read whole); the `131 result lines` / `548 passed` search of test-plan.md 1 hit (line 315, a 6 459-char line, read by offset: match at c4783, window c4663–c4961).
5. Expected amendments — the plan's list reads "none"; nothing to raise.
6. Disproved claims — the report's bullet reads "none"; nothing to dispose.

## Result
0 amendments applied · 0 rejected · 0 escalations. No spec body, key file or sidecar edited; no cascade sweep run (no amended passage to derive a pattern from) and no leaf re-derived.
