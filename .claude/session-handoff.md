# Session Handoff

**Last Updated:** 2026-11-19T15:20:00Z
**Branch:** build/escher-0.4.0 · 0 ahead of origin/build/escher-0.4.0 as read at this wrap's Setup (HEAD `70eb94880`, the operator pre-CI commit)
**Status:** clean — the chunk commit and push are this wrap's P7
**Last Commit:** chore(2026-11-19-build-directory-held): operator pre-CI commit (the wrap commit follows; see `git log -1`)

## Position
- Done: 2026-11-19-build-directory-held — the build directory's growth measured and mended. It is the one entry of 0.4.0's Epoch 1 «Foundation», so this wrap closes that epoch.
  - The cause: cargo mints a unit hash per feature resolution × lockfile × toolchain epoch, never per edit. A one-crate lock move added 184 test-binary hashes, +36.49 GB.
  - The mend: `scripts/target-sweep.py`, run by `ci-leg.sh fast` after a green verdict on a dev host, skipped under `CI`. A replayed day grew +57.90 GB unmended and −2.04 GB mended.
  - v040-01 implemented with its ref; P7 flips it to verified.
  - CI#44537789372 reads green 17/17 on `70eb94880`.
- Next: Epoch 2's head, «Upstream sync ahead of the debts» (`/andromeda-phase`), after the Epoch 1 diagnosis if P7's nudge names it.

## Work done
- Report closed (rev 4): `## After implement` and `## New text, by line`. Input snapped: I4, the wrap directive.
  - The close states the step 2 and step 9 controls in substance as deviations, both read by property.
  - It records the `CI=true` rebuild as a measured limit.
- Masters: 24 operations in tx 2343–2347.
  - security-plan 4: argv, no env read, the `CI` env value, locked-builds.
  - architecture 12: the fast contract and pre-push gate, embolden, the Windows entries, the state areas, exit codes 0–3, three names.
  - obs-plan 5: the sweep's stdout line and its keys.
  - test-plan 3: the pre-push gate, the sweep safe around a build, the build directory held by a sweep.
- Leaves: whole mode (the epoch closes), 15 comparers, 27 findings. 16 applied, 10 declined as detail, and 2 written by hand through `harness land`: the CLAUDE.md tree line, and a stale `tests-summary.md:21` that the comparer missed.
- Figure removal (inputs#I4): 96 code-stated test, file and target counts removed from 13 docs leaves (tx 2362–2374). Run readings were kept. Log: `.andromeda/runs/2026-11-19T14-42-20-wrap/curation.md`.
- Curation: T3 extended (the `<user>` placeholder now covers every control record a phase or implement run writes or quotes).
- Route: CARRY on «Dependency and CI debts» (tx 2377), for the `CI=true` rebuild under new hashes (about 48 GB, mechanism not found).

## Drift resolved
- 80 proposals and 4 of mine. Applied 24, rejected 44, escalations 0.
  - Rejected under check 2 (one fact twice across masters): 40. The sweep's exit codes were offered in 5 masters, its stdout line in 4, its lock refusal in 5. Each keeps one home, listed in `fanout-results.md`.
  - Rejected under check 7 (`not_master`): 4 — the keep set's construction, a no-log decision, local-baseline's figures, and the `CI=true` limit (the CARRY holds it).
- Fit: 24 fits.

## Notes
- The Bash guard refused three of my calls. The first, a `cd` into the run dir, stopped the wrap on the directive. The operator then narrowed the line: a refusal the guard spells is re-issued once; stop only when a wrap step has no accepted form. The other two (a pipe into `head`, a `cd` into the skills dir) were each re-issued once.
- Deferred learnings (Filter 5: the directive named one entry):
  - the sweep hazard: cargo 1.99's JSON names a build-script unit by its directory, an example by its hash-less uplift;
  - the operator's guard line above.
- Curation recurrence: `recurrence-despite-learning: host-linux.md §Paths` (a `cd` that moves the cwd).
- No tool runs a «figure sweep». This wrap ran its own search; the friction record names it.
- Carried from earlier wraps:
  - the CARRY leaf lines' test-file lists are the pre-split ones;
  - residual `res-h7acz56c` is route's to flip;
  - security-plan `dec-kxrtstbq`'s `verified_was` evidence still quotes `Cargo.lock:8040-8041`;
  - `host-linux.md` is 9 template lines behind (U04 `noted`).
- Left for the operator to remove:
  - `target/p5-baseline-debug` (101 GB, renamed aside at implement);
  - `target/cold-agent/` (`bash scripts/cold-agent.sh cleanup`);
  - `target/tmp/smoke-order-1`, `target/tmp/trial-pas` and `target/tmp/escher-session.head`;
  - implement's scratch clones `…/scratchpad/guardclone` and `…/guardclone-impl`.
- Last failed command: none.
