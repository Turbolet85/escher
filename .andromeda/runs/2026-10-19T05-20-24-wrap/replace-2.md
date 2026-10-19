- line: .andromeda/architecture.md:57
  old: «taken by escher-driver alone as its reader of MCP requests (packages/escher-driver/Cargo.toml:17)»
  new: «taken by escher-driver alone as its reader of MCP requests and of kept checks (packages/escher-driver/Cargo.toml:17)»
- line: .andromeda/architecture.md:115
  old: «a reader of MCP requests only that writes nothing»
  new: «a reader of MCP requests and kept checks only that writes nothing»
- line: .andromeda/architecture.md:175
  old: «the crate re-exports 45 names under»
  new: «the crate re-exports 49 names under»
- line: .andromeda/architecture.md:175
  old: «the self-description's `describe` from private modules, and `Busy`»
  new: «the self-description's `describe` and the recording's `Check`, `Step`, `answer_digest` and `MASKED_STAND_IN` from private modules, and `Busy`»
- line: .andromeda/architecture.md:175
  old: «the session holds a label, the harness, that optional step and one private record of the ids its screens have read»
  new: «the session holds a label, the harness, that optional step, one private record of the ids its screens have read and the optional crate-private recording `recording_into` sets»
- line: .andromeda/architecture.md:175
  old: «`NotSettled(Busy)` · `AnswerTooLarge` — thirteen, the last since 2026-10-10-driver-cli —»
  new: «`NotSettled(Busy)` · `AnswerTooLarge` · `RecordDirNotPrivate` · `RecordingFailed` — fifteen —»
- line: .andromeda/architecture.md:175
  old: «`already-running` through `answer-too-large`)»
  new: «`already-running` through `recording-failed`)»
- line: .andromeda/architecture.md:175
  old: «`SESSION_ERRORS: [ErrorSpec; 13]`»
  new: «`SESSION_ERRORS: [ErrorSpec; 15]`»
- line: .andromeda/architecture.md:175
  old: «The wire is crate-private and versioned `v2` since 2026-10-10-driver-cli:»
  new: «The wire is crate-private and versioned `v3`:»
- line: .andromeda/architecture.md:175
  old: «requests `hello v2`, `stop v2` and `call v2 <verb>»
  new: «requests `hello v3`, `stop v3` and `call v3 <verb>»
- line: .andromeda/architecture.md:175
  old: «replies `ok v2 pid={pid} label={label}»
  new: «replies `ok v3 pid={pid} label={label}»
- line: .andromeda/architecture.md:175
  old: «`ok v2 stopping`, `refused version` (a known word with another version)»
  new: «`ok v3 stopping`, `refused version` (a known word with another version)»
- line: .andromeda/architecture.md:175
  old: «to a call, `ok v2 accepted <json>`, `ok v2 refused <json>` or `ok v2 oversize`, an answer bounded»
  new: «to a call, `ok v3 accepted <json>`, `ok v3 refused <json>`, `ok v3 oversize` or `ok v3 recording-failed` — sent in place of the answer when the host's recording ended after the call, the client reading it as `RecordingFailed` — an answer bounded»
- line: .andromeda/architecture.md:175
  old: «`call` return `Unsupported`; a `Session` held in process works on every target.»
  new: «`call` return `Unsupported`, and so does `Session::recording_into`; a `Session` held in process without a recording works on every target.»
- line: .andromeda/architecture.md:175
  old: «reads MCP requests and writes nothing (the founder, 2026-10-10,»
  new: «reads MCP requests and kept checks and writes nothing (the founder, 2026-10-10,»
- line: .andromeda/architecture.md:175
  old: «`serve <app> --session <dir>` in exactly that shape, else usage; since 2026-10-10-mcp-surface `mcp` alone, else usage»
  new: «`serve <app> --session <dir>` in exactly that shape or with one `--record <dir>` after it, else usage; since 2026-10-10-mcp-surface `mcp` alone or with one `--record <dir>`, else usage; a `--record` anywhere else, twice or with no value is usage once the call passed `validate`»
- line: .andromeda/architecture.md:175
  old: «and the usage line names it (`; mcp role: mcp`);»
  new: «and the usage line names it (`; mcp role: mcp [--record <dir>]`), with `start <app> [--record <dir>]` and `host role: serve <app> --session <dir> [--record <dir>]`; `start --record` and `mcp --record` check the recording directory first, a refusal ending status `3` with nothing spawned or served;»
- line: .andromeda/architecture.md:175
  old: «as `serve <app> --session <dir>` with stdin, stdout and stderr closed»
  new: «as `serve <app> --session <dir>`, with `--record <dir>` after it when it was given one, with stdin, stdout and stderr closed»
- line: .andromeda/architecture.md:175
  old: «That record is the one thing the session keeps between calls:»
  new: «That record is the one thing of its screens the session keeps between calls:»
- line: .andromeda/architecture.md:175
  old: «4 exit statuses, 13 session errors (10 verbs»
  new: «4 exit statuses, 15 session errors (10 verbs»
- line: .andromeda/architecture.md:175
  old: «packages/escher-driver/src/mcp/result.rs:15-42»
  new: «packages/escher-driver/src/command.rs:32-60»
- line: .andromeda/architecture.md:178
  old: «— and no log, event or file carries the snapshot or its text;»
  new: «— and no log or event carries the snapshot or its text, nor does a kept check, which holds a recorded session's calls and its answers' digests alone;»
- line: .andromeda/architecture.md:180
  old: «`start <task>`, `status`, `stop` and»
  new: «`start <task> [--record <dir>]`, `status`, `stop` and»
- line: .andromeda/architecture.md:180
  old: «and the host role is `serve <task> --session <dir>`, which the `start` command spawns;»
  new: «and the host role is `serve <task> --session <dir> [--record <dir>]`, which the `start` command spawns;»
- line: .andromeda/architecture.md:180
  old: «the `mcp` role is `mcp` alone, an MCP server»
  new: «the `mcp` role is `mcp [--record <dir>]`, an MCP server»
- line: .andromeda/architecture.md:183
  old: «`host_timer`, `mcp_server` and `mcp_log`; the first red command ends the leg»
  new: «`host_timer`, `mcp_server`, `mcp_log` and `recording`; the first red command ends the leg»
- line: .andromeda/architecture.md:188
  old: «serving three requests at wire version `v2` —»
  new: «serving three requests at wire version `v3`, a call's replies including the fixed `recording-failed` —»
- line: .andromeda/architecture.md:191
  old: «under `CARGO_TARGET_TMPDIR`, i.e. `target/tmp/`, local only and uploaded by no CI leg»
  new: «under `CARGO_TARGET_TMPDIR`, i.e. `target/tmp/`, beside the recording checks' scratch directories and escher-driver's unit tests', which reach the workspace `target/tmp` through the compile-time `CARGO_MANIFEST_DIR`, local only and uploaded by no CI leg»
- line: .andromeda/architecture.md:192
  old: «and seven seven_guis test targets spawn the built `escher-session` binary»
  new: «and eight seven_guis test targets spawn the built `escher-session` binary»
- line: .andromeda/architecture.md:192
  old: «spoken to over piped stdin — the seven sharing the helpers of»
  new: «spoken to over piped stdin, and `recording` does both, with `--record` — the eight sharing the helpers of»
- line: .andromeda/architecture.md:297
  old: «its three types and the `MASKED_VALUE` constant re-exported)»
  new: «its three types, the `MASKED_VALUE` constant and `is_masked_input` re-exported)»
- line: .andromeda/architecture.md:300
  old: «fourteen private modules —»
  new: «fifteen private modules —»
- line: .andromeda/architecture.md:300
  old: «its crate-private table of the thirteen errors' kinds»
  new: «its crate-private table of the fifteen errors' kinds»
- line: .andromeda/architecture.md:300
  old: «`SessionCommand`, `validate_session`), refusal»
  new: «`SessionCommand`, `validate_session`, and the crate-private `call_of` that turns JSON arguments into a call), refusal»
- line: .andromeda/architecture.md:300
  old: «result (a tool call's arguments read into a call, its result and its written form)»
  new: «result (a tool call's result and its written form)»
- line: .andromeda/architecture.md:300
  old: «json the shot's line and mcp its image block) — re-exported at the crate root»
  new: «json the shot's line and mcp its image block) and recording (the crate-private writer of a kept check and its directory check, unix only, with `Check`, `Step`, `answer_digest` and `MASKED_STAND_IN`; json gaining a signed `integer` writer for it) — re-exported at the crate root»
- line: .andromeda/architecture.md:305
  old: «read through `mod session_common;` by four of the session checks,»
  new: «read through `mod session_common;` by five of the session checks,»
- line: .andromeda/architecture.md:305
  old: «tests/blitz-tests/tests/stand_session_quiet.rs:1-7); settle on a held instance»
  new: «tests/blitz-tests/tests/stand_session_quiet.rs:1-7); a recorded session — `stand_session_recording`: the four lean tasks recorded, read back and run again with every step's acceptance and digest equal in both layout modes, a typed password kept as the stand-in, a recording directory open to its group refused with nothing written (tests/blitz-tests/tests/stand_session_recording.rs:126-343); settle on a held instance»
- line: .andromeda/architecture.md:306
  old: «and since 2026-10-10-mcp-surface `tests/mcp_server.rs` and `tests/mcp_log.rs` — with their shared module»
  new: «and since 2026-10-10-mcp-surface `tests/mcp_server.rs` and `tests/mcp_log.rs`, and `tests/recording.rs` — with their shared module»
- line: .andromeda/architecture.md:306
  old: «and the crate's seven integration-test targets»
  new: «and the crate's eight integration-test targets»
