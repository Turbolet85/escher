- line: .andromeda/security-plan.md:81
old: «then records with the closed keys `tool` · `arguments` · `masked` · `accepted` · `digest` only,»
new: «then records with the closed keys `tool` · `arguments` · `masked` · `accepted` · `digest` · `answer` only,»
- line: .andromeda/security-plan.md:81
old: «`digest` 16 lowercase hex (an `answer` key refused until «Recorded-check replay»);»
new: «`digest` 16 lowercase hex and, since 2026-10-19-recorded-check-replay, `answer` required and a JSON object (missing, a string, a number, an array or `null` refused; key order not enforced, read as a map);»
- line: .andromeda/security-plan.md:81
old: «(35 refusal rows and edge rows at both bounds)»
new: «(40 refusal rows, 8 read rows and edge rows at both bounds)»
- line: .andromeda/security-plan.md:81
old: «so a replayed step still passes `validate` inside `Session::run` (packages/escher-driver/src/recording.rs:321-351) |»
new: «so a replayed step still passes `validate` inside `Session::run` — since 2026-10-19-recorded-check-replay `Check::replay` runs each kept step's call through `Session::run` alone, nothing on a session of another app, and stops at the first step whose acceptance or kept answer differs, its `Replay` and `Change` holding bools, counts and closed schema words only; the reader also reads the two committed checks under `tests/blitz-tests/checks/` (`stand_replay`) (packages/escher-driver/src/recording.rs:321-351; packages/escher-driver/src/replay.rs:40-70) |»
- line: .andromeda/security-plan.md:125
old: «and no log, event or file carries the snapshot or its text — a kept check (since 2026-10-19-session-recording) holds a recorded session's calls, the ids they named among them, and its answers' digests, never the snapshot or its text;»
new: «and no log or event carries the snapshot or its text — the one file that does is a kept check: since 2026-10-19-session-recording a recorded session's calls, the ids they named among them, and since 2026-10-19-recorded-check-replay each answer kept whole, a `snapshot`'s text and every diff included, a shot's pixels excepted, owner-only and never logged but for the two committed stand checks in git (ratified by the founder, 2026-10-19: F3 B, F1 A);»
- line: .andromeda/security-plan.md:125
old: «a fourth at rest, a kept check holding the ids a recorded session's calls named (ratified by the founder, 2026-10-19)»
new: «a fourth at rest, a kept check holding the ids a recorded session's calls named and, since 2026-10-19-recorded-check-replay, every id its kept answers read — owner-only, and in the two committed Counter checks under `tests/blitz-tests/checks/` in git (ratified by the founder, 2026-10-19)»
- line: .andromeda/security-plan.md:125
old: «a kept check's call targets;»
new: «a kept check's call targets and, since 2026-10-19-recorded-check-replay, its kept answers' ids;»
- line: .andromeda/security-plan.md:126
old: «and neither the snapshot nor its text reaches a log, event or file (ratified by the founder, 2026-10-07, as the `id` row records);»
new: «and neither the snapshot nor its text reaches a log or event, and a file only as a kept check's kept answers — since 2026-10-19-recorded-check-replay each answer kept whole, names in the snapshot text and diffs included, owner-only and never logged, and the two committed Counter checks in git (ratified by the founder, 2026-10-07 and 2026-10-19, as the `id` row records);»
- line: .andromeda/security-plan.md:133
old: «which, like the snapshot, reaches no log, event or file (ratified by the founder, 2026-10-07, as the `id` row records);»
new: «which reaches no log or event and a file only as a kept check's kept answer (since 2026-10-19-recorded-check-replay), where the mask holds — a value in it reads `MASKED_VALUE`, as on every carrier, beside the `type` stand-in `MASKED_STAND_IN` (ratified by the founder, 2026-10-07 and 2026-10-19, as the `id` row records);»
- line: .andromeda/security-plan.md:192
old: «a JSON array: the app label, then per answered call its verb, its arguments, its acceptance and a FNV-1a 64 digest of its written answer — holding call content»
new: «a JSON array: the app label, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer (a shot's PNG included) and, since 2026-10-19-recorded-check-replay, the answer itself as `answer` — the line the driver wrote, masked as on every carrier (a password's or file input's value as `MASKED_VALUE`), a shot's as `{"width","height"}` with no `png` — holding call content»
- line: .andromeda/security-plan.md:192
old: «and no screen reading (no snapshot text, name, value, diff or pixel), never logged,»
new: «and the screens' readings at rest — ids, names, values, snapshot text and diffs, never a pixel — never logged,»
- line: .andromeda/security-plan.md:192
old: «the call-content class ratified by the founder (2026-10-19, F1 A and F2 B; the answers kept whole, a shot's pixels excepted, owed at «Recorded-check replay»)»
new: «the call-content class ratified by the founder (2026-10-19, F1 A and F2 B) and each answer kept whole, a shot's pixels excepted, ratified by the founder (2026-10-19, F3 B)»
- line: .andromeda/security-plan.md:254
old: «of a kept check's bytes (`Check::from_bytes`), writing nothing»
new: «of a kept check's bytes (`Check::from_bytes`) and, since 2026-10-19-recorded-check-replay, of the driver's own answer line the replay compares (`from_str` in `Check::replay`), writing nothing»
