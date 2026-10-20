
## 2026-10-20-large-screen-in-parts — a part read's span and its silence
**Section:** §3 Observability Harness Contract → Logging stack · §4 Span / Trace Coverage · §6 Log Coverage · §8 PII Scrubbing & Compliance
**Change:**
- §4: an accepted `lint` and a part read (`Outcome::Part`) record `verb` alone; a part's number and the part count are never fielded; dioxus-native-dom's `Snapshot::text_parts` opens no span, emits no event and records no field.
- §6: a part read's line is `span="command" verb="snapshot"` (with `cause` when refused) and holds no part number, count, size or text; `host_log` and `mcp_log` each drive one part read at `trace` with 0 needles.
- §8: `stand_act_spans` 35 calls per mode (was 34), a part read among them.
- §3: the parts check `stand_snapshot_parts` is silent, and `text_parts` and `Outcome::Part` are never logged (a new bullet in the `absence · logs` group). `session_common` readers twenty-nine (was twenty-eight), `fixtures` readers 24 (was 23).
**Why:** this chunk's span reading and silence census, CI green 17/17 on 191e66fdb (CI#39410865604). No field is added, so no ruling is owed.
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/
