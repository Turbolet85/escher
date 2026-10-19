CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "Code compiled only off unix is unverified on this linux host until a non-unix build reads it — run `cargo check --tests --locked --target x86_64-pc-windows-gnu -p {crate}` before pushing a moved `cfg(unix)` gate" (confidence 0.8)
    Proof: two consecutive Windows CI reds, CI#39199277925 (an ungated recorded `mcp_log` test) and CI#39201074261 (`the_check` calling the unix-gated `mode` after a file-wide gate was split), and the windows-gnu `cargo check --tests` reading exit 0 on escher-driver and seven_guis after fix 2 (report.md, Decisions & corrections; evidence/operator-pass.md §Fix round 2). Signals: verified by measurement +0.4, specific technical detail +0.2, load-bearing for the next entry +0.2.
  Tier 3 (.claude/docs/session-learnings.md): + "An in-line edit beside a provenance parenthesis reads as new provenance" (confidence 0.8)
    Proof: this wrap's classify trail (`classify-2026-10-19-session-recording.json`), its first architecture check `VERDICT: FAIL provenance` with 5 spans whose counts were equal at base and now, PASS after three edits were re-shaped to keep their 8-token anchors. Signals: verified by measurement +0.4, specific technical detail +0.2, reached no other home +0.2.
  Filters: 2 dup (→ recurrence, handoff) · 3 rejected (one-off / not a learning) · 0 conflict · 0 deferred
  Load-bearing: "Code compiled only off unix is unverified on this linux host until a non-unix build reads it" → Recorded-check replay
  No-other-home: "An in-line edit beside a provenance parenthesis reads as new provenance"
  CLAUDE.md size: see P7 (health check 1)

Decisions & corrections, each weighed:
- The founder's word (inputs#I4) → not a learning: a ruling, carried by P2's amendments (ratified) and P5's CARRY on «Recorded-check replay».
- The operator's direction for the operator pass → not a learning: a direction for that pass, followed.
- The operator's direction after the first red → not a learning: its remedy is the T2 entry's fact, applied there.
- The operator's direction after the second red → not a learning: superseded for the target by inputs#I4.
- Learning candidate: off-unix code unverified until a non-unix check → applied (T2 testing.md).
- Learning candidate: `gh api …/logs` needs `--allow-escape-sequences` → rejected, Filter 1 duplicate of the T3 entry «2026-10-11 — `gh api` refuses a job's log that holds terminal escapes» — a DEFECT record → recurrence-despite-learning, handoff.
- Sweep hazard: bare `\b134\b` matched line-range citations → rejected, Filter 1 duplicate of the T3 entry «2026-10-12 — A sweep pattern for a bare count also matches a citation's digits» — a DEFECT record → recurrence-despite-learning, handoff.
- Guard refusals (`cd` ×4, a `cat` heredoc, a recursive `rm`) and this window's two (a `cd` into the skill dir, a tool piped into `tail`) → not new: host-linux «Paths & the working directory» and the 2026-10-13 run-bare rule recurred → recurrence-despite-learning, handoff (each guard held at one re-issue).
- Implement self-corrections (a heredoc nesting triple quotes; the foreign-temp removal a unit test caught) → rejected, one-off (−0.3), the second already a code fact.
- The delegate frame → not a learning: the record of who answered.
- S1 / S2 → not learnings: dispositioned in Spec claims disproved and Deviation 11.
- This window: the subject line, a comment line, is `preserved` by `cascade.py replace` and takes the Edit → rejected, one-off and the tool's documented row word.
- This window: the classify provenance anchor → applied (T3).
