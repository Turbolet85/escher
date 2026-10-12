group: layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`VERBS`, nine verbs in two levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session,»
  source: .andromeda/layout-templates.md:74 «over the ten verbs of the driver's verb table, in three levels — the six asked of the held instance»
  replace-with: «`VERBS`, ten verbs in three levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session, `help` of the tool itself,»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «and eight refusal `Cause`s each with a fixed remedy.»
  source: .andromeda/layout-templates.md:74 «`escher-session help` answers the tool's self-description — `escher_driver::describe()`, one uncoloured line of JSON with seven keys»
  replace-with: «and eight refusal `Cause`s each with a fixed remedy; `describe()` is the tool's self-description, one uncoloured line of JSON with seven keys (`verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`), every text a fixed string of the schema's tables, answered by `help` with no session reached.»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05`, `tools/list` the nine verbs of `VERBS`, `tools/call` run on the one session it holds in process — through `Session::run` for an instance verb, `start` · `status` · `stop` answered by the server —»
  source: .andromeda/layout-templates.md:74 «speaks the handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05` (answering `2025-11-25` to any other»
  replace-with: «handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05` (`2025-11-25` answered to any other), `tools/list` the ten verbs of `VERBS`, `help` last, `tools/call` run on the one session it holds in process — through `Session::run` for an instance verb, `start` · `status` · `stop` and `help` answered by the server —»

- leaf: CLAUDE.md:38   class: omission
  leaf-text: «is the driver's command line over the four — `start <task>`, the six instance verbs, `status`, `stop` —»
  source: .andromeda/layout-templates.md:74 «with `--session <dir>` exactly once on every command but `help`, which takes at most one and does not read it»
  replace-with: «is the driver's command line over the four — `start <task>`, the six instance verbs, `status`, `stop`, and `help`, the tool's self-description, which reads no `--session` —»

- leaf: CLAUDE.md:100   class: omission
  leaf-text: «· `scroll --id` · `status` · `stop`; one line of JSON on stdout»
  source: .andromeda/layout-templates.md:74 «`status` and `stop` of the session and, since 2026-10-12-self-description, `help` of the tool itself»
  replace-with: «· `scroll --id` · `status` · `stop`, and `help` (the tool's self-description, no `--session` needed); one line of JSON on stdout»

- leaf: CLAUDE.md:101   class: stale
  leaf-text: «its tools the nine verbs)»
  source: .andromeda/layout-templates.md:74 «lists the ten verbs of the verb table as its tools in order, `help` last»
  replace-with: «its tools the ten verbs, `help` last)»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «one verb table in two levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance and `start` · `status` · `stop` of the session,»
  source: .andromeda/layout-templates.md:74 «`status` and `stop` of the session and, since 2026-10-12-self-description, `help` of the tool itself»
  replace-with: «one verb table in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session and `help` of the tool itself,»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «the driver's nine verbs as MCP tools over stdio»
  source: .andromeda/layout-templates.md:74 «lists the ten verbs of the verb table as its tools in order, `help` last»
  replace-with: «the driver's ten verbs as MCP tools over stdio»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «newline-delimited JSON-RPC 2.0 in three handshake revisions»
  source: .andromeda/layout-templates.md:74 «speaks the handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05` (answering `2025-11-25` to any other»
  replace-with: «newline-delimited JSON-RPC 2.0 in four handshake revisions, `2025-11-25` answered to any other since 2026-10-12-self-description»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «, and — still to come — the driver's self-description and the headless screenshot.»
  source: .andromeda/layout-templates.md:74 «`escher-session help` answers the tool's self-description — `escher_driver::describe()`, one uncoloured line of JSON with seven keys»
  replace-with: «, the driver's self-description (built — `escher_driver::describe()`, one uncoloured line of JSON with seven keys, every text a fixed string of the schema's tables, answered by `escher-session help` and by the MCP `help` tool alike, reaching no session), and — still to come — the headless screenshot.»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «and `escher-session <verb> [arguments] --session <dir>` — escher's driver command line, built at 2026-10-10-driver-cli:»
  source: .andromeda/layout-templates.md:74 «with `--session <dir>` exactly once on every command but `help`, which takes at most one and does not read it»
  replace-with: «and `escher-session <verb> [arguments] --session <dir>` — escher's driver command line, built at 2026-10-10-driver-cli, its `help` (since 2026-10-12-self-description) reading no `--session` and answering the tool's self-description:»

- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «and the six task inputs are named by `for` / `aria-label` attributes, adding no element, class, style, wrapper or order.»
  source: .andromeda/layout-templates.md:10 «two lean tasks also state a role and a state by attributes alone — the flight booker's mode buttons `flight-one-way` and `flight-return` are toggle buttons stating `aria-pressed`»
  replace-with: «and the six task inputs are named by `for` / `aria-label` attributes, and the flight booker and CRUD state `aria-pressed` · `aria-invalid` · `role="listbox"`/`role="option"` with `aria-selected` by attributes alone, adding no element, class, style, wrapper or order.»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «every other command is one client run that answers one line of JSON on stdout (exit `0` accepted · `1` refused · `2` usage · `3` session error).»
  source: .andromeda/layout-templates.md:74 «with `--session <dir>` exactly once on every command but `help`, which takes at most one and does not read it»
  replace-with: «every other command is one client run that answers one line of JSON on stdout (exit `0` accepted · `1` refused · `2` usage · `3` session error); `help` (since 2026-10-12-self-description) answers the tool's self-description, reaching no session and reading no `--session`, and the `mcp` role lists it as its tenth tool.»
