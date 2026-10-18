# Fan-out results — 2026-10-18-one-virtual-clock (resume of run 2026-10-18T08-45-11-wrap)

Batch: one snapshot pair — before `tree 00f17192aad0c87a4035627e2228cb5a27c9bb03 index 2e199436526b3dc6ed78859618b151c086a13462 head 87f1207940e15913f200e11922a5837ae70b5a33`, after `tree 0a234799f23d0475eac0fd0a33f07c3561f9fc88`, index and head unchanged; `git diff-tree -r --name-status` printed exactly the seven `A … fanout-{doc}.md` lines. Every file split at its `- detector:` items; entity probe 0 in all seven; no fix owed, no twin. Receipts equal the files' counts. Total 47 proposals; 0 rejected for a source the report does not carry (each coordinate is a report bullet's or a New-text row's, a span of two rows of one file at most).

architecture: 14 proposals · fanout-architecture.md
note: no new socket, port, env var, IPC endpoint or crate; the no-change rows of the report stand.
1. apply — check 1 «Accurate this-chunk addition»; PROVISIONAL (inputs#I4 (a), F1 B) — §Standard Contracts → Document core: `DocumentConfig.clock` in the field list, the clock types in a new detail group.
2. apply — check 1 «Accurate this-chunk addition» — §Standard Contracts → Test harness: the harness's time is the documents' `VirtualClock`, `advance_time`.
3. apply — with 2 (dependent) — Test harness: the "`pump`, `tick` … unchanged" clause restated.
4. apply — check 1; PROVISIONAL (F2 A) — Driver session: `with_time`'s step returns nothing.
5. apply — with 4 (dependent); PROVISIONAL (F2 A · F2b A) — Driver session: `advance` on every session moves the one clock.
6. apply — with 4 (dependent); PROVISIONAL — Driver session: `advanced_ms` is exactly `ms`.
7. apply — with 4 (dependent) — Driver session: the 2026-10-07 record clause on a `time-unavailable` refusal made history.
8. apply — with 4 (dependent) — Headless stand: `timer_step` returns nothing and carries its remainder (also the citation sweep's claim-false row :158).
9. apply — with 4 (dependent) — CLIs: the three tasks with no step still take `advance`.
10. apply — check 1; PROVISIONAL (F2b A) — Driver session: `Cause` closed at eight.
11. apply — with 10 (dependent) — Driver session: all eight returned by code, six by the executor.
12. apply — check 1; PROVISIONAL (F1 B) — Driver session: `screenshot`'s help keeps one limit (also the citation sweep's claim-false row :160@c9471).
13. apply — check 1; PROVISIONAL (F3 A) — §Established Decisions [Scrolling and input]: the double-click window, smooth scroll and fling read the document's clock, values unchanged.
14. apply — check 1 «An engine behaviour change for every document», a delegate's answer → PROVISIONAL (the «Provisional discharge» rule: a mark added, none removed); placement by the operator's own word (inputs#I3 row 1, inputs#I4 (c)) — [Scrolling and input] detail: the document clock beside the seven, its merge surface on the CARRY on «Upstream sync ahead of visual lint and real-app readiness».
15. raised by main (check 5, the citation sweep's claim-false row architecture.md:263) — apply, routine — §Infrastructure Patterns, the blitz-dom module row: the module list gains `clock` (`mod clock;` at lib.rs:42, report Symbols / APIs).
16. raised by main (check 5; measured at this wrap: `mod session_common;` declared by 23 files of `tests/blitz-tests/tests/*.rs`, 22 at f45bb570) — apply, routine — the blitz-tests row: `session_common` readers twenty-two → twenty-three, `stand_virtual_clock` (report Files: new; Changes: `session_common/mod.rs` its companion).

security-plan: 7 proposals · fanout-security-plan.md
note: no unvalidated boundary; every proposal graded warning; D-security-auth and D-security-deps clean.
1. apply — check 1 «Accurate this-chunk addition»; PROVISIONAL (F2b A) — §Input Validation, Driver command schema row: eight causes, two by `validate`, six by the executor.
2. apply — with 1 (dependent) — §Error Handling, the refusal bullet: cause set closed at eight; the later dated clause "(nine) … unchanged" kept as history.
3. apply — with 1 (dependent) — §Logging & Monitoring: the verb table's eight causes; `stand_act_spans` reads an accepted `advance` line.
4. apply — check 1; PROVISIONAL (F2 A · F2b A) — §Input Validation, Driver command schema row: `advance` accepted on every session, moving the clock by exactly `ms`, `advanced_ms` = `ms`; the clamp retired.
5. apply — check 1; PROVISIONAL (F1 B) — §API Security, Settle and loads in flight: the wall-clock fade limit, its help sentence and its owed engine change retired.
6. apply — check 1 — §API Security, Settle and loads in flight: settle waits on and moves no clock; its `animating` reads the fade off the document's clock.
7. apply — check 1 — §Error Handling, panic paths: the epoch `unwrap`s retired (also the citation sweep's two claim-false rows :374).

design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds; the §Motion moves are the sweep's facts.
1. raised by main (check 5, plan entry 3; the citation sweep's claim-false rows design-system.md:291 ×2) — apply, routine; PROVISIONAL (F1 B · F2 A) — §Motion: the harness drives animation from the documents' `VirtualClock`, moved by `tick`, `advance_time` and the driver's `advance`; settle never moves it; the fade, smooth scroll and fling read it headlessly.
2. raised by main (check 5, report Counts item 15) — apply, routine — §Motion: settle's `animating` reads the scrollbar fade off the document's clock.

layout-templates: 0 proposals · fanout-layout-templates.md
note: no layout-templates site states a moved fact. Plan entry 4 → no site, nothing to amend (report: Expected amendments 4, not carried).

test-plan: 22 proposals · fanout-test-plan.md
note: §9 Legs and §10 Flakiness count 41 stand files; a coverage gap (sub-document clock, pan, fling) is a route call.
1. apply — check 1; PROVISIONAL (F2 A · F2b A) — §5 Session ↔ held instance: eight causes, `advance` on every session.
2. apply — with 1 (dependent) — §1 escher-driver: refusal.rs pins eight causes.
3. apply — with 1 (dependent) — §1 `stand_act_spans`: `fonts-unavailable` the eighth; the CRUD `advance` row accepted.
4. apply — with 1 (dependent) — §1 `stand_mcp_tools`: seven causes over MCP; the accepted `advance`'s `tool_call` span.
5. apply — with 1 (dependent) — §4 escher-driver: refusal.rs pins eight causes.
6. apply — with 1 (dependent) — §6 Command line: `time-unavailable` leaves the causes `cli_commands` provokes; `host_timer` exact.
7. apply — check 1; PROVISIONAL (F2 A) — §8 Time: `advance` moves the one clock; `timer_step` carries its remainder.
8. apply — with 1 (dependent) — §1 `stand_act_timer`: no-step sessions accept `advance`; the two renamed tests.
9. apply — with 1 (dependent) — §1 seven_guis library tests and `host_timer`: the renamed and the new test, 3 passed, 250 → 250.
10. apply — with 7 (dependent) — §3 Construction: `timer_step` returns nothing and carries its remainder.
11. apply — with 1 (dependent) — §3 → session-lifecycle (session-start): `advance` on every session, eight causes.
12. apply — check 1 (also the citation sweep's claim-false row :88) — §3 Core: `time`, `tick`, `advance_time` on the `VirtualClock`.
13. apply — check 1 — §7 Builders and options: harness documents carry the harness's clock.
14. apply — check 1; PROVISIONAL (F1 B) — §9 Engine features by runner: the fade's wall-clock limit retired.
15. apply — check 1 — §5: the one recorded sleep, `stand_virtual_clock`'s 800 ms.
16. apply — check 1 — §1 blitz-dom: the `clock` module's three unit tests.
17. apply — with 16 (dependent) — §4: what the clock unit tests pin.
18. apply — check 1 — §1 tests/blitz-tests: `virtual_clock_engine` 4 and `stand_virtual_clock` 2.
19. apply — check 1 — §2 Directory pattern: 132 · 131 → 134 · 133.
20. apply — check 1 on the operator's direction (inputs#I4 (b)) — §9 Per-OS: re-read on CI#39111579925 (`ci-tallies.md`).
21. apply — check 1 — §9 Local baseline: 192 · 991 → 194 · 1001 (the sub-bullet, not line 325).
22. apply — check 1 — §3 → session-lifecycle (session-proof): seven_guis library tests 3; `stand_virtual_clock` joins.
23. raised by main (check 5; measured at this wrap: `ls tests/blitz-tests/tests/stand_*.rs` 42, 41 at f45bb570; report Harness / gate surface: `stand_virtual_clock` joins `stand-repeat` by its prefix) — apply, routine — §9 Legs: 41 → 42 files.
24. raised by main (check 5, as 23) — apply, routine — §10 Flakiness budget: 41 → 42 files.
25. raised by main (check 5, as arch 16) — apply, routine — §2 Directory pattern: `session_common` read by 23 checks.
26. raised by main (check 5, as arch 16) — apply, routine — §3 Crate-local test helpers: `stand_virtual_clock` declares `mod session_common;`.

obs-plan: 4 proposals · fanout-obs-plan.md
note: the §3 silent census was left for measurement.
1. apply — check 1; PROVISIONAL (F2b A) — §4 command span: eight cause names, six by the executor.
2. apply — with 1 (dependent) — §6 escher-driver log line: eight cause names.
3. apply — check 1; PROVISIONAL (F2 A) — §4 command span: an accepted `advance` records the schema fields only.
4. apply — check 1 — §4 `tool_call`: an accepted `advance` tool call ends `accepted` with one child `command`.
5. raised by main (check 5, plan entry 6; measured at this wrap: a silence probe — `println!` `eprintln!` `print!` `eprint!` `dbg!` `set_global_default` `tracing_subscriber` `escher_telemetry` `env::var` `std::env` `tracing::` — reads 0 over `stand_virtual_clock.rs` and `virtual_clock_engine.rs`, and over `session_common/mod.rs` one pre-existing `std::env::current_exe()`, no env var) — apply, routine — §3 silent census: the two new checks are silent.
6. raised by main (check 5, as arch 16) — apply, routine — §3: the module twenty-two → twenty-three checks read, `stand_virtual_clock`.

a11y-plan: 0 proposals · fanout-a11y-plan.md
note: no a11y-plan claim retired; plan entry 7 would be a new line.
1. raised by main (check 5, plan entry 7; report Counts item 8, Outcome (a11y)) — apply, routine; PROVISIONAL (F3 A) — §5: two driver clicks on one point with no `advance` between them read as a double click by the session's clock; single-click focus unchanged.

## The other Validate checks
- Check 2 (cross-contradiction): none — test-plan 1 and 15 edit different clauses of §5's one line.
- Check 3 (intent): the scope record's three companion lines (two self, one on the operator's word) serve their files; the report keeps the chunk's intent. No escalation.
- Check 4 (absence): the detectors' "not among the sweep's rows" notes were read at their sites; the silence and count measurements above carry their searches.
- Check 6 (disproved claims): plan step 8's claim (plan.md:55, `describe.rs` and `json.rs` need no change) — no master site, a plan line → routed to P3 curation (report Decisions item 7).
- Boundary widening: none. `advance` keeps its validation (`ms` 1..=60000), carriers, fields and exits; a refusal leaving the set narrows it.
- PROVISIONAL: every master site stating a delegate answer's fact is marked PROVISIONAL, for the founder's epoch batch, with no card (inputs#I4 (a)). No mark is removed.
- Route (P5): the test-plan note's coverage gap — the sub-document's carried clock (`iframe.rs` 75), pan and the fling (`pointer.rs` 266 and 713, `scrolling.rs` 865) read on no check under the virtual clock — gets a route owner at P5.

Escalations: 0.
