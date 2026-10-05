# Validation Log — setup-project (escher, first run)

Pre-flight: ✓ CLAUDE.md at ./CLAUDE.md (health v1.0 · 3c0f4685)

| Check | Status | Diagnostic |
|---|---|---|
| 1 CLAUDE.md size | ✓ | 126/200 lines · T1 0.3 KB · 0 bullets over 600 B |
| 2 Section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 @-imports | ✓ | `@.claude/session-handoff.md` resolves (hand read) |
| 4 Rule files | ✓ | 5 files · frontmatter 4/4 · always-loaded 1 (security.md 2.0 KB) |
| 5 Core docs | ✓ | 5/5 |
| 6 Arch staleness | ✓ | architecture.md older than CLAUDE.md (hand read, mtime) |
| 7 session-handoff.md | ✓ | present, non-empty (hand read) |
| 8 .gitignore | ✓ | 6/6 entries (fragment rust) |
| 9 Plans + summaries | ✓ | 6/6 · 5/5 |
| 10 master-route | ✓ | present |
| 11 Operational artifacts | ✓ | seeded 2/2 · plane rust · pipeline 4/4 · 0 lines behind |
| 12 state.yaml | ✓ | parses · schema_version 3 · lean fields only (hand read) |
| 13 Agent harness | ⚠ | agent-driven, scripts/agent-run.{sh,ps1} missing — by design: test-plan §3 NOT YET MEASURED → owned by "Stand test contract" |
| 14 Pointer table | ✓ | 20 entries |
| Hook smoke (formatter · Bash guard · write guard · jq) | ✓ | formatter changed the control file; heredoc deny 2 · python allow 0 · cd deny 2 · cd . allow 0 · src allow 0 · target\ deny 2 · C:\…\target deny 2 |

Summary: 13 ✓ / 1 ⚠ / 0 – / 0 ✗ — decision: commit.
