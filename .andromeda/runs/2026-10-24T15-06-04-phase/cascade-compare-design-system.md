# Cascade compare — design-system · phase · 2026-10-24-stand-requirement-sweep
E items 29 · held 15 · lost at classify 3 · lost at strong 2 · cascade only 3 · body 6 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### motion#1.2 · lost at strong · .andromeda/design-system-detail/motion.md:9
- (.andromeda/design-system-detail/motion.md:9) A sweep check on the clock's smooth-scroll reader must match the 300 ms ease-in-out duration.
  > Smooth scrolls run 300 ms on a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:112-122; packages/blitz-dom/src/scrolling.rs:432-433)

### spacing#2.5 · lost at classify · .andromeda/design-system-detail/spacing.md:20
- (.andromeda/design-system-detail/spacing.md:20) Any fixture page the sweep adds (C1's `truncated`, `clipped`, `off-screen` and `text_extent` readings, both layout modes) is laid out under the engine's default body margin, so every bound or extent a fixture check asserts must account for it or the page must set its own body margin explicitly.
  > - blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:274-277)

### spacing#2.4 · lost at classify · .andromeda/design-system-detail/spacing.md:19
- (.andromeda/design-system-detail/spacing.md:19) The sweep re-proves the stand's lint readings (each lean task 0 findings at boot, the last CRUD row `clipped` inside `crud-list`) on the stand's own geometry; the stand's card and home padding stay as recorded — the sweep changes no stand spacing, and any stand spacing change would move those readings.
  > - seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:38-42; examples/seven_guis/src/app.rs:191)

### typography#1.9 · lost at classify · .andromeda/design-system-detail/typography.md:18
- (.andromeda/design-system-detail/typography.md:18) A stand-based lint reading (a CRUD row's `clipped` and `truncated`, a `contrast` finding's large-text threshold) is measured against the stand's own type values, which the check must not restate differently.
  > | seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |

### typography#1.13 · lost at strong · .andromeda/design-system-detail/typography.md:22
- (.andromeda/design-system-detail/typography.md:22) A fixture page that sets no font size lays its text out at the engine's generic base size, the value a fixture's expected text extent derives from.
  > | Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |

## Held

- color-palette#2.3
- color-palette#3.5
- color-palette#4.4
- depth-strategy#3.1
- motion#1.4
- motion#1.6
- motion#2.2
- motion#3.1
- motion#4.1
- surface#15.1
- typography#2.2
- typography#2.3
- typography#2.6
- typography#2.8
- typography#3.4

## Cascade only

- surface#1.1
- surface#1.2
- typography#3.7
