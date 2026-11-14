# Session Handoff

**Last Updated:** 2026-11-14T07:46:00Z
**Branch:** build/escher-0.3.0 · 0 ahead of origin/build/escher-0.3.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-11-14-seeded-randomness — the chunk's wrap commit (P7)

## Position
- Done: `2026-11-14-seeded-randomness`.
  - dioxus-native-dom holds an ungated `seed` module: `SeededRandom` (splitmix64, one shared stream per document), `use_seeded_random`, `DEFAULT_SEED` = 0.
  - `Harness::from_vdom` provides the default seed where the builder gives none, so two runs of one seed read the same state at every step.
  - `matrix#v030-13` verified. CI is green.
- Next: run `/andromeda-phase` to promote and plan «Bundled fonts», the next markerless entry of Epoch 4.

## Work done
- The wrap resumed at P2 from its P1 report in `.andromeda/runs/2026-11-14T07-12-40-wrap/`. The resume's relay was snapped as I5 and carried in Addendum 1.
- 10 row operations landed in masters TXs 796–800, and 12 leaves were re-derived (2 by hand). The record is that run dir: `fanout-results.md`, `cascade-dispositions.md`, `curation.md`.

## Drift resolved
- New rows:
  - architecture: the `SeededRandom` and `use_seeded_random`/`DEFAULT_SEED` contracts, the harness's run seed, `decision/one-run-seed`, `decision/no-prng-crate`;
  - architecture: `decision/no-seed-on-carriers`, PROVISIONAL — F1's answer A;
  - security-plan: the seeded source takes its seed from in-process code only.
- Rows changed:
  - architecture `decision/accessibility-feature-gating`: `seed` joins the ungated modules;
  - test-plan `from-vdom`: it provides the default run seed;
  - obs-plan `decision/checks-silent`: a re-run child prints ids or digests.
- Fit rejected 5 adds as `not_master`, three of them plan-expected amendments: the re-exec spawn in Occupied Resources, the seed in the silence census, and the seeded-runs check in test-plan §5. Nine duplicate adds were rejected so each fact keeps one home. No escalation.
- Route: residual `res-jqbadec2` (page-script `Math.random` unseeded, target next), with its line in `escher-0.4.0-incubator/deferred-from-0.3.0.md`. No tail edit.

## Notes
- F1 of this chunk (no seed on the driver's carriers) is still the founder's: the absence row stands PROVISIONAL. On his B, card 3 row 1's entry «Seed on the driver's carriers» goes before «Auto-exploration».
- F1 of `2026-11-13-local-files-for-local-documents` is still the founder's: «Remote pages and local files in the browser» stands PROVISIONAL.
- The 2026-10-06 precedent of registering a test-binary re-exec spawn in arch Occupied Resources does not hold: fit reads it `not_master` (the test code holds it).
- Still standing on the operator's word, for removal at his call:
  - the worktrees `target/tmp/trial-det` and `target/tmp/trial-a`;
  - their build dirs `target/trial` and `target/trial-a`;
  - the trial logs.
  - `target/tmp/trial-wt` is the founder's, and git marks it `prunable`.
- `host-linux.md` is 9 template lines behind (U04 `noted`). Regenerating it is the operator's word.
- Last failed command: none.

## Session End Status
Completed normally at 2026-11-14 09:42:31
