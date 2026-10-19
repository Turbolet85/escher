# Codebase Research — 2026-10-19-recorded-check-replay

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the
  `timeout` exit trap; no live leg here is bounded by `timeout`) · `.claude/rules/testing.md` — read in full, 9
  Session Additions applied (the windows-gnu `cargo check` for a moved `cfg(unix)` gate; searching `tests/`,
  `examples/*/tests/` and the crate's own `src/` for a pinned table or behaviour; `--no-fail-fast` for counts)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a's run is in progress, not folded) and no
  CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's take-up directive (F3 → B ratified; founder forks into one run-dir
  file; technical forks in dialog; the pre-CI subject the guide form; a windows-gnu `cargo check` before a push
  touching platform-gated code)

## Files inspected
- `packages/escher-driver/src/recording.rs` (1-440, 805-830) — the record writer `record_of` (`:94-118`: keys `tool`
  · `arguments` · `masked` · `accepted` · `digest`), `Recording::record` (`:150-175`, the digest computed from the
  `Result<Outcome, Refusal>` it is handed), the bounds (`:28-31`: 400 records, 1,048,576 bytes), the reader
  `Check::from_bytes` (`:316-335`) and `step_of` (`:391-421`, the closed key list at `:392-395`), `Step`'s four
  accessors (`:348-368`). The unit test row at `:818-820` refuses a record carrying `"answer"`.
- `packages/escher-driver/src/json.rs` (230-330) — `Outcome::to_json` (`:267`; a `snapshot` answers
  `{"text":…}` at `:269-273`), `Refusal::to_json` (`:312`), the shot's line `Outcome::Shot` writing `png` last
  (`:299-303`) and `shot_without_image` (`:256-258`, `pub(crate)`, the line the MCP text block carries). Its test
  pins `{"width":369,"height":420}` for the line without the image (`:642-644`).
- `packages/escher-driver/src/execute.rs` (120-160) — `Session::run` computes `masked`, runs the call, then hands
  `call`, `masked` and the result to the recorder (`:144-149`); every replayed step that goes through `run` is
  validated first (`execute` opens with `validate(call)?`, `:156`).
- `packages/escher-driver/src/session.rs` (90-160) — `recording_into`'s doc states «No screen reading is kept»
  (`:136-137`).
- `packages/escher-driver/src/lib.rs` (80-140) — the crate doc's recording paragraph (`:84-86`, «no screen
  reading») and the re-exports `Check, MASKED_STAND_IN, Step, answer_digest` (`:138`).
- `tests/blitz-tests/tests/session_common/mod.rs` (37-60, 240-300) — `hold(task, incremental)` boots a lean task and
  hands the Timer its step (`:37-51`); `replay(session, check)` runs each kept call through `Session::run` and
  returns `(accepted, digest)` (`:242-251`); `kept(check)` (`:254-260`).
- `tests/blitz-tests/tests/stand_session_recording.rs` (1-200) — the in-process round trip on the four lean tasks
  in both layout modes; its first test asserts that no name, value or snapshot-text line of the screens read occurs
  in the check (`a_recorded_session_replays_and_keeps_no_screen_reading`, `:127`), which the `answer` key reverses.
- `examples/seven_guis/tests/recording.rs` (1-30, 140-150) — the binary's recording checks: a CLI `start counter
  --record` and an `mcp --record` session leave byte-identical checks (`:143-144`).
- `examples/seven_guis/tests/host_log.rs` (36-60) and `mcp_log.rs` (69-80) — `recorded_needles` reads the kept check
  through `Check::from_bytes` and makes the directory's name and each digest a needle for stderr.
- `examples/seven_guis/src/session_host.rs` (whole) — the four lean tasks by slug; the Timer's session gets
  `stand::timer_step` (`:30-44`).
- `examples/seven_guis/src/stand.rs` (60-174) — `boot`, `boot_timer`, `timer_step`; the mount is
  `task_in_shell(props.task.task(), …)` (`stand_root`).
- `examples/seven_guis/src/app.rs` (80-170) — `task_in_shell` is `pub` (`:90`); the shell component `TaskShell` is
  private (`:163`), so a test cannot mount a variant task in the stand's own chrome today.
- `examples/seven_guis/src/tasks/counter.rs` (whole) — `Counter` is `pub`; ids `counter-value`, `counter-increment`;
  `onclick` adds 1.
- `.github/scripts/ci-leg.sh` (40-60) — `stand-repeat` takes every `tests/blitz-tests/tests/stand_*.rs` by glob
  (`:50-52`) and names seven_guis' eight process checks (`:57`).
- `scripts/agent-run.sh` (181-195) — `run stand` takes every `stand_*.rs` (`:181-185`).
- `.github/scripts/test_ci_workflows.py` (351-363) — `test_stand_repeat_leg_runs_three_rounds` pins words
  including `recording` and `stand_session_recording`.
- `.github/scripts/test_blitz_tests_targets.py` (whole) — auto-discovery; no manifest edit for a new file.
- `tests/blitz-tests/Cargo.toml` — `seven_guis` and `escher-driver` are dev-dependencies (`:22-23`).

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-19T07-10-33-phase/tree-query-2026-10-19-recorded-check-replay.json`)
- **from_bytes** — 18 call sites:
  - escher-driver's own tests: `recording.rs`, 11; `mcp_tests.rs:638`, `:651`;
  - blitz-tests: `stand_session_recording.rs:160`, `:290`;
  - seven_guis: `host_log.rs:46`, `mcp_log.rs:76`, `recording.rs:144`.

  Admitting `answer` changes no signature. Every caller still reads, and a check written with answers must still read
  back in all three seven_guis readers.
- **record_of** — 1 caller, `Recording::record` at `recording.rs:160`. The writer change is local. `Recording::record`
  is called by `Session::run` alone (`execute.rs:145`).
- **step_of** — 1 caller, `Check::from_bytes` at `recording.rs:331`.
- **answer_digest** — the re-export (`lib.rs:138`), `Recording::record` (`recording.rs:165`), `session_common::replay`
  (`:248`), `stand_session_recording.rs:148`, `:269` and escher-driver's tests. The digest's form does not change.
- **replay** / **kept** (session_common) — read by `stand_session_recording.rs` alone (`:186`, `:315`; `:180`, `:300`).
  If the replay runner moves into escher-driver, these two helpers and their one reader are the threading set.

## Patterns detected
- **The digest hashes the written line** (`recording.rs:41-47`): `Outcome::to_json` or `Refusal::to_json`. The same
  line is the answer «as the driver wrote it». A `snapshot` step's line is `{"text":…}`, not the bare MCP text block.
- **A shot's line without its image already exists** (`json.rs:256-258`): `shot_without_image(width, height)`, the
  MCP text block's line, pinned at `json.rs:642-644`.
- **Replay through the one entry point** (`session_common/mod.rs:242-251`): each kept call is run through
  `Session::run`, which validates first (`execute.rs:156`). Steps are re-validated by construction.
- **The kept form is written by hand** (`recording.rs:94-118`, the crate's `json::Object`). `serde_json` only reads
  (`recording.rs:321`).
- **A failure message names a mode, a task and a step index** (`stand_session_recording.rs:183`, `:189`). It never
  names an id, a name or a reading.

## Conventions to follow
- **One behaviour per blitz-tests file, both layout modes, `mod session_common;` after the `use` block**
  (`stand_session_recording.rs:26-30`). It is auto-discovered, so there is no manifest edit
  (`test_blitz_tests_targets.py:18-26`).
- **A check that records is `cfg(unix)`-gated item by item, beside an off-unix `Unsupported` check**
  (`examples/seven_guis/tests/recording.rs:17-20`). Replay itself reads a check and needs no unix mode: a replay
  target that only reads bytes and runs calls can run on every OS.
- **Test scratch under `CARGO_TARGET_TMPDIR`** (`session_common/mod.rs:299-301`).
- **The windows-gnu check after any `cfg(unix)` gate is moved** (`.claude/rules/testing.md`, 2026-10-19 addition;
  inputs#I1): `cargo check --tests --locked --target x86_64-pc-windows-gnu -p escher-driver -p seven_guis -p
  blitz-tests`.

## New files to create
- `tests/blitz-tests/tests/stand_replay.rs` — the replay target. Kept checks replayed on freshly booted stand
  sessions in both layout modes read green with every step equal. Each check against a variant whose behaviour
  differs reads red at the step that exercises the change, naming the step and the answer keys that differ.
- `tests/blitz-tests/checks/counter-cli.json` — the committed check of a Counter session recorded through the command
  line (`escher-session start counter --record`), as the binary wrote it (founder F1 A, inputs#I2).
- `tests/blitz-tests/checks/counter-mcp.json` — the committed check of a Counter session recorded through the `mcp`
  role (`escher-session mcp --record`), as the binary wrote it.
- `examples/seven_guis/tests/flows/record_checks.sh` — the POSIX `sh` recording of both sessions through the binary
  into a directory it is given: the freshness check runs it, and so does a deliberate re-record.
- `packages/escher-driver/src/replay.rs` — the replay runner as escher-driver API (T1, the operator's answer).

## Files to modify
- `packages/escher-driver/src/recording.rs` — write `answer` after `digest` (the written line, a shot's without
  `png`); `step_of` requires and reads it; unit tests (the `:818-820` row moves from refused to read; new rows
  for a step without `answer`, an `answer` of the wrong kind, an `answer` out of place).
- `packages/escher-driver/src/lib.rs` — the crate doc's recording paragraph (`:84-86`) and the re-exports of the
  replay types.
- `packages/escher-driver/src/session.rs` — `recording_into`'s doc (`:136-137`).
- `tests/blitz-tests/tests/stand_session_recording.rs` — its no-screen-reading assertion becomes «each kept answer
  is the line the driver wrote, a shot's without `png`; the masked sentinel occurs nowhere»; its replay through
  `Check::replay`.
- `tests/blitz-tests/tests/session_common/mod.rs` — `replay` gives way to `Check::replay`; `kept` stays.
- `examples/seven_guis/tests/recording.rs` — the freshness check: `record_checks.sh` run through the binary into a
  fresh directory reads byte-equal to the two committed checks (no hand edit).
- `examples/seven_guis/src/app.rs` — `TaskShell` made `pub`, a visibility change only (T2, the operator's answer),
  so the red witness mounts a variant task in the stand's own chrome.
- `.github/scripts/test_ci_workflows.py` — `test_stand_repeat_leg_runs_three_rounds` names `stand_replay` beside
  `stand_session_recording`.

## Sweeps
- `screen reading` over `packages/`, `examples/`, `tests/` (`*.rs`): 5 hits in 3 files (`recording.rs:40`,
  `lib.rs:85`, `session.rs:136-137`) · 3 changed · 0 no-change. The test name `keeps_no_screen_reading`
  (`stand_session_recording.rs:127`) is renamed with its assertion.
- `Check::from_bytes|from_bytes` readers (the graph): 18 sites · 0 changed by signature · 18 no-change (the reader's
  signature stands). `host_log.rs:46` and `mcp_log.rs:76` read a check the recorder writes, so it carries
  `answer` and still reads back. Their needles, the ids and names read from the same stand booted in process,
  already cover every id and name a kept answer can hold, so neither file changes. Seven_guis `recording` changes
  for the freshness check.
- `"answer"` over `packages/`, `examples/`, `tests/` (`*.rs`): 2 hits · 1 changed (`recording.rs:819`, the refused
  row) · 1 no-change (`blitz-vibey-script/tests/dom.rs:714`, an unrelated fetch fixture).

## Open questions
- Cross-OS determinism of a committed check. The test leg runs on linux, windows and macos. Replaying a check
  recorded on linux elsewhere has never been measured: a node's bounds in a diff or snapshot, the Timer's settle
  readings. Shots are byte-identical across processes on one host only (`stand_screenshot_repeat`). → blocks:
  implementation-scope (the committed scripts may need to stay to verbs whose answers hold across OSes; the CI run
  is the measurement).
- Whether a committed check stays under 1,048,576 bytes with every answer whole. A hosted `snapshot` line measured
  832–2,202 bytes (2026-10-10-driver-cli). A short script of tens of steps is far below the bound; a long session
  ends `recording-failed` as today. → blocks: implementation-scope (the scripts' length).
