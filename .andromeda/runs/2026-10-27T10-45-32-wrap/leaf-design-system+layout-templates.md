group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:13   class: retired
  leaf-text: «held provisionally in layout-templates §Surface: desktop-native → Primary screens, not as a design-system value»
  source: .andromeda/design-system-detail/color-palette.md:27 «The stand's accent (its load page's `load-fetch` included), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native»
  replace-with: «held in layout-templates §Surface: desktop-native → Primary screens, not as a design-system value»

- leaf: .claude/docs/design-summary.md:14   class: stale
  leaf-text: «| seven_guis invalid |»
  source: .andromeda/layout-templates.md:17 «- seven_guis stand invalid state — `#e53e3e` / `#fff5f5` / `#c53030` (examples/seven_guis/src/tasks/flight_booker.rs:194-198; examples/seven_guis/src/tasks/flight_booker.rs:229-237)»
  replace-with: «| seven_guis invalid (the stand; held in layout-templates §Surface: desktop-native → Primary screens, not as a design-system value) |»
