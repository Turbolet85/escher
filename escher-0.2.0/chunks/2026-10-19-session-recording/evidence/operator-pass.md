# Operator pass — 2026-10-19-session-recording

Fired by hand in plan order on the operator's direction (2026-10-19): «Run the operator pass … each entry by hand in
plan order … Stop and tell me if any entry is red; start no wrap.»

**Verdict: RED at entry 17.** CI#39199277925 failed `Test (windows)`. The pass stopped there and no wrap was started.

## Entry 15 — hygiene (fired as written, before the pre-CI commit)
```
gate v1.20 · 445f3409
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 113 (runs 108 · evidence 0 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
green — `contains hygiene: clean` holds.

## The pre-CI commit
`9da7b75dd chore(2026-10-19-session-recording): operator pre-CI commit, for the run this chunk's verdict reads`. Its
parent is `93336e886`. The tree was clean after the commit (`git status --short` read 0 rows).

## Entry 16 — `gate.py run --plan … --operator 16` (backgrounded)
```
operator entry 16 · history tripwire: git
 16 probe       green · exit 0 · 139.23s · 106844 B → 16.log · history moved: refs/remotes/origin/build/escher-0.2.0 93336e88→9da7b75d · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
not run — excluded · 16: 1-15,17
entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
```
green. The history move is the intended push of `9da7b75d` to `origin/build/escher-0.2.0`.

## Entry 17 — `ci.py conclusion --sha HEAD --wait 1800` (fired as written, backgrounded)
```
ci v1.3 · 2b582a9b
repo Turbolet85/escher (the push remote `origin`) · polled 31× over 929 s
9da7b75dd4ec verdict: red · checks 17/17 · first-fail +916 s Test (windows) · wall 916 s · runs CI#39199277925 complete…
runs: CI#39199277925 push completed/failure
failed 1: Test (windows) (failure)
log: gh api --allow-escape-sequences repos/Turbolet85/escher/actions/jobs/117889991186/logs
run failed: CI#39199277925 failure
```
**red** — `contains verdict: green` fails.

### The failing test, read from the job log (job 117889991186)
```
test a_recording_mcp_role_logs_nothing_of_its_checks_at_trace ... FAILED
thread 'a_recording_mcp_role_logs_nothing_of_its_checks_at_trace' (2524) panicked at examples\seven_guis\tests\mcp_log.rs:145:36:
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.33s
```
- **Cause, as read:** recording is unix-only by this chunk's design. `Session::recording_into` and
  `recording::prepare_dir` return `SessionError::Unsupported` off unix, so `escher-session mcp --record <dir>` ends
  with exit 3 before it answers.
- The new `mcp_log` test `a_recording_mcp_role_logs_nothing_of_its_checks_at_trace` is not `cfg(unix)`-gated. The
  file is gated only `not(target_arch = "wasm32")`.
- So on the Windows runner the test waits for a reply that never comes. It panics at `mcp_log.rs:145`, which is the
  snapshot reply's `string_at(...).expect`.
- This chunk's red, introduced by step 13's new test. The host's Linux gates cannot reproduce it, because there the
  recording works.
- **Not fixed: the operator's direction was to stop.** A fix would gate the recorded `mcp_log` test and its
  `recorded_needles` helper `cfg(unix)`, as `host_log` and `recording.rs` already are. It would land as a
  `fix(2026-10-19-session-recording): operator fix after CI run 39199277925, …` commit, re-pushed through entry 16 and
  re-read through entry 17.
- Cargo stops at the first failing target on that runner, so any seven_guis target after `mcp_log` went unrun on
  Windows in this run.

## Fix round 1 — on the operator's direction (2026-10-19)
The direction: «Gate that test and its helper with #[cfg(unix)]. Before committing, sweep every test this chunk added
or changed … for a recording path not gated to unix, and gate any you find the same way. Add a test that off unix
--record refuses with Unsupported … Commit it as the fix commit you named, then re-fire entries 16 and 17.»

- **Sweep:** one gap, `mcp_log`'s recorded test and its `recorded_needles` helper. Both were gated `#[cfg(unix)]`, along
  with the recorded-needle read inside `mcp_flow`. Every other recording path was already gated:
  - whole-file gates: `stand_session_recording.rs`, the seven_guis `recording.rs`, `cli_commands.rs`
  - per-item gates: the `host_log` recorded flow and its helper, the `mcp_tests` recording units, the `session` units,
    and the `recording` units (`cfg(all(test, unix))`)
  - the `cli`, `wire`, `describe`, `json` and `error` units touch no recording path.
- **Off-unix refusal pinned:**
  - `recording.rs` (`off_unix_tests`): `prepare_dir` and `Recording::create` return `Unsupported`.
  - `session.rs`: `recording_into` returns `Unsupported` and creates nothing.
  - `mcp_tests.rs`: a recording boot's `start` is answered `failed(Unsupported)`.
  - `examples/seven_guis/tests/recording.rs`, moved from a file-wide `cfg(unix)` to per-item gates:
    `mcp --record` ends with exit 3 and the fixed message on stderr, `start … --record` answers kind `unsupported`,
    and no directory is created.
- Linux before the commit: `cargo test -p escher-driver` 87 passed; `host_log` 2, `mcp_log` 2 and `recording` 3 passed;
  `ci-leg.sh clippy` exit 0; `cargo fmt --check` exit 0.
- **The `cfg(not(unix))` code could not be compiled on this host.** Only `x86_64-unknown-linux-gnu` is installed.
- Commit: `00a6c26bf fix(2026-10-19-session-recording): operator fix after CI run 39199277925, gate the recorded mcp_log check to unix and pin the off-unix refusal`.

### Entry 16, re-fired
```
operator entry 16 · history tripwire: git
 16 probe       green · exit 0 · 140.2s · 106945 B → 16.log · history moved: refs/remotes/origin/build/escher-0.2.0 9da7b75d→00a6c26b · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
not run — excluded · 16: 1-15,17
entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
```
green.

### Entry 17, re-fired
```
ci v1.3 · 2b582a9b
repo Turbolet85/escher (the push remote `origin`) · polled 22× over 651 s
00a6c26bf24c verdict: red · checks 17/17 · first-fail +653 s Test (windows) · runs CI#39201074261 in_progress/-
runs: CI#39201074261 push in_progress/-
failed 1: Test (windows) (failure)
log: gh api --allow-escape-sequences repos/Turbolet85/escher/actions/jobs/117895198720/logs
running 3: oldest Coverage report 356 s
run open: CI#39201074261 in_progress
```
**red.** Three jobs were still running when the first failure ended the wait.

**The failure, read from the job log (job 117895198720):** a compile error on Windows, from the fix itself.
```
error[E0425]: cannot find function `mode` in this scope
  --> examples\seven_guis\tests\recording.rs:67:13
67 |     assert!(mode(dir) == 0o700, "{surface}: the directory is owner-only");
note: found an item that was configured out
  --> examples\seven_guis\tests\recording.rs:40:4
39 | #[cfg(unix)]
error[E0425]: cannot find function `mode` in this scope
  --> examples\seven_guis\tests\recording.rs:69:13
error: could not compile `seven_guis` (test "recording") due to 2 previous errors
```
- Moving the file off its file-wide `cfg(unix)` left the helper `the_check`, which calls the unix-only `mode`, ungated.
- `#![cfg_attr(not(unix), allow(dead_code, …))]` silences an unused item, but it cannot excuse a call to an item that
  is configured out.
- The compile stopped the job, so no Windows test ran in this run.
- **Not fixed: the direction was to stop.** The fix would gate `the_check` `#[cfg(unix)]`.

## Fix round 2 — on the operator's direction (2026-10-19)
The direction: «Gate the_check with #[cfg(unix)] as the fix commit you named. Do NOT add the Windows target: no host
toolchain change while the founder is away. Instead, before committing, read recording.rs and every file this chunk
touched for an item gated to unix that an ungated item calls or names, and list each pair you checked in
operator-pass.md.»

- **No Windows target was added.** The host still holds only `x86_64-unknown-linux-gnu`.
- `the_check` (`examples/seven_guis/tests/recording.rs`) is gated `#[cfg(unix)]`.

### The audit
- **Basis.**
  - The 26 `.rs` files `git diff --name-only 93336e886 HEAD` names, read for every `cfg(unix)`, `cfg(not(unix))`,
    `cfg(all(test, unix))`, `cfg(all(test, not(unix)))` and `cfg_attr(not(unix), …)`.
  - 14 of the 26 carry a gate.
  - Each gated item's callers were found with `command grep -n` over its file. `mode(` and the `session_common`
    readers were searched over `tests/blitz-tests/tests/`.
- **The rule.** An item configured out off unix may be named only by an item that is itself configured out off
  unix, or by a twin defined under `cfg(not(unix))`.
- **Each pair below** reads: gated item → the item that names it → that naming item's own gate → verdict.

**`examples/seven_guis/tests/recording.rs`.** It carries `#![cfg(not(target_arch = "wasm32"))]` and
`#![cfg_attr(not(unix), allow(dead_code, unused_imports))]`.
- `use std::os::unix::fs::PermissionsExt` [unix] → `mode` [unix] · the test
  `a_recording_directory_open_to_its_group_is_refused_before_anything_runs` (`Permissions::from_mode`) [unix] — ok
- `use common::Started` [unix] → the test `the_command_line_and_the_mcp_role_keep_the_same_check` [unix] — ok
- `use escher_driver::SessionError` [not unix] → `off_unix_a_recording_directory_is_refused_as_unsupported`
  [not unix] — ok
- `fn mode` [unix] → `the_check` — **was ungated, the round-1 break; now [unix]** — ok
- `fn the_check` [unix, this round] → the test `the_command_line_and_the_mcp_role_keep_the_same_check` [unix] — ok
- the three unix tests [unix] → named by nothing — ok
- the off-unix test [not unix] → names only ungated items: `fresh`, `client`, `text`, `state_dir`, `string_at`,
  `Ran::line` and `SessionError` — ok
- the ungated helpers `fresh`, `request`, `tool_call`, `mcp`, `answered` and `BINARY` → each names only ungated items
  (`state_dir`, `fs`, `Command`, `Speaker`) — ok
- `common::Started` and `common::Host`, both `cfg(unix)` in `tests/common/mod.rs` → this file names `Started` under
  [unix] alone, and never names `Host` — ok

**`examples/seven_guis/tests/host_log.rs`.** It carries `#![cfg(not(target_arch = "wasm32"))]`.
- `BINARY` and `SENTINEL` [unix] → `host_flow` [unix] — ok
- `recorded_needles` [unix] → `host_flow` [unix] (L306) — ok
- `host_flow` [unix] → both tests [unix] — ok
- The only ungated item is `mod common;`, whose module is `#![allow(dead_code)]` — ok

**`examples/seven_guis/tests/mcp_log.rs`.** It carries `#![cfg(not(target_arch = "wasm32"))]`.
- `recorded_needles` [unix] → the statement `if let Some(kept) = record { … }` inside the ungated `mcp_flow` (L324),
  which is itself gated `#[cfg(unix)]` — ok
- `a_recording_mcp_role_logs_nothing_of_its_checks_at_trace` [unix] → named by nothing — ok
- off unix, `mcp_flow`'s `record` is still read at L140 (`--record`), and `found` and `lines` are still read by the
  ungated needle check — ok, no item is left orphaned

**`packages/escher-driver/src/recording.rs`.**
- `mod imp` [unix] and `mod imp` [not unix] each define `prepare_dir` (L198 · L271), `create_check` (L224 · L275),
  `rewrite` (L248 · L282) and `remove` (L259 · L286) with the same signatures.
  - Their ungated callers are `Recording::create` (L138, `create_check`), `Recording::record` (L170 `rewrite`,
    L172 `remove`) and `prepare_dir` (L182) — ok
- `mod off_unix_tests` [all(test, not unix)] → names the ungated `Path`, `prepare_dir`, `Recording` and
  `SessionError` — ok
- `mod tests` [all(test, unix)] → its `scratch` and `PermissionsExt` live inside it — ok

**`packages/escher-driver/src/session.rs`.**
- `scratch` [unix] → `a_recording_directory_open_to_others_is_refused_with_nothing_written` [unix] (L300) and
  `a_recording_that_ends_reads_failed_once` [unix] (L313) — ok
- `empty` [now ungated] → the two unix tests and `off_unix_a_recording_is_refused_as_unsupported` [not unix] — ok

**`packages/escher-driver/src/mcp_tests.rs`.**
- `scratch` [unix] → `each_session_a_recording_boot_starts_is_kept_as_one_check` [unix] (L617) and
  `a_call_that_ends_the_recording_is_answered_as_its_session_error` [unix] (L661) — ok
- `off_unix_a_recording_boot_answers_start_as_unsupported` [not unix] → names the ungated `empty`, `exchange`,
  `lines`, `tool_call`, `tool_reply` and `ToolResult` — ok

**`packages/escher-driver/src/client.rs` and `host.rs`.**
- Each `mod imp` [unix] has a twin `mod imp` [not unix] defining the same functions.
- Their ungated callers are `attach`, `stop`, `call` and `start` (client) and `serve` (host) — ok
- `read_answer` [`cfg_attr(not(unix), allow(dead_code))`] gains the arm `Reply::RecordingFailed`. It names the
  ungated `wire::Reply` — ok

**`packages/escher-driver/src/lib.rs`.**
- `mod wire` [`cfg_attr(not(unix), allow(dead_code))`] → `Reply::RecordingFailed` is constructed only in the unix
  `host::imp` and matched in the ungated `client::read_answer`. It is dead off unix and allowed — ok

**`packages/escher-driver/src/cli.rs`.**
- `an_argument_that_is_not_text_is_a_usage_error` [unix] → named by nothing — ok
- The new ungated code names `recording::prepare_dir` and `Session::recording_into`. Both are ungated with two
  `imp` arms — ok

**`tests/blitz-tests/tests/session_common/mod.rs`.**
- `mode` [unix] → `stand_session_recording.rs` L196 and L200, a file under `#![cfg(unix)]` — ok
- `stand_contrast.rs` and `stand_accessibility_ids.rs` call a `mode(incremental)` of their own, not this one — ok
- `replay` and `kept` [ungated] → name only the ungated `Check`, `Session::run` and `answer_digest` — ok

**Whole-file `#![cfg(unix)]`:** `stand_session_recording.rs`, `stand_session_lifecycle.rs`, `cli_commands.rs`.
- Configured out whole off unix, so no item outside any of them names an item inside — ok

**No unix gate:** `iframe.rs`, `dioxus-native-dom/src/{lib,snapshot}.rs`, `command.rs`, `describe.rs`, `error.rs`,
`execute.rs`, `json.rs`, `mcp.rs`, `mcp/result.rs`, `wire.rs`, `virtual_clock_engine.rs`. With nothing configured
out, no pair can break.

**Result:** one broken pair, `mode` ← `the_check`, now gated. No other pair names a configured-out item.
- **Named limit:** this is a read, not a compile. The `cfg(not(unix))` arms are compiled only by the Windows CI leg.

Linux after the gate: `cargo test -p seven_guis --locked --test recording` 3 passed; `cargo fmt --check` exit 0.
