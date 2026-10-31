group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»
- leaf: .claude/docs/services/escher-telemetry.md:33   class: omission
  leaf-text: «0 id and 0 name needles under both builds, its failures naming a kind and a count only.»
  source: .andromeda/obs-plan-detail/log-coverage.md:131 «`host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, under `#[cfg(unix)]` and the state directory `hl-popup`, reading the host at `trace`: 0 option needles»
  replace-with: «0 id and 0 name needles under both builds, its failures naming a kind and a count only; since 2026-10-31-select-popup a select's list opened and chosen at `trace` through the host (`host_log`, unix only, in `hl-popup`) and through the `mcp` role (`mcp_log`, recording nothing) reads 0 option needles.»
- leaf: .claude/docs/services/escher-telemetry.md:42   class: omission
  leaf-text: «then `snapshot --part 2` — 0 id and 0 name needles under both builds.»
  source: .andromeda/obs-plan.md:96 «`host_log` and `mcp_log` each read a list opened and chosen at `trace`, 0 needles, `host_log`'s test `#[cfg(unix)]` and `mcp_log`'s recording nothing»
  replace-with: «then `snapshot --part 2` — 0 id and 0 name needles under both builds; since 2026-10-31-select-popup `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace` (unix only, in `hl-popup`, its `command` span lines counted) and `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace` (recording nothing, no `ml-popup`) each read a select's list opened and chosen at `trace`: 0 option needles.»
