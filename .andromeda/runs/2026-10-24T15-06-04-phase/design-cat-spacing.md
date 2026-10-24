# design · Spacing — category read

## Carried
- (.andromeda/design-system-detail/spacing.md:20) Any fixture page the sweep adds (C1's `truncated`, `clipped`, `off-screen` and `text_extent` readings, both layout modes) is laid out under the engine's default body margin, so every bound or extent a fixture check asserts must account for it or the page must set its own body margin explicitly.
  > - blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:274-277)
- (.andromeda/design-system-detail/spacing.md:19) The sweep re-proves the stand's lint readings (each lean task 0 findings at boot, the last CRUD row `clipped` inside `crud-list`) on the stand's own geometry; the stand's card and home padding stay as recorded — the sweep changes no stand spacing, and any stand spacing change would move those readings.
  > - seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:38-42; examples/seven_guis/src/app.rs:191)
- (.andromeda/design-system.md:72) No project spacing scale exists, so a new fixture page's spacing literals cite no token and the "uses only design tokens" criterion has no spacing step to bind; a fixture states its pixel values directly.
  > > NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.

## Bindings
- **default body margin ↔ Depth Strategy (body)**: a fixture asserting a `clipped` or `off-screen` reading places its clipping box under the 8px body margin (.andromeda/design-system-detail/spacing.md:20), and the clip it reads stops at that box's padding box (.andromeda/design-system.md:87).
- **stand card padding ↔ a11y-plan / the stand's lint readings**: the stand's recorded padding (.andromeda/design-system-detail/spacing.md:19) is the geometry the re-proved 0-finding and CRUD `clipped` readings rest on; the sweep holds it unchanged.
