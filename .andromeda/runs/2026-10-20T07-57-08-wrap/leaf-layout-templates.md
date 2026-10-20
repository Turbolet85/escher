group: layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:27   class: stale
  leaf-text: «the in-process visual lint (`Session::lint`, escher-driver; in process only) — at boot each of the four lean tasks reads 0 findings in both layout modes (»
  source: .andromeda/layout-templates.md:46 «the visual lint (`Session::lint`, escher-driver, in process and as the `lint` verb on every carrier) since 2026-10-19-visual-lint: at boot each of the four lean tasks reads 0 findings»
  replace-with: «the visual lint (`Session::lint`, escher-driver, in process and as the `lint` verb on every carrier) — at boot each of the four lean tasks reads 0 findings in both layout modes, through the verb as through the reader (»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «the image as base64 text inside the line;»
  source: .andromeda/layout-templates.md:74 «the eighth `lint`, no argument (one named refused `malformed`), answering one uncoloured line `{"findings":[…]}` on stdout, exit 0, each finding keyed `id` · `defect` · `bounds` then its reason's keys»
  replace-with: «the image as base64 text inside the line; since 2026-10-19-visual-lint `escher-session lint`, the eighth instance verb of twelve, takes no argument (one named refused `malformed`) and answers one uncoloured line `{"findings":[…]}` on stdout with no ESC byte, status `0`, each finding keyed `id` · `defect` · `bounds` then its reason's keys — ids, closed words and numbers only — an app with no font it can paint text with refused `fonts-unavailable`, status `1`, and a `lint` step kept under `--record` like every instance verb; over MCP an accepted `lint` answers its JSON line of findings as one text block, the tool listed after `screenshot`;»
