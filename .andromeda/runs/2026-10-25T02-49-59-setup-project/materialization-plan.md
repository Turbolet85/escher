# Materialization plan — escher · re-run, upgrade form · 2026-10-25T02:49:59Z

Development Style: agent-driven (architecture.md:262) · stack fragment: rust (§Stack language row) · host: linux.
No upstream body read (upgrade form; no structural change named at invocation).

## Tier 1 — CLAUDE.md
- `overview` · `modules` · `warnings` · `pointer-table` · `workflow` · `architecture`: not re-derived — cascade-maintained, each standing byte for byte.
- `imports`: the template's (U01 ok) — no edit.
- `pointer-table`: U48 ok — no row edit.
- `deeper-topics`: recomputed from the files that exist — `.claude/docs/` 5 core + 5 summaries + `session-learnings.md` + `services/` 9 (blitz-dom, blitz-paint, blitz-shell, blitz-test-harness, blitz-traits, dioxus-native-dom, escher-driver, escher-telemetry, seven_guis); `.claude/rules/` 6 (security, host-linux, testing, observability, a11y, verification-harness) — equal to the block as it stands; no edit.
- Result: CLAUDE.md not written (backup taken at Setup 5: `.claude/backup/CLAUDE.md.pre-setup-2026-10-25T02-49-59`).

## Tier 2 — .claude/rules/
preserve: a11y.md · host-linux.md · observability.md · security.md · testing.md · verification-harness.md. Absent: none.
Host leaf: `host-linux.md`, rendered for linux — U04 `noted` (9 template lines missing above `## Session Additions`) → the card's Re-run row; regenerate on the operator's word only.

## Tier 3 — .claude/docs/
preserve: stack · conventions · commands · gotchas · workflow · security-summary · design-summary · tests-summary · obs-summary · a11y-summary · session-learnings · services/{the 9 above}. Absent: none.

## Agent harness
preserve: scripts/agent-run.sh · scripts/agent-run.ps1 · .claude/rules/verification-harness.md (health check 13: 5/5 verbs both scripts, `test -x` ok). Not compared — upgrade form.

## Hooks
U02 ok — settings.json not written.

## Code-graph pipeline
U03 ok (0 lines behind each of the triple); scip_pb2.py and requirements.txt present — nothing seeded.

## Code reviewer
`.claude/agents/code-reviewer.md` present — preserve.

## Gitignore
U56 behind → append the base ignore `.andromeda/runs/*/prompts/` to the root `.gitignore`'s setup block (after `__pycache__/`). U07 ok — no other base line. `.gitattributes` U06 ok — not written.

## Upgrade
HEAD at 5b: 10e8cc39197ca48317fc52403bde3503dde7154e
Path set at 5b (all expected-transient bookkeeping or an untracked run-dir path):
- ` M .andromeda/friction-log.ndjson`
- ` M .andromeda/runs/2026-10-24T21-49-26-wrap/evolve-2026-10-24-a11y-re-read.json`
- ` M .claude/session-handoff.md`
- `?? .andromeda/runs/2026-10-24T21-49-26-wrap/health-2026-10-24-a11y-re-read.json`

`upgrade.py detect --root .` (verbatim):

```
upgrade v1.14 · 796c8232
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-gra… · lines behind: code-graph.py 0 · code-graph-views.sql 0 · code-graph-cookbook.md 0
U04 · noted · setup · .claude/rul… · 9 template line(s) missing above `## Session Additions` — regenerate on the operator's word
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts … · scripts/agent-run.sh carries `ensure_fresh_artifacts`
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjs… · friction-log.ndjson LF · code-metrics.ndjson LF
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless in… · 2 markerless entries · 0 introducers behind markup or after the em-dash
U35 · ok · hand · masters' lo… · ok: test K, obs K, a11y K · n/a: infra K, test L, obs L, a11y L, security L, design L, layout L
U36 · ok · noted · .claude/docs/{security,design,tes… · .claude/docs/security-summary.md carries `cascade re-derives it`
U48 · ok · setup · CLAUDE.md pointer rows · 3 of 3 indexes named
U56 · behind · setup · .gitignore base ignore `.androm… · not ignored by the root .gitignore: .andromeda/runs/*/prompts/
upgrade: for setup 1 (U56) · awaiting a door 0 · noted 1 (U04) · INDETERMINATE 0 · 18 detectors of 50 registry entries
```

Acting rows: U56 (P5 step 3). Noted: U04 (card). P7.5: U11 ok, U12 ok, no host re-seed (U04 not `behind`) — nothing.

## Validation (step 9)
Consistent: one write planned (`.gitignore`), every other artifact preserved; no tracked path under `.andromeda/runs/*/prompts/` (0), so the new line untracks nothing.
