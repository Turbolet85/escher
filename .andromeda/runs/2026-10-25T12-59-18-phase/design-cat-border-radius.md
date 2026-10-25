# design · Border Radius — category read

## Carried
- (.andromeda/design-system-detail/border-radius.md:12) The flight booker's one-way / return choice rebuilt as a real `select`, and any control the stand gains to serve `cells` or a load in flight, must take the stand's own untokenized radii — an input-class control at the inputs' 4px, a button at 6px, a card at 8px — so the extended stand stays one visual family with the tasks it joins.
  > | seven_guis (untokenized) | cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px | (examples/seven_guis/src/tasks/counter.rs:41; examples/seven_guis/src/tasks/counter.rs:61; examples/seven_guis/src/tasks/temp_converter.rs:71; examples/seven_guis/src/app.rs:228; examples/seven_guis/src/app.rs:269) |
- (.andromeda/design-system.md:110) No radius token or scale exists to cite, so the chunk's new or changed stand controls must reuse the stand's literal values above and must not introduce a new radius value or a token set of their own — a radius scale is a design-system decision, not this chunk's.
  > NOT YET MEASURED — a project-wide radius scale: the examples, test and WPT slices (s01, s12, s13) recorded radius tokens as out of slice.

## Bindings
- **seven_guis radii ↔ the body's Border Radius absence**: the stand's untokenized row is the only radius reference the chunk's stand controls can match, because the body records no project-wide radius scale and no radius personality (.andromeda/design-system-detail/border-radius.md:12) (.andromeda/design-system.md:110)
- **seven_guis radii ↔ Color Palette (the stand accent)**: a select or button restyled for the flight booker keeps its radius from the stand row while its colours stay on the stand accent `#3a5ce5` and its states, whose contrast `stand_contrast` re-pins (.andromeda/design-system-detail/border-radius.md:12) (.andromeda/design-system.md:16)
