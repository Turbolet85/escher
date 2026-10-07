# Gate readings — 2026-10-07-driver-session (plan step 13)

One full run of the plan's `## Test Commands` block through the gate tool by /andromeda-implement, on the
working tree over base `7d9f351d`; written 2026-10-07T05:50Z. The tool's summary line, verbatim:

```
entries 27 · green 24 · red 0 · recorded 0 · timeout 0 · not-run 3
```

Entries 25, 26 and 27 read `not run — leg operator`: they are the operator pass's (hygiene, the pre-push gate
and push, the CI read) and were not fired here. No entry was deferred, skipped or voided. Iterations to green: 1.

## Each `new` entry: predicted against measured

| n | entry | baseline (plan) | predicted | measured |
|---|---|---|---|---|
| 1 | `cargo test -p escher-driver --locked --lib` | red, no such package | 0 → the crate's unit count | green · `13 passed; 0 failed; 0 ignored` |
| 2 | `cargo test -p seven_guis --locked --test host_binary` | red, no such test target | `2 passed; 0 failed` | green · `2 passed; 0 failed; 0 ignored` |
| 3 | `… --test stand_session_state` | red, no such target | 0 → 3 | green · `3 passed; 0 failed; 0 ignored` |
| 4 | `… --test stand_session_ids` | red, no such target | 0 → 2 | green · `2 passed; 0 failed; 0 ignored` |
| 5 | `… --test stand_session_fresh` | red, no such target | 0 → 2 | green · `2 passed; 0 failed; 0 ignored` |
| 6 | `… --test stand_session_lifecycle` | red, no such target | 0 → 1 passed, 1 ignored | green · `1 passed; 0 failed; 1 ignored` |
| 7 | `… --test stand_session_quiet` | red, no such target | 0 → 1 passed, 1 ignored | green · `1 passed; 0 failed; 1 ignored` |
| 8 | manifest probe (`grep -rl seven_guis … packages`) | green, exit 1 and no output | kept | green · exit 1 · no output |
| 9 | library census | red, crate absent | last line `0` at exit 1 | green · exit 1 · last line `0` |
| 10 | listener census | red, one empty line | `packages/escher-driver/src/host.rs` alone | green · exit 0 · last line `packages/escher-driver/src/host.rs` |
| 11 | stdout census | red, crate absent | last line `0` at exit 1 | green · exit 1 · last line `0` |
| 12 | stand manifest lines 1-34 against the base | green | kept | green · exit 0 |
| 13 | preservation guard (`git diff --quiet 7d9f351d… -- …`) | green | kept | green · exit 0 |
| 18 | stand `ok` count from the agent-run events | red, last line `63` | 63 → 72 | green · last line `72` |
| 19 | content-named keys in the agent-run events | red, `no-stand_session_quiet-events` | last line `0` | green · last line `0` |

## The other entries

| n | entry | measured |
|---|---|---|
| 14 | `cargo build -p seven_guis --bin seven_guis_native --locked` | green · exit 0 |
| 15 | the windowed stand's 10 s boot smoke (`WAYLAND_DISPLAY` set on this host) | green · exit 0 · `service.name=seven_guis` present |
| 16 | `bash scripts/agent-run.sh boot` | green · `"ready"` |
| 17 | `bash scripts/agent-run.sh run stand` | green · artifact `target/agent-run/events.jsonl` fresh · `run.end` reads passed 72 · failed 0 · ignored 3 |
| 20 | `bash scripts/agent-run.sh cleanup` | green |
| 21 | `bash .github/scripts/ci-leg.sh fast` | green · exit 0 |
| 22 | `bash .github/scripts/ci-leg.sh doc` | green · exit 0 |
| 23 | `bash .github/scripts/ci-leg.sh a11y` | green · exit 0 |
| 24 | `bash .github/scripts/ci-leg.sh audit` | green · exit 0 |

## The workspace count, from the fast leg's own log

Read from `target/ci-logs/test.log` written by entry 21 (its mtime is inside this run), every `test result:` line
summed:

| | result lines | passed | failed | ignored |
|---|---|---|---|---|
| before (the plan's local baseline) | 131 | 548 | 0 | 5 |
| predicted | 140 | 559 + the library's unit count | 0 | 7 |
| measured | 140 | 572 (= 559 + 13) | 0 | 7 |

The nine new result lines: the library's unit tests (13 · 0 · 0), its doc-tests (0 · 0 · 0), the
`escher-session` binary's unit-test target (0 · 0 · 0), `host_binary` (2 · 0 · 0) and the five
`stand_session_*` files (3 · 0 · 0, 2 · 0 · 0, 2 · 0 · 0, 1 · 0 · 1, 1 · 0 · 1). The `ci-scripts` leg reads
`Ran 64 tests` and `OK`, unchanged.

## Beside the block

- The three process checks (`stand_session_lifecycle`, `stand_session_quiet`, `host_binary`) were re-run 20
  times each after the block: 60 of 60 passed; `target/tmp/` was empty afterwards and no session process was
  left (`pgrep`).
- Not run locally, as the plan states: the MSRV build and the windows, macos, ios and android legs. This host
  has no windows or wasm target installed either, so the non-unix arms of `host.rs` and `client.rs` are
  compiled by no local gate; the operator's CI entry (27) is their witness.
- What the host binary writes to stderr at each `RUST_LOG` level is in `host-stderr-by-level.md`.
