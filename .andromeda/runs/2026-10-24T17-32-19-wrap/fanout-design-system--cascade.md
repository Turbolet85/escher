proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 122
    section: "## Surface: cli › ### Component Patterns"
    change: "after «and a session error its fixed message on stderr;» add «since 2026-10-24-stand-requirement-sweep the client's own stderr is read by a standing check — seven_guis `host_log`'s `a_recording_sessions_clients_log_nothing_of_it_at_trace` runs `start crud --record`, `type`, `snapshot`, `snapshot --part 1`, `lint` and `stop` at `RUST_LOG=trace` and reads 0 id, name, typed-sentinel, recording-path and recorded needles over all the clients' stderr (examples/seven_guis/tests/host_log.rs:40-130; as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md);»"
    sidecar: "surface#15.1 (cli-output): the escher-session client's own stderr gains a standing check — host_log's a_recording_sessions_clients_log_nothing_of_it_at_trace, 0 needles at trace under start --record (host_log 2 → 3)"
    rationale: "The report's Symbols / APIs (host_log.rs) and Count 8 state a new standing check of the `start --record` clients' own stderr at `trace` with 0 id, name, sentinel, recording-path and recorded needles (host_log 2 → 3); the entity states what the client writes on stderr and names no check of it. The stdout rule, the exit statuses and the paged `{\"text\",\"part\",\"parts\"}` shape are unchanged (the task's Untouched and Touches), so only this clause is added."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:59
