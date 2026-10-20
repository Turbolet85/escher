group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «the `mcp` role lists it as its eleventh tool, last, after `screenshot`»
  source: .andromeda/layout-templates.md:74 «lists the twelve verbs of the verb table as its tools in order, `lint` after `screenshot`, `help` last»
  replace-with: «the `mcp` role lists it as its twelfth tool, last, after `lint`»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «over the `mcp` role an accepted shot is an `image` block (`image/png`, base64) then a text block, the line without `png`.»
  source: .andromeda/layout-templates.md:74 «the eighth `lint`, no argument (one named refused `malformed`), answering one uncoloured line `{"findings":[…]}` on stdout, exit 0, each finding keyed `id` · `defect` · `bounds` then its reason's keys»
  replace-with: «over the `mcp` role an accepted shot is an `image` block (`image/png`, base64) then a text block, the line without `png`. The eighth instance verb, `lint --session <dir>`, takes no argument (one named refused `malformed`) and answers one uncoloured line `{"findings":[…]}` on stdout, exit 0, each finding keyed `id` · `defect` · `bounds` then its reason's keys — ids, closed words and numbers only — an app with no font it can paint text with refused `fonts-unavailable`, exit 1, a `lint` step kept under `--record` like every instance verb; over the `mcp` role an accepted `lint` answers its JSON line of findings as one text block.»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «reads the stand through the in-process visual lint `Session::lint` (escher-driver; in process only): at boot each of the four lean tasks reads 0 findings,»
  source: .andromeda/layout-templates.md:46 «The stand's layout reads clean to the visual lint (`Session::lint`, escher-driver, in process and as the `lint` verb on every carrier) since 2026-10-19-visual-lint: at boot each of the four lean tasks reads 0»
  replace-with: «reads the stand through the visual lint (`Session::lint`, escher-driver, in process and as the `lint` verb on every carrier): at boot each of the four lean tasks reads 0 findings in both layout modes, through the verb as through the reader,»
