group: architecture+security-plan+test-plan
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:3   class: omission
  leaf-text: «the schema's and the command line's input rules are security-plan §Input Validation (the Driver command schema, Driver session socket and CLI arguments rows).»
  source: .andromeda/security-plan.md:80 «| Driver recording directory (escher-driver) | Launcher-named directory path (`--record <dir>`, `Session::recording_into`) and its mode | Since 2026-10-19-session-recording: created mode `0700`»
  replace-with: «the schema's and the command line's input rules are security-plan §Input Validation (the Driver command schema, Driver session socket and CLI arguments rows and, since 2026-10-19-session-recording, the Driver recording directory and Recorded check rows).»

- leaf: .claude/docs/services/escher-driver.md:6   class: omission
  leaf-text: «and since 2026-10-12-self-description the **self-description** (`describe`) that `help` answers on both surfaces.»
  source: .andromeda/architecture.md:119 «A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check — `std` only, no dependency added, unix only like the socket»
  replace-with: «and since 2026-10-12-self-description the **self-description** (`describe`) that `help` answers on both surfaces, and since 2026-10-19-session-recording the **recording**: a session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check (`std` only, unix only like the socket).»

- leaf: .claude/docs/services/escher-driver.md:6   class: omission
  leaf-text: «and an MCP client's stdio (ratified by the founder, 2026-10-10, «Ратифицировать как есть»).»
  source: .andromeda/security-plan.md:125 «a fourth at rest, a kept check holding the ids a recorded session's calls named (ratified by the founder, 2026-10-19)»
  replace-with: «and an MCP client's stdio (ratified by the founder, 2026-10-10, «Ратифицировать как есть»); since 2026-10-19-session-recording a recorded session's calls — the ids they named among them — rest in a kept check, with its answers' digests and no screen reading (ratified by the founder, 2026-10-19).»

- leaf: .claude/docs/services/escher-driver.md:11   class: omission
  leaf-text: «`dioxus-native-dom` (`DioxusDocument`, `SnapshotDiff`)»
  source: .andromeda/architecture.md:196 «`is_masked_input(&ElementData) -> bool`, public and re-exported beside `MASKED_VALUE`: true for an `input` whose `type` reads `password` or `file`, ASCII case-insensitive»
  replace-with: «`dioxus-native-dom` (`DioxusDocument`, `SnapshotDiff` and, since 2026-10-19-session-recording, `is_masked_input`, the test the recorder reads to keep a masked `type` as its stand-in)»

- leaf: .claude/docs/services/escher-driver.md:11   class: stale
  leaf-text: «the reader of MCP requests only (the founder, 2026-10-10, «serde_json, только чтение»)»
  source: .andromeda/architecture.md:57 «taken by escher-driver alone as its reader of MCP requests and of kept checks (packages/escher-driver/Cargo.toml:17)»
  replace-with: «the reader of MCP requests and, since 2026-10-19-session-recording, of kept checks (`Check::from_bytes`), writing nothing (the founder, 2026-10-10, «serde_json, только чтение»)»

- leaf: .claude/docs/services/escher-driver.md:12   class: stale
  leaf-text: «`serde_json` reads MCP requests and writes nothing.»
  source: .andromeda/security-plan.md:254 «the reader of the MCP server's untrusted stdin (`from_str::<Value>`) and, since 2026-10-19-session-recording, of a kept check's bytes (`Check::from_bytes`), writing nothing»
  replace-with: «`serde_json` reads MCP requests and, since 2026-10-19-session-recording, kept checks, and writes nothing.»

- leaf: .claude/docs/services/escher-driver.md:16   class: omission
  leaf-text: «A session started without it carries none.»
  source: .andromeda/architecture.md:186 «The recording: `Session::recording_into(self, dir: &Path) -> Result<Session, SessionError>` makes the directory owner-only or refuses it (`RecordDirNotPrivate`) and creates the session's kept check»
  replace-with: «A session started without it carries none. `Session::recording_into(self, dir: &Path) -> Result<Session, SessionError>` (since 2026-10-19-session-recording; unix only, `Unsupported` elsewhere) makes the directory owner-only or refuses it (`RecordDirNotPrivate`) and creates the session's kept check `check-{n}.json` (`0600`, `create_new`, never replacing a file), its opening record `{"tool":"start","arguments":{"app":<label>}}`; `run` — its signature and `command` span unchanged — then appends one record per answered call, `tool` · `arguments` in the verb's schema order · `masked` (`true`, on a `type` alone) · `accepted` · `digest`, and rewrites the check whole; a call no record can spell (a verb outside the seven instance verbs, an argument its schema does not name or names twice) is not kept; a `type` whose element resolves to a masked control (`dioxus_native_dom::is_masked_input`) or to none keeps `MASKED_STAND_IN` (`"masked"`) and `masked:true`, every other `type` its text as it crossed; no screen reading is kept. `answer_digest(&Result<Outcome, Refusal>) -> u64` is FNV-1a 64 over the answer's `to_json()`, written as 16 lowercase hex digits. Past 400 records or 1,048,576 bytes, or on a failed write, the recording ends — its check removed, the call not rolled back — and `Session::recording_failed(&mut self) -> bool` reads true once. `Check::from_bytes(&[u8]) -> Result<Check, Refusal>` reads a kept check strictly, anything else refused `Cause::Malformed` with no fault, nothing of the file held and no panic path, building each `Call` through `command::call_of`; `Check::{app, steps}` and `Step::{call, accepted, digest, masked}` read it.»

- leaf: .claude/docs/services/escher-driver.md:23   class: stale
  leaf-text: «`SessionError`, thirteen: `AlreadyRunning` · `NoSession` · `Dead` · `HostExited(code)` · `Timeout` · `Protocol` · `InvalidLabel` · `StateDirTooLong` · `StateDirNotPrivate` · `Unsupported` · `Io(kind)` · `NotSettled(Busy)` · `AnswerTooLarge` (a call's answer over the socket's bound — the call ran)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:6 «`RecordDirNotPrivate` (a recording directory open to its group or others) and `RecordingFailed` (a recording that ended, read from an `ok v3 recording-failed` reply) — fifteen session errors»
  replace-with: «`SessionError`, fifteen: `AlreadyRunning` · `NoSession` · `Dead` · `HostExited(code)` · `Timeout` · `Protocol` · `InvalidLabel` · `StateDirTooLong` · `StateDirNotPrivate` · `Unsupported` · `Io(kind)` · `NotSettled(Busy)` · `AnswerTooLarge` (a call's answer over the socket's bound — the call ran) · `RecordDirNotPrivate` (a recording directory open to its group or others) · `RecordingFailed` (a recording that ended, read from an `ok v3 recording-failed` reply; both since 2026-10-19-session-recording)»

- leaf: .claude/docs/services/escher-driver.md:23   class: stale
  leaf-text: «the crate-private `SESSION_ERRORS: [ErrorSpec; 13]`»
  source: .andromeda/architecture.md:182 «`SESSION_ERRORS: [ErrorSpec; 15]` of `ErrorSpec { kind, meaning, remedy, repeats }` (`repeats` is not served)»
  replace-with: «the crate-private `SESSION_ERRORS: [ErrorSpec; 15]`»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «9 causes and 13 session errors with their remedies, 4 exit statuses (10 verbs and 8 causes until 2026-10-12-headless-screenshot)»
  source: .andromeda/architecture.md:182 «3 busy classes, 9 causes, 4 exit statuses, 15 session errors — reading no environment and printing nothing»
  replace-with: «9 causes and 15 session errors with their remedies (13 until 2026-10-19-session-recording), 4 exit statuses»

- leaf: .claude/docs/services/escher-driver.md:27   class: stale
  leaf-text: «`result` (a tool call's arguments read into a call, its result and its written form)»
  source: .andromeda/architecture.md:328 «result (a tool call's result and its written form) — and its tests in `mcp_tests.rs`)»
  replace-with: «`result` (a tool call's result and its written form; since 2026-10-19-session-recording its arguments are read into a call by `command`'s crate-private `call_of`, the builder a kept check's reader shares)»

- leaf: .claude/docs/services/escher-driver.md:27   class: omission
  leaf-text: «a refusal, a refused shot included, or a session error one text block, its JSON with `isError: true`; no `structuredContent`.»
  source: .andromeda/architecture.md:186 «the MCP server answers `ToolResult::failed(SessionError::RecordingFailed)` (`isError`), its `tool_call` span keeping `tool` and `ending` and `serve_mcp` its signature.»
  replace-with: «a refusal, a refused shot included, or a session error one text block, its JSON with `isError: true`; no `structuredContent`. Since 2026-10-19-session-recording a call after which the session's recording ended is answered `ToolResult::failed(SessionError::RecordingFailed)` (`isError`), its `tool_call` span keeping `tool` and `ending` and `serve_mcp` its signature.»

- leaf: .claude/docs/services/escher-driver.md:28   class: stale
  leaf-text: «`serve <app> --session <dir>` in exactly that shape, else usage; `mcp` alone, else usage (since 2026-10-10-mcp-surface);»
  source: .andromeda/architecture.md:182 «`serve <app> --session <dir>` in exactly that shape or with one `--record <dir>` after it, else usage; since 2026-10-10-mcp-surface `mcp` alone or with one `--record <dir>`, else usage; a `--record` a»
  replace-with: «`serve <app> --session <dir>` in exactly that shape or with one `--record <dir>` after it, else usage; `mcp` alone or with one `--record <dir>`, else usage (since 2026-10-10-mcp-surface; `--record` since 2026-10-19-session-recording); a `--record` anywhere else, twice or with no value is usage once the call passed `validate`;»

- leaf: .claude/docs/services/escher-driver.md:28   class: stale
  leaf-text: «and the usage line names it (`; mcp role: mcp`).»
  source: .andromeda/architecture.md:182 «and the usage line names it (`; mcp role: mcp [--record <dir>]`), with `start <app> [--record <dir>]` and `host role: serve <app> --session <dir> [--record <dir>]`; `start --record` and `mcp --record`»
  replace-with: «and the usage line names it (`; mcp role: mcp [--record <dir>]`), with `start <app> [--record <dir>]` and `host role: serve <app> --session <dir> [--record <dir>]`; `start --record` and `mcp --record` check the recording directory first, a refusal ending status `3` with nothing spawned or served.»

- leaf: .claude/docs/services/escher-driver.md:28   class: stale
  leaf-text: «`start` spawns the running binary again as `serve <app> --session <dir>`, its streams closed,»
  source: .andromeda/architecture.md:182 «`start` spawns the running binary again (`std::env::current_exe()`) as `serve <app> --session <dir>`, with `--record <dir>` after it when it was given one, with stdin, stdout and stderr closed»
  replace-with: «`start` spawns the running binary again as `serve <app> --session <dir>`, with `--record <dir>` after it when it was given one, its streams closed,»

- leaf: .claude/docs/services/escher-driver.md:32   class: stale
  leaf-text: «The wire (`wire.rs`, crate-private, versioned `v2`) is one request line and one reply line per connection: `hello v2` → `ok v2 pid= label= served= idle_expiry_s=`, `stop v2` → `ok v2 stopping`, `call v2 <verb> [<name>=<kind>:<value>]…` → `ok v2 accepted <json>` · `ok v2 refused <json>` · `ok v2 oversize`,»
  source: .andromeda/architecture.md:182 «The wire is crate-private and versioned `v3`: one request line and one reply line per connection, `\n`-terminated — requests `hello v3`, `stop v3` and `call v3 <verb> [<name>=<kind>:<value>]…`»
  replace-with: «The wire (`wire.rs`, crate-private, versioned `v3`, `v2` until 2026-10-19-session-recording) is one request line and one reply line per connection: `hello v3` → `ok v3 pid= label= served= idle_expiry_s=`, `stop v3` → `ok v3 stopping`, `call v3 <verb> [<name>=<kind>:<value>]…` → `ok v3 accepted <json>` · `ok v3 refused <json>` · `ok v3 oversize` · `ok v3 recording-failed` (sent in place of the answer when the host's recording ended after the call, the client reading it as `RecordingFailed`),»

- leaf: .claude/docs/services/escher-driver.md:34   class: stale
  leaf-text: «Unix only: on other targets `serve`, `start`, `attach`, `stop` and `call` return `Unsupported`; a `Session` held in process works everywhere — `run` included.»
  source: .andromeda/architecture.md:182 «On a non-unix target `serve`, `start`, `attach`, `stop` and — for a call the schema admits — `call` return `Unsupported`, and so does `Session::recording_into`; a `Session` held in process without a r»
  replace-with: «Unix only: on other targets `serve`, `start`, `attach`, `stop` and `call` return `Unsupported`, and so does `Session::recording_into`; a `Session` held in process without a recording works everywhere — `run` included.»

- leaf: .claude/docs/services/escher-driver.md:37   class: stale
  leaf-text: «What the session keeps is a record of the ids its screens have read, to tell `stale` from `not-found`:»
  source: .andromeda/architecture.md:182 «That record is the one thing of its screens the session keeps between calls: id text only — never a `NodeId`, a node or bounds — in a crate-private `SeenIds` behind one private field»
  replace-with: «The one thing of its screens the session keeps between calls is a record of the ids they have read, to tell `stale` from `not-found` (beside it, since 2026-10-19-session-recording, only the optional crate-private recording `recording_into` sets, which keeps calls and answers' digests, never a screen reading):»

- leaf: .claude/docs/services/escher-driver.md:46   class: stale
  leaf-text: «The one exception, since 2026-10-14-quality-gates, is the unix-only client unit test»
  source: .andromeda/security-plan.md:316 «since 2026-10-19-session-recording in escher-driver's unit tests, which reach the workspace `target/tmp` through it (packages/escher-driver/src/mcp_tests.rs:605-606)»
  replace-with: «Since 2026-10-19-session-recording escher-driver's unit tests reach the workspace `target/tmp` through the compile-time `env!("CARGO_MANIFEST_DIR")`; the other exception, since 2026-10-14-quality-gates, is the unix-only client unit test»

- leaf: .claude/docs/services/escher-driver.md:49   class: omission
  leaf-text: «the label, the record of ids, any argument,»
  source: .andromeda/security-plan.md:80 «no tool call, verb, argument or result field names the directory, and nothing of it is logged»
  replace-with: «the label, the record of ids, the recording, its directory or its kept check (since 2026-10-19-session-recording), any argument,»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «74 unit tests (`wire.rs` 12 · `session.rs` 4 · `error.rs` 5 · `refusal.rs` 3 · `schema_tests.rs` 6 · `command.rs` 7 · `execute.rs` 3 · `json.rs` 7 · `cli.rs` 5 · `client.rs` 3 · `mcp_tests.rs` 12 · `describe.rs` 2 · `image.rs` 5; `host.rs` none; 73 until»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «escher-driver's 87 unit tests (cli.rs 5 · client.rs 3 · command.rs 7 · describe.rs 2 · error.rs 5 · execute.rs 3 · image.rs 5 · json.rs 7 · mcp_tests.rs 14 · recording.rs 9 · refusal.rs 3»
  replace-with: «87 unit tests (`wire.rs` 12 · `session.rs` 6 · `error.rs` 5 · `refusal.rs` 3 · `schema_tests.rs` 6 · `command.rs` 7 · `execute.rs` 3 · `json.rs` 7 · `cli.rs` 5 · `client.rs` 3 · `mcp_tests.rs` 14 · `describe.rs` 2 · `image.rs` 5 · `recording.rs` 9; `host.rs` none; beside three off-unix tests the windows job alone runs; 74 until 2026-10-19-session-recording, 73 until»

- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «a hosted call, requests outside the grammar, and the idle expiry).»
  source: .andromeda/test-plan.md:27 «since 2026-10-19-session-recording a recorded session — `stand_session_recording` 3, declaring `mod session_common;`: a recorded session replays on each lean task in both layout modes with every step'»
  replace-with: «a hosted call, requests outside the grammar, and the idle expiry); since 2026-10-19-session-recording `stand_session_recording` (3: a recorded session replays on each lean task in both layout modes with every step's acceptance and digest equal and keeps no screen reading at rest; a password typed in a recorded session is kept as the stand-in, 0 sentinel occurrences and a secret of another length leaving byte-identical files; a recording directory open to its group is refused with nothing written).»

- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «`host_log` (a host on `crud` at `RUST_LOG=trace` while three calls run: one span line per call, no id, name or typed text, under both builds)»
  source: .andromeda/test-plan.md:185 «since 2026-10-19-session-recording `host_log` and `mcp_log` each also run a recording host and a recording `mcp` role at `RUST_LOG=trace` and read 0 of the recording needles»
  replace-with: «`host_log` (a host on `crud` at `RUST_LOG=trace` while three calls run: one span line per call, no id, name or typed text, under both builds; since 2026-10-19-session-recording a second test, a recording host reading 0 of the recording needles — the kept check's names and its directory's name — on stderr)»

- leaf: .claude/docs/services/escher-driver.md:63   class: stale
  leaf-text: «`mcp_log` (the role at `RUST_LOG=trace`: no id, name or typed text, one `tool_call` line per call, under both builds), not unix-gated but the comparison»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:10 «since 2026-10-19-session-recording a check that records is `cfg(unix)`-gated item by item, the in-process and `mcp` stdio ones included (seven_guis `mcp_log`'s recorded test, `recorded_needles`, `the_»
  replace-with: «`mcp_log` (the role at `RUST_LOG=trace`: no id, name or typed text, one `tool_call` line per call, under both builds; since 2026-10-19-session-recording a second test, a recording `mcp` role that logs nothing of its checks), not unix-gated but the comparison and, since that chunk, every check that records (`mcp_log`'s recorded test, `recorded_needles`, `the_check`)»

- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «and `cli_flow` (two POSIX `sh` flows, `counter.sh` and `flight.sh`);»
  source: .andromeda/test-plan.md:10 «since 2026-10-19-session-recording `tests/recording.rs` (3 tests, unix only, and 1 off-unix) — the command line and the `mcp` role keep the same check byte for byte, the state directory gone after `st»
  replace-with: «and `cli_flow` (two POSIX `sh` flows, `counter.sh` and `flight.sh`); since 2026-10-19-session-recording `recording` (3 tests, unix only, and 1 off-unix: the command line and the `mcp` role keep the same check byte for byte, the state directory gone after `stop`; a recording directory open to its group refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`; off unix, `--record` refused `Unsupported`);»
