# Fan-out results — 2026-10-19-visual-lint (resumed 2026-10-20)

Batch: one parallel batch of seven doc-agents, spawned at this resume (no `fanout-{doc}.md` stood in the run dir).
Path check: before `tree f62f78c8f3ffe0495dfe7329602adda1a7e267a5 index 9b1258e54bdbc3b970cbcd8cb5809d355c1ac1e8 head
5a9b806de22440a7fd193baf6f506e6074c979a7`; after `tree 30ff6458eddcf099ac54d3a78f186909fa54016d`, index and head
unchanged; `git diff-tree` printed exactly the seven `A …/fanout-{doc}.md` lines. Every file split at its `- detector:`
items, every detector one of its doc's scoped ones; entity probe 0 in all seven; no fix, so no twin. Every receipt's
count equals its file's.

architecture: 6 proposals · fanout-architecture.md
  note: D-arch-decisions holds; architecture.md:360's reader counts left for the count cascade.
security-plan: 1 proposal · fanout-security-plan.md
  note: D-security-auth and D-security-deps read no drift; `text_extent` is no external-input surface.
design-system: 0 proposals · fanout-design-system.md
  note: D-design-tokens unfired — `tokens n/a` on all three new surfaces; `lint_page`'s colours are planted test data.
layout-templates: 0 proposals · fanout-layout-templates.md
  note: D-layout-surface unfired; the plan's IA-notes amendment is outside its invariant (raised below).
test-plan: 16 proposals · fanout-test-plan.md
  note: D-tests-framework and D-tests-obs-harness hold; test-plan.md:68's session_common count found by reading.
obs-plan: 2 proposals · fanout-obs-plan.md
  note: D-obs-stack and D-obs-pii read no drift; obs-plan.md:70 and :79 reader counts flagged for the orchestrator.
a11y-plan: 0 proposals · fanout-a11y-plan.md
  note: neither detector fires; the plan's §6 amendment is outside them (raised below).

## Dispositions

Re-derivation tell (§Validate opening rule): every `basis` and coordinate is the report's own (Symbols / APIs ranges,
the New-text listing's rows — lint.rs 176-187, 190-230, 526-604; text_extent.rs 5-15, 18-54; lib.rs 69-70 · 111,
82-87 · 130 · 148; stand_lint.rs 21-103, 135-186, 188-205, 207-243, 245-280, 282-304; text_extent.rs 34-56, 58-83,
85-110) or the proposal's own doc. Rejected for a source the report does not carry: 0.

architecture
1. escalate — check 1: the plan's reviewed entry names «Boundary widening» (escalate) for §Standard Contracts → Driver
   session and directs the wrap to escalate it in full; F1/F2 are the operator's answers as delegate (inputs#I2,
   inputs#I3 row 1). Applied text re-derived from the report (`51 → 55` re-export count is the detector's, re-read at
   apply).
2. escalate — dependent of 1 (`fonts-unavailable` also from `Session::lint`); validates with its primary.
3. escalate — dependent of 1 (the record of ids fed by the lint's snapshot); validates with its primary.
4. apply — check 1: «Accurate this-chunk addition» (routine); the plan's reviewed entry names the change itself
   (§Standard Contracts → Scrolling, selection, tree: `BaseDocument::text_extent`, read-only, its limit). A read-only
   reader moves no engine behaviour, so no «engine behaviour change for every document» question.
5. escalate — dependent of 1 (the escher-driver scope row's private module `lint`); validates with its primary.
6. apply — dependent of 4 (the blitz-dom scope row's module list gains `text_extent`); «Accurate this-chunk addition».
7. (raised, check 5/4) apply — §Infrastructure Patterns, architecture.md:360: the shared modules' reader counts —
   `session_common` by 25 → 26 and `fixtures` by 19 (18 `stand_*` files and `font_face_sniffed_source`) → 20 (19
   `stand_*`); the report's Counts list both rows as changed (patterns `session-readers` @c157, `fixtures-readers`
   @c286) and no detector proposed them. Routine (report substantiates).

security-plan
1. apply — check 1: «Accurate this-chunk addition»; the detector reports its invariant holding (no external-input
   surface added) and the plan's reviewed entry names the findings half (ids, closed words and numbers only, crossing
   nothing) — the playbook rule for a detector-graded `escalate` with its invariant holding. Rides the lint question
   below for its PROVISIONAL status (F1), since it records the in-process lint.
2. (raised, check 5) apply — §Input Validation, a `Layout values` row: `BaseDocument::text_extent`, a read-only reader
   on every document — a stale or non-inline-root id reads `None` through `get_node`, never indexing; it mutates
   nothing; no log site, print, env read or panic path. The plan's entry names it; no detector proposed it. Routine.

test-plan
1–16. apply — check 1: «Accurate this-chunk addition» (routine); each is a count, a coverage line or a reading the
   report's Counts / Coverage / Outcome carry, at a test-plan row the report's patterns name (`stand-count`,
   `stand-files`, `bt-files`, `ws-tally`, `ci-tally`, `fixtures-readers`, `session-readers`, `cov-driver`); 10 (the
   :68 `session_common` count) is a restatement of the report's 25 → 26 fact the report's sweep left as a path citation
   — check 4 read the site at test-plan.md:68 and it states the count; dependents validate with their primaries.

obs-plan
1. apply — check 1: «Accurate this-chunk addition»; the plan's entry names it (§3 the four new files' silence).
   Rides the lint question below for F2's PROVISIONAL status.
2. apply — check 1: «Accurate this-chunk addition»; the plan's entry names it (§4 no span for the in-process lint).
   Rides the lint question below for F2's PROVISIONAL status.
3. (raised, check 4) apply — §3 Logging stack, obs-plan.md:70 @c6057: «the module twenty-five checks read» → twenty-six,
   `stand_lint` joining its list; the report's sweep left :70 as a path citation, the window reads the count. Routine.
4. (raised, check 4) apply — §3 absence · logs, obs-plan.md:79: «its 19 readers» → 20 (`stand_lint` the twentieth);
   the report's sweep left :79 no-change («the module's silence, true»), the line also states the count. Routine.

a11y-plan
1. (raised, check 5) apply — §3 Contrast verification harness (a11y-plan.md:80) and §6 Color contrast
   (a11y-plan.md:233): `Harness::text_contrast` gains its first non-test caller, the visual lint's `contrast`
   reading, under the same exempt rule (`enabled == Some(false)` exempt, `None` not measured, reported below
   `required_ratio()`), the stand at boot reading no contrast finding; the report's `text-contrast` pattern lists both
   rows changed. Routine.

layout-templates
1. (raised, check 5) apply — §Surface: desktop-native → IA notes (layout-templates.md:45): the stand at boot reads 0
   lint findings on each lean task in both layout modes; after twelve Creates the last CRUD row reads `clipped` inside
   `crud-list`, agreeing with the driver's `off-screen` refusal of its `click`. The plan's entry names it. Routine.

Check 2 (cross-contradiction): none — no two proposals edit one section in opposing directions.
Check 3 (intent-consistency): Deviations 1–2 justified by the plan's own guard (entry 7); Deviation 3 ruled by the
operator (inputs#I5, the addendum); Deviations 4–9 inside the plan's steps; 10 plan-sanctioned (gated arm). Scope
record: none (`scope: clean`).
Check 6 (disproved claims): the plan's `BoundingRect` / `Contain` premise — no master states it (report: `text-extent`
control unfired; `driver-session` rows describe no lint type) — DISPOSED: the plan's own text, carried in the report's
Deviations 1–2; no master amendment owed.
Citation dispositions: 0 `claim false` rows.

Escalation: architecture 1 · 2 · 3 · 5, with security-plan 1 and obs-plan 1 · 2 riding the same F1/F2 status — one
question to the operator (the founder away; the operator's answer is the delegate's, PROVISIONAL, listed for the
founder's Epoch 4 batch — inputs#I5).

## Resolution and apply
- Escalation resolved: the operator's answer «Record as built (Recommended)» by question dialog, 2026-10-20T01:08Z — the
  delegate's (the founder away, inputs#I5), PROVISIONAL for the founder's Epoch 4 batch; recorded verbatim in
  `escalation-1.md`, snapped as inputs#I6. architecture 1 · 2 · 3 · 5, security-plan 1, obs-plan 1 · 2 applied as built,
  each body and sidecar entry marking the in-process scope (F1) or the no-field rule (F2) PROVISIONAL.
- Applied: architecture 7 · security-plan 2 · test-plan 16 + 1 raised by the cascade sweep (the
  `registries/contracts/test-plan/session-lifecycle.md` unit count, `cascade-dispositions.md`) · obs-plan 4 · a11y-plan 1 ·
  layout-templates 1 = 32 amendments; 0 rejected; 0 declined.
- Body edits: `replace-1.md` … `replace-7.md` (`cascade.py replace`, read back) and four new lines by the Edit tool
  (architecture.md's two group bullets, security-plan.md's Layout values row, obs-plan.md's absence bullet,
  a11y-plan.md's §6 bullet, layout-templates.md's IA note). `classify.py check --mode standing`: architecture PASS,
  obs-plan PASS (after one insertion moved off a standing provenance clause, `replace-5.md`). `registry.py check`
  test-plan: 0 defects.
- Sidecars: six entries, each `sidecar.py check` on-form, appended by `splice.py`, the last line read back.
