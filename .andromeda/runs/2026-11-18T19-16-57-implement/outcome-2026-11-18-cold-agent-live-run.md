# Outcome — 2026-11-18-cold-agent-live-run · /implement run 2026-11-18T19-16-57-implement

**Outcome: surfaced** — on the operator's word (inputs#I4). Gate entry 12, the credential probe over the committed
live-* evidence, reads 1 hit. The hit is the field name `apiKeySource` (value `"none"`) in the masked transcript's
init record, line 1. The client writes that field on every run, and 0.2.0's committed copy carried it too. The probe's
regex `api[_-]?key` matches the name, not a credential, so it cannot read 0 on any real transcript, and its P5
controls planted files with no init record. The operator ruled «Stop here, surfaced»: «The probe is at fault, not the
evidence: keep live-transcript.jsonl byte-faithful, change nothing in it. I revise entry 12 through /andromeda-phase
next; implement then re-enters at entry 12 alone. The live run stands as fired and is never re-fired.» So no `ref` is
written for v030-15 and no chunk report is authored by this run.

## The live run — fired ONCE, passed
- Readiness entries 1-7: green 7 · red 0 (first firing, before the run).
- Entry 8 `claude --version`: green · exit 0 · history unmoved · `2.1.327 (Claude Code)`.
- Entry 9, THE ONE LIVE RUN: `9 e2e green · exit 0 · 107.72s · 2094 B → 9.log · history: unmoved`, every atom ✓,
  artifact fresh. Verdict `passed`, reasons none: 76 tool calls, 76 server calls, `counts_agree` true, 0 wrong
  (every `refused_by_cause` 0), the seven apps in `tasks_met`, `caused_by_kind` panic 1 · error 1 · log 1, `lint` ·
  `screenshot` · `advance` · `graph` · `actions` · `layout` 1 each, part reads 2, list openings 3, 7 recorded sessions
  with `recording_kept` true, a check of 71 steps and 46 expects with `check_passed` and `check_can_fail` true, 77
  turns, 103,539 ms, $1.0301 of the $3 cap. Nothing else ran `cold-agent.sh run`. It is never re-fired.
- Evidence (`.andromeda/cache/chunks/2026-11-18-cold-agent-live-run/evidence/`): `live-verdict.json`,
  `live-events.jsonl`, `live-check.json`, `live-transcript.jsonl` (host paths masked: session dir 2, home 5, 0 residual
  hits), `live-run.md`. Not copied: `server.log`, `client.log`, `target/cold-agent/record/`.

## Gates (the full block, then the re-run)
- Full block: green 13 · red 2 (12, 15). Entry 17 `ci-leg.sh fast` green, 606.25 s.
- Entry 15 (the counts probe) red: in docs/capabilities.md «in 76 / tool calls» was broken across a line wrap. Fixed by
  reflowing the line. Re-run `--only 7,12,13,14,15,16`: green 5 · red 1 (12).
- Entry 12 red, both firings (12.log, 12.2.log): `last line '0' ✗ (last: '1')`, the surfaced gap above.
- `fast` ran before the docs reflow. The reflow is a Markdown line break, and entries 14-16 re-ran green on it. The
  operator pass's push entry runs `fast` again.

## Files changed
- `scripts/cold-agent.sh` — the bar comment (:80-81) states the founder's F1 5 % for 0.3.0. Comment lines only
  (entry 3 green).
- `README.md` — heading `### 0.3.0 — built, its cold-agent run passed` (inputs#I3) and the measured-run closing
  sentence (76 calls, 0 wrong, what it used, its reach).
- `docs/capabilities.md` — the measured instance (the 2026-11-18 run), the reach paragraph with its six `stand_*`
  witnesses, and the 2026-10-25 and 2026-10-15 runs kept as the earlier runs.

## Deviations
- none from the plan's steps. No scope-record lines (`scope: clean — changed 3 · listed 3 · recorded 0`).

## Guard (entry 7, its last firing)
```
guard: allowed 3 — README.md, docs/capabilities.md, scripts/cold-agent.sh · companions excluded 0 · changed 3 · excluded 0
guard: clean
```

## Surfaced
- Entry 12's instrument: the regex `api[_-]?key` hits the client's init field `apiKeySource` in every real transcript.
  Owner: the operator, via /andromeda-phase (the plan's revision of entry 12). Then implement re-enters at entry 12
  alone, with no re-fire of the live run. After that come v030-15's `ref` (`--cleared-file`, inputs#I1) and the report.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `claude -p` (the live client) | entry 9 (this run) | terminated (census: none running) |
| `escher-session mcp` (live server child, check replay children) | entry 9 (this run) | terminated (census: none running) |
| `cargo` / test binaries of entries 1-17 | this run's gate calls | terminated (each entry exited) |
| 9 other `claude` sessions (overseers, builders, workers) | NOT DERIVED — operator/host sessions | left running: not this run's |

## Decisions & corrections
- The operator ruled entry 12's red surfaced, the transcript kept byte-faithful (inputs#I4, «Stop here, surfaced»,
  AskUserQuestion at implement P2).
- The Bash guard refused one `cd` into the skill's references directory, re-issued once with absolute paths.
- Left for the operator: `target/cold-agent/` (the pipe's state, `record/` owner-only — `bash scripts/cold-agent.sh
  cleanup` removes it once no longer needed).
