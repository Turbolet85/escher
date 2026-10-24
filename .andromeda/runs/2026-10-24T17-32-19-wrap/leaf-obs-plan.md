group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «are measured too — 28 runs by build and level, 0 needles in every run.»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11 «A recording session's clients are read by a standing check: `host_log`'s third test (unix only) runs each client command through `common::run` at `RUST_LOG=trace`»
  replace-with: «are measured too — 28 runs by build and level, 0 needles in every run (the client's a one-off reading). Since 2026-10-24-stand-requirement-sweep a recording session's clients are read by a standing check: `host_log`'s third test (unix only) runs each client command at `trace` — `start crud --record`, `type` with a sentinel, `snapshot`, `snapshot --part 1`, `lint`, `stop` — in `hl-client` and `hl-client-rec`, and over all the clients' own stderr reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles` occurrences under both builds; a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — owned by a `.andromeda/residuals.md` line (target: next).»

- leaf: .claude/rules/observability.md:36   class: omission
  leaf-text: «with 0 occurrences of a typed sentinel either way.»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11 «a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages»
  replace-with: «with 0 occurrences of a typed sentinel either way. A paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next); a recording session's clients' own stderr is read by a standing check since 2026-10-24-stand-requirement-sweep (`host_log`'s third test, unix only, at `trace` under both builds, 0 needles).»
