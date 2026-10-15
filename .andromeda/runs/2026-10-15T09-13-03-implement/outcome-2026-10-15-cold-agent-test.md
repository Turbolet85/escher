# /implement outcome — 2026-10-15-cold-agent-test

**Outcome: green + live.** Every non-operator gate is green. The founder's one live run (entry 10), fired once on the
operator's word (inputs#I7), reads `passed`. v010-15 is `implemented` with its ref
`escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-verdict.json`. Smoke: the pipe's boot path was driven
end to end by entry 10 at P2, so P3 did not re-drive it (the plan lists no smoke entry).

Next → the OPERATOR PASS: entries 12 (`gate.py hygiene`, as written), then the pre-CI commit, 13 (fast, clean-tree
guard, push) and 14 (`ci.py conclusion`, as written). Then `/andromeda-wrap-session`.

## Files changed
- `scripts/cold-agent.sh` — steps 1–8:
  - the `stand` task, and the built `escher-session` as a third precondition;
  - the `escher` server behind the `sh -c 'exec env RUST_LOG=info "$1" mcp 2>"$2"'` wrapper;
  - the fixed `STAND_PROMPT`, and the client line with `--allowedTools mcp__escher --model claude-opus-5-5
    --max-budget-usd 3`;
  - the restated verdict: `isolated` on the eleven tools, the span-line count, `counts_agree`, `spacer_calls` and
    `tasks_met` from the last snapshot of each started app, with the done-state table;
  - the check's extraction, validation and two replays.
- `scripts/cold_agent_stub.py` — deleted (`git rm`, so the deletion is staged).
- `.github/scripts/test_cold_agent.py` — step 9:
  - `ColdAgentStubTest` retired;
  - `ColdAgentPipeTest` now runs under a fake `target/debug/escher-session` that speaks the eleven tools, holds one
    app, answers screens by acting-call count and writes span lines when `RUST_LOG=info`;
  - 33 tests.
- `escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/`:
  - `operator-9-claude-version.txt`, `live-run.md`;
  - `live-verdict.json`, `live-events.jsonl`, `live-check.json` (byte copies);
  - `live-transcript.jsonl` (host-path-masked, 103 lines, each still JSON).
- `escher-0.1.0/verification-matrix.json` — v010-15: ref set, `planned → implemented` (the ledger write).
- Scope read: `scope: clean — changed 3 · listed 3 · recorded 0`. No `scope-record.md` lines (no out-of-list edit).

## Gates (run dir `.andromeda/runs/2026-10-15T09-13-03-implement`)
| # | entry | reading |
|---|---|---|
| 1 | `bash -n scripts/cold-agent.sh` | green |
| 2 | unit, `test_cold_agent.py` | green, `Ran 33 tests … OK` (27 → 33; the plan predicted about 32) |
| 3 | stub gone | green |
| 4 | census grep | green, exit 1, last line 0 |
| 5 | no packages/examples/tests/apps/wpt/manifest change | green, 0 |
| 6 | `ci-leg.sh ci-scripts` | green, `Ran 100 tests … OK` (94 → 100; predicted about 99) |
| 7 | `ci-leg.sh fast` | green, 138.98 s (186 `test result: ok`, 0 FAILED) |
| 8 | `ci-leg.sh doc` | green, a cached rebuild (no Rust delta) |
| 9 | `claude --version` (operator) | green, `2.1.296 (Claude Code)`, history unmoved; recorded in `evidence/operator-9-claude-version.txt` |
| 10 | build + `cold-agent.sh run stand` (operator, the live run) | green, every atom held, artifact fresh, 53.18 s, history unmoved; recorded in `evidence/live-run.md` |
| 11 | evidence reader | red at step 10 (exit 2, no input yet — as the plan states), then green (`1`) after step 11 |
| 12–14 | hygiene, push, CI | the operator pass's — not fired |

- Fix-loop: 1 iteration. Entry 2's first firing was red because my new passing-run test hard-coded
  `check_expects: 7`, while its own EXPECT table sums to 6. The pipe was right; the literal became the table's sum.
- Red-before-green control (entry 2's baseline names it): the 33 new tests, run against HEAD `01297a12`'s stub-era
  pipe in a scratch copy, read `FAILED (failures=33, errors=3)`.
- No deferral: every gate ran.

## The live run (v010-15)
- **outcome `passed`**, `reasons []`, client exit 0, `isolated` true on the eleven `mcp__escher__*` tools;
- 43 tool calls = 43 server span lines (`counts_agree` true); `refused_calls` 0 · `error_calls` 0 · `wrong_calls` 0;
- `spacer_calls` 0 — the agent never touched `task-header-spacer`, so no `covered` was observed for the deferred
  decision;
- `tasks_met` all four;
- the agent's check: 39 steps, 10 expectations, `check_passed` true, `check_can_fail` true;
- claude-opus-5-5 on 2.1.296, 44 turns, 48.8 s, **0.3197 USD of the $3 cap**.
- Fired once; not re-fired.

## Deviations from the plan, with justification
1. **Reasons beside a missing or invalid check** (steps 5 and 7 leave this open):
   - `no-check` stands alone;
   - an invalid check reads `check-failed` alone (nothing replayed);
   - `check-cannot-fail` is added only once a check was replayed.
   - Why: each reason then names one fault. A check that never ran is not also "cannot fail".
2. **An accepted `stop` ends "the app started"** for `tasks_met` (step 5 says "the app of the latest accepted
   `start`"). A snapshot taken after an app is stopped therefore counts for no app. The test
   `test_an_app_without_a_final_snapshot_is_unmet` pins it. Why: after `stop` no app is started, so the server answers
   such a snapshot `no-session`.
3. **`check.json` is written with `indent=2`** — "as parsed (`json.dump`)" named no layout. It is never printed.
4. **The prompt adds two clauses within step 4's content**: "starting and stopping each app in turn" and "before that
   app is stopped" in the check's wording. It still names no element id and no app slug: the test checks the six id
   tokens, `flight-` among them.
5. **A missing or failed replay child** (a reply not seen within 30 s, EOF, or an MCP error frame) reads as a replay
   failure: `check_passed` false, and `check_can_fail` false for that replay.

## Surfaced
- None as a spec↔reality gap.
- For the operator pass: `gate.py hygiene` lists one in-root home path at
  `.andromeda/runs/2026-10-15T08-01-57-phase/.build.txt:13`. That is the phase run's file, not this run's; it needs
  `gate.py respell --file` before entry 12 reads clean. This run's six evidence files read clean.
- The live build printed blitz-paint's known `variant Intrinsic is never constructed` warning under the per-package
  build. It is the handoff's deferred learning; nothing here touched it.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `escher-session mcp` + its python probe (the P1 shape probe) | this run | terminated (stdin closed, waited) |
| unittest, `ci-leg.sh fast`/`doc`/`ci-scripts` trees (entries 1–8, 11) | this run, through gate.py | terminated |
| `claude -p` client, its `sh`→`escher-session mcp` child, the two replay `escher-session mcp` children (entry 10) | a leg (operator entry 10) | terminated |

Measured: at 2026-10-15T09:26:37Z, `ps -eo pid,comm,args` held no `escher-session`, `claude -p`, `cold-agent` or
`seven_guis` process.

## Decisions & corrections
- The operator's word to fire entry 10 (`inputs#I7`): «The operator's word, on the founder's own ratification «Один
  запуск, до $3 (Рекомендую)» (inputs, 2026-10-15). Fire exactly once. Whatever it reads stands; no re-fire.» Fired
  once.
- A guard refusal: the Bash guard's `cd` arm refused a `cd` into the chunk's `inputs/` at Setup. It was re-issued
  with absolute paths and ran.
- No recurrence of a standing learning was met. Every pipeline tool was run bare or redirected to a file and read
  whole.
