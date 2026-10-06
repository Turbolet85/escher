# Fan-out results — 2026-10-06-upstream-sync-observation-model

Report: `escher-0.1.0/chunks/2026-10-06-upstream-sync-observation-model/report.md` · 7 Explore doc-agents, one parallel batch ·
detectors per prompt arch 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2 = 15 = drift-base `doc:` names (15 single-doc entries).
Keyed-contract lines: test-plan · obs-plan · a11y-plan (rendered); architecture `NOT MIGRATED` and the three plans without keyed contracts — line dropped.
Entity probe: entities=0 on every return (no `&lt;` `&gt;` `&amp;`).

| doc | verdict | stripped |
|---|---|---|
| architecture | `proposals: []` — 0 proposals | 3 comment lines: D-arch-resources / D-arch-decisions pass — Changes list no symbol, crate, schema, dependency or dev-tool; upstream 0 ahead (raw twin `.raw-fanout-architecture.md`) |
| security-plan | `proposals: []` — 0 proposals | 4 comment lines: no new input surface, no identity/token/key touch, no dependency (raw twin `.raw-fanout-security-plan.md`) |
| design-system | `proposals: []` — 0 proposals | 1 comment line: no new UI element, no `tokens` flag to read (raw twin `.raw-fanout-design-system.md`) |
| layout-templates | `proposals: []` — 0 proposals | 5 comment lines: no user-facing surface or region (raw twin `.raw-fanout-layout-templates.md`) |
| test-plan | `proposals: []` — 0 proposals | nothing stripped |
| obs-plan | `proposals: []` — 0 proposals | 4 comment lines: no hot-path op, no telemetry dependency, no logging; keyed contract untouched (raw twin `.raw-fanout-obs-plan.md`) |
| a11y-plan | `proposals: []` — 0 proposals | 3 comment lines: no interactive element, no schema change; keyed contract untouched (raw twin `.raw-fanout-a11y-plan.md`) |

## Validate
No proposal to disposition (0 across 7 docs).
1. Playbook — n/a (no proposal).
2. Cross-contradiction — n/a.
3. Intent-consistency — the report's one deviation ("upstream/main merged" in the working entry vs nothing merged) is justified by the founder's ruling (inputs#I2: an empty sync is a measured no-op, no merge commit); the master record's desc already states it ("0 ahead: measured no-op, no merge"). An empty merge set leaves the intent ("our tests and CI prove our logic survived") met by the gate block — no intent amendment. Scope record: none (gate.py scope clean, 0 recorded).
4. Absence needs evidence — no absence claim proposed.
5. Expected amendments — the plan lists none; the report confirms architecture.md:202 stays true (1 hit, sha = the measured tip) and the test-plan §9 baseline reproduced. Nothing to raise.
6. Disproved claims — the report's bullet reads none. Nothing to dispose.

Result: 0 amendments · 0 escalations · drift = 0.
