# Curation — 2026-10-10-mcp-surface (wrap resumed in 2026-10-11T08-35-17-wrap)

Session-local: this window holds the resumed wrap only (P1's window, which held implement and the operator pass, is
gone); the report's Decisions & corrections stood in for it.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "`gh api` refuses a job's log that holds terminal escapes"
    Proof: the operator pass's read of the failed `Build wasm examples` job (CI#38124028789 attempt 1) — `gh api
    …/jobs/{id}/logs` refused the output until given `--allow-escape-sequences` (report.md, Decisions & corrections,
    sweep hazards). Score: measured failure +0.4 · technical detail with context +0.2 · reached no other durable
    home this wrap +0.2 = 0.8.
  Filters: 2 dup · 1 task-specific · 0 conflict · 0 deferred
    - dup: "`tracing`'s callsite interest is process-global; a `with_default` check needs its own recorder" — this
      wrap amended it into obs-plan §3 (the `stand_mcp_tools` exception) and the cascade carried it to two leaves.
    - dup: "the gate tool's `--operator` call exits 0 on a red; its verdict is the printed word" — the host rule
      "When a tool prints its own verdict, the printed text outranks an unisolated `$?`" carries it.
    - dup, a recurrence: "`$TMPDIR` is unset in the Bash tool's shell" — `docs/session-learnings.md` 2026-10-07
      records the defect and its remedy; the operator pass hit it again → handoff, `recurrence-despite-learning`.
    - task-specific: "a regex for `tool_call` matches the stub verdict's `tool_calls`" — one document's token.
  No-other-home: "`gh api` refuses a job's log that holds terminal escapes"
  CLAUDE.md size: see P7 (health check 1).
