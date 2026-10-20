group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «The former two-argument form `<task> <state-dir>` is gone.»
  source: .andromeda/layout-templates.md:74 «`snapshot` taking an optional `--part` (a whole number, 1 to 1000) and answering one uncoloured line on stdout, exit 0 — `{"text":…}` for a whole screen, `{"text":…,"part":n,"parts":m}`»
  replace-with: «The first instance verb, `snapshot [--part <part>] --session <dir>`, takes an optional `--part` (a whole number, 1 to 1000) and answers one uncoloured line on stdout, exit 0 — `{"text":…}` for a whole screen, `{"text":…,"part":n,"parts":m}` for a screen read in parts, from part 1 without `--part`, a part past the last the empty text with the same `parts`, a `--part` outside its bound refused `malformed`, exit 1; over the `mcp` role a paged `snapshot` answers its JSON line `{"text","part","parts"}` as one text block, a whole screen still the bare screen text. The former two-argument form `<task> <state-dir>` is gone.»
