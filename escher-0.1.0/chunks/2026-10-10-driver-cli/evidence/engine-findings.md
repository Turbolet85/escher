# The two engine findings — the as-built readings (step 1)

Taken on the base commit `0493d26a` before any source file changed, 2026-10-10, in process, in both layout modes
(`incremental` false and true; the two agree on every figure below). Instrument: a throwaway test target under
`tests/blitz-tests/tests/` over `session_common` and two in-file fixtures; deleted once recorded, in no commit. Both
readings are pinned by standing checks this chunk adds (`stand_act_scroll`, `stand_act_obstructed`). No file under
`packages/blitz-dom` is edited (fork 9).

Viewport 800 × 600, scale 1, not scrolled in any reading. Bounds are left, top, right, bottom.

## 1. A scrolled box's own bounds, and a `click` naming the box

**Hypothesis (route, from 2026-10-07-refusal-detection):** a snapshot node's `bounds` for a box that is itself
scrolled reads shifted by the box's own scroll offset while the box has not moved; "a driver `click` naming such a
box lands off its centre by that offset, and its `off-screen` reading is made at the shifted point (not measured)".

### On the stand's CRUD

| Creates, then `scroll` naming the last row | `crud-list` bounds before | `crud-list` bounds after | Shift |
|---|---|---|---|
| 12 (the standing case, last row `crud-person-14`) | 24, 112.796875, 393, 532 | 24, 94.796875, 393, 514 | 18 up |
| 14 (last row `crud-person-16`) | 24, 112.796875, 393, 532 | 24, 36.796875, 393, 456 | 76 up |

The box did not move: the rows read after the scroll lie where a box at the unscrolled bounds shows them (after 12
Creates, `crud-person-14` reads 501.796875 to 530.796875, inside 112.796875 to 532). The scroll's diff names
`crud-list` among its changed nodes, with all fifteen rows.

A driver `click` naming `crud-list` after the scroll, and the row it selected (read by a `click` on `crud-delete`
and the id its diff removes):

| Creates | Centre by the unscrolled bounds | Row lying there | Centre by the bounds as read | Row lying there | `click` on `crud-list` | Row selected |
|---|---|---|---|---|---|---|
| 12 | 208.5, 322.3984375 | `crud-person-7` | 208.5, 304.3984375 | `crud-person-7` | accepted | `crud-person-7` |
| 14 | 208.5, 322.3984375 | `crud-person-9` | 208.5, 246.3984375 | `crud-person-7` | accepted | `crud-person-7` |

### On a fixture (a button, then a 100-high scrolling box of ten 40-high rows)

| Step | `fx-box` bounds |
|---|---|
| at boot | 8, 31, 792, 131 |
| after `scroll` naming the last row (offset 300) | 8, -269, 792, -169 |

A driver `click` naming `fx-box` after the scroll is **refused `off-screen`**, and no row's handler ran. The box is
on screen at 31 to 131.

### What the readings say of the hypothesis

**Confirmed, both halves.** The click point is the centre of the bounds as read, so it lands above the box's centre
by the box's scroll offset: at 12 Creates the 18 units stay inside one 29-high row and the row selected is the one at
the box's centre; at 14 Creates the click selects `crud-person-7` while `crud-person-9` lies at the centre. And the
`off-screen` reading is made at the shifted point: once the shift takes that point out of view, a `click` naming a
box that is in view is refused `off-screen`.

What stands for an agent: a box that is itself scrolled reads its `bounds` shifted by its own scroll offset and is
named in `changed` though it has not moved; a `click` or a `type` naming such a box lands that far from its centre,
or is refused `off-screen` while the box is in view. Elements inside the box read true bounds.

## 2. `covered` where a row scrolled out of its box extends

**Hypothesis (same origin):** "a control that sits where a scrolled-out row extends reads `covered`, and a pointer
click there lands on the hidden row (not measured — the census clicked no control under a clipped row)".

### On the stand's CRUD — no control reads it

For each of the seven controls (`back-btn`, `crud-filter`, `crud-name`, `crud-surname`, `crud-create`,
`crud-update`, `crud-delete`), the raw hit at the centre of its bounds:

| State | Controls whose hit answers the control itself | A row extends over a control's centre |
|---|---|---|
| 12 Creates, list not scrolled | 7 of 7 | no — `crud-person-14` reaches 548.796875; `crud-create`'s centre is at 560 |
| 12 Creates, after the `scroll` | 7 of 7 | no — `crud-person-0` reaches up to 95.796875; `crud-filter`'s centre is at 85.3984375 |
| 14 Creates, after the `scroll` | 7 of 7 | yes — `crud-person-1` reads 66.796875 to 95.796875 across `crud-filter`'s centre, and the hit there still answers `crud-filter`; a driver `click` on `crud-filter` is accepted |

So no stand control reads `covered` in any state measured, and the fixtures carry the case. (The hit at the centre of
a row scrolled out above the list answers what shows there — the task's body or `crud-filter` — not the row; the hit
at the centre of `crud-person-14`, scrolled out below the unscrolled list, answers that row, as measured before.)

### On two fixtures

| Fixture | Geometry | Raw hit at the control's centre | Driver `click` on the control | A raw pointer click there |
|---|---|---|---|---|
| a button **after** the box (the box not scrolled; rows 3 to 9 extend below it over the button) | box 8, 8, 792, 108 · button 8, 108, 71, 131 · row 2 reads 88 to 128 | the button | accepted; the button's handler ran | the button's handler ran; no row's |
| a button **before** the box (the box scrolled by 300; rows 0 to 7 extend above it over the button) | button 8, 8, 73, 31 · box on screen at 31 to 131 · row 7 reads 11 to 51 | `fx-row-7`, a row scrolled out of the box | **refused `covered`** | the row's handler ran; the button's did not |

### What the readings say of the hypothesis

**Confirmed in one arrangement, read false in the other.** A control that comes before a scrolling box in the
document, lying where content scrolled out of that box extends, reads `covered` though nothing shows over it, and a
pointer click there reaches the hidden row. A control that comes after the box is hit ahead of the box's
overflowing rows and is clicked. On the stand no control reads `covered` this way, including one a scrolled-out row
extends across.

What stands for an agent: a hit reaches content scrolled out of a scrolling box, so a control lying where such
content extends can read `covered` though nothing shows over it.
