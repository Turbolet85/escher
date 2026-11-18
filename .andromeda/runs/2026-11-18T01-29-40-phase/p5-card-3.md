# P5 review, after the re-snap — 2026-11-18-a11y-re-read · «A11y re-read»

**The plan:** `.andromeda/cache/docs/2026-11-18-a11y-re-read/plan.md` (rev 6), open it in an editor. No plan stands in a chunk's folder. Writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-a11y-re-read` diffs the chunk's records.

## What the review's word changed (inputs#I3)
The stand question is settled by a search, written in the plan's Implementation notes («The stand search») and in research's `## Sweeps`. It ran over the seven sources of the six served pages: `tasks/{counter,flight_booker,timer,crud,cells}.rs`, `stand.rs` (the load page) and `app.rs` (`TaskShell`). Their CSS is inline.
- **`hidden` · `display: none` · `visibility` · `:focus` · `focus-within`:** 6 hits, none on a focusable node.
  - The one `display: none` is `.load-done`, a paragraph.
  - Two are `overflow: hidden`.
  - Three are `:focus` outline or border rules.
- **Focusable elements:** 16 hits.
  - Every one is rendered unconditionally, but Cells' `cell-input` (inside `if is_selected`, `cells.rs:489`).
  - `app.rs:143` is Home's task cards, which no served page renders. The stand skips Home, and its `on_back` is a no-op.
- **Conditional renders:** four render an element.
  - The load page's `link`, the flight booker's `p#flight-booked` and CRUD's unfocusable filtered rows render nothing focusable.
  - `cell-input` does.
- **Result:** no served page has a control that can be focused and then hidden, so the CARRY's limit is not reachable on the stand. The fixture `key_press_after_container_lost_its_box` stands as its witness.
- **The one candidate, Cells' editor:** focused by a click, then unmounted by Enter, it takes the removal path. `document.rs:1442-1460` blurs it and clears focus, which is not the box-lost path.
  - It gains a stand-level reading on its screen: new step 4, `a_focused_cells_editor_unmounted_by_enter_drops_focus_and_never_reads_the_limit` in `stand_keyboard.rs`, both modes.
  - The engine's and the tree's focus are pinned as measured literals, neither naming the removed node. The forecast is the root `html` and the window.
  - A second `gate.py control` removes the removal path's `self.focus_node_id = None;` (`document.rs:1460`) and reads the test red.

## Moved by the fold
- **Touchpoints:** 3 → 4. `tests/blitz-tests/tests/stand_keyboard.rs` gains one test; no shared table changes.
- **Gates:** 9 → 10.
  - New entry 2, `cargo test -p blitz-tests --locked --test stand_keyboard`, baseline green with the standing tests (5.5 s).
  - The guard (entry 6) allows the fourth touchpoint. It re-baselines green, and its clone pair was re-run: a `stand_keyboard.rs` edit reads clean, and a `stand_contrast.rs` edit beside it reads outside 1 file.
- **Acceptance:** a criterion for the stand search and its Cells reading was added.
- **Steps:** 5 → 6; the controls are step 6.

## The re-snap (the review word of `.andromeda/runs/2026-11-18T01-29-40-phase/p5-card-2.md`)
- The review word is re-snapped as `inputs#I3`, its origin «the operator's review word, at the P5 review of `.andromeda/runs/2026-11-18T01-29-40-phase/p5-card-1.md`».
- Every citation of `inputs#I2` moved to `inputs#I3`: 4 in the plan (rev 5 → 6), 3 in research (rev 2 → 3) and the 2 in this card.
- `I2` stays in the chunk's manifest, uncited. The snapshot tool has no retire verb, and `inputs.py verify` reads it `UNCITED`.
- Nothing else changed: no step, criterion, touchpoint or gate entry.

## Validation
The whole mechanical set was re-run on rev 6:
- the fence parses (10 entries);
- planlint has 0 hits, its step references resolving 12 of 12;
- snapshot citations resolve (I1 cited 4 times, I3 7 times, I2 uncited), and research's lists parse (4 items);
- `gate.py baseline` reads every entry as at rev 5, so no baseline moved.

The check-7 code-graph WARN stands. Validation-1 is aligned: the reading you asked about is now measured, not inferred.

## Capabilities
None claimed (`v030-15` is «Cold-agent bar»'s).

## Beyond the plan
| Proposal | Owner |
|---|---|
| (standing — `.andromeda/runs/2026-11-18T01-29-40-phase/p5-card-1.md`) `docs/capabilities.md` §Accessibility reach goes stale with this chunk. It states «fourteen files and read 14 · 84 · 0 · 0 … on CI#40134371519» (a 0.2.0 run), and it does not name the CARRY's limit (a focused control that no longer shows keeps focus, the tree naming the window). The fix is to re-state the leg's eighteen files with this chunk's CI reading, and add the limit to the named limits. Approved at the review (inputs#I3). | The existing entry «Capability statement» (a `CARRY:` the wrap writes on it) |

Apply? (yes / review / cancel)
