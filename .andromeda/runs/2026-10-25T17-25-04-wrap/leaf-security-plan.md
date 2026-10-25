group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:19   class: stale
  leaf-text: «its one live run carrying the stand's screen — ids, names and values in snapshot texts and diffs — to the model provider;»
  source: .andromeda/security-plan-detail/threat-model-summary.md:37 «the pipe's second live run is a third carriage — through the `claude` client (2.1.308) and the operator's own login it carried six apps' screens to the model provider»
  replace-with: «its first live run carrying the stand's screen — ids, names and values in snapshot texts and diffs — to the model provider and its second, fired once, six apps' screens — ids, names and control values in snapshot texts, paged parts and diffs, Cells' contents, the flight booker's `flight-type` select with its options and value, one lint's findings and one screenshot, over 66 tool calls;»

- leaf: .claude/docs/security-summary.md:19   class: omission
  leaf-text: «; `check.json` never printed.»
  source: .andromeda/security-plan.md:63 «`check.json` is never printed; the live child's kept checks (`--record`) are read for bools and counts only and never printed, and a paged answer is decoded only as exactly `text` · `part` · `parts`.»
  replace-with: «; `check.json` never printed; the live server child runs `escher-session mcp --record target/cold-agent/record` (made `mkdir -m 700`, removed by `cleanup` with the state), its kept checks read by the pipe's `recording` for bools and counts only (`recorded_sessions`, `recording_kept`), never printed, copied or committed, the replays without `--record`; a snapshot answer is decoded as a paged line only when it is a JSON object with exactly the keys `text` · `part` · `parts` and a string `text`, else read as the raw answer.»

- leaf: .claude/docs/security-summary.md:22   class: stale
  leaf-text: «the cold-agent pipe's one live run is a second carriage — the stand's screen (element ids, accessible names and control values in snapshot texts and diffs, 43 tool calls, no `screenshot`) to the model provider»
  source: .andromeda/security-plan-detail/threat-model-summary.md:35 «Since 2026-10-15-cold-agent-test the cold-agent pipe's first live run is a second carriage: through the `claude` client and the operator's own login it carried the stand's screen»
  replace-with: «the cold-agent pipe's first live run is a second carriage — the stand's screen (element ids, accessible names and control values in snapshot texts and diffs, 43 tool calls, no `screenshot`) to the model provider — and its second live run a third (six apps' screens: ids, names and control values in snapshot texts, paged parts and diffs, Cells' contents, the flight booker's `flight-type` select with its options and value, one lint's findings and one screenshot, 66 tool calls, 0 wrong, fired once)»

- leaf: .claude/docs/security-summary.md:32   class: stale
  leaf-text: «a paged part read is measured in process only — through the host's own sink it is unmeasured, owned by a `.andromeda/residuals.md` line (target: next)»
  source: .andromeda/security-plan-detail/logging-monitoring.md:14 «and through the `escher-session` host's own sink, seven_guis `host_log` runs `serve cells` at `RUST_LOG=trace` in `hl-cells`, commits a sentinel into a part-2 cell»
  replace-with: «a paged part read is measured in process and through the `escher-session` host's own sink — seven_guis `host_log` runs `serve cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell and reads `snapshot --part 2`: 0 id and 0 name needles under both builds»

- leaf: .claude/docs/security-summary.md:36   class: omission
  leaf-text: «`target/cold-agent/check.json` (the agent's check as parsed)»
  source: .andromeda/security-plan-detail/logging-monitoring.md:21 «`target/cold-agent/record/` (made `mkdir -m 700`) holds the live server child's kept checks under `--record`, one owner-only check per accepted `start` — the agent session's screens at rest»
  replace-with: «`target/cold-agent/check.json` (the agent's check as parsed); `target/cold-agent/record/` (made `mkdir -m 700`: the live server child's kept checks under `--record`, one owner-only check per accepted `start` — the agent session's screens at rest — read by the pipe's `recording` for bools and counts only, never printed, copied or committed, removed by `cleanup` with the state; the replay children run without `--record`)»

- leaf: .claude/rules/security.md:23   class: omission
  leaf-text: «`check.json`, `server.log` and the transcript are never printed;»
  source: .andromeda/security-plan.md:403 «its server log, check, raw transcript and the live child's kept checks under `target/cold-agent/record/` are never printed»
  replace-with: «`check.json`, `server.log`, the transcript and the live server child's kept checks (`--record` into the owner-only `target/cold-agent/record/`, read for bools and counts only, never copied or committed, removed by `cleanup`; the replays without `--record`) are never printed; a paged answer is decoded only as a JSON object with exactly the keys `text` · `part` · `parts` and a string `text`, else read as the raw answer;»

- leaf: .claude/rules/security.md:23   class: omission
  leaf-text: «reached on the live run only — since 2026-10-15-cold-agent-test a second carriage of the stand's screen (ids, names, values in snapshot texts and diffs) to it»
  source: .andromeda/security-plan.md:27 «the cold-agent live runs carried the screen so, a screenshot and a lint's findings among it.»
  replace-with: «reached on the live runs only — since 2026-10-15-cold-agent-test a second carriage of the stand's screen (ids, names, values in snapshot texts and diffs) to it, and the second live run a third (six apps' screens: ids, names and values in snapshot texts, paged parts and diffs, a lint's findings and one screenshot)»
