# P5 review — 2026-10-12-self-description

**Chunk:** Self-description — verb list, help and schemas served by CLI and MCP from the one schema (v010-14) · working-route.md:81 · promoted `pending`.
**Plan:** `escher-0.1.0/chunks/2026-10-12-self-description/plan.md`: 537 lines (400 prose, 137 in the fence), 19 steps, 19 gate entries, 10 acceptance criteria.
**Waits on the founder:** `.andromeda/runs/2026-10-12T06-49-46-phase/founder-forks.md` has four forks. The plan is written on the recommendations `1A 2A 3A 4A`; any other answer returns it to P4. The operator's word below approves the plan on that condition.

## Goal
The tool serves its own description from the one table, on both surfaces:
- the verb list, each verb's help, the argument and result shapes;
- a help per node key;
- the refusal causes with their remedies;
- the exit statuses with their meanings;
- the session errors with their remedies.

`escher-session help` answers it as one line of JSON. A `help` tool call answers the same bytes over MCP. Each tool's description also carries its result fields.

The chunk also does the following:
- `state-dir-too-long` states its bound, and every session error names a remedy.
- A changed diff node names the keys that differ.
- The server speaks `2025-11-25`.
- It runs two owed checks: a password typed over MCP reads the mask, and every provoked cause is read with its remedy on its served form.

## Top acceptance criteria
- **v010-14** (the concretization preview is below).
- A password typed through an MCP `type` occurs 0 times in that call's result and in a following `snapshot`, and reads `MASKED_VALUE`, in both layout modes (`stand_mcp_masked`).
- On the flight booker's Book, each of the 8 changed nodes reads `differs` = `[bounds]`, in both layout modes (`stand_diff`). This is the P3 measurement made a check.
- `initialize` asking `2025-11-25` is answered `2025-11-25`. One asking `2026-07-28` is answered `2025-11-25`. `server/discover` stays `-32601`.
- On Linux, a 94-byte state directory starts a session and a 95-byte one ends `state-dir-too-long`, status 3, its message naming 94. The macOS figure, 90, is read, not measured.
- No new span field, dependency, listener or crate, and nothing outside the write set moves.

## Touchpoints
- **New:** `packages/escher-driver/src/describe.rs` and `tests/blitz-tests/tests/stand_mcp_masked.rs`.
- **Modified:**
  - escher-driver's `schema`, `error`, `refusal` (a doc comment), `json`, `cli`, `mcp`, `command` and `lib`;
  - dioxus-native-dom's `snapshot_diff` and `lib`;
  - `stand_diff`, `stand_act_scroll`, `stand_mcp_tools`, `cli_commands`, `mcp_server`, and seven_guis' `common` if a shared reader is needed;
  - `README.md`.

## Test commands
- Unit: `cargo test -p escher-driver --locked`, `cargo test -p dioxus-native-dom --locked`.
- Smoke: the built binary answering `help`.
- Integration: the new and changed check files, per package and under the workspace build, plus the standing checks nearest the change.
- Probes: lock and manifests unchanged; no print or panic outside tests; no terminal test; preservation of everything outside the write set.
- Harness and lints: `agent-run.sh run stand`, `ci-leg.sh fast` and `doc`.
- Operator: hygiene, then the push, then the CI read.

Baselines on the untouched tree: 7 green and 3 red (the smoke and the two runs naming the new `stand_mcp_masked` target), all red on what this chunk builds. Each of the three guards was read on both sides over a scratch clone (`controls-p5.md`).

## Concretization preview — v010-14
- **Now (outcome):** "The tool itself serves its verb list, help and schemas on both surfaces from one verb set, and every refusal cause the stand can provoke names a remedy."
- **Proposed (concrete):**
  - `escher-session help` answers, with status `0`, one line of JSON equal to `escher_driver::describe()`. It holds:
    - the ten verbs of `VERBS` in the table's order, each with its level, help, arguments (kind, bound, required, help) and result fields (kind, always, help);
    - the thirteen node keys with their help, the key names and the busy classes;
    - the eight causes with meaning and remedy;
    - the four exit statuses with their meanings;
    - the thirteen session errors with their remedies.

    Read by `cargo test -p seven_guis --locked --test cli_commands`.
  - Over `escher-session mcp`, a `help` tool call answers the same bytes. `tools/list` lists the same ten verbs, each description holding its result fields' help (`mcp_server`, `stand_mcp_tools`).
  - Every cause provoked on the stand and the fixture beside it is read on its served form, with a `remedy` equal to `Cause::remedy()`. Seven are read through the binary and seven through MCP (`covered` on the overlay fixture); the eight are covered together (`cli_commands`, `stand_mcp_tools`).
- Never weaker: each clause of the outcome has a check that reads the served output itself, not the table.
- Conditional: under founder fork 1C v010-14 is not claimed. Under 2B the MCP half is served through `initialize` and the plan returns to P4.

## Validation
- Mechanical checks 1–9 pass, re-run in full after the baseline writes.
- Validation-1 is aligned, and every CARRY is taken:
  - 1 → `help`, the exit statuses table;
  - 2 → founder fork 3;
  - 3 → remedies and the bound;
  - 4 → `differs`;
  - 5 → help per node key;
  - 6 → `2025-11-25`, founder fork 4;
  - 7 → `stand_mcp_masked`;
  - 8 → the report's re-count, step 19.
- CI since the last wrap: `5698e08e` green 16/16 (918 s); `bf973d33` green 16/16 (948 s, re-read at P5).
- The plan's prose sits at 400 lines, the edge of the band. The previous chunk's plan was 649 lines.

## Beyond the plan
Each row is a proposal the word approves that neither `plan.md` nor the matrix claim holds. Each rides a founder answer.

| proposal | owner | rides |
|---|---|---|
| Append to `.andromeda/playbook.md` a rule beside the closed-field rule: the program's own fixed text, held as `'static` data of the one table, rides a ratified carrier; text from a page, a call or a screen, a new carrier or a new exit still widens | this chunk's wrap | founder fork 1A |
| Record in `.andromeda/residuals.md`: the actionable-key check as a driver command, not in 0.1.0 | this chunk's wrap | founder fork 3A |
| Record in `.andromeda/residuals.md`: taking up MCP revision `2026-07-28` (stateless, `server/discover`) | this chunk's wrap | founder fork 4A |

Apply? (yes / review / cancel)
