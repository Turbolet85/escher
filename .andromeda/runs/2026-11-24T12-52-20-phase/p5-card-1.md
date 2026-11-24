# P5 review — 2026-11-24-canvas-draw-commands «Canvas draw commands»

The plan is in the store at `doc:2026-11-24-canvas-draw-commands/plan` (rev 3). Its render, to open in an editor, is
`.andromeda/cache/docs/2026-11-24-canvas-draw-commands/plan.md`. No plan stands in the chunk's folder, and writing in
that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff
--root . 2026-11-24-canvas-draw-commands` diffs the chunk's records.

## Goal
`canvas`, the sixteenth verb and the twelfth instance verb, reads one canvas element's draw commands by its stable id.
It runs no step and changes nothing.
- **The summary, by default:** `canvas`, the content-box size, counts per command kind, the layer depth and an FNV-1a
  64 digest of the full dump.
- **The full dump, with `--full`:** one line per command in paint order, layers indented and closed by `pop`, numbers
  rounded to 1/1000. Over 10,000 bytes it answers in parts of whole lines.

The founder's F1 B puts it on every ratified carrier, reaching the provider through MCP alone. His F3 A carries no text:
a glyph run is its count, size, brush, origins' box and an ids digest. His F5 B proves the carriers on a test-only
fixture, with Circle Drawer read in process. The delegate's F2 A, F4 A, F6 A, F7 A, F8 A and F9 A shape the rest
(inputs#I3).

## Top acceptance criteria
- Circle Drawer, in process and in both modes, answers the summary and the full dump. One committed table holds the
  digests and bytes at boot, after one click, and at 12 and 100 circles, read on linux, windows and macOS. A line diff
  across one click names exactly the new circle's lines.
- The 100-circle dump answers in parts, each within 10,000 B; its sizes are measured and stated.
- The verb crosses on the fixture:
  - the session socket from a test host;
  - the `escher-session` binary's command line against that host;
  - MCP (its `tool_call` parenting one `command`);
  - a kept check replayed and stopped on a flipped `digest`.
- No answer, MCP result or kept check holds a glyph id, a position, text, font bytes or a family name.
- The call changes nothing under pending Timer ticks. It refuses an unread id `not-found`, a deleted row's id `stale`
  and a fontless document `fonts-unavailable`; an element with no widget answers `canvas: false`.
- `actions` lists `canvas` with no cause on every node: the 14 cross-OS `actions` digests are re-read, and the 40
  snapshot and lint digests do not move. `explore` leaves the verb out by its field kind.
- `Cargo.lock` gains no package, version or checksum line. Audit, `fast`, `doc` and `stand-repeat` are green, and the
  final HEAD's CI is green.

## Touchpoints
- **New:** `escher-driver/src/canvas.rs` (with `canvas_tests.rs`); `stand_canvas_commands.rs` and
  `stand_canvas_carriers.rs`.
- **Modified:** 38 files.
  - the engine read in blitz-paint and the harness;
  - escher-driver's schema, command, run, writer and readers, and its manifest (anyrender and peniko from the
    workspace);
  - the fixtures and nine blitz-tests checks;
  - four seven_guis process checks;
  - the cold-agent pipe's tool lists;
  - `docs/capabilities.md` and README.

## Test commands and baselines (untouched tree)
| # | entry | baseline |
|---|---|---|
| 1 | `cargo test -p escher-driver --locked --lib` | green, 126 passed |
| 2 | the two new stand targets | red — no such target yet (this chunk's) |
| 3 | seven blitz-tests companions | green |
| 4 | seven_guis `cli_commands` · `mcp_server` · `host_log` · `mcp_log` | green |
| 5, 6 | `test_capability_statement.py` · `test_cold_agent.py` | green |
| 7 | the lock guard (git diff vs `8520d7743`) | green; control: dependency-list edit green, added package red (`control-gate7.md`) |
| 8 | `ci-leg.sh fast` | seen green at the last wrap |
| 9 | `ci-leg.sh doc` | green |
| 10 | `ci-leg.sh audit` | green |
| 11 | `ci-leg.sh stand-repeat` | green, 3 of 3 (1048 s) |
| 12–14 | the operator's hygiene, push, `ci.py --wait 2400` | leg operator |

Four one-shot `gate.py control` runs at implement, each with its red reading in the plan:
- a pump before the read;
- glyph ids written in place of the digest;
- exploration's canvas filter removed;
- the budget doubled.

## Concretization preview
None. **matrix#v040-10 is not claimed.** Its acceptance also holds the canvas lint and the grouping by virtual node,
which «Canvas lint» and «Circle Drawer's circles as virtual nodes» carry. After the word, step 4 writes one dated notes
line on what this chunk proves of it.

## Leans taken without a question (each settled by an artifact; confirm or turn any of them)
- **An element with no widget answers `canvas: false`** (zero size, no command), never a refusal. The causes are closed
  at eight, and none means «not a canvas».
- **A `--part` reads the full dump's part, with `--full` or without it.** No fault fits «part without full».
- **`canvas` refuses `fonts-unavailable` when the document cannot paint text,** as `screenshot` and `lint` do: a glyph
  run would be skipped.
- **Diffable means a line diff, run in the check.** No diff field joins the acting verbs. A command has no identity, so
  a changed command reads as one line removed and one added. This narrows the scope's «added, removed or changed»;
  identity comes with virtual nodes.
- **The cold-agent pipe's tool list takes `canvas`, and `COUNTED` does not.** No served app holds a canvas until
  «Cold-agent bar».
- **A filter's parameters are not read:** a layer writes ` filter` / ` backdrop` flags only, a stated limit.

## Checks
- All mechanical checks pass. Lint reads 0 hits. Four inputs verify (I3 unchanged). No host harm is recorded for the
  reused socket-host pattern.
- **Warning — size:** 716 prose lines against the ~150–400 guide. The sweep (38 files) matches the `layout` verb's (32),
  so it is not scope creep.
- The expected amendments in all seven masters carry pattern `rul-izni57qv`:
  - F1 B, F3 A and F5 B are the founder's own, so the wrap halts once as the answer card;
  - rows resting only on the delegate's answers land PROVISIONAL.

## Beyond the plan
| proposal | owner |
|---|---|
| A canvas command's identity, so a changed command reads as changed, not removed and added | existing entry «Circle Drawer's circles as virtual nodes — each circle a node, its draw commands grouped by it, with stand checks (v040-10, v040-11)» |
| `canvas` among the cold-agent pipe's counted verbs, once a served app holds a canvas | existing entry «Cold-agent bar — the pipe, its reply check and verb counts widened to canvas apps and new verbs; fresh-agent goals over canvas, virtual nodes, `drag` (v040-15)» |

Apply? (yes / review / cancel)
