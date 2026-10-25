group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:6   class: omission
  leaf-text: «the cold-agent pipe spawns `escher-session mcp` with `RUST_LOG=info` set on that child only»
  source: .andromeda/obs-plan.md:118 «spawned as `sh -c "exec env RUST_LOG=info <abs target/debug/escher-session> mcp --record <abs target/cold-agent/record>»
  replace-with: «the cold-agent pipe spawns its live `escher-session mcp --record <abs target/cold-agent/record>` child with `RUST_LOG=info` set on that child only (its kept checks owner-only under `target/cold-agent/record/`, never printed or copied)»

- leaf: .claude/docs/services/escher-telemetry.md:6   class: stale
  leaf-text: «which the pipe reads only to count ` span="tool_call" ` lines and their ` ending="refused" ` / ` ending="error" `, never printing or copying it; its two replay children send stderr to `/dev/null`.»
  source: .andromeda/obs-plan.md:118 «which the pipe reads only to count ` span="tool_call" ` lines, their ` ending="refused" ` / ` ending="error" ` and the accepted lines of each counted verb (`lint` · `screenshot` · `advance`)»
  replace-with: «which the pipe reads only to count ` span="tool_call" ` lines, their ` ending="refused" ` / ` ending="error" ` and the accepted lines of each counted verb (`lint` · `screenshot` · `advance`), never printing or copying it; its two replay children, spawned without `--record`, send stderr to `/dev/null`.»

- leaf: .claude/docs/services/escher-telemetry.md:33   class: stale
  leaf-text: «stays unmeasured (the host boots only the four lean tasks and none pages), owned by a `.andromeda/residuals.md` line (target: next)»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11 «a paged `part` read through the `escher-session` host's own sink is read by its fourth test (unix only), which runs `serve cells` at `RUST_LOG=trace` in `hl-cells`, commits a sentinel into a part-2 cell»
  replace-with: «is read by `host_log`'s fourth test (unix only) — `serve cells` at `RUST_LOG=trace` in `hl-cells`, a sentinel committed into a part-2 cell, then `snapshot --part 2` — 0 id and 0 name needles under both builds, its failures naming a kind and a count only»

- leaf: .claude/docs/services/escher-telemetry.md:42   class: omission
  leaf-text: «0 `recorded_needles` occurrences over the clients' own stderr, under both builds.»
  source: .andromeda/obs-plan-detail/log-coverage.md:49 «and `host_log`'s fourth test reads a paged part of a served Cells screen through the host's own sink (`serve cells`, `snapshot --part 2`), 0 id and 0 name needles under both builds;»
  replace-with: «0 `recorded_needles` occurrences over the clients' own stderr, under both builds; `host_log`'s fourth test (unix only) reads a paged part of a served Cells screen through the host's own sink — `serve cells` at `RUST_LOG=trace` in `hl-cells`, a sentinel committed into a part-2 cell, then `snapshot --part 2` — 0 id and 0 name needles under both builds.»
