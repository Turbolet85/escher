# P5 review — 2026-11-17-a-component-in-isolation «A component in isolation»

Plan render to open in an editor: `.andromeda/cache/docs/2026-11-17-a-component-in-isolation/plan.md` (rev 7). No plan
stands in the chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-a-component-in-isolation`
diffs the chunk's records.

## Goal
One stand component renders alone with the props its caller gives, at the stand's 800 × 600 viewport, bundled font
and default seed, in process only (the founder's F1 «A: только внутри процесса (Recommended)», inputs#I3). It is read
against its in-app copy. The chunk also tests `boundary_marks()`'s fallback branch (CARRY cry-c4emakc4) and joins
`layout` to the cold-agent pipe, uncounted (his «Добавить сейчас», inputs#I2 item 2). Its wrap lands the
`no-attribute-text` exception (cry-npoxtx67 / the held row of cry-2dk6n44y) and lifts the 24 delegate marks
(inputs#I2 items 1 and 3).

## Touchpoints
- New: `tests/blitz-tests/tests/stand_component_alone.rs`
- Modify: `packages/blitz-test-harness/src/harness.rs` (the constructor) · `examples/seven_guis/src/stand.rs` (the
  isolated boot) · `packages/dioxus-native-dom/src/boundary.rs` (the fallback test) · `scripts/cold-agent.sh` ·
  `.github/scripts/test_cold_agent.py` · `docs/capabilities.md`
- Untouched by construction: `packages/escher-driver/`, `examples/seven_guis/src/session_host.rs`. Gate 5 guards it
  against c7c09a7bf.

## Top acceptance criteria
1. v030-12 (below): `stand_component_alone` in both layout modes, with controls (a) and (b).
2. `boundary_marks()` on a document built around a vdom rebuilt in place reads one `Root` mark; control (c) reads zero.
3. Both pipe `VERBS` lists hold fifteen, `layout` after `actions`. `COUNTED` is unchanged. The fake answers `layout`
   as a read. The contract tests are green.
4. No carrier moved (gate 5), the session smoke passes, `fast` and `doc` are green, and the operator pass's CI read
   is `verdict: green`.

## Test commands (12 entries)
Unit `boundary` · unit harness · integration `stand_component_alone` · unittest of the cold-agent tests · the
no-carrier guard · `agent-run.sh boot` → `run session` (smoke) · `ci-leg.sh fast` · `ci-leg.sh doc` · operator:
hygiene → push (clean tree) → `ci.py conclusion --sha HEAD --wait 2100`.
Baselines on the untouched tree: entries 1, 2, 4–7 green. Entry 3 is red only because its target file does not exist
yet (`no test target named stand_component_alone`). Entries 8 and 9 were seen green at the last wrap. Three one-shot
controls, (a), (b) and (c), are /implement's, run through `gate.py control`.

## Concretization preview — matrix#v030-12
- Now: «One component renders alone with given props, and its reading matches the same component's reading inside
  its app.»
- Proposed: «In both layout modes, `stand_component_alone` reads: `TaskShell` alone with given props (`title`
  "Counter", a no-op `on_back`, `children` `Counter {}`) snapshots equal to the stand's Counter boot node for node
  (id, role, name, state, bounds within 1/64 px inclusive), with an equal UI graph. `Counter` alone reads every in-app
  `TaskShell/Counter/` node with the leading `TaskShell/` removed from ids holding a `/`, with roles, names and states
  equal, `counter-value` at 60 × 28 and `counter-increment` at 95.515625 × 39, and graph state `count` having
  `Counter` as its component and sole reader. A click moves `counter-value` from `0` to `1`. The isolated document
  reads 800 × 600 with a font that paints text.» Method `integration`, unchanged.
- Why it narrows nothing: bounds equality is kept wherever the in-app parent is the viewport (the whole TaskShell
  case). Where the parent is `task-body` (800 × 554 at y 46), the outcome is read by stated construction, measured
  at P3 from `tests/blitz-tests/checks/counter-cli.json`.

## Reads to name
- The rows files sum to 226,311 B, under the 250,000 B whole-read bound. The obs, a11y and design rows files were
  read by search for the plan's terms, not whole. That search found obs-plan's
  `decision/graph-and-actions-not-counted-verbs`, now an Expected amendment.
- Aggregate B: no master read `picked 0`.

## Beyond the plan
- none
