# Outcome — 2026-11-15-taffy-from-upstream-git

**Run:** 2026-11-15T20-54-54-implement · **Outcome:** green + smoke · 1 iteration

## Files changed
- `Cargo.toml` — the `taffy` line: registry `version = "0.15"` → `git = "https://github.com/DioxusLabs/taffy", rev = "e90751fdd6cec284ced294a225311b22464c6181"`, features unchanged (by pick)
- `Cargo.lock` — the `taffy` package: `0.15.1` registry + checksum → `0.15.0` git source at `e90751fd`, no checksum (3 −, 2 +), dependency list unchanged (step 2, `cargo metadata`)
- `packages/blitz-dom/src/layout/inline/layout.rs` — `crate::taffy_node_id(node_id).into(),` as `place_floated_box`'s first argument, line 753 (by pick `1a11761bc`)

The eight picks `1a11761bc` `4a5ab9cc9` `510b592ab` `1e5479ae6` `b8f4ba6fd` `9babd796a` `d3ed4e2c7` `ccfe15332`, each by
`git -c rerere.enabled=false cherry-pick -n`, each conflicting on `Cargo.lock` alone, taken ours. Record:
`.andromeda/cache/chunks/2026-11-15-taffy-from-upstream-git/evidence/take.md` (picks, eight trailers, lock delta, left-out commits, tallies).

## Deviations
none. Scope record: none — `gate.py scope` `scope: clean — changed 3 · listed 3 · recorded 0 … · excluded 185`.

## Gates
`gate.py run` (one firing): `entries 21 · green 18 · red 0 · recorded 0 · timeout 0 · not-run 3`. Entries 1–18 green;
19 (`gate.py hygiene`), 20 (the push), 21 (`ci.py conclusion`) `not run — leg operator`. No `gate.py guard` entry in the plan.
Tallies equal HEAD's: `test` 239 · 1464 passed · 0 failed · 13 ignored · `embolden` 6 · `a11y` 14 · 115 · `run stand`
294 · 0 · 8, `"outcome": "passed"` · `stand-repeat: 3 of 3 rounds green` · audit `advisories ok, licenses ok`.
No deferral. Matrix: no capability owned by the chunk (`coverage` 0 rows) — no `ref` written.

## Smoke
✓ — the plan's `role = 'smoke'` entry (`RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124`) ran
green in the P2 block, logging `service.name=seven_guis`; not re-run.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `seven_guis_native` (smoke, entry 10) | this run (gate entry) | terminated — `gate.py census --comm seven_guis_native --comm escher-session --comm cargo --args agent-run`: `census: none running` |
| `cargo` / test binaries (entries 7–18) | this run (gate entries) | terminated — same census |
| `escher-session` hosts (stand checks) | this run (gate entries) | terminated — same census |

## Next
The OPERATOR PASS: entries 19 (`gate.py hygiene`), 20 (the push, history-moving — the operator's) and 21
(`ci.py conclusion --sha HEAD --wait 2100`), the pass writing `evidence/operator-pass.md`; then `/andromeda-wrap-session`.

## Decisions & corrections
- Guard refusal: the Bash guard refused a `cd` into the skill's references dir (moves the session cwd); re-issued as Read-tool reads. One re-issue.
