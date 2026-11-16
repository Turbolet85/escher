# P5 review — 2026-11-16-layout-explained «Layout explained» (card 2, your review folded)

The plan to read in an editor: `.andromeda/cache/docs/2026-11-16-layout-explained/plan.md`, a render of
`doc:2026-11-16-layout-explained/plan` rev 3. No plan stands in the chunk's folder. Writing in that file changes no
record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-16-layout-explained` diffs the chunk's records.

## What your review changed (`.andromeda/runs/2026-11-16T00-58-04-phase/relay-2.md`, snapped as inputs#I5)
- **v030-11 is not claimed.** At P5 step 4 there is no `matrix.py claim` and no matrix note. The capability stays
  `planned`, unclaimed, until the read is on the carriers or the founder rules that the in-process form meets «an
  agent can ask».
- The acceptance checks stay, as the chunk's own gates. The former matrix#v030-11 criterion is now a `(driver)`
  criterion that names v030-11 as built and not claimed.
- An Implementation note binds the report: its Outcome says «v030-11 built, not claimed».
- Proposals 1–3 stand as written (below).
- The whole P5 check set re-ran on rev 3: the gate fence parses (11 entries, every new one baselined), planlint
  reports 0 hits, research lists parse at 10 items, and inputs read 5 entries, all cited, 0 unparsed.

## Goal
Rust code holding a driver session asks, by stable id, why an element sits where it does and at its size:
- the algorithm that placed it, its container's id and its offset there;
- every scroll and transform that moves it;
- per axis, a closed size cause (a stated rule of 12 steps, falling back to `layout`) beside the computed inputs.

It is in process only, on F1's option 2, answered by you as delegate, PROVISIONAL (inputs#I4). `Placer::App
{ canvas }` is reserved for 0.4.0, and the verb word `layout` is kept for the later widening.

## Touchpoints (10 files, unchanged from card 1)
- New:
  - `packages/blitz-dom/src/layout_reason.rs`
  - `packages/escher-driver/src/layout.rs` and `layout_tests.rs`
  - `tests/blitz-tests/tests/layout_reason.rs`
  - `tests/blitz-tests/tests/stand_layout.rs`
- Modified:
  - blitz-dom's and dioxus-native-dom's `lib.rs`
  - escher-driver `execute.rs` (`resolve` made `pub(crate)`) and `lib.rs`
  - `docs/capabilities.md`

## Acceptance criteria (the chunk's own gates; no capability claimed)
1. **(driver)** On the six served screens, in both modes, every snapshot node's answer has bounds equal to the
   snapshot's. The offset chain reproduces them to 1/64 px. `cell-A0` reads `Grid` in `cells-grid` (height `Style`,
   width `Grid`). A CRUD row reads `Flex`/`Stretch` in `crud-list`. `crud-person-14`, scrolled, reads a `Scroll`
   shift held by `crud-list`. This is v030-11's outcome, built and not claimed.
2. **(engine)** Every placing word and every size-cause word is read on a fixture, in two counted tables, in both
   modes.
3. **(driver)** The read changes nothing and moves no clock. It refuses `not-found`/`stale`. It writes no `escher_`
   line at TRACE.
4. **(driver)** No `NodeId` in the answer. `Placer::App` is reserved and produced by no path.
5. **(boundary · engine · quality)** Nothing widens and no count moves. The fast leg, `stand-repeat` and the coverage
   floors are green, and so is CI on the pushed HEAD.

## Test commands (11 entries, unchanged; baselines on the untouched tree)
| Entry | Gate | Baseline |
|---|---|---|
| 1 | `cargo test -p escher-driver --locked --lib layout::` | green, vacuous (0 selected) |
| 2 | `layout_reason` | red: no target yet, this chunk's |
| 3 | `stand_layout` | red: no target yet, this chunk's |
| 4 | `text_extent` | green, 6 passed |
| 5 | `stand_actions` | green, 8 passed |
| 6 | `ci-leg.sh fast` | seen green at the last wrap |
| 7 | `ci-leg.sh stand-repeat` | green, 3 of 3 rounds, 736 s |
| 8 | `ci-leg.sh coverage` | green: escher-driver 98.05 %, dioxus-native-dom 89.88 %, blitz-test-harness 96.08 % |
| 9–11 | operator: hygiene · push · `ci.py conclusion --wait 2400` | — |

## Concretization preview
None: no capability is claimed. v030-11's acceptance stays as written in the ledger, `planned`, unclaimed.

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
