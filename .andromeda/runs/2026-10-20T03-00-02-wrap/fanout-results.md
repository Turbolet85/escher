# Fan-out results — 2026-10-20-load-wait (wrap 2026-10-20T03-00-02, resumed)

Path check: before `tree e313340a4f0f157f1854dc65c13c6f1628430629 index 07fc8696b1b7d9b9c712b3e4a73d7140427bbf63 head 8dfd1c02b090adcc31224234b319a75400907b30`;
after `tree edcf28b772b3a1a217daeda3e97dbec48bde8bdd`, index and head unchanged; `git diff-tree -r --name-status` printed
exactly the seven `A …/fanout-{doc}.md` lines. Entity probe 0 in every file; no file fixed, so no twin. Every
receipt's count equals its file's.

Validate: 0 proposals rejected for a source the report does not carry (every coordinate is the report's or its
New-text section's). No proposal is a boundary widening or a PROVISIONAL discharge: the `settled` help is the
program's own fixed text inside a ratified carrier, and the wait crosses nothing. The retired «no driver command waits
on a load» was the operator's 2026-10-07 rule; requirement v020-07 names it as the residual it ends, so the version's
intent rules the reversal (check 1 · check 3). The four security proposals carry the detector's `escalate` grade with
its invariant reported holding and the plan's reviewed list naming each change — the playbook's graded-escalate rule,
routine. 0 escalations.

architecture: 4 proposals · fanout-architecture.md
note: D-arch-decisions clean; Harness::settle's «never waits» stays; the Existing Scopes row names no session item (read: architecture.md:348 holds no `LOAD_WAIT`-shaped list — no omission).
1. apply — check 1 (Accurate this-chunk addition): re-exports 55 → 56, `LOAD_WAIT` among the session's names (192@c48)
2. apply — check 1: «settling moves no time and waits on no load» → the load wait over `Harness::settle` (192@c38779), the mechanism stated once in a new group bullet
3. apply — check 1, with 2: the `act` sentence settles through the load wait (192@c1244)
4. apply — check 1, with 2: the clock clause — no clock but the session's load wait (192@c24965)
5. (raised by main, check 5 — the plan's implementation note «Replay determinism»: state the timing sensitivity in the Driver session amendment) apply — §Standard Contracts → Driver session: a kept check whose app meets a load can read `settled` differently on a replay whose load lands at another time (recorded, not established); the committed Counter checks meet no load — in the load-wait group bullet
6. (raised by main, cascade step 2 — the `fxreaders` row the sweep printed at architecture.md:363, a stale site no detector named) apply — §Existing Scopes: `session_common` readers 26 → 27, `fixtures` 20 → 21 (19 → 20 `stand_*`)

security-plan: 4 proposals · fanout-security-plan.md
note: D-security-auth and D-security-deps do not fire; no new external-input surface.
1. apply — playbook graded-escalate rule (invariant holds, plan list names it): the «Settle and loads in flight» row — settle never waits; the driver's acting step waits up to `LOAD_WAIT`, so the absent request timeout cannot hang a command (222@c2)
2. apply — same rule, with 1: «no driver command waits on a load (the operator, 2026-10-07)» → an acting command waits on a load for at most `LOAD_WAIT`, stated in the `settled` help (222@c2234)
3. apply — same rule: the executor reads no clock itself, its settled step's load wait reading the wall clock (83@c4819)
4. apply — same rule, with 3: the session joins the clock readers, for the load wait alone (83@c8549)

design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds; the §Motion expected amendment left to main.
1. (raised by main, check 5 — Expected amendment design-system §Motion §Animation runtime) apply — routine, the report's fact `motion`: the driver's load wait moves no clock, so a timer or an animation does not run while an acting step waits on a load

layout-templates: 0 proposals · fanout-layout-templates.md
note: D-layout-surface holds; the cli bullet describes the help texts by shape and quotes no field help.
1. (raised by main, check 5 — Expected amendment layout-templates §Surface: cli → Primary screens) apply — routine, the report's fact `settled`: the acting tools' descriptions, through the `settled` field's help, state the 5 s load wait

test-plan: 24 proposals · fanout-test-plan.md
note: D-tests-framework and D-tests-obs-harness hold; test-plan.md:91 stays true of `Harness::settle`.
1. apply — check 1: §1 escher-driver 94 → 99 unit tests, 64 inline · 35 `#[path]` (22@c21)
2. apply — with 1: §1 per-file session.rs 6 → 10
3. apply — with 1: §1 per-file schema_tests.rs 6 → 7
4. apply — with 1: §3 → Session lifecycle (key file) session-proof's escher-driver count 94 → 99
5. apply — check 1: §4 escher-driver unit coverage of the load wait and the `settled` help pin
6. apply — check 1: §1 tests/blitz-tests `stand_load_wait` 2 entered
7. apply — check 1: §2 Directory pattern 140 → 141 `.rs` files, 139 → 140 targets (68@c3405,c3422)
8. apply — check 1: §2 `session_common` readers 26 → 27 (68@c1349)
9. apply — with 8: §3 helpers `session_common` 26 → 27 (103@c2926)
10. apply — check 1: §2 `fixtures` readers 20 → 21, `stand_*` 19 → 20 (68@c2501)
11. apply — with 10: §3 helpers `fixtures` 20 → 21 (104)
12. apply — with 10: §3 test-data bootstrap «the 19 `stand_*` readers» → 20 (132@c421) — a site of fact `fxreaders` the report's pre-pass sweep did not list (read)
13. apply — check 1: §3 Proof `run stand` 185 · 0 · 7 over 46 appended (119@c10078)
14. apply — with 13: §3 test-data bootstrap's `run stand` reading 183 → 185 (132@c755)
15. apply — with 13: §9 Legs stand-repeat 45 → 46 files (323@c1299)
16. apply — with 13: §10 Flakiness budget 45 → 46 files, the stand repeat 162 · 645 · 0 · 21 (352)
17. apply — check 1: §9 Per-OS tallies re-read on CI#39360124515 (333)
18. apply — check 1: §9 Local baseline 202 · 1062 · 0 · 12 (339@c18738)
19. apply — check 1: §10 Coverage floors escher-driver 4695/4794 = 97.93 % on CI#39360124515 (351)
20. apply — check 1: §5 the second recorded exception to «no sleep and no wall-clock read» (185@c2589,c2665)
21. apply — check 1: §5 Session ↔ held instance — the acting step's bounded load wait and its proof
22. apply — check 1: §8 `LateNet` recorded beside `HeldNet`
23. apply — check 1: §3 → Session lifecycle (key file) session-start — `act` and the acting verbs wait on a load up to `LOAD_WAIT`
24. apply — check 1: §3 → Session lifecycle (key file) session-proof — `stand_load_wait` 2 entered

obs-plan: 6 proposals · fanout-obs-plan.md
note: D-obs-pii no drift; no obs-plan site holds the two retired statements.
1. apply — check 1: §3 Logging stack — the session reads a clock for the load wait alone (70@c2829)
2. apply — check 1: §4 «Still observed absent» — no span, event or field of the load wait's own (119@c4421)
3. apply — check 1: §4 `passes` on a waited step (119@c766)
4. apply — check 1: §3 detail (absence · logs) `fixtures` readers 20 → 21 (79@c404)
5. apply — check 1: §3 Logging stack «the module twenty-six checks read» → twenty-seven, `stand_load_wait` added (70@c6057) — a site of fact `screaders` the report's pre-pass sweep, keyed on digits, did not list (read)
6. apply — check 1: §3 detail (absence · logs) the load-wait check is silent — a new bullet in the open group

a11y-plan: 0 proposals · fanout-a11y-plan.md
note: D-a11y-surface and D-a11y-obs-schema unfired; §2's changed-set claim stays true.
