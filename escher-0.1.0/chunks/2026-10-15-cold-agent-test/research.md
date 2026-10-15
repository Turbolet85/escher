# Codebase Research — 2026-10-15-cold-agent-test

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 9 · **Graph queries:** 3 (rust plane)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (its cold-agent section and its one Session Addition, the `timeout` exit-124 trap for a gate entry); `.claude/rules/testing.md` — read in full (7 Session Additions; the `test_` helper-prefix trap in a `.github/scripts` unittest file applies to the contract tests)
- **Platform issues consulted:** none — no runner-only bullet; the one CI read (Setup 5a) is green
- **External inputs:** `inputs#I1` — the operator's arguments at take-up: tools bare or read whole; no mutation runs (the founder's standing word); every founder fork raised as a dialog, including how and where the live cold agent runs and what it may cost; every deferral a residuals line naming his label · `inputs#I2` — the founder's answers to the four P4 forks, relayed verbatim by the overseer as his chosen option labels (the overseer's record, snapped at P4) · `inputs#I3` — the P4 dialog's answer as it arrived, with the overseer's note planning all four lean tasks in the one run under the one $3 cap

## Files inspected
- `scripts/cold-agent.sh` (full, 285 lines) — the pipe: four verbs, embedded python3 `json` events and verdict (`EVENTS_PY`, :26-215), the stub spawned through a per-run `mcp.json` (:179-188), the client line (:256-258: `--tools "" --strict-mcp-config --mcp-config … --allowedTools mcp__stub --setting-sources "" --disable-slash-commands --no-session-persistence --max-budget-usd 1`, `timeout 600`), the stub-bound terms of the verdict: `STUB_TOOLS` (:35-36), `isolated` (:92-111, server name `stub`), `counts_agree` against the stub's `calls.jsonl` (:149), `final_count` from `stub-state.json` (:134-135, `TARGET_COUNT` 3, :37), the task allowlist `TASKS=(counter)` and its fixed prompt (:12, :15)
- `scripts/cold_agent_stub.py` (full, 171 lines) — the toy counter; nothing in it is reused by the swap
- `.github/scripts/test_cold_agent.py` (:1-125, :205-284) — 27 tests (`ColdAgentStubTest` 6 at :126, `ColdAgentPipeTest` 21 at :205; re-derived: `grep -n 'def test_'`, 27 hits); the `claude` shim (:25-83) reads the server named `stub` from `mcp.json` (:34-35) and drives it as a minimal MCP client; the pipe is copied into a temp root (:206-224) — a fake server there needs no built binary
- `escher-0.1.0/chunks/2026-10-10-mcp-surface/evidence/live-trial.py` (full) — the precedent for every mechanism this swap needs: the server spawned through `sh -c 'tee … | exec env RUST_LOG=info "$2" mcp 2>"$3"'` (:138) so its stderr lands in a file the client never touches; the call count read from that file as `span="tool_call"` lines with `ending="refused"` / `ending="error"` (:92-101); completion read from the last `mcp__escher__snapshot` tool result in the transcript (:77-89, `' "3" id="counter-value" '`); isolation with server name `escher` (:60-74)
- `escher-0.1.0/chunks/2026-10-10-mcp-surface/evidence/live-trial.md` — the trial fired once on 2026-10-11 with client `claude` 2.1.296: `client_exit 0 · isolated true · tool_calls 7 · refused 0 · errors 0 · end_state reached` (:16)
- `escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/evidence/live-run.md` — the one budgeted live run: `claude-opus-5-5`, 5 calls, 0 wrong, 0.0417662 USD, 12 s wall; the transcript copy masked of two host paths
- `packages/escher-driver/src/mcp.rs` (:60-117, :538-539) — `tool_call` span at INFO under `escher_driver`, fields `tool` and `ending` (`accepted` · `refused` · `error`), one per `tools/call`; a session verb and `help` leave no `command` child
- `examples/seven_guis/src/session_host.rs` (:8-48) — the binary's `TASKS` are the four lean tasks (`counter` · `flight-booker` · `timer` · `crud`) and the Timer's session takes `stand::timer_step` (:41); the `mcp` role boots through the same table (`packages/escher-driver/src/cli.rs:350`)
- `packages/escher-driver/src/refusal.rs` (:77-80, :111-113) — `covered`'s meaning (two measured limits) and remedy ("another element covers this one: act on the covering element or dismiss it first")
- `tests/blitz-tests/tests/stand_act_ids.rs` (:58-73) — `refused_at_boot` pins `task-header-spacer` → `Cause::Covered` on all four lean tasks
- `examples/seven_guis/src/app.rs` (:174, :329) — `div { id: "task-header-spacer" }`, styled by id
- `examples/seven_guis/tests/flows/counter.sh`, `flight.sh` (full) — the screen text a check can match: `Paragraph \"1\" id=\"counter-value\"`, `"name":"Booked one-way flight on 24.12.2026"`
- `.andromeda/playbook.md` (:33-74) — the escalate patterns read before P4's round (Boundary widening :36)
- `claude --version` / `claude --help` on the host — `2.1.296 (Claude Code)`, the trial's version; every flag the pipe passes is listed, and `--model <model>` exists (help :134)

## Graph impact
- **serve_mcp** — 12 call sites (`packages/escher-driver/src/cli.rs:16`, `:351`; `lib.rs:122`; mcp.rs unit tests; `stand_mcp_masked.rs:14`, `:97`; `stand_mcp_tools.rs:31`, `:86`, `:498`, `:527`) — unchanged by the swap: the pipe spawns the built binary's `mcp` role and calls no Rust symbol
- **Cause::Covered** — 1 definition (`packages/escher-driver/src/refusal.rs:25`), 18 references (`cli.rs:718-719`, `execute.rs:364`, `json.rs:676`, `refusal.rs:42/57/77/111`, `stand_act_ids.rs:62-69`, `stand_act_obstructed.rs:279/288/604`, `stand_act_spans.rs:680`, `stand_mcp_tools.rs:512/515`) — reached only if item 8 rewords a text: then the texts live at `refusal.rs:77` and `:111`, and a pin that reads a text back is a companion (none of the 18 sites names the text itself — they name the variant; a text pin is a name grep at P4 if the fork takes the rewording)

## Patterns detected
- **The server behind a stderr-capturing wrapper** (`live-trial.py:138`): `mcp.json`'s command is `sh -c 'exec env RUST_LOG=info "$1" mcp 2>"$2"'`, so the server's sink lands in a pipe-owned file at `info`, where `tool_call` lines print (obs-plan §6: no span line at the default `warn`).
- **Counting by span name** (`live-trial.py:92-101`): count lines holding ` span="tool_call" `, split by ` ending="refused" ` / ` ending="error" `; never raw lines (a workspace build adds `blitz_dom::document` INFO lines, obs-plan §6).
- **Completion from the tool's own answer** (`live-trial.py:77-89`): the text of the last `snapshot` tool result in the transcript is the screen the tool wrote; the agent's reply is never read for it.
- **A minimal MCP client in Python** (`test_cold_agent.py:36-51`): `initialize`, `notifications/initialized`, `tools/list`, `tools/call` over a child's pipes — the shape a pipe-side replay of the agent's check takes.

## Conventions to follow
- **Events and verdict through python3 `json` only, never bash strings** (`scripts/cold-agent.sh:24-26`); counts and identities only, no transcript text, argument, result or reply (`scripts/cold-agent.sh:24-25`).
- **Usage before precondition** (`scripts/cold-agent.sh:230-242`): exit 2 on a bad verb, count or task before `claude` or `python3` is looked for; exit 3 for a missing host tool — the built binary joins the preconditions.
- **State under `target/cold-agent/` only, the session dir a `mktemp -d` removed at exit** (`scripts/cold-agent.sh:244-266`).
- **Contract tests never name a helper `test_…`** (`.claude/rules/testing.md`, Session Additions 2026-10-06) and check `Ran N tests` against the cases written.

## New files to create
- `escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/` — the live run's record: live-run.md, the verdict, events, the agent's check and the host-path-masked transcript

## Files to modify
- `scripts/cold-agent.sh` — the swap: the escher server behind a stderr-capturing wrapper, the stand-task allowlist with fixed prompts naming no id, the isolation set of the eleven escher tools, the span-line call count, the completion oracle and the check's delivery and replay as P4 decides
- `scripts/cold_agent_stub.py` — removed with the stub (the shim tests stand in a fake server of their own)
- `.github/scripts/test_cold_agent.py` — the contract tests follow the swap under the shim with a fake `escher-session` in the temp root; the stub's test class retires

## Sweeps
- `cold-agent|cold_agent|mcp__stub` over every tracked file outside `.andromeda/`, `escher-0.1.0/` and `.claude/` (`git ls-files` piped to `/usr/bin/grep -l`): 5 hits · 3 changed (`scripts/cold-agent.sh`, `scripts/cold_agent_stub.py`, `.github/scripts/test_cold_agent.py`) · 2 no-change (`scripts/cold-agent.ps1` — a pass-through with no logic; `CLAUDE.md` — re-derived by the wrap's cascade)
- the same pattern over `.claude/`: 11 hits outside `backup/` (four rule files, six docs, `session-learnings.md`) · 0 changed · 11 no-change (curated by the wrap, never by implement)

## Done-state screens (recorded at P4, the oracle's atoms)
- Read on `target/debug/escher-session` built at `01297a12` (`cargo build --locked -p seven_guis --bin escher-session`, exit 0), each lean task driven to its done state through the command line under `target/tmp/cold-probe`, the final `snapshot` text read (the MCP `snapshot` result is the same `to_text` — mcp.rs:68):
  - counter, three clicks on `counter-increment`: `Paragraph "3" id="counter-value"`
  - flight booker, `flight-return` clicked, `24.12.2026` and `31.12.2026` typed, Book clicked: `Paragraph "Booked return flight: 24.12.2026 → 31.12.2026" id="flight-booked"`
  - timer, the slider clicked, `press arrow-left` ×10, `advance 10000`: `Paragraph "Elapsed: 10.0s" id="timer-elapsed"` and `Unknown "10.0s" id="timer-duration-value"` (with the duration left at 15, `advance 10000` also reads `Elapsed: 10.0s` — the duration line is what proves the slider moved)
  - crud, Ada Lovelace created, Tisch renamed Tischler and updated, Emil, Hans selected and deleted: `ListBoxOption "Lovelace, Ada"`, `ListBoxOption "Tischler, Roman"`, and no `id="crud-person-0"`
- Measured on the way: `press --key arrowleft` is refused `malformed` ("the value of `key` is outside its bound"); the key names are `arrow-left` etc. (`escher-session help`'s `key_names`) — a wrong call a fresh agent can make, counted, never deciding
- `escher-session help` writes 18,354 bytes (`len(json.dumps(...))` of its line) — what a fresh agent's first `help` costs in context

## Open questions
- none — the three P4 forks were answered by the founder (inputs#I2, inputs#I3): one live run on the dev host under $3; all four lean tasks in that one session; the replayable JSON check; `covered` deferred to 0.2.0 and observed
