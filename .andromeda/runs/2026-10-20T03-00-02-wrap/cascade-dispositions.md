# Cascade dispositions — 2026-10-20-load-wait (wrap 2026-10-20T03-00-02)

**The search.** `cascade.py sweep --patterns-file cascade-patterns.toml` (trail `cascade-2026-10-20-load-wait.json`),
baseline 165cfe44 (the parent of the oldest pre-CI commit 8dfd1c02), run after the last body amendment (the arch:363
fold, which the first run of the same set found). Fourteen patterns fired, each on its known-positive control:
`noload` (the retired «waits on no load» / «never waits on one», every verb form) · `nocmdwait` («no driver command
waits») · `actover` (`act` over settle, `act(step)`, `Session::act`) · `reexp` (55 / 56 names) · `noclock` (every
«reads no clock» form) · `screaders` (26 / 27 `session_common` readers, digits and words) · `fxreaders` (20 / 21
`fixtures` readers, 19 / 20 `stand_*`) · `tfiles` (140 / 141 files, 139 / 140 targets) · `runstand` (183 / 185, 45 /
46 files) · `duunit` (94 / 99 unit tests, the per-file session.rs and schema_tests.rs counts) · `wstally` · `ostally`
· `cover` · `sleepexc` (the «no sleep and no wall-clock read» rule and its exceptions). A fifteenth, `settlehelp` (the
`settled` help's wording), fired no control over the pre-pass masters and was dropped — the help text is quoted by no
master (the report's own `settled` fact: 0 hits). Sections read beyond the rows: architecture §Standard Contracts
(Test harness and Driver session) and §Existing Scopes; security-plan §Input Validation and §API Security; test-plan
§1–§5, §8–§10 and its session-lifecycle key file; obs-plan §3–§4; design-system §Motion; layout-templates §Surface:
cli. Not looked for: the `settled` help's own wording (no control), leaf statements whose words share no pattern
(step 3's comparers read every statement).

total (14 patterns) · 81 rows over 18 files
per class · new 19/6 · standing 23/5 · leaf 39/12 · curation 0/0 · base 0/0

## Master rows (new · standing)
- architecture.md:182 noload, noclock — no change: the Test harness contract's settle «never waits on one» and «reads no clock but the harness's own» stay true of `Harness::settle`, which this chunk did not change.
- architecture.md:192 reexp new — amended (56 names, `LOAD_WAIT`).
- architecture.md:192 noclock @c32912 — no change: the session's record of ids reads no clock, still true.
- architecture.md:199 actover new — amended (the load-wait group bullet).
- architecture.md:363 fxreaders standing edited mixed ×2 — amended in this pass (`session_common` 26 → 27, `fixtures` 20 → 21, `stand_*` 19 → 20); the `mod common;` 21 beside it is the `common` module's count, unmoved.
- security-plan.md:83 noclock ×3 — @c2692 no change (validation reads no clock); @c4819 and @c8591 amended (the executor reads no clock itself; the session's load wait joins the clocks).
- security-plan.md:222 noload ×3, noclock, actover new — amended: @c147 settle itself never waits (true, kept), @c2479 the command line's transport bounds wait on no load (true, kept), @c2732 the retired rule rewritten; @c2334 validation reads no clock (true); @c418 the new acting-step clause.
- design-system.md:292 noload new — amended (the load wait moves no clock).
- test-plan.md:91 noload, noclock — no change: the Test harness settle row, true of `Harness::settle`.
- test-plan.md:22 duunit new ×3 — amended.
- test-plan.md:27 actover @c20662 — no change: the Timer check's `Session::act` reading (no load on the stand); sleepexc @c50006 — no change: `stand_virtual_clock`'s 800 ms history.
- test-plan.md:68 screaders, fxreaders ×2, tfiles ×2 new — amended.
- test-plan.md:103 screaders new — amended.
- test-plan.md:104 fxreaders ×2 new — amended.
- test-plan.md:119 runstand standing edited mixed ×2 — amended: the earlier reading kept as history, the new one appended.
- test-plan.md:132 fxreaders, runstand new — amended.
- test-plan.md:185 actover @c393 — no change: `stand_settle`'s `Session::act` reading (no load in it); sleepexc ×3 — amended (the second exception) and @c652 «no sleep or clock read in the check» of `stand_settle`, true.
- test-plan.md:293 actover — no change: the Timer fake's pass list (`Session::act` among settles), true.
- test-plan.md:324 runstand new · 353 runstand new · 352 cover, ostally new · 334 ostally standing edited mixed · 353 ostally standing edited mixed · 340 wstally standing edited mixed — amended: the new reading added, the earlier ones kept as history.
- test-plan.md:333 sleepexc @c4649 — no change: `stand_virtual_clock`'s recorded sleep, ratified history.
- obs-plan.md:70 actover @c3007 — no change: «its settled step `Session::act` … included» names the crate's modules that print nothing, true; screaders new — amended.
- obs-plan.md:79 fxreaders new — amended.
- obs-plan.md:120 actover @c4796 — amended (the load wait's absent span, `Session::act`'s own absence still true).
- session-lifecycle.md:5 actover — amended (session-start); :11 actover @c4305 — no change (`stand_settle`'s count); duunit new ×3 — amended.

## Leaf rows (39 over 12 files) — each leaf joins step 3's set; none is patched here
CLAUDE.md:37 · :110 (noload, nocmdwait, actover) · .claude/rules/security.md:22 · .claude/rules/verification-harness.md:21, :32 · .claude/rules/observability.md:24 · .claude/rules/testing.md:15, :30, :48 · .claude/docs/security-summary.md:17 · .claude/docs/obs-summary.md:49 · .claude/docs/tests-summary.md:18, :21, :24, :29, :30, :43 · .claude/docs/commands.md:61 · .claude/docs/services/blitz-test-harness.md:20 · .claude/docs/services/escher-driver.md:12, :17, :38, :60 · .claude/docs/services/seven_guis.md:21 → leaf — re-derived (step 3).

## Curation homes · judgment bases
0 rows · 0 rows.

## Leaves
`cascade.py leaves` (pass mode — the epoch does not close: Epoch 4 holds 2 markerless entries, `route.py epoch`), the
step-2 leaf rows added: 12 groups · 23 leaves; `re-pointed .andromeda/a11y-plan.md` (citation digits alone — no
changed source); `unmapped .claude/rules/host-linux.md (host)` — the host's own leaf, no master's, no comparer's.
One comparer per group in one batch; path check: before `tree a2491abcc6fa4aa46f4028d2fb5a1f4d7cd981c4`, after
`tree 1622b09726155a63e42d0f076d82ec6a84eead32`, index and head unchanged, exactly the twelve `A …/leaf-{group}.md`
lines (architecture-1's agent stopped after its file was written, a background job of its own still open). Every
receipt's count equals its file's blocks.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 0 | 0 | 0 | 0 |
| architecture-2 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+design-system+layout-templates+test-plan+obs-plan (CLAUDE.md) | pass | 4 | 4 | 0 | 0 |
| architecture+security-plan+test-plan (services/escher-driver.md) | pass | 11 | 11 | 0 | 0 |
| architecture+layout-templates (services/seven_guis.md) | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan (commands.md · services/blitz-test-harness.md) | pass | 3 | 3 | 0 | 0 |
| architecture+obs-plan (services/escher-telemetry.md) | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 2 | 2 | 0 | 0 |
| design-system+layout-templates | pass | 1 | 1 | 0 | 0 |
| test-plan-1 | pass | 11 | 11 | 0 | 0 |
| test-plan-2 | pass | 7 | 7 | 0 | 0 |
| obs-plan | pass | 3 | 3 | 0 | 0 |

`total · blocks 42 · applied 42` (`leaves --apply`, dry run first). Each finding's source side was read at its cited
line: every one states what this pass's amended bodies say (the load wait, the retired «no driver command waits on a
load», the moved counts and readings). Leaf statements of settle itself («waits on no load», «reads no clock but the
harness's» — rules/verification-harness.md:32, services/blitz-test-harness.md:20) stay true and were left.
