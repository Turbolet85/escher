group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:31   class: omission
  leaf-text: «- The allowlist reaches this sink only: the upstream apps' `fmt::init()` (stdout) and the WPT runner's `env_logger` stay unscrubbed.»
  source: .andromeda/obs-plan-detail/log-coverage.md:49 «its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the span set or to the scrub's limits is made in that document by hand»
  replace-with: «- The allowlist reaches this sink only: the upstream apps' `fmt::init()` (stdout) and the WPT runner's `env_logger` stay unscrubbed. The published statement `docs/capabilities.md` restates the scrub's limits (the raw panic message, `log.file` on a bridged `js_console` record, the dropped third-party WARN or ERROR, the unscrubbed upstream apps and WPT runner) and the driver's `command` and `tool_call` spans; its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the span set or to the scrub's limits is made in that document by hand.»
