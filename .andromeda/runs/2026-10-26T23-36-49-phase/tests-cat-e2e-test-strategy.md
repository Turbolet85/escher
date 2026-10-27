# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:8) The command-line witness of the lint must stay green with no change: after `lint.rs::findings` is factored, a hosted `lint` must still answer status 0 and the same one line, and `cli_commands` keeps its count (14).
  > a hosted `lint` answers status `0` and one line `{"findings":[]}` (examples/seven_guis/tests/cli_commands.rs:419-440);
- (.andromeda/test-plan-detail/e2e-test-strategy.md:8) The factoring must leave the `lint` verb's answer fields unchanged on the command line, so that every instance verb still answers exactly its fields.
  > each of the eight instance-level verbs is accepted and answers exactly its fields
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The MCP process check of the listing reads the same `tools/list` rows that the refactored `mcp_tests.rs` listing test asserts: twelve verbs in `VERBS` order. Both checks must keep reading those rows, and `mcp_server` keeps its count (6).
  > `mcp_server` (6): the Counter flow by tool calls alone (`initialize`, `tools/list` offering the verbs in order — twelve — `start`, `snapshot`, `click`)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The MCP process witness of the lint must stay green: a `lint` tool call must still answer the same line of findings after `findings` is factored.
  > a `lint` tool call answering its line of findings (examples/seven_guis/tests/mcp_server.rs:340-358)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The in-process MCP witness of the lint must stay green in both layout modes. That means the reader's same findings and the same `command` span parentage, because the chunk changes no span or field.
  > `stand_mcp_tools` (6, its sixth a `lint` tool call answering the reader's findings and parenting its `command` span; tests/blitz-tests/tests/stand_mcp_tools.rs:706-766)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:12) The tool listing that the refactored listing test reads must stay byte-for-byte the same, with and without `--record`. Only the test is restructured, and the listing it asserts does not move.
  > its listing and `help` the same bytes with and without it, and its kept check byte-identical to the command line's (examples/seven_guis/tests/recording.rs:111-331)
- (.andromeda/test-plan.md:289) The command-line E2E driver of the lint verb must pass unchanged. It runs each driver command as a process of the binary.
  > - **Command line (`escher-session`, unix only):** each driver command is run as a process of the package's own binary against a host its `start` command spawned — `cli_commands`, the two shell flows of `cli_flow` and `host_timer`
- (.andromeda/test-plan.md:291) The MCP E2E drivers of the listing and the lint must pass unchanged: as a child process and in process, in both layout modes.
  > - **MCP (`escher-session mcp`):** the binary's `mcp` role is spoken to as a child over piped stdin through the shared `Speaker` (`mcp_server`, `mcp_log`) and in process in both layout modes (`stand_mcp_tools`, `stand_mcp_masked`)

## Bindings
- **The MCP listing's E2E rows ↔ the body's §4 escher-driver unit tests**: the process check `mcp_server` reads `tools/list` as twelve verbs in order (.andromeda/test-plan-detail/e2e-test-strategy.md:11). The unit test `the_listing_is_the_twelve_verbs_of_the_table` refactored in this chunk is one of the MCP unit pins (.andromeda/test-plan.md:208). Both must assert the same rows.
- **The lint's E2E witnesses ↔ the body's §9 engine features by runner**: the `cli_commands` and `stand_mcp_tools` lint witnesses (.andromeda/test-plan-detail/e2e-test-strategy.md:8; .andromeda/test-plan-detail/e2e-test-strategy.md:11) must be green in the workspace build, not only in a per-package run. The body says a green per-package run is not the workspace leg's green (.andromeda/test-plan.md:392).
- **The E2E process checks ↔ the body's §10 flakiness budget**: seven_guis' process checks (`cli_commands`, `mcp_server`) run three rounds with no retry in `stand-repeat` (.andromeda/test-plan.md:413). `stand_requirement_sweep`'s refactored test asserts that each 0.2.0 requirement names its witness inside that leg (.andromeda/test-plan.md:377).
