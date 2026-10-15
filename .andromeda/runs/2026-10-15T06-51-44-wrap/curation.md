# Curation — 2026-10-14-quality-gates (wrap 2026-10-15T06-51-44, resumed)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + security.md: "A listener or env read that only a test makes … is registered as test-only … before it lands" (confidence 0.9)
    Proof: the operator's answer «Register test-only» to this wrap's escalation (architecture 6–8, security-plan 12; fanout-results.md, Escalate — resolved), with the note that such a bind is not a crossing and is registered as `PAINT_TREE_BENCH_HTML` and `CARGO_TARGET_TMPDIR` are. Signals: an explicit operator direction +0.4, categorical language ("is not a crossing, so it is not the founder's") +0.3, specific detail +0.2. security.md carries no `paths:` — judged by the Tier-1 bar: one sentence, 362 B.
  Tier 3 (.claude/docs/session-learnings.md): + "In a master sweep, `flake` reads Nix and `repeated` reads everything" (confidence 0.8)
    Proof: report.md, Decisions & corrections — the site sweep at this wrap's P1: `flake` 14 standing rows, 13 of them Nix (`flake` / `flake.nix`); `repeated` 11 master rows, none about the new leg. Signals: measured +0.4, specific detail +0.2, no other durable home +0.2.
                                              + "`git fetch upstream main` moves FETCH_HEAD; read upstream's head live" (confidence 0.8)
    Proof: report.md, Decisions & corrections — at implement `git fetch upstream main` printed `main -> FETCH_HEAD`; `git ls-remote upstream refs/heads/main` read `e72650a6…`, equal to `upstream/main` (evidence/merge-surface.md). Signals: measured +0.4, specific detail +0.2, no other durable home +0.2.
  Filters: 2 dup (both recurrences, → handoff) · 0 task-specific · 0 conflict · 2 deferred (→ handoff)
  No-other-home: "In a master sweep, `flake` reads Nix and `repeated` reads everything"
  No-other-home: "`git fetch upstream main` moves FETCH_HEAD; read upstream's head live"
  CLAUDE.md size: 147/200 · T1 2.2 KB, 0 over 600 B (health.py check 1)

## Every Decisions & corrections entry, dispositioned
- Guard refusals (two, implement P1 `cd`, wrap P1 heredoc) — recurrence of host-linux.md's Paths rule and its Transports rule; at this resumed window two more `cd` refusals (the references dir, the run dir), each cost its one re-issue → handoff, recurrence-despite-learning (host-linux.md Paths & the working directory)
- Recurrence of host-linux.md Session Addition 2026-10-13 (pipeline tools read whole) — recurred at implement, at P1, and in this window (the leaf apply's saved output read through `tail -25`) → handoff, recurrence-despite-learning
- Correction at implement: evidence/clippy.md cited a leg log the later `fast` entry overwrote — the master already states each leg log is truncated at the leg's start (test-plan §9 Failure logs) → rejected — Filter 4, 0.6 (measured +0.4, specific +0.2; a master home exists, so no-other-home does not apply)
- Correction at implement: evidence/flake-repeats.md's unmeasured reason replaced by the measured fact → not a learning (an evidence-file correction, no rule)
- Operator direction inputs#I4: read the stand-repeat CI verdict with `ci.py`, not a log pattern → deferred (→ handoff, Filter 5 cap; 0.7)
- Operator directive inputs#I5 (entry 16 fired as written; the commit subject; entries 17–18) → not a learning (this chunk's operator pass, executed)
- Sweep hazard: `flake` matches Nix, `repeated` matches 11 rows → applied (Tier 3)
- Sweep hazard: bare numbers match citation digits; a count pattern wants a delimiter → rejected — Filter 1, duplicate of session-learnings.md's entry on `\b805\b` matching citation ranges; a DEFECT record recurring → handoff, recurrence-despite-learning
- Sweep hazard: `gh run view --log` colour codes as literal `^[[…m` → rejected — Filter 1, duplicate of session-learnings.md's entry on literal colour codes; it recurred at implement (a parser read 0 targets) → handoff, recurrence-despite-learning
- Sweep hazard: `git fetch upstream main` printed `main -> FETCH_HEAD`; ls-remote read live → applied (Tier 3)
- Decision (in-scope): the leg anchors its reads to the script's own directory → not a learning (a design choice in the code, stated in architecture §Standard Contracts → CI contracts this wrap)
- Measured, not this chunk's: `cargo test -p escher-driver` alone warns `variant Intrinsic is never constructed` in blitz-paint → deferred (→ handoff, Filter 5 cap; 0.8 — an additive facet of CLAUDE.md's 2026-10-10 per-crate-lint entry)
- Measured, the hover hypothesis held → rejected — Filter 1, duplicate of testing.md Session Addition 2026-10-07 (a second fixture confirming it, no new facet)
- This window: the operator's answer at the escalation (a test-only bind is no crossing) → applied (Tier 2, security.md)
- This window: a leaf comparer returned while its own background work still ran, and was stopped before the leaf apply → not a learning (the tool's telemetry — evolve)
