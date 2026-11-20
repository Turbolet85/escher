# Founder forks — Select list and selection debts (v040-04)

Chunk `2026-11-20-select-list-and-selection-debts`, escher-0.4.0, Epoch 2 — Debts of 0.1.0–0.3.0.
Intent §Findings 4: each debt comes to the founder as a fork — **fixed**, or **kept as a residual with his reason**.
Five forks, one per debt. Each one lists its measurement, every option with its cost, the recommended option first,
and what it depends on. A fork can be answered by its letter alone, e.g. «F1 A · F2 A · F3 A · F4 C · F5 A».

What every "fix" answer brings with it:
- **Engine changes** (F2, F3, F4) change behaviour for every document, remote pages included. Like the earlier
  select-list rulings, each one is ratified by your answer.
- **Proof**: every fix is proven headless in both layout modes, on the stand or on a fixture page. Every "keep"
  leaves its residual standing, with your reason, and its named limit in `docs/capabilities.md`.
- **Out of scope**: the select list's thumb drag (`res-ammkrycp`) belongs to the later `drag` entry.

---

## F1 — Highlighted-row contrast (`res-tym5wois`)

**Measured**
- An open list paints its highlighted row in two fixed engine colours: black text `#000000` on grey `#c8c8c8`.
  The contrast is 12.56:1. No page can change these colours (`SelectListView` constants,
  `blitz-dom/src/node/select_list.rs:114-116`).
- A `multiple` select's chosen row uses the same pair, but from the default stylesheet
  (`default.css:226-230`). A page's own CSS can override it there.
- The lint's `contrast` check (`escher-driver/src/lint.rs:537-554`) reads each row's colours from the page's
  styles. On the highlighted row it therefore reads the plain row's colours, not the grey highlight that is
  actually painted.
- Result: the lint can never miss a bad highlight, because 12.56:1 always passes. But when a page's plain rows
  fail, the lint also reports the highlighted row, which is not showing those colours: a false finding on that
  one row.
- On the stand, the flight booker's plain rows read 17.40:1, so the stand shows no finding either way today.
- A `multiple` select's chosen row is already read correctly, page overrides included, because its colours come
  from the stylesheet.

**Options**
- **A (recommended) — The lint reads the highlighted row's painted colours.** The highlighted row is measured as
  painted: the engine constants, through the harness's one contrast formula. A test also pins the open list's
  highlight constants to the stylesheet's chosen-row colours, so the two can't drift apart.
  - *Cost:* small. A change in the driver lint plus one check in each layout mode. No engine change for other
    documents and nothing new on the carriers (a contrast finding keeps its shape).
  - *Gain:* the lint reports what is on screen, and the shared-pair promise is checked by a test.
- **B — Keep.** The residual stays, with the reason: the highlight is a fixed 12.56:1 pair that cannot fail, and
  the plain-row reading on that row is a stated limit.
  - *Cost:* nothing now. The possible false finding on the highlighted row stays.
- **C — The highlight takes the page's colours** (e.g. from the page's chosen-option styles), and the lint reads
  them.
  - *Cost:* medium, and it undoes your earlier F6 ruling that the list's colours are engine constants, not styles.
    No web standard defines a page-styled picker highlight; Chromium uses system colours. A page could then make
    the highlight unreadable, which is a new kind of failure to lint.

**Depends on:** none. F4 A leans on it: a sized single select's chosen row uses the chosen-row colour pair.

---

## F2 — A larger option font is clipped (`res-2iu7qn2e`)

**Measured**
- Every row in an open list gets one height: the select's own font on one line, plus 1px above and below
  (`blitz-dom/src/document.rs:985-986`, measured on the select's text).
- Each row's text is laid out in that row's own font (`select_row_layout`). The text is then clipped to the row
  (`blitz-paint/src/render/select_list.rs:121-124`).
- So an option or group whose authored font is larger than its select's font is cut to the select's line
  height. A smaller font is centred.
- All the list's geometry (hits, scrolling, the thumb, the 20-row cap, placement) assumes rows of equal height.

**Options**
- **A (recommended) — Row height = the tallest row's line height.** Rows keep one shared height, but it becomes
  the largest line height across all rows (headers included) plus the padding.
  - *Cost:* small. Opening the list already measures every row's width, so the height comes from the same loop.
    Geometry, hits, scrolling and placement are unchanged.
  - *Trade-off:* one large option makes every row tall, so fewer rows fit on screen before scrolling.
- **B — Each row gets its own height** (closer to how a browser's own picker sizes mixed fonts).
  - *Cost:* medium to large. The equal-height model is rewritten: hits, scroll-into-view, the thumb, which rows
    are in view, placement and the 20-row cap, with their unit tests and `select_popup`'s geometry tests.
- **C — Keep.** The clipping stays as a stated limit.
  - *Cost:* nothing now. Pages with larger option fonts stay cut off.

**Depends on:** none. It is an engine change for every document (A or B).

---

## F3 — The highlighted row is invisible to agents and assistive tech (`res-caouqg3t`)

**Measured**
- While the list is open, focus stays on the select: in the engine, in the snapshot and in the accessibility
  tree. The stand's keyboard walks require all three to agree.
- The tree reports the select as `expanded`. The highlighted row appears in neither the tree nor the snapshot.
- Moving the highlight records no change, so the tree is not refreshed and a `press` returns an empty diff
  (the change-tracking rule for open lists).
- So an agent pressing arrow keys in an open list sees the highlight only in a screenshot. A screen reader hears
  that the list opened, but not which row is highlighted.
- The snapshot has 14 node keys. Your 2026-10-10 rule: a new field with a closed set of values inside an
  already ratified carrier is not a boundary widening.

**Options**
- **A (recommended) — Report the highlight in both the tree and the snapshot.**
  - The select's tree node points to the highlighted option as its active descendant, the standard way an
    assistive technology hears the current row while focus stays on the select.
  - The snapshot gets one new key, `highlighted`: true or false, written only when true and only on the
    highlighted row.
  - A highlight move becomes a recorded change, so the tree refreshes and an acting call's diff names the row
    (`differs: highlighted`).
  - *Cost:* medium. The snapshot goes from 14 to 15 keys and the diff readings from 12 to 13. The help text and
    the key count pinned in the capability statement change. Because the key is written only when true,
    existing screens are byte-identical, so the cross-OS answer digests and the committed checks don't change.
    Focus stays on the select, so the keyboard walks hold.
  - *Note:* the existing pin "`aria-activedescendant` unread" stays. The engine would set the active descendant
    itself and still read no page attribute.
- **B — Tree only.** Set the active descendant in the tree; add nothing to the snapshot.
  - *Cost:* small to medium. Screen readers would hear the row, but no assistive technology runs here to confirm
    it (the windowed witness is still owed). Agents stay blind to the highlight.
- **C — Move the tree's focus onto the highlighted row.** The snapshot's `focused` then names the row, and no new
  key is needed.
  - *Cost:* medium. It breaks the ratified rule that tree focus stays on the select while the list is open, and
    the stand's check that engine, snapshot and tree focus agree at every step.
- **D — Keep.** The row stays visible only in screenshots, as a stated limit.

**Depends on:** none of the other forks. F4 A interacts with it: a sized single select opens no list, so it has
no highlight, and its current row is read as `selected`.

---

## F4 — A single select's `size` is ignored (`res-ysz67imk`)

**Measured**
- `size` is read only on a `multiple` select (your F8, 2026-10-31). On a single select it is ignored
  (`blitz-dom/src/stylo/presentational_hints.rs:554`).
- So `<select size="4">` without `multiple` shows as a closed dropdown and opens the engine's list. HTML and
  browsers show it as an in-page list box, `size` rows tall, where the arrow keys move the choice directly and no
  popup opens.
- No stand app uses a sized single select. The limit is named in `docs/capabilities.md` (the accessibility reach).

**Options**
- **A — Render it as a list box, per HTML.** A single select with a valid `size` above 1 is laid out like a
  `multiple` select, but keeps one choice.
  - Its rows appear in the page, `size` × 1.2em tall, scrolling past that. It opens no list.
  - The arrows, Home, End and typeahead move the choice. A click on a row chooses it.
  - Tree role ListBox; each row reads its own position on screen in the snapshot; the lint treats its rows like a
    `multiple` select's.
  - *Cost:* large, probably the biggest item in this chunk. It touches the default stylesheet, the height hint,
    the select's key model (a third arm beside the closed single select and the `multiple`), row clicks, which
    selects open a list, the tree role, the snapshot's positions, the lint, and the driver's `type`/`actions`
    readings, with fixture proofs in both layout modes. It also overturns your F8.
- **B — Keep.** The limit stays, with your reason (e.g. no stand app needs it; the popup works).
  - *Cost:* nothing now.
- **C — Fix it in a separate route entry.** This chunk closes F1–F3 and F5. The sized single select gets its own
  entry later in Epoch 2 (or in 0.5.0's demo, if a sample needs it).
  - *Cost:* none here. A new route entry, which the wrap would insert.

My lean is between A and C. If you want this version to close every debt in full, take A. C closes the same debt
with less risk to this chunk's size. I'd pick **C** if the chunk's size matters to you, and **A** otherwise.

**Depends on:** F1 (under A, its chosen row uses the chosen-row colour pair, which F1 A pins) and F3 (under A, it
has no list and no highlight).

---

## F5 — Cells doesn't report its selected cell (`res-wvatzhpe`)

**Measured** (a live `escher-session` run on the Cells task, today):
- The Cells snapshot is **53,890 bytes in 6 parts**, 740 lines.
- Its 676 data cells each read `GenericContainer "" id="cell-A0" clickable @59,77 80x22`: no role, no selected
  state. The grid reads `GenericContainer "Cells"`.
- The UI graph answer is 293,869 bytes; the actions answer is 118,293 bytes.
- The current cell (the keyboard cursor, drawn with the 2px outline) is shown only by a CSS class. The flight
  booker and CRUD report their selected item by attribute.
- A stand rule says stand controls take names, roles and states through attributes alone (no new elements).
- The seven served tasks' boot readings are pinned by a committed cross-OS digest table.

**Options**
- **A (recommended) — The current cell reports itself as selected.** `aria-selected="true"` goes on the current
  cell only; other cells report nothing.
  - *Cost:* small. A stand-only change, by attribute alone, with no engine change. The snapshot grows by 9 bytes
    (one ` selected` word); a cursor move shows up in the diff as `selected` on two cells.
  - The Cells rows in the cross-OS digest table must be recomputed on the three CI OSes.
  - Cells' keyboard walk, contrast checks and ids are unchanged.
- **B — Full grid semantics:** `role="grid"`, `role="row"`, `role="gridcell"`, plus A's selected cell.
  - *Cost:* medium. ARIA's grid needs row elements, which Cells doesn't have. Adding them breaks the
    "attributes alone" stand rule and moves the layout, so the snapshot, the graph, the cross-OS digests and the
    keyboard and contrast checks all change.
  - Using `gridcell` on the cells *without* rows would be incomplete ARIA. It would shrink the snapshot by about
    5,400 bytes, because `GridCell` is shorter than `GenericContainer`.
- **C — Like A, but every other cell reports `selected="false"`.**
  - *Cost:* the snapshot grows by about 8,100 bytes (675 × ` unselected`), to about 62 KB, most likely 7 parts.
    No gain for agents over A.
- **D — Keep.** The residual stays, with your reason.

**Depends on:** none. A, B and C each change only Cells. "Selected" here means the current cell. While a cell is
being edited, it is also the current cell.

---

## How the forks depend on each other
- **F4 → F1:** under F4 A, the sized single select's chosen row uses the chosen-row colour pair, which F1 A pins
  with a test.
- **F4 → F3:** under F4 A, a sized single select opens no list, so F3's highlight reading doesn't apply to it;
  its current row reads `selected`.
- **F3 C ↔ the stand's keyboard walks:** only F3 C reopens the rule that focus stays on the select. A, B and D
  leave it alone.
- **F1, F2, F5:** independent of each other and of F3.

## Recommended set
**F1 A · F2 A · F3 A · F4 C · F5 A.** F4 A instead of C if you want every debt closed in this chunk.
