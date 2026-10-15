# The operator's words in this session, 2026-10-15, verbatim

## 1. The operator pass directive (after /andromeda-implement's green report)

Run the operator pass and stop before the wrap. Fire entry 16 (hygiene) AS WRITTEN, by its own command line, never through `gate.py run --operator`. Commit as `chore(2026-10-14-quality-gates): operator pre-CI commit, for the run this chunk verdict reads`. Then fire 17 (fast, push) and 18 (the CI read through ci.py; confirm that 'Stand checks repeated' is among the checks), and write evidence/ci.md. Report the sha, the run id, the check count and the verdict.

## 2. The wrap's invocation

/andromeda-wrap-session run P1 only and stop
