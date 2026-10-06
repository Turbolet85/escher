
## 2026-10-06-snapshot-state-fidelity — bridge log-site citations re-pointed
**Section:** §6 Log Coverage → Logged events · §8 PII Scrubbing & Compliance → Values logged as-is
**Change:** no claim changes. 6 `mutation_writer.rs` line citations re-pointed: §6 `:119-205` · `:305` · `:388` are now `:152-238` · `:338` · `:421`; §8 `:150` · `:202` · `:388` are now `:183` · `:235` · `:421`.
**Why:** the chunk inserted a 33-line constant above the bridge's log sites and added none — the `trace!` count stays 13, and the snapshot module and the two new test files hold no log, print or env site.
**Ref:** .andromeda/runs/2026-10-06T22-39-16-wrap/
