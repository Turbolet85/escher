# Implement outcome — 2026-10-14-stand-requirement-sweep

**Outcome: green + smoke.** The 14 non-leg gate entries read green in one pass: trail
`.andromeda/runs/2026-10-14T21-20-05-implement/`, summary `entries 17 · green 14 · red 0 · recorded 0 · timeout 0 ·
not-run 3`. The 3 not run are the operator entries 15 · 16 · 17. There is no engine change: the preservation probe
(entry 8) and the task-file probe (entry 9) are both green.

**Next → the OPERATOR PASS**, then `/andromeda-wrap-session`:
- entry 15: `gate.py hygiene`, before the pre-CI commit;
- entry 16: the `fast` gate, the clean-tree guard and the push, after `chore(2026-10-14-stand-requirement-sweep):
  operator pre-CI commit`;
- entry 17: `ci.py conclusion --sha HEAD --wait 1800`. Its macOS matrix leg is the one reading of
  `a_state_directory_of_90_bytes_holds_a_session_and_one_of_91_is_refused_with_its_bound` (step 10 / C9). That test
  is compiled out on this Linux host.

## Files changed

New (3):
- `tests/blitz-tests/tests/stand_requirement_sweep.rs`: the driver sweep, and `WITNESSES`, 16 rows.
- `tests/blitz-tests/tests/stand_non_lean_tasks.rs`: C3's count, C12's dialog rule, the dialog's keys.
- `tests/blitz-tests/tests/password_caret_click.rs`: C11.

Modified (12):
- `examples/seven_guis/src/tasks/{temp_converter,circle_drawer,cells}.rs`: 9 added `id:` lines, nothing else (entry
  9 counts 0 other changed lines).
- `tests/blitz-tests/tests/stand_actionable_keys.rs`: the pin moved to 0 · 0 · 0.
- `tests/blitz-tests/tests/stand_act_obstructed.rs`: C5.
- `tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs`: C4 ×3 and C6(1).
- `tests/blitz-tests/tests/scrolled_box_client_rect.rs`: C6(2).
- `tests/blitz-tests/tests/range_input_interaction.rs`: C13 ×2.
- `examples/seven_guis/tests/cli_commands.rs`: C9.
- `packages/escher-driver/src/{refusal,json,lib}.rs`: `covered`'s second measured limit.

Evidence (written by hand, as plan steps 11 and 12 direct):
- `chunks/…/evidence/measurements.md`;
- `chunks/…/evidence/log-counts.md`, and its instrument `evidence/log-counts.py`.

Scope record: `chunks/…/scope-record.md`, 3 lines; `gate.py scope` reads `clean`.

## Measured readings (every prediction held unless said)

| Item | Predicted | Measured |
|---|---|---|
| C3: reacts to a click, no role | 0 on 6 tasks, Circle Drawer 1, Cells 676 | the same |
| C2: `unkeyed_actionable()` on the non-lean tasks | 0 · 0 · 0 | 0 · 0 · 0, and 0 with the dialog open |
| C12: the dialog range width | 240 | 240 |
| C5: driver click beside a one-axis clip | `covered` | `covered`, instance unchanged |
| C4: hit at an `img` / inside a `srcdoc` iframe / inside a filled `input` | unmeasured | `img` / `iframe` / `input` |
| C6(2): vertical-lr scrolled box | rect unchanged | unchanged, per-package build and workspace build |
| C13: removals and type change | re-seed and re-sanitize | 5.5 · 6 · Home 0 · End 100; text clears, back re-seeds 4 |
| C11: caret index, narrow vs wide | they differ | 5 vs 2: they differ |
| C16: workspace build `blitz_dom::document` lines | may rise | 2, where the dated reading is 1: host 6 lines / 978 B (dated 5 / 860), `mcp` 11 (dated 10); built alone unchanged at 4 / 742 and 9; 0 needles everywhere |
| `run stand` | 40 files, 162 + the tests added | 168 · 0 · 7 over 40 files (+6) |
| Workspace | — | 185 · 951 · 0 · 12 (base 182 · 937 · 0 · 12; +3 targets, +14 tests) |
| `a11y` leg | six files unchanged | 6 · 6 · 4 · 8 · 5 · 14 |

## Deviations from the plan, with justifications

1. **The scope record's 3 lines.** `refusal.rs`, `json.rs` and `lib.rs` are `in-intent · serves step 5`. Research's
   lists named `schema.rs` for C5, while plan step 5 puts the sentence in `covered`'s meaning, its pinned test
   sentence and the crate doc. `schema.rs` is untouched.
2. **The sweep's step order.**
   - Implemented order: snapshot → `click task-title` (empty diff) → `press tab` (first Tab stop focused) → the
     acting call → two screenshots → the `not-found` click.
   - Why it differs from the plan's listing (acting call first): since 2026-10-13 a click focuses a button. A Tab
     after `click("counter-increment")` would focus the stop after it, not `tab_order(task)[0]`, and a heading click
     after a focus risks a blur in its diff.
   - Every listed assertion is made.
3. **`WITNESSES`' subjects.**
   - Two v010-14 witnesses reach no session (`help`), and the plan's subject set (a lean task, `fixture`, `all four`)
     has no task-less value. They carry `AllFour`, and `Subject::AllFour`'s doc says "for a verb that reaches no
     session, whichever task a host boots".
   - Matrix refs that are unit tests in `packages/escher-driver/src` (v010-13, v010-14) are not carried: the plan
     makes the file a test file.
   - Witnesses per row follow the matrix refs, trimmed to one or more per task-specific half.
4. **C6(1)'s fixture.** The phrase "a hit inside the box where scrolled-out content would lie" has no point inside
   the box as written: scrolled-out content lies outside it. The fixture gives it one:
   - the first row is as wide as the box and the rest are 200 wide;
   - the point lies inside the box, where the first row lies unscrolled and beside the narrower row showing there;
   - it answers the box. A walk that dropped the scroll would answer row 0, and one that dropped the transform would
     answer the stage.
5. **A reading measured beside C6(1), and pinned as that test's fixture condition.** `get_client_bounding_rect`
   reads a `translate(50px, 30px)` box at its layout position `(0, 40, 300, 140)`, not where its transform moves it.
   The test's points are the rects' points moved by the stated translate. This is a new engine fact; no route entry
   owns it (see Surfaced).
6. **`cli_commands`' helper.** It adds `assert!(!message.contains(&given))`, "holds no path", which the plan asks for
   the 90 case. Since the helper is shared, the 94 Linux test now asserts it too.
7. **The C11 fixture.** The plan reads "the bullet it paints at". It is the same number as the editor's caret index
   in characters: paint maps the index to that many bullets (`render.rs:997-1000`). So the test reads the index once,
   in characters.
8. **The C3 table.** It is keyed by the `#task-title` text, which is the same as the plan's task names.
9. **The step 11 instrument** is kept as `evidence/log-counts.py`: Python, not a plane file. It leaves
   `target/debug/escher-session` workspace-built, and later per-package builds rebuild it.

## Surfaced (no spec authored — for the wrap)

- **The client rect reads a transformed box untransformed** (deviation 5). This is a measured engine reading that no
  CARRY or residual names. It needs a route owner at the wrap. It is not a spec↔reality gap against a stated
  contract: architecture's bridge says a box "reads its own `bounds` where it stands", and says nothing of
  transforms.
- **C16 moved obs-plan's dated counts** under the workspace build (2 `focussed node` lines, 6 / 978 B, 11): obs-plan
  §2 · §6 · §8 to amend, as the plan's expected amendments say.
- **`.claude/docs/services/escher-driver.md`** still quotes `covered`'s one-limit sentence. It is a doc outside
  implement's write set.
- No gated cap. No matrix write: `matrix.py show --chunk` reads 0 claimed. The plan claims none, and the pool is
  `[v010-15]`.

## Process hygiene

| Process | Started by | Final state |
|---|---|---|
| cargo / rustc / test binaries of the gate entries 1–14 | this run (gate tool) | terminated, measured by `ps -eo pid,comm,args` at 21:39:15Z: no `cargo`, `escher-sessio` or `seven_guis` process |
| `escher-session` session hosts started by `stand_session_*`, `cli_*`, `host_*` and `mcp_*` | a leg (those tests) | terminated, same census |
| `escher-session serve crud` ×4 and `escher-session mcp` ×4 (step 11 capture) | this run | terminated: each exit 0, no state dir left, same census |
| ad-hoc `cargo test` runs during P1 | this run | terminated, same census |

## Decisions & corrections

- Guard refusal: `bash-guard.sh` refused a `cat` heredoc writing a scratch script ("a cat/tee heredoc with a file
  target"). It was re-issued once through the Write tool and ran.
- No operator correction or ruling arrived during this run. No recurrence of a standing learning was met: every
  pipeline tool ran bare or redirected to a file read whole, and no bounded-repetition grep was used.
