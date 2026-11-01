# design · Border Radius — category read

## Carried
- (.andromeda/design-system-detail/border-radius.md:14) The take's resolution of the conflict predicted in blitz-dom's `assets/default.css` must keep the default button's radius at this value, or a take that moves it must re-measure and record the new value here.
  > | blitz-dom default button | 1px | (packages/blitz-dom/assets/default.css:108) |
- (.andromeda/design-system-detail/border-radius.md:22) Paint changes the take carries, the inline box decorations of `f76777766` among the candidates, must leave per-corner elliptical radii resolved from the computed values and scaled to device pixels.
  > - Per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1398-1415)

## Bindings
- **Default button radius and corner-radius handling ↔ layout-templates (the stand's screens)**: a stand screenshot or bounds reading the take moves is re-measured and recorded, and the stand's own card, button, input, task-card, tag and flight-type select radii are recorded in layout-templates, never in this category (.andromeda/design-system-detail/border-radius.md:12)
