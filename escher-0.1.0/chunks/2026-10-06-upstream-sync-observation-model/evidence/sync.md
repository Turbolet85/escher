# Upstream sync — measured no-op (2026-10-06-upstream-sync-observation-model)

Measured at /implement on the untouched tree, chunk start commit `42b80ad94e1c96014c0325e2a3f4a7135ff1f9ca`.
Gate block: entries 7 · green 7 · red 0 (gate v1.11, /implement run `2026-10-06T18-06-37-implement`).

## Pin (entry 1)
- `git ls-remote upstream refs/heads/main` → `2335458530518cdf167c55ce635fae99323e0789	refs/heads/main` (read 2026-10-06T18:07:56Z; also read inside the gate run, entry 1 green)
- `git merge-base --is-ancestor 2335458530518cdf167c55ce635fae99323e0789 HEAD` → exit 0
- Entry 1 last line: `pin-held`. Upstream is 0 commits past the pin; nothing merged, no merge commit.

## No-change guard (entry 2)
- Paths outside `.andromeda/`, `escher-0.1.0/`, `.claude/session-handoff.md` differing from `42b80ad9`: `0` (exit 1).

## Gates (entries 3–6)
- `bash .github/scripts/ci-leg.sh fast` → exit 0; `target/ci-logs/test.log` fresh for this run.
- Count entry: `lines 124 passed 454 failed 0 ignored 5` — the test-plan §9 baseline, reproduced unchanged.
- CI-scripts leg: `Ran 64 tests in 3.403s` / `OK`.
- `bash .github/scripts/ci-leg.sh doc` → exit 0.

## Fork CI (entry 7)
- `42b80ad94e1c verdict: green · checks 16/16 · wall 491 s · runs CI#37495205882 completed/success` (repo Turbolet85/escher).
