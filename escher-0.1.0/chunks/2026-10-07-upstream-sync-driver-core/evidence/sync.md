# Upstream sync — measured no-op (2026-10-07-upstream-sync-driver-core)

Measured at /implement on the untouched tree, chunk start commit `8d156de1fcced9157289286909a2cdb3b5be5558` (HEAD when the legs ran).
Gate block: entries 9 · green 9 · red 0 (gate v1.11, /implement run `2026-10-07T04-38-06-implement`).

## Pin (entry 1)
- `git ls-remote upstream refs/heads/main` → `2335458530518cdf167c55ce635fae99323e0789	refs/heads/main` (read 2026-10-07T04:38:32Z; also read inside the gate run, entry 1 green)
- `git merge-base --is-ancestor 2335458530518cdf167c55ce635fae99323e0789 HEAD` → exit 0
- Entry 1 last line: `pin-held`. Upstream is 0 commits past the pin; nothing merged, no merge commit.

## No-change guard (entry 2)
- Paths outside `.andromeda/`, `escher-0.1.0/`, `.claude/session-handoff.md` differing from `8d156de1`: `0` (exit 1).

## Gates (entries 3–7)
- `bash .github/scripts/ci-leg.sh fast` → exit 0; `target/ci-logs/test.log` fresh for this run.
- Count entry: `lines 131 passed 548 failed 0 ignored 5` — the test-plan §9 baseline, reproduced unchanged.
- CI-scripts leg: `Ran 64 tests in 3.375s` / `OK`.
- Accessibility files in this run's `test.log`: `5` — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `accessibility_names`, `stand_accessibility_ids`.
- `bash .github/scripts/ci-leg.sh doc` → exit 0.
- Timings: not re-measured — the legs ran on a warm build cache, so their wall times are no baseline.

## Fork CI (entries 8–9)
- `8d156de1fcce verdict: green · checks 16/16 · wall 414 s · runs CI#37571032838 completed/success` (repo Turbolet85/escher).
- The two jobs read by name on that sha: `Accessibility (a11y) tests=success, Dependency audit=success`.

## Not claimed
- No verification-matrix capability. No PROVISIONAL item is touched by this record: the three stand as the masters state them.
