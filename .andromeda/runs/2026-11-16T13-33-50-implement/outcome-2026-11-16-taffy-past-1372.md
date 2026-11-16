# Outcome — 2026-11-16-taffy-past-1372 (implement run 2026-11-16T13-33-50-implement)

**Outcome: green + smoke.** Next → the OPERATOR PASS: the plan's `operator` entries 19 (`gate.py hygiene`), 20 (the
push) and 21 (`ci.py conclusion --sha HEAD --wait 2400`), which this skill never fires; then `/andromeda-wrap-session`.

## Files changed (16, base `930ac5d6fc9e39677f1bcc6199e92259161fe93b`)
The measured `take.patch` (md5 `802cba9d59a46c67081a42b33179c25d`), applied whole by `git apply --index`:
`Cargo.toml` · `Cargo.lock` · `packages/blitz-dom/src/{debug.rs, events/pointer.rs, layout/inline/layout.rs,
layout/mod.rs, layout/replaced.rs, layout/writing_mode.rs, mutator.rs, node/node.rs, node/scrollbar.rs, scrolling.rs}` ·
`tests/blitz-tests/tests/{display_contents.rs, inline_box_scrollable_overflow.rs, pre_overflow_scroll.rs}` ·
`wpt/runner/src/test_runners/attr_test.rs`. On top of it (unstaged):
- step 2 — `packages/blitz-dom/src/node/node.rs` `Node::hit_inner`, the clip stop's early return answers
  `x - content_box_offset.x`, `y - content_box_offset.y` (pays `cry-yrdxzd7k`);
- step 3 — `packages/blitz-dom/src/mutator.rs` `a_border_hit_on_a_clipping_inline_root_reads_content_box_coordinates`,
  both layout modes.
Evidence: `.andromeda/cache/chunks/2026-11-16-taffy-past-1372/evidence/take.md` (the picks, trailers, lock delta,
tallies) and `evidence/control-1.txt`.

## Gates (gate trail `.andromeda/runs/2026-11-16T13-33-50-implement/gate-2026-11-16-taffy-past-1372.json`)
`entries 21 · green 18 · red 0 · recorded 0 · timeout 0 · not-run 3` — the three not run are `leg operator`. First
firing, no iteration. No deferral (rust delta 16; walk-class 0 marked).
Step-6 control: `control clip-stop-frame · entry 6 · holds — in place green, removed red · exit 0 ✗ (exit 101)` ·
`control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
Smoke: entry 10 (`role = 'smoke'`) ran green as a P2 gate — stayed up 10 s, logged `service.name=seven_guis`.
No `gate.py guard` entry in the block.

Tallies (`target/ci-logs/`, ANSI stripped): `test` 242 · 1511 · 0 · 13 (forecast 1511; +10 over HEAD attributed by
name in take.md); `embolden` 1 · 6; `a11y` 14 · 115 · 0; `stand-repeat` 258 · 1380 · 0 · 24, 3 of 3 rounds green;
`agent-run run all` run.end 851 passed · 0 failed · 13 ignored (845 + upstream's 6).

## Deviations
- The plan's touchpoint line and research call upstream's `inline_box_scrollable_overflow.rs` additions «five»
  reversed-origin tests; the file adds six (`git diff 930ac5d6f`, the added `#[test]` functions, listed in take.md).
  A wording count only — the patch was applied as measured, the tally forecast (1511) held.
- scope record: none — `gate.py scope` clean, 0 recorded (`changed 16 · listed 16 · recorded 0`).

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| seven_guis_native (the smoke) | a leg (entry 10) | terminated — `timeout 10` |
| cargo / rustc / escher-session (the legs, the control's clone) | a leg | terminated |

`gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc`: `census: none running`.

## Decisions & corrections
- Guard refusal: the Bash guard refused `cd` into the skill's references directory («moves the session cwd»); re-issued
  once with absolute paths.
