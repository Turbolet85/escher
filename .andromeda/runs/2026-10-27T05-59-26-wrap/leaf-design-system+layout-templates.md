group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:13   class: omission
  leaf-text: «| seven_guis (the stand) |»
  source: .andromeda/design-system-detail/color-palette.md:27 «The stand's accent (its load page's `load-fetch` included), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens»
  replace-with: «| seven_guis (the stand; held provisionally in layout-templates §Surface: desktop-native → Primary screens, not as a design-system value) |»

- leaf: .claude/docs/design-summary.md:15   class: omission
  leaf-text: «| seven_guis success |»
  source: .andromeda/layout-templates.md:16 «the flight booker's success message is `#ebf8ee` / `#68d391` / `#276749`»
  replace-with: «| seven_guis success (the stand; held in layout-templates §Surface: desktop-native → Primary screens, not as a design-system value) |»

- leaf: .claude/docs/design-summary.md:24   class: retired
  leaf-text: «WASM builds bundle DejaVu Sans for every generic family, and so does the native headless stand (system fonts off, decoded by seven_guis' `woff` feature)»
  source: .andromeda/design-system-detail/typography.md:31 «The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value»
  replace-with: «WASM builds bundle DejaVu Sans for every generic family (the native headless stand's font registration is held in architecture §Standard Contracts → Headless stand, not as a design-system typography value)»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «nothing written there at its end»
  source: .andromeda/layout-templates.md:88 «its end writes nothing on stdout — exit 0 on `Ok`, exit 3 with the error's fixed message on stderr on `Err`»
  replace-with: «nothing written there at its end — exit `0` on `Ok`, exit `3` with the error's fixed message on stderr on `Err`»
