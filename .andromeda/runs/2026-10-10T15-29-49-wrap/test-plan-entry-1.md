
## 2026-10-10-scrolling-box-bounds-and-hit — the two pinned engine readings read fixed, their checks restated
**Section:** §1 Coverage scope → tests/blitz-tests (`stand_act_obstructed`, `stand_act_scroll`) · §3 → blitz-test-harness, Input helpers and Inspection helpers · §5 Boundaries covered → Session ↔ held instance
**Change:**
- §5: was "two engine readings are pinned as they read, not fixed — a scrolled box's `bounds` read shifted … and a control that comes before a scrolling box … reads `covered`"; now both are fixed in the engine and their checks restated — a scrolled box reads the same `bounds` before and after its content is scrolled, the scroll's diff does not name it, a `click` naming it lands on the row at its centre; a button lying where a scrolled-out row extends is clicked, its handler runs and no row's, and no element reads focused after. One reading stays: a hit still reaches content clipped by `contain: paint`.
- §5, the `scroll` clause: was "the list's `bounds` taken before the scroll, since a scrolled box's own `bounds` read shifted by its scroll offset"; now the list's `bounds` are taken once, before the scroll, and a scroll of its content does not move them.
- §1: `stand_act_obstructed` stays 5 and `stand_act_scroll` 6 — the two tests each gained at 2026-10-10-driver-cli are restated to the fixed readings, three renamed, none added or removed.
- §3 Input helpers: was "exercised through the driver's `scroll` only"; now `stand_actionable_keys` calls `scroll_into_view` too, and a raw `click` on an element outside the padding box of a box that clips it no longer lands on it. It still has no check of its own.
- §3 Inspection helpers: gains the measured reading of `layout_rect`, `layout_rect_of` and `center_of` for a scrolled box — its position less its own scroll offset, (40, 50) scrolled by (70, 120) reading (−30, −70) — pinned, not fixed.
**Why:** the bounds reader and the hit walk were fixed in the engine on the founder's ruling of 2026-10-10. A check written with a raw harness click on an element outside its clipping box leaned on the old hit and holds none of the words a sweep for it searches: it is found by running the stand gates.
**Kept:** the harness's three position readers as they read — their fix is owned on the route.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/
