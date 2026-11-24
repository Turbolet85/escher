CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A guard hook's refusal of a write the step needs is a stop …" (confidence 0.7) · + "Once the operator pre-CI commit stands in HEAD, a fix the pass needs lands as a fix commit inside the operator pass …" (confidence 0.8)
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup (homes in a master) · 3 task-specific · 0 conflict · 0 deferred
  Extended: T1/CLAUDE.md: "2026-11-18: A docs leaf states no per-file test count …" + "a comparer's finding whose leaf-text keeps such a count re-pins it — read every finding against the report's moved counts and decline one that does" (confidence 0.8)
  No-other-home: "a comparer's finding whose leaf-text keeps a stale count re-pins it"
  CLAUDE.md size: 169/200 · T1 6.8 KB, 0 over 600 B (health.py check 1)

## Applied
- T1 «A guard hook's refusal of a write the step needs is a stop …» — explicit user correction +0.4, «next time» language +0.3 = 0.7.
  Proof: the operator's ruling in the pass, after the write guard refused Edit-tool writes under `target/tmp/probe-circle-digest` and the edit was applied by a script: «A hook refusal is not routed around by a script: next time a guard refuses a write the step needs, stop and tell me first.» (report `## After implement` → Decisions & corrections after implement, item 3; `evidence/operator-pass.md`, *The operator's words*); restated by the wrap directive point 7 (`inputs#I5`).
- T1 «Once the operator pre-CI commit stands in HEAD … fix commit inside the operator pass …» — operator ruling +0.4, verified by measurement (the guard's real refusal) +0.4 = 0.8.
  Proof: `/andromeda-phase`'s pending guard refused the revision with `32a0354d1` in HEAD's history; the operator ruled «(1): the fix commit inside the operator pass, the guard stands», and `276d5d127` landed so, green on CI#45305097481 (`evidence/operator-pass.md`, *The operator's words* and §6).
- Extended T1 2026-11-18 entry — verified by measurement +0.4, specific technical detail +0.2, reached no other durable home +0.2 = 0.8.
  Proof: the whole-mode comparer `a11y-plan-1`'s finding `a11y-summary.md:25#1` kept «Circle Drawer 1» while the report's Counts / qualifiers moved reads `stand_non_lean_tasks` `MEASURED` and `CLICKABLE` Circle Drawer 1 → 0; it was declined at apply (`cascade-dispositions.md` §Leaves).

## Decisions & corrections — dispositions
Implement's:
- Bash guard refused `cd` into the skill's references (twice) and a `cat` heredoc — not a learning: the guard enforces it and its one re-issue passed.
- The operator stopped implement and revised the plan (gate 9 `4+0`) — rejected, task-specific (Filter 2).
- Run 2026-11-23T22-16-09-implement: the guard refused a `cd` once — not a learning, as above.
- The canvas's client rect is its border box while a widget paints in its content box — rejected, dup: its home is architecture `decision/widget-click-point-content-box`, landed this wrap (tx 3479).
After implement's:
- Measure a CI red's cause on a throwaway `build/` branch before any fix; no narrowing, no per-host digest — rejected, Filter 4: operator direction +0.4, detail +0.2, could be task-specific −0.2 = 0.4 (a direction for this pass's red).
- Adopt the snap only if macOS and Windows read it equal — rejected, task-specific (Filter 2).
- The guard refusal routed around by a script; «stop and tell me first» — applied, Tier 1.
- The pending guard and the fix commit inside the pass — applied, Tier 1.
- The CPU renderer's Apple glyph-origin snap; canvas text byte-equal only on whole device pixels — rejected, dup: its home is test-plan `decision/canvas-text-on-whole-device-pixels` (provisional, tx 3483), and `rules/testing.md:36` now carries it (cascade).
- The wrap directive's whose-word assignment — rejected, task-specific (Filter 2).
This wrap's conversation:
- The comparer finding that re-pinned a stale count — extended in place (above).
- Proposers offering one fact in up to five masters — not a learning: Validate check 2 handles it (a friction record of the reconcile checkpoint).
- Citation sweep row `.claude/docs/session-learnings.md:433` `host.rs:350` — routed to P3 by `cites.py dispositions`: an illustrative prefix example in a lesson, not a claim about the file — no change.
