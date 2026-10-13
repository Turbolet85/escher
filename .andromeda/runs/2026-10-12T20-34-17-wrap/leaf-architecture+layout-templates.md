group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «the `mcp` role lists it as its tenth tool»
  source: .andromeda/layout-templates.md:74 «lists the eleven verbs of the verb table as its tools in order, `help` last»
  replace-with: «the `mcp` role lists it as its eleventh tool, last, after `screenshot`»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «The former two-argument form `<task> <state-dir>` is gone.»
  source: .andromeda/layout-templates.md:74 «the seventh `screenshot` after `scroll` since 2026-10-12-headless-screenshot, taking an optional `--id` (the whole screen without it, with it the element the id names, cropped to the part of its bounds that can be seen)»
  replace-with: «Since 2026-10-12-headless-screenshot the seventh instance verb, `screenshot [--id <id>] --session <dir>`, answers one uncoloured line `{"width":…,"height":…,"png":"<base64>"}` on stdout, exit 0 — the whole screen, or with `--id` the element cropped to the part of its bounds that can be seen — an app with no font it can paint text with refused `fonts-unavailable` and an element shot that crops to no pixel `off-screen`, exit 1; over the `mcp` role an accepted shot is an `image` block (`image/png`, base64) then a text block, the line without `png`. The former two-argument form `<task> <state-dir>` is gone.»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «The twelve driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states}.rs` drive a held stand session through `escher_driver::Session::run`, by stable id.»
  source: .andromeda/architecture.md:263 «since 2026-10-12-headless-screenshot a thirteenth driver-action file, `stand_act_screenshot`»
  replace-with: «The thirteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot}.rs` drive a held stand session through `escher_driver::Session::run`, by stable id; since 2026-10-12-headless-screenshot `stand_screenshot_repeat` checks a shot of one state byte-identical in two child runs and in the parent, in both layout modes.»
