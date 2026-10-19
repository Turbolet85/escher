# Fan-out results — 2026-10-19-session-recording (wrap 2026-10-19T05-20-24-wrap, resumed)

Snapshot before: `tree 7557d0a7faf39045db05d1751d9f78c3edd1e49f index 74e556ef87848dfb94655bc24487af8d65705b11 head 3cbb0eeef1d8f7c9a34784e70b2ee68ddd01f7d3`
Snapshot after: `tree 4523185211a2c150b0b6c722b9b3e1709acf3cfc index 74e556ef… head 3cbb0eee…` — `git diff-tree` printed exactly the seven
`A {run_dir}/fanout-{doc}.md` lines; index and head unchanged. No file needed a fix (no text before `proposals:`, entity
probe 0 in all seven), so no raw twin. Every receipt's count equals its file's items.

Card = on the Boundary widening answer card (`answer-card.md`). Rules cited by the playbook's pattern name.

## architecture: 25 proposals · fanout-architecture.md
note: the two escalate proposals are the at-rest widening; the 49 re-export count to confirm against lib.rs — confirmed
(45 names at base `93336e88`, 49 now: `Check`, `MASKED_STAND_IN`, `Step`, `answer_digest` added; read at
`packages/escher-driver/src/lib.rs:128-145` by main).
1. §Occupied Resources → Filesystem, the recording directory — escalate, check 1 «Boundary widening» → card
2. §Occupied Resources → Filesystem, the recording tests' scratch under `target/tmp` — apply, check 1 «Accurate this-chunk addition» (test-only, per security rules' 2026-10-15 addition)
3. §Standard Contracts → Driver session, the recording API and kept-check form — escalate, check 1 «Boundary widening» → card
4. §Standard Contracts → Driver session, `SessionError` 13 → 15 — apply, check 1 «The program's own fixed text inside a ratified carrier»
5. same, `[ErrorSpec; 15]` — apply, with 4
6. same, `describe()` 15 session errors — apply, with 4
7. same, wire `v3` and the `recording-failed` reply — apply, check 1 «A field inside a ratified carrier» (one fixed reply word, no new carrier, verb, argument, exit or log field)
8. §Occupied Resources → Network ports and listeners, wire `v3` — apply, with 7
9. §Standard Contracts → Driver session, `--record <dir>` on the command line — escalate, check 1 «Boundary widening» (a validated surface admits a new input) → card
10. §Standard Contracts → CLIs, `[--record <dir>]` on three shapes — escalate, with 9 → card
11. §Standard Contracts → Driver session, what a session holds gains its recording — escalate, with 3 → card
12. same, «the one thing the session keeps between calls» restated — escalate, with 3 → card
13. same, off-unix `Unsupported` list gains `recording_into` — apply, «Accurate this-chunk addition»
14. §Standard Contracts → Dioxus DOM bridge, public `is_masked_input` — apply, «Accurate this-chunk addition» (no reading changed)
15. §Existing Scopes → dioxus-native-dom, re-export `is_masked_input` — apply, with 14
16. §Standard Contracts → Dioxus DOM bridge, «no log, event or file…» restated — escalate, «Boundary widening» → card
17. §Existing Scopes → escher-driver, fifteen private modules — apply, «Accurate this-chunk addition»
18. §Standard Contracts → CI contracts, stand-repeat gains `recording` — apply, «Accurate this-chunk addition»
19. §Occupied Resources → Process-wide state, eight seven_guis targets — apply, «Accurate this-chunk addition»
20. §Existing Scopes → seven_guis, eight integration-test targets — apply, with 19
21. §Existing Scopes → blitz-tests, `stand_session_recording`, five session_common readers — apply, «Accurate this-chunk addition»
22. §Established Decisions → [Driver session], the kept check — escalate, «Boundary widening» → card
23. §Established Decisions → [Driver session], `serde_json` also reads kept checks — apply, «Accurate this-chunk addition» (the founder's «serde_json, только чтение» holds: it still writes nothing; no reversal)
24. §Standard Contracts → Driver session, the `serde_json` sentence — apply, with 23
25. §Stack and Technologies → Media, `serde_json` row — apply, with 23 (read at :57: «as its reader of MCP requests» is now partial; the report's no-change on it is superseded by this read)
26. (main, check 5 — citation `claim false`) §Standard Contracts → Driver session, `packages/escher-driver/src/mcp/result.rs:15-42` → `packages/escher-driver/src/command.rs:32-60` (`call_of` moved) — apply, «Accurate this-chunk addition»

## security-plan: 13 proposals · fanout-security-plan.md
note: D-security-auth and D-security-deps clean; two standing claims flagged (§Dependency Security's serde_json bullet,
§Secret Management's `CARGO_MANIFEST_DIR` bullet) — read by main, raised as 16 and 17.
1. §Input Validation, the kept-check reader row — escalate, «Boundary widening» (a new input class: a kept check's bytes) → card
2. §Input Validation, the recording directory row — escalate, «Boundary widening» → card
3. §Input Validation, CLI arguments row `--record` — escalate, «Boundary widening» → card
4. §Threat Model Summary, host role shape — escalate, with 3 → card
5. §Input Validation, socket row wire `v3` — apply, «A field inside a ratified carrier» (the detector's escalate grade is its own severity; its invariant — validation present — holds)
6. §Input Validation, id row hosted answer `v3` — apply, with 5
7. §API Security, wire `v3` — apply, with 5
8. §Error Handling, `v3` and 15 session errors — apply, with 5 and «The program's own fixed text inside a ratified carrier»
9. §Input Validation, password row, the masked `type` at rest — escalate, «Boundary widening» → card
10. §Input Validation, id row «no log, event or file…» restated — escalate, «Boundary widening» → card
11. §Input Validation, id row, a fourth exit: the kept check — escalate, with 10 → card
12. §Input Validation, id row, the second exit list — escalate, with 10 → card
13. §Data Protection, At rest — per medium, the kept check — escalate, «Boundary widening» → card
14. (main, check 5 — citation `claim false`) §Input Validation, MCP server row (:81) `mcp/result.rs:15-42` → `command.rs:32-60` — apply
15. (main, check 5 — citation `claim false`) §Input Validation, MCP row (:83) the same — apply
16. (main, raised from the note, read at :251) §Dependency Security, `serde_json` also the reader of kept checks, still writing nothing — apply, «Accurate this-chunk addition»
17. (main, raised from the note, read at :313) §Secret Management, `CARGO_MANIFEST_DIR` also read by escher-driver's unit tests for `target/tmp` (report Deviation 8; `packages/escher-driver/src/mcp_tests.rs:605-606`) — apply, «Accurate this-chunk addition»

## design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds; expected amendment 7 left to the orchestrator.
1. (main, check 5 — expected amendment 7) §Surface: cli §Component Patterns, the exit-3 class holds the two recording session errors, no new exit — apply, «Accurate this-chunk addition»

## layout-templates: 5 proposals · fanout-layout-templates.md
note: the fourth is a swept restatement the pattern did not fire on.
1. §Surface: cli → Primary screens, `escher-session` row gains `--record` and the kept check — escalate, «Boundary widening» → card
2. same, the `mcp` role `[--record <dir>]` — escalate, with 1 → card
3. same, the host role `[--record <dir>]`, `start` checking before it spawns — escalate, with 1 → card
4. same, the usage ending names a stray `--record` — escalate, with 1 → card
5. §Surface: desktop-native → Primary screens, host role shape — escalate, with 1 → card

## test-plan: 30 proposals · fanout-test-plan.md
note: D-tests-framework and D-tests-obs-harness hold; the stand log format's host-role shape moves on both sides (test 24 ↔ obs 8).
1. §3 → session-proof, the recording checks — apply, «Accurate this-chunk addition» (per-file counts re-derived from the report at apply)
2. §3 → session-wire `v3` — apply, «A field inside a ratified carrier»
3. §3 → session-binary `--record` — escalate, «Boundary widening» → card
4. §3 → session-start, `recording_into` and the kept check — escalate, «Boundary widening» → card
5. §3 → session-host, two new error edges — apply, «The program's own fixed text inside a ratified carrier»
6. §3 → session-checks, unix gates and the off-unix checks — apply, «Accurate this-chunk addition»
7. §1 → escher-driver 74 → 87 unit tests — apply, «Accurate this-chunk addition»
8. §1 → escher-driver error.rs fifteen — apply, with 7
9. §1 → escher-driver wire.rs `v3` — apply, with 7
10. §4 → escher-driver, the recorder's tests — apply, «Accurate this-chunk addition»
11. §4 → escher-driver wire.rs `v3` — apply, with 10
12. §4 → escher-driver error.rs fifteen — apply, with 10
13. §1 → dioxus-native-dom 72 → 73 — apply, «Accurate this-chunk addition»
14. §1 → blitz-dom (clock), the iframe clock test — apply, «Accurate this-chunk addition»
15. §1 → tests/blitz-tests `virtual_clock_engine` 5 — apply, «Accurate this-chunk addition»
16. §1 → tests/blitz-tests, `stand_session_recording` 3 — apply, «Accurate this-chunk addition»
17. §2 → Directory pattern 135 / 134 — apply, «Accurate this-chunk addition»
18. §2 → Directory pattern, 24 readers — apply, with 17
19. §3 → Crate-local helpers, `replay` · `kept` · `mode` — apply, «Accurate this-chunk addition»
20. §1 → apps, seven_guis eight targets — apply, «Accurate this-chunk addition»
21. §1 → apps, `mod common;` read by all eight — apply, with 20
22. §1 → apps, `host_log` / `mcp_log` 2 each — apply, with 20
23. §1 → apps, host role shape — apply, with 20 (a shape restatement; the widening is carded at its owners)
24. §3 → Stand log format, host role shape, spans unchanged — apply, with 23
25. §5 → Session host ↔ what it writes, the recorded runs — apply, «Accurate this-chunk addition»
26. §6 → MCP, `mcp_log` 2 and `mcp --record` — apply, «Accurate this-chunk addition»
27. §6 → Command line, `--record` on `start` — apply, «Accurate this-chunk addition» (test coverage of the carded surface)
28. §9 → Legs, stand-repeat gains `recording` — apply, «Accurate this-chunk addition»
29. §9 → Legs, 42 → 43 files — apply, with 28
30. §10 → Flakiness budget 43 and eight — apply, with 28

## obs-plan: 11 proposals · fanout-obs-plan.md
note: D-obs-stack and D-obs-pii hold; the kept check is a file at rest, not a log.
1. §4, recording opens no span, both spans unchanged — apply, «Accurate this-chunk addition»
2. §3 Logging stack, the fifteenth module `recording` silent — apply, «Accurate this-chunk addition» (the at-rest content it names is the carded fact, stated at its owners; here the silence alone)
3. §3 Logging stack, `serde_json` also reads kept checks — apply, «Accurate this-chunk addition»
4. §3 Logging stack, 24 readers, five of six `stand_session_*` — apply, «Accurate this-chunk addition»
5. §3 Logging stack, six silent `stand_session_*` — apply, with 4
6. §3 Logging stack, the eighth seven_guis spawner — apply, «Accurate this-chunk addition»
7. §3 detail absence · logs, the recording checks' silence — apply, «Accurate this-chunk addition»
8. §6 escher-driver, wire `v3` — apply, «A field inside a ratified carrier»
9. §9 Telemetry artifact handling, the recording-directory row — escalate, «Boundary widening» → card
10. §6 examples, `escher-session` usage line and endings `--record` — apply, «Accurate this-chunk addition» (shape and endings; the widening carded at its owners)
11. §3 Logging stack, the `mcp` role with one `--record` — apply, with 10

## a11y-plan: 0 proposals · fanout-a11y-plan.md
note: no interactive UI element; schemas unmoved; the snapshot.rs coordinates are the citation sweep's (written: a11y-plan 8 citations).

## Checks across the set
- Rejected for a source the report does not carry: 0.
- Check 2 cross-contradiction: none (test 24 ↔ obs 8/10 move the same shape the same way).
- Check 3 intent-consistency: scope record none (`gate.py scope` clean, 0 recorded); the report's 13 deviations each carry their justification; none outside intent.
- Check 4 absence: the detectors' caught-all claims rest on the report's pre-pass sweep rows; lines over 2 000 chars read by offset at apply.
- Check 5 expected amendments: 1 → arch 1 · 2 → arch 3–12 · 3 → sec 1–3, 9–13 · 4 → obs 1, 9 · 5 → test 1–5, 28–30 · 6 → layout 1–5 · 7 → design 1 (raised). Citation `claim false` rows (3) → arch 26, sec 14, sec 15.
- Check 6 disproved claims: plan step 8 → disposed against the plan, its master fact applied by arch 6; the lifecycle «unchanged» note → carried by arch 7 / test 2 / sec 5; the folded CARRY's hypothesis → route, P5 (its CARRY line is archived by P7's compaction).
- Card: arch 1, 3, 9, 10, 11, 12, 16, 22 · sec 1, 2, 3, 4, 9, 10, 11, 12, 13 · layout 1–5 · test 3, 4 · obs 9 — 25 proposals; all others apply (65 — the 84 returned and the 6 main raised make 90).
