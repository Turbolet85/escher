# layouts · standing conventions

- S1 · each new headless stand check runs at the stand's pinned viewport and reads a held instance as a fresh boot · surface-desktop-native
- S2 · each new stand control or fixture control is keyed, carries its id as an attribute alone, and names its role and state by attributes alone · surface-desktop-native
  decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
- S3 · each new stand page or fixture lints clean at boot · surface-desktop-native
  decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD, the load page and the order page read zero visual-lint findings, in both layout modes. · guard: stand_lint
