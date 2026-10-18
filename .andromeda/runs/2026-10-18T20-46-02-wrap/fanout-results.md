# Fan-out results — 2026-10-18-engine-debts (gated wrap, resumed)

Snapshot pair: before `tree 0bc53937949770f08c67349595e1cd7d531c7525 index 702dad9344f73a6215006a43604d20dbc37dfd0e head 1c9bc58b5ea2332dcc2013d2407f0fe7374c798c` · after `tree 918fb3bff74436198484f6e05683528398307887 index 702dad93… head 1c9bc58b…` — `git diff-tree -r --name-status` printed exactly the seven `A {run_dir}/fanout-{doc}.md` lines; index and head unchanged. No file needed a fix (each opens `proposals: []`, one `note:` line, no entity), so no twin.

- architecture: 0 proposals · fanout-architecture.md — note: the detectors do not fire; the PROVISIONAL discharge at architecture 86 ×4 · 90 · 98 · 102 · 143 · 145 ×2 · 149 · 164 · 178 ×2 (ED) · 91 · 139 · 169 · 173 · 175 ×5 · 180 (VC) · 248 (CF) is P2's Provisional-discharge escalation.
  - 1 (main-raised, check 5 — the plan's §Established Decisions entry; Provisional discharge) · §Established Decisions, §Standard Contracts (Document core · Mutation, query and CSSOM · Scrolling, selection, tree · Driver session) · the engine-debts class: 14 marks → `ratified by the founder (2026-10-18)` — escalate (playbook:48) → resolved: the operator's word inputs#I11 (b), on the founder's own word inputs#I9 item 1 «Утвердить всё (Рекомендую)» → apply
  - 2 (main-raised; Provisional discharge) · §Established Decisions, §Standard Contracts (Document core · harness · Driver session · CLIs) · the one-virtual-clock class: 10 marks → ratified — escalate (playbook:48) → resolved: inputs#I11 (b), on inputs#I9 item 3 «Утвердить всё (Рекомендую)» → apply
  - 3 (main-raised; Provisional discharge) · §Upstream sync (line 248) · the cursor-fix class: 1 mark → ratified — escalate (playbook:48) → resolved: inputs#I11 (b), on inputs#I9 item 2 «Утвердить (Рекомендую)» → apply
- security-plan: 0 proposals · fanout-security-plan.md — note: no detector fires; the marks at 119 · 127 · 128 · 130 · 131 (ED) · 81 ×2 · 217 · 357 · 403 (VC) · 246 (CF) are the discharge's.
  - 1 (main-raised; Provisional discharge) · §Input Validation (markup attributes, accessibility reads, `inert`, keyboard activation, password caret) · ED: 5 marks → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 1 → apply
  - 2 (main-raised; Provisional discharge) · §Input Validation (driver causes, `advance`) · §Surfaces (screenshot fade) · §Refusals · §Logging · VC: 5 marks → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 3 → apply
  - 3 (main-raised; Provisional discharge) · §Dependency Security (line 246) · CF: 1 mark → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 2 → apply
- design-system: 0 proposals · fanout-design-system.md — note: D-design-tokens has nothing to read; 216 (ED) · 291 (VC) · 316 (CF) are the discharge's.
  - 1 (main-raised; Provisional discharge) · line 216 (the hit at a box paint clips) · ED: 1 mark → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 1 → apply
  - 2 (main-raised; Provisional discharge) · §Motion, line 291 · VC: 1 mark → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 3 → apply
  - 3 (main-raised; Provisional discharge) · line 316 (the cursor) · CF: 1 mark → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 2 → apply
- layout-templates: 0 proposals · fanout-layout-templates.md — note: no surface added, no `prov` site.
- test-plan: 0 proposals · fanout-test-plan.md — note: no detector fires; 19 · 93 (ED) · 10 · 22 · 27 ×3 · 88 · 89 · 144 · 183 · 287 · 327 (VC) and the key file `contracts/test-plan/session-lifecycle.md` 5 · 11 (VC) are the discharge's; 25 · 328 · 334 true, no change.
  - 1 (main-raised; Provisional discharge) · §3 / §8 rows at 19 · 93 · ED: 2 marks → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 1 → apply
  - 2 (main-raised; Provisional discharge) · rows at 10 · 22 · 27 ×3 · 88 · 89 · 144 · 183 · 287 · 327 · VC: 11 marks → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 3 → apply
  - 3 (main-raised; Provisional discharge) · §3 → session-lifecycle (key file lines 5 · 11) · VC: 2 marks → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 3 → apply
- obs-plan: 0 proposals · fanout-obs-plan.md — note: 114 ×2 · 115 · 188 (VC) the discharge's; 74 a citation, no change.
  - 1 (main-raised; Provisional discharge) · lines 114 ×2 · 115 · 188 · VC: 4 marks → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 3 → apply
- a11y-plan: 0 proposals · fanout-a11y-plan.md — note: no detector fires; 46 · 105 · 147 · 151 · 161 · 170 · 190 · 200 · 268 · 285 ×2 · 287 (ED) · 162 (VC) the discharge's; 46's residual ownership holds (residuals.md:33 open, target next).
  - 1 (main-raised; Provisional discharge) · ED: 12 marks → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 1 → apply
  - 2 (main-raised; Provisional discharge) · line 162 (the double click) · VC: 1 mark → ratified — escalate → resolved: inputs#I11 (b) on inputs#I9 item 3 → apply

Validate:
- check 1 (playbook): every raised item matches «Provisional discharge» (playbook:48, escalate). The rule's own condition holds — the founder's own word, relayed with his chosen labels verbatim (inputs#I9) — and the operator named all three batches at this wrap (inputs#I11 (b), the answer to the card question report-gated.md's Decisions & corrections left open). Resolved, no further halt. None is a boundary widening: the ratification moves no crossing.
- check 2 (cross-contradiction): none — one edit form, one direction.
- check 3 (intent): the report diverges from no acceptance; the scope read's 21 unrecorded files are answered by the operator's ruling (inputs#I10), re-read unchanged at this resume (addendum).
- check 4 (absence): the site list is the P1 pre-pass sweep's `prov` rows (57 rows / 71 marks), re-counted per file at this P2 by a read-only count (architecture 25 · security-plan 11 · design-system 3 · test-plan 13 · obs-plan 4 · a11y-plan 13 · session-lifecycle.md 2 = 71); the cascade's step-2 sweep re-checks it after apply.
- check 5 (expected amendments): the plan's one remainder (architecture §Established Decisions, «each PROVISIONAL on `delegate operator` … under «Provisional discharge»») → architecture 1.
- check 6 (disproved claims): none in the report.
- Rejected for a source the report does not carry: 0.
