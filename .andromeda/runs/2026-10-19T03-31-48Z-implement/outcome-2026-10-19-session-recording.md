# Implement outcome — 2026-10-19-session-recording

**Outcome: green + smoke.** Next → the OPERATOR PASS (plan entries 15 hygiene, 16 the pre-push gate and push, 17 the
CI read), then `/andromeda-wrap-session`.

## Gates
- Block run twice (`.andromeda/runs/2026-10-19T03-31-48Z-implement/gate-2026-10-19-session-recording.json`).
  - First run: `entries 17 · green 10 · red 4 (6,9,11,13) · recorded 0 · timeout 0 · not-run 3`. All four reds had
    one cause: `stand_session_lifecycle.rs:135` "row 0: a request outside the protocol is refused as malformed".
    The test wrote raw `call v2 …` grammar rows, which wire `v3` reads as version refusals.
  - After the fix: `entries 17 · green 14 · red 0 · recorded 0 · timeout 0 · not-run 3` (15–17 `leg operator`).
- Coverage: `packages/escher-driver 4906/4993 = 98.26 % (floor 97 %)` · dioxus-native-dom 84.74 % ·
  blitz-test-harness 93.95 % — `coverage floor: 3 of 3 crates at or above their floors`.
- stand-repeat: `stand-repeat: 3 of 3 rounds green` (313 s).
- No deferral: every language the chunk touched ran its gates. Matrix: no capability claimed by this chunk
  (`claimed by 2026-10-19-session-recording: 0`), so no ref was written.

## Smoke (P3)
- Fired on the boot-path condition: the `escher-session` command line changed. The plan lists no smoke entry, so it
  was driven by hand.
- `bash scripts/agent-run.sh boot` → ready; `run stand_session_recording` → 3 passed, `cargo_exit 0`; `status`
  truthful.
- Real binary: `escher-session start counter --session target/tmp/smk-s-0408 --record target/tmp/smk-rec-0408`, then
  `click`, `snapshot` and `stop`, all exit 0.
  - The run left exactly one `check-1.json`, `-rw-------`, in a `drwx------` directory, and the state directory was gone.
  - The check holds the opening record and two steps: calls, acceptance and 16-hex digests, no screen reading.
  - `target/tmp/smk-rec-0408/` is left in place (gitignored); the host's guard refuses a recursive rm.

## Files changed
- **New (3):**
  - `packages/escher-driver/src/recording.rs`
  - `tests/blitz-tests/tests/stand_session_recording.rs`
  - `examples/seven_guis/tests/recording.rs`
- **Modified (25):**
  - `packages/dioxus-native-dom/src/{snapshot,lib}.rs`
  - `packages/escher-driver/src/{command,session,execute,error,describe,wire,host,client,mcp,mcp_tests,cli,lib,json}.rs`
    and `packages/escher-driver/src/mcp/result.rs`
  - `packages/blitz-dom/src/iframe.rs`
  - `tests/blitz-tests/tests/{session_common/mod,virtual_clock_engine,stand_session_lifecycle}.rs`
  - `examples/seven_guis/tests/{host_log,mcp_log,cli_commands}.rs`
  - `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`
- **Scope:** `scope: clean — changed 28 · listed 28 · recorded 0`. Every edit lies inside research's two lists, and no
  scope-record line was needed.

## Deviations from plan, with justifications
1. **`stand_session_lifecycle.rs` edited.** The plan said it stays unchanged under F1 A, but it pins the wire version.
   - Its four grammar rows move `v2` → `v3`, and `call v2 snapshot` now reads `refused version` beside `call v1`.
   - Research's Files-to-modify list carries the file.
2. **Arguments are written in the schema's order, not the order they crossed.**
   - An MCP call's arguments come out of a `serde_json` map in a different order from argv's. Schema order is what
     makes the command line's and the `mcp` role's checks byte-identical (proven by `examples/seven_guis/tests/recording.rs`).
   - It is also the order `validate` checks arguments in, so a replay meets every rule the same way.
3. **A call no record can spell is not recorded:** a verb outside the seven instance verbs, or an argument the
   verb's schema does not name or passes twice.
   - Each is refused before it runs.
   - The reader's grammar (step 3) cannot hold such a call, so a replay of it would differ.
4. **Recording is unix-only.** On other platforms `Session::recording_into` returns `SessionError::Unsupported`,
   matching the session socket. The check's owner-only modes are unix modes.
5. **What the recorder does with files it did not create.** It never removes one. A temporary file already standing
   at its rewrite path (`.check-{n}.json.tmp`) fails the write, and the recording ends with only its own check removed.
   - A unit test found the first version removing that temporary file. It was fixed in `recording.rs` and is pinned in
     the `recording`, `session` and `mcp_tests` units.
6. **`mcp --record <open dir>` ends as a protocol role does:** exit 3, its fixed message on stderr, stdout empty.
   Stdout belongs to the protocol.
7. **`--record` on any verb but `start` reads as usage** (exit 2) once the call itself has passed `validate`.
   - A call the schema refuses stays a refusal, the same order `--session` keeps.
   - Before this chunk, a stray `--record` was an unnamed argument (exit 1).
8. **Unit-test temp dirs:** escher-driver's new units write under the workspace's `target/tmp`, reached by the
   compile-time `CARGO_MANIFEST_DIR`, which is already a registered read. No `TMPDIR` read was added.
9. **`MASKED_STAND_IN` is `"masked"`.** It is a non-empty fixed text, so a replayed `type` into a password field gives
   the same masked diff the original typing did.
10. **Unchanged after all:** `schema_tests.rs` (it pins no session-error count) and `examples/seven_guis/tests/common/mod.rs`
    (no new runner was needed). dioxus-native-dom gained one unit test for `is_masked_input`.

## Surfaced (for the wrap — nothing authored)
- **S1 · plan step 8 against step 5 — `help`'s line.** Step 8 says "`help`'s line … byte-identical to before the
  chunk", but step 5 grows `SESSION_ERRORS` from 13 to 15, and `describe()` serves that table.
  - So `help`'s line now holds two more session errors, `record-dir-not-private` and `recording-failed`.
  - The acceptance's own reading holds: `help`'s line and `tools/list` are byte-identical with and without `--record`
    (`examples/seven_guis/tests/recording.rs`).
  - The verb table and `tools/list` are unchanged.
- **S2 · a `type` whose target never resolves is kept masked.** This covers a `type` refused `stale` or `not-found`.
  - The plan keeps `masked` for "a `type` whose target was masked".
  - When no control resolves, the target cannot be read, so the recorder writes `MASKED_STAND_IN` and `masked:true`
    rather than keep text that may be a password.
  - Replay fidelity holds, because a refusal holds nothing of the text.
  - A `type` refused by `validate` keeps its text as it crossed, because a `text` out of its bound decides that
    refusal. The wrap's F2 card should name this rule.
- Counts the wrap re-reads (plan §Implementation notes):
  - `SESSION_ERRORS` reads 15 (`command grep -c 'kind: "' packages/escher-driver/src/error.rs` → 15).
  - escher-driver private modules 14 → 15 (`recording`); the wire `v2` → `v3`.
  - blitz-tests files 134 → 135, stand-repeat `stand_*` files 42 → 43, seven_guis test targets 7 → 8.
  - `session_common` readers 23 → 24: `stand_session_recording` joins, and the module gains `replay`, `kept` and
    `mode`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `escher-session serve counter` host (pid 2566779, the smoke's `start`) | this run | terminated — `stop` answered `{"stopped":true}` |
| hosts and `mcp` children the gate entries started | a leg (the gate block's entries) | terminated — each held behind its kill-and-reap guard |
| census `--comm escher-session --comm seven_guis_nati --args 'serve counter' --args 'serve crud' --args 'escher-session mcp'` | — | `census: none running` |

## Decisions & corrections
- **Guard refusal, recurrence of host-linux «Paths & the working directory».** A `cd` into a chunk or skill
  references directory was refused 3 times (`bash-guard.sh`: "cd … moves the session cwd"). Each was re-issued once
  with absolute paths.
- **Guard refusal.** A `cat` heredoc append to `packages/blitz-dom/src/iframe.rs` was refused ("a cat/tee heredoc with
  a file target"). It was re-issued once through the Edit tool.
- **Guard refusal.** A recursive `rm` of a smoke directory was refused by the host's deny rule. The call was
  re-issued once with a fresh unique directory, and the old target was left.
- **Self-correction.** An inline python heredoc edit script nesting raw triple quotes failed with a SyntaxError and
  changed nothing. It was re-run from a scratchpad file by path.
