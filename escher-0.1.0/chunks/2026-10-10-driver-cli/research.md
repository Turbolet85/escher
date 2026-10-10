# Codebase Research — 2026-10-10-driver-cli

## Scope
- **Depth:** deep · **Reads:** 22 files · **Globs/Greps:** 9 batches · **Graph queries:** 3 (plane `rust`, trace `tree-query-2026-10-10-driver-cli.json` in the run dir)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, 1 Session Addition applied (a boot smoke bounded by `timeout N` never reads green on exit 124: an entry is authored so its own exit is 0) · `.claude/rules/testing.md` — 6 Session Additions applied (no deleting key in a check; `--no-fail-fast` before counting a red run's result lines; one stream when attributing cargo lines)
- **Platform issues consulted:** none — no runner-only bullet: the one CI row scope carries is a run in progress, not a red
- **External inputs:** `inputs#I1` — who decides which fork of this plan (the founder: what the agent meets; the operator: technical), and the founder's purpose for the surface as restated 2026-10-09

## Files inspected
- `packages/escher-driver/src/lib.rs` (full) — the crate doc states the sentences this chunk makes false: "Two requests cross it, `hello` and `stop`" (`:13-16`), "Nothing of the schema, of a call or of an outcome crosses the socket" (`:32-33`); 33 re-exports (`:63-74`).
- `packages/escher-driver/src/wire.rs` (full) — `Request { Hello, Stop }` (`:25-28`), `Reply` four arms (`:31-40`), `VERSION = "v1"` (`:22`), `MAX_REQUEST_BYTES = 64` (`:16`), `MAX_REPLY_BYTES = 128` (`:20`), `IO_BOUND` 2 s on each read and write (`:13`); a request is exactly two space-separated ASCII words (`:65-68`); 8 unit tests pin the grammar, one of them holds `snapshot v1` to `RefusedMalformed` (`:218`).
- `packages/escher-driver/src/host.rs` (full) — `serve(state_dir, session)` takes the `Session` and never runs it: the loop is handed `session.label()` alone (`:48`), answers one connection at a time on the thread that owns the instance (`:107-123`), and a broken connection changes nothing (`:119-120`). The state directory is created `0700` or refused when group- or other-open (`:62-78`), the socket set `0600` (`:95`).
- `packages/escher-driver/src/client.rs` (full) — `start(state_dir, host: std::process::Command, ready)` spawns the caller's command as given and polls `attach` every 25 ms (`:103-139`); `attach` and `stop` are one `exchange` each (`:82-101`); `exchange` reads the reply under `IO_BOUND` and `MAX_REPLY_BYTES` (`:153-177`) — a command's reply read through it would time out after 2 s and be cut at 128 bytes.
- `packages/escher-driver/src/error.rs` (full) — `SessionError`, 12 variants, every message fixed text (`:12-37`, `:39-68`); `NotSettled(Busy)` is returned by `Session::act` only.
- `packages/escher-driver/src/execute.rs` (`:1-340`) — `Outcome` four variants (`:18-60`); `Session::run` is the one entry (`:108-117`), `validate` first (`:120`); an instance that did not go quiet is `settled: false` with `busy` in an `Outcome`, never an error (`:15-16`); the click point is the centre of the snapshot's `bounds` plus the viewport's scroll (`:259-266`, `:298-302`), `in_view` is read at that centre (`:312-320`), `covered` is a hit (`:324-337`).
- `packages/escher-driver/src/command.rs` (`:1-250`) — `Call { verb: String, args: Vec<(String, ArgValue)> }` (`:21-26`), `ArgValue { Text, Number(i64), Flag }` (`:10-17`), `validate` matches the six verbs by name and falls through to `UnknownVerb` (`:191-210`).
- `packages/escher-driver/src/refusal.rs` (`:1-190`) — eight causes each with `name` · `meaning` · `remedy` as `&'static str` (`:43-105`); `Fault` five arms holding at most the schema's own argument name (`:119-130`); `Refusal` exposes `cause()` and `fault()` (`:168-175`).
- `packages/escher-driver/src/schema.rs` (`:1-300`) — the table already names every key an answer needs: `snapshot` returns one field `text` of kind `Text` (`:200-210`); the acting verbs return `settled` · `busy` · `added` · `removed` · `changed` (+ `advanced_ms`, + `in_view`) (`:147-198`); a node's nine fields are `NODE_FIELDS` = `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `focused` · `bounds` (`:33-35`); every verb, argument and field carries a `help` string (`:86`, `:131`, `:140`).
- `packages/escher-driver/src/session.rs` (`:1-185`) — `SeenIds::record` stores `id.to_string()` with no length check; the bound counts entries (`:48-65`); `MAX_SEEN_IDS = 4096` (`:25`).
- `packages/escher-driver/Cargo.toml` (full) — three dependencies, no `[features]`, no binary (`:13-16`).
- `examples/seven_guis/src/session_host.rs` (full) — closed argv `<task> <state-dir>` (`:24-32`), four slugs (`:13-18`), boots with `stand::options(true)` (`:39`, `:43`), the Timer's session carries `stand::timer_step` (`:45-48`), exits `2` usage · `0` after stop · `1` on an error (`:31`, `:51-56`), installs the sink first (`:20-22`).
- `examples/seven_guis/Cargo.toml` (full) — two `[[bin]]` tables: `seven_guis_native` (`:8-10`) and `escher-session` at `src/session_host.rs` (`:45-47`); the native block names `escher-driver` (`:36`).
- `examples/seven_guis/tests/host_binary.rs` · `host_log.rs` · `common/mod.rs` (full) — the binary is reached through `env!("CARGO_BIN_EXE_escher-session")`; `host_log` reads needles from an in-process boot, drains both streams from the spawn on, never uses `start`, asserts stdout empty, every stderr line stamped, 0 id and 0 name needles (`host_log.rs:42-158`); `host_binary` asserts stdout empty and exit `2` with nothing created on an unknown task (`host_binary.rs:72`, `:83-102`).
- `tests/blitz-tests/tests/session_common/mod.rs` (full) — the in-process call builders and readers, the `Host` guard, `READY` 60 s and `EXIT` 10 s.
- `packages/dioxus-native-dom/src/snapshot_text.rs` (`:1-76`) — `to_text` writes role in `Debug` spelling, name and id in `str`'s `Debug` quoting, state tokens, `@x,y wxh` with shortest-decimal numbers (`:23-28`, `:48-76`); `SNAPSHOT_TEXT_BUDGET = 10_000` is a ceiling a caller compares, never enforced (`:9-16`).
- `packages/dioxus-native-dom/src/{snapshot,snapshot_diff}.rs` (public items, by grep) — `SnapshotNode { id, role, name, state, bounds, children }`, `NodeState { enabled, checked, value, focused }`, `SnapshotDiff { added, removed, changed }` of `DiffNode { id, parent, role, name, state, bounds }`.
- `examples/seven_guis/src/stand.rs` (`:1-118`) — `options(incremental)` sets `incremental: Some(incremental)` (`:58-69`); `timer_step` delivers whole 100 ms ticks (`:86-98`).
- `packages/blitz-dom/src/events/pointer.rs` (`:505-544`) — a single click in a text input moves the caret to the click point (`driver.move_to_point`, `:516-534`).
- `packages/blitz-test-harness/src/input.rs` (`:100-109`, `:216-226`) — `click_at` is pointer down + up + pump; `type_text` presses one character key per `char`.
- `packages/blitz-dom/src/document.rs` (`:2250-2292`) · `packages/blitz-dom/src/layout/writing_mode.rs` (`:368-401`) — both bodies of `physical_unrounded_geometry` start the containing-block walk at the node itself and subtract each node's own `scroll_offset()` (`document.rs:2262-2267`; `writing_mode.rs:382-388`, `:398-399`); `get_client_bounding_rect` reads through it (`document.rs:2281`).
- `scripts/code-graph-cookbook.md` (full) — read before the queries.

## Graph impact (from the code-graph query; plane `rust`)
- **serve** — 3 call sites outside the crate: `main @ examples/seven_guis/src/session_host.rs:49`, `session_host @ tests/blitz-tests/tests/stand_session_lifecycle.rs:103`, `session_host_with_typed_text @ tests/blitz-tests/tests/stand_session_quiet.rs:101`. A changed signature or a `serve` that now runs calls reaches all three; the two test hosts are in-process `Session`s with no sink.
- **start · attach · stop** (the client's) — `the_binary_serves_and_stops @ examples/seven_guis/tests/host_binary.rs:37`/`:42`/`:46`, `the_host_logs_no_id_and_no_name_at_trace @ examples/seven_guis/tests/host_log.rs:91`/`:105`, `start_attach_stop_and_each_edge @ tests/blitz-tests/tests/stand_session_lifecycle.rs` (13 sites, `:44-93`), `nothing_of_the_screen_leaves_the_session_process @ tests/blitz-tests/tests/stand_session_quiet.rs:49`/`:52`/`:65`. Additive client functions leave them untouched. The name `start` also resolves `Session::start` (`session.rs:95`), whose callers are the in-process checks.
- **run** (`Session::run`) — 12 call sites, every one under `tests/blitz-tests/tests/` (`session_common/mod.rs:151`, `:194`; `stand_act_{diff,ids,refused,spans,timer}.rs`); none in `examples/` or in the host: no code path outside a test runs a driver command today.
- **validate** — called from `execute @ packages/escher-driver/src/execute.rs:120` and the crate's own unit tests only.
- **crate edges** — inbound to `escher-driver`: `seven_guis`, `blitz-tests`; outbound: `dioxus-native-dom`, `blitz-dom`, `blitz-test-harness`, `blitz-traits` (usage-based; the manifest names three). No `packages/` crate names `seven_guis`.
- **Names free** — `json`, `cli`, `to_json`, `command_line`: 0 symbols (probe: `Request`, `Reply`, `SeenIds` found on the same query, so the plane built).

## Patterns detected
- **The crossing does not exist, and the built wire cannot hold one** (`host.rs:48`; `wire.rs:16`, `:20`, `:25-28`): the host is handed the label only; a request is two words in at most 64 bytes while an `id` argument admits 1024 bytes and `text` 4096 (`schema.rs:5`, `:8`), and a snapshot's text has a 10,000-byte ceiling that is compared, not enforced. Any form on this socket states a new version, new request and reply bounds and a way to carry arbitrary text (an id holds `/`, `:`, `[`, `]`; typed text holds spaces and line breaks) — research's answer to the extracts' "can the wire be extended": yes, at the one bind site, by a second request class beside `hello` and `stop`.
- **A hosted command meets the client's 2 s read bound** (`wire.rs:13`; `client.rs:165-173`): the reply to a command is read under the lifecycle's `IO_BOUND`. On the stand a command returns in milliseconds, but the bound is a wall-clock limit on the answer, so the plan states the bound a command's reply is read under and what a client that hits it exits with.
- **The schema already states the JSON's keys** (`schema.rs:33-35`, `:147-210`): each verb's result fields and each node's nine fields are named, kinded and helped in the one table. A JSON answer keyed by those names adds no vocabulary; a JSON form that names anything else is a second statement. `snapshot`'s one result field is `text` — "the screen as one text" — so a string field is what the table says today, and a JSON view of the tree would be a new field kind in it.
- **A refusal's JSON is a walk over fixed strings** (`refusal.rs:43-105`, `:119-130`, `:168-175`): cause name, meaning, remedy, and for `malformed` the fault with the schema's own argument name. Nothing of the call can enter it.
- **Three endings exist in the types** (`execute.rs:15-16`, `:108`; `error.rs:12-37`): `Ok(Outcome)` with `settled` true or false, `Err(Refusal)`, and a `SessionError` at the lifecycle. A command line adds a fourth, usage. The host's standing statuses are `0` · `1` · `2` (`session_host.rs:31`, `:51-56`); the repository's scripts use `0` · `1` · `2` · `3` (agent-run, cold-agent).
- **Typing into a filled control lands inside the value** — by code read, not measured: `type` clicks the centre of the control then types (`execute.rs:133-139`), a single click in a text input moves the caret to the click point (`pointer.rs:516-534`), and `type_text` presses one character key per `char` at the caret (`input.rs:222-226`). The Flight Booker's two date inputs are filled at boot (`examples/seven_guis/src/tasks/flight_booker.rs:84`, `:92`). What the value reads after a `type` there is unmeasured; the plan's first check measures it in process, in both layout modes.
- **A scrolled box's bounds shift by its own offset in both engine bodies** (`document.rs:2262-2267`; `writing_mode.rs:382-388`): the walk subtracts the node's own scroll offset, so the reading holds whichever body a build compiles. The driver derives its click point and its `off-screen` reading from those bounds (`execute.rs:259-266`, `:312-320`), so the freight's hypothesis follows from the code; it is still not measured.
- **The two builds differ in engine features** — re-derived: `cargo tree --workspace --locked -e features -i blitz-dom` names 16 distinct `blitz-dom feature "…"` (with `tracing` and `writing-mode`); `cargo tree -p seven_guis --locked -e features -i blitz-dom` names 5 (`accessibility`, `accesskit`, `custom-widget`, `system-fonts`, `woff`; neither `tracing` nor `writing-mode`). Counted with `grep -o -E 'blitz-dom feature "[a-z_-]+"' | sort -u` on each output; a first, line-anchored pattern read 0 on both and was discarded. The `escher-session` binary's stderr by level under the workspace build stays unmeasured.
- **A check reaches a real binary only as its own package's** (`host_binary.rs:15`, `host_log.rs:11`): `CARGO_BIN_EXE_*` names the binaries of the package under test. A command line carried by the stand's own binary is reachable by the stand's tests with what exists; a client binary in another package is not reachable from a seven_guis test, and a test in that package cannot name the stand's host (no `packages/` manifest names an example).
- **The host reads one layout mode** (`session_host.rs:39`, `:43`): `stand::options(true)`. `stand_snapshot_text` already holds each screen's text equal across the two modes in process, so a one-mode host proof can be held against either.
- **Shell alone can branch and match** — `sh` is the one interpreter a flow needs: a command's exit status drives `if`, and `case "$out" in *'…'*)` matches a substring of stdout with no `grep`, `jq` or `sed`. Whether the JSON is one line matters to that: a single-line answer is matchable and survives `$(…)` unchanged.

## Conventions to follow
- **Closed argv, usage on stderr, exit 2 before anything runs** (`session_host.rs:24-32`; `host_binary.rs:83-102`).
- **Fixed-text errors with no path, label, id or value** (`error.rs:10`, `:76-101`); a JSON or stderr form of one renders the fixed text.
- **Tables stated in their unit test with the row count asserted; an assertion names a row index only** (`schema.rs:297-470`, `command.rs:243-594`).
- **Process checks**: `cfg(unix)`, own host started and stopped, every wait bounded, kill-and-reap guard, both streams drained from the spawn, failure prints kinds and counts (`host_log.rs:13-23`, `:125-158`; `tests/common/mod.rs:22-32`).
- **No panic path on what a call supplies** (`command.rs:159-241`): argv decoding and wire decoding are new input paths under the same rule; a panic message would reach stderr past the scrub.
- **Sink first in `main`, stderr only** (`session_host.rs:20-22`).

## New files to create
- `packages/escher-driver/src/json.rs` — the written JSON form of an outcome, a refusal, a session error and a session command's answer
- `packages/escher-driver/src/cli.rs` — argv to a `Call`, the command line's one entry function, the exit codes
- `examples/seven_guis/tests/cli_commands.rs` — every command through the real binary: stdout, stderr, exit code
- `examples/seven_guis/tests/cli_flow.rs` — the check that runs the shell stand flows
- `examples/seven_guis/tests/flows/` — the two shell stand flow scripts
- `examples/seven_guis/tests/host_timer.rs` — the timer host booted and moved by a hosted `advance`
- `tests/blitz-tests/tests/stand_act_filled.rs` — `type` replaces what a control holds, measured in both layout modes

## Files to modify
- `packages/escher-driver/src/refusal.rs` — the measured limit of `covered` in the cause's meaning (added at the P5 review, inputs#I4)
- `packages/escher-driver/src/schema.rs` — the levels, the three session rows, the name kind, the count kind, the measured limit of `bounds` in two help strings
- `packages/escher-driver/src/command.rs` — the two checks, one per level
- `packages/escher-driver/src/execute.rs` — `type` selects a value-holding control's content before typing
- `packages/escher-driver/src/session.rs` — the bound on a recorded id
- `packages/escher-driver/src/wire.rs` — the request and reply that carry a call and its answer, their bounds, the version
- `packages/escher-driver/src/host.rs` — `serve` runs a call on the session it holds and expires when idle
- `packages/escher-driver/src/client.rs` — the client function that sends a call and returns its answer
- `packages/escher-driver/src/error.rs` — the thirteenth session edge
- `packages/escher-driver/src/lib.rs` — the crate doc and the re-exports
- `examples/seven_guis/src/session_host.rs` — the binary's `main` hands argv to the command line
- `examples/seven_guis/tests/host_binary.rs` — the binary's argv, streams and exit codes restated
- `examples/seven_guis/tests/host_log.rs` — the typed sentinel, the command-span lines
- `examples/seven_guis/tests/common/mod.rs` — shared helpers for the process checks
- `tests/blitz-tests/tests/stand_act_scroll.rs` — the scrolled box's click point, pinned as measured
- `tests/blitz-tests/tests/stand_act_obstructed.rs` — `covered` under a scrolled-out row, pinned as measured
- `tests/blitz-tests/tests/stand_session_lifecycle.rs` — a caller of `serve` and of the client
- `tests/blitz-tests/tests/stand_session_quiet.rs` — a caller of `serve` and of the client
- `tests/blitz-tests/tests/session_common/mod.rs` — what the two session checks share
- `README.md` — the sentence that lists the driver as planned

## Open questions
- none — the three questions P3 carried were plan-decisions and are answered: the crossing and the eight surface forks by the founder (`inputs#I3`), the three technical forks by the operator (`inputs#I2`). The two lists above are the decided branch; the fork-branch files P3 listed (a client crate, a JSON dependency's manifest lines) are dropped.
