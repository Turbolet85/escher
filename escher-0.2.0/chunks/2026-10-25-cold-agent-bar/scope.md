# Scope — 2026-10-25-cold-agent-bar

**Working entry (verbatim, escher-0.2.0/working-route.md:55):** Cold-agent bar — fresh agent over 0.2.0's new
capabilities via the cold-agent pipe, operator's login; judged by the founder's bar from 0.1.0's baseline (v020-11).
No freight block stands on the line (`route.py pins`: freight blocks 0).

**Cap:** v020-11 «Cold-agent bar» — requirement: 0.2.0 ends with a cold-agent run over its new capabilities, judged
against a wrong-call bar the founder sets from 0.1.0's baseline of 0 wrong in 43 (intent §7; requirements.md: «covers
v020-04…v020-09 end to end»). Acceptance (as routed, `method: dynamic-external`): a fresh agent given only the tool
exercises 0.2.0's new capabilities in one recorded run whose wrong-call count meets the bar the founder set from 0.1.0's
baseline, and the check it writes passes on replay.

**The founder's word (inputs#I1, relayed by overseer1; the operator's directive inputs#I2):** asked which app the cold
agent tests on, he chose option 1, extend the stand, and widened it: «поскольку живого проекта на котором можно
потестить нет то думаю надо расширять стенд, чтоб он покрывал все проверки, так что первый вариант наверно». The
stand is to cover ALL the checks, the load wait included. «наверно» is his hedge, not a delegate's answer. The phase
confirms the direction at P4 together with the wrong-call bar, which stays his fork at P4 (inputs#I2). This answers his
route ruling Q1 «Решить в том пункте (Рекомендую)», which put the task reach to this chunk
(escher-0.2.0/chunks/2026-10-16-handler-splits-and-readme/inputs/I2-route-0.2.0-founder-answers.md.txt).

## What this chunk builds
- **The stand extended so the driver serves every 0.2.0 check.** Verified at P3: `start` boots four lean tasks
  (examples/seven_guis/src/session_host.rs:13-18). The select, the load wait and the screen in parts are proven only
  on fixtures (`stand_act_select`, `stand_load_wait`, `stand_snapshot_parts`) that `start` does not boot (research.md
  §Files inspected). Three moves:
  - **`cells` served to the driver.** Verified, and measured at P3 (research.md §Measured): Cells' screen reads 47,949
    bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`. Every driver call answers in under 0.5 s in both layout
    modes, far inside the 30 s reply bound. A committed formula reads in the snapshot (`=A0*2` → `"10"` on `cell-B1`),
    and `unkeyed_actionable` reads 0. `lint` at boot reads 490 findings, the grid being wider and taller than the
    viewport, and that reading is recorded as measured. `Task::Cells` and its `task_in_shell` mount already exist
    (app.rs:14, :122-124). It joins the stand through a boot of its own beside `LeanTask`, never as a fifth variant:
    `LeanTask` has 466 reference sites in 49 files, and 22 `stand_*` files iterate `LeanTask::ALL` against tables
    measured for four (research.md §Graph impact).
  - **Flight booker's one-way / return choice a real `select`.** Verified: today it is two buttons (flight_booker.rs:
    66-81). A controlled select is supported, since `oninput` delivers the chosen option's value
    (dioxus_document.rs:377-382; select_interaction.rs:34-46). The pins it moves are 13 test files
    (research.md §Sweeps). [premise-corrected: `examples/seven_guis/tests/flows/flight.sh` books one-way and touches no
    mode button (flight.sh:34-61), so it does not move; `git grep -P 'flight-(one-way|return)(?!-date)'` holds 0 hits
    outside `tests/blitz-tests/tests/` and `flight_booker.rs`]
  - **A load in flight on the stand.** Verified premise: the stand boots offline (`stand::options`, `net_provider:
    None`, stand.rs:58-69), and the fixtures hold a load through `HeldNet` (fixtures/mod.rs:87-130) or the in-file
    `LateNet`. A per-boot provider through `..stand::options(_)` keeps every other boot offline
    (stand_load_wait.rs:66-70; `stand::options` has 27 referencing file names). How the stand carries a load in
    flight, and where, is the founder's fork at P4.
- **The cold-agent pipe given goals over 0.2.0's capabilities.** Verified: `scripts/cold-agent.sh` holds one task set
  `stand` (:14), a prompt naming four apps (:17-27) and a `DONE` text per app (:53-58). New goals reach v020-04 …
  v020-09. Its contract tests (`.github/scripts/test_cold_agent.py`, 575 lines) move with it. Found at P3:
  (a) a paged `snapshot` answers over MCP as its JSON line `{"text","part","parts"}`, whose quotes are escaped, so
  `tasks_met`'s substring reading must decode the line's `text` before matching a Cells goal. (b) The server's
  `tool_call` lines name `tool`, so the judge can count accepted calls per verb from `server.log` (closed words and
  counts only) to show `lint`, `screenshot`, `advance` and a part read were exercised.
- **One live run on the operator's login, judged by the founder's bar.** Verified: the pipe spawns `claude` (model
  `claude-opus-5-5`, `--max-budget-usd 3`, `timeout 600`, cold-agent.sh:476-478) over `escher-session mcp`. It counts
  wrong calls from the server's `tool_call` lines (:148-157, :352) and replays the agent's check on fresh children.
  0.1.0's run is the baseline: 4 tasks, 43 calls, 0 wrong, $0.32 (intent §7). The carriage of the stand's screen to
  the model provider was ratified for ONE run («Один запуск, до $3 (Рекомендую)», 2026-10-15, security-plan's
  cold-agent row and its sidecar entry, security-history.md). A second run carrying new screens (Cells' contents, the
  select's options, shots, lint findings) is the founder's fork at P4, a boundary widening by playbook :36 if he
  widens it.
- **The wrong-call bar** — the founder's fork at P4 (inputs#I1 «Still open», inputs#I2; residuals.md:9
  `absorbed:v020-11`), set from 0.1.0's baseline of 0 wrong in 43. Today `wrong_calls` decides nothing
  (cold-agent.sh:352). The bar enters the verdict as a reason.
- **«Recorded run … passes on replay».** [premise-corrected: the agent cannot start a recording, because `--record`
  is the launcher's flag (`start`, the host role, the `mcp` role) and no tool takes it. No command line replays a kept
  check, because `Check::replay` is in-process only (packages/escher-driver/src/replay.rs:46), reached by `stand_replay`.
  So «the check it writes» is the agent-written check, which the pipe replays on fresh children (cold-agent.sh:281-305),
  and «one recorded run» is the live `mcp` child launched with `--record` into the pipe's own state, kept owner-only
  and never committed or printed (security-plan §Data Protection, kept checks). The judge reads only that the kept
  check's step count agrees with the server's instance-verb calls.]

## CI read at take-up (Setup 5a)
- `e84dc8181` (the last wrap's flip, 2026-10-25-capability-statement): green · 17/17 checks · wall 1153 s ·
  CI#40193730816. No red or `not green` to fold.

## Boundaries
- No new driver verb, argument, refusal cause, session error, carrier, log field or span, and no engine change for
  every document. The stand is example code, and the select model and the load wait already stand. Verified: the
  select is controlled through existing events, and a load provider is injected per boot. A stand change that would
  need an engine change surfaces as a P4 fork, never a silent widening.
- The `start` app set moves the usage line and the MCP `start` tool's `enum`, which is the program's own fixed text on
  a ratified carrier (cli.rs:343-365, mcp/listing.rs:63-72), not a widening.
- The stand's coverage claims that move are re-pinned in the tests that hold them: every stand screen under the
  budget (CLAUDE.md's snapshot row), the lean tasks' tables, and `stand-repeat`'s targets. The masters' prose is the
  wrap's amendment, never a phase edit.
- No credential in code, fixtures, logs or committed evidence. The run's transcript and check are host-path-masked
  before commit (security-plan §Surfaces, `gate.py hygiene` P1). The deferred learning «Committed run-dir and evidence
  text must not spell a host temp path, even as prose» applies to every committed line.
- Viola, the capability statement's text (v020-10, verified) and any 0.3.0 matter are out. The statement's «the
  2026-10-15 live cold-agent run measured at eleven tools» line is the wrap's to re-read after this run.
- watch: the host-sink part read (residuals.md:38, target: next) becomes measurable once `cells` is served. Taking it
  is a plan choice, not an obligation of this entry.
