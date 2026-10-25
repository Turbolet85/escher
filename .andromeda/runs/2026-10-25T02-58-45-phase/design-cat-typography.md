# design · Typography — category read

## Carried
- (.andromeda/design-system.md:46) The statement's headless-screenshot capability must state this font limit as the body words it: a shot of an app with no `sans-serif` family is refused, never painted with its text skipped.
  > A driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped.
- (.andromeda/design-system-detail/typography.md:34) Any line the statement writes about the screenshot's font limit must match this detail: the reader is `BaseDocument::can_paint_text`, the refusal is `fonts-unavailable`, nothing is painted, and the stand's bundled font passes.
  > Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check
- (.andromeda/design-system-detail/typography.md:30) Where the statement says what the stand proves its capabilities on (and why a shot reads the same across processes on one host), it must describe the stand's font as it stands: one bundled DejaVu Sans for every generic, with system fonts off, on native.
  > The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature

## Bindings
- **Typography's `fonts-unavailable` refusal ↔ the body's Surface: cli**: the refusal the statement names for a font-less app reaches an agent as a refused call, which the command line reports with exit status `1`, and that status class has to match the statement's description of the carriers (.andromeda/design-system.md:46) ↔ (.andromeda/design-system.md:208).
- **Typography's stand font ↔ the statement's limit on shot bytes across OSes**: one bundled font does not make PNG bytes match across operating systems. The statement has to keep "byte-identical across processes on one host" separate from the cross-OS limit, and must not use the bundled font to claim more (.andromeda/design-system-detail/typography.md:30).
