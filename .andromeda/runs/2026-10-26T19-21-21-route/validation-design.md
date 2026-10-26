# Design validation — route draft

## Rewrite
- `Bundled fonts`: "a run's fonts bundled; two hosts read one state the same way" → "a run's fonts bundled — the stand's DejaVu Sans pin held for every generic; `fonts-unavailable` refusal kept; two hosts read one state alike"
  Reason: Per design-system §Typography → Loading (detail `.andromeda/design-system-detail/typography.md`), the stand already registers bundled DejaVu Sans with system fonts off and screenshot/lint refuse `fonts-unavailable`, so this chunk has to extend that face rather than add a second one. The draft never opened §Typography (`plans-not-opened.md` row `design-system.md`).
- `Engine debts`: "select-option overlap" → "select-option overlap — the flight booker's one lint `overlap` re-read against its measured stand reading"
  Reason: Per layout-templates §Surface: desktop-native → IA notes (visual lint), the stand's boot layout reading has the flight booker at one `overlap` that is a select-options artefact rather than a layout defect, so fixing it changes the stand's measured layout reading.
- `Stand requirement sweep`: "coverage floors held, new verbs' stderr id-free" → "coverage floors held, new verbs' stderr id-free; stand layout held — TaskShell mount, pinned viewport, lint findings as measured, no stand markup/id/class/style moved"
  Reason: Per layout-templates §Surface: desktop-native → Primary screens and IA notes, every earlier chunk kept the stand's mount, pinned 800×600 viewport and lint readings unchanged, and 0.3.0's engine debts (transformed scrolling box, select overlap) plus the new state and isolation boots touch that layout with no other chunk re-reading it.
- `A component in isolation`: "one component rendered alone with given props" → "one component rendered alone with given props, at the stand's pinned viewport and bundled font"
  Reason: Per layout-templates §Surface: desktop-native (stand at 800×600, scale 1.0, Light, bundled DejaVu Sans) and design-system §Typography → Loading (without a resolvable font, text measures 0×0 or the shot is refused), an isolated render needs the stand's layout context so its readings match the in-app reading.
