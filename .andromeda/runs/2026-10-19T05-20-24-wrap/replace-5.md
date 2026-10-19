- line: .andromeda/security-plan.md:24
  old: «(`serve <task> --session <dir>`, since 2026-10-10-driver-cli; the two-argument form is gone)»
  new: «(`serve <task> --session <dir> [--record <dir>]`, since 2026-10-10-driver-cli, its `--record` since 2026-10-19-session-recording; the two-argument form is gone)»
- line: .andromeda/security-plan.md:78
  old: «at wire version `v2`: `hello v2`, `stop v2` and, since 2026-10-10-driver-cli, `call v2 <verb>»
  new: «at wire version `v3` (`v2` until 2026-10-19-session-recording): `hello v3`, `stop v3` and, since 2026-10-10-driver-cli, `call v3 <verb>»
- line: .andromeda/security-plan.md:78
  old: «answered `ok v2 accepted <json>` or `ok v2 refused <json>`;»
  new: «answered `ok v3 accepted <json>` or `ok v3 refused <json>`;»
- line: .andromeda/security-plan.md:78
  old: «over it the reply is `ok v2 oversize`, the call having run —»
  new: «over it the reply is `ok v3 oversize`, the call having run, and a call after which the host's recording ended is answered the fixed `ok v3 recording-failed` in place of its answer, the call having run and read by the client as `SessionError::RecordingFailed` —»
- line: .andromeda/security-plan.md:81
  old: «after `validate_session` (packages/escher-driver/src/mcp/result.rs:15-42;»
  new: «after `validate_session` (packages/escher-driver/src/command.rs:32-60;»
- line: .andromeda/security-plan.md:82
  old: «`serve <app> --session <dir>` in exactly that shape, else usage; since 2026-10-10-mcp-surface the first word `mcp` alone selects the MCP role, and `mcp` with any other word after it — `mcp x`, `mcp --session d` — is usage»
  new: «`serve <app> --session <dir>` in exactly that shape or, since 2026-10-19-session-recording, with one `--record <dir>` after it, else usage; since 2026-10-10-mcp-surface the first word `mcp` alone or with one `--record <dir>` selects the MCP role, and `mcp` with any other word after it — `mcp x`, `mcp --session d` — is usage; `--record <dir>` is taken at most once, on `start <app> --session <dir> --record <dir>`, the host role and the `mcp` role, and anywhere else, twice or with no value is usage, status 2, once the call passed `validate` (a refused call stays a refusal; a stray `--record` was an unnamed argument, status 1, before); `start` checks the recording directory before it spawns, a refusal status 3 with nothing spawned, and `mcp --record` before it serves, a refusal status 3 with its fixed message on stderr and stdout empty; the path is never written back»
- line: .andromeda/security-plan.md:82
  old: «the usage line naming the role (`; mcp role: mcp`);»
  new: «the usage line naming the role (`; mcp role: mcp [--record <dir>]`, beside `start <app> [--record <dir>]` and `host role: serve <app> --session <dir> [--record <dir>]`);»
- line: .andromeda/security-plan.md:82
  old: «as `serve <app> --session <dir>` with its three streams closed»
  new: «as `serve <app> --session <dir>`, passing on a given `--record <dir>`, with its three streams closed»
- line: .andromeda/security-plan.md:83
  old: «packages/escher-driver/src/mcp/framing.rs:146-176; packages/escher-driver/src/mcp/result.rs:15-42;»
  new: «packages/escher-driver/src/mcp/framing.rs:146-176; packages/escher-driver/src/command.rs:32-60;»
- line: .andromeda/security-plan.md:123
  old: «(`ok v2 accepted <json>`)»
  new: «(`ok v3 accepted <json>`)»
- line: .andromeda/security-plan.md:123
  old: «and no log, event or file carries the snapshot or its text;»
  new: «and no log, event or file carries the snapshot or its text — a kept check (since 2026-10-19-session-recording) holds a recorded session's calls, the ids they named among them, and its answers' digests, never the snapshot or its text;»
- line: .andromeda/security-plan.md:123
  old: «so escher's own code now gives the id three exits, the platform adapter,»
  new: «so escher's own code now gives the id three exits — and, since 2026-10-19-session-recording, a fourth at rest, a kept check holding the ids a recorded session's calls named (ratified by the founder, 2026-10-19) — the platform adapter,»
- line: .andromeda/security-plan.md:123
  old: «the id's exits being the platform adapter, since 2026-10-10-driver-cli the driver's answers and, since 2026-10-10-mcp-surface, MCP tool results;»
  new: «the id's exits being the platform adapter, since 2026-10-10-driver-cli the driver's answers, since 2026-10-10-mcp-surface MCP tool results and, since 2026-10-19-session-recording, a kept check's call targets;»
- line: .andromeda/security-plan.md:131
  old: «a file input's paint is unchanged — as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md |»
  new: «a file input's paint is unchanged — as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md; since 2026-10-19-session-recording a recorded `type` whose element resolves to a masked control — `dioxus_native_dom::is_masked_input`, now public, the test the snapshot's `value` reader calls, no reading changed — or to none (refused `stale` or `not-found`) is kept in the check as `MASKED_STAND_IN` (`"masked"`) with `masked:true`, never the text, its length or a digest of it, while any other `type`'s text, one refused by `validate` included, is kept as it crossed (ratified by the founder, 2026-10-19: F1 A · F2 B); measured: 0 sentinel occurrences, and a secret of another length leaves byte-identical files (packages/dioxus-native-dom/src/snapshot.rs:226-233; packages/escher-driver/src/execute.rs:339-343; tests/blitz-tests/tests/stand_session_recording.rs:274-323) |»
- line: .andromeda/security-plan.md:208
  old: «(`call`, wire `v2`)»
  new: «(`call`, wire `v3` since 2026-10-19-session-recording, its replies including the fixed `ok v3 recording-failed`)»
- line: .andromeda/security-plan.md:251
  old: «the reader of the MCP server's untrusted stdin (`from_str::<Value>`), writing nothing»
  new: «the reader of the MCP server's untrusted stdin (`from_str::<Value>`) and, since 2026-10-19-session-recording, of a kept check's bytes (`Check::from_bytes`), writing nothing»
- line: .andromeda/security-plan.md:313
  old: «in a test (packages/blitz-vibey-script/tests/preact.rs:13)»
  new: «in a test (packages/blitz-vibey-script/tests/preact.rs:13), since 2026-10-19-session-recording in escher-driver's unit tests, which reach the workspace `target/tmp` through it (packages/escher-driver/src/mcp_tests.rs:605-606)»
- line: .andromeda/security-plan.md:357
  old: «sent on the session socket as `ok v2 refused <json>`»
  new: «sent on the session socket as `ok v3 refused <json>`»
- line: .andromeda/security-plan.md:357
  old: «the crate-private table `SESSION_ERRORS: [ErrorSpec; 13]`,»
  new: «the crate-private table `SESSION_ERRORS: [ErrorSpec; 15]` — `RecordDirNotPrivate` (`record-dir-not-private`) and `RecordingFailed` (`recording-failed`) its rows since 2026-10-19-session-recording, each exit `3`, so `help`'s `session_errors` serves 15 —»
