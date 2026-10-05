# Operator pass — 2026-10-05-fork-ci-reached

Plan §Test Commands entries 10-15, driven by the session on the operator's word (operator, 2026-10-05, P5 review;
re-given in the implement session: "go — run the operator pass now, entries 10-15 in order").

## Entry 10 — hygiene

`python -X utf8 {tools_dir}/gate.py hygiene` · exit 0 · atoms `exit 0` ✓ · `contains hygiene: clean` ✓

```
hygiene: clean — read 39 (runs 34 · evidence 2 · inputs 3) · trails 14 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## Pre-CI commit

`4268555d` — `chore(2026-10-05-fork-ci-reached): operator pre-CI commit, for the run this chunk's verdict reads`
(`git add -A` after entry 10 read clean; parent `50c13b59`).

## Entry 11 — fast leg, clean-tree guard, push

`bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
· exit 0 · fast leg green (ci-scripts `Ran 16 tests … OK`) · `50c13b59..4268555d  build/escher-0.1.0 -> build/escher-0.1.0`.
The push triggered exactly one workflow: CI run **37375560233** (event `push`); Publish Browser, WPT and Post WPT
results did not fire.

## Entry 12 — cold CI read (run id 37375560233, attempt 1)

`python -X utf8 {tools_dir}/ci.py conclusion --sha HEAD --wait 6000` · exit 0 · atoms `exit 0` ✓ · `contains verdict: green` ✓

```
4268555dd2ea verdict: green · checks 13/13 · wall 1255 s · runs CI#37375560233 completed/success
```

## Entry 13 — rerun

`gh run rerun 37375560233` · exit 0 (attempt 2 read `in_progress` 15 s later).

Caches the cold attempt saved (`gh cache list`, read before attempt 2 finished): 11 `v0-rust-*` entries, one per
compiling job (7 linux — test-features-default, clippy, doc, build-counter, build-msrv, build-wasm-examples,
build-features-default — + 4 matrix — windows, macos, ios, android), all on `refs/heads/build/escher-0.1.0`, plus 1
`cache-apt-pkgs_*` (the matrix's apt action) — 12 entries. The plan's Implementation notes counted "6 linux + 4 matrix
= 10"; the workflow has 7 compiling linux jobs (research.md's own uncached-job count). `gh api repos/Turbolet85/escher/actions/cache/usage` read right after the cold run lagged
(`active_caches_count` 1); after the warm run it read `active_caches_count` 12 · `active_caches_size_in_bytes`
9 728 732 229 (≈ 9.73 GB, ≈ 97 % of GitHub's 10 GB per-repository budget). Rust caches range 0.18 GB (doc) to
1.10 GB (android).

## Entry 14 — warm CI read (same run, attempt 2)

`python -X utf8 {tools_dir}/ci.py conclusion --sha HEAD --wait 6000` · exit 0 · atoms `exit 0` ✓ · `contains verdict: green` ✓

```
4268555dd2ea verdict: green · checks 13/13 · wall 475 s · runs CI#37375560233 completed/success
```

## Entry 15 — per-job timings (report-only)

The entry's literal form, `gh run view 37375560233 --json jobs --jq …`, exited 1: `failed to get run: HTTP 404`
against `repos/DioxusLabs/blitz/…` — this checkout has no `gh repo set-default` and carries an `upstream` remote
(`DioxusLabs/blitz`), so `gh` resolved the run against upstream. Re-fired as `gh run view 37375560233 -R
Turbolet85/escher --json jobs --jq '.jobs[] | [.name, .startedAt, .completedAt] | @tsv'` · exit 0. The cold
attempt's rows were read with the same `-R` form before entry 13 fired.

| job | cold (s) | warm (s) | cold start→end (UTC) | warm start→end (UTC) |
|---|---|---|---|---|
| Rustfmt | 13 | 16 | 21:28:38→21:28:51 | 21:50:08→21:50:24 |
| Test CI scripts | 4 | 4 | 21:28:40→21:28:44 | 21:50:08→21:50:12 |
| Test [default features] | 315 | 169 | 21:28:38→21:33:53 | 21:50:07→21:52:56 |
| Clippy | 189 | 42 | 21:33:38→21:36:47 | 21:50:07→21:50:49 |
| Test (android) | 485 | 176 | 21:36:51→21:44:56 | 21:53:00→21:55:56 |
| Build [default features] | 275 | 67 | 21:36:51→21:41:26 | 21:53:00→21:54:07 |
| Build counter example | 175 | 45 | 21:36:52→21:39:47 | 21:53:00→21:53:45 |
| MSRV Build [Rust 1.91] | 258 | 63 | 21:36:50→21:41:08 | 21:53:00→21:54:03 |
| Test (ios) | 420 | 80 | 21:37:03→21:44:03 | 21:53:03→21:54:23 |
| Test (windows) | 760 | 302 | 21:36:53→21:49:33 | 21:53:00→21:58:02 |
| Documentation | 32 | 17 | 21:36:50→21:37:22 | 21:53:00→21:53:17 |
| Build wasm examples | 268 | 67 | 21:36:51→21:41:19 | 21:53:00→21:54:07 |
| Test (macos) | 563 | 156 | 21:37:04→21:46:27 | 21:53:07→21:55:43 |

- Run wall-clock (ci.py): cold 1255 s → warm 475 s (−62 %).
- Fast set finished: cold 21:36:47 (Clippy; 8 m 09 s after the first fast job started), warm 21:52:56 (Test
  [default features]; 2 m 49 s). On the cold attempt Clippy started at 21:33:38, 5 m after the other three fast jobs
  (queue time; cause not measured).
- The warm fast `test` leg read 169 s against 315 s cold — the plan predicted "far below"; what was measured is
  −46 %, with the warm leg still above the host's warm 11.44 s (`baseline-rerun.md`). The slowest warm leg is the
  windows matrix test, 302 s.
