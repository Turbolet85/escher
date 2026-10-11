# Report — 2026-10-10-mcp-surface

**Chunk:** MCP surface — the driver's commands as MCP tools over stdio, local to the invoking user, no listener or auth surface, a tool call parenting its trace
**Date:** 2026-10-11T08:40Z
**Commits:** `69a399da feat(2026-10-10-snapshot-states): …` (the previous wrap's commit, this chunk's base) · `36877a23 chore(2026-10-10-mcp-surface): operator pre-CI commit, for the run this chunk verdict reads` (`git log --since=2026-10-10T19:06:02Z`)

## Changes (structured — detectors read this)
- **Files:** base `69a399da` → work tree, `gate.py scope` (wrap P1): changed 14 · listed 14 · recorded 0.
  - New: `packages/escher-driver/src/mcp.rs` (1204 lines) · `examples/seven_guis/tests/mcp_server.rs` (278) ·
    `examples/seven_guis/tests/mcp_log.rs` (240) · `tests/blitz-tests/tests/stand_mcp_tools.rs` (406) — counts from
    `cites.py added` (§New text below).
  - Modified: `packages/escher-driver/src/{lib,cli,json}.rs` · `packages/escher-driver/Cargo.toml` · `Cargo.toml` ·
    `Cargo.lock` · `examples/seven_guis/src/session_host.rs` · `examples/seven_guis/tests/{common/mod,host_binary}.rs` ·
    `README.md`.
  - Chunk folder (not source): `evidence/{controls,counts,live-trial,operator-pass}.md`, `evidence/live-trial.py`.
- **Symbols / APIs:**
  - `escher_driver::serve_mcp(input: impl BufRead, output: impl Write, apps: &[&str], boot: impl FnMut(&str) ->
    Result<Session, SessionError>, idle_expiry: Duration) -> Result<(), SessionError>` — new public fn, re-exported at
    the crate root (`lib.rs:105`; defined `mcp.rs:49-111`, head `:83`). The crate now re-exports **41** public names and
    holds **12** private modules (`mod mcp;` at `lib.rs:91`) — read off `lib.rs`: 11 modules + `mcp`; 40 names + 1.
    - Newline-delimited JSON-RPC 2.0 in the handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05`;
      `initialize` answers the asked revision when it is one of the three, else `2025-06-18`; `capabilities:
      {"tools":{}}`; `serverInfo {name: "escher", version: escher-driver's CARGO_PKG_VERSION}` (`mcp.rs:319-339`).
      `ping` → `{}`; `tools/list` → the nine verbs of `VERBS` in order (`mcp.rs:341-397`); `tools/call`; every other
      method, `server/discover` included, `-32601`. A request with no `id` is a notification: no reply, nothing runs.
    - Request line ≤ **65,536** bytes (`MAX_MCP_REQUEST_BYTES`, private), read from one `BufRead` held for life
      (`read_request`, `mcp.rs:128-168`); a longer line is read to its terminator, dropped, answered `-32700`; an empty
      line skipped; a last line without terminator read as a line; non-UTF-8 or non-JSON → `-32700`; not an object with
      `jsonrpc: "2.0"` and a string `method` → `-32600` (`envelope`, `mcp.rs:247-277`).
    - `id` echoed only as an i64 integer or a string ≤ **128** bytes (`Id::admitted`, `mcp.rs:178-199`); another form →
      `-32600` with `id: null`. The echo is the one client-sent value written back.
    - Protocol errors: fixed code + fixed message (`parse error` · `invalid request` · `method not found` · `invalid
      params` · `unknown tool`), no `data` (`ProtocolError`, `mcp.rs:201-230`).
    - Tool listing (`tool`, `mcp.rs:341-390`): `name` = verb name, `description` = verb help, `inputSchema {type:
      object, properties, required, additionalProperties: false}`; per arg `type` by kind (`id`/`text`/`name` →
      string; `key` → string + `enum` `KEY_NAMES`; `flag` → boolean; `milliseconds` → integer, `minimum` 1, `maximum`
      `MAX_MILLISECONDS`), `start`'s `app` `enum` = the binary's apps; `description` = arg help + ` (` + `bound()` + `)`.
    - A tool call (`Server::tool_call`, `mcp.rs:554-588`): `params` an object with a string `name`, `arguments`
      optional object, else `-32602 invalid params`; a name not in `VERBS` → `-32602 unknown tool`; each argument value:
      string → `Text`, i64 → `Number`, bool → `Flag`, any other → refused `malformed` (`WrongKind(table name)` or
      `Unnamed`) before anything runs (`call_of`, `mcp.rs:399-423`).
    - Instance verbs run through `Session::run` on the held session; with none held the call is validated first (a
      refusal if malformed), else `SessionError::NoSession`. Session verbs answered by the server: `start` (an app
      outside `apps` refused `malformed` out-of-bound on `app`; `AlreadyRunning` when held; else `boot(app)`, held, answer
      `json::started`), `status` (`json::status`, `served` = tool calls answered on the held session since its `start`),
      `stop` (drops it, `json::stopped()`) (`Server::run` / `start`, `mcp.rs:590-642`).
    - Tool result `{content:[{type:"text",text}], isError}`: accepted `snapshot` → the screen text itself; any other
      accepted call → its JSON line; a refusal → `Refusal::to_json`, a session error → `SessionError::to_json`, both
      `isError: true`; a text over **1,048,576** bytes (`wire::MAX_ANSWER_BYTES`, reused) → `AnswerTooLarge`'s result
      (`ToolResult`, `mcp.rs:443-491`). No `structuredContent` on any result.
    - Idle expiry checked when a request arrives with a session held (`Server::expire`, `mcp.rs:515-522`); no timer, no
      thread.
    - Replies written by `json.rs`'s `Object`/`string` only; `serde_json` reads requests (`from_str::<Value>`) and
      writes nothing (the no-panic probe, gate entry 10: 0 hits of print/unwrap/expect/panic/`serde_json::to_`/
      `Serialize` outside the test module).
  - `escher_driver::command_line` — `boot` bound widened `FnOnce` → `FnMut` (`cli.rs:78`; `run`'s at `:324`); its one caller outside the
    crate, `main` at `examples/seven_guis/src/session_host.rs`, compiles unchanged. The private `cli::run` gained an
    `apps` parameter (to hand `serve_mcp` the app names); its three inline test call sites pass `&APPS`.
  - The `mcp` role of the command line: `read` takes the first word `mcp` beside `serve`, before the verb table
    (`MCP_ROLE`, `cli.rs:35-37`; `cli.rs:135-141`); `mcp` alone → the role, any other shape → usage (exit 2, nothing on
    stdout); `run` maps it to `serve_mcp(stdin().lock(), stdout().lock(), apps, boot, IDLE_EXPIRY)`; its ending writes
    nothing on stdout — `Ok` exit `0`, `Err` exit `3` with the error's fixed message on stderr only (`Ending::Spoken`).
    The usage line gains `; mcp role: mcp` after the host role. `command_line`'s doc names the role.
  - `json.rs`: `string` and `Object` (`new` · `text` · `flag` · `count` · `json` · `end`) made `pub(crate)`
    (`json.rs:16 · 45 · 51 · 69 · 73 · 78 · 83 · 87`); no behaviour change.
  - Span: `tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)`
    (`tool_call_span`, `mcp.rs:493-496`), opened and entered per `tools/call` request only, before its params are
    read, until its reply is written; `tool` = the table's word (absent for an unknown tool), `ending` = `accepted` ·
    `refused` · `error` (a session error or a protocol error); no other field. The `command` span `Session::run` makes
    has it as parent by nesting — `execute.rs` unchanged (gate entry 12, `git diff --quiet` over the executor and 24
    other paths: exit 0).
  - Remaining callers: `Session::run`'s production callers are now `host.rs` (`answer`) and `mcp.rs`
    (`Server::run`); the hand writer's readers `cli.rs`, `client.rs`, `host.rs`, `mcp.rs`.
  - Test-module surface: `examples/seven_guis/tests/common/mod.rs` gains `string_at` (166-195), `REPLY_BOUND` (30 s),
    `Speaker` (200-208; `spawn` · `send` · `reply_bytes` · `reply` · `close`, 219-327) with kill-and-reap `Drop`
    (329-334), and `Spoke` (210-217).
- **Crates / modules:** escher-driver gains the private module `mcp` (12 modules). No crate added or removed.
- **Dependencies:** `serde_json = "1"` added to the root `[workspace.dependencies]` (`Cargo.toml:166`, beside `serde`)
  and `serde_json = { workspace = true }` to `packages/escher-driver/Cargo.toml:16`. No `[features]`. `Cargo.lock`
  changes by the one line `"serde_json",` in escher-driver's dependency list (`Cargo.lock:3100`) — no package enters,
  none moves (gate entry 8: `git diff -U0 69a399da -- Cargo.lock`, every changed line the one added, count 0). It was
  already in both graphs: `cargo tree --locked -p escher-driver -e normal -i serde_json` → `serde_json v1.0.151 ←
  dioxus-html v0.7.10 ← dioxus-native-dom`; `-p seven_guis` the same through `dioxus-document` (evidence/controls.md).
  No platform adapter enters the stand graph (gate entry 9: `accesskit_(xplat|winit)` count 0). Audit green, no ignore
  added (gate entry 17).
- **Schema / config:** none — no env var (the `mcp` role reads none; the trial sets `RUST_LOG=info` on its own child),
  no config key, no new scrub field (`tool` and `ending` are not in `CONTENT_FIELDS`, so escher's sink prints them).
  The sink prints a closed `tool_call` span as `… escher_driver … span="tool_call" tool="start" ending="accepted"
  message="close" time.busy=… time.idle=…` (read on the hand-run role, 2026-10-11).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** (evidence/counts.md, each with its command)
  - Workspace `cargo test --workspace --locked` (`ci-leg.sh fast`, `target/ci-logs/test.log`): **170** result lines ·
    **805** passed · 0 failed · **11** ignored (from 167 · 791 · 0 · 11) — stated at `test-plan.md:329`.
  - `run stand` `run.end`: **127** passed · 0 failed · **6** ignored over **33** files (from 125 · 0 · 6 over 32) —
    stated at `test-plan.md:116`.
  - escher-driver unit tests **59** (from 50: `mcp.rs` 9; `cli.rs` still 5 tests, its tables grown 61 → 64 and 17 → 19
    rows) — stated at `registries/contracts/test-plan/session-lifecycle.md:11` ("escher-driver's 50 unit tests (cli.rs 5
    · client.rs 2 · …)").
  - seven_guis integration targets **7** (from 5: + `mcp_server`, `mcp_log`) — stated at `test-plan.md:10` ("holds five
    integration-test targets"), `architecture.md:150` ("five seven_guis test targets spawn the built `escher-session`
    binary"), `obs-plan.md:70` ("three more seven_guis targets spawn the same sink-installing binary").
  - escher-driver re-exports 40 → **41**, private modules 11 → **12** — stated at `architecture.md:133` and `:257`.
  - escher-driver's named crates three → **four** — `architecture.md:101` ("three crates").
  - The `mcp` role's stderr at `info` over one CRUD flow: **9** lines built alone, **10** built with the workspace (the
    tenth `INFO blitz_dom::document … message=[redacted]`); **5** `tool_call` + **3** `command` lines in each.
- **Dev-tool versions:** none — `claude` read at 2.1.296 on the dev host for the trial (the scope's research read
  2.1.289 at P3; the client is the cold-agent pipe's, not a build tool of this chunk). The crate `serde_json` is not a
  host tool.
- **Harness / gate surface:**
  - New check targets: `tests/blitz-tests/tests/stand_mcp_tools.rs` (a stand check: `run stand` reads 33 files),
    `examples/seven_guis/tests/mcp_server.rs`, `examples/seven_guis/tests/mcp_log.rs` — none `cfg(unix)`-gated but
    `mcp_server`'s one CLI-comparison test, so the windows job runs them (it did: CI#38124028789 attempt 2, `Test
    (windows)` green).
  - `stand_mcp_tools` installs a hand-written `tracing::Subscriber` (`Recorder`, 160-207) with
    `tracing::subscriber::with_default`; its second test runs under its own recorder because a thread reaching the
    driver's callsites with no subscriber leaves their interest cached as never (measured: the parent test read 0 spans
    beside it, passed alone; then 15/15 repeated runs green).
  - The shared module's `Speaker` (a child spoken to over piped stdin, stdout read line by line on a thread, 30 s bound).
  - The one-shot `evidence/live-trial.py` (stdlib Python; one `claude -p` with the pipe's flags, `--allowedTools
    mcp__escher`, the server via `sh -c 'tee "$1" | exec env RUST_LOG=info "$2" mcp 2>"$3"'`, its files in
    `target/mcp-trial/`, one printed figures line). The cold-agent pipe, its stub and its contract tests are unchanged.
- **Cross-project / external claims:**
  - `inputs.py verify` (wrap P1): `I1 · message (the operator, 2026-10-10, the phase invocation) · copy message · n/a` ·
    `I2 · message (the operator, 2026-10-10, P4 dialog) · copy message · n/a` · `I3 ·
    ../additional/escher-overseer/relays/mcp-surface-founder-answers.md · copy no-repo · unchanged`; 3 entries, all
    cited, 0 drifted · 0 vanished · 0 broken.
  - CI: `Turbolet85/escher` CI#38124028789 on `36877a237318` (the pre-CI commit) — attempt 1 **red**: `Build wasm
    examples` failed in its toolchain step (`rustup` could not download `channel-rust-stable.toml` from
    static.rust-lang.org, connection reset) before any compile; the operator confirmed the cause and directed the rerun
    of the failed job; attempt 2 **green · checks 16/16 · wall 1267 s** (`ci.py conclusion`, evidence/operator-pass.md).
  - The live trial (evidence/live-trial.md): `claude` 2.1.296 on the dev host, model reached through the operator's own
    login, once: `trial: client_exit 0 · isolated true · tool_calls 7 · refused 0 · errors 0 · end_state reached ·
    protocol_version 2025-11-25 · discover sent`. The host client's first request was `server/discover` (answered
    `-32601`), then `initialize` asking `2025-11-25` — a revision this server does not speak; it answered `2025-06-18`
    and the client proceeded (read from `target/mcp-trial/requests.jsonl`, methods only).
  - The MCP specification pages research fetched 2026-10-10 (research.md §Scope) — not re-fetched.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - Plan step 5 (`plan.md:110-111`): "the call becomes a `Call { verb, args }`, one argument per member in the
    object's order". Measured false as built: `serde_json`'s default `Map` is a `BTreeMap`, so members arrive in key
    order; its `preserve_order` feature would add `indexmap` to the lock, which gate entry 8 forbids. Effect: only which
    fault a call with several faults names first; a repeated JSON key collapses to its last value, so `Repeated` cannot
    arise over MCP. Disposition owed at Validate (a plan text, no master states it).
  - Plan step 10 (`plan.md:211`): "stdout holds exactly one line per request that carried an id". A non-JSON line and
    an over-bound line carry no readable id and are answered (`id: null`, `-32700`), as JSON-RPC requires; `mcp_server`
    asserts one reply per line sent that is not a notification. Disposition owed at Validate.
- **Expected amendments (from plan):** sites by `sites.py` (scratchpad; a regex per fact over the seven masters and the
  7 files under `.andromeda/registries/`, hit lines listed) and `grep -n -o` for the counts.
  - architecture §Standard Contracts → Driver session (41 re-exports, 12 modules, `serve_mcp` and the `mcp` role, "an
    MCP tool" leaves the Not built list, the writer names a third reader) — **carried**: Symbols / APIs, Counts.
    Sites: `re-export count` architecture:1 (133) · `private modules` architecture:1 (257) · `Not built` architecture:1
    (133) · `written form` architecture:3 (133, 257, 263) · security-plan:2 (121, 340) · test-plan:3 (26, 143, 181).
  - architecture §Established Decisions → [Driver session] (four crates, `serde_json` a reader only; the MCP crossing
    ratified — "Boundary widening", the founder 2026-10-10, «Ратифицировать как есть», `inputs#I3`) — **carried**:
    Dependencies, Symbols / APIs. Sites: `three crates` architecture:1 (101) · `serde_json` architecture:1 (64).
  - architecture §Occupied Resources (the `mcp` role, a stdio child with no listener; the server name `escher` in a
    client's config; the role a third reach of `escher_telemetry::init`) — **carried**: Symbols / APIs, Harness.
    Sites: `stdio|one listener` architecture:5 (66, 141, 146, 150, 257) · `` `stub` `` architecture:1 (154) ·
    `escher_telemetry::init` architecture:3 (150, 153, 193).
  - architecture §Stack and Technologies (`serde_json` 1.0.151 in escher-driver) — **carried**: Dependencies. Site:
    architecture:64.
  - architecture §Project Intent → Front page — **carried**: Files (README). Sites: `README|Front page` architecture:2
    (138, 210).
  - security-plan §Threat Model Summary (the MCP carrier) — **carried**: Symbols / APIs, Cross-project (the live
    trial: the screen's content reached the model provider through the client). Sites: `MCP` security-plan:3 (73, 77,
    118).
  - security-plan §Input Validation (a row for the MCP server's input; the Driver command schema row at three surfaces;
    the `id` row at three exits — "Boundary widening", the founder's 1A, `inputs#I3`) — **carried**: Symbols / APIs.
    Sites: security-plan:73 (the stub row), :77 (the schema row), :118 (the `id` row); `snapshot text` security-plan:5
    (118-121, 186).
  - security-plan §API Security (still one listener) — **carried**: Symbols / APIs (no listener, port or socket added).
    Site: `one listener` security-plan:198.
  - security-plan §Dependency Security (`serde_json` named, its hand review) — **carried**: Dependencies. Sites:
    `serde_json` security-plan:0 — the section names reviewed crates by prose; the detector locates it.
  - obs-plan §4 (a second span `tool_call` (`tool`, `ending`), the parent of `command` — "Boundary widening", the
    founder 2026-10-10, «Span с tool и ending», `inputs#I3`) — **carried**: Symbols / APIs (Span). Sites: `command`
    span obs-plan:3 (106, 147, 179) · `tool_call` obs-plan:0 (the 1 regex hit, :158, is the stub verdict's
    `tool_calls`).
  - obs-plan §3 · §6 (the role's stderr, its by-level counts) — **carried**: Counts (9 / 10 lines at `info`). Sites:
    `escher_telemetry::init` obs-plan:3 (68-70) · `host role` obs-plan:5 (30, 69, 106, 174, 179).
  - test-plan §2 · §4 · §5 · §6 · §9 (the three new checks, the shared `Speaker`, escher-driver's unit count, seven_guis'
    seven targets, the workspace and `run stand` counts, `session-proof`) — **carried**: Counts, Harness. Sites:
    test-plan:10 · :116 · :329; key `contracts/test-plan/session-lifecycle.md:11`; `session-proof` test-plan:318 · key
    `session-lifecycle.md:11` · key `test-plan-contracts.toml:14`; `Speaker|tests/common/mod.rs` test-plan:8 (10, 26,
    49, 50, 66, 100, 103, 329).
  - a11y-plan §1 (the stand graph re-read with `serde_json` named) — **carried**: Dependencies (gate entry 9 count 0).
    Sites: a11y-plan:0 for `serde_json`; the Dioxus-crates bullet the plan cites is §1's.
  - a11y-plan §7 (MCP results a further reader of the snapshot model) — **carried**: Symbols / APIs (a `snapshot`
    result is `to_text`'s output). Sites: `snapshot text|Snapshot::to_text` a11y-plan:2 (56, 275).
  - design-system §Surface: cli → Component Patterns (the MCP stdout stream) — **carried**: Symbols / APIs (the role's
    stdout is protocol lines only, no ESC; gate entry 11: no `IsTerminal` in the driver). Sites: `README` design-system:1
    (351); the cli surface's Component Patterns is the detector's read.
  - layout-templates §Surface: cli (`escher-session`) (the `mcp` role and its stdout shape) — **carried**: Symbols /
    APIs. Sites: `host role` layout-templates:2 (10, 74) · `MCP|stdio` layout-templates:1 (73).
  - Owed at the wrap, a route pin: the current MCP revision (2026-07-28: per-request `_meta`, `server/discover`), with
    the trial's readings `protocol_version 2025-11-25` and `discover sent` — **carried**: Cross-project. Owner: P5.
- **Coverage of new surfaces:**
  - `escher-session mcp` (stdin/stdout JSON-RPC) → validation {byte bound · JSON parse · envelope · id form · `call_of`
    kind check · `validate`/`validate_session` ✓} · instrumentation {`tool_call` span + `command` child ✓} · PII {no
    id, name, value or typed text on stderr at `trace` ✓ (`mcp_log`, both builds); results carry them by the ratified
    crossing} · tests {unit (9 in `mcp.rs`) / integ (`mcp_server`, `mcp_log`, `stand_mcp_tools`) / e2e (the live trial)}
    · a11y n/a · tokens n/a.
  - `escher_driver::serve_mcp` (library) → validation ✓ · instrumentation ✓ · PII ✓ · tests {unit / integ in both
    layout modes} · a11y n/a · tokens n/a.
  - `tool_call` span → validation n/a · instrumentation ✓ · PII {two closed-value fields ✓} · tests {`stand_mcp_tools`
    parent and field set; `mcp_log` line count and field names} · a11y n/a · tokens n/a.

## Deviations from intent
- **Argument order** — members in key order, not object order (Spec claims disproved, first bullet). Justification:
  the lock probe (gate entry 8) and "no `[features]` table" (plan step 2) forbid `preserve_order`; the effect is limited
  to which fault is named first.
- **`cli::run` signature** — gained `apps` (plan step 4 said its test call sites stay as they are). Justification: the
  `mcp` role hands `serve_mcp` the app names; `run` had none. In-list file.
- **`start` on an app outside `apps`** — plan silent; refused `malformed` with `Fault::OutOfBound("app")` (the table's
  own arg name, `START_APP`), consistent with the listing's `enum`. The binary's boot would return `InvalidLabel`; the
  library no longer relies on it.
- **Instance verb with no session held** — validated first (a malformed call is its refusal), else `no-session`, as
  the command line checks a call before the session is asked; with a session held the call goes through
  `Session::run`, whose `command` span records the cause.
- **`served`** — counts every tool call answered while held, except the `start` that created the session; protocol
  errors are not tool results and are not counted.
- **Reply count in `mcp_server`** — one per non-notification line (Spec claims disproved, second bullet).
- **The second test of `stand_mcp_tools` under its own recorder** — not in the plan; the plan's subscriber design
  needs it under libtest's parallel threads (Harness / gate surface).
- **Unit-test sessions** — the empty app `Harness::from_component(|| Ok(Default::default()))` gives a real `Session`
  for `AlreadyRunning`, `served` and the idle expiry, where the plan named only a failing boot.
- scope record: none — `gate.py scope` clean, 0 recorded (changed 14 · listed 14).

## Decisions & corrections
- The operator directed the operator pass whole (entry 18 the trial once, hygiene, the pre-CI commit with its fixed
  subject, entry 20, entry 21), "stop and tell me if any entry is red"; on the red, the operator confirmed the cause
  ("the wasm job died in rustup on a connection reset from static.rust-lang.org before any compile") and directed the
  wait, `gh run rerun 38124028789 --failed -R Turbolet85/escher`, and entry 21 again.
- The operator asked this wrap to run P1 only and stop.
- A correction by self-check during implement: the first draft of `evidence/controls.md` named which test failed under
  three mutations without having read the test names; those parentheticals were removed.
- Sweep hazards found this chunk:
  - `tracing`'s callsite interest is process-global: a check that installs a scoped subscriber with `with_default`
    reads no span when another test thread reached the same callsites with no subscriber first — the check passes alone
    and reads 0 beside its sibling.
  - The gate tool's `--operator` call exits 0 on a red; the CI read's verdict is its printed `verdict:` word.
  - In the Bash tool `$TMPDIR` is unset on this host — a redirect to `$TMPDIR/x` writes to `/x` (measured: permission
    denied); use the session scratchpad.
  - `gh api …/jobs/{id}/logs` refuses output with terminal escapes unless given `--allow-escape-sequences`.
  - A regex for `tool_call` matches the stub verdict's `tool_calls` (obs-plan:158) — read each hit.

## Outcome
- **(matrix) v010-13** — met: `tools/list` offers the nine verbs of `VERBS` in order with their args, required set and
  help (`mcp.rs` unit `the_listing_is_the_nine_verbs_of_the_table`, `mcp_server`); the Counter flow by tool calls alone
  in process in both layout modes (`stand_mcp_tools`) and through the binary as a child (`mcp_server`), per-package and
  workspace builds (gate entries 3, 4, 5); the live trial reads `isolated true` and `end_state reached`
  (evidence/live-trial.md). Status `implemented` (P7 flips).
- **(operator, inputs#I2)** — met: a `snapshot` text block equals `Session::run`'s screen text byte for byte in both
  modes, and the command line's `text` field unescaped on a fresh Counter (`stand_mcp_tools`, `mcp_server`'s unix test);
  other accepted results are the JSON line; refusals and session errors `isError: true`.
- **(security) protocol errors run nothing, leak nothing, no panic path** — met: `mcp_server` (discover, non-JSON,
  65,537 bytes, unknown tool, refused and malformed calls; the next snapshot equal; no reply holds the refused id, the
  tool name or the filler) · the module's tables · gate entry 10 count 0.
- **(security) id echo** — met: the `id` table, 128 echoed and 129 refused (mutation M3 red).
- **(obs) one `tool_call` span per call, `tool`/`ending` only, parent of `command`** — met: `stand_mcp_tools` (both
  modes; mutation M1 red) · `stand_act_spans` green (the command span's eight fields).
- **(obs, security) stderr at `trace`** — met: `mcp_log` 0 sentinel · 0 id · 0 name needles, one `tool_call` line per
  call, stdout protocol lines only — per-package (gate entry 4) and workspace (entry 5) builds.
- **(design)** — met: no ESC byte in any reply (`Speaker::reply`); gate entry 11 count 0.
- **(layouts)** — met: `mcp x` and `mcp --session d` read as usage (cli unit rows); `host_binary`, `cli_commands`,
  `cli_flow` green (gate entry 6).
- **(arch, security) `serde_json`** — met: named in the root workspace deps and escher-driver only; lock one line;
  adapter count 0; audit green; no JSON written through it.
- **(tests)** — met: `ci-leg.sh fast` and `doc` green (gate entries 15, 16); CI on `36877a23` `verdict: green · checks
  16/16` (attempt 2).
- Gates (implement's run `.andromeda/runs/2026-10-11T07-48-16-implement/`, by `run` text):
  - `cargo test -p escher-driver --locked` — green · exit 0 · lacks FAILED
  - `cargo build --locked -p seven_guis --bin escher-session && printf … initialize … | target/debug/escher-session mcp`
    (smoke) — green · exit 0 · contains `"protocolVersion":"2025-06-18"`
  - `cargo test -p blitz-tests --locked --test stand_mcp_tools` — green
  - `cargo test -p seven_guis --locked --test mcp_server --test mcp_log` — green
  - `cargo test --workspace --locked --test stand_mcp_tools --test mcp_server --test mcp_log` — green
  - `cargo test -p seven_guis --locked --test cli_commands --test cli_flow --test host_binary --test host_log --test
    host_timer` — green
  - `cargo test -p blitz-tests --locked --test stand_act_spans --test stand_act_refused --test stand_act_timer --test
    stand_session_lifecycle --test stand_snapshot_text` — green
  - `git diff -U0 69a399da… -- Cargo.lock | … | grep -c -v -x '+ "serde_json",'` — green · exit 1 · last line 0
  - `cargo tree --locked -p seven_guis -e normal | grep -c -E 'accesskit_(xplat|winit)'` — green · exit 1 · last line 0
  - `sed '/^#\[cfg(test)\]/,$d' packages/escher-driver/src/mcp.rs | grep -c -E '…'` — green · exit 1 · last line 0
  - `cat packages/escher-driver/src/*.rs | grep -c -E 'IsTerminal|is_terminal|isatty'` — green · exit 1 · last line 0
  - `git diff --quiet 69a399da… -- …execute.rs … deny.toml` — green · exit 0
  - `bash scripts/agent-run.sh boot` — green · exit 0
  - `bash scripts/agent-run.sh run stand` — green · exit 0 (127 · 0 · 6 over 33 files)
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0
  - `bash .github/scripts/ci-leg.sh doc` — green · exit 0
  - `bash .github/scripts/ci-leg.sh audit` — green · exit 0
  - `cargo build … && python3 …/evidence/live-trial.py` — `leg = 'operator'`: green · exit 0 · history unmoved
    (evidence/operator-pass.md, evidence/live-trial.md)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, fired as written:
    `hygiene: clean — read 47`
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.1.0` — `leg = 'operator'`: green · exit 0 · history moved `refs/remotes/origin/build/escher-0.1.0
    69a399da→36877a23` (the intended push)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg =
    'operator'`: first firing `verdict: red` (attempt 1, `Build wasm examples`, the runner's rustup download); after the
    operator-directed rerun `verdict: green · checks 16/16 · attempt 2 · wall 1267 s` — the reading this chunk takes.
  - Mutation controls M1–M5 each turned their check red, file restored by sha256 (evidence/controls.md).
  - Smoke: the boot path changed (a new role, `FnMut` boot); the smoke entry green twice; `agent-run.sh status`
    truthful.
- Watches: none folded.
- Outcome basis: the operator pass ran — Setup 4's commit list `36877a23 chore(2026-10-10-mcp-surface): operator pre-CI
  commit, …` (one commit, no fix commits) and the final HEAD's CI run CI#38124028789 attempt 2, recorded in
  evidence/operator-pass.md; implement's P4 report (this conversation) for everything else. Uncommitted since the
  pre-CI commit: `evidence/operator-pass.md` only.
- Process hygiene: implement's census — none left running; re-measured at wrap P1 (`ps -eo pid,comm,args` for
  `escher-session`, `seven_guis`, test binaries, cargo, rustc, `claude -p`): none. `target/mcp-trial/` holds the live
  trial's four files (overwriting the earlier stand-in dry run's), not committed.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 69a399da (the parent of the oldest pre-CI commit 36877a23) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 1 line(s) in 1 range(s)
added: 3100
### Cargo.toml — added 1 line(s) in 1 range(s)
added: 166
### README.md — added 1 line(s) in 1 range(s)
added: 48
### examples/seven_guis/src/session_host.rs — added 2 line(s) in 1 range(s)
added: 4-5
### examples/seven_guis/tests/common/mod.rs — added 173 line(s) in 2 range(s)
added: 4-6 · 166-335
- 166-195 @170 «pub fn string_at(json: &str, key: &str) -> Option<String> {»
  - 174-194 «loop {»
- 200-208 @203 «pub struct Speaker {»
- 210-217 @213 «pub struct Spoke {»
- 219-327 «impl Speaker {»
  - 220-255 @221 «pub fn spawn(mut command: std::process::Command) -> Speaker {»
  - 257-267 @258 «pub fn send(&mut self, line: &[u8]) {»
  - 269-278 @273 «pub fn reply_bytes(&self) -> Vec<u8> {»
  - 280-292 @283 «pub fn reply(&self) -> String {»
  - 294-326 @296 «pub fn close(mut self) -> Spoke {»
- 329-334 «impl Drop for Speaker {»
  - 330-333 «fn drop(&mut self) {»
### examples/seven_guis/tests/host_binary.rs — added 3 line(s) in 2 range(s)
added: 145-146 · 150
### examples/seven_guis/tests/mcp_log.rs — new file · 240 line(s)
- 27-36 @29 «const TOOL_CALL_FIELDS: [&str; 6] = [»
- 41-45 «fn tool_call(id: u32, name: &str, arguments: &str) -> String {»
  - 42-44 «format!(»
- 47-240 @48 «fn the_mcp_role_logs_no_id_no_name_and_no_typed_text_at_trace() {»
  - 53-59 «let ids: Vec<String> = harness»
  - 60-68 «let text_of = |selector: &str| -> Vec<String> {»
  - 71-74 «assert!(»
  - 77-82 «assert!(»
  - 88-90 «speaker.send(»
  - 92-106 @94 «let calls = [»
  - 108-119 «for (index, (name, arguments, _, is_error)) in calls.iter().enumerate() {»
  - 123-126 «let ids_answered = ids»
  - 127-131 «assert!(»
  - 132-135 «assert!(»
  - 138-142 «assert_eq!(»
  - 143-147 «assert!(»
  - 148-151 «let protocol = replies»
  - 152-156 «assert!(»
  - 160-163 «let unstamped = stderr»
  - 165-168 «assert_eq!(»
  - 170-177 «let span_lines = |span: &str| -> Vec<&str> {»
  - 179-185 «assert_eq!(»
  - 186-209 «for (index, line) in tool_calls.iter().enumerate() {»
  - 210-213 «let refused = tool_calls»
  - 218-221 «assert_eq!(»
  - 223-228 «let found = |needles: &[String]| {»
  - 232-239 «assert!(»
### examples/seven_guis/tests/mcp_server.rs — new file · 278 line(s)
- 37-41 «fn mcp() -> Speaker {»
- 43-47 «fn tool_call(id: u32, name: &str, arguments: &str) -> String {»
  - 44-46 «format!(»
- 49-69 @51 «fn result(reply: &str, id: u32) -> (bool, String) {»
  - 52-57 «assert!(»
  - 58-66 «let is_error = if reply.ends_with(r#""isError":false}}"#) {»
- 71-76 @73 «fn call(speaker: &mut Speaker, id: u32, name: &str, arguments: &str) -> (bool, String) {»
- 78-84 @80 «fn accepted(speaker: &mut Speaker, id: u32, name: &str, arguments: &str) -> String {»
- 86-93 @87 «fn names(listing: &str) -> Vec<String> {»
  - 88-92 «listing»
- 95-235 @96 «fn the_mcp_role_completes_the_counter_flow_by_tool_calls_alone() {»
  - 100-102 «speaker.send(»
  - 105-109 «assert!(»
  - 116-119 «assert!(»
  - 122-125 «assert!(»
  - 128-131 «assert!(»
  - 133-136 «assert!(»
  - 137-143 «for id in 5..8 {»
  - 147-151 «assert_eq!(»
  - 154-160 @155 «let (is_error, refused) = call(»
  - 161-164 «assert!(»
  - 165-168 «assert!(»
  - 170-173 «assert!(»
  - 174-177 «assert!(»
  - 191-198 «for (row, (reply, id, code)) in errors.iter().enumerate() {»
  - 200-203 «assert!(»
  - 209-213 «assert_eq!(»
  - 214-218 «assert!(»
  - 220-225 @221 «let replies = [&refused, &malformed]»
  - 226-233 «let leaks = replies»
- 237-278 @239 «fn a_fresh_counters_snapshot_reads_the_same_through_mcp_and_the_command_line() {»
  - 246-250 «assert_eq!(»
  - 257-260 «let _started = Started {»
  - 263-264 «let through_command_line =»
  - 268-271 «assert!(»
  - 272-277 «assert!(»
### packages/escher-driver/Cargo.toml — added 1 line(s) in 1 range(s)
added: 16
### packages/escher-driver/src/cli.rs — added 62 line(s) in 24 range(s)
added: 15 · 35-37 · 47 · 55-58 · 69 · 78 · 81 · 105-106 · 135-141 · 231-232 · 266 · 271-276 · 281-282 · 300 · 321-325
       339-345 · 442-445 · 644 · 696-702 · 717 · 753 · 765 · 768 · 777
  - 135-141 «if word == MCP_ROLE {»
### packages/escher-driver/src/json.rs — added 8 line(s) in 8 range(s)
added: 16 · 45 · 51 · 69 · 73 · 78 · 83 · 87
### packages/escher-driver/src/lib.rs — added 11 line(s) in 4 range(s)
added: 43-44 · 66-72 · 91 · 105
### packages/escher-driver/src/mcp.rs — new file · 1204 line(s)
- 42-47 @44 «const START_APP: &str = match schema::START.args {»
- 49-111 @83 «pub fn serve_mcp(»
  - 90-95 «let mut server = Server {»
  - 96-110 «loop {»
- 113-115 «fn io_error(error: io::Error) -> SessionError {»
- 117-126 @119 «enum Request {»
- 128-168 @130 «fn read_request(input: &mut impl BufRead) -> io::Result<Request> {»
  - 133-167 «loop {»
- 170-176 @172 «enum Id {»
- 178-199 «impl Id {»
  - 179-186 @180 «fn admitted(value: &Value) -> Option<Id> {»
  - 188-198 «fn written(&self) -> String {»
- 201-209 @203 «enum ProtocolError {»
- 211-230 «impl ProtocolError {»
  - 212-219 «const fn code(self) -> i32 {»
  - 221-229 «const fn message(self) -> &'static str {»
- 232-245 @234 «enum Envelope {»
  - 239-244 @240 «Request {»
- 247-277 @249 «fn envelope(line: &[u8]) -> Envelope {»
  - 250-252 «let Ok(text) = std::str::from_utf8(line) else {»
  - 253-255 «let Ok(value) = serde_json::from_str::<Value>(text) else {»
  - 256-258 «let Value::Object(mut request) = value else {»
  - 261-267 «let method = match (spoken, request.remove("method")) {»
  - 268-276 «match id {»
- 279-285 «fn reply(id: &Id, member: &str, value: &str) -> String {»
- 287-289 «fn result_reply(id: &Id, result: &str) -> String {»
- 291-296 «fn error_reply(id: &Id, error: ProtocolError) -> String {»
- 298-302 «fn write_line(output: &mut impl Write, line: &str) -> io::Result<()> {»
- 304-308 @305 «fn list(items: impl IntoIterator<Item = String>) -> String {»
- 310-317 @311 «fn strings<'a>(items: impl IntoIterator<Item = &'a str>) -> String {»
  - 312-316 «list(items.into_iter().map(|item| {»
- 319-339 @321 «fn initialized(params: Option<&Value>) -> String {»
  - 322-324 «let asked = params»
  - 325-328 «let version = PROTOCOL_VERSIONS»
- 341-390 @343 «fn tool(verb: &VerbSpec, apps: &[&str]) -> String {»
  - 345-371 «for arg in verb.args {»
  - 375-383 «input_schema.json(»
- 392-397 @393 «fn tool_list(apps: &[&str]) -> String {»
- 399-423 @401 «fn call_of(verb: &'static VerbSpec, arguments: &Map<String, Value>) -> Result<Call, Refusal> {»
  - 403-418 «for (name, value) in arguments {»
  - 419-422 «Ok(Call {»
- 425-431 @427 «enum Ending {»
- 433-441 «impl Ending {»
  - 434-440 «const fn word(self) -> &'static str {»
- 443-448 @445 «struct ToolResult {»
- 450-491 «impl ToolResult {»
  - 451-456 «fn accepted(text: String) -> ToolResult {»
  - 458-463 «fn refused(refusal: Refusal) -> ToolResult {»
  - 465-470 «fn failed(error: SessionError) -> ToolResult {»
  - 472-480 @474 «fn bounded(self) -> ToolResult {»
  - 482-490 «fn written(&self) -> String {»
- 493-496 @494 «fn tool_call_span() -> Span {»
- 498-505 @499 «struct Held {»
- 507-512 «struct Server<'a, B> {»
- 514-652 «impl<B: FnMut(&str) -> Result<Session, SessionError>> Server<'_, B> {»
  - 515-522 @516 «fn expire(&mut self) {»
  - 524-552 «fn handle(&mut self, line: &[u8], output: &mut impl Write) -> io::Result<()> {»
  - 554-588 «fn tool_call(»
  - 590-619 «fn run(&mut self, verb: &'static VerbSpec, call: &Call) -> ToolResult {»
  - 621-642 «fn start(&mut self, app: String) -> ToolResult {»
  - 644-651 «fn hello(&self, held: &Held) -> Hello {»
- 654-1204 @655 «mod tests {»
  - 666-668 «fn never(_: &str) -> Result<Session, SessionError> {»
  - 670-673 @671 «fn empty(app: &str) -> Result<Session, SessionError> {»
  - 675-686 @676 «fn exchange(»
  - 688-690 «fn request(id: &str, method: &str, params: &str) -> String {»
  - 692-698 «fn tool_call(id: u32, name: &str, arguments: &str) -> String {»
  - 700-702 «fn error(id: &str, code: i32, message: &str) -> String {»
  - 704-706 «fn tool_reply(id: u32, result: &ToolResult) -> String {»
  - 708-712 «fn lines(requests: &[String]) -> Vec<u8> {»
  - 714-771 @715 «fn each_line_is_read_by_its_rules() {»
  - 773-811 @774 «fn a_line_is_read_whole_across_reads_and_an_unended_last_line_is_one() {»
  - 813-845 @814 «fn an_id_is_echoed_only_in_its_two_forms() {»
  - 847-922 @848 «fn each_method_gets_its_reply() {»
  - 924-989 @925 «fn each_argument_value_becomes_its_kind_or_refuses_the_call() {»
  - 991-1010 @992 «fn listed(verb: &VerbSpec) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {»
  - 1012-1077 @1013 «fn the_listing_is_the_nine_verbs_of_the_table() {»
  - 1079-1165 @1080 «fn the_session_verbs_are_answered_by_the_server() {»
  - 1167-1183 @1168 «fn a_session_idle_for_its_expiry_is_dropped_when_a_request_arrives() {»
  - 1185-1203 @1186 «fn a_result_over_the_answer_bound_is_replaced_by_the_session_error() {»
### tests/blitz-tests/tests/stand_mcp_tools.rs — new file · 406 line(s)
- 27-33 @28 «const TASKS: [LeanTask; 4] = [»
- 44-53 @45 «fn boot(incremental: bool) -> impl FnMut(&str) -> Result<Session, SessionError> {»
  - 46-52 «move |app| {»
- 55-57 «fn request(id: usize, method: &str, params: &str) -> String {»
- 59-65 «fn tool_call(id: usize, name: &str, arguments: &str) -> String {»
  - 60-64 «request(»
- 67-86 @68 «fn serve(requests: &[String], incremental: bool) -> Vec<String> {»
  - 73-79 «serve_mcp(»
  - 81-85 «String::from_utf8(output)»
- 88-122 @91 «fn result(reply: &str) -> (bool, String) {»
  - 92-94 «let (_, rest) = reply»
  - 97-115 «loop {»
  - 117-120 «assert!(»
- 124-131 @126 «struct Seen {»
- 133-138 @134 «struct Recorded {»
- 146-154 «impl Visit for Fields<'_> {»
  - 147-149 «fn record_str(&mut self, field: &Field, value: &str) {»
  - 151-153 «fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {»
- 156-158 «fn index(id: &Id) -> usize {»
- 160-207 «impl Subscriber for Recorder {»
  - 161-163 «fn enabled(&self, _: &Metadata<'_>) -> bool {»
  - 165-181 «fn new_span(&self, attributes: &Attributes<'_>) -> Id {»
  - 183-187 «fn record(&self, span: &Id, values: &Record<'_>) {»
  - 193-195 «fn enter(&self, span: &Id) {»
  - 197-206 «fn exit(&self, span: &Id) {»
- 209-381 @210 «fn a_scripted_mcp_session_completes_the_counter_flow_and_parents_each_command() {»
  - 212-376 «for incremental in [false, true] {»
  - 377-380 «assert!(»
- 383-406 @384 «fn a_held_timer_gets_its_time_step_through_start() {»
  - 385-405 «for incremental in [false, true] {»
