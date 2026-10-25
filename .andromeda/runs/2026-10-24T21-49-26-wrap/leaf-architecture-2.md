group: architecture-2
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «The fifteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot,transformed,select}.rs` drive a held stand session through `escher_driver::Session::run`, by stable id;»
  source: .andromeda/architecture-detail/existing-scopes.md:111 «5 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:79-256), run in both the `a11y` CI leg and the `stand-repeat` leg (by its `stand_` prefix)»
  replace-with: «The fifteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot,transformed,select}.rs` drive a held stand session through `escher_driver::Session::run`, by stable id, `stand_act_select` also run in the `a11y` CI leg (one of its fourteen named files);»
