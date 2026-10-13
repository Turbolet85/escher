group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:33   class: omission
  leaf-text: «0 id needles, 0 name needles and 0 occurrences of a typed sentinel at `trace` under both builds (`mcp_log`).»
  source: .andromeda/obs-plan.md:180 «a shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension — measured: 0 occurrences of the first 64 base64 characters of a shot»
  replace-with: «0 id needles, 0 name needles and 0 occurrences of a typed sentinel at `trace` under both builds (`mcp_log`); since 2026-10-12-headless-screenshot a shot's command-span line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension — 0 occurrences of the first 64 base64 characters of a shot in the `escher-session` host's and the `mcp` role's stderr at `trace`, under both builds (`host_log`, `mcp_log`).»

- leaf: .claude/docs/services/escher-telemetry.md:42   class: omission
  leaf-text: «the real `escher-session` binary at `RUST_LOG=trace`: no stable id and no accessible name on stderr.»
  source: .andromeda/obs-plan.md:70 «`mcp_log` reads the role's stderr at `trace` into its assertions only, under both builds — 0 typed-sentinel occurrences, 0 id needles, 0 name needles, one `tool_call` line per call»
  replace-with: «the real `escher-session` binary at `RUST_LOG=trace`: no stable id, no accessible name, no typed sentinel and no shot's image bytes on stderr, under both builds; `cargo test -p seven_guis --test mcp_log` — the same binary's `mcp` role at `trace`: 0 typed-sentinel occurrences, 0 id needles, 0 name needles, no shot's image bytes, one `tool_call` line per call.»
