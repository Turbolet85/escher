group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «the image as base64 text inside the line; since 2026-10-19-visual-lint»
  source: .andromeda/design-system.md:398 «`escher-session snapshot [--part <part>]` answers one uncoloured line with no ESC byte, `{"text":…}` for a whole screen and `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`»
  replace-with: «the image as base64 text inside the line; `escher-session snapshot [--part <part>]` (a whole number, 1 to 1000, outside it refused `malformed`, status `1`) answers one uncoloured line with no ESC byte, `{"text":…}` for a whole screen and `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`; since 2026-10-19-visual-lint»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «a tool result's text the screen text for a `snapshot`»
  source: .andromeda/design-system.md:398 «a tool result's text the screen text for a `snapshot` (its JSON line `{"text","part","parts"}` for a screen read in parts)»
  replace-with: «a tool result's text the screen text for a `snapshot` (its JSON line `{"text","part","parts"}` for a screen read in parts)»
