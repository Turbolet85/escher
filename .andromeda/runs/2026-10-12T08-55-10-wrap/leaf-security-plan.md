group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 31 · «_This section is owned by `/andromeda-wrap-session`. setup-p»
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`VERBS`, nine verbs in two levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session, each with its argument and result shapes»
  source: .andromeda/security-plan.md:81 «`validate` checks a call against a closed `'static` table of ten verbs in three levels»
  replace-with: «`VERBS`, ten verbs in three levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session, `help` of the tool itself, each with its argument and result shapes»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «a returned node is keyed by the twelve `NODE_FIELDS`»
  source: .andromeda/security-plan.md:78 «thirteen since 2026-10-12-self-description added `differs` after `bounds` on a changed node only — a list of closed words»
  replace-with: «a returned node is keyed by the thirteen `NODE_FIELDS` (the thirteenth, `differs`, a changed node's list of closed words)»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05`, `tools/list` the nine verbs of `VERBS`»
  source: .andromeda/security-plan.md:83 «since 2026-10-12-self-description `initialize`'s revision matched against a closed table of four (`2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`), any other»
  replace-with: «handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05` (any other answered `2025-11-25`), `tools/list` the ten verbs of `VERBS`»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «`start` · `status` · `stop` answered by the server»
  source: .andromeda/security-plan.md:81 «answering `start`, `status`, `stop` and, since 2026-10-12-self-description, `help` itself after `validate_session`»
  replace-with: «`start` · `status` · `stop` · `help` answered by the server»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «an element's selected, pressed and invalid readings inside the text and the diff, as closed words and booleans»
  source: .andromeda/security-plan.md:78 «thirteen since 2026-10-12-self-description added `differs` after `bounds` on a changed node only — a list of closed words»
  replace-with: «an element's selected, pressed and invalid readings inside the text and the diff and a changed node's `differs`, as closed words and booleans»

- leaf: CLAUDE.md:38   class: omission
  leaf-text: «the driver's command line over the four — `start <task>`, the six instance verbs, `status`, `stop` —»
  source: .andromeda/security-plan.md:82 «since 2026-10-12-self-description `help` with no argument and at most one `--session <dir>`, the directory not read, answers `describe()`'s one line of JSON on stdout, status 0, reaching no session»
  replace-with: «the driver's command line over the four — `start <task>`, the six instance verbs, `status`, `stop`, and `help`, which reaches no session —»

- leaf: CLAUDE.md:100   class: omission
  leaf-text: «`advance --ms` · `scroll --id` · `status` · `stop`; one line of JSON on stdout»
  source: .andromeda/security-plan.md:82 «since 2026-10-12-self-description `help` with no argument and at most one `--session <dir>`, the directory not read, answers `describe()`'s one line of JSON on stdout, status 0, reaching no session»
  replace-with: «`advance --ms` · `scroll --id` · `status` · `stop` · `help` (its `--session` optional and unread); one line of JSON on stdout»

- leaf: CLAUDE.md:101   class: stale
  leaf-text: «its tools the nine verbs)»
  source: .andromeda/security-plan.md:203 «the driver's verbs as MCP tools — ten since 2026-10-12-self-description, the tenth, `help`, answered by the server itself»
  replace-with: «its tools the ten verbs)»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «one verb table in two levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance and `start` · `status` · `stop` of the session»
  source: .andromeda/security-plan.md:81 «`validate` checks a call against a closed `'static` table of ten verbs in three levels»
  replace-with: «one verb table in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session and `help` of the tool itself»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «the driver's nine verbs as MCP tools over stdio»
  source: .andromeda/security-plan.md:203 «the driver's verbs as MCP tools — ten since 2026-10-12-self-description, the tenth, `help`, answered by the server itself»
  replace-with: «the driver's ten verbs as MCP tools over stdio»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «newline-delimited JSON-RPC 2.0 in three handshake revisions»
  source: .andromeda/security-plan.md:83 «since 2026-10-12-self-description `initialize`'s revision matched against a closed table of four (`2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`), any other»
  replace-with: «newline-delimited JSON-RPC 2.0 in four handshake revisions, any other answered `2025-11-25`»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «and — still to come — the driver's self-description and the headless screenshot.»
  source: .andromeda/security-plan.md:81 «and, since 2026-10-12-self-description, the one asked of the tool itself, `help` (no argument; `Level::Tool`, its seven result fields fixed table text»
  replace-with: «the driver's self-description (built — `help`, the tenth verb, asked of the tool itself, answering the schema's fixed table text on the command line and as an MCP tool, reaching no session), and — still to come — the headless screenshot.»

- leaf: .claude/rules/security.md:22   class: stale
  leaf-text: «a session-level one (`start` · `status` · `stop`) by `validate_session` — before anything runs: a closed verb table of nine verbs in two levels»
  source: .andromeda/security-plan.md:81 «`validate` checks a call against a closed `'static` table of ten verbs in three levels»
  replace-with: «a session-level one (`start` · `status` · `stop`) or the tool-level `help` by `validate_session` — before anything runs: a closed verb table of ten verbs in three levels»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «inside the text and the diff three state readings (selected, pressed, invalid) cross as closed words and booleans»
  source: .andromeda/security-plan.md:78 «thirteen since 2026-10-12-self-description added `differs` after `bounds` on a changed node only — a list of closed words»
  replace-with: «inside the text and the diff three state readings (selected, pressed, invalid) and, since 2026-10-12-self-description, a changed node's `differs` cross as closed words and booleans»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «carries the same content to the MCP client that spawned it, over its own stdin and stdout»
  source: .andromeda/security-plan.md:83 «the program's own fixed `'static` text inside the ratified carrier, ratified by the founder (2026-10-12, «Внутри, с правилом», relayed verbatim by the overseer)»
  replace-with: «carries the same content — and, since 2026-10-12-self-description, `help`'s and the session errors' fixed table text, the program's own `'static` text inside the ratified carrier (ratified by the founder, 2026-10-12) — to the MCP client that spawned it, over its own stdin and stdout»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «checks a call against a closed table of nine verbs in two levels»
  source: .andromeda/security-plan.md:81 «`validate` checks a call against a closed `'static` table of ten verbs in three levels»
  replace-with: «checks a call against a closed table of ten verbs in three levels»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «which `validate` refuses as `unknown-verb` and `validate_session` checks»
  source: .andromeda/security-plan.md:81 «and, since 2026-10-12-self-description, the one asked of the tool itself, `help` (no argument; `Level::Tool`, its seven result fields fixed table text»
  replace-with: «and, since 2026-10-12-self-description, one asked of the tool itself, `help` (no argument; its seven result fields fixed table text), a further widening ratified by the founder (2026-10-12), which `validate` refuses as `unknown-verb` and `validate_session` checks»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «run by the MCP server through `Session::run` (session verbs through `validate_session`)»
  source: .andromeda/security-plan.md:81 «answering `start`, `status`, `stop` and, since 2026-10-12-self-description, `help` itself after `validate_session`»
  replace-with: «run by the MCP server through `Session::run` (session verbs and `help` through `validate_session`, answered by the server itself)»

- leaf: .claude/docs/security-summary.md:20   class: omission
  leaf-text: «which since 2026-10-10-snapshot-states also hold three state readings (selected, pressed, invalid) as closed words and booleans, never an attribute's value as typed»
  source: .andromeda/security-plan.md:78 «thirteen since 2026-10-12-self-description added `differs` after `bounds` on a changed node only — a list of closed words»
  replace-with: «which since 2026-10-10-snapshot-states also hold three state readings (selected, pressed, invalid) as closed words and booleans and since 2026-10-12-self-description a changed node's `differs`, a list of closed words, never an attribute's value as typed»

- leaf: .claude/docs/security-summary.md:21   class: omission
  leaf-text: «since 2026-10-10-mcp-surface `mcp` alone selects the MCP role»
  source: .andromeda/security-plan.md:82 «since 2026-10-12-self-description `help` with no argument and at most one `--session <dir>`, the directory not read, answers `describe()`'s one line of JSON on stdout, status 0, reaching no session»
  replace-with: «since 2026-10-12-self-description `help` with no argument and at most one `--session <dir>`, the directory not read, answers fixed table text as one line of JSON on stdout, status 0, reaching no session (`help x` refused `malformed`, status 1); since 2026-10-10-mcp-surface `mcp` alone selects the MCP role»

- leaf: .claude/docs/security-summary.md:21   class: omission
  leaf-text: «session error, 3 — none writing back anything the caller typed»
  source: .andromeda/security-plan.md:82 «the error object on stdout — carrying the error's fixed `remedy` beside `kind` and `message` since 2026-10-12-self-description —»
  replace-with: «session error, the error object with its fixed `remedy` beside `kind` and `message`, 3 — none writing back anything the caller typed»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «a method outside `initialize` · `ping` · `tools/list` · `tools/call` `-32601`, a notification running nothing»
  source: .andromeda/security-plan.md:83 «since 2026-10-12-self-description `initialize`'s revision matched against a closed table of four (`2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`), any other»
  replace-with: «a method outside `initialize` · `ping` · `tools/list` · `tools/call` `-32601`, `initialize`'s revision matched against a closed table of four (`2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`), any other answered `2025-11-25`, a notification running nothing»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «a result at most 1,048,576 bytes, carrying ids, names, masked values and the snapshot text by the crossing»
  source: .andromeda/security-plan.md:83 «the program's own fixed `'static` text inside the ratified carrier, ratified by the founder (2026-10-12, «Внутри, с правилом», relayed verbatim by the overseer)»
  replace-with: «a result at most 1,048,576 bytes, carrying ids, names, masked values and the snapshot text by the crossing, and since 2026-10-12-self-description the `help` tool's and the session errors' fixed table text, the program's own `'static` text inside the ratified carrier (ratified by the founder, 2026-10-12, «Внутри, с правилом»)»
