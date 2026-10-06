# Operator pass — entries 16-19 (tool numbering; the plan's prose says 15-18)

Driven by the session on the overseer's word under the founder's standing delegation of technical decisions
(2026-10-05), in order.

| entry | command | reading |
|---|---|---|
| 16 | `gate.py hygiene` | exit 0 · `hygiene: clean — read 35 (runs 31 · evidence 4 · inputs 0) … 0 host paths kept` |
| — | pre-CI commit | `5dc809a1e7664a5763eaee444042c31da2e36698` `chore(2026-10-05-ci-gate-legs): operator pre-CI commit, for the run this chunk's verdict reads` (whole tree, per the 4268555d precedent) |
| 17 | `ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` | exit 0 · fast gate test counts `407 0 3`, ci-scripts `Ran 23 tests` `OK` · push `15e36b8a..5dc809a1  build/escher-0.1.0 -> build/escher-0.1.0` |
| 18 | `ci.py conclusion --sha HEAD --wait 3000` | exit 0 · `5dc809a1e766 verdict: green · checks 16/16 · wall 769 s · runs CI#37386253475 completed/success` (polled 26× over 776 s) |
| 19 | `gh run view 37386253475 -R Turbolet85/escher --json jobs …` | per-job table below (recorded) |

## Per-job timings — run 37386253475

| job | started | completed | seconds |
|---|---|---|---|
| Test CI scripts | 23:03:26 | 23:03:33 | 7 |
| Rustfmt | 23:03:26 | 23:03:43 | 17 |
| Clippy | 23:03:27 | 23:04:27 | 60 |
| Test [default features] | 23:03:26 | 23:07:45 | 259 (cache: `No cache found` — the new `workspace-test` key, first save) |
| Dependency audit | 23:07:49 | 23:08:19 | 30 |
| Build counter example | 23:07:48 | 23:08:51 | 63 |
| Build wasm examples | 23:07:48 | 23:09:02 | 74 |
| Build [default features] | 23:07:48 | 23:09:05 | 77 |
| MSRV Build [Rust 1.91] | 23:07:48 | 23:09:08 | 80 |
| Accessibility (a11y) tests | 23:07:48 | 23:09:40 | 112 |
| Test (ios) | 23:07:55 | 23:10:14 | 139 |
| Documentation | 23:07:48 | 23:11:00 | 192 (now compiles the workspace) |
| Test (android) | 23:07:50 | 23:11:22 | 212 |
| Coverage report | 23:07:49 | 23:12:05 | 256 (uncached, cold) |
| Test (macos) | 23:07:55 | 23:12:34 | 279 |
| Test (windows) | 23:07:49 | 23:16:15 | 506 — the critical path |

Pipeline wall 769 s (first start 23:03:26 → last completion 23:16:15). The uncached coverage job (256 s) finishes 251 s
before the windows matrix leg, so it did NOT lengthen the pipeline wall-clock on this run. References: the last warm
run 431 s (CI#37381915775), the first cold build-branch run 1255 s.

## Cache readings

- **a11y restore (the prediction, now measured):** rust-cache input `shared-key: workspace-test`, `save-if: false`;
  `Cache Key: v0-rust-workspace-test-Linux-x64-db256abb-798a8cb8` · `Cache hit for:
  v0-rust-workspace-test-Linux-x64-db256abb-798a8cb8` · `Cache Size: ~960 MB (1006409498 B)`. The a11y job log
  carries 0 `Saving cache` lines.
- **The test job** saved that key (`Cache Key: v0-rust-workspace-test-Linux-x64-db256abb-798a8cb8`, `No cache found`
  on restore, post step `Sent 1006409498 of 1006409498 (100.0%)`).
- **Coverage report artifact:** `Artifact coverage-report has been successfully uploaded! Final size is 226003
  bytes` · expires 2026-10-12T23:12:02Z (7 days). CI TOTAL line: 34900 lines, 16315 missed, 53.25%.
- **Fork Actions cache after the run** (`gh cache list -R Turbolet85/escher`, read 2026-10-05 ≈ 23:20Z):
  **12 entries · 10 723 071 252 B (10.72 GB)** — over the 10 GB budget. The `workspace-test` entry the a11y job
  restored and clippy's entry are already ABSENT (evicted); two old-lockfile entries (`…-68851ead`, android and
  windows matrix, ≈ 2.16 GB together) are still present beside their new `…-798a8cb8` twins, and the doc job now
  holds a 649 MB entry. So the next run will likely restore no cache for the test and a11y jobs until the stale
  `68851ead` entries age out. This is the arch §Occupied Resources → CI infrastructure re-measure the plan assigns
  to the wrap; nothing was deleted here.
