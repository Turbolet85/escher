group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/escher-driver.md:6   class: stale
  leaf-text: «since 2026-10-10-mcp-surface the **MCP server** that serves the nine verbs as MCP tools over a caller's reader and writer (the `escher-session mcp` role)»
  source: .andromeda/architecture.md:133 «`tools/list` answers the ten verbs of `VERBS` in order, `help` last»
  replace-with: «since 2026-10-10-mcp-surface the **MCP server** that serves the ten verbs as MCP tools over a caller's reader and writer (the `escher-session mcp` role), and since 2026-10-12-self-description the **self-description** (`describe`) that `help` answers on both surfaces»

- leaf: .claude/docs/services/escher-driver.md:20   class: stale
  leaf-text: «a node is keyed by `NODE_FIELDS` in order — twelve words: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` — with»
  source: .andromeda/architecture.md:133 «each keyed by the thirteen keys `NODE_FIELDS` names: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` · `differs`»
  replace-with: «a node is keyed by `NODE_FIELDS` in order — thirteen keys: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` · `differs` (since 2026-10-12-self-description; written on a changed node only, after `bounds`: the node-key words whose reading differs from before the step, a node that only moved reading `bounds` alone, an added node written without it) — with»

- leaf: .claude/docs/services/escher-driver.md:20   class: stale
  leaf-text: «a session error `{"error":{"kind","message"}}`»
  source: .andromeda/architecture.md:133 «`SessionError::to_json` — `{"error":{"kind","message","remedy"}}`, the remedy since 2026-10-12-self-description»
  replace-with: «a session error `{"error":{"kind","message","remedy"}}` (the remedy since 2026-10-12-self-description)»

- leaf: .claude/docs/services/escher-driver.md:23   class: stale
  leaf-text: «`kind()` is one fixed kebab-case word per variant. A session error carries no remedy, and `state-dir-too-long` states no bound.»
  source: .andromeda/architecture.md:133 «`remedy() -> &'static str`, and `to_json()`; since that chunk each variant's kind, meaning and remedy are one row of a crate-private table, `SESSION_ERRORS: [ErrorSpec; 13]`»
  replace-with: «`kind()` is one fixed kebab-case word per variant and, since 2026-10-12-self-description, `remedy()` a fixed remedy: each variant's kind, meaning and remedy are one row of the crate-private `SESSION_ERRORS: [ErrorSpec; 13]` (`ErrorSpec { kind, meaning, remedy, repeats }`, `repeats` not served), which `kind()`, `remedy()` and `Display` read; `dead` repeats `no-session`'s remedy and `answer-too-large` `not-settled`'s; `StateDirTooLong`'s message states its bound as a figure — at most 94 bytes as given on Linux, 90 on every other target — and its remedy is a shorter path for the state directory, or a relative one.»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «the verb table, nine verbs in two levels (`Level::Instance` · `Level::Session`)»
  source: .andromeda/architecture.md:133 «`VERBS` lists ten verbs in three levels (`Level` is `Instance` · `Session` · `Tool`, its `name()` the words `instance` · `session` · `tool`; the third level since 2026-10-12-self-description»
  replace-with: «the verb table, ten verbs in three levels (`Level::Instance` · `Level::Session` · `Level::Tool`, the third since 2026-10-12-self-description)»

- leaf: .claude/docs/services/escher-driver.md:24   class: omission
  leaf-text: «then `start` (`app`), `status` and `stop` (session-level; ratified by the founder, 2026-10-10);»
  source: .andromeda/architecture.md:133 «`help` (no argument; result `verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`, each field with its help»
  replace-with: «then `start` (`app`), `status` and `stop` (session-level; ratified by the founder, 2026-10-10), then `help` (tool-level, no argument; result `verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`, each field with its help; since 2026-10-12-self-description, ratified by the founder, 2026-10-12);»

- leaf: .claude/docs/services/escher-driver.md:24   class: omission
  leaf-text: «so an agent learns them from the tool; the crate docs say the same in one sentence.»
  source: .andromeda/architecture.md:133 «since that chunk the constant is `[NodeFieldSpec; 13]`, each row `NodeFieldSpec { name, help }` whose help states the key's reading»
  replace-with: «so an agent learns them from the tool; since 2026-10-12-self-description `NODE_FIELDS` is `[NodeFieldSpec; 13]`, each row `NodeFieldSpec { name, help }` whose help states the key's reading — the two limits among them — and the crate docs' one sentence says each limit is stated in its node key's help and that a changed node names the keys whose reading differs (`differs`).»

- leaf: .claude/docs/services/escher-driver.md:24   class: omission
  leaf-text: «`scroll` adds `in_view` (always present).»
  source: .andromeda/architecture.md:133 «`describe() -> String` (since 2026-10-12-self-description) writes the tool's self-description, the answer of `help`, as one line of JSON through the crate's `json` writer»
  replace-with: «`scroll` adds `in_view` (always present); `help`'s fields are of the kinds `Words` · `Entries` (nine `FieldKind`s). Since 2026-10-12-self-description the command line's four exit statuses are the schema's `EXIT_STATUSES: [ExitStatus; 4]` (`ExitStatus { status, name, meaning }`: `0` `accepted` · `1` `refused` · `2` `usage` · `3` `session-error`), which the command line's status constants read and `help` serves, and `describe() -> String` writes the tool's self-description, `help`'s answer, as one line of JSON from the tables alone — 10 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes and 13 session errors with their remedies, 4 exit statuses — reading no environment and printing nothing; the command line's `help` and the MCP `help` tool serve the same bytes.»

- leaf: .claude/docs/services/escher-driver.md:25   class: stale
  leaf-text: «the verb by exact name (`unknown-verb`; a session-level verb too)»
  source: .andromeda/architecture.md:133 «so does a session-level verb or, since 2026-10-12-self-description, the tool-level `help`, refused before its arguments are read»
  replace-with: «the verb by exact name (`unknown-verb`; a session-level verb and, since 2026-10-12-self-description, the tool-level `help` too)»

- leaf: .claude/docs/services/escher-driver.md:25   class: stale
  leaf-text: «is the same check over the three session-level rows (`SessionCommand`: `Start { app }` · `Status` · `Stop`).»
  source: .andromeda/architecture.md:133 «with `SessionCommand` exactly `Start { app }` · `Status` · `Stop` · `Help` and a `spec()` of its own (`Help`'s is `HELP`)»
  replace-with: «is the same check over the three session-level rows and, since 2026-10-12-self-description, the tool-level `help` — an instance-level verb `unknown-verb` there, an argument to `help` `malformed` with `Fault::Unnamed` (`SessionCommand`: `Start { app }` · `Status` · `Stop` · `Help`, `Help`'s spec `HELP`).»

- leaf: .claude/docs/services/escher-driver.md:27   class: stale
  leaf-text: «newline-delimited JSON-RPC 2.0 in the handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05` on the `BufRead`»
  source: .andromeda/architecture.md:133 «in the handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05` (`PROTOCOL_VERSIONS`; the first since 2026-10-12-self-description»
  replace-with: «newline-delimited JSON-RPC 2.0 in the handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05` (`PROTOCOL_VERSIONS`; the first since 2026-10-12-self-description) on the `BufRead`»

- leaf: .claude/docs/services/escher-driver.md:27   class: stale
  leaf-text: «`initialize` answers the asked revision when it is one of the three, else `2025-06-18` (»
  source: .andromeda/architecture.md:133 «`initialize` answers the asked revision when it is one of the four, else `LATEST_VERSION`, `2025-11-25` — until 2026-10-12-self-description one of three, else `2025-06-18`»
  replace-with: «`initialize` answers the asked revision when it is one of the four, else `LATEST_VERSION`, `2025-11-25` (until 2026-10-12-self-description one of three, else `2025-06-18`; the current revision `2026-07-28` is answered so and not taken up, the founder's ruling of 2026-10-12) (»

- leaf: .claude/docs/services/escher-driver.md:27   class: stale
  leaf-text: «`tools/list` → the nine verbs of `VERBS` in order, each with an `inputSchema`»
  source: .andromeda/architecture.md:133 «each a tool named by its verb with a `description` — its help and, since 2026-10-12-self-description, then `. Returns: ` and each result field as `{name} ({kind}): {help}` joined by `; `»
  replace-with: «`tools/list` → the ten verbs of `VERBS` in order, `help` last, each with a `description` — its help, then, since 2026-10-12-self-description, `. Returns: ` and each result field as `{name} ({kind}): {help}` joined by `; ` (the listing 2,984 → 8,844 bytes) — and an `inputSchema`»

- leaf: .claude/docs/services/escher-driver.md:27   class: omission
  leaf-text: «`status` and `stop` are answered by the server.»
  source: .andromeda/architecture.md:133 «`help` held session or not, its text block `describe()`'s line, accepted, and counted in a held session's `served` as every tool call is»
  replace-with: «`status`, `stop` and, since 2026-10-12-self-description, `help` are answered by the server — `help` held session or not, its text `describe()`'s line, accepted, and counted in a held session's `served` as every tool call is.»

- leaf: .claude/docs/services/escher-driver.md:28   class: omission
  leaf-text: «`mcp` alone, else usage (since 2026-10-10-mcp-surface);»
  source: .andromeda/architecture.md:133 «since 2026-10-12-self-description `help` with no argument and at most one `--session <dir>`, which is not read, answers `describe()`'s line on stdout, status `0`, reaching no session»
  replace-with: «`mcp` alone, else usage (since 2026-10-10-mcp-surface); `help` with no argument and at most one `--session <dir>`, which is not read, answers `describe()`'s line on stdout, status `0`, reaching no session — `help` with an argument is refused `malformed`, `1`, and `--session` with no directory or given twice is usage, `2` (since 2026-10-12-self-description);»

- leaf: .claude/docs/services/escher-driver.md:32   class: omission
  leaf-text: «the next surface that carries any of it is a new crossing question.»
  source: .andromeda/architecture.md:101 «since 2026-10-12-self-description the command line's stdout and the `mcp` role's stdio also carry the program's own fixed text, held as `'static` data of the one table»
  replace-with: «the next surface that carries any of it is a new crossing question. Since 2026-10-12-self-description the command line's stdout and the `mcp` role's stdio also carry the program's own fixed text, held as `'static` data of the one table — the `help` self-description, a session error's remedy and each tool's description with its result fields — riding those ratified carriers, and a changed node's `differs` crosses as a closed set by the rule of 2026-10-10 (ratified by the founder, 2026-10-12); text from a page, a call or a screen, a new carrier or a new exit still widens.»

- leaf: .claude/docs/services/escher-driver.md:42   class: stale
  leaf-text: «an over-long one is `StateDirTooLong` (status `3`), and the message states no bound.»
  source: .andromeda/architecture.md:133 «`StateDirTooLong`'s message states its bound as a figure, at most 94 bytes as given on Linux and 90 on every other target»
  replace-with: «an over-long one is `StateDirTooLong` (status `3`), whose message states the bound — at most 94 bytes as given on Linux, 90 on every other target — and whose remedy is a shorter path, or a relative one, which counts only its own bytes.»

- leaf: .claude/docs/services/escher-driver.md:49   class: stale
  leaf-text: «a session-level verb's too, beside `unknown-verb`»
  source: .andromeda/architecture.md:133 «a session-level verb or, since 2026-10-12-self-description, the tool-level `help` handed to `run` is refused `unknown-verb` with its table word recorded»
  replace-with: «a session-level verb's and, since 2026-10-12-self-description, the tool-level `help`'s too, beside `unknown-verb`»

- leaf: .claude/docs/services/escher-driver.md:50   class: stale
  leaf-text: «while `start` · `status` · `stop` leave a `tool_call` with no child»
  source: .andromeda/architecture.md:133 «and a call the server answers itself — `help` among them — has no `command` child»
  replace-with: «while `start` · `status` · `stop` and, since 2026-10-12-self-description, `help` leave a `tool_call` with no child»

- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «and in process `stand_mcp_tools` (blitz-tests; the flow through `serve_mcp` in both layout modes, one `tool_call` span the parent of each `command`).»
  source: .andromeda/architecture.md:263 «since 2026-10-12-self-description every cause a tool call reaches served with its remedy, `covered` on an overlay fixture, and `help` answered by the server, parenting no `command` span»
  replace-with: «and in process `stand_mcp_tools` (blitz-tests; the flow through `serve_mcp` in both layout modes, one `tool_call` span the parent of each `command`; since 2026-10-12-self-description every cause a tool call reaches served with its remedy, `covered` on an overlay fixture, and `help` answered by the server, parenting no `command` span) and, since 2026-10-12-self-description, `stand_mcp_masked` (a password typed through an MCP `type` reads `MASKED_VALUE` and never the typed text, role `PasswordInput` with a non-empty name, in both layout modes; it declares no `mod session_common;`).»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «every other command is one client run that answers one line of JSON on stdout (exit `0` accepted · `1` refused · `2` usage · `3` session error).»
  source: .andromeda/architecture.md:138 «each with `--session <dir>` exactly once but `help`, which needs none, admits at most one, reads it not, and answers the self-description's one line with status `0`, reaching no session»
  replace-with: «every other command is one client run that answers one line of JSON on stdout (exit `0` accepted · `1` refused · `2` usage · `3` session error — the four read from `EXIT_STATUSES` and served by `help`), save `help` (since 2026-10-12-self-description), which needs no `--session` — it admits at most one and reads it not — and answers the self-description's one line with status `0`, reaching no session.»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «The driver-action checks `stand_act_{ids,diff,timer,refused,keys,range}.rs` drive a held stand session through `escher_driver::Session::run`, by stable id.»
  source: .andromeda/architecture.md:263 «the driver's calls run on a held stand session through `Session::run` — the twelve files `stand_act_ids`, `stand_act_diff`, `stand_act_timer`, `stand_act_refused`, `stand_act_keys`, `stand_act_range`,»
  replace-with: «The twelve driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states}.rs` drive a held stand session through `escher_driver::Session::run`, by stable id.»

- leaf: .claude/docs/services/escher-telemetry.md:33   class: omission
  leaf-text: «and before that in process, 0 occurrences in a sink capture of the driver's commands (`stand_act_spans`).»
  source: .andromeda/architecture.md:150 «with 0 id needles, 0 name needles and 0 occurrences of a typed sentinel at `trace` under both builds (`mcp_log`)»
  replace-with: «and before that in process, 0 occurrences in a sink capture of the driver's commands (`stand_act_spans`); since 2026-10-10-mcp-surface the `escher-session mcp` role's stderr is measured too — one `tool_call` line per tool call and a `command` line per instance call, with 0 id needles, 0 name needles and 0 occurrences of a typed sentinel at `trace` under both builds (`mcp_log`).»
