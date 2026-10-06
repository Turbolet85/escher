# Scope — 2026-10-06-upstream-sync-observation-model

**Working entry (verbatim):** Upstream sync ahead of the observation model — upstream/main merged, our changes kept additive; our tests and CI prove our logic survived (per intent §Principles)

**Intent anchor:** `escher-0.1.0/intent.md` §Principles, "Upstream stays in reach": an "Upstream sync" chunk at each epoch boundary merges `upstream/main`, and our tests and CI prove our logic survived. Small and often, with our changes mostly additive (founder, 2026-10-06).

## What it builds
- **A measured no-op sync.** At take-up, `upstream/main` is `23354585` ("Update Taffy for cyclic percentage grid minimum contributions (#1070)", committed 2026-10-06T01:33:46Z). That is the sha the previous sync (`2026-10-06-upstream-sync-element-identity`) merged. `git ls-remote upstream refs/heads/main` reads the same sha. `git merge-base HEAD upstream/main` = `23354585`, so upstream is **0 commits** past the merge base and ours is 27 past it (inputs#I1).
- The founder ruled it a measured no-op (inputs#I2): record the measured upstream pin and the 0-ahead read, make no merge commit, and do no work beyond what the gates require.
- What the chunk delivers: the measurement, re-read at /implement, showing that `upstream/main` is still `23354585` and still an ancestor of HEAD, and the gate verdict on the unchanged tree. The ruling covers 0 ahead only. If upstream has moved by then, it does not cover the move. That is a halt for the operator, never a silent merge: a non-empty merge is a different chunk shape. The arch, security and obs extracts each carry this halt as an acceptance contribution. Re-measured at P3: `git ls-remote upstream refs/heads/main` = `23354585`, and `git merge-base --is-ancestor 23354585 HEAD` exits 0.
- Proof that our logic survived: fork CI on HEAD `42b80ad9` is green, checks 16/16 (CI#37495205882, Setup 5a). `[premise-corrected: "as the gate contract requires for zero source delta" would let the Rust legs be `defer`red. But CLAUDE.md Critical Warnings reads "Work is not done until `bash .github/scripts/ci-leg.sh fast` … and `bash .github/scripts/ci-leg.sh doc` … pass". So both legs RUN, not deferred, and no PREREQ is left for the next chunk. P3 measured the last leg logs at 15:14 (124 result lines, 454 · 0 · 5, `Ran 64 tests … OK`). No code path has changed since `9285fe75` (`git log -1 -- packages examples apps tests wpt Cargo.toml Cargo.lock`).]`

## Boundaries and surfaces
- No merge commit, no source edit, no dependency or lock change, no new crate, port, env var or listener. Arch §Occupied Resources is unchanged.
- Additivity holds trivially: nothing is merged, so nothing of ours is touched.
- Coupled dependency pins (CLAUDE.md Critical Warnings): untouched.

## Out of scope
- Any observation-model work (Snapshot model and later entries).
- Adopting upstream APIs. The `CARRY: upstream inline-geometry APIs` stays on Snapshot model (working-route line 39). It is not this entry's freight.
- Any merge of a later upstream sha. That goes to the next sync, or to an operator ruling at /implement if upstream moves before then.

## Folded freight and verdicts
- Freight on the taken-up entry: none (`route.py pins`: 0 blocks on line 37).
- CI verdict (Setup 5a, last wrap flip `47bbf38f` through HEAD):
  - `42b80ad9` green, checks 16/16, wall 491 s (CI#37495205882)
  - `47bbf38f` green, checks 16/16, wall 692 s (CI#37469322019)

  No red to disposition.
- Matrix: a sync makes no 0.1.0 capability verifiable, so the chunk links none. This is the same reading as the previous sync's scope.
