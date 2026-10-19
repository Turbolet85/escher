group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:20   class: stale
  leaf-text: «the wire is a closed line grammar at `v2` — `hello`, `stop` and, since 2026-10-10-driver-cli, `call v2 <verb> [<name>=<kind>:<value>]…`»
  source: .andromeda/security-plan.md:78 «at wire version `v3` (`v2` until 2026-10-19-session-recording): `hello v3`, `stop v3` and, since 2026-10-10-driver-cli, `call v3 <verb> [<name>=<kind>:<value>]…` — kinds `t` text · `n` whole number · »
  replace-with: «the wire is a closed line grammar at `v3` (`v2` until 2026-10-19-session-recording) — `hello`, `stop` and, since 2026-10-10-driver-cli, `call v3 <verb> [<name>=<kind>:<value>]…`»

- leaf: .claude/docs/security-summary.md:20   class: omission
  leaf-text: «a call's answer (at most 1,048,576 bytes, `oversize` past it)»
  source: .andromeda/security-plan.md:78 «over it the reply is `ok v3 oversize`, the call having run, and a call after which the host's recording ended is answered the fixed `ok v3 recording-failed` in place of its answer, the call having run»
  replace-with: «a call's answer (at most 1,048,576 bytes, `oversize` past it; since 2026-10-19-session-recording a call after which the host's recording ended is answered the fixed `ok v3 recording-failed` in place of its answer, the call having run, read by the client as `SessionError::RecordingFailed`)»

- leaf: .claude/docs/security-summary.md:21   class: omission
  leaf-text: «`start` re-runs the binary as `serve <task> --session <dir>` with its streams closed and neither kills nor reaps it»
  source: .andromeda/security-plan.md:84 «`--record <dir>` is taken at most once, on `start <app> --session <dir> --record <dir>`, the host role and the `mcp` role, and anywhere else, twice or with no value is usage, status 2, once the call p»
  replace-with: «`start` re-runs the binary as `serve <task> --session <dir>`, passing on a given `--record <dir>`, with its streams closed and neither kills nor reaps it; since 2026-10-19-session-recording `--record <dir>` is taken at most once, on `start`, the host role and the `mcp` role, and anywhere else, twice or with no value is usage, status 2, once the call passed `validate`; `start` checks the recording directory before it spawns and `mcp --record` before it serves (`0700` created when absent, a group- or other-open one refused `record-dir-not-private`, status 3, nothing written or spawned; ratified by the founder, 2026-10-19, F1 A), and the path is never written back»

- leaf: .claude/docs/security-summary.md:21   class: stale
  leaf-text: «since 2026-10-10-mcp-surface `mcp` alone selects the MCP role (any other shape after it is usage)»
  source: .andromeda/security-plan.md:84 «since 2026-10-10-mcp-surface the first word `mcp` alone or with one `--record <dir>` selects the MCP role, and `mcp` with any other word after it — `mcp x`, `mcp --session d` — is usage; `--record <di»
  replace-with: «since 2026-10-10-mcp-surface `mcp` alone or, since 2026-10-19-session-recording, with one `--record <dir>` selects the MCP role (any other shape after it is usage)»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «`serde_json` reads requests and writes nothing»
  source: .andromeda/security-plan.md:81 «Since 2026-10-19-session-recording a strict reader: at most 1,048,576 bytes, a JSON array of at most 400 records — the opening `{"tool":"start","arguments":{"app":…}}`, then records with the closed ke»
  replace-with: «`serde_json` reads requests and, since 2026-10-19-session-recording, a kept check's untrusted bytes (`Check::from_bytes`, a strict reader: at most 1,048,576 bytes, a JSON array of at most 400 records with closed keys, `tool` one of the seven instance verbs, anything else refused `malformed` holding nothing of the file, no panic path, each replayed step still passing `validate` inside `Session::run`), and writes nothing»

- leaf: .claude/docs/security-summary.md:32   class: omission
  leaf-text: «no log, event or file of escher's carries any of the three»
  source: .andromeda/security-plan.md:125 «no log, event or file carries the snapshot or its text — a kept check (since 2026-10-19-session-recording) holds a recorded session's calls, the ids they named among them, and its answers' digests, ne»
  replace-with: «no log, event or file of escher's carries any of the three — a kept check (since 2026-10-19-session-recording, under `--record <dir>` or `Session::recording_into`: one `check-{n}.json`, `0600`, created `create_new` and never replaced, in a `0700` launcher-named directory, never logged, unix only) holds a recorded session's calls — the ids they named, a fourth exit of the id, at rest (ratified by the founder, 2026-10-19), `press` keys, `advance` ms and text typed into unmasked controls — and an FNV-1a 64 digest of each answer, never a screen reading (no snapshot text, name, value, diff or pixel); a failed write or a passed bound (400 records, 1,048,576 bytes) ends the recording, its check removed»

- leaf: .claude/docs/security-summary.md:32   class: omission
  leaf-text: «so a password typed through the driver never leaves as pixels in a `screenshot`»
  source: .andromeda/security-plan.md:133 «since 2026-10-19-session-recording a recorded `type` whose element resolves to a masked control — `dioxus_native_dom::is_masked_input`, now public, the test the snapshot's `value` reader calls, no rea»
  replace-with: «so a password typed through the driver never leaves as pixels in a `screenshot`, and since 2026-10-19-session-recording a recorded `type` whose element resolves to a masked control (`dioxus_native_dom::is_masked_input`) or to none is kept in the check as `MASKED_STAND_IN` (`"masked"`) with `masked:true`, never the text, its length or a digest of it (ratified by the founder, 2026-10-19: F1 A · F2 B)»

- leaf: .claude/rules/security.md:21   class: stale
  leaf-text: «Its wire (`v2`) carries»
  source: .andromeda/security-plan.md:78 «at wire version `v3` (`v2` until 2026-10-19-session-recording): `hello v3`, `stop v3` and, since 2026-10-10-driver-cli, `call v3 <verb> [<name>=<kind>:<value>]…` — kinds `t` text · `n` whole number · »
  replace-with: «Its wire (`v3`, `v2` until 2026-10-19-session-recording) carries»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «an answer is bounded at 1,048,576 bytes (`oversize` past it — the call ran and is not rolled back)»
  source: .andromeda/security-plan.md:78 «over it the reply is `ok v3 oversize`, the call having run, and a call after which the host's recording ended is answered the fixed `ok v3 recording-failed` in place of its answer, the call having run»
  replace-with: «an answer is bounded at 1,048,576 bytes (`oversize` past it — the call ran and is not rolled back), and a call after which the host's recording ended is answered the fixed `ok v3 recording-failed` in place of its answer, the call having run»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «The next surface that carries any of it is a new crossing question.»
  source: .andromeda/security-plan.md:192 «- **Files (kept checks):** since 2026-10-19-session-recording, under `--record <dir>` on `escher-session` (`start`, the host role, the `mcp` role) or `Session::recording_into` in process, one `check-{»
  replace-with: «Since 2026-10-19-session-recording a kept check carries call content at rest (ratified by the founder, 2026-10-19, F1 A · F2 B): under `--record <dir>` (`start`, the host role, the `mcp` role) or `Session::recording_into`, one `check-{n}.json` (`0600`, `create_new`, never replaced) per recorded session in a `0700` launcher-named directory (a group- or other-open one refused `record-dir-not-private`, status 3, nothing written or spawned) — per answered call its verb, arguments (the ids named, `press` keys, `advance` ms, text typed into unmasked controls; a masked or unresolved `type`'s text as `MASKED_STAND_IN`, never the text, its length or a digest of it), acceptance and an FNV-1a 64 digest of its answer, no screen reading; never logged, no tool call, verb, argument or field names the directory; a failed write or a passed bound (400 records, 1,048,576 bytes) ends the recording, its check removed; a kept check's bytes are untrusted, read by the strict `Check::from_bytes` (closed keys, `tool` one of the seven instance verbs, else `malformed` holding nothing of the file, no panic path), each step still validated inside `Session::run`. The next surface that carries any of it is a new crossing question.»
