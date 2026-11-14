# P5 review — 2026-11-14-start-in-a-state «Start in a state»

The plan's render to open in an editor: `.andromeda/cache/docs/2026-11-14-start-in-a-state/plan.md` (rev 3).
No plan stands in the chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-start-in-a-state` diffs the
chunk's records.

## Goal
A driver session starts in a named fixture state of its app, and its first screen reads as that state does when reached
by hand from boot. A state is static data: a name plus the driver calls that reach it from boot. `Session::reach` runs
those calls through `Session::run`, the one validated path, on a session that is already started. `Session::start` and
its 35 call sites are unchanged.

The stand names three states:
- CRUD `fourteen-people`: twelve Creates.
- CRUD `person-selected`: a row selected.
- flight booker `return-flight`: a return flight chosen.

The shared fixtures module gains `hold_in`.

## The founder's fork, provisional
F1 asks how a state crosses the driver's carriers. It is filed in
`.andromeda/runs/2026-11-14T11-32-16-phase/founder-forks.md`. It is a boundary widening under `rul-izni57qv`. The options:
- **A (recommended):** an optional `state` argument on `start`.
- **B:** the states become served app names.
- **C:** no carrier yet.

Following your directive (inputs#I1), the plan builds **C** and marks it PROVISIONAL. Nothing crosses the command line,
MCP, the socket, or a kept check's form.
- Because the reach runs through `Session::run`, a recording attached before it keeps the steps as ordinary steps.
- The kept check therefore opens on its app and replays from boot unchanged.

## Top acceptance criteria
- `matrix#v030-08`: covered by `stand_start_in_state` in both layout modes.
  - Each of the three states answers `snapshot`, `lint`, `actions` and `graph` with the same lines as a session driven
    by hand from boot. The check states its by-hand calls itself.
  - A started session takes the next call (`click` on `crud-person-14`, refused `off-screen`) the way the by-hand
    session does.
  - An unknown name is refused `UnknownState` and nothing runs.
- A refused step stops the reach with its index and cause, as `Refused { step: 1, cause: Disabled }`.
- `hold_in` holds a fixture page in a state that reads as the same page clicked by hand.
- A recorded session that reaches a state replays from boot (unix only).
- No span field, log site, print or env read is added. `fast` and `doc` pass, and CI is green on the final HEAD.

## Touchpoints
- New:
  - `packages/escher-driver/src/fixture.rs`
  - `packages/escher-driver/src/fixture_tests.rs`
  - `tests/blitz-tests/tests/stand_start_in_state.rs`
- Modified:
  - `packages/escher-driver/src/lib.rs`
  - `examples/seven_guis/src/stand.rs`
  - `tests/blitz-tests/tests/fixtures/mod.rs`
  - `docs/capabilities.md`

## Test commands and baselines (untouched tree)
1. `cargo test -p escher-driver --locked`: green.
2. `cargo test -p blitz-tests --test stand_start_in_state --locked`: red. The target does not exist yet; this is the
   predicted red.
3. `cargo test -p seven_guis --locked`: green.
4. `ci-leg.sh doc`: green.
5. `ci-leg.sh fast`: last seen green at the bundled-fonts wrap.
6. Operator: hygiene.
7. Operator: push, behind a clean-tree guard.
8. Operator: `ci.py conclusion --wait 1800`. The diff carries a `cfg(unix)` test item.

Controls at implement: `gate.py control` on two guards, firing entry 2, each case named by the test alone.

## Concretization preview — `matrix#v030-08`
- **Now:** «A session starts in a named fixture state, and its first screen reads as that state does when reached by
  hand from boot.»
- **Proposed:**
  - In process, in both layout modes, a driver session started in each of the stand's three named fixture states (CRUD
    `fourteen-people`, CRUD `person-selected`, flight booker `return-flight`) answers `snapshot`, `lint`, `actions` and
    `graph` with the same lines as a session of the same task driven by hand from boot through the driver's own calls.
  - It takes the next call as that session does.
  - A name the task's table does not hold is refused `UnknownState` with nothing run (`stand_start_in_state`).
- The outcome is not weakened. «Named fixture state» and «first screen» become three named states and four readings.
  «In process» is the provisional F1 option C.

## For your read
- **The row read was not re-run after P3's closure.** The closure added one new file, `fixture.rs`. The pinned scope
  already named the driver session library, and the correction narrows the change (no carrier). Say so if you want the
  row read re-run whole.
- **Planlint 12 (d) listed two claims; both hold.**
  - `Check::from_bytes` is read, not changed.
  - «byte for byte» is the name comparison.

## Beyond the plan
| # | Proposal | Owner / place |
|---|---|---|
| 1 | On the founder's answer A or B to F1, add a new route entry «State on the driver's carriers» before «Auto-exploration». It builds the chosen carrier shape on this chunk's in-process reach. On C, the PROVISIONAL absence rows are ratified as they stand. | working-route, Epoch 4, before «Auto-exploration» (conditional on F1) |

Apply? (yes / review / cancel)
