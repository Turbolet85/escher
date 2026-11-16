# Materialization plan — escher (re-run, upgrade form)

Setup: escher · Development Style agent-driven · re-run — upgrade form (no upstream body read).
Primary language: rust (root `Cargo.toml`). Host: linux. Masters: rows.

## Tier 1
- overview · modules · warnings · pointer-table · workflow · architecture: not re-derived — cascade-maintained, each
  standing byte for byte (warnings: 10 bullets).
- pointer-table: U48 `ok` — no row edit.
- imports: U01 `ok` — the template's one line (`@.claude/session-handoff.md`), no edit.
- deeper-topics: recomputed from the files that exist — matches the block as it stands, no edit.
- CLAUDE.md: not written (no Edit).

## Tier 2 (preserve)
- `.claude/rules/security.md` · `testing.md` · `observability.md` · `a11y.md` · `verification-harness.md` — preserve.
- `.claude/rules/host-linux.md` — preserve; U04 `noted` (9 template lines missing above `## Session Additions`) → card.

## Tier 3 (preserve)
- Core: `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md` — preserve.
- Summaries: security · design · tests · obs · a11y — preserve.
- services: blitz-dom · blitz-traits · blitz-paint · blitz-shell · blitz-test-harness · dioxus-native-dom ·
  escher-telemetry · escher-driver · seven_guis — preserve.
- `session-learnings.md` — preserve.
- Absent: none.

## Agent harness
- `scripts/agent-run.sh` (and `scripts/agent-run.ps1`, held, not owed on linux) — preserve; not compared (upgrade form).
- `.claude/rules/verification-harness.md` — preserve.

## Hooks
- `.claude/settings.json`: U02 `ok` (write guard, Bash guard, PostToolUse rustfmt on the stdin prologue) — kept.
- U68 `behind` (wake absent) → add the matrix's PreToolUse `ScheduleWakeup|CronCreate` row calling
  `bash ~/.claude/skills/andromeda-tools/hooks/wake-guard.sh` (timeout 5). Backup:
  `.claude/backup/settings.json.pre-setup-2026-11-16T11-45-00`. `env` kept.

## Code-graph pipeline
- U03 `ok` — lines behind 0 for each of the triple; `scip_pb2.py` · `requirements.txt` present — no write.

## Code reviewer
- `.claude/agents/code-reviewer.md` — preserve.

## Gitignore / gitattributes
- U07 · U56 `ok`; U06 `ok`; U05 `rustfmt.toml` present — no write.

## Upgrade
HEAD at 5b: `06c8ed8a25344134bfdff52da959cfc164be46b8`

Path set at 5b:
```
 M .andromeda/db/log/2026-11-16T00-55-22-session-2026-11-16.ndjson
 M .andromeda/db/log/2026-11-16T04-48-25-2026-11-16-errors-with-the-action.ndjson
 M .andromeda/runs/2026-11-16T08-47-42-wrap/evolve-2026-11-16-errors-with-the-action.json
 M .claude/session-handoff.md
?? .andromeda/db/log/2026-11-16T10-31-25-2026-11-16-taffy-past-1372.ndjson
?? .andromeda/runs/2026-11-16T09-58-46-phase/ (11 files)
```

`upgrade.py detect --root .` (verbatim):
```
upgrade v1.26 · 41830636
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-gra… · lines behind: code-graph.py 0 · code-graph-views.sql 0 · code-graph-cookbook.md 0
U04 · noted · setup · .claude/rul… · 9 template line(s) missing above `## Session Additions` — regenerate on the operator's word
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag (rows)
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts … · scripts/agent-run.sh carries `ensure_fresh_artifacts`
U11 · n/a · hand · code-metrics.ndjson … · past U61 the audit's records are `metric` rows: no ledger file, no terminator
U12 · n/a · hand · .andromeda/residuals.md header · past U61 the residuals are `residual` rows: no file, no header
U13 · ok · hand · .andromeda/{doc}-amendme… · masters in the store — every sidecar entry on the form held at write (U60)
U14 · ok · hand · working-route markerless in… · 9 markerless entries · 0 introducers behind markup or after the em-dash
U35 · ok · hand · masters' lo… · masters in the store — the logs and the keyed contracts out of the body, a key regrown refused held at write (U60)
U36 · ok · noted · .claude/docs/{security,design,tes… · .claude/docs/security-summary.md carries `cascade re-derives it`
U48 · ok · hand · CLAUDE.md p… · masters in the store — CLAUDE.md's pointer rows naming `rec.py contracts {plan}` held at write (U60)
U56 · ok · setup · .gitignore base ignor… · a run's prompts/ ignored by the root .gitignore (.andromeda/runs/*/prompts/)
U57 · ok · hand · the friction family in the store — `.andromeda/db/{moved, log/*.ndjson, reco… · friction: in the store
U58 · ok · hand · the spine in the store — `version` · `epoch` · `chunk` · `carry` · `capability`… · spine: in the store
U59 · ok · hand · a chunk's and a version's documents and files in the store — `doc` · `section` ·… · docs: in the store
U60 · ok · hand · the masters in the store — the seven `.andromeda/{doc}.md` bodies, their cate… · masters: in the store
U61 · ok · hand · the bases in the store — `.andromeda/residuals.md`, `playbook.md`, `drift-base.… · bases: in the store
U62 · ok · hand · the run dirs sealed — every `.andromeda/runs/{run}/` but the sealer's own as `.a… · runs: in the store
U63 · ok · hand · the harness files as the store's render — CLAUDE.md, `.claude/rules/**`, `.cl… · harness: in the store
U65 · ok · hand · the masters as rows — every `section` and `fact` of the seven ma… · masters: 7 master(s) stand as rows
U68 · behind · setup · .claude/settings.json hooks — the PreToolUse wake guard (`ScheduleWakeup|CronCreat… · wake absent
upgrade: for setup 1 (U68) · blocks phase 0 · awaiting a door 0 · noted 1 (U04) · INDETERMINATE 0 · 27 detectors of 62 registry entries
```

Acting rows: U68 (Phase 5 step 2). Noted to the card: U04.
