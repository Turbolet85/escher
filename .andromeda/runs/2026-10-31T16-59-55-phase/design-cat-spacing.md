# design · Spacing — category read

## Carried
- (.andromeda/design-system-detail/spacing.md:21) A `multiple` select's rows must keep this padding while the popup is added, and the single select's opened list must take its row spacing from a rule of its own, not by widening these selectors.
  > blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
- (.andromeda/design-system-detail/spacing.md:29) An opened list that scrolls its options must draw its scrollbar at this thumb geometry, with no second scrollbar geometry for the popup.
  > Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
- (.andromeda/design-system.md:70) The popup's row and list spacing must be stated as new engine default values, since no project scale step exists for it to cite, and the chunk must not claim one.
  > NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.

## Bindings
- **Popup scrollbar geometry ↔ Depth Strategy**: an opened list's scrollbar margin and thumb must stay inside what hit testing answers for a clipping node, the list being a new overlay in hit testing; (.andromeda/design-system-detail/spacing.md:29) with (.andromeda/design-system.md:85)
- **Stand padding ↔ layout-templates**: any padding the stand's `select#flight-type` page needs around its popup is held in layout-templates and must not enter this file as a design-system value; (.andromeda/design-system-detail/spacing.md:19)
- **Popup row spacing ↔ a11y-plan**: the new row padding must be read against the a11y-plan's minimum sizes and focus rings, this category's relation to that plan; (.andromeda/design-system-detail/spacing.md:21)
