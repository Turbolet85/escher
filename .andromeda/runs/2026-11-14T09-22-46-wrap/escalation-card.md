# Escalation card — 2026-11-14-bundled-fonts, P2 (1 operation)

Q1 — design-system `decision/stand-families-read-bundled-font` (ops-design-system.md, @row_add n=1, area typography)
- design-system decision/stand-families-read-bundled-font · fit: stands as decision/unavailable-families-use-default-font (design-system-fit.md n=1, sha 53bf5e45a6d4)
- The pair (check 2): architecture `decision/stand-typography-dejavu-sans`, as set here (fit: fits), states «every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface». Expected amendment 3 (the plan approved at P5, inputs#I4) names design-system §Typography for the same reading.
- Options:
  - A (recommended) — reject the add: the families reading keeps its one home in the architecture row, which takes `stand_bundled_font_families` as its guard by a second set (ops-main-architecture.2.md, fitted in the next wave). Expected amendment 3 is then disposed into architecture.
  - B — set the row fit names instead: design-system `decision/unavailable-families-use-default-font` gains the stand reading. Fit's `stands`, but that row is upstream's engine fallback, which the stand row does not restate.
  - C — apply the add as the plan named, in your own words, and drop the architecture row's «not on the design-system product surface» clause by a second set.
