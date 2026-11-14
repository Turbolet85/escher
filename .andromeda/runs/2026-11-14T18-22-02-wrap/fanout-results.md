architecture: add 1 · set 3 · retire 0 · none 0 · ops-architecture.md
security-plan: add 1 · set 2 · retire 0 · none 0 · ops-security-plan.md
design-system: add 0 · set 1 · retire 0 · none 0 · ops-design-system.md
layout-templates: no proposer
test-plan: add 0 · set 1 · retire 0 · none 2 · ops-test-plan.md
obs-plan: add 0 · set 1 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: no proposer

Main's own: ops-main-architecture.md (set 2 — expected amendments 2 and 6), ops-main-architecture.2.md (set 1 — expected amendment 6's second cell). A test-plan guard set (expected amendment 3's guard half, the six browser tests) was written and withdrawn before any check: `rec.py rows-check` reads `over — guard: 391 chars, 120 at the most` — the cell holds no nine names, so the half is not carried; the six tests stand named in the report and the code.

ops-architecture.md
decision/file-url-load-rule · reject — check 3: «or as the user's own top-level load» narrows the engine-wide rule to the browser's user loads, while an embedder's top-level load (the stand, rdme) is one too and stays ungated; main's ops-main-architecture.md sets the row to the report's reading (expected amendment 2)
decision/sub-resource-requests-carry-initiator · apply — check 1: rul-ipb3atbm, accurate this-chunk addition (expected amendment 2, fct-ytg535zb); a narrowing, the invariant holds
decision/justified-expect-unwrap · apply — check 6: the report's imprecise-row entry (fct-qb4ltnez) — favicon.rs:76 the one test-module allow with no justification beside it, history.rs:131 justified; rul-ipb3atbm
decision/browser-favicon-file-gate · reject — check 2: the fact's home is security-plan decision/file-urls-only-from-file-documents

ops-security-plan.md
decision/file-urls-only-from-file-documents · apply — check 1: rul-ipb3atbm, accurate this-chunk addition (expected amendment 1); a narrowing, its why unchanged
decision/file-url-only-for-file-documents · apply — check 1: rul-ipb3atbm (expected amendment 1, the threat-model reach of the same control)
decision/file-gate-covers-document-started-loads · reject — check 7: stands as decision/file-urls-only-from-file-documents (security-plan-fit.md n=3) — the set above names the gated document loads, and «or a top-level load» keeps the user's own loads outside the gate

ops-design-system.md
decision/favicon-from-first-icon-link · reject — check 2: the fact's home is security-plan decision/file-urls-only-from-file-documents; the report reads this row unchanged (expected amendment 5, fct-pesxy5mo: which href is read is unchanged)

ops-test-plan.md
decision/file-urls-only-for-file-documents · apply — check 5: expected amendment 3, its rule half (fct-ytg535zb); the guard half not carried — the cell's bound, above

ops-obs-plan.md
decision/no-file-loads-from-non-file-documents · apply — check 5: expected amendment 4 (fct-ytg535zb; the app's refusals silent, no log site)

ops-main-architecture.md
decision/file-url-load-rule · apply — check 5: expected amendment 2 (fct-ytg535zb, fct-pesxy5mo) — a navigation a non-file page starts is no top-level load in the browser app
decision/no-seed-on-carriers · escalate → Q1 — check 1: rul-3ndrhilc, Provisional discharge (escalate) — the founder's own word, relayed verbatim (inputs#I2), expected amendment 6 · resolved: apply (Q1) — relayed verbatim by overseer1, 2026-11-14, inputs#I2: «A: нет, сид только при загрузке (Recommended)» (the operator's «Apply both», 2026-11-14)

ops-main-architecture.2.md
decision/no-seed-on-carriers · escalate → Q1 — check 1: rul-3ndrhilc, Provisional discharge — the why clause restated on the founder's own word (inputs#I2) · resolved: apply (Q1) — relayed verbatim by overseer1, 2026-11-14, inputs#I2: «A: нет, сид только при загрузке (Recommended)» (the operator's «Apply both», 2026-11-14)
