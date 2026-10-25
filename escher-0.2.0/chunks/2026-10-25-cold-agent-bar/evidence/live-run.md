# The founder's one live cold-agent run — 2026-10-25-cold-agent-bar, step 12

Fired once, on the founder's word «Запускать сейчас (Рекомендую)» (inputs#I6, relayed verbatim by the operator,
2026-10-25), on the dev host with the operator's own Claude Code login, through `gate.py run --operator`. Never re-fired.
Run dir `.andromeda/runs/2026-10-25T14-36-08-implement/` (gate trail `gate-2026-10-25-cold-agent-bar.json`).

## The entry lines, verbatim (header left out)

```
operator entry 22 · history tripwire: none of 7 forms matched — not a clearance
 22 probe       green · exit 0 · 0.01s · 22 B → 22.log · history: unmoved · claude --version
not run — excluded · 26: 1-21,23-27
entries 27 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 26
```

Entry 22's log: `2.1.308 (Claude Code)`.

```
operator entry 23 · history tripwire: none of 7 forms matched — not a clearance
 23 e2e         green · exit 0 · 108.47s · 1829 B → 23.log · history: unmoved · cargo build --locked -p seven_guis --bin escher-session &&… (95 chars)
not run — excluded · 26: 1-22,24-27
entries 27 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 26
```

Entry 23's atoms all held: `exit 0`, `"outcome": "passed"`, `"isolated": true`, `"counts_agree": true`,
`"check_passed": true`, `"check_can_fail": true`, `"recording_kept": true`.

## The counts (from `live-verdict.json` and the masked transcript)

| reading | value |
|---|---|
| outcome · reasons | `passed` · none |
| client · model | Claude Code 2.1.308 · `claude-opus-5-5` |
| isolated | true — the twelve `mcp__escher__*` tools, one server `escher` |
| tool calls · server tool calls · counts agree | 66 · 66 · true |
| wrong calls (refused 0 + errors 0 + foreign 0 + denials 0) | 0 of 66 — 0 % against the founder's 5 % bar («Не больше 5% (Рекомендую)») |
| transcript errors · spacer calls | 0 · 0 |
| tool calls by verb | advance 1 · click 14 · lint 1 · press 13 · screenshot 1 · scroll 1 · snapshot 13 · start 6 · stop 6 · type 10 |
| lint · screenshot · advance calls (accepted) | 1 · 1 · 1 |
| part reads | 2 |
| recorded sessions · recording kept | 6 · true (six owner-only kept checks under `target/cold-agent/record/`, `0700`/`0600`, none committed or printed) |
| tasks met | counter · flight-booker · timer · crud · cells · load (all six) |
| agent's check | 59 steps · 16 expects · replayed green (`check_passed`) and failing with its acting steps left out (`check_can_fail`) |
| turns · duration | 67 · 102,704 ms (entry wall 108.47 s) |
| cost | $0.8244 of the $3 cap |
| tokens | input 62 · output 7,009 · cache read 828,018 · cache creation 64,800 |

Against 0.1.0's baseline (intent §7): 4 tasks, 43 calls, 0 wrong, $0.32 → this run 6 tasks, 66 calls, 0 wrong, $0.82.

## The files beside this one
- `live-verdict.json` and `live-events.jsonl` — `target/cold-agent/verdict.json` and `events.jsonl` as written.
- `live-check.json` — the agent's check as the pipe parsed it (`target/cold-agent/check.json`).
- `live-transcript.jsonl` — `target/cold-agent/transcript.jsonl` with every host path masked: the per-run session dir
  as `$TMPDIR/session` (2 sites), the home dir as `~` (5 sites); 0 home, root or `/tmp/` spellings left.
- Not committed: `server.log`, `client.log` and the six kept checks (security-plan §Data Protection).
