# Validation log — escher · setup re-run, upgrade form · 2026-10-10T00-36-59

## Phase 7 — the operator's word
- Card shown with one proposal (U03: `scripts/code-graph.py` 27 lines behind its template). The operator: "yes — apply U03."
- No `regenerate {leaf}`, no host-leaf re-seed, no `review`.

## The U03 write (Phase 6's drift rule, after "yes")
- `scripts/code-graph.py` brought to the template's fenced body by four anchored Edits (the rust plane's `config` and
  five-argument `argv` · the ts plane's five-argument `argv` · the indexer call with its transient config and the
  default-features fallback · the two result lines' note).
- `diff` against the template's rendered region: identical (exit 0). md5 `a2aeacc7727d7f54e784edf65a450f6c`, the
  hash the checkpoint named before the write. `python -m py_compile`: exit 0. `git ls-files --eol`: `i/lf w/lf`.
- Backup of the earlier file: `.claude/backup/code-graph.py.pre-setup-2026-10-10T00-36-59`
  (md5 `9f982a7fed402835edfaa32d85a5ca3d`).
- Not run: a plane rebuild. The rust plane regenerates at the first code-graph query after this run's commit
  (`built.head` ≠ HEAD), then with the all-features index; whether that index succeeds on this workspace, and how
  long it takes, is not measured here.

## Phase 7.5 — re-detect
- Step 1: U11 and U12 read `ok`; U04 `ok` — nothing applied.
- Step 2: `upgrade.py detect --root .` → `U03 · ok`; summary
  `upgrade: for setup 0 · awaiting a door 0 · noted 0 · INDETERMINATE 0 · 17 detectors of 45 registry entries`.

## Phase 8 — health checks (14 ✓ / 0 ⚠)
`health.py check --root . --stack rust --style agent-driven` (health v1.0 · 1358832b; trail `health-no-marker.json`):
- pre-flight ✓ · CLAUDE.md at ./CLAUDE.md
- check 1 ✓ · CLAUDE.md 140/200 lines · T1 1.6 KB · 0 of 4 bullets over 600 B
- check 2 ✓ · 20 markers · 10 starts · 0 orphan / mismatch / unclosed
- check 4 ✓ · 6 rule files · frontmatter 4/4 parsed · always-loaded 2 (7.9 KB)
- check 5 ✓ · core docs 5/5
- check 8 ✓ · 6/6 entries satisfied by the root .gitignore (fragment: rust)
- check 9 ✓ · plans 6/6 by name · summaries 5/5 by name
- check 10 ✓ · .andromeda/master-route.md present
- check 11 ✓ · seeded 2/2 · planes rust · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0
- check 13 ✓ · agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5
- check 14 ✓ · pointer table 28 entries (threshold 5)

By hand:
- check 3 ✓ · one `@` line (`CLAUDE.md:111` → `.claude/session-handoff.md`), the file exists
- check 6 ✓ · `architecture.md` mtime 2026-10-09 23:10:27 +0200 ≤ `CLAUDE.md` mtime 2026-10-10 02:33:30 +0200
- check 7 ✓ · `.claude/session-handoff.md` present and non-empty
- check 12 ✓ · `.andromeda/state.yaml`: `schema_version: 3` and the three lean fields (`last_wrap` ·
  `tree_db_refreshed_at` · `session_count`), nothing else
