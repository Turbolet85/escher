# Founder forks — 2026-11-20-stand-test-setup-shared (phase P4)

The founder is away (the operator's directive, `inputs#I1`). Each fork below is his; the least-change option stands
in the plan meanwhile, and nothing written presumes another answer.

## F1 — the upstream-taken tests' duplication

**What was measured.** The code audit's own duplication recipe (jscpd 5.4.0, the 0.3.0 Epoch 6 record's command and
summarizer, reproduced exactly on the two audit boundaries' archives) reads, at this chunk's base `1b0125c10`:
pct **4.05** (6,722 of 166,122 lines), where the 0.3.0 audits read 3.80 → 3.81 → 3.89. The stand checks' own share
is unchanged since the 0.3.0 Epoch 6 audit (75 pairs / 870 lines). The whole rise past 3.89 % came with the
2026-11-19 upstream sync (`aedc67c60`, upstream `547d7514b`): 55 clone pairs in
`packages/blitz-vibey-cdp-server/tests/` (1,090 lines) and `packages/blitz-vibey-script/tests/` (132), upstream's own
test files, taken as written.

**The fork.** This chunk folds the stand checks' repeated setup and brings *their* reading back under its 0.3.0
Epoch 4 value (v040-08's acceptance names "the stand tests' duplication"). The whole reading the next boundary audit
prints will still carry upstream's 55 pairs. Either:

- **A — keep them as upstream's (least change; stands meanwhile).** The fork never edits an upstream-taken test file;
  the next audit reads the whole pct with upstream's share named apart, and v040-08's duplication leg is judged on
  the stand checks' reading alone.
- **B — fold them in the fork.** A later chunk de-duplicates upstream's cdp-server and vibey-script tests: a standing
  divergence from upstream in files every upstream sync touches, re-conflicting at each take.

**Where it rests until answered.** A `## Beyond the plan` row of this chunk's P5 card proposes a `WATCH:` on
«Codebase health» (the v040-08 entry whose acceptance already takes "kept by the founder's word") carrying the whole
reading and upstream's share, so the answer lands with the work that judges the cap.
