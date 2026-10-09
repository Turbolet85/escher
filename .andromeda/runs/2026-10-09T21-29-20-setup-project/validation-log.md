# Validation log — setup re-run, upgrade form · 2026-10-09

## Upgrade re-detect (Phase 7.5 step 2)
- `U02 · ok-uncommitted · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin pr…` — ✓ (was `behind`: `write inline · bash inline`)
- `U04 · ok` before and after the operator's `regenerate host-linux.md` (registry U49 has no detector).
- Summary line printed: `upgrade: for setup 1 (U02) · awaiting a door 0 · noted 0 · INDETERMINATE 0 · 17 detectors of 45 registry entries` — its count holds U02 while the write is uncommitted.
- U11 · U12 read `ok`: Phase 7.5 step 1 applied nothing.

## Health checks (`health v1.0 · 1358832b`, trail `health-no-marker.json`)
| Check | Status | Diagnostic |
|---|---|---|
| pre-flight | ✓ | CLAUDE.md at ./CLAUDE.md |
| 1 CLAUDE.md size | ✓ | 138/200 lines · T1 0.9 KB · 0 of 2 bullets over 600 B |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | `CLAUDE.md:111` `@.claude/session-handoff.md` — the file exists |
| 4 rule frontmatter · size | ✓ | 6 rule files · frontmatter 4/4 parsed · always-loaded 2 (7.7 KB): `host-linux.md` 2.6 KB, `security.md` 5.2 KB |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | `architecture.md` mtime 2026-10-09 23:10:27 +0200 ≤ `CLAUDE.md` mtime 2026-10-09 23:13:54 +0200 |
| 7 session-handoff (hand) | ✓ | present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root `.gitignore` (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master-route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | `schema_version: 3` · `last_wrap` · `tree_db_refreshed_at` · `session_count` — the lean fields only |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 28 entries (threshold 5) |

## Hook smoke — each command run as stored, input on stdin, from the project root
`jq` at `/usr/bin/jq`; `CLAUDE_PROJECT_DIR` unset in the probing shell (the guard fell back to the shell's directory).

| Arm | Want | Got |
|---|---|---|
| formatter on a mis-formatted `.setup-validation-test.rs` (the file changed; removed after) | 0 | 0 |
| Bash guard — `cat > x.md <<'EOF'…` | 2 | 2 |
| Bash guard — `python - <<'PY'…` | 0 | 0 |
| Bash guard — `cd .andromeda && ls` | 2 | 2 |
| Bash guard — `cd . && ls` | 0 | 0 |
| Bash guard — `ls && cd .andromeda` | 2 | 2 |
| Bash guard — `ls; ( cd .andromeda && ls )` | 0 | 0 |
| write guard — `src/x.rs` | 0 | 0 |
| write guard — `target\x.rs` (printf octal form) | 2 | 2 |
| write guard — `C:\p\target\x.rs` (printf octal form) | 2 | 2 |

Refusal texts, as the guards printed them on stderr:
- heredoc arm (201 B): `Blocked: a cat/tee heredoc with a file target - a new document goes through the Write tool, an append through the Edit tool anchored on the file's last lines, a script to a scratchpad file run by path`
- cd arms (202 B each): `Blocked: cd (.andromeda) moves the session cwd for every later call - keep it: absolute paths, git -C DIR, a tool dir flag, or a subshell ( cd DIR && ... ); a cd into the cwd or the project root passes`
- write guard: `Blocked: edit to a generated directory (target/x.rs)` · `Blocked: edit to a generated directory (C:/p/target/x.rs)`

Not exercised: the 6500-byte arm (a Windows/Git-Bash host's; this host is linux); the hooks as the harness itself invokes them inside a live session.

## Summary
14 ✓ / 0 ⚠ / 0 – / 0 ✗ · hook smoke 10 of 10 arms · upgrade re-detect ✓ — decision: commit.
