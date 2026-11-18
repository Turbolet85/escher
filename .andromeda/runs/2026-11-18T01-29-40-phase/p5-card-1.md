# P5 review — 2026-11-18-a11y-re-read · «A11y re-read»

**The plan:** `.andromeda/cache/docs/2026-11-18-a11y-re-read/plan.md` (rev 3), open it in an editor. No plan stands in a chunk's folder. Writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-a11y-re-read` diffs the chunk's records.

## Goal
Re-assert SC 2.1.1, 2.4.3 and 1.4.3 on the stand and its six served pages at HEAD, after 0.3.0's engine debts, select list and bundled fonts. The leg already reads green at HEAD `712e09b3c`: 14 files, 115 passed, 0 failed. Neither `stand_keyboard` nor `stand_contrast` has changed since select-popup, so nothing is re-baselined. The chunk:
- widens the `a11y` leg from 14 to 18 files on your word (inputs#I1);
- pins the CARRY's focus limit as a named SC 2.4.3 gap.

## Top acceptance criteria
- `ci-leg.sh a11y` exits 0 with 18 result lines, every one `ok`. The stand checks and their tables are unchanged, held by a guard limited to the three touchpoints.
- The leg holds the fourteen files plus `key_press_after_container_lost_its_box`, `select_popup`, `select_popup_painted` and `stand_act_popup`. The `stand-repeat` and `embolden` arms are unchanged, and `test_ci_workflows.py`'s tuple holds the 18 words.
- The CARRY's gap is pinned by `a_focused_control_that_no_longer_shows_keeps_focus_the_tree_naming_the_window`, in both modes, for `hidden`, `display:none` and CSS alone:
  - engine focus stays on the control;
  - the tree's focus reads the window;
  - one Tab moves to a measured literal (forecast `#after`);
  - a `gate.py control` on the tree-focus fallback reads it red.
- `fast` is green, and the operator pass's CI read of the pushed run is `verdict: green`.

## Touchpoints
- `.github/scripts/ci-leg.sh` — four `--test` words added to the `a11y` arm
- `.github/scripts/test_ci_workflows.py` — four words added to the `a11y` tuple
- `tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs` — the gap test and one `//!` sentence

## Test commands (9 entries)
| # | Entry | Baseline |
|---|---|---|
| 1 | `cargo test … --test key_press_after_container_lost_its_box` | green: the 3 standing tests |
| 2 | `ci-leg.sh a11y` | green: 14 files, 115 passed |
| 3 | count of the leg's `ok` result lines, `last line 18` | red at 14 (the subject); must-pass green over an 18-line log in a scratch clone |
| 4 | `ci-leg.sh ci-scripts` | green |
| 5 | `gate.py guard --base 712e09b3c` with the three touchpoints allowed | green; the clone pair reads a touchpoint edit clean and a `stand_contrast.rs` edit outside |
| 6 | `ci-leg.sh fast` | seen green at the last wrap |
| 7–9 | operator: hygiene, clean-tree push, `ci.py conclusion --wait 2700` | — |

Controls are recorded in `.andromeda/runs/2026-11-18T01-29-40-phase/p5-controls.md`.

## Capabilities
None claimed. The one pooled cap, `v030-15`, is «Cold-agent bar»'s, and its working entry names it. No concretization preview.

## Validation
- Mechanical checks pass: the fence parses, planlint has 0 hits, snapshot citations resolve (I1), and research's lists are parsed.
- WARN, check 7: no code-graph trace. The modify-set holds one `.rs` test file, and no engine or driver symbol changes, so research recorded the graph as not applicable.
- Size: about 110 lines of plan prose, under the ~150 floor but inside the 50 % band.
- Validation-1: aligned with the scope, with one reading for you to confirm. The CARRY says this entry's SC 2.4.3 re-assertion «on the stand and served pages reads it». The plan reads the gap on a fixture (`key_press_after_container_lost_its_box`), now in the `a11y` leg. It adds no stand-level reading, because no stand or served-page step is known to hide a focused control. That is unmeasured, not shown.
- CI on `712e09b3c` (CI#44215480492) now reads green, 17/17, wall 1802 s. No job was cancelled and `Coverage report` is green. The wrap counts that run for both WATCHes: `cry-y6764bis` (1/3 → 2/3) and `cry-nxmqzwtr` (0/3 → 1/3).

## Beyond the plan
| Proposal | Owner |
|---|---|
| `docs/capabilities.md` §Accessibility reach goes stale with this chunk. It states «fourteen files and read 14 · 84 · 0 · 0 … on CI#40134371519» (a 0.2.0 run), and it does not name the CARRY's limit (a focused control that no longer shows keeps focus, the tree naming the window). The fix is to re-state the leg's eighteen files with this chunk's CI reading, and add the limit to the named limits. | The existing entry «Capability statement» (a `CARRY:` the wrap writes on it) |

Apply? (yes / review / cancel)
