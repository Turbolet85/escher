# Scope — 2026-10-07-upstream-sync-driver-core

**Working entry (verbatim):** Upstream sync ahead of the driver core — upstream/main merged, our changes kept mostly additive; our tests and CI prove our logic survived (per intent §Principles)

**Intent anchor:** `escher-0.1.0/intent.md` §Principles, "Upstream stays in reach": an "Upstream sync" chunk at each epoch boundary merges `upstream/main`, and our tests and CI prove our logic survived. Small and often, with our changes mostly additive (founder, 2026-10-06).

## What it builds
- **A measured no-op sync.** At take-up, `git ls-remote upstream refs/heads/main` reads `23354585` ("Update Taffy for cyclic percentage grid minimum contributions (#1070)", committed 2026-10-06T01:33:46Z). That is the sha `2026-10-06-upstream-sync-element-identity` merged and `2026-10-06-upstream-sync-observation-model` measured unchanged. `git merge-base HEAD upstream/main` = `23354585`, so upstream is **0 commits** past the merge base and ours is 43 past it (inputs#I1). There is nothing to merge. Re-measured at P3 (2026-10-07T04:31Z): `git ls-remote upstream refs/heads/main` = `23354585`, `git merge-base --is-ancestor 23354585 HEAD` exits 0, and `.andromeda/architecture.md:202` names the same sha as "the next sync's merge base".
- The shape is leaned, not asked: the founder ruled the same empty sync a measured no-op one epoch boundary ago (`escher-0.1.0/chunks/2026-10-06-upstream-sync-observation-model/inputs/I2-relay-2.md.txt` — "record the measured upstream pin and the 0-ahead read, no merge commit, and no work beyond what the gates require"). That ruling was given for that entry; this scope applies it to the identical measurement, and the P5 review card is where the operator confirms or rejects the lean.
- What the chunk delivers: the measurement, re-read at /implement, showing that `upstream/main` is still `23354585` and still an ancestor of HEAD, and the gate verdict on the unchanged tree. The lean covers 0 ahead only. If upstream has moved by then, that is a halt for the operator, never a silent merge: a non-empty merge is a different chunk shape.
- Proof that our logic survived: fork CI green on HEAD `8d156de1`, plus both local legs RUN, not deferred — `bash .github/scripts/ci-leg.sh fast` and `bash .github/scripts/ci-leg.sh doc` (CLAUDE.md Critical Warnings: "Work is not done until … pass"; the previous sync's scope corrected a "deferred for zero source delta" premise to this reading). `[premise-corrected: "the CI verdict on 8d156de1 is not yet available, so the proof is owed, not held" was the take-up reading. P3 re-read the same run settled: `8d156de1fcce verdict: green · checks 16/16 · wall 414 s · runs CI#37571032838 completed/success` (`ci.py conclusion --sha 8d156de1…`, 2026-10-07T04:31Z), the 16 checks including "Dependency audit" and "Accessibility (a11y) tests" (`gh api repos/Turbolet85/escher/commits/8d156de1…/check-runs`). No code path differs between that commit and the tree: `git log -1 --format=%h -- packages examples apps tests wpt Cargo.toml Cargo.lock` = `4e90f108`, and `git diff --stat 4e90f108 8d156de1` over those paths plus `scripts .github deny.toml` is empty. The CI half of the proof is held; the two local legs are still /implement's to run.]`

## Boundaries and surfaces
- No merge commit, no source edit, no dependency or lock change, no new crate, port, env var or listener. Arch §Occupied Resources is unchanged.
- Additivity holds trivially: nothing is merged, so nothing of ours is touched.
- Coupled dependency pins (CLAUDE.md Critical Warnings): untouched.

## Out of scope
- Any driver-core work (Driver session and the later Epoch 4 entries).
- Any merge of a later upstream sha. That goes to the next sync ("Upstream sync ahead of agent surfaces"), or to an operator ruling at /implement if upstream moves before then.
- The handoff's note for a non-empty merge — three lines appended to the tail of the inherited `packages/dioxus-native-dom/src/dioxus_document.rs` (the test module's declaration) are the only new lines of ours in that file — is not exercised here; it stands for the next sync that merges something.

## Folded freight and verdicts
- Freight on the taken-up entry: none (`route.py pins`: 0 blocks on line 52; `route.py markerless`: freight 0).
- CI verdict (Setup 5a, last wrap flip `8d156de1` through HEAD — the one sha):
  - `8d156de1` **verdict not yet available** — in progress at 2026-10-07T04:23Z, checks 4/4 so far, oldest running check "Test [default features]" at 146 s (CI#37571032838). Not a green and not folded as one.

  - P3 re-read (2026-10-07T04:31Z): `8d156de1` green, checks 16/16, wall 414 s (CI#37571032838).

  No red and no `not green` to disposition at take-up. The handoff records the pre-CI commit `4e90f108` green (CI#37568558201); that run is below the flip and was not read here.
- Matrix: a sync makes no 0.1.0 capability verifiable, so the chunk links none. This is the same reading as the two previous syncs' scopes.
