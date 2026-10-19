group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «`describe()`, the `start` spawn, the host's idle expiry, which writes no record,»
  source: .andromeda/obs-plan.md:115 «the recording — `Session::recording_into`, `Session::recording_failed`, the recorder, `Check::from_bytes` and the wire's `recording-failed` reply — and the host's idle expiry»
  replace-with: «`describe()`, the `start` spawn, the recording — `Session::recording_into`, `Session::recording_failed`, the recorder, `Check::from_bytes` and the wire's `recording-failed` reply —, the host's idle expiry, which writes no record,»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the builder's three state reads hold no log site, and the command span keeps its eight fields with none for a state;»
  source: .andromeda/obs-plan.md:115 «the span keeps its eight fields, a recording held or not, and an MCP call after which the recording ended is answered `isError` as `RecordingFailed`»
  replace-with: «the builder's three state reads hold no log site, and the command span keeps its eight fields with none for a state, a recording held or not, and an MCP call after which the recording ended is answered `isError` as `RecordingFailed`, its `tool_call` span ending `error` with `tool` and `ending` alone;»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «and the document's clock and `Harness::advance_time` add no log site, span, field, log field or env read.»
  source: .andromeda/obs-plan.md:76 «The session-recording checks are silent: `stand_session_recording` (a `session_common` reader, through its `replay`, `kept` and `mode`) and seven_guis `recording` install no subscriber»
  replace-with: «and the document's clock and `Harness::advance_time` add no log site, span, field, log field or env read; the session-recording checks `stand_session_recording` (a `session_common` reader) and seven_guis `recording` likewise install no subscriber, read no env var and print nothing, their readings going into assertions only, and the fifteenth escher-driver module, `recording`, holds no tracing site, env read or print, writing each answered call's arguments, `accepted` and its answer's FNV-1a 64 digest into the kept check through the crate's `json` writer — a file at rest, never a log line, field or print.»

- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «under both builds (`host_log`, `mcp_log`).»
  source: .andromeda/obs-plan.md:76 «`host_log` and `mcp_log` each read a recording host and a recording `mcp` role at `trace`: 0 of the recording needles (the kept check's directory name and contents), the span counts unchanged»
  replace-with: «under both builds (`host_log`, `mcp_log`). A recording host and a recording `mcp` role read at `trace` hold 0 of the recording needles (the kept check's directory name and contents), the span counts unchanged (`host_log`, `mcp_log`); the kept check itself holds call content at rest, unmasked typed text included — a file, never a log line, field or print.»

- leaf: .claude/docs/obs-summary.md:32   class: omission
  leaf-text: «The agent-run state area `target/agent-run/` and the cold-agent pipe's `target/cold-agent/` are local only — no CI job uploads either.»
  source: .andromeda/obs-plan.md:341 «it holds call content at rest — target ids, a `press` key, an `advance`'s ms, unmasked typed text, a `type` to a masked or unresolved control as `masked` —»
  replace-with: «The agent-run state area `target/agent-run/`, the cold-agent pipe's `target/cold-agent/` and a session's recording directory (named by `--record` or `Session::recording_into`, `0700`, one `check-{n}.json` (`0600`, `create_new`) per recorded session holding call content at rest — target ids, a `press` key, an `advance`'s ms, unmasked typed text, a `type` to a masked or unresolved control as `masked` — with each call's `accepted` and answer digest, no screen reading and no log or event line; unix only; the checks write theirs under `target/tmp/`) are local only — no CI job uploads any of them.»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «carry no span of their own; a `screenshot` call's command span»
  source: .andromeda/obs-plan.md:115 «the recording — `Session::recording_into`, `Session::recording_failed`, the recorder, `Check::from_bytes` and the wire's `recording-failed` reply — and the host's idle expiry»
  replace-with: «carry no span of their own, nor does the recording (`Session::recording_into`, `Session::recording_failed`, the recorder, `Check::from_bytes`, the wire's `recording-failed` reply); the command span keeps its eight fields a recording held or not, and an MCP call after which the recording ended is answered `isError` as `RecordingFailed`, its `tool_call` ending `error` with `tool` and `ending` alone; a `screenshot` call's command span»

- leaf: .claude/rules/observability.md:32   class: omission
  leaf-text: «never a log line, and none of it goes into a log field»
  source: .andromeda/obs-plan.md:76 «writes each answered call's arguments, `accepted` and its answer's FNV-1a 64 digest into the kept check through the crate's `json` writer — a file at rest, never a log line, field or print»
  replace-with: «never a log line, and none of it goes into a log field; a recording's kept check (`check-{n}.json`, `0600`) holds each answered call's arguments — unmasked typed text included —, its `accepted` and its answer's FNV-1a 64 digest at rest, written through the crate's `json` writer, never a log line, field or print (`host_log` and `mcp_log` read 0 of the recording needles at `trace`, the span counts unchanged)»
