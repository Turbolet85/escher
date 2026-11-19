# Curation — 2026-11-19-upstream-sync-ahead-of-the-debts

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a blitz-tests target prints no engine trace — locate with temporary prints restored by sha256" (confidence 0.8)
                                              + testing.md: "a check over a just-measured path pins the measured value" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 7 task-specific · 0 conflict · 0 deferred (cap not reached: 3 applied)
  No-other-home: "a blitz-tests target prints no engine trace …" · "a check over a just-measured path pins the measured value" · "git apply --3way records rerere preimages"
  Extended: T2/host-linux.md: "2026-10-27: `rerere.enabled` is on for this checkout …" + "`git apply --3way` during a merge records preimages too"
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

## Applied
- T2 testing.md, 2026-11-19 (tracing silent in a blitz-tests target) — signals: verified by measurement +0.4 · specific detail +0.2 · no other home +0.2 = 0.8.
  Proof: implement's Decisions & corrections, «Sweep hazard: under a test target with no tracing subscriber, and with blitz-dom's `tracing` feature off, `RUST_LOG=trace` prints nothing; the engine-side cause was located with temporary `eprintln!` lines, each file restored by sha256» (report rev 3; outcome-2026-11-19-upstream-sync-ahead-of-the-debts.md, Decisions & corrections).
- T2 testing.md, 2026-11-19 (a measuring check pins the measured literal) — signals: verified by a real red +0.4 · specific detail +0.2 · no other home +0.2 = 0.8.
  Proof: implement's Decisions & corrections, «a new check asserting what a path "should" do (the fragment mask fetching nothing) read red against the measurement; a measuring check pins the measured literal» — `tests/blitz-tests/tests/svg_mask_attribute_fetch.rs` pins the `data:` URL read (report, Spec claims disproved, W8).
- Extended T2 host-linux.md 2026-10-27 (rerere) — the additive facet `git apply --3way`; signals: verified by measurement +0.4 · specific +0.2 · no other home +0.2 = 0.8. A recurrence of a defect entry: logged `recurrence-despite-learning: host-linux.md 2026-10-27` in the handoff.
  Proof: implement's Decisions & corrections, «`git apply --3way` during the merge wrote rerere preimages to the shared rr-cache; cleared with `git rerere clear`, later applies under `-c rerere.enabled=false`».

## Every Decisions & corrections entry, dispositioned
Implement's:
- The operator held the host for another project (no cargo until «host free») — not a learning: a one-time host arrangement (Filter 2).
- inputs#I6 (four lock updates; locate the sniff failure first) — rejected, Filter 2: a ruling on this chunk; recorded as a deviation (inputs#I9).
- inputs#I7 (`--no-fail-fast` once; confirm on an upstream clone; fix in the engine with its own test) — rejected, Filter 2: this defect's ruling; the generalised rule a proposer offered was rejected at P2 (check 3).
- inputs#I8 (option 1; prove no move; W8 recorded) — rejected, Filter 2: task-specific.
- W8 «a mask round the stamping constructor: stop» — measured not round it — rejected, Filter 2: the fact landed in security-plan decision/svg-mask-url-fetch.
- rerere recurrence via `git apply --3way` — extended (above).
- Guard refusals, each re-issued once (two `cd`s, one pipe into `sed`) — not a learning: the guard working (`guard-refused`); the `cd` arm is a recurrence of host-linux.md §Paths, logged in the handoff.
- Sweep hazard: tracing silent in a test target — applied (above).
- Sweep hazard: a «should» check reads red against a measurement — applied (above).
After implement's:
- inputs#I9: W1–W9 the founder's rulings, one stop on their card — not a learning: this wrap's direction, carried out (Q1).
- inputs#I9: the founder away → `founder-forks.md` — not a learning: the contract's away form already states it.
- inputs#I9: a guard refusal re-issued once in its spelled form — not a learning: the operator's standing line, recorded in the previous wrap's handoff; this wrap re-issued three refusals once each (a pipe into `head`, a `cd` into the run dir, a `cat` heredoc to a file).
- inputs#I9: no hand edit of a leaf — rejected, Filter 2: this wrap's direction; its effect (stack.md's three stale figures left) is in `cascade-dispositions.md` and the handoff.
- The pass took the merge commit by `git add -A` with `MERGE_HEAD` standing — not a learning: a fact of the pass, as the plan's step 12 read.

## Citation rows (P2's `citation-dispositions.md`, curation home — P3)
- `.claude/docs/session-learnings.md:328` `render.rs:805-812` (moved) — no change: an illustrative example of a count pattern matching a range, not a claim about `render.rs`.
- `.claude/docs/session-learnings.md:363` `host.rs:350` (out of range) — no change: an illustrative example of a prefix collision, not a claim about `host.rs`.
