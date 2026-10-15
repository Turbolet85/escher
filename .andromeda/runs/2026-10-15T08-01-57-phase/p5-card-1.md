# P5 review — 2026-10-15-cold-agent-test · "Cold-agent test" (the last entry of escher-0.1.0)

Plan: `escher-0.1.0/chunks/2026-10-15-cold-agent-test/plan.md` (386 lines, 12 steps, 14 gate entries, 8 criteria) ·
research: `…/research.md` · scope: `…/scope.md` (amended at validation-1 to the founder's answers).

## Goal
Swap the cold-agent pipe from its toy stub to the driver's real MCP surface and spend the founder's ONE live run on it.
A fresh `claude -p` session, whose only tools are the eleven `mcp__escher__*` tools of the built `escher-session mcp`,
does all four lean stand tasks in one session (counter → 3; a return flight 24.12.2026 → 31.12.2026; the timer set to
10 s and run full; in CRUD, Ada Lovelace added, Tisch renamed Tischler, Hans Emil deleted). It then writes a JSON check
that the pipe replays on a fresh session with no model. Wrong calls are counted from the server's own `tool_call` span
lines and decide nothing: they are the 0.2.0 baseline. No engine, driver, schema, telemetry, harness or stand change.
No mutation runs (inputs#I1).

## The founder's answers this plan carries (inputs#I2, relayed verbatim by the overseer as his chosen option labels)
- Q1 «Один запуск, до $3 (Рекомендую)»: ONE live run on the dev host with the operator's own login.
  `--model claude-opus-5-5`, `--max-budget-usd 3`, `timeout 600`. The result stands as read, never re-fired without his
  word. It also ratifies the swap's crossing (playbook Boundary widening :36): the stand's screen now reaches the model
  provider on the pipe's live run.
- Q2 «Все четыре в одной сессии» (not the recommendation): all four lean tasks in that one run, under the one cap.
- Q3 «JSON-проверка с повтором (Рекомендую)»: the check is a JSON array of tool calls, each optionally with an
  `expect` list, handed over as the reply's last ```json block. It is replayed with no model, and replayed again with
  its acting steps left out to show it can fail.
- Q4 «Отложить на 0.2.0, наблюдать (Рекомендую)»: `covered` on `task-header-spacer` stays as it reads. The live run
  counts `spacer_calls`, and a residuals line owns the decision (below).

## Top acceptance criteria
1. v010-15: the one live run (`cargo build --locked -p seven_guis --bin escher-session && bash scripts/cold-agent.sh
   run stand`) reads `passed`, with `isolated`, `counts_agree`, all four apps in `tasks_met` (each from the last
   `snapshot` answer of its app, never the reply), `check_passed` and `check_can_fail`. `wrong_calls` is recorded and
   decides nothing. Recorded at `evidence/live-verdict.json`.
2. The contract tests under the `claude` shim and a fake `escher-session` are green, along with the CI-scripts leg.
   No live model, credential or built binary is used in CI.
3. Usage gives exit 2 before any lookup (`counter` is no longer a task). The prompt names no stand id. The census
   finds no listener, network library or env read.
4. Events and verdict hold no scrub key and none of the planted markers. The transcript copy is host-path-masked, and
   hygiene reads clean.
5. No file under `packages`, `examples`, `tests`, `apps`, `wpt` or the manifests moves. `fast` and `doc` are green,
   and the pushed sha's CI is green.

## Touchpoints
- `scripts/cold-agent.sh`: the escher server behind `sh -c 'exec env RUST_LOG=info "$1" mcp 2>"$2"'`, the `stand`
  task and its fixed prompt, the restated verdict, the done-state atoms (recorded at P4 from real screens), and the
  check's extraction, validation and two replays.
- `scripts/cold_agent_stub.py`: deleted.
- `.github/scripts/test_cold_agent.py`: the stub class retires; the pipe cases run over a fake server.
- `escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/`: the live run's record.

## Test commands (14 entries)
- Non-operator: `bash -n` · the cold-agent unittest · stub gone · env/listener census · no Rust/stand diff from
  `01297a12` · the CI-scripts leg · `fast` · `doc` · the evidence reader (fired after the live run).
  - Baselines and controls: `p5-controls.md` (each guard red on its must-fail input, green on its must-pass one, in a
    scratch clone).
- Operator: `claude --version` · THE live run (fired once by /implement P2) · hygiene · the pre-push gate with the
  clean-tree guard and push · `ci.py conclusion --wait 1500`.
- No entry but the live one can start a model session.

## Concretization preview — v010-15 (the operator's never-weaken checkpoint)
- **Now:** "A fresh agent with no prior context, given only the tool, completes a stand task and writes a check that
  passes; the run records its count of wrong calls."
- **Proposed:** "One live run of `bash scripts/cold-agent.sh run stand` on the operator host (the founder's one run,
  ≤ $3, claude-opus-5-5) reads `passed`, measured as follows:
  - **Fresh agent with only the tool:** a `claude -p` session whose only tools are the eleven `mcp__escher__*` tools
    of the built `escher-session mcp` (`isolated` true), run from a directory naming nothing.
  - **Tasks done:** it completes all four lean stand tasks (counter, flight booker, timer, CRUD), each read done from
    the last `snapshot` tool result of its app (`tasks_met` all four, never the reply).
  - **A check that passes:** the session ends with a JSON check the pipe replays on a fresh `escher-session mcp` with
    no model, every step answered without error and every expectation met (`check_passed` true).
  - **A check that can fail:** the same check with its acting steps left out misses at least one snapshot expectation
    in each app (`check_can_fail` true).
  - **Wrong calls:** the verdict records `wrong_calls` (refused and errored tool calls counted from the server's
    `tool_call` lines, plus foreign tools and permission denials). The server's lines agree with the transcript
    (`counts_agree` true). The count is the 0.1.0 baseline and decides nothing.

  As recorded at escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-verdict.json." Method stays
  `dynamic-external`. This strengthens the acceptance (four tasks, a replayed and ablated check), never weakens it. If
  the one run reads `failed`, no ref is written, the cap stays unverified and the result stands as read.

## Expected amendments (wrap), in plan.md
- test-plan §3 and §4 · security-plan §Input Validation, §Threat Model Summary, §Secret Management, §Logging · obs-plan
  §3/§6/§8/§9 · architecture §Occupied Resources and §Standard Contracts · layout-templates §Surface: cli.
- The Boundary widening entry (security-plan) carries the pattern and who answered: the founder, relayed verbatim by the
  overseer as his chosen option label (inputs#I2). So the wrap halts on it once, as the answer card.

## Beyond the plan
| # | proposal | owner / place | the founder's word |
|---|---|---|---|
| 1 | A `.andromeda/residuals.md` line, target 0.2.0: whether `covered` and its remedy are the right words for a box with no height (`task-header-spacer`, refused `covered` on all four lean tasks; pinned by `stand_act_ids::refused_at_boot`). Its evidence is this chunk's live-run `spacer_calls` and the 2026-10-10 trial's 0 calls on it. CARRY 3 on working-route.md:100 is spent into it. | the wrap's route-resolve writes the line | «Отложить на 0.2.0, наблюдать (Рекомендую)» (inputs#I2) |
