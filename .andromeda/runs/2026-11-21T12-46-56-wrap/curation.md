# Curation — 2026-11-21-engine-debts

CLAUDE.md ecosystem: no new learnings this session.
  Tier 1 (CLAUDE.md USER:session-learnings): none
  Tier 2 (.claude/rules/*): none
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 5 dup · 1 task-specific · 0 conflict · 0 deferred
  CLAUDE.md size: see the `check 1` row of this run's `health-2026-11-21-engine-debts.json`

## Decisions & corrections — implement's
- The operator's stored-waker ruling (`inputs#I5`) — rejected, Filter 1 (dup): a master row holds it, test-plan `decision/render-wait-by-stored-waker` (TX 2891), and the cascade carried it to `.claude/rules/testing.md:16` and `.claude/docs/tests-summary.md:37`.
- The founder's Tab ruling (`inputs#I6`) — not a learning: a ratified engine decision, held by a11y-plan `decision/tab-and-shift-tab-move-focus` and `decision/focus-within-hidden-holder-limit` (TX 2889).
- The test hazard (a not-focused assertion holds either way after the fixup) — rejected, Filter 1 (dup): test-plan `decision/focus-test-pins-where-focus-lands` (TX 2891), cascaded to `.claude/rules/testing.md:23` and `.claude/docs/tests-summary.md:52`.
- The dioxus-core livelock hazard — rejected, Filter 1 (dup): architecture `decision/bridge-buffered-render`'s except (TX 2887), cascaded to `.claude/docs/services/dioxus-native-dom.md:31`.
- A count pinned as a literal in a CI-script test — rejected, Filter 1 (dup): test-plan `decision/count-and-rename-sweeps` (TX 2891), cascaded to `.claude/rules/testing.md:22`.
- Guard refusals (a `cd` out of the cwd; a `cat` heredoc with a file target) — rejected, Filter 1 (dup): `.claude/rules/host-linux.md` §Paths and §Transports state both; each refusal cost its one re-issue, so no recurrence.
- `gate.py control` refusing a 17-character control id — rejected, Filter 2 (task-specific): the tool's own refusal states its 1–16 form.

## Decisions & corrections after implement
- none in the pass record. The wrap's directive (one card for the founder's engine-change answers) — not a learning: Tier 1's 2026-10-10 entry already states the one-card halt.

## The conversation
- The citation sweep's one row (`citation-dispositions.md`): `.claude/docs/session-learnings.md:405` `host.rs:350`, out of range — not a learning and no edit: the line uses `host.rs:350` as an example of a prefix match, not as a citation.
- This wrap's guard refusals (a `cd` out of the cwd, a pipeline tool piped into a filter), each re-issued once — rejected, Filter 1 (dup): `host-linux.md` and the 2026-10-13 Session Addition state both.
