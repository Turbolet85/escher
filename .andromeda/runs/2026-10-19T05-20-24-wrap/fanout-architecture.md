proposals:
  - detector: D-arch-resources
    severity: escalate
    section: '§Occupied Resources → Filesystem'
    change: 'Add, beside the driver session state directory, the recording directory: a path the launcher names by `--record <dir>` (on `start`, the host role `serve` and the `mcp` role; never an env read or a default), created `0700` by `Session::recording_into` when absent (its parent must exist) and refused `RecordDirNotPrivate` when present and open to its group or others, holding one `check-{n}.json` per session (`0600`, `create_new`, `n` the smallest from 1 naming no file, never replaced), rewritten whole after every call through `.check-{n}.json.tmp` (`0600`) and a rename, never removed by escher save the check itself when its recording ends (a failed write or a passed bound), a file the recorder did not create never removed; unix only (`Unsupported` elsewhere); the state directory still holds `session.sock` alone and is removed at `stop`.'
    sidecar: '2026-10-19-session-recording: §Occupied Resources → Filesystem gains the `--record` recording directory (`0700`) and its kept check `check-{n}.json` (`0600`), a new at-rest medium.'
    rationale: 'The report (Symbols / APIs → Ports, sockets, env vars; Schema / config; Expected amendment 1) names a new filesystem resource the chunk lands, a boundary widening (call content at rest) the founder ratified as F1 A / F2 B (inputs#I4, I5); unregistered in §Occupied Resources. Escalate so the card cites his word per the 2026-10-10 rule.'
    basis: '.andromeda/architecture.md:191'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Filesystem'
    change: 'Beside the session checks own state directories under `target/tmp/`, register the recording checks scratch directories (blitz-tests `stand_session_recording`, seven_guis `recording`, `host_log` and `mcp_log` recorded runs) and escher-driver unit tests scratch directories under the workspace `target/tmp`, reached through the compile-time `CARGO_MANIFEST_DIR` (no `TMPDIR` read) — test-only, local, uploaded by no CI leg.'
    sidecar: '2026-10-19-session-recording: §Occupied Resources → Filesystem registers the recording tests and escher-driver unit tests scratch directories under target/tmp (test-only).'
    rationale: 'Report Symbols / APIs → Ports, sockets, env vars and Deviation 8: unit tests write under the workspace `target/tmp` through `CARGO_MANIFEST_DIR`; the Filesystem bullet enumerates the session checks test directories and does not hold these new test-written locations.'
    basis: '.andromeda/architecture.md:191'
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'Register the recording API: the crate re-exports 49 names (45 + `Check`, `Step`, `answer_digest`, `MASKED_STAND_IN`); `Session::recording_into(self, dir) -> Result<Session, SessionError>` (dir `0700`, `check-{n}.json` `0600` create_new, opening record `{"tool":"start","arguments":{"app":…}}`, rewritten whole through a temporary and a rename; off unix `Unsupported`) and `Session::recording_failed(&mut self) -> bool` (true once after a recording ended); `Session::run` (signature and command span unchanged) records each answered call as `{tool, arguments (schema order), [masked:true], accepted, digest}`, a call no record can spell kept not, a `type` on a masked or unresolved element kept as `MASKED_STAND_IN` ("masked") with `masked:true`, bounds 400 records / 1,048,576 bytes ending the recording (check removed, call not rolled back); `answer_digest` FNV-1a 64 over `Outcome::to_json()` / `Refusal::to_json()`, 16 lowercase hex; `Check::from_bytes` a strict bounded reader refusing `Cause::Malformed` with nothing held, `Check::{app, steps}`, `Step::{call, accepted, digest, masked}`; the MCP server answers a held session whose `recording_failed()` reads true with `ToolResult::failed(SessionError::RecordingFailed)` (`isError`), `serve_mcp` signature and the `tool_call` span unchanged.'
    sidecar: '2026-10-19-session-recording: §Standard Contracts → Driver session registers `recording_into`, `recording_failed`, `Check`, `Step`, `answer_digest`, `MASKED_STAND_IN` and the kept-check form (re-exports 45 → 49).'
    rationale: 'Report Symbols / APIs and Schema / config list these new public APIs and the kept-check form; Expected amendment 2 places them in §Standard Contracts → Driver session, which still reads «re-exports 45 names» and names none of them. The 49 is 45 plus the four re-exports the report names — the orchestrator should confirm against lib.rs.'
    basis: '.andromeda/architecture.md:175'
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: '`SessionError` is exactly the thirteen variants plus `RecordDirNotPrivate` (kind `record-dir-not-private`) and `RecordingFailed` (kind `recording-failed`) — fifteen, the last two since 2026-10-19-session-recording, each with a fixed meaning and remedy, exit `3`.'
    sidecar: '2026-10-19-session-recording: SessionError variants 13 → 15 (`RecordDirNotPrivate`, `RecordingFailed`).'
    rationale: 'Report Symbols / APIs: two new variants; the contract enumerates the variants and reads «thirteen, the last since 2026-10-10-driver-cli».'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'The crate-private table reads `SESSION_ERRORS: [ErrorSpec; 15]`.'
    sidecar: '2026-10-19-session-recording: `SESSION_ERRORS: [ErrorSpec; 13]` → 15.'
    rationale: 'Report Counts / qualifiers moved [serr]: architecture.md:175 «`[ErrorSpec; 13]`» changed.'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: '`describe()` serves 15 session errors (13 until 2026-10-19-session-recording), so `help` line gains the two entries; the verb table and `tools/list` unchanged.'
    sidecar: '2026-10-19-session-recording: describe() «13 session errors» → 15.'
    rationale: 'Report [serr] architecture.md:175 «13 session errors» changed; Spec claims disproved (plan step 8): `help` line changed, pinned by the describe.rs count row `[11, 13, 12, 3, 8, 4, 15]`.'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'The wire is versioned `v3` since 2026-10-19-session-recording (`v2` since 2026-10-10-driver-cli): requests `hello v3`, `stop v3`, `call v3 …`, and one new fixed reply `ok v3 recording-failed` (`Reply::RecordingFailed`), which the host sends in place of the answer when `recording_failed()` reads true after a call and the client reads as `SessionError::RecordingFailed`; a `v2` request reads `refused version`; no other request, reply or bound changed.'
    sidecar: '2026-10-19-session-recording: the session wire v2 → v3 with the `ok v3 recording-failed` reply.'
    rationale: 'Report Symbols / APIs → The wire and [wire] (architecture.md:175 changed); Spec claims disproved: `call v2 snapshot` now reads `refused version`.'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Network ports and listeners'
    change: 'The session socket serves its three requests at wire version `v3` (since 2026-10-19-session-recording; `v2` before), adding the fixed `ok v3 recording-failed` reply; still no second listener.'
    sidecar: '2026-10-19-session-recording: the listener row wire version v2 → v3.'
    rationale: 'Report [wire]: architecture.md:188 changed, each row stating the current grammar.'
    basis: '.andromeda/architecture.md:188'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'The command line reader takes `--record <dir>` at most once on `start <app> --session <dir> --record <dir>`, `serve <app> --session <dir> [--record <dir>]` (in that order) and `mcp [--record <dir>]`, anywhere else, twice or with no value usage (exit 2) once the call passed `validate`; `start` checks the recording directory before it spawns and passes `--record` to the host (`serve <app> --session <dir> --record <dir>`), a refusal exit 3 with nothing spawned; `mcp --record` checks it before serving, a refusal exit 3, its message on stderr, stdout empty; the usage line reads `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]`; nothing typed written back.'
    sidecar: '2026-10-19-session-recording: the command line reader, the start spawn and the usage line gain `[--record <dir>]` on start, the host role and the mcp role.'
    rationale: 'Report Symbols / APIs → The command line and [usage]: architecture.md:175 (@c27684, @c29558, @c30037) changed; the contract still reads `serve <app> --session <dir>` «in exactly that shape» and `mcp` «alone».'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → CLIs'
    change: '`start <task> [--record <dir>]`; the host role is `serve <task> --session <dir> [--record <dir>]`; the `mcp` role is `mcp [--record <dir>]` (since 2026-10-19-session-recording; `mcp` alone until then).'
    sidecar: '2026-10-19-session-recording: CLIs row — escher-session start, host role and mcp role gain `[--record <dir>]`.'
    rationale: 'Report [usage]: architecture.md:180 (@c2644, @c2757 «`mcp` alone») changed.'
    basis: '.andromeda/architecture.md:180'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'The session holds a label, the harness, that optional step, one private record of the ids its screens have read and, since 2026-10-19-session-recording, an optional crate-private `Recording` (its check path and kept records) set by `recording_into`.'
    sidecar: '2026-10-19-session-recording: what a Session holds gains the optional recording.'
    rationale: 'Report Symbols / APIs: `recording_into` gives a session a held recording and `run` records into it; the contract enumerates what the session holds and omits it.'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'Restate «That record is the one thing the session keeps between calls»: the record of ids is the one thing the session keeps between calls of what its screens read; a session recording into a directory also keeps, at rest, the calls it answered (never a screen reading).'
    sidecar: '2026-10-19-session-recording: «the one thing the session keeps between calls» restated beside the kept check.'
    rationale: 'Report Symbols / APIs (`Session::run` now records each answered call when a recording is held); the sentence asserting the id record is the only between-call state no longer holds for a recording session.'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'On a non-unix target `serve`, `start`, `attach`, `stop`, a schema-admitted `call` and, since 2026-10-19-session-recording, `Session::recording_into` return `Unsupported`; a `Session` held in process without a recording works on every target.'
    sidecar: '2026-10-19-session-recording: the off-unix Unsupported list gains `recording_into`.'
    rationale: 'Report Symbols / APIs («Off unix it returns `SessionError::Unsupported`») and Deviation 4 (recording is unix-only).'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Dioxus DOM bridge'
    change: 'Register `dioxus_native_dom::is_masked_input(&ElementData) -> bool` (public, re-exported beside `MASKED_VALUE`): true for an `input` whose `type` is `password` or `file`, ASCII case-insensitive; the snapshot `value` reader calls it, no reading changed; escher-driver reads it to keep a masked `type` as the stand-in.'
    sidecar: '2026-10-19-session-recording: Dioxus DOM bridge registers the public `is_masked_input`.'
    rationale: 'Report Symbols / APIs: `is_masked_input` is a new public API of dioxus-native-dom; the bridge contract names `MASKED_VALUE` as the crate public constant and states the password / file test inline with no public reader.'
    basis: '.andromeda/architecture.md:178'
  - detector: D-arch-resources
    severity: warning
    section: '§Existing Scopes → dioxus-native-dom'
    change: 'The snapshot module re-exports its three types, the `MASKED_VALUE` constant and, since 2026-10-19-session-recording, `is_masked_input`.'
    sidecar: '2026-10-19-session-recording: dioxus-native-dom scope row — snapshot re-exports `is_masked_input`.'
    rationale: 'Same new public symbol (report Symbols / APIs; `packages/dioxus-native-dom/src/lib.rs` added line 35); the row lists the snapshot module re-exports without it.'
    basis: '.andromeda/architecture.md'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Dioxus DOM bridge'
    change: 'Restate «no log, event or file carries the snapshot or its text»: no log or event carries it, while a kept check carries call content — target ids, a `press` key, an `advance` ms, unmasked typed text — and no screen reading; the later «no log, event or file carries a diff» stands.'
    sidecar: '2026-10-19-session-recording: the no-file sentence of the snapshot text restated beside the kept check.'
    rationale: 'Report [nofile]: architecture.md:178 @c21771 changed (restated), @c26395 no-change.'
    basis: '.andromeda/architecture.md:178'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Existing Scopes → escher-driver'
    change: 'fifteen private modules — adding recording (the crate-private `Recording` writer and directory check behind a unix `imp` and an off-unix stub, `Check`, `Step`, `answer_digest`, `MASKED_STAND_IN`, re-exported); error the table of the fifteen errors; command also holding the crate-private `call_of`, moved from mcp result and shared by the MCP server and `Check::from_bytes`; json gaining a signed `integer` writer; the row purpose naming the session recording.'
    sidecar: '2026-10-19-session-recording: escher-driver scope row — private modules 14 → 15 (`recording`), error table 13 → 15, `call_of` in command.'
    rationale: 'Report Crates / modules (14 → 15 private modules), [modules] architecture.md:300 changed, [serr] (15 session errors), and `command::call_of` moved to command.rs.'
    basis: '.andromeda/architecture.md:300'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → CI contracts'
    change: 'The `stand-repeat` leg runs `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and `recording`, the new `stand_session_recording` joining by its `stand_` prefix; `test_stand_repeat_leg_runs_three_rounds` pins both words.'
    sidecar: '2026-10-19-session-recording: stand-repeat seven_guis list gains `recording`.'
    rationale: 'Report Harness / gate surface and [sgtargets]: architecture.md:183 target list changed.'
    basis: '.andromeda/architecture.md:183'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Process-wide state and threads'
    change: 'eight seven_guis test targets spawn the built `escher-session` binary through `CARGO_BIN_EXE_escher-session`, `recording` since 2026-10-19-session-recording.'
    sidecar: '2026-10-19-session-recording: seven → eight seven_guis test targets spawning escher-session.'
    rationale: 'Report [sgtargets]: architecture.md:192 «seven seven_guis test targets» changed.'
    basis: '.andromeda/architecture.md:192'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Existing Scopes → seven_guis'
    change: 'the crate eight integration-test targets, all spawning the built `escher-session` binary, adding since 2026-10-19-session-recording `tests/recording.rs`.'
    sidecar: '2026-10-19-session-recording: seven_guis scope row — integration-test targets 7 → 8 (`tests/recording.rs`).'
    rationale: 'Same moved count (report [sgtargets], 7 → 8; new file `examples/seven_guis/tests/recording.rs`), worded «the crate seven integration-test targets» in the seven_guis row, which the report sweep pattern did not reach.'
    basis: '.andromeda/architecture.md'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Existing Scopes → blitz-tests'
    change: 'Register `stand_session_recording` (the four lean tasks round trip in both layout modes, a password kept as the stand-in, a group-open directory refused) and read `tests/session_common/mod.rs` by five of the session checks, its helpers gaining `replay`, `kept` and the unix-only `mode`.'
    sidecar: '2026-10-19-session-recording: blitz-tests scope row — `stand_session_recording` registered; session_common read by five session checks, helpers `replay`, `kept`, `mode`.'
    rationale: 'Report Files (new `tests/blitz-tests/tests/stand_session_recording.rs`) and [readers]: architecture.md:305 @c19440 «by four of the session checks» → five.'
    basis: '.andromeda/architecture.md:305'
  - detector: D-arch-decisions
    severity: escalate
    section: '§Established Decisions → [Driver session]'
    change: 'Add: since 2026-10-19-session-recording a session a launcher names a recording directory for (`--record`) keeps its answered calls at rest as one owner-only replayable check — call content (target ids, a `press` key, an `advance` ms, unmasked typed text; a masked or unresolved `type` as the stand-in) and each answer as a digest only, no screen reading, never logged — std-only, no dependency added, unix only (`Unsupported` elsewhere, as the socket), a boundary widening ratified by the founder (F1 A, F2 B, inputs#I4, his option labels quoted verbatim), F3 C standing as the interim answer form, its widening to B owed at «Recorded-check replay».'
    sidecar: '2026-10-19-session-recording: [Driver session] decision records the kept check as a ratified at-rest widening (F1 A, F2 B).'
    rationale: 'Report Decisions & corrections and Addendum (inputs#I4, I5): the founder ratified F1 A / F2 B and the kept call content; the decision states every carrier widening with its ratification and has none for this at-rest medium. Escalate: a boundary widening goes on the halt card citing his word.'
    basis: '.andromeda/architecture.md:115'
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Driver session]'
    change: '`serde_json` a reader of MCP requests and, since 2026-10-19-session-recording, of kept checks (`Check::from_bytes`) that writes nothing — the founder «serde_json, только чтение» unmoved.'
    sidecar: '2026-10-19-session-recording: serde_json «a reader of MCP requests only» → MCP requests and kept checks, still writing nothing.'
    rationale: 'Report [serde]: architecture.md:115 @c2418 «a reader of MCP requests only that writes nothing» → stale; `Check::from_bytes` reads with `serde_json`; Outcome (arch): still a reader only, Dependencies none added — the locked «только чтение» holds, only the «MCP requests only» scope moved.'
    basis: '.andromeda/architecture.md:115'
  - detector: D-arch-decisions
    severity: warning
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: '`serde_json`, taken at 2026-10-10-mcp-surface, reads MCP requests and, since 2026-10-19-session-recording, kept checks, and writes nothing.'
    sidecar: '2026-10-19-session-recording: the json contract serde_json sentence extended to kept checks.'
    rationale: 'Report [serde]: architecture.md:175 @c25200 «reads MCP requests and writes nothing» → extend.'
    basis: '.andromeda/architecture.md:175'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies → Media'
    change: 'serde_json "1" taken by escher-driver alone as its reader of MCP requests and, since 2026-10-19-session-recording, of kept checks.'
    sidecar: '2026-10-19-session-recording: Stack Media row — serde_json also reads kept checks.'
    rationale: 'Same retired scope worded «as its reader of MCP requests»; the report [serde] disposed architecture.md:57 no-change as a dependency fact — offered for the orchestrator to keep or drop.'
    basis: '.andromeda/architecture.md:57'
    dependent-of: D-arch-decisions
note: 'The two escalate proposals are the at-rest boundary widening (F1 A, F2 B, ratified by the founder per inputs#I4 / I5) and belong on one halt card; quote his option labels verbatim from inputs/I4-relay-1.md.txt, which this detector did not read. The 49 re-export count is 45 plus the four names the report lists — confirm against packages/escher-driver/src/lib.rs. No new crate, port, socket or env var, and no new dependency (D-arch-decisions otherwise clean).'
