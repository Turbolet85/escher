# Tab order on the base markup — step 6's one-shot measurement

- **Measured:** 2026-10-06T12:27Z, at /implement, before step 5's markup attributes landed (`git status --short examples/` empty; the four task files byte-equal to `076d74cb`).
- **How:** each lean task booted through `stand::boot(task, stand::options(incremental))`, then `focus_next_node` from the root element, each focused node read through `DioxusDocument::element_id`, until it returned `None` or repeated. This is `focus_sequence` in `tests/blitz-tests/tests/stand_accessibility_ids.rs`.
- **Reading:** the first run used empty expected lists, and its assertion printed the incremental=false sequences below. The second run pinned those lists in `tab_order`, and `tab_order_is_unchanged` read `ok` in both layout modes on the same base markup.

| Task | Focus sequence (both layout modes) |
|---|---|
| Counter | `back-btn` → `counter-increment` |
| FlightBooker | `back-btn` → `flight-one-way` → `flight-return` → `flight-start` → `flight-book` |
| Timer | `back-btn` → `timer-duration` → `timer-reset` |
| Crud | `back-btn` → `crud-filter` → `crud-name` → `crud-surname` → `crud-create` |

The base markup skips `flight-return-date` (disabled while one-way) and `crud-update` / `crud-delete` (disabled with no selection). Step 8's `tab_order_is_unchanged` pins these four lists.
