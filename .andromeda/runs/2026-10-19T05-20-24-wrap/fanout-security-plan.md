proposals:
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      Add a row «Recorded check (escher-driver `Check::from_bytes`) | A kept check's bytes, untrusted | Since 2026-10-19-session-recording
      a strict reader: at most 1,048,576 bytes, a JSON array of at most 400 records — the opening `{"tool":"start","arguments":{"app":…}}`,
      then records with the closed keys `tool` · `arguments` · `masked` · `accepted` · `digest` only, `tool` one of the seven instance verbs,
      argument names from that verb's schema, `masked` true on a `type` alone, `digest` 16 lowercase hex (an `answer` key refused until
      «Recorded-check replay»); anything else refused `Cause::Malformed` with no fault, holding nothing of the file, with no panic path;
      read by `serde_json`, which writes nothing, each `Call` built through `command::call_of`, the builder the MCP server shares, so a
      replayed step still passes `validate` inside `Session::run`».
    sidecar: >-
      §Input Validation — new row: the kept-check reader `Check::from_bytes`, a bounded closed grammar refused `malformed` with nothing held
      (2026-10-19-session-recording).
    rationale: >-
      The report's Symbols / APIs add `Check::from_bytes(&[u8]) -> Result<Check, Refusal>`, a deserialized-struct input surface, and its
      Coverage of new surfaces lists it as untrusted input validated by a strict closed grammar (35 refusal rows, edge rows at both bounds);
      §Input Validation holds no row for it, so the boundary is unrecorded where the detector reads its validation.
    basis: packages/escher-driver/src/recording.rs:306-335
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      Add a row «Driver recording directory (escher-driver)» beside the state-directory row: a caller-named path (`--record <dir>`, a
      parameter of `Session::recording_into`) created `0700` when absent (its parent must exist), a present one open to its group or to
      others refused `RecordDirNotPrivate` (`record-dir-not-private`, exit 3) with nothing written; `check-{n}.json` created `0600` with
      `create_new`, `n` the smallest from 1 naming no file, so no file is ever replaced; the check rewritten whole after every call through
      `.check-{n}.json.tmp` (`0600`) and a rename; a call no record can spell (a verb outside the seven instance verbs, an argument the
      verb's schema does not name or passes twice) not recorded; a failed write or a passed bound (400 records, 1,048,576 bytes) ends the
      recording — its check removed, nothing further recorded, `recording_failed()` true once, the call run and not rolled back; a file the
      recorder did not create never removed, and no kept check otherwise removed by escher; off unix `Unsupported`.
    sidecar: >-
      §Input Validation — new row: the recording directory (`0700`, `RecordDirNotPrivate`) and its `0600` `check-{n}.json`, create_new,
      bounded, never replacing a file (2026-10-19-session-recording).
    rationale: >-
      The report's Symbols / APIs (`Session::recording_into`, `recording_failed`, Ports, sockets, env vars) name a new filesystem location
      taken from the caller, checked by mode before anything is written; Coverage of new surfaces lists «validation closed argv reader +
      directory mode check»; §Input Validation's directory row covers the state directory alone.
    basis: packages/escher-driver/src/session.rs:127-149; packages/escher-driver/src/recording.rs:198-243
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      In the «CLI arguments (escher-session)» row (security-plan.md:82): `--record <dir>` is taken at most once, on
      `start <app> --session <dir> --record <dir>`, on the host role `serve <app> --session <dir> --record <dir>` (in that order) and on the
      `mcp` role `mcp --record <dir>` — so the host role is no longer «in exactly that shape» and `mcp` no longer «alone»; anywhere else,
      given twice or with no value it is usage (status 2) once the call passed `validate`, a refused call staying a refusal (a stray
      `--record` was an unnamed argument, status 1, before); `start` checks the recording directory before it spawns and passes `--record`
      to the host it spawns, a refusal there status 3 with nothing spawned; `mcp --record` checks it before it serves, a refusal status 3
      with its fixed message on stderr and stdout empty; the usage line reads `start <app> [--record <dir>]`,
      `host role: serve <app> --session <dir> [--record <dir>]` and `mcp role: mcp [--record <dir>]`; nothing the caller typed — the path
      included — is written back.
    sidecar: >-
      §Input Validation CLI-arguments row — `--record <dir>` on `start` · `serve` · `mcp`, its usage and status-3 endings, the usage line
      (2026-10-19-session-recording).
    rationale: >-
      The report's Symbols / APIs (The command line) widen the closed argv reader with `--record` on three shapes; the row still states the
      host role in exactly one shape, `mcp` alone, the usage line `; mcp role: mcp` and `start` spawning `serve <app> --session <dir>`.
    basis: packages/escher-driver/src/cli.rs:205-213; packages/escher-driver/src/cli.rs:446-467
  - detector: D-security-input
    severity: escalate
    section: Threat Model Summary
    change: >-
      The local-IPC vector's entry point (security-plan.md:24) names the host role as `serve <task> --session <dir> [--record <dir>]`,
      the `start` that spawns it passing a given `--record` on.
    sidecar: >-
      §Threat Model Summary — the host role's shape gains `[--record <dir>]` (2026-10-19-session-recording).
    rationale: >-
      Same claim as the CLI-arguments row: the report's [usage] row lists security-plan.md:24 as changed, and its Symbols / APIs give the
      host role `serve <app> --session <dir> --record <dir>`.
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      In the «Driver session socket (escher-driver)» row (security-plan.md:78) the closed grammar is at wire version `v3` — `hello v3`,
      `stop v3`, `call v3 <verb> …`, a request at an earlier version (`call v2 snapshot`) refused `version` — its replies
      `ok v3 accepted|refused <json>` and `ok v3 oversize`, plus one new fixed reply `ok v3 recording-failed` (`Reply::RecordingFailed`)
      that the host sends in place of the answer when its recording ended after the call, read by the client as
      `SessionError::RecordingFailed`; no other request, reply or bound changed; the dated «the wire staying `v2`» stays as history.
    sidecar: >-
      §Input Validation socket row — wire `v2` → `v3`, the fixed `ok v3 recording-failed` reply (2026-10-19-session-recording).
    rationale: >-
      The report's Symbols / APIs (The wire) move `v2` → `v3` with one new fixed reply; Spec claims disproved records `call v2 snapshot`
      now reading `refused version`; the row's grammar, which is the socket's request-line validation, still reads `v2`.
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      In the `id` (stable element id) row (security-plan.md:123) the hosted `snapshot`'s socket answer reads `ok v3 accepted <json>`.
    sidecar: >-
      §Input Validation id row — the hosted answer's grammar `v3` (2026-10-19-session-recording).
    rationale: >-
      Same claim as the socket row: the report's [wire] row lists security-plan.md:123 as changed.
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: API Security
    change: >-
      The listener paragraph (security-plan.md:208) serves `call` at wire `v3`, its replies including the fixed `ok v3 recording-failed`.
    sidecar: >-
      §API Security — the socket's wire `v3` (2026-10-19-session-recording).
    rationale: >-
      Same claim as the socket row: the report's [wire] row lists security-plan.md:208 as changed.
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Error Handling
    change: >-
      At security-plan.md:357 a refusal is sent on the socket as `ok v3 refused <json>`, and `SESSION_ERRORS` is `[ErrorSpec; 15]`,
      gaining `RecordDirNotPrivate` (`record-dir-not-private`) and `RecordingFailed` (`recording-failed`), each with a fixed meaning and
      remedy and exit `3`, so `help`'s `session_errors` serves 15.
    sidecar: >-
      §Error Handling — wire `v3`; `SESSION_ERRORS` 13 → 15 with the two recording errors (2026-10-19-session-recording).
    rationale: >-
      The report's [wire] and [serr] rows list security-plan.md:357 as changed; its Symbols / APIs add the two variants and grow
      `SESSION_ERRORS` 13 → 15.
    basis: packages/escher-driver/src/error.rs:163-176
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      The «password and file `input` value» row gains the kept check: a recorded `type` whose element resolves to a masked control — the
      now-public `dioxus_native_dom::is_masked_input` (an `input` whose `type` is `password` or `file`, ASCII case-insensitive), the test
      the snapshot's `value` reader calls, no snapshot reading changed — or resolves to none (refused `stale` or `not-found`) keeps
      `MASKED_STAND_IN` (`"masked"`) and `masked:true`, never the text or its length; any other `type`'s text, a `type` refused by
      `validate` included, is kept as it crossed (F1 A · F2 B, the founder's word, inputs#I4); measured: 0 sentinel occurrences, and a
      secret of another length leaves byte-identical files (`a_password_typed_in_a_recorded_session_is_kept_as_the_stand_in`).
    sidecar: >-
      §Input Validation password row — a masked `type` kept as `MASKED_STAND_IN` in a kept check; `is_masked_input` public
      (2026-10-19-session-recording).
    rationale: >-
      The report's Symbols / APIs (`Session::run`, `is_masked_input`, `MASKED_STAND_IN`), Deviation 11 and Outcome «(security, password)»
      add a new resting place for typed text with its own mask rule; the row records the mask on every other exit and names none at rest;
      expected amendment 3 names this row.
    basis: packages/dioxus-native-dom/src/snapshot.rs:226-233; packages/escher-driver/src/execute.rs:339-343; tests/blitz-tests/tests/stand_session_recording.rs:274-323
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      In the `id` row (security-plan.md:123), «no log, event or file carries the snapshot or its text» is restated: no log or event does,
      and since 2026-10-19-session-recording a file does carry call content — a kept check holds the ids its calls named, a `press` key, an
      `advance`'s ms and unmasked typed text — and no screen reading (no snapshot text, name, value or diff).
    sidecar: >-
      §Input Validation id row — the 2026-10-10 «no log, event or file» sentence restated for the kept check (2026-10-19-session-recording).
    rationale: >-
      The report's [nofile] row lists security-plan.md:123 @c6330 as changed (its @c9683 «no log, event or file carries a diff» no-change),
      and the recording-directory row this file proposes is the file that retires «no file».
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      In the `id` row (security-plan.md:123), «escher's own code now gives the id three exits, the platform adapter, the driver's answers
      and … MCP tool results» gains a fourth since 2026-10-19-session-recording: a kept check at rest, holding the ids a recorded session's
      calls named.
    sidecar: >-
      §Input Validation id row — the kept check as an at-rest exit of a call's target ids (2026-10-19-session-recording).
    rationale: >-
      The report's Schema / config and [nofile] row: a kept check carries target ids; the row enumerates the id's exits as closed.
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: >-
      In the `id` row (security-plan.md:123), the second enumeration «the id's exits being the platform adapter, since 2026-10-10-driver-cli
      the driver's answers and, since 2026-10-10-mcp-surface, MCP tool results» gains «and, since 2026-10-19-session-recording, a kept
      check's call targets».
    sidecar: >-
      §Input Validation id row — the second exit list gains the kept check (2026-10-19-session-recording).
    rationale: >-
      Same claim restated in the row's `unkeyed_actionable` passage; a duplicated exit list would survive a single-site apply.
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Data Protection
    change: >-
      «At rest — per medium» (security-plan.md:187) gains a bullet «Files (kept checks)»: since 2026-10-19-session-recording, under
      `--record <dir>` or `Session::recording_into`, one `check-{n}.json` (`0600`) per session in a `0700` directory, a JSON array — the
      app label, then per call its verb, arguments, acceptance and a FNV-1a 64 digest of its written answer — holding call content (target
      ids, `press` keys, `advance` ms, unmasked typed text; a masked control's text as `MASKED_STAND_IN`) and no screen reading, never
      logged, and never removed by escher but for its own check when a recording ends; the call-content class ratified by the founder with
      F1 A and F2 B (inputs#I4).
    sidecar: >-
      §Data Protection At rest — the kept check as a new medium (2026-10-19-session-recording).
    rationale: >-
      The report's [atrest] row lists security-plan.md:187 as extend (the kept check a medium), Coverage of new surfaces calls the check «a
      new at-rest medium», and expected amendment 3 names §Data Protection (At rest).
    dependent-of: D-security-input
note: >-
  D-security-auth and D-security-deps find no drift — the recording adds no auth, token, secret or key (access is the file modes), and the
  report's Dependencies bullet adds and bumps nothing; two standing claims the report sweeps as no-change may still want a look —
  §Dependency Security's serde_json bullet calls it «the reader of the MCP server's untrusted stdin» while it now also reads kept checks
  (still writing nothing), and §Secret Management's `CARGO_MANIFEST_DIR` bullet lists no escher-driver unit-test site though those tests
  now read it for `target/tmp`; the wire/serr/usage proposals overlap the P2 cascade's leaf rows on the same lines.
