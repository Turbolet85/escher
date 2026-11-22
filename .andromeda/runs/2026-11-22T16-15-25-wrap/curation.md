CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  corrected in place "2026-10-05: … pass `-R Turbolet85/escher` on every read of the fork" → `-R` to `gh run` and `gh cache`, the repo positional to `gh repo view`, the fork's path in `gh api`'s (no `-R`), tagged [corrected 2026-11-22]
  Proof: the operator's directive (inputs#I3); measured this session — `gh repo view --help` reads `gh repo view [<repository>] [flags]`, and `gh api --help` lists no `-R` / `--repo` flag (a `command grep -E '^\s+-R|--repo'` over its help printed no line, exit read beside it). The bullet reads 512 B (python `len(line.encode())`).
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "The fork's share of a function is read against upstream's text, not by blame"
  Proof: this session's measurement at `b1806b34e` — the audit's 8 blame-«fork» functions (`.andromeda/runs/2026-11-22T08-24-00-code-audit/proposals.md`, Single-epoch movements) each compared with `547d7514b`'s text: 5 byte-identical, 3 the same cognitive score (58 · 51 · 37); every fork-blamed line traced to an «operator pre-CI commit» sync landing; across all 94 over the ceiling, 81 equal to upstream's score and 13 above it (`build_accessibility_node` 50 against 19).
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
  CLAUDE.md size: 166/200 · T1 5.7 KB, 0 over 600 B

Candidates weighed (the no-op path has no report; the conversation is the source):
- the operator's correction of the 2026-10-05 Tier-1 learning — corrected in place (Tier 1).
- the audit's blame proxy mis-attributing sync-landed lines and missing fork growth inside upstream bodies — applied (Tier 3).
- the founder's «from now on» rule on fork additions to upstream functions — not a learning here: a master's rule, its home ruled at A3 (architecture, at the corrective chunk's wrap) and carried on «Epoch 2 corrective» (cry-6gvl75lk).
- the audit's raw jscpd report not kept in its run dir (the stand reading re-run from the recipe) — not a learning: one occurrence, the recipe reproduced the audit's whole reading exactly.
- the Bash guard's refusals (a `cd` that moves the cwd, a heredoc with a file target) — not a learning: each re-issued once in its form, already the guard's own stated rule.
