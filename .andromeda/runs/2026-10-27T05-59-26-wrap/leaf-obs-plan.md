group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:14   class: stale
  leaf-text: «(no-op when off; off in the stand)»
  source: .andromeda/obs-plan.md:46 «a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on, while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off»
  replace-with: «(no-op when off; off in a package-alone build, blitz-dom's turned on by a workspace-wide build)»

- leaf: .claude/docs/obs-summary.md:30   class: stale
  leaf-text: «No counters, histograms or exporters exist.»
  source: .andromeda/obs-plan.md:143 «blitz-paint's layer counters, the bench, screenshot and `multi_doc` examples»
  replace-with: «No metric counters, gauges, histograms or exporters exist; blitz-paint's layer counters are held in process, and the bench, screenshot and `multi_doc` examples and `paint_tree_bench` print their timings.»

- leaf: .claude/docs/obs-summary.md:32   class: omission
  leaf-text: «The agent-run state area `target/agent-run/`, the cold-agent pipe's `target/cold-agent/` and a session's recording directory»
  source: .andromeda/obs-plan.md:283 «Local state stays local, uploaded by no CI leg: the agent-run and cold-agent state under `target/`, a session's state directory (`0700`, its socket alone, no log, event or screen content)»
  replace-with: «The agent-run state area `target/agent-run/`, the cold-agent pipe's `target/cold-agent/`, a session's state directory (`0700`, its socket `session.sock` alone while a session is up, no log, event or screen content) and a session's recording directory (refused `record-dir-not-private` when open to its group or others)»

- leaf: .claude/rules/observability.md:19   class: stale
  leaf-text: «(the stand leaves the engine features off)»
  source: .andromeda/obs-plan.md:46 «a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on, while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off»
  replace-with: «(the engine features stay off in a package-alone build; a workspace-wide build turns blitz-dom's on)»

- leaf: .claude/rules/observability.md:26   class: stale
  leaf-text: «writes JSON-line run events (`run.start`, `run.end`, `status`, `cleanup`) to stdout and `target/cold-agent/events.jsonl`»
  source: .andromeda/obs-plan.md:170 «`run.start {ts, task}` · `run.end {every verdict field}`; `status {the verdict}` or `status {outcome: "none"}` (exit 3) and `cleanup {outcome: "done"}` are printed only»
  replace-with: «writes JSON-line run events (`run.start`, `run.end`, `status`, `cleanup`) to stdout, `run.start` and `run.end` also appended to `target/cold-agent/events.jsonl`»
