group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:42   class: omission
  leaf-text: «`mcp_log` a recording `mcp --record <dir>` role at `trace`: 0 of the recording needles (the kept check's directory name and contents), the span counts unchanged.»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11 «A recording session's clients are read by a standing check: `host_log`'s third test (unix only) runs each client command through `common::run` at `RUST_LOG=trace`»
  replace-with: «`mcp_log` a recording `mcp --record <dir>` role at `trace`: 0 of the recording needles (the kept check's directory name and contents), the span counts unchanged; since 2026-10-24-stand-requirement-sweep `host_log`'s third test (unix only) is a standing check of a recording session's clients — each client command (`start crud --record`, `type` with a sentinel, `snapshot`, `snapshot --part 1`, `lint`, `stop`) run through `common::run` at `RUST_LOG=trace` in `hl-client` and `hl-client-rec`, guarded by `common::Started` — reading 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles` occurrences over the clients' own stderr, under both builds.»

- leaf: .claude/docs/services/escher-telemetry.md:33   class: omission
  leaf-text: «`host_log` and `mcp_log` each drive one part read at `trace` with 0 needles.»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11 «a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next)»
  replace-with: «`host_log` and `mcp_log` each drive one part read at `trace` with 0 needles — a part 1 read of a lean task, which pages nothing; a paged read (part 2 of the shared `many_items` page) is read in process by `stand_act_spans`, its ids and names joining the needles, none on a captured line, while a paged `part` read through the `escher-session` host's own sink stays unmeasured (the host boots only the four lean tasks and none pages), owned by a `.andromeda/residuals.md` line (target: next).»
