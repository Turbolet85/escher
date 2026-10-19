proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the Recorded check (escher-driver `Check::from_bytes`) row"
    change: "The closed record keys become `tool` · `arguments` · `masked` · `accepted` · `digest` · `answer`, `answer` required and a JSON object (the kept line inlined, written after `digest`; missing, or a string, number, array or `null`, refused `Cause::Malformed`; key order not enforced, read as a map), the clause «an `answer` key refused until «Recorded-check replay»» dropped, the refusal rows 35 → 40 (read rows 6 → 8), the bounds (400 records, 1,048,576 bytes) unchanged; and, since 2026-10-19-recorded-check-replay, the replay built — `Check::replay(&self, &mut Session) -> Replay` runs each kept step's `call()` through `Session::run` alone, so every step is validated again, a session whose `label()` is not the check's app runs nothing, the run stops at the first step whose acceptance or answer differs, and `Replay` · `Change` hold bools, counts and closed schema words only (no reading, no `serde_json` type); the reader also reads the two committed checks under `tests/blitz-tests/checks/` (`stand_replay`)."
    sidecar: "security-plan §Input Validation, Recorded check row: closed keys gain the required object `answer` (F3 B), refusal rows 35 → 40, and `Check::replay` re-validating every kept step through `Session::run` (2026-10-19-recorded-check-replay)."
    rationale: "Report Changes → Symbols / APIs (the reader: `step_of`'s closed key list adds `answer`, required and an object, else `Malformed`; the replay through `Session::run`), Deviations 4 (refused table 35 → 40, read table 6 → 8), Counts `record-keys` (`security-plan.md:81` ×2, stale) and `owner-title` (`security-plan.md:81`, discharged by this chunk); Expected amendments name the Recorded check row."
    basis: "packages/escher-driver/src/recording.rs:412-423; packages/escher-driver/src/recording.rs:443; packages/escher-driver/src/replay.rs:41-69"
  - detector: D-security-input
    severity: escalate
    section: "§Data Protection → At rest — per medium → Files (kept checks)"
    change: "Per answered call a kept check holds its verb, its arguments, its acceptance, the FNV-1a 64 digest of its whole written answer (a shot's PNG included) and, since 2026-10-19-recorded-check-replay, `answer`: the answer's line as the driver wrote it, masked as on every carrier (a password's or file input's value as `MASKED_VALUE`, a masked or unresolved `type`'s text as `MASKED_STAND_IN`), a shot's as `{\"width\",\"height\"}` with no `png` — so a kept check now holds the screens' ids, names, values, snapshot text and diffs at rest and no pixel; «no screen reading (no snapshot text, name, value, diff or pixel)» and «owed at «Recorded-check replay»» retired; still never logged, owner-only (`0600` in a `0700` directory) — a boundary widening ratified by the founder (F3 B, 2026-10-19-session-recording inputs#I4; this chunk's inputs#I1)."
    sidecar: "security-plan §Data Protection, Files (kept checks): each kept answer whole at rest, a shot's pixels excepted, masked as on every carrier — F3 B discharged (2026-10-19-recorded-check-replay)."
    rationale: "Report Changes → Schema / config (every step's `answer`, «A kept check now holds the screens' ids, names, values, snapshot text and diffs at rest; it held none before»), Counts `reading` / `at-rest` / `digest` / `owner-title` (`security-plan.md:192`, stale), Expected amendments (§Data Protection At rest, Files (kept checks))."
    basis: "packages/escher-driver/src/recording.rs:55-60; packages/escher-driver/src/recording.rs:108-113; packages/escher-driver/src/recording.rs:134-136"
  - detector: D-security-input
    severity: escalate
    section: "§Data Protection → At rest — per medium (a new bullet after Files (kept checks))"
    change: "Add «Files (committed checks)»: since 2026-10-19-recorded-check-replay two kept checks are committed to the public repository, `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` (git mode `100644`), the stated exception to owner-only — the repository's own scripted stand Counter sessions, written by `examples/seven_guis/tests/flows/record_checks.sh` (POSIX `sh`, its scratch made `mkdir -m 700`, printing only `step N` on failure), holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name or secret (read by the operator, inputs#I3), no `screenshot` step since `f15c28fc` (a shot's PNG bytes differ by OS, CI#39225375577); read only through the strict `Check::from_bytes` (`stand_replay`) and compared byte for byte with a fresh recording (seven_guis `recording::the_committed_checks_are_what_the_binary_records`) — a boundary widening ratified by the founder (F1 A, «Коммитить две проверки (Рекомендую)», inputs#I2)."
    sidecar: "security-plan §Data Protection: the two committed Counter checks under `tests/blitz-tests/checks/` recorded as the founder's F1 A exception to owner-only, read by `Check::from_bytes` alone (2026-10-19-recorded-check-replay)."
    rationale: "Report Changes → Files (New: the two checks and `record_checks.sh`), Schema / config (Two committed checks in a public repository, F1 A inputs#I2, content and the no-shot narrowing), Reverted (the screenshot step removed at `f15c28fc`), Coverage of new surfaces (`counter-{cli,mcp}.json` validated by `Check::from_bytes`); Expected amendments name this exception; pattern `checks-dir` 0 master sites, so no master states it yet."
    basis: "tests/blitz-tests/checks/counter-cli.json:1-8; tests/blitz-tests/checks/counter-mcp.json:1-11; examples/seven_guis/tests/flows/record_checks.sh:21-26"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the Markup attributes `id` (stable element id) row"
    change: "Replace «no log, event or file carries the snapshot or its text — a kept check (since 2026-10-19-session-recording) holds a recorded session's calls, the ids they named among them, and its answers' digests, never the snapshot or its text» with: no log or event carries the snapshot or its text, and the one file that does is a kept check — since 2026-10-19-session-recording a recorded session's calls, the ids they named among them, and since 2026-10-19-recorded-check-replay each answer kept whole (a `snapshot`'s text and every diff included, a shot's pixels excepted), owner-only and never logged (F3 B)."
    sidecar: "security-plan §Input Validation `id` row: a kept check now carries the snapshot text and diffs at rest, its answers kept whole (2026-10-19-recorded-check-replay)."
    rationale: "Report Counts pattern `reading`: `security-plan.md:125` @c6510 stale; Expected amendments name «the «no log, event or file carries…» sentence»."
    basis: "packages/escher-driver/src/recording.rs:134-136"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the Markup attributes `id` (stable element id) row"
    change: "Restate the id's fourth exit at rest («a kept check holding the ids a recorded session's calls named») as a kept check holding the ids a recorded session's calls named and, since 2026-10-19-recorded-check-replay, every id its kept answers read (snapshot text, diffs) — owner-only, and in the two committed Counter checks under `tests/blitz-tests/checks/` in git (F3 B, F1 A)."
    sidecar: "security-plan §Input Validation `id` row: the id's at-rest exit widened to the ids a kept answer reads, and the committed checks (2026-10-19-recorded-check-replay)."
    rationale: "Report Counts pattern `at-rest`: `security-plan.md:125` @c7353 stale (what a kept check holds at rest); Schema / config (answer kept whole; the committed checks)."
    basis: "packages/escher-driver/src/recording.rs:134-136"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the Markup attributes `id` (stable element id) row"
    change: "In the `unkeyed_actionable` clause's list of the id's exits, «since 2026-10-19-session-recording, a kept check's call targets» becomes a kept check's call targets and, since 2026-10-19-recorded-check-replay, its kept answers' ids."
    sidecar: "security-plan §Input Validation `id` row: the exits list's kept-check clause includes the kept answers' ids (2026-10-19-recorded-check-replay)."
    rationale: "Same at-rest claim restated later in the row (report Counts `at-rest`, `security-plan.md:125`); with the answer kept whole a kept check holds more ids than its call targets (Changes → Schema / config)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the Markup attributes accessible name row"
    change: "«neither the snapshot nor its text reaches a log, event or file» becomes: neither reaches a log or event, and a file only as a kept check's kept answers — since 2026-10-19-recorded-check-replay each answer kept whole, names in the snapshot text and diffs included, owner-only and never logged (F3 B), and the two committed Counter checks in git (F1 A)."
    sidecar: "security-plan §Input Validation accessible-name row: names now reach a file at rest through a kept check's answers (2026-10-19-recorded-check-replay)."
    rationale: "A restatement of the retired «no file carries the snapshot or its text» claim (report Counts `reading`, Changes → Schema / config: a kept check now holds names at rest); the report gives no line for this row."
    basis: ".andromeda/security-plan.md (accessible-name row)"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the Markup attributes password and file `input` value row"
    change: "«which, like the snapshot, reaches no log, event or file» becomes: reaches no log or event and a file only as a kept check's kept answer (since 2026-10-19-recorded-check-replay), where the mask holds — a value in it reads `MASKED_VALUE`, as on every carrier, beside the `type` stand-in `MASKED_STAND_IN` (sentinel 0)."
    sidecar: "security-plan §Input Validation password/file value row: the masked text now reaches a kept check's answers at rest, masked (2026-10-19-recorded-check-replay)."
    rationale: "A restatement of the retired «no file carries the snapshot text» claim; report Changes → Schema / config («masked as on every carrier»), Coverage of new surfaces (the `answer` key: PII redacted, a password's or file input's value as the mask, sentinel 0); the report gives no line for this row."
    basis: ".andromeda/security-plan.md (password and file `input` value row)"
    dependent-of: D-security-input
note: "D-security-auth and D-security-deps hold — no identity/token/key handling touched and no dependency added (report Dependencies: none, no Cargo.toml/Cargo.lock). §Dependency Security's serde_json bullet lists its readers as MCP stdin and a kept check's bytes; the replay's private `change_of` (replay.rs:72-93) adds a third read site (`serde_json::from_str` on the driver's own kept line, read-only) — still «writing nothing», so true as written; the orchestrator may extend the reader list."
