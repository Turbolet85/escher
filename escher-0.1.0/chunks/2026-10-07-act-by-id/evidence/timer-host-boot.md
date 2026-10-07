# Timer host boot — the one branch `session_host.rs` gained, booted for real

A hand-driven smoke at implement P3, on the dev host (Linux), 2026-10-07T14:00Z. It has no listed
producer: the plan's two smoke entries boot the `counter` host (`host_binary`) and the windowed
stand, and `host_log` boots `crud`, so no listed entry starts `escher-session` on the `timer`
task — the only task whose session is handed a time step (step 8).

## What was run

- The binary the gate run built: `target/debug/escher-session`, built after the last source edit.
- `target/debug/escher-session timer target/tmp/p3-timer`, `RUST_LOG=info`, stdin closed, stdout
  and stderr piped, under an outer bound of 60 s; the socket was awaited for at most 30 s and the
  exit for at most 10 s, with SIGTERM then SIGKILL held in reserve (neither was needed).
- Two lines sent over `session.sock`, each on its own connection, as the wire states them:
  `hello v1` twice, then `stop v1`.

## What it read

| reading | value |
|---|---|
| state directory mode while serving | `0700` |
| first `hello` reply | `ok v1 pid=<the spawned pid> label=timer served=0` |
| second `hello` reply | differs from the first (its count) |
| `stop` reply | `ok v1 stopping` |
| host exit code | `0` |
| host stdout | 0 bytes |
| host stderr | 1 line: `INFO escher_telemetry service.name=seven_guis service.version=0.1.0 message=telemetry installed` |
| state directory after exit | removed |
| `escher-session` processes after exit | none |

## What it proves, and what it does not

- The Timer host boots with the stand's time step handed to its session, serves, stops and exits
  0, printing nothing to stdout — the boot path step 8 edits runs.
- It does not prove the step moves time under the host: no command reaches the host's instance
  (the socket carries `hello` and `stop` only), so `advance` is proven in process only
  (`stand_act_timer`, `stand::tests`).
