## 2026-10-19-session-recording — wire v3, fifteen session errors, the recording module and the eighth seven_guis target
**Section:** §Stack and Technologies → Media · §Established Decisions → [Driver session] · §Standard Contracts → Driver session, CI contracts · §Occupied Resources → Network ports and listeners, Filesystem, Process-wide state and threads · §Infrastructure Patterns · §Existing Scopes → dioxus-native-dom, escher-driver, blitz-tests, seven_guis
**Change:**
- The wire: was `v2`; now `v3` — every request and reply, and one fixed reply, `ok v3 recording-failed`, sent in place of an answer and read as `RecordingFailed`; the listener row and «a call travels as a `call v3` request line» the same.
- `SessionError`: was thirteen variants; now fifteen (`RecordDirNotPrivate`, `RecordingFailed`), `SESSION_ERRORS: [ErrorSpec; 15]`, `describe()` serving 15 session errors; the earlier-reading parenthesis «10 verbs and 8 causes until …» left the body.
- `serde_json`: was «a reader of MCP requests only»; now of MCP requests and kept checks, still writing nothing — the Media row, the decision and the json contract.
- `call_of` is cited at `command.rs:32-60` (was `mcp/result.rs:15-42`). The escher-driver row: was fourteen private modules; now fifteen (`recording`), the error table fifteen, command holding the crate-private `call_of`, result no longer reading arguments, json gaining a signed `integer` writer.
- The dioxus-native-dom row re-exports `is_masked_input`.
- stand-repeat's seven_guis list gains `recording`; was seven seven_guis targets spawning `escher-session`, now eight; the seven_guis row lists `tests/recording.rs`; the blitz-tests row registers `stand_session_recording`, `session_common` read by five session checks.
- Filesystem: the recording checks' and escher-driver's unit tests' scratch under `target/tmp`, through the compile-time `CARGO_MANIFEST_DIR` (test-only).
- Infrastructure Patterns: `start` spawns the host with `--record <dir>` after it when given one.
**Why:** this chunk's own shipped facts (the playbook's «Accurate this-chunk addition»); the reply word and the two errors' fixed text sit inside ratified carriers («A field inside a ratified carrier», «The program's own fixed text inside a ratified carrier»), and the test-only scratch is no crossing.
**Ref:** .andromeda/runs/2026-10-19T05-20-24-wrap/
