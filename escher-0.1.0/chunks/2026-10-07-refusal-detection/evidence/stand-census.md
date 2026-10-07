# Stand census — 2026-10-07-refusal-detection (plan step 9)

Measured 2026-10-07 through `Session::run`, with plan steps 1–6 in place, by a scratch test
booted through `seven_guis::stand` with `stand::options(incremental)` (800 × 600, scale 1, the
bundled font). The scratch test was removed after the reading; `stand_act_ids` and
`stand_act_scroll` assert what is recorded here. Both layout modes read the same in every row.

## Which snapshot nodes a driver `click` is refused on

One fresh session per node, the node named to `click` at boot.

| task | mode | nodes | accepted | refused |
|---|---|---|---|---|
| Counter | non-incremental | 13 | 12 | `task-header-spacer` — `covered` |
| Counter | incremental | 13 | 12 | `task-header-spacer` — `covered` |
| Flight Booker | non-incremental | 18 | 16 | `task-header-spacer` — `covered` · `flight-return-date` — `disabled` |
| Flight Booker | incremental | 18 | 16 | `task-header-spacer` — `covered` · `flight-return-date` — `disabled` |
| Timer | non-incremental | 19 | 18 | `task-header-spacer` — `covered` |
| Timer | incremental | 19 | 18 | `task-header-spacer` — `covered` |
| CRUD | non-incremental | 27 | 24 | `task-header-spacer` — `covered` · `crud-update` — `disabled` · `crud-delete` — `disabled` |
| CRUD | incremental | 27 | 24 | `task-header-spacer` — `covered` · `crud-update` — `disabled` · `crud-delete` — `disabled` |

Per mode: 77 nodes, 70 accepted, 7 refused (3 `disabled`, 4 `covered`).

- **Against the forecast** (77 nodes, 3 refused, 74 accepted): the node count holds and the
  three disabled controls are refused as predicted. The forecast missed `task-header-spacer`,
  refused `covered` in all four tasks.
- **Why the spacer reads `covered`**: its bounds are 0 high (603 × 0 on Counter, 564 × 0 on
  Flight Booker, 620 × 0 on Timer, 619 × 0 on CRUD, each at y 22.5). The point a click would
  land at is its centre, and the raw hit there answers the header that holds it, which is
  neither the spacer nor one of its descendants. Before this chunk a driver click on it ran and
  landed on the header.
- **`timer-progress`**, 0 wide at boot — the node the code read did not settle — is accepted:
  the hit at its centre answers the element itself.

## The CRUD list

- **Clicks**: 12 driver clicks on `crud-create` put the last row's centre outside the list's
  box. Each click returned `settled` and added one row; the twelfth added `crud-person-14`.
- **At boot**: `crud-list` bounds (24, 112.796875, 369 × 419.203125); `task-body` bounds
  (0, 46, 800 × 554); a row is 367 × 29 (`crud-person-0` at y 113.796875).
- **After the twelfth click**: `crud-person-14` bounds (25, 519.796875, 367 × 29), centre y
  534.296875; the list's box ends at y 532. The viewport is unscrolled.
- **Which box clips the row**: `crud-list`. The engine reader (`visible_region`) answers
  (25, 113.796875, 367 × 417.203125) for the row — the list's padding box — not `task-body`'s
  box and not the viewport.
- **A `click` on the row**: refused `off-screen`.
- **`scroll` naming the row**: `settled` true, `busy` none, `in_view` true, in one step. The
  diff adds nothing, removes nothing and names 16 nodes changed: `crud-list` and the 15 rows
  `crud-person-0` … `crud-person-14`.
- **Bounds after the scroll**: `crud-person-14` (25, 501.796875, 367 × 29) — the list scrolled
  by 18, the row's bottom now at the list's padding edge; `crud-person-0`
  (25, 95.796875, 367 × 29), its centre above the list's padding box.
- **The document reads animating after the scroll** (the overlay-scrollbar fade) and the step
  still reads `settled`, as the plan's implementation notes predicted.
- **A second `scroll` naming the same row**: `settled`, `in_view` true, an empty diff.
- **A `click` on the row after the scroll**: accepted; its diff names `crud-update` and
  `crud-delete` changed (both now enabled).
- **The first row after the scroll**: `click` on `crud-person-0` refused `off-screen`;
  `crud-person-1` accepted.

## Two findings for the wrap, decided by nothing here

- **The raw hit at the clipped row's centre answers the clipped row itself.** Before the
  scroll, a hit at (208.5, 534.296875) — below the list's box — resolves to `crud-person-14`:
  the hit walk passes a point outside a scrolling box to that box's scrolled-out children, as
  research predicted from `node.rs`. The driver never sends that click: `off-screen` is read
  first, from geometry.
- **A scrolled box's own `bounds` read shifted by its own scroll offset.** `crud-list` did not
  move on the screen, and its snapshot `bounds` read (24, 112.796875, …) before the scroll and
  (24, 94.796875, …) after it — 18 lower in y, the list's own scroll offset. That is why the
  diff names `crud-list` among the changed nodes. The cause is in the engine's
  `get_client_bounding_rect`, which positions a node through a reader that subtracts the
  node's own scroll offset; the box's content moves by that offset, the box does not. It
  predates this chunk and lives in `packages/blitz-dom/src/document.rs`, a file this chunk
  does not edit. Consequence for the checks: "the row's centre lies inside the list's box"
  cannot be read against the list's `bounds` taken after the scroll (the row's centre, y
  516.296875, is below the misread box's end, y 514); `stand_act_scroll` reads it against the
  list's `bounds` taken while the list was unscrolled, which is the box the list still
  occupies, and against the engine reader.
