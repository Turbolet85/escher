
## 2026-10-06-cold-agent-run-pipe — the cold-agent pipe log, scrub position and artifact
**Section:** §3 Observability Harness Contract · §6 Log Coverage (Log format) · §8 Scrubbing · §9 Telemetry artifact handling
**Change:**
- §3: the cold-agent pipe log — python3-`json` events (`run.start`, `run.end` appended to `target/cold-agent/events.jsonl`; `status`, `cleanup` printed only), `verdict.json`, the stub's call log; no `tracing`, OTel or third-party logger, no env read.
- §6: its log format — the four events, the verdict fields, the call-log line `{seq, tool, outcome, cause}` (an unknown tool logged `tool: null`), the raw transcript and client stderr never printed.
- §8: the pipe sits outside escher's sink with no content-named key and no transcript content in events or verdict; `transcript.jsonl` is raw by design (model output, tool I/O, host paths, the client's socket path, a rate-limit line), never printed, gitignored; the one committed copy host-path-masked.
- §9: a `Cold-agent pipe state` row — local only, gitignored, uploaded by no CI leg, removed by its `cleanup` alone; the per-run session dir removed on exit.
**Why:** the cold-agent run pipe chunk added a harness log beside agent-run's. The §8 record was a D-obs-pii escalation, ratified at this wrap by the overseer under the founder's standing delegation; no rule added — `gate.py hygiene` P1 already refuses an unmasked host path in committed evidence.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/
