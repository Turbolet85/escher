# The cold-agent live run — 2026-10-15 (v010-15)

The founder's one live run («Один запуск, до $3 (Рекомендую)», «Все четыре в одной сессии», inputs#I2), fired once by
/implement P2 on the operator's word (inputs#I7), through `gate.py run --plan … --operator 10`, on the dev host under the
operator's own Claude Code login. Not re-fired; it stands as read.

## Command

```
cargo build --locked -p seven_guis --bin escher-session && bash scripts/cold-agent.sh run stand
```

- Start: 2026-10-15T09:24:13Z (the call) · `run.start` 2026-10-15T09:24:15Z
- End: `run.end` 2026-10-15T09:25:06Z · the call 2026-10-15T09:25:06Z
- Wall clock: 53.18 s for the entry (the build 2.09 s of it); the client's `duration_ms` 48,809
- Exit: 0 · history: unmoved

The entry's line, verbatim:

```
operator entry 10 · history tripwire: none of 7 forms matched — not a clearance
 10 e2e         green · exit 0 · 53.18s · 1641 B → 10.log · history: unmoved · cargo build --locked -p seven_guis --bin escher-session &&… (95 chars)
entries 14 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 13
```

## The atoms (`gate.py show --n 10`)

| atom | reading |
|---|---|
| `exit 0` | ✓ |
| `contains "outcome": "passed"` | ✓ |
| `contains "isolated": true` | ✓ |
| `contains "counts_agree": true` | ✓ |
| `contains "check_passed": true` | ✓ |
| `contains "check_can_fail": true` | ✓ |
| artifact `target/cold-agent/verdict.json` | fresh |

## The verdict (`live-verdict.json`, every field)

| field | value |
|---|---|
| task | `stand` |
| client_exit | 0 |
| isolated | true |
| session_tools | the eleven `mcp__escher__{verb}`: advance · click · help · press · screenshot · scroll · snapshot · start · status · stop · type |
| client_version | 2.1.296 |
| model | claude-opus-5-5 |
| tool_calls | 43 |
| server_tool_calls | 43 |
| refused_calls | 0 |
| error_calls | 0 |
| wrong_calls | 0 |
| transcript_errors | 0 |
| counts_agree | true |
| spacer_calls | 0 |
| tasks_met | counter · flight-booker · timer · crud |
| check_steps | 39 |
| check_expects | 10 |
| check_passed | true |
| check_can_fail | true |
| num_turns | 44 |
| duration_ms | 48809 |
| cost_usd | 0.31969800000000004 |
| input_tokens | 34 |
| output_tokens | 4076 |
| cache_read_input_tokens | 241490 |
| cache_creation_input_tokens | 23718 |
| transcript_file | `target/cold-agent/transcript.jsonl` |
| outcome | **passed** |
| reasons | `[]` |
| ts | 2026-10-15T09:25:06Z |

## Cost against the cap

0.3197 USD of the $3 cap (`--max-budget-usd 3`): 10.7 %.

## Wrong calls by kind (recorded, deciding nothing — `.andromeda/residuals.md:9`'s baseline)

| kind | count |
|---|---|
| refused by the server (` ending="refused" ` span lines) | 0 |
| ended in a session error (` ending="error" ` span lines) | 0 |
| a tool outside the `escher` server | 0 |
| a permission denial in the result | 0 |
| **wrong_calls** | **0** |

The server's own span lines, by tool and ending (counted from `target/cold-agent/server.log`, which is not copied): start 4 ·
snapshot 8 · click 10 · type 6 · press 10 · advance 1 · stop 4, every one `accepted`; 43 lines, equal to the transcript's
43 escher tool uses. No `help`, `status`, `scroll` or `screenshot` call was made.

## `covered` on the header spacer (the founder's «Отложить на 0.2.0, наблюдать (Рекомендую)»)

`spacer_calls` = 0: the agent made no call on `task-header-spacer`, so the run observed no `covered` refusal on it. The
deferral's residuals line reads this count.

## The check

`live-check.json` holds the agent's check as parsed: 39 steps, 10 expectation texts, each app started, driven, snapshotted
and stopped in turn. Replayed by the pipe on a fresh `escher-session mcp` child with no model, every step answered
without `isError` and every expectation held (`check_passed`). Replayed again on a second fresh child with its acting
steps left out, at least one snapshot expectation of each of the four apps was absent (`check_can_fail`).

## Files

`live-verdict.json`, `live-events.jsonl` and `live-check.json` are byte copies of `target/cold-agent/{verdict.json,
events.jsonl, check.json}` (`cmp` equal). `live-transcript.jsonl` is `transcript.jsonl` with its host paths masked, in the
2026-10-06 form: the session directory → `/tmp/<session-dir>`, the home prefix → `/home/<user>/`. It keeps the same
103 lines, each still JSON. `server.log` and `client.log` (0 bytes) are not copied; their counts are in the verdict.
