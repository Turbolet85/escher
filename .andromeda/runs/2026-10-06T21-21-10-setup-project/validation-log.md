# Validation log — setup-project re-run (the upgrade), escher

Run dir `.andromeda/runs/2026-10-06T21-21-10-setup-project/` · branch `build/escher-0.1.0` · HEAD at Setup `9944dec5`.

## Pre-flight
`pre-flight · ✓ · CLAUDE.md at ./CLAUDE.md`

## Checks 1–14

| Check | Status | Diagnostic |
|---|---|---|
| 1 CLAUDE.md size | ✓ | 132/200 lines · T1 0.4 KB · 0 of 1 bullets over 600 B |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | one import, `@.claude/session-handoff.md`, target present |
| 4 rule files | ✓ | 5 files · frontmatter 4/4 parsed · always-loaded 1 (`security.md`, 3.1 KB) · 0 entries over 1.5 KB |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | `architecture.md` older than `CLAUDE.md` (mtime 1791320859 ≤ 1791322007) |
| 7 session-handoff (hand) | ✓ | present, 3315 B |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root `.gitignore` (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master-route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | parses · `schema_version: 3` · only the four lean fields |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 24 entries (threshold 5) |

Tool line: `checks run: 10 ✓ / 0 ⚠ / 0 – · INDETERMINATE: 0` (`health v1.0 · 3c0f4685`; trail `health-no-marker.json`).

## Hook smoke

Run from the project root through each hook's own stdin path, the commands taken as stored (`jq` on PATH; `CLAUDE_PROJECT_DIR` unset, so the cd arm fell back to the shell's directory).

| Arm | Wanted | Got |
|---|---|---|
| formatter on a mis-formatted `.setup-validation-test.rs` | exit 0, file changed | exit 0, reformatted; temp file removed |
| Bash guard · a cat heredoc with a file target | 2 | 2 |
| Bash guard · a python heredoc | 0 | 0 |
| Bash guard · `cd .andromeda && ls` | 2 | 2 |
| Bash guard · `cd . && ls` | 0 | 0 |
| Bash guard · `ls && cd .andromeda` | 2 | 2 |
| Bash guard · `ls; ( cd .andromeda && ls )` | 0 | 0 |
| write guard · `src/x.rs` | 0 | 0 |
| write guard · `target\x.rs` | 2 | 2 |
| write guard · `C:\p\target\x.rs` | 2 | 2 |

Hook smoke ✓. Before it, the rendered `settings.json` was compared in python against the hooks matrix: both PreToolUse entries equal the matrix's fenced forms; the write guard, the PostToolUse entry and `env` equal the pre-setup backup; the Bash guard differs from it (3858 B against 2821 B).

## Upgrade re-detect (Phase 7.5 step 2)

`upgrade v1.5 · eeed2076` — rows this run wrote: `U02 · ok-uncommitted` (was `behind`, `leading-cd`); `U01 · ok` (CLAUDE.md was edited; the imports block is untouched). No setup-class row `INDETERMINATE`. No U11 / U12 apply (both `ok` at Phase 0). ✓

## Summary

14 ✓ / 0 ⚠ / 0 – / 0 ✗ · hook smoke ✓ · upgrade re-detect ✓ — **decision: commit.**
