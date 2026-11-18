# Fan-out results — 2026-11-18-capability-statement

architecture: add 0 · set 3 · retire 0 · none 1 · ops-architecture.md
security-plan: add 0 · set 2 · retire 0 · none 0 · ops-security-plan.md
design-system: no proposer
layout-templates: add 1 · set 0 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 0 · set 6 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 1 · set 1 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 1 · set 1 · retire 0 · none 0 · ops-a11y-plan.md

Check 5 (expected amendments): architecture project-intent/decision/capability-statement → ops-architecture.md decision/capability-statement; test-plan capability-statement-pinned · capabilities-doc-pinned-to-schema · capability-check-names-held-to-schema → ops-test-plan.md n=1 · 2 · 3; a11y-plan capability-statement-reach-by-hand → ops-a11y-plan.md n=1; masters citing docs/capabilities.md by line → no row holds a citation (the citation sweep: `masters: rows`). Every entry matched.
Check 6 (disproved claims): the plan's «first reads 46» premise and research's «under-reads» note stand in the plan and research alone (WIT: no master site, the pre-pass refusal) — routed to curation P3 as the sweep hazard the report records.
Citation dispositions: 1 row, curation home — P3; no `claim false`.

ops-architecture.md
decision/capability-statement-matches-schema · reject — check 2: the fact's home is architecture decision/capability-statement (the leg counts held to the CI leg runner, set there); this row is the schema's half and stays true
decision/capability-statement · apply — check 1: rul-ipb3atbm, an accurate this-chunk addition — the plan's expected amendment, the report's Harness / gate surface: two leg counts now pinned
decision/cargo-deny-advisory-gate · reject — check 2: the fact's home is security-plan decision/advisory-ignores-by-id

ops-security-plan.md
decision/advisory-ignores-by-id · apply — check 1: rul-ipb3atbm — the report's Schema / config: each ignore reason now names its owner by residual row id, no product line names the residuals file
decision/capability-statement-mcp-crossing · reject — check 2: `caused`'s crossing has its home in security-plan decision/caused-text-crossing, and the limits clause is architecture decision/capability-statement's; the row stands true as it reads

ops-layout-templates.md
decision/randomness-seed-in-process-only · reject — check 2: the fact's home is architecture decision/no-seed-on-carriers (and security-plan decision/seeded-source-in-process-seed-only)

ops-test-plan.md
decision/capability-statement-pinned · apply — check 1: rul-ipb3atbm — the plan's expected amendment, the report's Harness / gate surface
decision/capabilities-doc-pinned-to-schema · apply — check 1: rul-ipb3atbm — the plan's expected amendment; its rule held every count to the schema's tables, false for the two leg counts
decision/capability-check-names-held-to-schema · apply — check 1: rul-ipb3atbm — the plan's expected amendment; «only … the schema's tables» false since the pin reads ci-leg.sh
decision/capability-statement-from-schema · apply — check 1: rul-ipb3atbm — a correction: the statement now holds counts not read from the schema (leg counts, witness counts), the shorter rule keeps it true
decision/requirement-witness-inside-stand-repeat · reject — check 7: not_master type 5 — the added clause lengthens a row still true; the page and the sidecar hold the hand-stated witness counts
decision/mutation-controlled-tests · reject — check 1: the control tool's case grammar is the pipeline's own instrument, not escher's convention — routed to curation P3 (the report's Instrument limit)

ops-obs-plan.md
decision/scrub-by-field-name · reject — check 2: `caused` keeping an app's text as written has its home in security-plan decision/caused-text-crossing; the row stands true as it reads
decision/seed-in-process-only · reject — check 2: the fact's home is architecture decision/no-seed-on-carriers

ops-a11y-plan.md
decision/capability-statement-reach-by-hand · apply — check 1: rul-ipb3atbm — the plan's expected amendment; its why «reads none of the section's claims» is false since the pin reads the leg's file count
decision/leg-counts-stated-in-capability-statement · reject — check 2: the fact's home is test-plan decision/capability-statement-pinned
