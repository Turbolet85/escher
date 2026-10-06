CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "In a `.github/scripts` unittest file, never give a helper method a `test_` prefix — `unittest` collects it as a test case; name helpers without it and check the run's `Ran N tests` against the cases written." (confidence 0.8)
    Proof: implement P1's first run of `python3 -m unittest discover -s .github/scripts -p 'test_agent_run.py'` printed `Ran 15 tests` for 14 written cases — the helper `test_files(self, *names)` was collected; renamed `add_test_files`, the re-run printed `Ran 14 tests` (this session; the gate entry 1 then read 14 green).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 1 task-specific (the operator's one-off directive to drive entries 21-23) · 0 conflict · 0 deferred · 1 below threshold (the `TMPDIR` substring census hazard — 0.2 detail − 0.3 one-off, never fired)
  Recurrence: "The project's Bash guards refuse a heredoc written to a file and a leading cd" (session-learnings.md, 2026-10-06) — a leading `cd` was refused twice again this session (into a chunk evidence dir; into `.andromeda/`) → handoff Deferred learnings as recurrence-despite-learning
  No-other-home: "never give a unittest helper a `test_` prefix"
  CLAUDE.md size: see the P7 console report (health check 1 row)
