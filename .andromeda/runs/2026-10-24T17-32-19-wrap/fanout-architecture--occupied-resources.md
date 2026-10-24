proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 25
    section: "## Occupied Resources"
    change: "In the session checks' state-directory list, read `hb-*`, `hl-trace`, `hl-client`, `hl-client-rec`, `cc-*`, `cf-*` and `ht-timer` (seven_guis: `host_binary`, `host_log`, `cli_commands`, `cli_flow`, `host_timer`) under `CARGO_TARGET_TMPDIR`, i.e. `target/tmp/`, and add to the line's citations `examples/seven_guis/tests/host_log.rs:40-130` and `hl-client` and `hl-client-rec` at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md — the two state directories of `host_log`'s `a_recording_sessions_clients_log_nothing_of_it_at_trace` (`start crud --record` run as a client), local only and uploaded by no CI leg."
    sidecar: "Occupied Resources › Filesystem: the session checks' state directories gain `hl-client` and `hl-client-rec` (seven_guis `host_log`, under `target/tmp/`), as measured at 2026-10-24-stand-requirement-sweep."
    rationale: "The report's Changes (`host_log.rs`, 40-130) land two new test state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`; Counts fact 9 (`state-dirs`) names `occupied-resources.md:25` Moved, and Outcome says 'an arch §Occupied Resources filesystem line owed at P2 (Count 9, Expected amendments)'. The list at line 25 names `hl-trace` alone for `host_log` — an unregistered resource."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:172
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 42
    section: "## Occupied Resources"
    change: "After '`cli_commands` runs the binary once per command as a client, its sessions started by the binary's own `start` command,' add: 'and since 2026-10-24-stand-requirement-sweep `host_log`'s unix-only `a_recording_sessions_clients_log_nothing_of_it_at_trace` does the same through `common::run`, `RUST_LOG=trace` set on each client — `start crud --record`, `type`, `snapshot`, `snapshot --part 1`, `lint`, `stop` — in `hl-client` and `hl-client-rec`, its session guarded by `common::Started { dir, pid }` (examples/seven_guis/tests/host_log.rs:40-130),' — the eight targets unchanged in number."
    sidecar: "Occupied Resources › Process-wide state and threads: seven_guis `host_log` also runs the `escher-session` binary as a client per command (`start crud --record` … `stop`, `RUST_LOG=trace` on each), as measured at 2026-10-24-stand-requirement-sweep."
    rationale: "The report's Changes (`host_log.rs`): the new test 'runs each client command through `common::run` with `RUST_LOG=trace`, in the state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`, guarded by `common::Started { dir, pid }`', commands `start crud --record` · `type` · `snapshot` · `snapshot --part 1` · `lint` · `stop`. Line 42 describes `host_log` only as spawning the host role itself, so the client-run spawn of the binary (and its `start`-spawned host) is new process detail for this entry; no target is added (Crates / modules: none)."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:59
note: "hl-rec / hl-rec-kept stay unlisted at line 25 (a pre-existing gap the report names at fact 9, not this chunk's resource, left for a basis-bearing read); the host_log.rs:22 · :156 · :156-175 citations at lines 25 and 43 sit after the added ranges 11-15 · 40-131 and are left to cites.py; D-arch-decisions: no new dependency (Dependencies: none) — no drift."
