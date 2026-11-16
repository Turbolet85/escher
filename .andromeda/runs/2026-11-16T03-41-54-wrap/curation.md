CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "`rust-code-analysis-cli` names its per-file output by the file's basename … key the complexity measure on each file's full path" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "rust-code-analysis-cli's basename output overwrites same-named files"
  No-other-home: "gate.py control's holds needs every named case to read failed"
  Extended: T2/testing.md: "In a `gate.py control` file a control's `case` names the test function alone" + "its `holds` needs every named case to read failed, so a control names only the case its edit reddens"
  CLAUDE.md size: 164/200 · T1 4.6 KB, 0 over 600 B

## Applied
- T2 testing.md, new: rust-code-analysis-cli basename overwrite (0.8 = measured +0.4 · specific detail +0.2 · no other
  home +0.2).
  Proof: implement's Decisions & corrections, «A sweep hazard in measuring complexity» — the two new files
  `packages/blitz-dom/src/layout_reason.rs` and `tests/blitz-tests/tests/layout_reason.rs` overwrote one another's
  output; the measure was re-run through a script keyed on the full path (report, doc:2026-11-16-layout-explained/report).
- T2 testing.md, extended (the 2026-11-14 `gate.py control` entry): holds needs every named case red (0.8 = measured
  +0.4 · specific detail +0.2 · no other home +0.2).
  Proof: implement's Decisions & corrections and outcome — naming the old no-change case beside the new one would read
  INDETERMINATE, so control-2 names the new case alone (`evidence/control-2.txt`).
- Landed: `rec.py harness land … .claude/rules/testing.md` → rev 23, tx 1330.

## Every Decisions & corrections entry, dispositioned
- implement · «A tolerance stated by the plan is read inclusively» — not a learning here: amended into test-plan
  (decision/stated-tolerance-read-inclusively, tx 1316), the master owns it.
- implement · «A control of a "changes nothing" read needs pending work» — not a learning here: amended into
  test-plan (decision/no-change-read-control-needs-pending-work, tx 1316).
- implement · «`gate.py control`'s `holds` needs every named case to read failed» — extended (above).
- implement · «A guard refusal» (bash-guard on a `cd`) — not a learning: the guard put the call on its documented path.
- implement · «A sweep hazard in measuring complexity» — applied (above).
- implement · «Measured stand readings» — rejected, Filter 2: task-specific readings of this chunk's check.
- implement outcome · «The first background gate block was stopped by TaskStop» — rejected, Filter 2: a one-off
  procedure of this run.
- After implement · the wrap directive (inputs#I7) — not a learning: a ruling, landed as the PROVISIONAL architecture
  row and the route's CARRY (P5).
- After implement · «The pass recorded no correction or ruling» — nothing to weigh.
- Citation sweep · `.claude/docs/session-learnings.md:336` out-of-range `host.rs:350` — not a claim: an illustrative
  prefix example, left as is.
- Pre-pass sweep · `rules/testing.md:70` (the 2026-11-14 entry's «38 and 35») — not a correction: the entry's own
  guidance (count the declarers before citing) holds; its figures are dated readings, the anchored greps now reading
  40 (`^mod session_common;`) and 36 (`^mod fixtures;`) over `tests/blitz-tests/tests/*.rs` at this wrap.
