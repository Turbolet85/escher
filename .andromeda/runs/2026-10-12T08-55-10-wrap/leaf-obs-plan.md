group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «a session-level verb handed to `run` records its table word beside `unknown-verb`»
  source: .andromeda/obs-plan.md:106 «or, since 2026-10-12-self-description, the tool-level `help` handed to `Session::run` is refused `unknown-verb` and its span still records the verb's table word»
  replace-with: «a session-level verb or, since 2026-10-12-self-description, the tool-level `help` handed to `run` records its table word beside `unknown-verb`»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the parent of the call's `command` span by nesting;»
  source: .andromeda/obs-plan.md:107 «while the session verbs `start` · `status` · `stop` and, since 2026-10-12-self-description, `help`, answered by the server, leave a `tool_call` with no `command` child»
  replace-with: «the parent of the call's `command` span by nesting — the session verbs `start` · `status` · `stop` and, since 2026-10-12-self-description, `help`, answered by the server, leave a `tool_call` with no `command` child, a `help` call ending `accepted` (measured in process by `stand_mcp_tools`), no field added;»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «nor do the wire, the client's `call`, `command_line`, the `start` spawn,»
  source: .andromeda/obs-plan.md:106 «the client's `call`, `command_line` and its argv reader — its `help` ending, since 2026-10-12-self-description, reaching no session and opening no span — `describe()`, the `start` command's host spawn»
  replace-with: «nor do the wire, the client's `call`, `command_line` — its `help` ending, since 2026-10-12-self-description, reaching no session and opening no span —, `describe()`, the `start` spawn,»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «read no env var and no clock and print nothing, and the session's record»
  source: .andromeda/obs-plan.md:70 «and so, since 2026-10-12-self-description, does `describe` as to subscriber, env var and print (its clock not stated by that chunk's report)»
  replace-with: «read no env var and no clock and print nothing — and so, since 2026-10-12-self-description, does `describe` as to subscriber, env var and print (its clock not stated), building the self-description as one line of JSON for the command line's `help` and the MCP `help` tool, never a log line —, and the session's record»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «and the engine's state check `accessibility_states` install no subscriber and print nothing;»
  source: .andromeda/obs-plan.md:70 «for `stand_mcp_masked` since 2026-10-12-self-description — a password typed over MCP in process in both layout modes, its results read into assertions only — which installs no subscriber»
  replace-with: «and the engine's state check `accessibility_states` install no subscriber and print nothing, and `stand_mcp_masked` (since 2026-10-12-self-description, a password typed over MCP in process, its results read into assertions only) installs no subscriber and reads no env var;»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «the parent of the call's `command` span.»
  source: .andromeda/obs-plan.md:107 «while the session verbs `start` · `status` · `stop` and, since 2026-10-12-self-description, `help`, answered by the server, leave a `tool_call` with no `command` child»
  replace-with: «the parent of the call's `command` span — the session verbs `start` · `status` · `stop` and, since 2026-10-12-self-description, `help`, answered by the server, leave a `tool_call` with no `command` child.»
