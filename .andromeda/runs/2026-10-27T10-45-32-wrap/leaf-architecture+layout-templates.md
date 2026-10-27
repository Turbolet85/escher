group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:41   class: omission
  leaf-text: «exit 124 (still up when stopped) and `service.name=seven_guis` in the log.»
  source: .andromeda/architecture-detail/established-decisions.md:141 «held by the windowed `seven_guis_native` smoke boot and `focus_redraw` — an upstream engine change for every document.»
  replace-with: «exit 124 (still up when stopped) and `service.name=seven_guis` in the log; with `focus_redraw` it holds the windowed shell's redraw requests travelling through its event loop, an upstream engine change for every document.»
