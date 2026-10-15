group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/escher-driver.md:22   class: omission
  leaf-text: «the answer is read under 30 s per read.»
  source: .andromeda/architecture.md:133 «the answer is read under a 30 s bound on each read (`Timeout` past it; pinned since 2026-10-14-quality-gates by the unix-only `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout`»
  replace-with: «the answer is read under 30 s per read (`Timeout` past it; pinned since 2026-10-14-quality-gates by the unix-only unit test `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout`, which reads the bound equal to `wire::ANSWER_BOUND` and a read past a shortened bound on a listener that never answers as `Timeout`).»

- leaf: .claude/docs/services/escher-driver.md:36   class: omission
  leaf-text: «a write to a closed pipe is dropped.»
  source: .andromeda/architecture.md:133 «a write to a closed pipe is dropped, not a panic — pinned since 2026-10-14-quality-gates by `cli_commands`' `a_closed_stdout_drops_the_answer_and_keeps_the_ending`»
  replace-with: «a write to a closed pipe is dropped, not a panic — pinned since 2026-10-14-quality-gates by `cli_commands`' `a_closed_stdout_drops_the_answer_and_keeps_the_ending` (with its stdout's read end closed, `help` exits 0 and `snapshot` with no session 3, neither printing a panic).»

- leaf: .claude/docs/services/escher-driver.md:46   class: omission
  leaf-text: «- A test's working directory is its package: build a check's state directory from `env!("CARGO_TARGET_TMPDIR")`, never from a relative path.»
  source: .andromeda/architecture.md:146 «beside it, since 2026-10-14-quality-gates, one test-only bind: escher-driver's unix-only unit test `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout` binds a `UnixListener` in `eab-{pid}`»
  replace-with: «- A test's working directory is its package: build a check's state directory from `env!("CARGO_TARGET_TMPDIR")`, never from a relative path. The one exception, since 2026-10-14-quality-gates, is the unix-only client unit test `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout`: it binds a test-only `UnixListener` in `eab-{pid}` under the system temporary directory (`std::env::temp_dir()`, reading `TMPDIR` on unix — test code only), a short name so the socket's path fits, accepts one connection, never answers and removes the directory; no binary serves it and no caller reaches it, so it is no crossing (the operator, 2026-10-15).»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «no id, name or typed text, none at the default level)»
  source: .andromeda/architecture.md:133 «since 2026-10-14-quality-gates a not-settled call is read on a real captured line — `stand_act_spans` drives `press tab` on an in-file fixture that mounts a freshly keyed element on every render»
  replace-with: «no id, name or typed text, none at the default level; since 2026-10-14-quality-gates a not-settled call read on a real captured line — `press tab` on an in-file fixture that mounts a freshly keyed element on every render, in both layout modes, its `command` line carrying `settled=false` and `busy="render"`, no `passes` and no `cause`, six of the eight fields)»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «`tests/cli_commands.rs` runs every driver command as a process on the four tasks;»
  source: .andromeda/architecture.md:133 «a write to a closed pipe is dropped, not a panic — pinned since 2026-10-14-quality-gates by `cli_commands`' `a_closed_stdout_drops_the_answer_and_keeps_the_ending`»
  replace-with: «`tests/cli_commands.rs` runs every driver command as a process on the four tasks and, since 2026-10-14-quality-gates, pins that a write to a closed stdout is dropped (`a_closed_stdout_drops_the_answer_and_keeps_the_ending`: `help` exits 0 and `snapshot` with no session 3, neither printing a panic);»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «the two MCP targets run on windows too.»
  source: .andromeda/architecture.md:141 «then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server` and `mcp_log`; the first red command ends the leg with its status»
  replace-with: «the two MCP targets run on windows too. Since 2026-10-14-quality-gates the `stand-repeat` CI leg runs these seven by name (`cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log`) in each of its three rounds — a zero-failure budget, no retry.»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «They assert unconditionally under the bundled font — no font skip, no sleep, no `dispatch_recorded`.»
  source: .andromeda/architecture.md:141 «`stand-repeat` (since 2026-10-14-quality-gates, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs`»
  replace-with: «They assert unconditionally under the bundled font — no font skip, no sleep, no `dispatch_recorded`. Since 2026-10-14-quality-gates the `stand-repeat` CI leg (not a fast leg) runs every `tests/blitz-tests/tests/stand_*.rs`, one `--test` each, in three rounds — a zero-failure budget, no retry, no `--ignored`.»
