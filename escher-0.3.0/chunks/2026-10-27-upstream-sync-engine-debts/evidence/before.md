# Before — 2026-10-27-upstream-sync-engine-debts

Read at /implement Setup, 2026-10-27T09:58:03Z (run `.andromeda/runs/2026-10-27T09-58-03-implement/`).

- **Base:** `de85792f5e9dcb895d75bb65a3b91e79a4ea24c2` — `git rev-parse HEAD` on `build/escher-0.3.0`, the plan's Metadata base.
- **Upstream read:** `upstream/main` = `ebacd68bd35552e42ed546faa7fa53dfccc109af` (`git log -1 --format=%H upstream/main`), the plan's measured sha; unmoved.
- **CI on the base:** CI#40466575975 green, 17/17 (scope.md §CI verdict, trail `.andromeda/runs/2026-10-27T07-42-37-phase/ci-no-marker.json`).
- **The eleven shas** resolve as commits in this repository (`git cat-file -t`: 11 `commit`).
- **Worktrees:** `target/tmp/trial-wt` registered, detached at `02b24fccf` (shape A's trial commit; research.md §The trial takes names `fb1f87191`, shape B, as its HEAD — the worktree reads shape A). Left as it is; its removal is the founder's.
- **`.git/rr-cache`:** 16 entries before the take; every pick runs `-c rerere.enabled=false`.

## P5's baseline readings on the base (the plan's `baseline` keys, 2026-10-27T09:38Z–09:43Z)

| reading | on the base |
|---|---|
| workspace tally (test-plan-detail/ci-integration.md:43, CI#40434241601) | 208 lines · 1117 passed · 0 failed · 12 ignored |
| `run stand` run.end | passed 205 · failed 0 · ignored 7 (50 `stand_*` files) |
| `run all` run.end | passed 630 · failed 0 · ignored 12 |
| a11y leg | lines 14 · passed 85 · failed 0 |
| `tests/blitz-tests/tests/*.rs` | 147 files |
| `tests/blitz-tests/tests/stand_*.rs` | 50 files |
| blitz-vibey-script `tests/dom.rs` | 27 passed |
| dioxus-native-dom lib | 81 passed |
| blitz-dom lib | 106 passed |
| integration readings entry | 23 targets · 125 passed · 0 failed |
| seven_guis per-package | 12 result lines · 41 passed |
| CI-scripts | Ran 108 tests · OK |
| audit | advisories ok · licenses ok |
| exact-take entry | exit 1 (an empty left side) |
| file-identity entry | 23 differing |
| fixed-sha census | last line 2 |
