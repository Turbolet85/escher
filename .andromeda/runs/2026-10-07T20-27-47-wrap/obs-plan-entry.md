
## 2026-10-07-refusal-detection — eight cause names returned; the record among the silent values
**Section:** §3 Observability Harness Contract → Logging stack · §4 Span / Trace Coverage · §9 (one citation)
**Change:**
- §3: the driver crate stays silent with the refusal detection and the sixth verb `scroll`; the session's record of ids — id text, at most 4096 ids, no reader outside the crate, no `Debug` — is printed, logged and fielded by nothing. The silent check set names nine `stand_act_*` checks (was six) and `scroll_into_view_nested`; fourteen of the fifteen session-holding checks read `session_common` (was eleven of twelve).
- §4: the refusal-cause field's domain is returned by code eight of eight (was four of eight), six by the executor.
- Two citations re-pointed (`command.rs:161` → `:167`; `session_common/mod.rs:194-202` → `:232-240`).
**Why:** the chunk built the detection under the crate's silence constraint; the per-command span is still owed by the route entry "Driver command spans". The plan's wording "six of the eight" was the executor's share; all eight are returned.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/
