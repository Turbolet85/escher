# Scope — 2026-10-15-cold-agent-test

**Working entry (escher-0.1.0/working-route.md:100):** Cold-agent test — fresh agent given only the tool completes a
stand task and writes a passing check, wrong calls counted (v010-15). The last entry of Epoch 6 and of escher-0.1.0.

**Capability:** v010-15 — "A fresh agent with no prior context, given only the tool, completes a stand task and writes
a check that passes; the run records its count of wrong calls." (verification-matrix.json#v010-15, `dynamic-external`,
unclaimed). Requirement line: covers v010-01…v010-14 end to end; the count is 0.1.0's baseline, a bar starts in 0.2.0
(requirements.md:21; intent §9).

**Operator's arguments (inputs#I1):** pipeline tools bare or read whole; the founder's word stands — no mutation runs;
every fork that is the founder's raised as a dialog, including how and where the live cold agent runs and what it may
cost; every deferral a `.andromeda/residuals.md` line naming his label.

**CI read at take-up (Setup 5a):** `01297a12` (the 2026-10-14-quality-gates wrap) — green, 17/17 checks, CI#38700638904,
wall 987 s. No red to disposition.

**Gate:** none — the entry carries no `BLOCKED-ON`.

## What this chunk builds

1. **The pipe swap (CARRY 1).** `scripts/cold-agent.sh` stops spawning the stdlib stub and gives the fresh session the
   driver's own MCP surface — the `escher-session mcp` role, the eleven verbs of `VERBS` as tools — and its task
   allowlist moves from the stub's toy `counter` to a stand task. Kept: the isolation check (the session's tools are the
   escher server's alone, one server, connected; no skills, no slash commands; `apiKeySource` none; a run directory whose
   path names nothing), the wrong-call count, and the verdict that reads `passed` only on positive evidence (test-plan §3).
2. **An independent call count** (verified at P3). The stub's own call log (`calls.jsonl`, `{seq, tool, outcome,
   cause}`) is what `counts_agree` checks the transcript against; with the stub gone the second count is the `tool_call`
   span line escher's sink writes on the `mcp` role's stderr per tool call — fields `tool` and `ending` (`accepted` ·
   `refused` · `error`) only (`packages/escher-driver/src/mcp.rs:81-85`, `:538-539`), printed at `info`, captured by a
   `sh -c 'exec env RUST_LOG=info … mcp 2>{file}'` wrapper in `mcp.json` and counted by span name, as the 2026-10-11 live
   trial did (`escher-0.1.0/chunks/2026-10-10-mcp-surface/evidence/live-trial.py:92-101`, `:138`).
3. **A completion oracle that is not the agent's reply** (verified at P3). The stub's state file (`final_count`) is
   gone, and the MCP server's session lives in the client's process and ends with it. The tool's own answer stands in:
   the text of the last `snapshot` tool result in the transcript is the screen the tool wrote (`live-trial.py:77-89`),
   or a fresh session the pipe drives itself — never what the agent says it did.
4. **"Writes a passing check"** (verified at P3: no form exists anywhere — intent §9, requirements.md:21 and the
   matrix acceptance name none). The check's form, its delivery by an agent holding only the tool, the runner and what
   makes it non-vacuous are this chunk's to decide (a P4 fork — inputs#I1 makes "how the live cold agent runs" the
   founder's).
5. **Contract tests** (verified at P3). `.github/scripts/test_cold_agent.py` — 27 tests, `ColdAgentStubTest` 6 and
   `ColdAgentPipeTest` 21, under a `claude` shim that drives the server `mcp.json` names (:34-36) — follows the swap: the
   stub's class retires, the pipe tests drive a fake `escher-session` in the temp root (the `ci-scripts` leg builds no
   binary), read the eleven tools, the span count and the oracle; still no live model and no credential in CI.
6. **The live run (CARRY 1 + inputs#I1).** One live run of the swapped pipe on a stand task, its verdict and its
   wrong-call count recorded as the chunk's evidence (transcript host-path-masked before any copy is committed). The
   crossings — the spawned `claude` client, the stdio server, the outbound model path, the operator's Claude Code login —
   were ratified by the founder, 2026-10-06; his budget ruling was ONE live run (spent by 2026-10-06-cold-agent-run-pipe),
   so a further live run needs the operator's word, and how and where it runs and what it may cost are raised as a
   founder dialog (inputs#I1). Never in CI. Answered at P4 by the founder (inputs#I2, inputs#I3): one live run on the
   dev host with the operator's own login, at most $3 («Один запуск, до $3 (Рекомендую)»), covering all four lean
   tasks — counter, flight booker, timer, CRUD — in that one session («Все четыре в одной сессии»), the agent's check a
   replayable JSON list of tool calls («JSON-проверка с повтором (Рекомендую)»). (Amended at P5 validation-1: the
   entry's "a stand task" is met by four in one run.)
7. **The wrong-call bar (CARRY 2).** The founder's ruling, 2026-10-06: 0.1.0 counts wrong calls and sets no bar; its run
   is the baseline that shows which values are reasonable, and a bar starts in 0.2.0 (`.andromeda/residuals.md:9`, open,
   target 0.2.0). This chunk records the count and decides nothing on it; the residuals line is the 0.2.0 owner, and the
   count it reads at take-up is this chunk's.
8. **`covered` for a box with no height (CARRY 3).** A driver `click` on `task-header-spacer`, in each of the four lean
   tasks, is refused `covered`; whether `covered` and its remedy text are the right words for such a box "is decided by
   nothing yet, and a cause's texts are reworded only with a recorded reason". The wrap placed it here, on the entry that
   counts an agent's wrong calls, with "move it if it reads wrong". This chunk decides it, observes it in the live run,
   or defers it — a deferral is a residuals line naming the founder's label (inputs#I1); a reworded cause is a schema
   change with a recorded reason. Answered at P4 by the founder (inputs#I2): observed in the live run and deferred to
   0.2.0 («Отложить на 0.2.0, наблюдать (Рекомендую)») — no text moves; the residuals line names his label.
9. **The first live trial as context (CARRY 4).** The 2026-10-10 trial (49 calls in 64 s on the CLI of `177d1652`, three
   tasks, 3 unintended failures all `state-dir-too-long`; record at `.andromeda/runs/2026-10-10T13-01-58-wrap/relay-1.md`)
   wrote no check and is not this test's run — v010-15's baseline is still to take. Not exercised there, so not measured:
   `press`, `--shift`, `advance`, `scroll`, the `timer` app and the idle expiry.

## Freight mechanism claims (verbatim, for P3's closure)

- Verified at P3: Measured at the 2026-10-06 live run (2026-10-06-cold-agent-run-pipe's report, Cross-project):
  "`--allowedTools mcp__stub` (server-wide) let the MCP calls run in `-p` mode, and `--tools ""` removes only the
  built-in tools." — the same flags with `--allowedTools mcp__escher` ran the 2026-10-11 trial `isolated true` on client
  2.1.296 (live-trial.md:16), and the host's client reads `2.1.296 (Claude Code)` today with every flag in its `--help`.
- Verified at P3: "its bounds are 0 high and the hit at its centre answers the header that holds it, while
  `timer-progress`, 0 wide at boot, is hit at its own centre and accepted, so a zero size does not predict the reading
  (measured at escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/stand-census.md)" — still pinned at HEAD by
  `stand_act_ids::refused_at_boot` (tests/blitz-tests/tests/stand_act_ids.rs:58-73), green on CI#38700638904.

## Boundaries

- No engine, driver, schema, telemetry or harness change is needed for the swap (verified at P3): the `mcp` role boots
  the four lean tasks through the stand binary's own table, the Timer with its step (examples/seven_guis/src/
  session_host.rs:13-41; packages/escher-driver/src/cli.rs:351), and its `tool_call` span exists. A change to a
  refusal's text (item 8, `packages/escher-driver/src/refusal.rs:77`, `:111`) is the one schema edit in reach, and only
  with a recorded reason.
- No new listener, port, env var or workspace crate; the run directory, `target/cold-agent/` and the session's own
  stdio stay the pipe's whole footprint (arch §Occupied Resources). Any new carrier of call or screen content is a
  boundary question for the founder.
- The live run is never fired in CI or in a loop (verification-harness rules); the contract tests stay shim-only.
- No mutation runs (inputs#I1): a check's power to fail is shown by baselines and shims, never a mutation control.
- The wrap amends the masters' cold-agent sections (test-plan §3 · security-plan §Input Validation's two cold-agent
  rows and §Logging · obs-plan §3/§6/§8 · arch's directory tree) and the verification-harness rule file to the measured
  pipe.
