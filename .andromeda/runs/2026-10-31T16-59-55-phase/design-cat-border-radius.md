# design · Border Radius — category read

## Carried
- (.andromeda/design-system-detail/border-radius.md:12) Any radius the stand's flight-type select or its popup row takes must be held in layout-templates, and must not be entered as a design-system value.
  > The stand's card, button, input, task-card, tag and flight-type select radii are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values
- (.andromeda/design-system-detail/border-radius.md:22) A corner radius on the opened list must be resolved from its computed border radii and scaled to device pixels, on every renderer, not painted as a fixed logical value.
  > Per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels
- (.andromeda/design-system.md:108) The opened list is a new rectangular container with no project radius scale to cite, so any radius it takes must be recorded as an untokenized engine value and must not claim a token or a scale step.
  > NOT YET MEASURED — a project-wide radius scale: the examples, test and WPT slices (s01, s12, s13) recorded radius tokens as out of slice.

## Bindings
- **Flight-type select radii ↔ layout-templates**: the radius value is held in layout-templates §Surface: desktop-native → Primary screens, and this category holds only the pointer to it (.andromeda/design-system-detail/border-radius.md:12)
- **The list's painted corners ↔ the body's radius gap**: the corners must follow the computed, device-scaled path, while the value cites no token because the body records no scale (.andromeda/design-system-detail/border-radius.md:22) (.andromeda/design-system.md:108)
