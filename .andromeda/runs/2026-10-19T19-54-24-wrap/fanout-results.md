# Fan-out results — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

Batch: snapshot before `tree 118c7bc4a63597d756b5d7927738f4c2c2e29170 index 21749b427f779c5046e2de267805224e40e823e2 head 79603d1ef633cde4dfc841e3bb4da23a39b86115`;
after `tree 01de87637d4071b427701a7dc0ddb920c745c4c4`, index and head unchanged; `git diff-tree` printed exactly the seven
`A …/fanout-{doc}.md` lines. No file needed a fix (no twin); every receipt's count equals its file's.

architecture: 7 proposals · fanout-architecture.md
  note: no new port, socket, env var, crate, dependency or log site; every behaviour fact PROVISIONAL (inputs#I2, card row 5); citation re-points are the sweep's.
  1. apply — check 1 (Accurate this-chunk addition; plan Expected amendment 2) · §Established Decisions → [Unsupported features] (:85), `:focus-within` out of the always-false list, its matching stated, PROVISIONAL. Also settles citation-dispositions row arch:85 (claim false).
  2. reject — check 1 (Registry over-reach: a public API symbol; architecture enumerates no `Node` method — `is_hovered` stands nowhere in it)
  3. apply — check 1 (Accurate this-chunk addition; plan Expected amendment 3) · §Established Decisions → [DOM semantics] (:98), the not-in-document refusal, the `FOCUS_WITHIN` chain moves and the focus redraw, PROVISIONAL; text re-derived from the report (the proposal's changed-set cite to `focus_within.rs` dropped — the report's basis is the `changed_set_…` test)
  4. apply — check 1 (Accurate this-chunk addition; report Counts › A focus set or cleared requests a redraw) · §Cross-cutting Patterns → Service wiring (:268), dependent of 3
  5. apply — check 1 (plan Expected amendment 1) · §Project Intent `Upstream sync:` (:278), PROVISIONAL
  6. apply — check 1 (report Counts › The CARRY owner; P5 card row 3, the operator's own word) · §Established Decisions (:90), CARRY owner → «Upstream sync ahead of polish and ship»
  7. apply — dependent of 6 · §Established Decisions (:91), the clock's CARRY owner, same move (card row 4)

security-plan: 1 proposal · fanout-security-plan.md
  note: D-security-input and D-security-auth hold; Retired none.
  1. apply — check 1 (plan Expected amendment 7) · §Dependency Security (Pinning), a sibling of :250 for this sync, PROVISIONAL

design-system: 0 proposals · fanout-design-system.md
  note: D-design-tokens holds — no new UI, tokens n/a.

layout-templates: 0 proposals · fanout-layout-templates.md
  note: D-layout-surface holds — no surface or region added.

test-plan: 5 proposals · fanout-test-plan.md
  note: D-tests-framework and D-tests-obs-harness hold; proposal 5 names a site the report's sweep said did not exist.
  1. apply — check 1 (plan Expected amendment 5) · §1 Test Scope Summary, the blitz-tests bullet gains `focus_within` 7 and `focus_redraw` 4
  2. apply — check 1 (plan Expected amendment 5) · §2 Directory pattern (:68), 138 `.rs` files / 137 targets
  3. apply — check 1 (plan Expected amendment 5) · §9 Per-OS test tallies (:330), re-read on CI#39312031677 with the two 2026-10-19 chunks attributed
  4. apply — check 1 (Accurate this-chunk addition; report Counts › The workspace tally) · §9 local workspace baseline (:336), 199 · 1045
  5. apply — check 1 (Accurate this-chunk addition) + check 4 · §3 Test Harness Contract → Proof (:118), `run stand` 178 · 0 · 7 over 44 files and `run all` 586 · 0 · 12. The report's "No master site states it" (Counts › The `run all` agent-run figure) is disproved: the series stands at test-plan.md:118, last re-counted at 2026-10-17 (546; `run stand` 168 over 40). Its pattern `\b575\b` could not meet a site that never held 575 (the report's own Sweep hazard). Read at :118's tail by offset.

obs-plan: 1 proposal · fanout-obs-plan.md
  note: D-obs-stack and D-obs-pii hold; obs-plan.md:197's cite already reads `:1805-1806` (the sweep's).
  1. apply — check 1 (plan Expected amendment 6) · §3 Logging stack, a new bullet inside the `absence · logs` group (an open set appends inside its group)

a11y-plan: 1 proposal · fanout-a11y-plan.md
  note: D-a11y-surface and D-a11y-obs-schema hold; :154 is silent on `FOCUS_WITHIN`, not contradicted, and the report homes the focus facts in architecture [DOM semantics].
  1. apply — check 1 (plan Expected amendment 4) · §5 Keyboard Navigation, Focus order (:155), PROVISIONAL. Also settles citation-dispositions row a11y:155 (claim false).

## Checks over the set
- 2 cross-contradiction: none — no two proposals edit one section in opposing directions.
- 3 intent-consistency: the scope record is empty (`gate.py scope` clean, 0 recorded), and every applied proposal lies inside the entry's intent and the plan's acceptance criteria. Nothing escalates.
- 5 expected-amendments reconciliation: 1 → arch 5 · 2 → arch 1 · 3 → arch 3 (+ arch 4) · 4 → a11y 1 · 5 → test 1, 2, 3 · 6 → obs 1 · 7 → security 1 · 8 → the citation sweep (233 re-pointed; obs-plan.md:197 reads `document.rs:1805-1806` and architecture.md:268 `keyboard.rs:58`). Every entry is matched. `citation-dispositions.md`'s two `claim false` rows → arch 1 and a11y 1.
- 6 disproved claims: (a) the `mutator.rs:731-733` cite in research.md:11 / plan.md:421 is a chunk-folder fact with no master, ledger or scope site. It stays recorded in `evidence/take.md` and `evidence/readings.md` (DISPOSED, no channel owes a write). (b) research.md §Open questions is answered by measurement with no edit (DISPOSED, closed).
- The playbook's «An engine behaviour change for every document» rule governs the take: a fork answered by a delegate → recorded PROVISIONAL, discharged by the Provisional discharge rule at the Epoch 4 boundary (card row 5). The take is not a boundary widening: nothing new crosses a carrier, so no escalation.
- Escalations: 0. Rejected for a source the report does not carry: 0. Rejected by the playbook: 1 (arch 2).
