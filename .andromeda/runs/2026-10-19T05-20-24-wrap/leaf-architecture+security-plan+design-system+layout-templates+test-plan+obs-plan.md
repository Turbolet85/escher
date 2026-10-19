group: architecture+security-plan+design-system+layout-templates+test-plan+obs-plan
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`SessionError` (thirteen)»
  source: .andromeda/architecture.md:182 «`NotSettled(Busy)` · `AnswerTooLarge` · `RecordDirNotPrivate` · `RecordingFailed` — fifteen —»
  replace-with: «`SessionError` (fifteen — `record-dir-not-private` and `recording-failed` since 2026-10-19-session-recording)»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «wire `v2`»
  source: .andromeda/architecture.md:182 «The wire is crate-private and versioned `v3`: one request line and one reply line per connection»
  replace-with: «wire `v3` (a call's replies including the fixed `ok v3 recording-failed`)»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «the crate installs no subscriber.»
  source: .andromeda/architecture.md:186 «- The recording: `Session::recording_into(self, dir: &Path) -> Result<Session, SessionError>` makes the directory owner-only or refuses it (`RecordDirNotPrivate`) and creates the session's kept check»
  replace-with: «the crate installs no subscriber. A session its launcher hands a recording directory (`--record <dir>` on `start`, the host role and the `mcp` role; `Session::recording_into` in process, unix only, `Unsupported` elsewhere) keeps its answered calls as one owner-only replayable check — `check-{n}.json` (`0600`, `create_new`, never replacing a file) in a `0700` directory, refused `record-dir-not-private` when open to its group or others — each instance-verb call's `tool` · `arguments` · `accepted` and its answer's FNV-1a 64 `digest` (`answer_digest`), a masked or unresolved `type`'s text kept as `MASKED_STAND_IN`, no screen reading, never logged; past 400 records or 1,048,576 bytes, or on a failed write, the recording ends — its check removed, the call not rolled back, answered as the session error `recording-failed` (the socket's fixed `ok v3 recording-failed`, over MCP `isError`) — and `Check::from_bytes` reads a kept check strictly through `serde_json`, anything else refused `malformed` (ratified by the founder, 2026-10-19).»

- leaf: CLAUDE.md:38   class: omission
  leaf-text: «`start <task>`, the seven instance verbs, `status`, `stop`, and `help`, the tool's self-description, which reads no `--session` and reaches no session — and, in its host role `serve <task> --session <dir>`, hosts one of them as a driver session, the Timer's carrying that step, and in its `mcp` role `escher-session mcp` is an MCP server»
  source: .andromeda/layout-templates.md:73 «Since 2026-10-19-session-recording `--record <dir>`, taken at most once on `start`, the host role and the `mcp` role, keeps the session as one owner-only replayable check»
  replace-with: «`start <task> [--record <dir>]`, the seven instance verbs, `status`, `stop`, and `help`, the tool's self-description, which reads no `--session` and reaches no session — and, in its host role `serve <task> --session <dir> [--record <dir>]`, hosts one of them as a driver session, the Timer's carrying that step, and in its `mcp` role `escher-session mcp [--record <dir>]` is an MCP server»

- leaf: CLAUDE.md:83   class: omission
  leaf-text: «| Driver session (held instance · settled step · session socket and its wire · idle expiry · host role) | `packages/escher-driver/src/{session,host,client,wire,error}.rs` · host `examples/seven_guis/src/session_host.rs` · checks `tests/blitz-tests/tests/stand_session_*.rs` and `stand_settle.rs` (shared `tests/session_common/mod.rs`) and `examples/seven_guis/tests/host_{binary,log,timer}.rs`»
  source: .andromeda/architecture.md:328 «and recording (the crate-private writer of a kept check and its directory check, unix only, with `Check`, `Step`, `answer_digest` and `MASKED_STAND_IN`;»
  replace-with: «| Driver session (held instance · settled step · session socket and its wire · idle expiry · host role · session recording and its kept check) | `packages/escher-driver/src/{session,host,client,wire,error,recording}.rs` · host `examples/seven_guis/src/session_host.rs` · checks `tests/blitz-tests/tests/stand_session_*.rs` and `stand_settle.rs` (shared `tests/session_common/mod.rs`) and `examples/seven_guis/tests/{host_binary,host_log,host_timer,recording}.rs`»

- leaf: CLAUDE.md:100   class: omission
  leaf-text: «`start {task}`, then»
  source: .andromeda/architecture.md:201 «`scroll --id`, `start <task> [--record <dir>]`, `status`, `stop` and, since 2026-10-12-self-description, `help`»
  replace-with: «`start {task} [--record {dir}]` (a recording directory keeping the session as one owner-only replayable check), then»

- leaf: CLAUDE.md:101   class: omission
  leaf-text: «`target/debug/escher-session mcp`»
  source: .andromeda/architecture.md:201 «since 2026-10-10-mcp-surface the `mcp` role is `mcp [--record <dir>]`, an MCP server on the binary's standard streams»
  replace-with: «`target/debug/escher-session mcp [--record {dir}]`»

- leaf: CLAUDE.md:44   class: omission
  leaf-text: «no listener, ratified by the founder (2026-10-10) —»
  source: .andromeda/security-plan.md:125 «and, since 2026-10-19-session-recording, a fourth at rest, a kept check holding the ids a recorded session's calls named (ratified by the founder, 2026-10-19)»
  replace-with: «no listener, ratified by the founder (2026-10-10) — and, under `--record <dir>`, a kept check holds a recorded session's calls at rest — target ids, `press` keys, `advance` ms, text typed into unmasked controls, a masked or unresolved `type` as the stand-in — and its answers' digests, no screen reading, in an owner-only file never logged (ratified by the founder, 2026-10-19) —»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «the stand hosts its own through the `escher-session` binary)»
  source: .andromeda/architecture.md:119 «- A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check — `std` only, no dependency added, unix only like the socket»
  replace-with: «the stand hosts its own through the `escher-session` binary; since 2026-10-19-session-recording a session whose launcher names a recording directory (`--record`, `Session::recording_into`) keeps its answered calls as one owner-only replayable check — the calls as they crossed, a masked `type` as one fixed stand-in, each answer a 64-bit digest and no screen reading — read back strictly by `Check::from_bytes` and replayed with every step's acceptance and digest equal, ratified by the founder (2026-10-19))»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «a password or file input's value reads the fixed mask `MASKED_VALUE`»
  source: .andromeda/architecture.md:196 «- `is_masked_input(&ElementData) -> bool`, public and re-exported beside `MASKED_VALUE`: true for an `input` whose `type` reads `password` or `file`, ASCII case-insensitive»
  replace-with: «a password or file input's value reads the fixed mask `MASKED_VALUE` (`is_masked_input`, public since 2026-10-19-session-recording, the test that reader calls and escher-driver's recorder reads)»
