group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «and a session error's object carries its fixed `remedy`; since 2026-10-10-mcp-surface»
  source: .andromeda/design-system.md:397 «`escher-session screenshot [--id]` answers one uncoloured line `{"width":…,"height":…,"png":"<base64>"}` on stdout with no ESC byte, status `0`»
  replace-with: «and a session error's object carries its fixed `remedy`; since 2026-10-12-headless-screenshot `escher-session screenshot [--id]` answers one uncoloured line `{"width":…,"height":…,"png":"<base64>"}` on stdout with no ESC byte, status `0`, the image as base64 text inside the line; since 2026-10-10-mcp-surface»

- leaf: .claude/docs/design-summary.md:28   class: stale
  leaf-text: «the self-description's line for `help`, the JSON line otherwise»
  source: .andromeda/design-system.md:397 «two content blocks for a `screenshot` — an `image` block (`mimeType` `image/png`, the PNG as base64) then a text block, the shot's line without `png` — and the JSON line otherwise»
  replace-with: «the self-description's line for `help`, two content blocks for a `screenshot` — an `image` block (`mimeType` `image/png`, the PNG as base64) then a text block, the shot's line without `png` — and the JSON line otherwise, every reply still one line»

- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «decoded by seven_guis' `woff` feature)»
  source: .andromeda/design-system.md:132 «a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted»
  replace-with: «decoded by seven_guis' `woff` feature); since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped — and the stand's bundled font passes that check»
