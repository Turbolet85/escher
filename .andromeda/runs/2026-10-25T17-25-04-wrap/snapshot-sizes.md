# Hosted snapshot line sizes — re-measured at the 2026-10-25-cold-agent-bar wrap (P2 cascade step 2)

The sweep's `four-tasks` rows at architecture-detail/standard-contracts.md:230 and test-plan-detail/e2e-test-strategy.md:8
state the hosted `snapshot` line's size on the four lean tasks at boot (832 · 1,393 · 1,368 · 2,202 bytes, as measured at
2026-10-10-snapshot-states). This chunk moved the flight booker's mode to a `select#flight-type`, so the reading was taken
again on the dev host (linux x86_64), with `target/debug/escher-session` built after the chunk's last source commit
(`55bf3425`): per task `start {task} --session target/tmp/wsz-{task}`, `snapshot`, its stdout counted by `wc -c` (JSON
and line break included), `stop`.

| task | exit | bytes |
|---|---|---|
| counter | 0 | 832 |
| flight-booker | 0 | 1403 |
| timer | 0 | 1368 |
| crud | 0 | 2208 |

Every line inside `SNAPSHOT_TEXT_BUDGET` (10,000 bytes).

The screen text inside each line (`json.loads(line)["text"]`, UTF-8 bytes) — the in-process `Snapshot::to_text` size
standard-contracts.md:230 states as Counter 755 · FlightBooker 1287 · Timer 1259 · Crud 2049:

| task | text bytes |
|---|---|
| counter | 755 |
| flight-booker | 1295 |
| timer | 1259 |
| crud | 2055 |

Counter and Timer equal the recorded figures, so the decode reads the same text the in-process reader writes; the
flight booker (the select) and CRUD moved. One layout mode only — the host's default build; the both-modes claim is
`stand_snapshot_text`'s, which asserts the budget, not these sizes.
