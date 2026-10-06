
## 2026-10-06-id-persistence — one println! in the stand checks: the re-exec child's captured ids
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet)
**Change:** the stand bullet said "no `escher_telemetry::init`, no `println!`, no env read in it or its checks". It now says: no `escher_telemetry::init` and no env read; no `println!` save `stand_id_persistence`'s re-executed child, which prints its pid and ids to stdout that the parent captures and reads only into its assertion — never a log. A headless boot still has no escher sink.
**Why:** the fresh-process proof needs the child's ids in the parent. Captured stdout read by an assertion is not a log channel, so the no-sink invariant holds.
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/
