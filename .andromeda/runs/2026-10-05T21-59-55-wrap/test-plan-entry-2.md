
## 2026-10-05-fork-ci-reached — local baseline re-measured under line-tables-only debuginfo
**Section:** §9 CI Integration (Local baseline)
**Change:** was the full-debuginfo dev profile's figures, per "2026-10-05-as-built-baseline — the local baseline reading" — build 120.26 / 5.54 s, blitz-tests 486.45 / 7.41 s, workspace tests 1632.88 / 13.08 s cold/warm; now the dev profile carries `debug = "line-tables-only"` and the baseline reads build 63.90 / 2.07 s, blitz-tests 45.36 / 6.51 s (61 lines, 255 · 0 · 3, font-skip line absent), workspace tests 52.10 / 11.44 s (108 lines, 407 · 0 · 3), cold total 161.36 s against 2239.59 s, `target/debug` 32G; fmt and clippy legs exit 0. The workspace rustdoc reading (exit 101, 3 crates, 9 errors) keeps its earlier evidence, not re-measured.
**Why:** the §9 baseline's "dev profile" condition moved; the counts held, so the profile change cost no test.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/
