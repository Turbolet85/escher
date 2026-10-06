
## 2026-10-06-stand-test-contract — the agent-run harness log
**Section:** §3 Observability Harness Contract · §6 Log Coverage · §8 PII Scrubbing & Compliance · §9 CI Integration
**Change:**
- §3: a new "Agent-run harness log" paragraph — `scripts/agent-run.sh` writes JSON lines encoded by python3's `json` (`boot`, `run.start`, `test`, `run.end` printed and appended to `target/agent-run/events.jsonl`; `status`, `cleanup` printed only); harness metadata, not a telemetry sink. The marker was "product mode, a JSON log schema, log file location, …"; it now reads "a JSON schema or log-file location for escher's own sink".
- §6: a "Log format (the agent-run harness)" block — the event fields, one `test` per libtest line, nothing read inside a `failures:` block, raw output in `run.log`; the marker is scoped "for escher's own sink".
- §8: the scrub-reach list adds the harness log — outside escher's scrub, no content-named field, no captured output; `run.log` unscrubbed like `target/ci-logs/`.
- §9: the artifact table adds "Agent-run harness state" — `target/agent-run/`, local only, uploaded by no CI leg.
**Why:** the stand test contract chunk added a JSON-line harness log beside escher's text sink.
**Kept:** escher's own sink stays one text line per event; its JSON schema and file sink stay unmeasured.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/
