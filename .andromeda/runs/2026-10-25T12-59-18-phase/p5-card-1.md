# P5 review — 2026-10-25-cold-agent-bar · «Cold-agent bar»

Plan: `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/plan.md`, 357 prose lines and a 27-entry gate fence.
Research: `research.md`. Scope: `scope.md`, closed at P3.

## Goal
0.2.0 ends with one cold-agent run over its new capabilities, judged by the founder's bar. The founder answered at P4
(inputs#I3): F1 «Да, весь стенд (Рекомендую)» · F2 «Страница загрузки рядом (Рекомендую)» · F3 «Один запуск, до $3
(Рекомендую)» · F4 «Не больше 5% (Рекомендую)». The chunk:
- extends the stand: `start` serves Cells and a load page beside the four lean tasks, and the flight booker's
  one-way / return choice becomes a real controlled `select`;
- gives the cold-agent pipe goals over recording, lint, `advance`, the load wait, the select and the read in parts,
  with the founder's 5 % bar as a verdict reason;
- fires the one live run on the operator's word.

No engine, driver, verb, cause, span field, dependency or lock changes.

## Top acceptance criteria
1. v020-11: the live run reads `passed` (the concretization below).
2. The two new stand checks are green in both layout modes: Cells read in parts with a formula in its last part, and
   the load page's click ends settled with its message and the clock unmoved.
3. The a11y leg and `stand-repeat` are green with the flight booker's select re-pinned: Tab orders, Enter/Space rows,
   the contrast row at or above 4.5, the select named "Flight type".
4. The contract tests cover every new verdict path (6 apps, paged decode, 5 % bar at 5/100 passes and 6/100 fails,
   capability unused, recording not kept).
5. `host_log` measures a Cells part read through the host's own sink: 0 id and name needles under both builds
   (residuals.md:38).
6. The no-engine-change count, the silence census and the pipe census read 0. `fast`, `doc`, `coverage` and the CI
   read are green.

## Touchpoints
- **New:** `stand_cells_served.rs` and `stand_load_page.rs`, and the chunk's `evidence/`.
- **Modified:**
  - stand code: `flight_booker.rs`, `stand.rs` (`boot_cells`, `boot_load`, the in-process stand-in),
    `session_host.rs`;
  - the shared tables: `common/mod.rs` and `session_common/mod.rs`;
  - 13 re-pinned stand checks plus `stand_diff` and `stand_snapshot`;
  - seven_guis' `cli_commands`, `host_log` and `common/mod.rs`;
  - the pipe: `scripts/cold-agent.sh` and `test_cold_agent.py`.

## Test commands (27 entries)
- **Baselines:** each new entry was baselined on the untouched tree at P5.
  - Entry 2 is red: no target yet, the subject the chunk builds.
  - Entry 24 is red: the record is absent.
  - Every other entry is green.
- **Readings that matter:** `run stand` 197 · 0 · 7; a11y 14 files · 84; `stand-repeat` 3 of 3 rounds green
  (198 lines, 837 passed); coverage 97.99 · 85.57 · 94.15 %.
- **Controls:** each inline guard has its must-pass and must-fail reading, fired as the entry in a scratch clone
  (`p5-controls.md`).
- **Operator legs (5):** `claude --version`, then THE LIVE RUN — reserved for the operator's word and fired once —
  then hygiene, the push, and the CI read.

## Concretization preview — v020-11 (the only cap claimed)
- **Current acceptance:** «A fresh agent given only the tool exercises 0.2.0's new capabilities in one recorded run
  whose wrong-call count meets the bar the founder set from 0.1.0's baseline, and the check it writes passes on
  replay.»
- **Proposed:** the founder's one live run (F3) of `cargo build --locked -p seven_guis --bin escher-session && bash
  scripts/cold-agent.sh run stand` reads `outcome: passed` at
  `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json`. The live server child runs
  `mcp --record` into the pipe's state, so the run is recorded. The verdict reads:
  - `isolated`: the twelve tools;
  - `counts_agree`;
  - `tasks_met` all six:
    - counter;
    - flight booker, through its select (v020-08);
    - timer, by `advance` on the one clock (v020-06);
    - CRUD;
    - Cells, read across its parts (v020-09);
    - the load page, after its load is waited out (v020-07);
  - `lint_calls` ≥ 1 (v020-05), `screenshot_calls` ≥ 1, `advance_calls` ≥ 1, `part_reads` ≥ 1;
  - `recording_kept`: every session left one kept check (v020-04);
  - `check_passed` and `check_can_fail`: the agent's check replayed on fresh children;
  - `wrong_calls` ≤ 5 % of `tool_calls`, by F4.
- **Never weakened:** each half of the current form stays. «given only the tool» is `isolated`. «exercises the new
  capabilities» is the six apps and the four counts. «one recorded run» is the `--record` child. «meets the bar» is
  the 5 % reason. «passes on replay» is `check_passed` and `check_can_fail`.
- **Corrected at P3:** the agent cannot start a recording, and no command line replays a kept check. So the replay
  half is the agent's own check, which the pipe replays, and the recording half is the `--record` child
  (scope.md, premise-corrected).
- **Gated?** No: nothing blocks it. The run waits only on the operator's word inside `/implement`.

## The sampler (forced, operator's word)
- architecture: `sampler: E carried 126 · the cascade lost 6 (conventions#2.1, design-philosophy#1.1, inherited-defaults#1.1, project-intent#3.1, project-intent#4.1, stack-and-technologies#1.11)`
- security-plan: `sampler: E carried 74 · the cascade lost 9 (authentication-authorization#2.5, dependency-security#1.0, dependency-security#2.1, dependency-security#6.1, error-handling#2.1, input-validation#1.35, secret-management#2.13, threat-model-summary#1.1 …)` — the ninth is threat-model-summary#5.1
- design-system: `sampler: E carried 24 · the cascade lost 1 (spacing#2.4)`
- test-plan: `sampler: E carried 97 · the cascade lost 7 (ci-integration#5.1, ci-integration#7.1, quality-gates-coverage-targets#2.1, test-strategy#4.1, test-strategy#5.1, unit-test-strategy#6.1, unit-test-strategy#7.1)`
- obs-plan: `sampler: E carried 40 · the cascade lost 3 (ci-integration#1.6, error-capture-reporting#1.2, error-capture-reporting#1.4)`
- a11y-plan: `sampler: E carried 63 · the cascade lost 4 (a11y-scope-summary#1.13, a11y-strategy#4.1, aria-patterns-roles#2.1, visual-design-verification#3.4)`

All 30 lost items stand in `plan.md` marked «(sampler)» (Implementation notes → Carried from the sampler; four also in
the gate notes and Constraints).

## Aggregates and flags
- B: 0 `No domain coverage`. C: the bindings reciprocate.
- Val-1: aligned. The host-sink part read the scope listed as `watch:` is taken by step 9.
- The arch sidecar summary printed `supersedes: UNRESOLVED 1`. It is noted, and no plan line rests on it.
- Wrap escalation: the live run's carriage is a boundary widening on the founder's own F3 answer (playbook :36). The
  wrap halts on it once, as the answer card (`plan.md` Expected amendments).

## Beyond the plan
| # | proposal the word approves | owner |
|---|---|---|
| 1 | `.andromeda/residuals.md:38` (the host-sink paged part read, unmeasured) is discharged by step 9's measurement | «Cold-agent bar» (working-route.md:55), this chunk's wrap route-resolve |
| 2 | `.andromeda/residuals.md:9` (the cold-agent wrong-call bar, `absorbed:v020-11`) is closed by the founder's F4, 5 % | «Cold-agent bar», this chunk's wrap route-resolve |
| 3 | `docs/capabilities.md`:288-289 (the 2026-10-15 run «with eleven tools, 43…») is re-read and restated against this run's measured verdict | «Cold-agent bar», this chunk's wrap (the phase does not edit it) |

Apply? (yes / review / cancel)
