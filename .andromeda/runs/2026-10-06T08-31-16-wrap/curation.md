# Curation — 2026-10-06-upstream-sync-element-identity

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "On this host `grep` is ugrep, and a long bounded repetition can print nothing"
    Proof: two sweeps this wrap (P1: an alternation of `.{0,90}`-context patterns over the seven masters; the `.{0,200}`
    context reads of architecture.md:85 / security-plan.md:321) printed `ugrep: error … exceeds complexity limits` to
    stderr and nothing to stdout; the same alternation in Python returned 4 master sites (architecture.md:85, :124,
    :257; security-plan.md:321). Score: measured +0.4 · specific detail +0.2 = 0.6, + no-other-home +0.2 (no master,
    route annotation or rule carries it) = 0.8.
  Extended: T3/session-learnings.md: "A chunk that moves cited source lines stales the masters' file:line citations" + "key the map on full paths; read in-hunk citations against the new code"
    Proof: the plan's basename-keyed count (architecture 59 · security-plan 33 · …) matched unchanged files sharing the
    basenames; the full-path map measured 414 moved numbers over 17 paths. Six citations fell inside rewritten hunks:
    two of them stated claims the merge falsified (architecture.md:85 "normal left unset"; security-plan.md:321 "mapped
    to none") — report Spec claims disproved 1–2. Score: measured +0.4 · specific detail +0.2 = 0.6, + no-other-home
    +0.2 = 0.8.
  Filters: 0 dup · 2 task-specific (the report-date estimate caught by the stamp hook — a hook already enforces it;
    the per-entry sidecar Section guess — one-off) · 0 conflict · 0 deferred
  No-other-home: "On this host `grep` is ugrep, and a long bounded repetition can print nothing"
  No-other-home: "key the citation map on full paths; read in-hunk citations against the new code"
  Recurrence (→ handoff Deferred learnings): recurrence-despite-learning: "A chunk that moves cited source lines stales
    the masters' file:line citations" (2026-10-05, Tier 3) — the 2026-10-06-headless-stand wrap left 53 root
    `Cargo.toml` citations one line low after its line-61 insert; fixed at this wrap on the operator's word.
