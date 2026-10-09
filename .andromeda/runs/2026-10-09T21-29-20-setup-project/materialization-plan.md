# Materialization plan — escher · setup re-run, upgrade form

- Run: 2026-10-09T21:29:20Z · run dir `.andromeda/runs/2026-10-09T21-29-20-setup-project/`
- Form: re-run — upgrade form. The invocation named no structural change, so no upstream body was read.
- Read: the upstreams' existence (8 masters + `input.md`, all present) · `architecture.md:191` (Development Style
  signal: agent-driven) · `architecture.md:44` (§Stack language row: Rust Cargo workspace, edition 2024) · Setup 5b's
  two guards · `upgrade.py detect` · `upgrade.py host` (`host: linux`) · `CLAUDE.md` · the `.claude/` and `scripts/`
  listings.
- Development Style: agent-driven · stack fragment: rust · host: linux

## Tier 1 — CLAUDE.md
| Block | Disposition |
|---|---|
| `overview` | not re-derived — cascade-maintained; stands byte for byte |
| `modules` | not re-derived — cascade-maintained; stands byte for byte |
| `warnings` | not re-derived — cascade-maintained; 10 bullets, not re-selected |
| `pointer-table` | not re-derived — cascade-maintained; U48 reads `ok` (3 of 3 indexes named) — no row edit |
| `workflow` | not re-derived — cascade-maintained; stands byte for byte |
| `architecture` | not re-derived — cascade-maintained; stands byte for byte |
| `imports` | U01 reads `ok` — the one line `@.claude/session-handoff.md` is the template's; no edit |
| `deeper-topics` | recomputed from the files that exist: 5 summaries · 5 core · 9 services · `session-learnings.md` · 6 rule files — equal to the block as it stands; no edit |

No Edit falls due, so CLAUDE.md is not written. Backup taken at Setup 5:
`.claude/backup/CLAUDE.md.pre-setup-2026-10-09T21-29-20` (md5 `412f6c0d8f1e2f6c5fd8c171ca8e4e6b`, equal to the live file).

## Tier 2 — .claude/rules/ (present files, each `preserve`)
- `security.md` — preserve
- `host-linux.md` — **regenerate** — the operator's word at the Phase 7 card (2026-10-09), naming registry U49 (class
  `operator`, no detector: the template retagged the backslash-pair bullet under `## Encoding & heredocs` to `win32`).
  U04 read `ok` before it — its detector counts template lines a leaf lacks, never one the template dropped. Done
  through `upgrade.py apply --id U04 --regenerate`: 50 → 47 lines, the three lines of that bullet removed, 0 lines
  below `## Session Additions` (the section is empty), the heading byte-identical. Backups:
  `.claude/backup/host-linux.md.pre-setup-2026-10-09T21-29-20-setup-project` (the tool's) and
  `.claude/backup/host-linux.md.pre-setup-2026-10-09T21-29-20` (taken by hand before the call; the same bytes).
- `testing.md` — preserve
- `observability.md` — preserve
- `a11y.md` — preserve
- `verification-harness.md` — preserve

Absent among the files health checks name: none.

## Tier 3 — .claude/docs/ (present files, each `preserve`)
- Core: `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md` — preserve
- Summaries: `security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md` — preserve
- Services: `blitz-dom.md` · `blitz-paint.md` · `blitz-shell.md` · `blitz-test-harness.md` · `blitz-traits.md` ·
  `dioxus-native-dom.md` · `escher-driver.md` · `escher-telemetry.md` · `seven_guis.md` — preserve
- `session-learnings.md` — wrap's; untouched

Absent among the files health checks name: none.

## Agent harness
- `scripts/agent-run.sh` — present, preserve; carries `boot` · `run` · `status` · `cleanup` · `logs` (lines 165 · 176 · 211 · 215 · 220)
- `scripts/agent-run.ps1` — present, preserve; a four-line entry that passes every argument to `agent-run.sh`
- `.claude/rules/verification-harness.md` — present, preserve
- No fresh render is made on the upgrade form: nothing compared, backed up or noted.

## Code reviewer
- `.claude/agents/code-reviewer.md` — present, preserve (language row: Rust)

## Hooks — .claude/settings.json
- U02 reads `behind`: `write inline · bash inline` — both PreToolUse guards are the inline form.
- Write: back up to `.claude/backup/settings.json.pre-setup-2026-10-09T21-29-20`, then replace the two PreToolUse
  entries with the matrix's render — `bash ~/.claude/skills/andromeda-tools/hooks/write-guard.sh` (matcher
  `Edit|MultiEdit|Write|NotebookEdit`, timeout 5) and `bash ~/.claude/skills/andromeda-tools/hooks/bash-guard.sh`
  (matcher `Bash`, timeout 5).
- PostToolUse formatter (`Edit|MultiEdit|Write`, timeout 30): the present command equals the matrix's Rust render
  (stdin prologue, `rustfmt "$f"`) — rendered to the same bytes. Linter: none at write time (clippy is a gate).
  Type checker: skipped (Rust).
- `env` (`PYTHONUTF8` · `PYTHONIOENCODING`) present — kept. No other entry exists.
- The install's scripts read: `write-guard.sh` 1179 B · `bash-guard.sh` 5110 B, 0 CR lines each; `jq` and `rustfmt` on PATH.

## Gitignore · gitattributes · formatter config
- `.gitignore` — U07 `ok`: every base ignore decided by the root `.gitignore`, depth 2 included; nothing appended.
- `.gitattributes` — U06 `ok`: carries `* text=auto eol=lf`; unchanged.
- `rustfmt.toml` — U05 `ok`: present; unchanged.

## Code-graph pipeline
- Plane: rust (root `Cargo.toml`). Seeded files all present: `code-graph.py` · `code-graph-views.sql` ·
  `code-graph-cookbook.md` · `scip_pb2.py` · `requirements.txt`.
- U03 reads `ok` (its fact column is clipped in the listing; `ok` is returned only when no file is a line behind —
  `upgrade.py:407`). Health check 11 prints the per-file count at Phase 8.
- Operational seeds present, none written: `.andromeda/state.yaml` · `.claude/session-handoff.md` ·
  `.andromeda/drift-base.md` · `.andromeda/playbook.md` · `.claude/docs/session-learnings.md` · `.andromeda/cache/`.

## Upgrade
Setup 5b's readings:
- HEAD `b03a5fc7cd7b405d87c54d8fd2085c71d88dadb5` on `build/escher-0.1.0`
- `route.py cursor`: `records 28 · complete 28 · pending 0 · gated 0` · `half-promote 0 of 28 stamped lines vs 28 master records`
- Porcelain path set (all expected-transient):
  - ` M .andromeda/friction-log.ndjson`
  - ` M .andromeda/runs/2026-10-09T20-50-14-wrap/evolve-2026-10-09-audit-corrections-agent-surfaces.json`
  - ` M .claude/session-handoff.md`
  - `?? .andromeda/runs/2026-10-09T20-50-14-wrap/health-2026-10-09-audit-corrections-agent-surfaces.json`

`upgrade.py detect --root .` — verbatim, as printed (the tool clips its fact column with `…`):

```
upgrade v1.8 · 30b07c54
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write inline · bash inline
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · ok · setup · .claude/rules/host-{os}.md · every template line present above `## Session Additions`
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh carries `ensure_fresh_artifa…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 13 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: test K, obs K, a11y K · n/a: infra K, test L, obs L, a11y L, se…
U36 · ok · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summary…
U48 · ok · setup · CLAUDE.md pointer rows · 3 of 3 indexes named
upgrade: for setup 1 (U02) · awaiting a door 0 · noted 0 · INDETERMINATE 0 · 17 detectors of 45 registry entries
```

Acting rows: U02 alone (Phase 5 step 2). No hand row is behind, no noted row is behind, none INDETERMINATE.
No host-leaf re-seed (U04 `ok`), so Phase 0 step 8a does not run; U11 and U12 read `ok`, so Phase 7.5 step 1 applies nothing.

## Consistency
- Every phase step that meets `not re-derived` writes nothing.
- The writes outside the run dir and the backups: `.claude/settings.json` (U02) and `.claude/rules/host-linux.md`
  (the operator's `regenerate`, U49).
