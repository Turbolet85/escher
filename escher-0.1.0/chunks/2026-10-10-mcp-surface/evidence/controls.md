# Controls — 2026-10-10-mcp-surface

Readings /implement made by hand: the measurements taken before any source moved (plan step 1), and each named
check's mutation control. Paths under the repository are written relative to its root.

## Step 1 — measured before any source change (2026-10-11T07:5xZ, tree at base `69a399da`, no source delta)

- `cargo tree --locked -p escher-driver -e normal -i serde_json` → exit 0, first line `serde_json v1.0.151`
  (`└── dioxus-html v0.7.10` → `dioxus-native-dom v0.7.0` → `blitz-test-harness` → `escher-driver`).
- `cargo tree --locked -p seven_guis -e normal -i serde_json` → first line `serde_json v1.0.151`
  (`├── dioxus-document v0.7.10` → `dioxus-native v0.7.0`).
- The base's workspace test count, re-read from `target/ci-logs/test.log` (written 2026-10-10T19:10:38Z by
  `ci-leg.sh fast`, 50 s before the base commit; `git diff --stat 69a399da` over `packages tests examples scripts
  .github apps wpt Cargo.toml Cargo.lock` is empty, so the log is of the base's source): **167** `test result:`
  lines, **791** passed, **0** failed, **11** ignored — the standing figures, re-read by a parse of every
  `test result:` line, not copied.

## Mutation controls (2026-10-11T08:1xZ, after the first full green pass)

Each mutation written over `packages/escher-driver/src/mcp.rs`, its check run, the file's saved bytes written back
and its sha256 compared before the next (sha256 `0c2fd5e104f538a6…` matched after the batch). Every check read red
under its mutation; entries 1–5 and 10 were re-run green after the restore (the trail's second firing of each).

| mutation | check run | reading |
|---|---|---|
| M1 the `tool_call` span made but not entered | `cargo test -p blitz-tests --locked --test stand_mcp_tools` | exit 101 · 1 test FAILED |
| M2 a `snapshot` result written as its JSON line | `cargo test -p seven_guis --locked --test mcp_server` | exit 101 · 2 tests FAILED |
| M3 the string `id` echo bound widened to 129 bytes | `cargo test -p escher-driver --locked` | exit 101 · 1 test FAILED |
| M4 the 65,536-byte request bound not applied | `cargo test -p escher-driver --locked` | exit 101 · 1 test FAILED |
| M5 the `tool_call` span under a target other than `escher_driver` | `cargo test -p seven_guis --locked --test mcp_log` | exit 101 · 1 test FAILED |

## The live-trial script, dry-run against a `claude` stand-in (no model reached)

`evidence/live-trial.py` was run once with a stand-in `claude` first on PATH (a scratch script, not kept) that spawns
the one server the script's `mcp.json` names, drives the Counter to 3 over it (with one refused click) and writes a
stream-json transcript of the shape the script reads. Printed:
`trial: client_exit 0 · isolated true · tool_calls 7 · refused 1 · errors 0 · end_state reached · protocol_version 2025-06-18 · discover not sent`
— exit 0. It proves the `sh -c` server line (requests teed, `exec` of the binary, stderr to the log) and the readers;
it is not the trial. The live run is the operator pass's entry 18. The dry run's files stand in `target/mcp-trial/`
(removing them was denied in this session); the script removes those four files before it runs.
