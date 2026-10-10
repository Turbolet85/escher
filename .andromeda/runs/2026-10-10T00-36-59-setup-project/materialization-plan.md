# Materialization plan — escher · setup re-run, upgrade form

- Run: 2026-10-10T00:36:59Z · run dir `.andromeda/runs/2026-10-10T00-36-59-setup-project/`
- Form: re-run — upgrade form. The invocation named no structural change, so no upstream body was read.
- Read: the upstreams' existence (8 masters + `input.md`, all present) · `architecture.md:191` (Development Style
  signal: agent-driven) · `architecture.md:44` (§Stack language row: Rust Cargo workspace, edition 2024) · Setup 5b's
  two guards · `upgrade.py detect` · `upgrade.py host` (`host: linux`) · `CLAUDE.md` · the `.claude/` and `scripts/`
  listings · for the acting row U03, its three templates (`references/scripts-templates/code-graph.py.md` ·
  `code-graph-views.sql.md` · `code-graph-cookbook.md`) and the project's three present files.
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
`.claude/backup/CLAUDE.md.pre-setup-2026-10-10T00-36-59` (md5 `db18f8db8f1f274e651318e7bbedb250`, equal to the live file).

## Tier 2 — .claude/rules/ (present files, each `preserve`)
- `security.md` — preserve
- `host-linux.md` — preserve (U04 `ok`: every template line present above `## Session Additions`; one entry below it, wrap's)
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
- `scripts/agent-run.sh` — present, preserve
- `scripts/agent-run.ps1` — present, preserve
- `.claude/rules/verification-harness.md` — present, preserve
- No fresh render is made on the upgrade form: nothing compared, backed up or noted. Phase 8's check 13 reads the five verbs.

## Code reviewer
- `.claude/agents/code-reviewer.md` — present, preserve (language row: Rust)

## Hooks — .claude/settings.json
- U02 reads `ok`: `write current · bash current · PostToolUse on the stdin prologue`. No entry is on an old form, so
  nothing is replaced and `settings.json` is neither backed up nor written.

## Gitignore · gitattributes · formatter config
- `.gitignore` — U07 `ok`: every base ignore decided by the root `.gitignore`, depth 2 included; nothing appended.
- `.gitattributes` — U06 `ok`: carries `* text=auto eol=lf`; unchanged.
- `rustfmt.toml` — U05 `ok`: present; unchanged.

## Code-graph pipeline
- Plane: rust (root `Cargo.toml`). Seeded files all present: `code-graph.py` · `code-graph-views.sql` ·
  `code-graph-cookbook.md` · `scip_pb2.py` · `requirements.txt`. Nothing is seeded.
- **U03 reads `behind`** (the tool clips its fact column). Per file, by `health.behind` over the template's rendered
  region (measured this run, read-only):
  - `scripts/code-graph.py` — **27 lines behind** (template 407 lines · present 388). The template file changed in the
    install on 2026-10-10 (mtime 02:36 local).
  - `scripts/code-graph-views.sql` — 0 behind; `diff` against the template's fence: identical.
  - `scripts/code-graph-cookbook.md` — 0 behind; `diff` against the template: identical (cut marker present in both).
- The `code-graph.py` drift, whole (`diff -u` present → template: 3 hunks, every `-` line the earlier form of a
  template line — the present file carries no project edit):
  1. The rust plane gains `"config": {"cargo": {"features": "all"}}` and its `argv` takes a fifth argument, passing
     `--config-path {cfg}` to `rust-analyzer scip` — every Cargo feature on, so a target behind
     `#[cfg(feature = …)]` is indexed. The ts plane's `argv` takes the same fifth argument and ignores it.
  2. The build writes that config as a transient `indexer.json` beside the SCIP dump, runs the indexer, falls back to
     a default-features index when the all-features one fails (the result line then ends
     `(default features - the all-features index failed)`), and removes the transient.
  3. The two result lines carry that note.
- Drift rule (Phase 6): the present file is backed up —
  `.claude/backup/code-graph.py.pre-setup-2026-10-10T00-36-59` (md5 `9f982a7fed402835edfaa32d85a5ca3d`, equal to the
  live file) — and the replacement is a PROPOSAL on the Phase 7 card. On the operator's "yes" `scripts/code-graph.py`
  is written as the template's fenced body (md5 of that body as rendered to the scratchpad:
  `a2aeacc7727d7f54e784edf65a450f6c`); declined, U03 stays `behind` and nothing of it is written.
- Rebuild, expected once: the plane's freshness is `built.head == HEAD` (or the root stamp `== HEAD`), so the commit
  this run makes moves HEAD and the next code-graph query regenerates the rust plane — with the all-features index.
  `code-graph-views.sql` is not replaced, so `built.views` forces nothing.
- Operational seeds present, none written: `.andromeda/state.yaml` · `.claude/session-handoff.md` ·
  `.andromeda/drift-base.md` · `.andromeda/playbook.md` · `.claude/docs/session-learnings.md` · `.andromeda/cache/`
  (`rust/` · `tree.db.commit` = `57f0495d26eb46f1bfda5407fe293425b1b5ceab`).

## Upgrade
Setup 5b's readings:
- HEAD `57f0495d26eb46f1bfda5407fe293425b1b5ceab` on `build/escher-0.1.0`
- `route.py cursor`: `records 28 · complete 28 · pending 0 · gated 0` · `half-promote 0 of 28 stamped lines vs 28 master records`
- Porcelain path set (all expected-transient):
  - ` M .claude/session-handoff.md`

`upgrade.py detect --root .` — verbatim, as printed (the tool clips its fact column with `…`):

```
upgrade v1.8 · 30b07c54
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · behind · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph…
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
upgrade: for setup 1 (U03) · awaiting a door 0 · noted 0 · INDETERMINATE 0 · 17 detectors of 45 registry entries
```

Acting rows: U03 alone (Phase 6's drift rule — a proposal). No hand row is behind, no noted row is behind, none
INDETERMINATE. No host-leaf re-seed (U04 `ok`), so Phase 0 step 8a does not run; U11 and U12 read `ok`, so Phase 7.5
step 1 applies nothing.

## Consistency
- Every phase step that meets `not re-derived` writes nothing.
- The one write outside the run dir and the backups: `scripts/code-graph.py` (U03), only on the operator's "yes".
