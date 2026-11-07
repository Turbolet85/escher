# Curation — 2026-11-07-mounteddata-buffered-render (wrap 2026-11-07T12-33-28)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A regression test ported from upstream across a dependency's major version is a fix's witness only once it reads red with the fix reverted …"
    Proof: escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/mutation-control.md — with `poll`'s borrowed render restored, upstream's ported `task_polled_during_render_can_access_document` read `ok` (`1 passed; 0 failed`) while the probe's two `…_spawned_acts_*` pins read FAILED, `RefCell already borrowed` (`8 passed; 2 failed`); it falsified the plan's Goal, its first acceptance criterion and test-plan fct-e2zfwpza. Score: measurement +0.4 · specific detail +0.2 · no other home +0.2 (the masters now carry this chunk's witness facts, not the general rule) = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 below threshold
  No-other-home: "A regression test ported from upstream across a dependency's major version is a fix's witness only once it reads red with the fix reverted"
  CLAUDE.md size: 153/200 · T1 4.3 KB, 0 over 600 B

Decisions & corrections, each weighed:
- The operator's ruling on the witness premise (inputs#I3) — not a learning: a ruling, landed in the masters at P2.
- The mutation control's finding (a ported test insensitive on 0.7) — applied, Tier 2 testing.md (above).
- The CI-log attribution hazard (`Running` lines and `test result:` lines out of order) — rejected, Filter 4: measurement +0.4 · specific detail +0.2 = 0.6, and its home is now a master (test-plan ci-integration, amended at P2), so no conditional signal applies.
- The guard refusal of a `cd` into the skill's references — not a learning: a hook refusal that cost its one re-issue.
- The operator running the pass by hand (inputs#I3) — not a learning: a one-off direction.

This window's candidates:
- A check-5 raise whose target no proposal names fails the apply writer's OWN_FILE — not a learning: a pipeline defect, recorded in the friction stream (2026-11-07T13:33:54Z-c).
- A `· as:` direction that named the limit's owner by its origin marker put provenance into four master facts — rejected, Filter 4: gate failure +0.4 · specific detail +0.2 = 0.6; recorded as friction (2026-11-07T13:33:54Z-e).
- The leaf comparers leaving the 14 harness owner sites the sweep printed — not a learning: a pipeline miss, recorded as friction (2026-11-07T13:33:54Z-g).
