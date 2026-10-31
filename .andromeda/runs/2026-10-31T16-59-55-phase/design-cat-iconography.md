# design · Iconography — category read

## Carried
- (.andromeda/design-system-detail/iconography.md:8) A pointer over the opened list must read its cursor by this one order, with no popup-only cursor rule: an option's label takes the CSS `cursor` keyword, else this order's Text over selectable text or Default, and a point outside every box still shows the Default arrow.
  > The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise — a pointer over no box included, so the shell keeps a visible arrow there
- (.andromeda/design-system.md:149) The broad statement of the same rule stands unchanged across the new overlay's paint and hit path, in every document and renderer the list opens in.
  > The cursor is the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise — a pointer over no box included, so the shell keeps a visible arrow there.

## Bindings
- **Cursor order ↔ Depth Strategy (hit testing)**: the cursor rule reads what the hit test answers, so a list painted past a clipping ancestor must answer the hit test wherever it shows, or the pointer over a visible option falls to the no-box Default arrow, (.andromeda/design-system-detail/iconography.md:8) and (.andromeda/design-system.md:85)
