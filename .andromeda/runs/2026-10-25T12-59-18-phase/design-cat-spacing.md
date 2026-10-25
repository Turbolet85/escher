# design · Spacing — category read

## Carried
- (.andromeda/design-system-detail/spacing.md:19) The flight booker's one-way / return choice rebuilt as a `select`, and any stand screen this chunk adds or serves (Cells), must sit inside the stand's recorded card spacing — the card's 24px 32px padding and 16px gap and the home's padding — so the new control introduces no untokenized spacing value of its own and the stand's lint (`overlap` · `clipped`) keeps reading 0 findings on the lean tasks at boot.
  > - seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:38-42; examples/seven_guis/src/app.rs:191)
- (.andromeda/design-system.md:72) No project-wide spacing scale stands, so the stand's own card values above are the only reference the changed flight booker and a served Cells screen must match; this chunk must not invent a scale or tokens while extending the stand.
  > NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.

## Bindings
- **seven_guis card spacing ↔ Color Palette (body)**: the select placed in the flight booker card sits in the card whose controls carry the stand accent, and every control text/background pair in it must keep SC 1.4.3 as `stand_contrast` records, (.andromeda/design-system-detail/spacing.md:19) ↔ (.andromeda/design-system.md:16)
- **seven_guis card spacing ↔ a11y-plan (contrast and focus order)**: a select moved into the card's layout moves the stand's Tab order and contrast rows, which a11y-plan owns; spacing binds only the placement, (.andromeda/design-system-detail/spacing.md:19)
