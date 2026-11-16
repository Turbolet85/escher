# Session Handoff

**Last Updated:** 2026-11-16T21:15:00Z
**Branch:** build/escher-0.3.0 · 0 ahead of origin/build/escher-0.3.0 as read at this wrap's Setup (HEAD `151261549`, the operator fix)
**Status:** clean
**Last Commit:** 2026-11-16-error-text-in-the-answer — Error text in the answer: `caused` carries each record's text from every source, scrubbed by field name and bounded (session 107, see `git log -1`)

## Position
- Done: 2026-11-16-error-text-in-the-answer, wrapped in `.andromeda/runs/2026-11-16T20-37-56-wrap/`. An acting answer's `caused` now carries one item per panic, boundary error and record, with message, location, boundary, cause chain and scrubbed fields. Each text is cut at 1024 bytes, 16 items are kept and the rest counted in `caused_past`. The capture admits engine records at warn and above and installs a chaining panic hook. Every error boundary is read through `boundary_marks()`. `password` joined `CONTENT_FIELDS`. The operator pass fixed one Windows red (CI#43940101161, the MCP comparison now decodes the text whole); CI#43946700868 green, 17/17. No capability claimed.
- Next: `/andromeda-phase` to promote and plan «Error field in the answer» (`chk-42b4r2a4`), the founder's follow-up, now Epoch 5's head.
- Epoch 5 runs: «Error field in the answer» → «Layout on the carriers» → «A component in isolation».

## Work done
- Masters, tx 1518–1522, 38 row operations: architecture 10 (caused, caused_past, capture bounds, `boundary_marks`, `subscriber`, `TRACING_COMPILED`, the hook exception), security-plan 10 (`caused-text-crossing` and `root-boundary-err-crossing` replace their closed-words parents; `password` in the scrub lists; the mask exception for an author's message), obs-plan 12 (`scrub-by-field-name` replaces `scrub-at-sink-not-call-sites`; engine sites write fixed messages; the capture admits the engine), test-plan 6.
- Leaves: 48 findings applied over 13 files (CLAUDE.md, rules security/observability/testing, six docs, four service docs).
- Curation: no new learnings (6 duplicates); 2 recurrences in Notes.
- Route, tx 1538–1539: «Error field in the answer» inserted after this chunk on the founder's word; CARRY `cry-c4emakc4` on «A component in isolation» (the untested `boundary_marks()` fallback); residual `res-k2zggoqx` (the registry that only grows, target next), mirrored into `escher-0.4.0-incubator/deferred-from-0.3.0.md`.

## Drift resolved
- 38 row operations landed (37 proposed, 1 raised by the wrap: `no-screen-content-in-logs`). 1 escalation: the widening card Q1 (14 operations), resolved apply on the founder's «Применить все 14 (Recommended)», relayed by the overseer (inputs#I5).
- 24 proposals rejected: 22 as one fact offered by several masters (one home kept), 1 false (a11y `password-never-revealed`: a caused message can hold typed text), 1 `not_master` at fit (obs-plan's capture hook row). Citations: 0 re-pointed.

## Notes
- recurrence-despite-learning: the Tier 3 entry «The rustfmt write hook leaves a crate root unformatted…» (Extended 2026-10-06: a scripted edit fires no hook). Implement's python-replace edits read a rustfmt red at `fast`.
- recurrence-despite-learning: the Tier 3 entry on a build capture kept in a run dir. Research kept four probe logs holding home paths under `evidence/`, refused at hygiene and moved to the scratchpad (inputs#I3).
- `decision/layout-read-in-process-only` still reads `state: provisional` until «Layout on the carriers» lands.
- Open (carried): security-plan `dec-kxrtstbq`'s `verified_was` evidence still quotes `Cargo.lock:8040-8041` and a registry source with its checksum. It waits for a masters re-verification.
- `host-linux.md` is 9 template lines behind (U04 `noted`). Regenerating it is the operator's word.
- Last failed command: none.
