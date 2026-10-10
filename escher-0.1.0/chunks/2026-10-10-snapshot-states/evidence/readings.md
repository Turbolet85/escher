# Readings — 2026-10-10-snapshot-states (plan step 11)

Taken by /andromeda-implement on 2026-10-10 after the gate block read
`entries 20 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 3` (the three not run are the operator legs 18,
19 and 20). Each figure is measured on this tree; where the plan stated a prediction it is beside the reading.

## Sizes

| Task | Text at boot, incremental false · true | Standing | Predicted | In-file ceiling | Hosted `snapshot` line (stdout) | Standing |
|---|---|---|---|---|---|---|
| counter | 755 · 755 B | 755 | unchanged | 1024 | 832 B | 832 |
| flight booker | 1287 · 1287 B | 1269 | 1287 | 1536 | 1393 B | 1375 |
| timer | 1259 · 1259 B | 1259 | unchanged | 1536 | 1368 B | 1368 |
| CRUD | 2049 · 2049 B | 2034 | 2049 | 2304 | 2202 B | 2187 |

- The text sizes were read off the failure message of `stand_snapshot_text::every_screen_fits_the_budget` under a
  one-shot mutation of its ceiling comparison (`len > ceiling` → `len > 0`), the file restored by bytes with sha256
  equal; no green entry prints them.
- The hosted lines: per task `escher-session start`, `snapshot`, `stop` through the binary built alone
  (`cargo build -p seven_guis --bin escher-session --locked`), `RUST_LOG` unset — every status 0, one line of stdout,
  0 bytes of stderr, 0 ESC bytes, no state directory left. The host boots with incremental layout on; the `text`
  inside each line is the in-process size above.

## The three states through the binary, end to end

One hosted session on the flight booker and one on the CRUD, each command a run of `escher-session`, every status 0:

- flight booker at boot: the snapshot's line of the one-way button holds `pressed`, the return button's `unpressed`;
  neither date line holds `invalid`.
- `click` on the return button: `changed` names the one-way button, the return button and the return date; the two
  buttons' nodes carry `"pressed":false` and `"pressed":true`, each with `"invalid":false` and no `selected` key —
  nine keys, in the table's order.
- `type` of a text that is no date into the start date: its node carries `"invalid":true`, and the next snapshot's
  line reads `… value="…" invalid focused @…`; `type` of a date: `"invalid":false`.
- CRUD at boot: the list's line opens `ListBox`, each row's `ListBoxOption … unselected`.
- `click` on a row: `changed` names that row with `"selected":true`; a `click` on another row returns the first with
  `"selected":false` and the second with `"selected":true`; the next snapshot's line of the second holds `selected`.

## The host's log by level

The instrument an earlier chunk committed, run as written
(`python3 escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/by-level.py`, exit 0; it writes nothing into the tree):
a `snapshot`, a `click` on Create and a `type` of a sentinel on the CRUD, under both builds, at four `RUST_LOG`
settings. The figures did not move from the standing ones:

| Build | unset | `info` · `debug` · `trace` | Id needles | Name needles | Sentinel |
|---|---|---|---|---|---|
| alone | 0 B · 0 lines | 742 B · 4 lines each (INFO `escher_driver` ×3, INFO `escher_telemetry` ×1) | 0 of 15 | 0 of 6 | 0 |
| workspace | 0 B · 0 lines | 860 B · 5 lines each (one INFO `blitz_dom::document` more) | 0 of 15 | 0 of 6 | 0 |

Host exit 0, client statuses 0 · 0 · 0, stdout 0 B and no state directory left in all eight runs. `host_log` itself
is green under both builds (entries 8 and 9). Not measured: a flow that clicks a row or a mode button under this
instrument — its three commands are the standing ones.

## Counts the wrap restates

| What | Before (base `f0cb9dd7`) | Now |
|---|---|---|
| `.rs` files directly under `tests/blitz-tests/tests/` | 106 | 108 |
| of them `stand_*.rs` · `accessibility_*.rs` | 31 · 3 | 32 · 4 |
| `bash scripts/agent-run.sh run stand` | 119 passed · 0 failed · 6 ignored over 31 files (standing) | 125 passed · 0 failed · 6 ignored over 32 files, `outcome` passed |
| workspace test leg (`target/ci-logs/test.log`, this run's) | 165 result lines · 776 passed · 0 failed · 11 ignored (standing) | 167 result lines · 791 passed · 0 failed · 11 ignored |
| `a11y` leg | 6 + 6 + 3 | 6 + 6 + 3 |
| dioxus-native-dom unit tests (`cargo test -p dioxus-native-dom --lib`) | 55 | 60 — `snapshot.rs` 14 → 17, `snapshot_text.rs` 7 → 8, `snapshot_diff.rs` 8 → 9 |
| escher-driver unit tests | 50 | 50 — `schema.rs` 4 and `json.rs` 7, restated, none added |
| `stand_snapshot_state.rs` · `stand_act_diff.rs` tests | 8 · 6 | 11 · 6 |
| new files' tests | — | `accessibility_states.rs` 4 · `stand_act_states.rs` 3 |
| readers of `mod common;` · `mod session_common;` | 17 · 16 | 17 · 17 (`stand_act_states` reads `session_common` alone) |

The 15 tests the workspace gained: 4 + 3 in the new files, 3 in `stand_snapshot_state`, 5 unit tests.

## Other readings

- The builder gained no log site: entry 11 reads exit 1 with last line 0. `packages/blitz-dom/src/accessibility.rs`
  differs from the base by +31 −1 (the one removed line is the `use` line, re-wrapped).
- The two task files differ from the base by added attribute lines only: entry 10 reads exit 1 with last line 0;
  `flight_booker.rs` +4 −0, `crud.rs` +3 −0.
- The two new check files hold no print, no env read and no subscriber: `println!`, `eprintln!`, `dbg!`, `std::env`,
  `env!`, `tracing`, `subscriber`, `escher_telemetry` — 0 hits over both.
- Upstreamable (CARRY 3): against the local ref `upstream/main` at `7832c177`, `accessibility.rs` now differs by
  +57 −1. The ref was read as it stands in this checkout, not fetched. No pull request was opened.
- Not measured: any platform adapter's reading of the three states; a browser's reading of a state value by case;
  the windowed stand, which was not booted.
