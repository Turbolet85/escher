
## 2026-10-05-fork-ci-reached — one dev-profile debuginfo level for host and CI
**Section:** §Established Decisions ([Build profiles])
**Change:** was six named profiles (`profile` … `tiny`) and no `[profile.dev]`; now `[profile.dev] debug = "line-tables-only"` precedes them, inherited by the `test` profile, one level for the dev host and CI; `production` and every other profile unchanged. Its effect is recorded as measured: a blitz-tests binary's debuginfo share 82 % → 55 %, the cold local baseline (build + blitz-tests + workspace tests) 2239.59 s → 161.36 s (dev host, 32 CPUs).
**Why:** debuginfo was 82 % of a test binary's bytes and the cold test-profile compile dominated; the P4 fork chose a workspace stanza over a CI-only `CARGO_PROFILE_DEV_DEBUG` (no new env var, no host/CI divergence) — answered by the overseer under the founder's explicit delegation, relayed verbatim by the operator at the P5 review.
**Kept:** consolidating the 59 blitz-tests binaries was deferred until measured after cache + debuginfo (the same P4 round).
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/
