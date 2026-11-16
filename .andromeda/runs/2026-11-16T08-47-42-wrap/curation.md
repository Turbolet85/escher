CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "Never push to answer a CI red while that run is still going — a push cancels it and hides its later reds; wait until it completes, read every failed job, and fix all its reds in one commit." (confidence 1.0)
    Proof: the operator's direction in the pass (operator-pass.md, Corrections and rulings: «do NOT push until CI run 43825097857 has COMPLETED …»); measured — CI#43822370596 on f1fd8c600 was cut by the next push before its Documentation job ended, hiding the rustdoc red CI#43825097857 then read. Signals: user correction +0.4 · «do NOT» +0.3 · verified by measurement +0.4 → 1.1, capped 1.0.
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "Rustdoc runs only in the doc leg" (confidence 0.8)
    Proof: CI#43825097857, Documentation job 133160264057 red on `[`capture`]` at packages/escher-telemetry/src/lib.rs:24, present since f1fd8c600 and green in every local gate; `ci-leg.sh doc` exit 101 before, 0 after (operator-pass.md). Signals: verified by measurement +0.4 · specific technical detail +0.2 · no other home +0.2 (the test-plan add was rejected not_master at fit; no route annotation, no master row) → 0.8. Tier 3: no rule file's `paths:` cover crate doc comments.
                                              + "Reproduce a CI red on the host before fixing it" (confidence 0.7)
    Proof: the operator's direction (operator-pass.md: «First REPRODUCE the red on this host before changing anything … No timing sleeps, no retry»); the forced-order reproduction read `repro: the recorder saw 0 command spans` and overturned the rebuild-the-cache fix (red) before fix B landed (green). Signals: user correction +0.4 · «no … no» +0.3 → 0.7. Tier 3: not path-scoped, multi-sentence.
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "Rustdoc runs only in the doc leg"
  CLAUDE.md size: 165/200 · T1 4.9 KB, 0 over 600 B

Dispositions of *Decisions & corrections* (implement's, then After implement's):
- implement · the operator ruled framework targets at warn and error (inputs#I5) — not a learning: amended into obs-plan `decision/capture-framework-targets-warn-up` at P2 (the master owns it).
- implement · the Err case on its own session (inputs#I6) — not a learning: amended into test-plan `decision/root-boundary-err-on-own-session` and security-plan `decision/root-boundary-err-as-screen-content`.
- implement · sweep hazard, the capture under tracing-core's one-dispatcher mode — not a learning: amended into architecture `decision/capture-dispatcher-pair` and `decision/capture-before-own-recorder` (the master owns it).
- implement · sweep hazard, the Dioxus framework traces from four crates — not a learning: the target list stands in obs-plan `decision/capture-framework-targets-warn-up`.
- implement · guard refusals (cd into the registry, a cat heredoc) — not a learning: each cost its one re-issue, the guard working.
- after implement · reproduce first, fix the cause, no sleeps or retry — applied, Tier 3.
- after implement · fix B, keep the twin with a comment — not a learning: the code holds it and architecture `decision/capture-dispatcher-pair` states why.
- after implement · no push until the run completes, every red in one commit — applied, Tier 1.
- after implement · sweep hazard, no `doc` entry and no rustdoc in `fast` — applied, Tier 3.
- after implement · sweep hazard, `ci.py conclusion` stops at the first failed check — folded into the Tier 1 entry (its mechanism clause).
- after implement · the founder answered F1 (inputs#I7) — not a learning: landed as security-plan `decision/caused-answer-crossing` on Q1 (inputs#I9); the text follow-up is the route's (P5).
- after implement · the founder chose `layout --id` for v030-11 (inputs#I7) — not a learning: the route's («Layout on the carriers», P5).
- citation sweep · `.claude/docs/session-learnings.md:336` `host.rs:350` out of range — no change: an illustrative prefix example in a past entry, not a claim about `host.rs` (citation-dispositions.md).
