group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the sixteenth, `replay`, holds no tracing site, env read or print and calls `serde_json` only to read.»
  source: .andromeda/obs-plan.md:77 «- The upstream focus checks are silent: `focus_within` and `focus_redraw`, upstream's own targets, install no subscriber, read no env var and print nothing — `focus_redraw`'s `RecordingShell` a counting `ShellProvider`, no sink»
  replace-with: «the sixteenth, `replay`, holds no tracing site, env read or print and calls `serde_json` only to read; the upstream focus checks `focus_within` and `focus_redraw`, upstream's own targets, likewise install no subscriber, read no env var and print nothing (`focus_redraw`'s `RecordingShell` a counting `ShellProvider`, no sink), and the engine changes they pin — `:focus-within` matching, the not-in-document focus refusal and the focus-move redraw (PROVISIONAL) — add no log site, span, field or env read, the `Focussed node` INFO line standing after the new gate, `host_log` and `mcp_log` passing under both builds.»
