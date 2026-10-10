# Readings re-measured, and what the report carries (plan steps 11 and 12)

Implement's record, 2026-10-10, of the finished tree: the gate block's second full pass (run dir
`.andromeda/runs/2026-10-10T14-33-38-implement/`, 21 entries green, the 3 operator entries not fired). The red-first
readings, the mutation controls and what the pins measured are `controls.md`; the conformance reading is
`conformance.md`.

## Step 11 — re-measured, against what stood

| Reading | Stood | Predicted | Measured |
|---|---|---|---|
| `bash scripts/agent-run.sh run stand` (`run.end`) | 119 passed · 0 failed · 6 ignored over 31 files | unchanged | 119 passed · 0 failed · 6 ignored over 31 files |
| workspace result lines (`target/ci-logs/test.log`, written by the `fast` leg of this pass) | 162 lines · 760 passed · 0 failed · 11 ignored | 165 lines | 165 lines · 776 passed · 0 failed · 11 ignored |
| `cargo test -p escher-driver --locked`, unit tests | 50 | — | 50 passed |
| `bash .github/scripts/ci-leg.sh a11y`, its three targets | 6 + 6 + 3 | unchanged | 6 + 6 + 3 |

The 16 added passes are the three new files' tests: 6 + 9 + 1. No test was added to or removed from
`stand_act_scroll` (6) or `stand_act_obstructed` (5).

The stated tables in the blast radius, re-read by their own unedited checks, all green: `stand_act_ids` (its 77-node
table), `stand_act_diff`, `stand_diff`, `stand_snapshot`, `stand_snapshot_state`, `stand_accessibility_ids`,
`stand_act_spans`. No row of any of them moved.

## One standing check leaned on the hit defect — a finding, with its cause

`stand_actionable_keys::the_non_lean_tasks_are_measured` turned red at the first full pass (the stand run and the
`fast` leg, one cause). It opens each non-lean task by a raw harness click on the task's Home card. Measured at the
stand's viewport, Home unscrolled: `#home` is a scrolling box 600 high (`overflow-y: auto`), six cards lie inside it,
and the seventh card, Cells, lies at 615 to 684 — below the box. Its centre is out of view by `visible_region`, and a
hit there no longer lands on it; on the engine as built the hit reached it through Home's clipped-out content, which
is the defect step 6 fixes. The count the test pins (2 · 3 · 676) did not move.

The check now scrolls each card into view before its click (`Harness::scroll_into_view`), which is what a user or the
driver does. That file is outside research's lists: `scope-record.md` holds its line (companion, serves step 6). No
stand file was edited. Not measured: the windowed stand, where the same card is reached by scrolling Home.

## Step 12 — for the report

- **Upstreamable, both fixes**, cut against upstream `main` at `7832c177`; no upstream pull request opened (scope.md,
  CARRY 5). The engine diff is four files: `document.rs`, `layout/writing_mode.rs`, `scrolling.rs`, `node/node.rs`.
- **The second reader** (`Node::absolute_position`), for its route owner: measured figures in `controls.md` §6 — a
  box at (40, 50) scrolled by (70, 120) reads (−30, −70), both layout modes, both builds; pinned by
  `scrolled_box_absolute_position`, red by design when the reader is fixed. The harness's `layout_rect`,
  `layout_rect_of` and `center_of` are built on it.
- **The remainder under option 1B**: a `contain: paint` box with `overflow: visible` is still hit through, measured
  and pinned (`hit_clipped_at_scrolling_box`, its last test), and said by `covered`'s meaning. Not measured: an
  image's, a sub-document's and a text input's own box.
- **The one-axis case**: `overflow-x: clip` beside `overflow-y: visible` computes as written, and the hit stops at the
  box's whole padding box, as paint clips it. `visible_region` narrows such a box on its clipped axis only, so a
  target lying outside the box along its visible axis reads in view by the visible region while a hit there does not
  reach it: the driver would read it `covered`, not `off-screen`.
- **The conformance reading**: taken, inside its bound — 966 → 967 subtests passed of 2292 run; one subtest moved
  fail to pass (`css/cssom-view/getBoundingClientRect-scroll.html`, "getBoundingClientRect for a scrolled scroll
  container"), none moved pass to fail.
- **Shared modules per new check file**: `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box` and
  `scrolled_box_absolute_position` declare neither `common` nor `session_common`; none installs a subscriber, none
  carries a `cfg(unix)` gate.
- **CI runs of the operator pass**: none yet — the three operator entries (22, 23, 24) are not implement's to fire.

## How two plan steps were settled where the plan left room

- **Step 6, a transformed box.** The step tests the box's own area "before the scroll offset is added" and leaves the
  transform handling as it is; the walk inverse-transforms its point after the offset is added. The own-area and
  padding-box tests are made on the unscrolled point carried through the same inverse transform — paint pushes the
  clip before it translates the content by the scroll offset. For a box with no transform this is the plan's point
  exactly. **No check of this chunk reads a box that is both transformed and scrolled**; the standing `paint_order`
  and the rest of entry 7 are green.
- **Step 5, the loop's shape.** Both bodies keep the walk from the node itself and leave the node's own offset out
  inside it. Starting the walk at the containing block would have put `layout_data()` on an added line of the
  `writing_mode.rs` body, which the engine-lines probe (entry 15) forbids.
- **Step 7, the fixture test of `stand_act_scroll`.** Every row of the fixture rewrites one count, so "lands on the
  row at its centre" is read as: one row's bounds hold the box's centre, the hit there answers that row, and the
  click naming the box rewrites the count. On the stand's CRUD the row is identified by deleting it.

## Left on disk, outside git

`target/wpt-checkout/` (1.1 G), `target/wpt-reading/` (the four result files) and the release build of the runner
under `target/release/` — all ignored by git, none deleted.
