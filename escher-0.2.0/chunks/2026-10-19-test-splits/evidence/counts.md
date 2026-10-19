# Counts — 2026-10-19-test-splits (implement, run `.andromeda/runs/2026-10-19T20-53-01-implement/`)

Read from that run's gate trail (`.andromeda/runs/2026-10-19T20-53-01-implement/gate-2026-10-19-test-splits.json`),
one firing of the block, every non-operator entry green (`entries 26 · green 22 · red 0 · recorded 0 · timeout 0 · not-run 4`).

- escher-driver unit tests (entry 1): `test result: ok. 92 passed; 0 failed` in the lib target; `--list` equal to
  `driver-tests-before.txt` (entry 2, `cmp` exit 0).
- Source sizes by `wc -l` (entry 10): `packages/escher-driver/src/cli.rs 478` · `packages/escher-driver/src/recording.rs 459`
  — as predicted. The new files: `cli_tests.rs` 559 · `recording_tests.rs` 517 · `recording_off_unix_tests.rs` 15.
- seven_guis (entry 11): exit 0, no `FAILED`; the two log checks' `--list` equal to `log-tests-before.txt` (entry 12).
- jscpd 5.4.0 over `host_log.rs`, `mcp_log.rs`, `common/mod.rs` (entry 13): 0 clones (base 6 clones, 95 lines).
- Workspace tally over the fast leg's fresh `target/ci-logs/test.log` (entries 19, 21): `lines 199 passed 1045 failed 0
  ignored 12` — unmoved from the last wrap's local baseline; both log checks ran as their own targets (entry 20, `2`).
- Silence census of the new files and the shared module (entry 15): 0; `info_span!` sites in escher-driver (entry 16): 2;
  the env-read set (entry 14) as at the base.
- Coverage floor and stand-repeat: CI-only, read through the operator pass's CI entry (25).
