group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «which opens no span, emits no event, reads no env var, mutates nothing and prints, panics and logs nothing, the crate still taking no `tracing` — and the settle loop»
  source: .andromeda/obs-plan.md:119 «the in-process `Session::lint` — no verb, never handed to `Session::run` — with blitz-dom's read-only `BaseDocument::text_extent` and `Harness::text_contrast`, the lint's contrast reader»
  replace-with: «which opens no span, emits no event, reads no env var, mutates nothing and prints, panics and logs nothing, the crate still taking no `tracing` — nor the in-process `Session::lint` — no verb, never handed to `Session::run` — with blitz-dom's read-only `BaseDocument::text_extent` and `Harness::text_contrast`, the lint's contrast reader, which open no span, emit no event and record no field (PROVISIONAL: no new field), escher-driver keeping its 2 `info_span!` sites — and the settle loop»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the sixteenth, `replay`, holds no tracing site, env read or print and calls `serde_json` only to read;»
  source: .andromeda/obs-plan.md:80 «The visual lint is silent: escher-driver's private module `lint` (`Session::lint` and the public `Finding` · `Defect` · `Reason` · `Bounds`), blitz-dom's private module `text_extent`»
  replace-with: «the sixteenth, `replay`, holds no tracing site, env read or print and calls `serde_json` only to read; the visual lint — escher-driver's private module `lint` (`Session::lint` and the public `Finding` · `Defect` · `Reason` · `Bounds`), blitz-dom's private module `text_extent` (`BaseDocument::text_extent`, read-only) and the checks `stand_lint` (a `fixtures` and `session_common` reader, both layout modes) and `text_extent` — holds no print, log site, subscriber, sink, env read, file or socket, its readings going into assertions only, and `Finding` and `Reason` derive `Debug` and hold stable ids, closed words and numbers only — no `NodeId`, page text, colour or value — and nothing logs, fields or prints one;»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «the wire's `recording-failed` reply); the command span keeps its eight fields a recording held or not»
  source: .andromeda/obs-plan.md:119 «the in-process `Session::lint` — no verb, never handed to `Session::run` — with blitz-dom's read-only `BaseDocument::text_extent` and `Harness::text_contrast`, the lint's contrast reader»
  replace-with: «the wire's `recording-failed` reply), nor the in-process `Session::lint` (no verb, never handed to `Session::run`) with blitz-dom's read-only `BaseDocument::text_extent`, which open no span, emit no event and record no field, escher-driver keeping its 2 `info_span!` sites — and nothing logs, fields or prints a lint `Finding` or `Reason`, which derive `Debug` and hold stable ids; the command span keeps its eight fields a recording held or not»
