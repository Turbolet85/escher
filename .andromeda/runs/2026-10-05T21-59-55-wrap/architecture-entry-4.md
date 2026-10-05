
## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `Cargo.toml`, `ci.yml`, `wpt.yml` or `publish-browser.yml` lines
**Change:** 64 citations re-pointed to the moved files — `Cargo.toml` from line 195 on +3 (the `[profile.dev]` stanza), `wpt.yml` from line 26 on +1 and `publish-browser.yml` from line 37 on +1 (the repository guards), `ci.yml` by a range map over the rewritten file (e.g. fmt job 135-151, clippy job 153-173, the jobs 30-309); the two citations of the removed opt-level rewrite went with the CI/CD rewrite. No claim text changed by the re-point.
**Why:** this chunk moved the cited lines; a stale `file:line` sends every later reader to the wrong code.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/
