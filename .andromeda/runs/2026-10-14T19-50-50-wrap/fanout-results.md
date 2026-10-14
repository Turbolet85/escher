# Fan-out results — 2026-10-14-stand-a11y-assertions (2a resume; P2 re-fanned, no earlier fanout file in the run dir)

Path check: before `tree 68009472e1ede4d75ef74a8491ee49d6c17aa7d4 index 5977a52866d3a09b75728cf14c7fd72966778259 head aab4cda979ba79aadbe2c9f16e1b848321c2620a` · after `tree 34059a4313057606816b5682278d9c6101ca0615`, index and head unchanged · `git diff-tree` printed exactly the seven `A {run_dir}/fanout-{doc}.md` lines. No file needed a fix (no text before `proposals:`, entities probe 0), so no raw twin. Every receipt count equals its file's `detector:` count.

Validation basis: each proposal read against the report's Changes and its `## New text, by line` section — every added-text coordinate a proposal carries is a listing row or added range (stand_contrast.rs 168-187 · 206-279 · 281-297 · 299-417 · 419-471 · 577-605 · 607-635 · 637-661; stand_keyboard.rs 209-235 · 237-268 · 270-275 · 291-322 · 324-342; stand_accessibility_ids.rs 26-37 · 39-43 · 255-308 · 310-352 · 363-410; common/mod.rs 72-99 · 133-136) or a coordinate the report's bullets state (crud.rs:59, ci-leg.sh:34, test_ci_workflows.py:289-290). Rejected for an unreported source: 0. No proposal is a boundary widening (no carrier, field, listener, crate or input class added — report Coverage of new surfaces); none discharges a PROVISIONAL mark; none changes engine behaviour. The two `escalate`-graded security proposals fall under the playbook rule "A proposal graded `escalate` by a detector's own severity, where that detector reports its invariant holding and the plan's reviewed list names the change" (the plan's entry "security-plan §Input Validation `id` row"). The owner moves to `.andromeda/residuals.md` lines rest on the operator's P5 word (inputs#I5: rows 1–3 to residuals, `(target: next)`, each naming the founder's label), a recorded direction; P5 of this wrap lands the three lines.

architecture: 7 proposals · fanout-architecture.md
note: no new resource; the two owner re-homes (:86, :122) depend on P5 writing the residuals lines; :136 and the refresh-mechanism rows stay.
  1. apply — check 1 "Accurate this-chunk addition" (routine); plan entry "architecture §Standard Contracts → CI contracts" — the `a11y` leg list at six files (:141).
  2. apply — dependent-of 1 (routine) — Existing Scopes, `accessibility_states`'s "the `a11y` CI leg stays its three named files" (:263).
  3. apply — check 1 routine; report Counts `record-tally` arch:263 ×3 and the citation-dispositions `claim false` row — `stand_contrast` 5 tests, record 14 · 0 · 3 · 1, TEXT_ROWS, STATE_ROWS, in the a11y leg (:263).
  4. apply — check 1 routine; report Counts `stand_keyboard` 11 → 14, SC 2.1.1 / 2.4.3 in the enabled states (:263).
  5. apply — check 1 routine; report Symbols `ENABLED_TAB_ORDER`, `REGION_NAMES`, `common-tables` arch:263 (:263).
  6. apply — check 1 routine; report Counts `inert` / `route-entry` arch:86 (owner moves, F3), P5 card row 3 — [Unsupported features] owner → residuals line (:86).
  7. apply — dependent-of 6 (routine) — Mutation, query and CSSOM: the AT witness's owner → residuals line, F1 (:122 @5163).

security-plan: 2 proposals · fanout-security-plan.md
note: D-security-auth and D-security-deps fire on nothing; both proposals carry the detector's fixed `escalate` grade though neither is a widening.
  1. apply — check 1: the playbook's escalate-graded-holding rule (routine); plan entry "security-plan §Input Validation `id` row" — the `&dyn Document` tree proof and the refresh's two halves headless, the windowed witness owed and re-homed (F1) (:123).
  2. apply — dependent-of 1 (routine) — the control-states row's witness owner → residuals line, F1 (:125 @2064).

design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds; the accent-row change is the plan's expected amendment, outside the detector's invariant — the orchestrator's to carry.
  1. apply — raised by main, check 5 (routine; plan entry "design-system §Color Palette (Core Colors)", citation-dispositions `claim false` row design-system.md:24) — §Color Palette → Core Colors, the `seven_guis accent` row: rest `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, white 5.47 · 6.83 · 8.27, the five accent controls passing at 5.47 in `stand_contrast`; the 4.39 fail retired.

layout-templates: 1 proposal · fanout-layout-templates.md
note: no duplicate occurrence; :45's `crud-list` rows and the six-stop boot sequence at :10 stay true.
  1. apply — check 1 routine; plan entry "layout-templates §Surface: desktop-native → Primary screens" — `crud-list` named `People` by `aria-label` (crud.rs:59), the current-option gap still stated (:10).

test-plan: 10 proposals · fanout-test-plan.md
note: D-tests-obs-harness finds no drift; test-plan.md:142's "4.48 and 4.39" stays by design.
  1. apply — check 1 routine; plan entry "test-plan §1" — `stand_contrast` 2 → 5, record 14 · 0 · 3 · 1, the three tables, in the a11y leg; "stays three files" and "records and gates nothing … owned by the route entry" retired (:26 @30295–31235).
  2. apply — check 1 routine; plan entry "test-plan §1" — `stand_keyboard` 11 → 14 and its three tests (:26).
  3. apply — check 1 routine; plan entry "test-plan §1" — `stand_accessibility_ids` 5 → 8 and its three tests; the adapter-gap owner → residuals line, F1 (:26 @3960, @10436).
  4. apply — dependent-of 1 (routine) — `accessibility_states`'s "not by the `a11y` leg, which stays three files" (:26 @10166).
  5. apply — check 1 routine; plan entry "test-plan §3" — the shared module gains `ENABLED_TAB_ORDER` and `REGION_NAMES` (:100).
  6. apply — check 1 routine; plan entry "test-plan §3" — `run stand` 162 · 0 · 7 over 38 files (:116).
  7. apply — check 1 routine; plan entry "test-plan §9" — workspace 182 · 937 · 0 · 12 (:330).
  8. apply — check 1 routine; plan entry "test-plan §9", citation-dispositions `claim false` row test-plan.md:315 — the a11y leg's six files (:315).
  9. apply — dependent-of 8 (routine); plan entry "test-plan §4" — `LegScriptTest`'s a11y row pins six files (:145).
  10. apply — dependent-of 8 (routine); plan entry "test-plan §9 … its local reading" — the a11y leg's local reading at six files, the 6 · 6 · 3 history kept.

obs-plan: 1 proposal · fanout-obs-plan.md
note: D-obs-instrumentation and D-obs-pii find no drift; the "19 of which read the shared module" count is left to the orchestrator — `stand_accessibility_ids` already reads it (test-plan.md:100 lists it among the 19), so the count holds.
  1. apply — check 1 routine; plan entry "obs-plan §3" — the silence census restated over `stand_contrast`, `stand_keyboard`, `stand_accessibility_ids` and the shared module (:70).

a11y-plan: 14 proposals · fanout-a11y-plan.md
note: D-a11y-obs-schema finds no drift; the keyed contract's lines 3 and 4 hold; lines 75, 158, 282, 326 read and hold.
  1. apply — check 1 routine; plan entry "a11y-plan §6", citation-dispositions `claim false` row a11y-plan.md:229 — the re-coloured accent, record 14 · 0 · 3 · 1, TEXT_ROWS, STATE_ROWS, hover and active, in the a11y leg; "not met" and the route-entry owner retired (:229).
  2. apply — dependent-of 1 (routine) — §6 NOT YET MEASURED note: the list rows, the text outside the controls and the colour states measured (:270).
  3. apply — dependent-of 1 (routine); citation-dispositions `claim false` row a11y-plan.md:77 — §3 Contrast verification harness: three tables, in the a11y leg (:77).
  4. apply — check 1 routine; plan entry "a11y-plan §9", citation-dispositions `claim false` row a11y-plan.md:340 — the leg at six files, F4, its log reading (:340).
  5. apply — dependent-of 4 (routine) — §3 tool pick: "the `a11y` leg staying its three files" (:68).
  6. apply — check 1 routine; plan entry "a11y-plan §5" — Enter and Space on `crud-update` / `crud-delete`, 4 rows (:166).
  7. apply — check 1 routine; plan entry "a11y-plan §5" — the enabled-state walks, `ENABLED_TAB_ORDER`, every control reached by Tab (:213).
  8. apply — check 1 routine; plan entry "a11y-plan §5 … `inert` re-homed (F3)" — owner → residuals line (:186).
  9. apply — check 1 routine; plan entry "a11y-plan §5 … the tree's current-option gap … re-homed (F2)" — owner → residuals line; the list named `People` (:196).
  10. apply — check 1 routine; plan entry "a11y-plan §7" — `crud-list` named, the interactive-role sweep, the `&dyn Document` tree (:280).
  11. apply — dependent-of 10 (routine) — the attribute-state bullet: the empty name retired, the `aria-activedescendant` owner → residuals line, F2 (:283).
  12. apply — check 1 routine; plan entry "a11y-plan §1 and §2" — the refresh's two halves headless, the windowed witness re-homed, F1 (:36).
  13. apply — dependent-of 12 (routine) — the state-attributes bullet's witness owner → residuals line, F1 (:46).
  14. apply — check 1 routine; plan entry "a11y-plan §1 and §2 … the adapter decision re-homed (F1)" (:14).

## Checks 2–6
- Check 2 (cross-contradiction): no two proposals edit one section in opposing directions; the arch/security/a11y/test owner moves all name the same three residuals lines.
- Check 3 (intent-consistency): the report's work matches the entry "Stand a11y assertions" and the plan's acceptance criteria (Outcome: every criterion Met); scope record: none (`gate.py scope` clean, 0 recorded). The entry's title says "an a11y CI leg gating merges"; the founder's F4 widened the existing leg with no required status check — the plan's own reading, no divergence.
- Check 4 (absence needs evidence): each "no other occurrence" claim is the site sweep's (report Counts) and this pass's cascade sweep, read by offset where a line passes 2 000 chars (`cascade.py window` / the session's windows at architecture.md:86, :122, :136, :141, :263; security-plan.md:119, :123, :125, :144, :246; test-plan.md:24, :26; a11y-plan.md listed lines).
- Check 5 (expected amendments): all twelve plan entries matched — eleven by proposals, design-system's raised by main (design-system 1). Citation-dispositions `claim false` rows: architecture.md:263 (architecture 3), design-system.md:24 (design-system 1), test-plan.md:26 (test-plan 1), test-plan.md:315 (test-plan 8), a11y-plan.md:77 (a11y-plan 3), a11y-plan.md:229 (a11y-plan 1), a11y-plan.md:340 (a11y-plan 4) — all raised.
- Check 6 (disproved claims): (1) research.md's `#333` at 11.59 for the filter label — a chunk-artifact claim, no master holds the label's colour (report); its measured value stands in `TEXT_ROWS` and the report → DISPOSED, no amendment; (2) CARRY 9's predicted 4.39 on the selected row — route text on this chunk's frozen line → DISPOSED to P5/P7 (the line's annotations strip at flip-compaction; the measured 5.47 is in a11y-plan 1); (3) the pre-chunk masters → matched by the planned amendments above.

Escalations: 0. Answer card: none.
