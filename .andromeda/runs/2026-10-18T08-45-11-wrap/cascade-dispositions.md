# Cascade dispositions — 2026-10-18-one-virtual-clock

**The search.** `cascade.py sweep --patterns-file .andromeda/runs/2026-10-18T08-45-11-wrap/cascade-patterns.toml` (trail `cascade-2026-10-18-one-virtual-clock.json`), baseline f45bb570, run after every body amendment of this pass had landed. 25 patterns written; `title` refused (its control never fired over the pre-pass masters — the chunk's title is in no master) and dropped; the other 24 re-fired by `--pattern`. Each pattern spans the claim's own verbs, not only its name: the retired cause count (`causes-nine`: nine causes · cause names · ninth · `[Cause; 9]` · all nine · seven by the executor · closed at nine · the other seven), the refusal (`time-unavail`, `refuses-adv`), the step's return and the clamp (`step-returns`), the dropped remainder (`remainder`), the clock left still (`clock-still`: stays where it was · leaves that clock · reading the same before and after · harness's animation clock · reads no clock), the wall-time fade (`wall-fade`, `help-text`, `fade-residual`), the epoch unwraps (`epoch`), "`tick` … unchanged" (`tick-unchanged`), the moved counts (`sc-readers`, `stand-files`, `bt-targets`, `local-tally`, `repeat-tally`, `sg-units`, `renamed-tests`, `mcp-causes`, `module-list`), the new clock (`doc-clock`), the double click (`dblclick`), the step itself (`with-time`) and the CARRY's owner (`carry-owner`). Sections read beyond the rows: architecture §Established Decisions, §Standard Contracts (Document core, Test harness, Headless stand, Driver session, CLIs), §Infrastructure Patterns; security-plan §Input Validation, §API Security, §Error Handling, §Logging & Monitoring; design-system §Motion; test-plan §1–§10 and the session-lifecycle key file; obs-plan §3, §4, §6; a11y-plan §5.

**Counts**, copied from the listing: `total (24 patterns) · 113 rows over 19 files` · `per class · new 32/7 · standing 37/5 · leaf 44/12 · curation 0/0 · base 0/0`. Zero-row patterns whose control fired: `epoch`, `tick-unchanged`, `bt-targets`, `sg-units`, `renamed-tests`, `module-list`, `fade-residual` — every retired wording they name is gone from the masters.

## Master rows — new (32)
Every `new` row is this pass's own text (the amended lines and the three new detail groups / bullets): architecture.md:91, :139, :165, :169, :175, :180, :305; security-plan.md:81, :217, :374; design-system.md:291; test-plan.md:16, :27, :89, :155, :258, :287, :327, :334; obs-plan.md:70, :75; a11y-plan.md:162; session-lifecycle.md:11 → amended (each re-read; no intra-line duplicate of a retired mechanism).

## Master rows — standing (37)
- carry-owner: architecture.md:90 (the engine-debts paragraph's owner) and :91 (this pass's bullet naming the same CARRY) → no change, both name the live owner.
- causes-nine: architecture.md:145 @6096 (a CRUD "ninth row") → no change, a different ninth; architecture.md:175 @19677 ("the remedy, the nine causes and their names unchanged", a 2026-10-07 reword's record) → no change, a dated history clause; test-plan.md:27 @19985, @32280 (a ninth control row, a ninth list row) → no change; test-plan.md:163 ("seven files of slice s07") → no change, a census.
- step-returns: security-plan.md:135 ("clamped to the f32 range") → no change, another claim.
- clock-still: architecture.md:165 @8485 (settle "reads no clock but the harness's own") → no change, true; security-plan.md:81 @2692 (`validate` reads no clock) and @4771 (the executor reads no clock — `advance` moves the harness's clock through `advance_time` and reads none) → no change, true; security-plan.md:217 @1837 (its validation reads no clock) → no change; test-plan.md:90 (settle reads no clock but the harness's) → no change, true.
- wall-fade: test-plan.md:183 ×2 ("no wall-clock read in the checks", this pass's "repeat across wall time") → no change / amended.
- refuses-adv: architecture.md:180 (this pass's "no time step and still taking `advance`") → amended.
- local-tally, repeat-tally: test-plan.md:328 (§9 Per-OS, the earlier readings of the series, kept as the line's record) → no change; this pass appends the CI#39111579925 reading on the same line.
- help-text: architecture.md:175 @18936 ("the tool itself said two measured limits", the 2026-10-10 history) → no change, dated history.
- doc-clock: architecture.md:122 and test-plan.md:99 (blitz-vibey-script's own `clock.rs`) → no change, another clock.
- mcp-causes: test-plan.md:27 @35838 → amended (this pass).
- dblclick: architecture.md:86 → amended (the clock qualifier); architecture.md:164, :305 @19201 and test-plan.md:19 (the password double click) → no change, true.
- with-time: architecture.md:173 (the stand's `timer_step`), :175 @1396/@1527/@15058/@35639, :180, security-plan.md:81 @4571, test-plan.md:27 @20985, :88, :287, session-lifecycle.md:5 @2571 → amended (this pass); architecture.md:175 @2104 (the boot and the time step are the caller's), :300 @668 (session holds `with_time`), test-plan.md:10 @7195 (the timer's the one session carrying a step), test-plan.md:27 @35717 (a held Timer gets its step through `start`), test-plan.md:102 @389 (`hold` hands the timer the step, the others none), obs-plan.md:70 @4209 (`Session::with_time` reaches no log), session-lifecycle.md:5 @3310 (`act` and `with_time` in process), :9 @2222/@2294 (the timer's session given the step, the other three none), :11 @3229 (this pass's) → no change, each true: the step and its holders are unchanged, only what `advance` does around it moved.

## Leaf rows (44 over 12 files)
Each leaf joins cascade step 3's set: CLAUDE.md:26, :37, :38, :81, :110; .claude/rules/security.md:22; .claude/rules/testing.md:15; .claude/rules/verification-harness.md:32, :36; .claude/docs/commands.md:61; .claude/docs/obs-summary.md:49; .claude/docs/security-summary.md:23; .claude/docs/services/blitz-paint.md:18; .claude/docs/services/blitz-test-harness.md:20; .claude/docs/services/escher-driver.md:6, :16, :18, :19, :24, :26, :29, :35, :61; .claude/docs/services/seven_guis.md:11, :19; .claude/docs/tests-summary.md:14, :21, :24, :29, :43 → re-derived (step 3).

Curation homes: 0 rows. Judgment bases: 0 rows.

## Leaves
`cascade.py leaves --add` (the 12 leaf files above): `total · pass · 12 group(s) · 25 leaves · 391501 B · unmapped 1`; layout-templates read `re-pointed` (citation digits alone, no changed source). Mode: pass for every group — this chunk does not close Epoch 3 (`route.py epoch`: 2 markerless entries remain under it). One comparer per group in one batch inside one snapshot pair (before `tree 0862902b…`, after `tree 6850e3aa…`, index and head unchanged; exactly the twelve `A … leaf-{group}.md` lines). `leaves --apply`, dry run then write: `total · blocks 48 · applied 48`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 2 | 2 | 0 | 0 |
| architecture-2 | pass | 12 | 12 | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| CLAUDE.md (architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan) | pass | 7 | 7 | 0 | 0 |
| architecture+test-plan | pass | 5 | 5 | 0 | 0 |
| architecture+obs-plan | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 2 | 2 | 0 | 0 |
| design-system | pass | 1 | 1 | 0 | 0 |
| test-plan-1 | pass | 10 | 10 | 0 | 0 |
| test-plan-2 | pass | 4 | 4 | 0 | 0 |
| obs-plan | pass | 3 | 3 | 0 | 0 |
| a11y-plan | pass | 2 | 2 | 0 | 0 |

Every finding's source side is a body this pass amended (the eight causes, `advance` on the one clock, the step returning nothing, the harness's `VirtualClock` and `advance_time`, the screenshot help's one limit, the clock checks and counts, the double click on the document's clock), read at its cited master line. Omissions applied where a reader of the leaf would otherwise act against the source (an engine time read added outside `clock.rs`, a harness check that moves no clock, a test reading an `advance` refused). `unmapped .claude/rules/host-linux.md (host)` → the host leaf, no master states it, no comparer. The sweep's other leaf rows (CLAUDE.md:26, :38, :81; commands.md:61's time-step clause; blitz-paint.md:18; blitz-test-harness.md:20; escher-driver.md:6, :29; tests-summary.md:21's password clause) were read by their comparers and found true: no finding.
