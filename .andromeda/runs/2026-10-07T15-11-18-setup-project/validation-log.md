# Validation Log — setup-project re-run · 2026-10-07T15-11-18

## Pre-flight
`pre-flight · ✓ · CLAUDE.md at ./CLAUDE.md` (`health v1.0 · b1b10944`; trail `health-no-marker.json`).

## Checks 1–14

| Check | Status | Diagnostic |
|---|---|---|
| 1 CLAUDE.md size | ✓ | 138/200 lines · T1 0.9 KB · 0 of 2 bullets over 600 B |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports valid (hand) | ✓ | one import, `@.claude/session-handoff.md` (CLAUDE.md:111); the file exists |
| 4 rule files | ✓ | 6 rule files · frontmatter 4/4 parsed · always-loaded 2 (7.3 KB): `security.md` 4.5 KB · `host-linux.md` 2.8 KB; no file past the read cap |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | `architecture.md` is 0.54 h older than `CLAUDE.md` |
| 7 session-handoff (hand) | ✓ | present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root `.gitignore` (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master-route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | parses; `schema_version: 3`; only `last_wrap` · `tree_db_refreshed_at` · `session_count` |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 28 entries (threshold 5) |

## Hook smoke (each hook's stored command, its input on stdin, run from the Bash tool)

| Arm | Expected | Read |
|---|---|---|
| formatter on a mis-formatted `.setup-validation-test.rs` | exit 0, file changed | exit 0, file changed; temp file removed |
| Bash guard · `cat > x.md <<'EOF'` heredoc | 2 | 2 |
| Bash guard · `python - <<'PY'` heredoc | 0 | 0 |
| Bash guard · `cd .andromeda && ls` | 2 | 2 |
| Bash guard · `cd . && ls` | 0 | 0 |
| Bash guard · `ls && cd .andromeda` | 2 | 2 |
| Bash guard · `ls; ( cd .andromeda && ls )` | 0 | 0 |
| write guard · `src/x.rs` | 0 | 0 |
| write guard · `target\x.rs` | 2 | 2 |
| write guard · `C:\p\target\x.rs` | 2 | 2 |

`jq` on PATH (`/usr/bin/jq`). 10 of 10 arms.

## Upgrade re-detect (Phase 7.5 step 2)

`upgrade v1.6 · 814083ff` — the one row this run wrote:
`U04 · ok-uncommitted · setup · .claude/rules/host-{os}.md · every template line present above ## Session Additions`.
No setup-class row reads `behind` or `INDETERMINATE` once U04 is committed; the summary line still counts U04 for setup until then.

## Summary

14 ✓ / 0 ⚠ / 0 – / 0 ✗ · hook smoke ✓ · upgrade re-detect ✓ → **commit**.
