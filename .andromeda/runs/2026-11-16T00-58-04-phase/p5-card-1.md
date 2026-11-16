# P5 review — 2026-11-16-layout-explained «Layout explained»

The plan to read in an editor: `.andromeda/cache/docs/2026-11-16-layout-explained/plan.md`, a render of
`doc:2026-11-16-layout-explained/plan` rev 2. No plan stands in the chunk's folder. Writing in that file changes no
record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-16-layout-explained` diffs the chunk's records.

## Goal
Rust code holding a driver session asks, by stable id, why an element sits where it does and at its size:
- the algorithm that placed it, its container's id and its offset there;
- every scroll and transform that moves it;
- per axis, a closed size cause (a stated rule of 12 steps, falling back to `layout`) beside the computed inputs.

It is **in process only**, on F1's option 2 as answered by you as delegate, PROVISIONAL (inputs#I4). The placing
kind keeps `Placer::App { canvas }` reserved for 0.4.0, and the verb word `layout` is kept for the later widening.

## Touchpoints (10 files)
- New:
  - `packages/blitz-dom/src/layout_reason.rs`: the read-only `BaseDocument::layout_reason`.
  - `packages/escher-driver/src/layout.rs` and `layout_tests.rs`: `Session::layout`.
  - `tests/blitz-tests/tests/layout_reason.rs`: fixtures, both modes.
  - `tests/blitz-tests/tests/stand_layout.rs`: the six served screens, both modes.
- Modified:
  - blitz-dom's and dioxus-native-dom's `lib.rs`: re-exports.
  - escher-driver `execute.rs`: `resolve` made `pub(crate)`.
  - escher-driver `lib.rs`.
  - `docs/capabilities.md`.
- No verb, schema, MCP, socket, recording, CLI or seven_guis file. No count moves.

## Top acceptance criteria
1. **matrix#v030-11.** On the six served screens, in both modes, every snapshot node's answer has bounds equal to the
   snapshot's. The offset chain reproduces them to 1/64 px. `cell-A0` reads `Grid` in `cells-grid` (height `Style`,
   width `Grid`). A CRUD row reads `Flex`/`Stretch` in `crud-list`. `crud-person-14`, scrolled, reads a `Scroll`
   shift held by `crud-list`.
2. Every placing word and every size-cause word is read on a fixture, in two counted tables, in both modes.
3. The read changes nothing and moves no clock. It refuses `not-found`/`stale`. It writes no `escher_` line at TRACE.
4. No `NodeId` in the answer. `Placer::App` is reserved and produced by no path.
5. Nothing widens: `VERBS` stays fourteen and the capability counts are unmoved. The fast leg, `stand-repeat` and the
   coverage floors are green, and so is CI on the pushed HEAD.

## Test commands (11 entries, baselined on the untouched tree)
| Entry | Gate | Baseline |
|---|---|---|
| 1 | `cargo test -p escher-driver --locked --lib layout::` | green, vacuous (0 selected) |
| 2 | `cargo test -p blitz-tests --locked --test layout_reason` | red: no target yet, this chunk's |
| 3 | `cargo test -p blitz-tests --locked --test stand_layout` | red: no target yet, this chunk's |
| 4 | `text_extent` | green, 6 passed |
| 5 | `stand_actions` | green, 8 passed |
| 6 | `bash .github/scripts/ci-leg.sh fast` | seen green at the last wrap |
| 7 | `bash .github/scripts/ci-leg.sh stand-repeat` | green, 3 of 3 rounds, 736 s |
| 8 | `bash .github/scripts/ci-leg.sh coverage` | green: escher-driver 98.05 %, dioxus-native-dom 89.88 %, blitz-test-harness 96.08 % |
| 9–11 | operator: hygiene · push · `ci.py conclusion --wait 2400` | — |

There is no smoke entry: no boot path is touched. Three one-shot `gate.py control` runs are implement's (tests S4).

## Concretization preview — the one claim
- **v030-11** (integration). Current acceptance: «Asked of an element on the stand, an answer gives why it sits where
  it does and at its size, traced through the style and layout that placed it.»
- Proposed concrete form: «`Session::layout`, asked in process of every snapshot node of the six served stand screens
  in both layout modes, answers with:
  - bounds equal to the node's snapshot bounds;
  - the placing algorithm, the container's stable id and the offset there, which with the container's scroll
    reproduce the bounds to 1/64 px wherever no transform is on the chain;
  - every scroll and transform shift;
  - per axis a size cause with its computed inputs.

  `cell-A0` reads `Grid` in `cells-grid` with height `Style` and width `Grid`. A CRUD row reads `Flex` in `crud-list`
  with width `Stretch`. `crud-person-14`, scrolled into view, reads a `Scroll` shift held by `crud-list`. Proven by
  `stand_layout`. The read reaches no carrier: in process only, PROVISIONAL (inputs#I4).»
- The never-weaken read is yours to make. The outcome is kept whole: why it is there, why at that size, through style
  and layout. What narrows is who can ask: Rust code holding a session, not an MCP or command-line agent, until the
  founder rules on F1. The requirement line says «an agent can ask». An in-process agent can; an agent on the carriers
  cannot yet. The claim's `notes` line will say so.

## Validation-1
Aligned. The scope gained one bullet recording F1's PROVISIONAL answer (rev 4). The plan builds the scope's read in
process, and the scope's verb-branch surfaces are marked as the later widening's.

## Watch
- CI for `b7a306490` (the last wrap's flip) was in progress at Setup. The wrap reads it.
- The PROVISIONAL absence of a carrier goes to the wrap as a full escalation (patterns `rul-izni57qv`,
  `rul-3ndrhilc`), discharged by the founder's own word.

## Beyond the plan
| # | Proposal | Owner |
|---|---|---|
| 1 | Deferral: the layout read on the carriers (the `layout` verb on the socket, command line, MCP and kept checks). It waits on the founder's own word on F1 (`.andromeda/runs/2026-11-16T00-58-04-phase/founder-forks.md`). | Proposed new entry «Layout on the carriers», placed directly after «Layout explained» only if the founder takes option 1; none if he keeps option 2. |
| 2 | v030-11's witness row in `stand_requirement_sweep`'s `WITNESSES` | «Stand requirement sweep» (existing entry) |
| 3 | The cold-agent bar cannot count the layout read while it stays in process | «Cold-agent bar» (existing entry) |

Apply? (yes / review / cancel)
