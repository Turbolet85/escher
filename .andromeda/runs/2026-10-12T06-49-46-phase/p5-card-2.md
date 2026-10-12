# P5 review, second card — 2026-10-12-self-description

**Chunk:** Self-description — verb list, help and schemas served by CLI and MCP from the one schema (v010-14) · working-route.md:81 · promoted `pending`.
**Plan:** `escher-0.1.0/chunks/2026-10-12-self-description/plan.md`: 542 lines (404 prose, 138 in the fence), 19 steps, 19 gate entries, 10 acceptance criteria.
**Answers the first card's review** (`p5-card-1.md`, the operator's review snapped as `inputs#I4`).

## What the review changed
- **The founder's forks are answered.** He chose `1A 2A 3A 4A` («Внутри, с правилом» · «help — десятый глагол» · «Не в 0.1.0, в остатки» · «Не сейчас, в остатки»), relayed verbatim by the overseer. The relay is snapped as `inputs#I3` and cited where the plan named a fork:
  - the Metadata;
  - step 3;
  - the v010-14 criterion and the bound criterion;
  - the two rejected approaches;
  - the security-plan and matrix amendment lines;
  - the wrap's three extras.

  Every answer is the option the plan was written on, so nothing re-plans, and the conditional lines ("under 1C…", "under 2B…") are gone.
- **Step 13** (`mcp_server`, through the binary): a `help` tool call made first after `initialize`, before any `start`, answers the document, not an error. Step 8 now says the server answers `help` whether or not it holds a session. The v010-14 criterion includes the first call.
- **Step 3**: the pairwise-distinct requirement on remedies is dropped. Where two session-error kinds truly share a next step, the later row names the earlier kind whose remedy it repeats word for word. That mark lives in the table and is not served. The unit test asserts two remedies are equal exactly where the later row names the earlier. Each remedy stays non-empty and holds no `/`.

## Goal (unchanged)
The tool serves its own description from the one table, on both surfaces:
- the verb list, each verb's help, the argument and result shapes;
- a help per node key;
- the causes with their remedies;
- the exit statuses with their meanings;
- the session errors with their remedies.

`escher-session help` and an MCP `help` call answer the same bytes. Each tool's description carries its result fields.

The chunk also does the following:
- every session error gains a remedy, and `state-dir-too-long` states its bound;
- a changed diff node names the keys that differ (`differs`);
- the server speaks `2025-11-25`.

It runs the two owed checks: a password typed over MCP, and every provoked cause with its remedy.

## Top acceptance criteria
- **v010-14** (preview below).
- A password typed through an MCP `type` occurs 0 times in that result and a following `snapshot`, and reads `MASKED_VALUE`, in both layout modes (`stand_mcp_masked`).
- On the flight booker's Book, each of the 8 changed nodes reads `differs` = `[bounds]`, in both layout modes (`stand_diff`).
- Revision answers: `initialize` asking `2025-11-25` is answered `2025-11-25`, and asking `2026-07-28` is answered `2025-11-25`. `server/discover` stays `-32601`.
- On Linux, a 94-byte state directory starts and stops a session. A 95-byte one ends `state-dir-too-long`, status 3, its message naming 94. The macOS figure, 90, is read, not measured.
- Nothing else moves: no new span field, dependency, listener or crate, and nothing outside the write set.

## Touchpoints (unchanged)
- **New:** `packages/escher-driver/src/describe.rs` and `tests/blitz-tests/tests/stand_mcp_masked.rs`.
- **Modified:**
  - escher-driver's `schema`, `error`, `refusal` (doc comment), `json`, `cli`, `mcp`, `command` and `lib`;
  - dioxus-native-dom's `snapshot_diff` and `lib`;
  - `stand_diff`, `stand_act_scroll`, `stand_mcp_tools`, `cli_commands`, `mcp_server`, and seven_guis' `common` only if needed;
  - `README.md`.

## Test commands (unchanged fence)
- Unit, both crates.
- The `help` smoke.
- Integration: the new and changed files per package and under the workspace build, plus the nearest standing checks.
- Probes: lock, no-print, terminal and preservation.
- `agent-run.sh run stand`, `ci-leg.sh fast` and `doc`.
- Operator: hygiene, push, CI read.

The fence did not change, so the baselines stand: 7 green and 3 red on the untouched tree. All three reds are what this chunk builds. The three guards' controls are in `controls-p5.md`.

## Concretization preview — v010-14
- **Now (outcome):** "The tool itself serves its verb list, help and schemas on both surfaces from one verb set, and every refusal cause the stand can provoke names a remedy."
- **Proposed (concrete):**
  - `escher-session help` answers, with status `0`, one line of JSON equal to `escher_driver::describe()`. It holds:
    - the ten verbs of `VERBS` in order, each with its level, help, arguments (kind, bound, required, help) and result fields (kind, always, help);
    - the thirteen node keys with their help, the key names and the busy classes;
    - the eight causes with meaning and remedy;
    - the four exit statuses with their meanings;
    - the thirteen session errors with their remedies.

    Read by `cli_commands`.
  - Over `escher-session mcp`, a `help` tool call answers the same bytes, including when it is the first call after `initialize`, before any `start`. `tools/list` lists the same ten verbs, each description holding its result fields' help (`mcp_server`, `stand_mcp_tools`).
  - Every cause provoked on the stand or the fixture beside it is read on its served form with `remedy` equal to `Cause::remedy()`. Seven are read through the binary and seven through MCP (`covered` on the overlay fixture); together they cover the eight.
- Never weaker: each clause is read off the served output itself, not off the table. The claim rests on founder forks 1A and 2A, both answered so (`inputs#I3`).

## Validation (re-run in full after the fold)
- **Dry-run:** parses 19 entries.
- **Baseline listing:** 6 seen green, 0 not seen, 10 new (each with its `baseline` key), 3 operator legs.
- **planlint:** 0 hits.
- **Research lists:** parsed, 19 items (2 new, 17 to modify).
- **Modify paths:** all 17 exist.
- **Plan prose:** 404 lines, 4 over the ~400 band's top. The added lines are the fold's own: the step 13 row, step 3's shared-remedy rule and the input citations.
- **Validation-1:** aligned. The two additions sit inside the scope's CARRY 1 and CARRY 3.

## Beyond the plan
Each row is a proposal the word approves that neither `plan.md` nor the matrix claim holds. Each rides a founder answer, now given (`inputs#I3`).

| proposal | owner | rides |
|---|---|---|
| Append to `.andromeda/playbook.md` a rule beside the closed-field rule: the program's own fixed text, held as `'static` data of the one table, rides a ratified carrier; text from a page, a call or a screen, a new carrier or a new exit still widens | this chunk's wrap | founder fork 1A («Внутри, с правилом») |
| Record in `.andromeda/residuals.md`: the actionable-key check as a driver command, not in 0.1.0 | this chunk's wrap | founder fork 3A («Не в 0.1.0, в остатки») |
| Record in `.andromeda/residuals.md`: taking up MCP revision `2026-07-28` (stateless, `server/discover`) | this chunk's wrap | founder fork 4A («Не сейчас, в остатки») |

Apply? (yes / review / cancel)
