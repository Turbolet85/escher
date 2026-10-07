## 2026-10-07-settle-detection — the settle wait exists and is silent
**Section:** §3 Observability Harness Contract → Logging stack (the no-subscriber census) · §4 Span / Trace Coverage (the escher-driver bullet)
**Change:**
- §3: the census of what installs no subscriber, reads no env var and prints nothing now names the session step `Session::act`, the harness's settle loop (`Harness::settle`; no `tracing` dependency in the crate) and the check `stand_settle`; was "the five `stand_session_*` checks and … their module"; now the shared session module is read by four of those five and by `stand_settle`.
- §4: the driver bullet names `Session::act` and the settle wait it runs, `Harness::settle`, as observed absent of spans, events and a `tracing` dependency; the settle wait exists and carries no span — one span per driver command, covering it, stays owed by the route entry "Driver command spans".
**Why:** the chunk built the wait the owed span is for and kept both crates silent by its constraint. Rule for later chunks: add no span, event or log line to the settle loop or the session step before "Driver command spans".
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/
