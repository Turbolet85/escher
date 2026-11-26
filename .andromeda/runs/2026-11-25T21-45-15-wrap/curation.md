# Curation — 2026-11-25-circle-drawer-virtual-nodes

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none new
  Tier 2 (.claude/rules/*):                   testing.md 2026-10-19 corrected in place (a correction, outside the cap)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 4 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 below threshold
  Extended: T1/CLAUDE.md: "2026-11-23: When a wrap's row sets only rescope a claim … still sweep the claim's old phrasings" + "run that sweep before the answer card, so the card lists every row it finds"
  No-other-home: "run the old-phrasing sweep before the answer card"
  CLAUDE.md size: 169/200 · T1 6.9 KB, 0 over 600 B

## Applied
- **Correction** — `.claude/rules/testing.md` `## Session Additions`, the 2026-10-19 entry (a local windows `cargo check --tests`), corrected in place with `[corrected 2026-11-26: …]` (TX 4052), on the escher overseer's wrap directive point 7 (inputs#I9).
  Proof: the local check cannot build on this host — aws-lc-sys needs `x86_64-w64-mingw32-gcc` (the report's Decisions & corrections, «Not run on this host», the P5 baseline). The CI windows job-log read with its unread-log guard is the witness: operator-pass entry 16 printed `snapshot 0` only after `Compiling blitz-tests v` was seen (`chunks/2026-11-25-circle-drawer-virtual-nodes/evidence/operator-pass.md` §Entry 16; CI#45700323454).
- **Extension** — CLAUDE.md `USER:session-learnings`, the 2026-11-23 entry, `Extended 2026-11-26` (TX 4053), within the Tier 1 cap (474 B). Confidence 0.8: verified by measurement +0.4 · specific detail +0.2 · no other home +0.2 (not on the route, in no master, rule or detector).
  Proof: this wrap's own step-2 sweep (`cascade-patterns.toml`, pattern THREEREADS, `sweep-pass-1.1.txt`) found security-plan `contract/stable-element-id/virtual-node-form` and test-plan `contract/element-id/virtual-form` after the answer card Q1 had been answered. They cost a second founder question, Q2 (`answer-card.md`, inputs#I12).

## Weighed and not written
Implement's *Decisions & corrections*:
- Revision 1, the tag spelled ` node=` (inputs#I7): not a learning — a decision; its home is design-system `decision/canvas-line-node-tag`, landed this wrap.
- The operator's word on hygiene's P1 row (inputs#I6): not a learning — a one-off ruling on one file.
- The plan's off-tree model (the grouping script put every command's centre at its circle's): rejected — task-specific (Filter 2).
- Driver clicks on Circle Drawer all land at the canvas's centre: rejected — duplicate (Filter 1); its home is test-plan `decision/circles-apart-written-through-attribute`, landed this wrap, and the cascade wrote it into `testing.md`'s body.
- Guard refusals (a `cd` into the skill's references directory, a `cat` heredoc), each re-issued once: not a learning — the guard put the call back on its path.
- A `cat > /dev/null;` call that blocked on stdin: rejected — one-off (Filter 4, 0.2).
- Recurrence of `testing.md` 2026-11-25 (sweep a help text's pins by its opening clause), met: not a learning — the entry worked.
- `testing.md` 2026-10-19's windows check not run on this host: corrected (above).
Implement's outcome files:
- «A scripted write fires no format hook» (the fmt red at `stand_circle_nodes.rs:563`): rejected — Filter 4, exactly 0.6 (measurement +0.4 · detail +0.2). Implement's own skill text already holds the rule, so the no-other-home signal does not fire.
- The sweep hazard of the line tag ` n=` sharing the glyph line's `n=` key: rejected — fixed by revision 1, so no hazard stands; the master row holds the reason.
The close's *Decisions & corrections after implement*:
- Whose word each answer is (inputs#I9): not a learning — the directive's ruling, carried by the sidecar entries' authorities.
- F1 B1 and F6 B not built here: not learnings — route CARRYs (P5).
- The deviations as measured: rejected — task-specific (Filter 2); the report holds them.
- Owners: carried by P5's pins; the `testing.md` correction is applied above.
- v040-11 stays unclaimed: not a learning — the ledger note (P7.3).
This wrap's conversation:
- The answer card's question dialog timed out, so the dialog fallback was taken: not a learning — the letter's designed path.
- The old-phrasing sweep caught two rows after the card: extended (above).
- A row restated past its 300-character bound twice, caught by `rows-check` before the fit wave: rejected — one-off (Filter 4).
The citation sweep's curation-home row:
- `.claude/docs/session-learnings.md:450` `host.rs:350`, out of range: no change — an illustrative example of a prefix match, not a claim about the code (`citation-dispositions.md`).
