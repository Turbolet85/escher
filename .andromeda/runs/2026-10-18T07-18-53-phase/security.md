# security extract

## Relevance
partial — the chunk is an engine timing change. It crosses no trust boundary and adds no input class. It does touch the driver's validated `advance` surface, its fixed refusal and help text, the command span's field set and the panic paths the plan records on the engine's wall-clock reads.

## Constraints
- `advance`'s argument keeps its recorded kind and bound, `milliseconds`, a whole number from 1 to 60000. Validation and the executor read no clock; the plan places the clocks in the host (idle expiry) and the MCP server. A virtual clock must therefore be moved by what the session's caller hands over and must never be read from wall time inside the executor, schema or validation. Adding a verb, an argument kind or a result field, or a wait, widens the validated surface and is not a silent change (per security-plan §Input Validation, Driver command schema row).
- The refusal set is closed at nine causes, each with a fixed `'static` name, meaning and remedy that holds nothing a call supplies. The plan also records that all nine are returned by code, `advance` without a time step returning `time-unavailable`. If one clock lets `advance` answer on every task, that recorded fact changes, and so may `time-unavailable`'s meaning text. Any rewording stays a fixed `'static` string. No cause is added or renamed without a ruling (per security-plan §Error Handling, Error format).
- The command span keeps exactly its eight fields of fixed words, bools and counts (`verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`). `advanced_ms`, a shot's image, its size and its dimensions are named as never fielded, so no clock reading or time value becomes a field (per security-plan §Input Validation, Driver command schema row).
- A settle reads no clock and never waits on a load, and its pass loop is bounded at 64. A `screenshot` resolves at the harness's own time with no tick and no pump, so it moves no time. Time on a held instance moves only when a caller asks with `advance`. The one clock must keep all three properties (per security-plan §API Security, Settle and loads in flight row).
- That same row records the overlay-scrollbar fade's wall-clock limit as stated in `screenshot`'s help and owned by a residual. Removing or rewriting that sentence touches the program's own fixed table text, which `describe()` serves and the command line and MCP carry. It must stay a fixed `'static` string holding nothing from a page, a call or a screen (per security-plan §API Security, Settle and loads in flight row; per security-plan §Error Handling, Error format).
- The plan lists the engine's time reads that `unwrap` the duration since the Unix epoch as panic paths (`scrolling.rs:515-518`, `pointer.rs:717-720`). A replacement clock read must add no `unwrap`, `expect` or `panic!` on anything a call supplies, and that includes the `ms` of `advance`, whose reported time is clamped to the `ms` asked (per security-plan §Error Handling, Panic paths; per security-plan §Input Validation, Driver command schema row).
- The escher-driver library and the `escher-session` binary read no env var beyond `RUST_LOG`. The workspace binds one listener. A clock mode chosen by an env var, a new crate or a new listener is a registration (arch §Occupied Resources plus the security plan), never a silent add (per security-plan §Secret Management, Environment values read; per security-plan §API Security).

## Patterns to follow
- The time step reaches a session as a value its caller supplies (`Session::with_time`). `advance` without one is refused `time-unavailable` with nothing run (per security-plan §Input Validation, Driver command schema row). One clock would follow the same shape: time handed in by the session's owner and read from no ambient wall clock.
- A refused call changes nothing by construction. `validate` takes no `Session`, and executor refusals leave the snapshot and both focus readings as they were (per security-plan §Input Validation, Driver command schema row).
- `scrollbar_activity` is keyed by `NodeId`, and a stale id must resolve to `None` through `get` rather than panic (per security-plan §Error Handling, Graceful degradation).
- The tool states its own limits in its fixed table text (`describe()`, `SESSION_ERRORS`, the cause texts), which rides the ratified carriers as the program's own `'static` text (per security-plan §Error Handling, Error format).

## Anti-patterns to avoid
- Fielding a time in the `command` or `tool_call` span or in a new log site: a clock value, `advanced_ms`, or a time a screen reads (per security-plan §Input Validation, Driver command schema row; per security-plan §Logging & Monitoring).
- Reading the clock mode or a time step from an env var, or adding a dependency, without registering it and reviewing it by hand (per security-plan §Secret Management; per security-plan §Dependency Security, Pinning).
- Letting a driver verb wait. No command waits on a load, and no instance verb takes a wait argument or answers a wait field (per security-plan §API Security, Settle and loads in flight row).

## Contract bindings
- security ↔ obs ↔ tests: the 0-needle stderr captures `host_log` and `mcp_log` (both builds, `RUST_LOG=trace`) and `stand_act_spans` (the span's field set, in-process capture) carry this domain's no-content-in-logs rule. A new engine tracing site must keep to the sink's safe-field rule (per security-plan §Logging & Monitoring).
- security ↔ arch: any new env var, listener or crate is an arch §Occupied Resources registration before it lands (per security-plan §Secret Management; per security-plan §API Security).
- security ↔ tests: `screenshot`'s help text is served by `describe()` and pinned by escher-driver's schema tests. A rewrite of it, or of `time-unavailable`'s texts, moves those pins (per security-plan §Error Handling, Error format).

## Acceptance criteria contributions
- An `advance` call's command span still records only the eight recorded fields, with no `advanced_ms` or other time field, and `advance --ms 0` and `--ms 60001` are still refused `malformed` with nothing run (per security-plan §Input Validation, Driver command schema row).
- `host_log` and `mcp_log` stay green under both builds: 0 typed-sentinel occurrences, 0 id needles, 0 name needles and 0 of a shot's first 64 base64 characters at `RUST_LOG=trace` (per security-plan §Logging & Monitoring).
- After the change, the help and cause texts that `describe()` serves are fixed `'static` strings holding no path and nothing from a call or a screen, and the cause set is still nine names (per security-plan §Error Handling, Error format).
- The audit leg reads `advisories ok` and `licenses ok` with `deny.toml` unchanged. A package entering `Cargo.lock` carries a recorded hand review (per security-plan §Dependency Security).
