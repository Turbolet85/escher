group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «The fourteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot,transformed}.rs`»
  source: .andromeda/architecture.md:359 «a fifteenth, `stand_act_select`, a select driven by id — at boot its node, `ComboBox` with its name and value and its options as its children»
  replace-with: «The fifteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot,transformed,select}.rs`»

- leaf: .claude/docs/services/seven_guis.md:38   class: retired
  leaf-text: «(escher-driver; PROVISIONAL: in process only)»
  source: .andromeda/layout-templates.md:46 «The stand's layout reads clean to the in-process visual lint (`Session::lint`, escher-driver; in process only) since 2026-10-19-visual-lint»
  replace-with: «(escher-driver; in process only)»
