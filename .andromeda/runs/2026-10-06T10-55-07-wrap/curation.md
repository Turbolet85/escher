# Curation — 2026-10-06-id-persistence

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A presentation guard that diffs grepped source lines must strip leading whitespace" (confidence 0.8)
    Proof: the gate entry `git show 065295540ec0…:examples/seven_guis/src/tasks/crud.rs | grep -E 'class:|style|id: "|^const CSS' | diff - <(…)` read red (exit 1) at implement P2. Its only diff line was the row `class:` expression shifted 40 → 36 leading spaces after the row loop was un-nested. The operator's correction (2026-10-06): "the guard's intent is class strings unchanged, not indentation". The whitespace-stripped form reads `presentation-unchanged`, exit 0 (escher-0.1.0/chunks/2026-10-06-id-persistence/evidence/entry-3-whitespace.md). Signals: explicit user correction +0.4 · verified by a real gate failure +0.4.
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred (cap) · 2 below threshold
    - "a `key:` inside an `if` within a `for` does not key a Dioxus list" — 0.6 (measured +0.4, technical detail +0.2) → rejected at exactly 0.6. It already landed this wrap in architecture §Standard Contracts → Dioxus DOM bridge (P2 amendment) and its dioxus-native-dom leaf, so the conditional signals do not apply.
    - "DioxusDocument's html/head/body/#main skeleton survives a remount" — 0.6 → rejected; amended into architecture this wrap.
  Recurrences (→ handoff Deferred learnings, not re-added):
    - recurrence-despite-learning: "The project's Bash guards refuse a heredoc written to a file and a leading cd, and read payload prose too". This wrap, a `cat >> evidence <<EOF` append was refused (Edit tool used), and so was a leading `cd` into the scratchpad prompts dir (absolute paths used).
    - recurrence-despite-learning: "On this host `grep` is ugrep, and a long bounded repetition can print nothing". A P1 site sweep with `grep -oE '.{0,60}crud.{0,80}'` failed "exceeds complexity limits" (stderr seen; Python used).
  CLAUDE.md size: see the P7 health check 1 row
