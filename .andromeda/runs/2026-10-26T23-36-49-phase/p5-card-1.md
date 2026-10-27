# P5 review — 2026-10-26-codebase-health · Codebase health (v030-01)

Plan: `escher-0.3.0/chunks/2026-10-26-codebase-health/plan.md`. Research and scope sit beside it.

## Goal
Bring the three escher-written functions the Epoch 5 audit found over the cognitive-complexity ceiling of 15 back
under it, by table walks or named module-level helpers:
- `lint.rs::findings` (16)
- `mcp_tests.rs::the_listing_is_the_twelve_verbs_of_the_table` (18)
- `stand_requirement_sweep.rs::every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (18)

This follows the founder's M1 «Код escher, тесты таблицами (Рекомендую)» (inputs#I1). Test functions stay counted.
`clear_interaction_state_for_removed_node` (17) and all of blitz-dom stay untouched. No behaviour, verb, field,
carrier, log site, span or dependency moves.

## Key fact from research
The audit's reading (rust-code-analysis 0.0.25 `cognitive.sum`) aggregates nested spaces. A closure or nested `fn`
stays counted in its parent, so the helpers must be module-level items. The population reads 88 at the base, re-derived
over the audit's 429 files, and is predicted to read 85 after the chunk.

## Top acceptance criteria
- The complexity entry (the audit's own recipe):
  - the three are not listed over 15, and the engine row still reads 17.0;
  - `over 15: 85`, with no new entrant.
- `stand_lint` 7 unchanged under both builds: `[0, 1, 0, 0]` at boot, the select-option `overlap` neither fixed nor
  moved, the planted page's seven findings and the twelfth-Create `clipped` row.
- The served pages, the MCP lint call, `host_log` / `mcp_log` and `stand_replay`'s committed checks are unchanged.
- The two tests keep their names, targets and rows. Each is red on a recorded row-guard control and green restored
  (implement's step 4, five controls).
- Silence census 0; 2 `info_span!` sites; no events; the guard is clean outside the three files.
- `fast`, `doc`, `a11y`, `stand-repeat` (204 · 864 · 0 · 21) and `coverage` (escher-driver ≥ 97 %, last 97.99 %) are
  green, and the CI run on the pushed sha is green with per-job tallies equal to CI#40389488295.

## Touchpoints
- `packages/escher-driver/src/lint.rs`
- `packages/escher-driver/src/mcp_tests.rs`
- `tests/blitz-tests/tests/stand_requirement_sweep.rs`

There are no new files.

## Test commands
22 entries:
- the complexity reading, the escher-driver unit tests, the per-package and workspace witness sets, and the seven_guis
  binary checks;
- the census, `env!`, span and event probes and the preservation guard;
- agent-run boot and `run stand`, and the `a11y`, `stand-repeat`, `coverage`, `fast` and `doc` legs plus the tally;
- the operator pass: hygiene, push, the CI read and the per-job tallies.

All 16 new entries were baselined on the untouched tree:
- The complexity entry reads red with the subject this chunk builds (the three rows, 88).
- Every other entry reads green.
- The instrument and the inline guards ran their must-pass and must-fail controls in a scratch clone
  (`.andromeda/runs/2026-10-26T23-36-49-phase/p5-controls.md`).

## Concretization preview — v030-01 (method: integration)
**Current acceptance:** On the code audit's own complexity reading, the three escher-written functions the Epoch 5
audit found over the ceiling of 15 read at or under it, the two tests still counted and asserting the same rows, the
engine function left as it was, and every check green.

**Proposed concrete form:**
- **The reading:** the Epoch 5 audit's own recipe — rust-code-analysis 0.0.25, every space of kind function (closures
  included), `cognitive.sum > 15` — over its 429 listed files
  (`.andromeda/runs/2026-10-25T21-43-01-code-audit/srcfiles.txt`).
- **The three functions:** `lint.rs::findings`, `the_listing_is_the_twelve_verbs_of_the_table` and
  `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` are not listed over 15. No function the chunk adds is
  listed, and the population reads `over 15: 85` (88 at take-up).
- **The two tests:** they keep their names and targets and are still counted. Each is read red on recorded row-guard
  controls (a dropped `ci-leg.sh` name, a witness name no file holds, a listing result field, an argument's bound) and
  green restored.
- **The engine function:** `clear_interaction_state_for_removed_node` still reads 17.0, and no file under
  `packages/blitz-dom/` changes.
- **Every check green:** `ci-leg.sh fast`, `doc`, `a11y`, `stand-repeat` and `coverage`, and the CI run on the pushed
  sha.

The outcome is not weakened. "At or under it" is kept, and the population count of 85 is added as the
no-shifted-creep evidence.

No gated criterion: the chunk was taken up on no block.

## Sampler (forced on the operator's word, inputs#I2)
- architecture: `sampler: E carried 65 · the cascade lost 11 (conventions#6.1, cross-cutting-patterns#1.1, design-philosophy#3.1, design-philosophy#4.1, established-decisions#16.2, existing-scopes#1.7, occupied-resources#11.1, occupied-resources#4.1 …) · listed 5 (conventions#5.1, conventions#7.1, design-philosophy#5.1, existing-scopes#3.1, existing-scopes#3.2)`
- security-plan: `sampler: E carried 29 · the cascade lost 0 · listed 1 (dependency-security#1.0)`
- design-system: `sampler: E carried 1 · the cascade lost 0`
- test-plan: `sampler: E carried 55 · the cascade lost 2 (ci-integration#5.1, test-strategy#4.1) · listed 1 (unit-test-strategy#1.1)`
- obs-plan: `sampler: E carried 17 · the cascade lost 6 (ci-integration#1.10, ci-integration#1.4, pii-scrubbing-compliance#1.1, pii-scrubbing-compliance#2.1, span-trace-coverage#3.1, telemetry-strategy#1.1) · listed 2 (log-coverage#9.3, telemetry-strategy#3.3)`
- a11y-plan: `sampler: E carried 11 · the cascade lost 0`

Every lost item is carried in the plan, marked `(sampler)`, in the steps, the gate notes and the criteria.
architecture's list also holds occupied-resources#6.2 and stack-and-technologies#1.1 / #1.20 past the line's `…`.

## Notes
- Five other escher-written functions read over 15 on the same population: `cells.rs::tokenize` 22, `cli.rs::read` 20,
  `execute.rs::execute` 22, `mcp.rs::run` 18 and `stand_accessibility_ids.rs::every_interactive_role_element_reads_a_name`
  16. They predate the 84 baseline and are outside the founder's M1 (inputs#I1). The plan leaves them as they are and
  proposes no owner.
- The masters' line citations into the three files are re-pointed by the wrap's citation sweep. They are listed as
  Expected amendments (wrap) under six of the masters.
- One planlint HIT stands, 4 (13), on the operator's per-job tally read. It is not `ci.py`, and is resolved by its note,
  which names the read `ci.py` never makes and carries its known-positive control on CI#40389488295.

## Beyond the plan
none
