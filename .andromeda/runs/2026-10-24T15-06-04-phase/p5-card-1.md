# P5 review card 1 — 2026-10-24-stand-requirement-sweep

Plan: `escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/plan.md` · research: `…/research.md` · scope: `…/scope.md`
· run dir `.andromeda/runs/2026-10-24T15-06-04-phase/` (resumed at P4 on the operator's word).

## Goal
Every 0.2.0 capability whose requirement is a behaviour of the screen (v020-03…09) is proven headless on the stand or
a fixture, with every witness inside `stand-repeat`'s three-round, zero-failure budget. The sweep re-proves and claims
nothing. C1 (the visual lint's readings that no check asserts) is closed. The new verbs' stderr reads id-free on
two more readings. The coverage floors are re-read and held.

## Settled at P4
- **The founder's own word** on the one fork, by question dialog in the overseer session, relayed verbatim:
  «Список в скрипте (Рекомендую)». It sets a fixed `--test` list of the ten non-`stand_` witnesses in
  `ci-leg.sh`'s stand-repeat, pinned in `test_ci_workflows.py`, with the files keeping their names. It ratifies the
  selection change. Snapped as `inputs#I1` (`escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/inputs/`).
- **Leans** (each named in the plan's Constraints / Implementation notes):
  - The record is a second table, `WITNESSES_020`, in `stand_requirement_sweep.rs`, with `WITNESSES` byte-unchanged.
  - C1's readings are pinned as the spec states them: limits, not defects. Changing `lint.rs` or `text_extent.rs`
    would be the founder's fork, and none is raised.
  - Both stderr readings are measured: a paged part read in `stand_act_spans`' sink capture, and the
    `start --record` clients' stderr at `trace` in `host_log`.
  - The coverage floors are re-read, with no raise and no blitz-dom floor.
  - No mutation run, and no re-read of v020-01's complexity and size instruments. Both caps are verified, and the
    record names their unit tests as run by the `test` leg.
  - Research's one new file is folded into `stand_lint.rs` and `text_extent.rs` (one behaviour per file, no new
    target), and research's two lists were rewritten to match (`scope lists: parsed 9 items`).

## Top acceptance criteria
- `stand_requirement_sweep::every_0_2_0_requirement_names_its_witness_inside_stand_repeat`: 9 rows in order, every
  named test found, and every integration witness inside stand-repeat by prefix or by the named list read from
  `ci-leg.sh`.
- `bash .github/scripts/ci-leg.sh stand-repeat` → `stand-repeat: 3 of 3 rounds green`, with the ten in every round.
- C1: `stand_lint` gains `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it` (predicted findings
  in the plan's step 2, measured at implement). `text_extent` gains three limit tests (RTL, vertical, scaled box),
  green under `-p blitz-tests` and `--workspace`.
- `host_log` 3 and `mcp_log` 2 pass under both builds. The clients read 0 id, name, sentinel, recording-path and
  kept-check needles at `trace`. `stand_act_spans` reads 36 calls per mode, a paged part 2 of ≥ 2 among them.
- `ci-leg.sh coverage` → `coverage floor: 3 of 3 crates at or above their floors`, the floors unchanged.
- No engine, driver, telemetry, stand, manifest, lock or committed-check change (`guard: clean`, the source guard 0).
  The a11y leg is unmoved, and the stand graph holds no platform adapter.
- `fast` and `doc` pass. The CI run on the pushed sha reads `verdict: green`, `Stand checks repeated` and
  `Coverage report` among its checks.

## Touchpoints
No new files. Modified: `tests/blitz-tests/tests/{stand_requirement_sweep,stand_lint,text_extent,stand_act_spans,stand_snapshot_parts}.rs`,
`tests/blitz-tests/tests/fixtures/mod.rs`, `examples/seven_guis/tests/host_log.rs`, `.github/scripts/ci-leg.sh`,
`.github/scripts/test_ci_workflows.py`.

## Test commands
23 `[[gate]]` entries: the per-package and workspace runs of the touched checks and of the ten witnesses, the windows
`cargo check` (a new `cfg(unix)` item), the silence census, the adapter census, the source guard, `gate.py guard`,
agent-run boot and `run stand`, the `a11y`, `stand-repeat`, `coverage`, `fast` and `doc` legs, the workspace tally,
and the operator pass (hygiene, push, `ci.py`, the per-OS tally). Baselines and controls: each new entry's
`baseline` is in the plan. Controls are in `.andromeda/runs/2026-10-24T15-06-04-phase/p5-controls.md`.

## Concretization preview
None — no capability is claimed. The pool is v020-10 · v020-11, owned by «Capability statement» and «Cold-agent bar».
The wrap writes dated notes on v020-03, -05, -06 and -08 (their witnesses now inside stand-repeat), with status, ref
and acceptance untouched.

## Sampler (the forced full read beside the cascade)
- architecture — sampler: E carried 139 · the cascade lost 0
- security-plan — sampler: E carried 84 · the cascade lost 2 (dependency-security#5.1, dependency-security#2.6)
- design-system — sampler: E carried 29 · the cascade lost 5 (motion#1.2, spacing#2.5, spacing#2.4, typography#1.9, typography#1.13)
- test-plan — sampler: E carried 193 · the cascade lost 4 (ci-integration#5.1, ci-integration#6.1, mocking-stubbing-discipline#1.6, unit-test-strategy#6.1)
- obs-plan — sampler: E carried 85 · the cascade lost 0
- a11y-plan — sampler: E carried 76 · the cascade lost 7 (a11y-scope-summary#1.9, a11y-strategy#1.1, a11y-strategy#1.2,
  a11y-strategy#2.4, aria-patterns-roles#2.5, keyboard-navigation#4.9, visual-design-verification#1.6)

All 18 (sampler) items are taken. Each is marked (sampler) where the plan cites it:
- security dependency-security#2.6 and #5.1: the source guard (entry 10, manifests, lock and `deny.toml`), plus
  Constraints.
- design motion#1.2: v020-06's row names the smooth-scroll test. spacing#2.5: step 2's page inside the body margin.
  spacing#2.4 and typography#1.9: Constraints. typography#1.13: step 3.
- tests ci-integration#5.1: the per-OS tally entry. ci-integration#6.1: the workspace tally and the coverage entry.
  mocking-stubbing-discipline#1.6: v020-07's row and Constraints. unit-test-strategy#6.1: v020-08's `Unit` row.
- a11y a11y-scope-summary#1.9: the adapter census and Constraints. a11y-strategy#1.1, #1.2, #2.4 and
  aria-patterns-roles#2.5: Implementation notes and Constraints. keyboard-navigation#4.9: v020-03's row.
  visual-design-verification#1.6: Constraints.

## Validation-1
Aligned with `scope.md` and the working entry. Each leg of the entry has its step: the record and witnesses, "inside
stand-repeat", the coverage floors, the new verbs' stderr, and C1. The boundaries hold: v020-10/11, «A11y re-read»'s
tables, and no engine or boundary change. The record found no requirement without a witness beyond C1's readings.

## Beyond the plan
| # | Proposal | Owner / place |
|---|---|---|
| 1 | A paged `part` read through the `escher-session` host's own sink stays unmeasured: the host boots only the four lean tasks, and none pages. It is stated as a limit in the masters (Expected amendments), and its measurement is owned. | a new `.andromeda/residuals.md` line, «a paged `part` read through the escher-session host's sink» (target: next), placed at this chunk's wrap |
