# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:8) The statement's command-line carrier must be stated as unix only and as a binary run once per command against a host its `start` spawned, as the E2E checks prove it.
  > **Command line (`escher-session`, unix only; since 2026-10-10-driver-cli):** each driver command is run as a process of the package's own binary against a host its `start` command spawned
- (.andromeda/test-plan-detail/e2e-test-strategy.md:8) What the statement says the command line can do must match the proven reach: all eight instance verbs accepted on it, and `lint` among them.
  > each of the eight instance-level verbs is accepted and answers exactly its fields
- (.andromeda/test-plan-detail/e2e-test-strategy.md:8) The statement's exit statuses must match the four endings the command-line checks assert.
  > a call a target cannot take is refused with its cause, status `1`; a call the schema refuses reaches no session; a command with no session is a session error, status `3`; a line that is no command is a usage error, status `2`;
- (.andromeda/test-plan-detail/e2e-test-strategy.md:8) Any snapshot size the statement quotes must match the measured stand readings and the 10,000-byte budget, with their dates kept.
  > a snapshot is the in-process text as one line inside its 10,000-byte budget (832 · 1,393 · 1,368 · 2,202 bytes on the four tasks, line break included, by hand smoke at 2026-10-10-snapshot-states; 832 · 1,375 · 1,368 · 2,187 before it)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:9) The statement's limits must include the state-directory path bound exactly as measured: 94 bytes on Linux, 90 off Linux, the latter witnessed on one macOS CI run.
  > a state directory of 94 bytes as given starts and stops while one of 95 ends `state-dir-too-long`, status `3`, its message naming 94 and its remedy the table's, and, since 2026-10-14-stand-requirement-sweep, off Linux one of 90 bytes starts and stops while one of 91 ends `state-dir-too-long`, status `3`, with no path in the message — its one witness the fork's CI run on the macOS leg, CI#38642493451, job 116123587286, `cli_commands` 11 passed on `aarch64-apple-darwin` (examples/seven_guis/tests/cli_commands.rs:708-712)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:9) A platform-reach claim for the command line's select-all and the shell flows on macOS must not go past its one witness.
  > the fork's CI run on the macOS leg is the one witness of the select-all modifier and of the two flows there
- (.andromeda/test-plan-detail/e2e-test-strategy.md:10) The statement's recording capability must match what the binary is proven to keep: one owner-only check in an owner-only directory, a group-open directory refused with nothing spawned.
  > `start <task> --record <dir>` run as a process keeps one `0600` check in a `0700` directory, the state directory gone after `stop`, a group-open recording directory refused status `3` with nothing spawned (seven_guis `recording`)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:10) The shot's cross-OS PNG difference must be stated as a limit as the E2E record holds it: a check compared byte for byte holds no `screenshot` step.
  > the scripts holding no `screenshot` step since a shot's PNG bytes differ by OS (CI#39225375577)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The MCP surface's tool count in the statement (and CARRY 1's README fix) must read twelve, as the MCP E2E check asserts the listing.
  > `mcp_server` (6): the Counter flow by tool calls alone (`initialize`, `tools/list` offering the verbs in order — twelve — `start`, `snapshot`, `click`)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) What an MCP client carries onward must be stated with its measured instance as it stands: one real-client trial, outside CI, its protocol reading taken before the server spoke `2025-11-25`.
  > a real client drove it once, in the operator pass, not in CI: one `claude -p` (2.1.296 on the dev host, the cold-agent pipe's flags, `--allowedTools mcp__escher`, the operator's own login) reading `trial: client_exit 0 · isolated true · tool_calls 7 · refused 0 · errors 0 · end_state reached · protocol_version 2025-11-25 · discover sent` — the client's first request `server/discover` (answered `-32601`), then `initialize` asking `2025-11-25`, answered `2025-06-18`, and the client went on — the trial's reading, taken before the server spoke `2025-11-25`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:12) A claim that a kept check is the same across the command line and MCP must match the recording E2E reading.
  > seven_guis `recording` speaks the role with `--record`: its listing and `help` the same bytes with and without it, and its kept check byte-identical to the command line's
- (.andromeda/test-plan-detail/e2e-test-strategy.md:19) The statement must not claim a windowed end-to-end run: the UI is driven through the binary, and no test launches a window.
  > still no test launches a window; since 2026-10-10-driver-cli the UI is driven through that binary
- (.andromeda/test-plan.md:286) The body's broad line for the command line, the committed checks and the shot's OS limit that the statement summarises.
  > - **Command line (`escher-session`, unix only):** each driver command is run as a process of the package's own binary against a host its `start` command spawned — `cli_commands`, the two shell flows of `cli_flow` and `host_timer`; a recorded session run as a process keeps its check, and the committed checks are read byte-equal to what the binary records, the scripts holding no `screenshot` step since a shot's PNG bytes differ by OS
- (.andromeda/test-plan.md:288) The body's broad line for the MCP surface's E2E reach, which the statement's MCP claims must not exceed.
  > - **MCP (`escher-session mcp`):** the binary's `mcp` role is spoken to as a child over piped stdin through the shared `Speaker` (`mcp_server`, `mcp_log`) and in process in both layout modes (`stand_mcp_tools`, `stand_mcp_masked`); a real client drove it once, in the operator pass, not in CI
- (.andromeda/test-plan.md:301) The statement's limits must carry the unmeasured windowed or browser-driven end-to-end run.
  > > NOT YET MEASURED — windowed or browser-driven end-to-end runs lay outside every slice read

## Bindings
- **MCP live trial ↔ body §3 cold-agent proof**: the one real-client trial (.andromeda/test-plan-detail/e2e-test-strategy.md:11) and the cold-agent live run, fired once with eleven tools and not re-fired since `lint` (.andromeda/test-plan.md:187), are together the measured instance of what an MCP client carries onward; the statement states both readings as taken, neither as a twelve-tool run.
- **Shot OS difference ↔ body §2 pixel tests**: the E2E scripts holding no `screenshot` step (.andromeda/test-plan-detail/e2e-test-strategy.md:10) and the body's rule that a check compared byte for byte across OSes holds no shot (.andromeda/test-plan.md:89) state one limit; the statement states it once.
- **No windowed run ↔ body §1 assistive-technology owed check**: no test launching a window (.andromeda/test-plan-detail/e2e-test-strategy.md:19; .andromeda/test-plan.md:301) and the accessibility states unread at an assistive technology, owed to its residuals line (.andromeda/test-plan.md:48), bound the statement's a11y reach to the headless half.
- **Twelve-tool listing ↔ CARRY 1 (README)**: the asserted MCP listing of twelve (.andromeda/test-plan-detail/e2e-test-strategy.md:11) is the value README's «eleven driver commands» must be corrected to.
- **State-directory bound ↔ security-plan §Surfaces / the schema's session errors**: the 94 / 90-byte `state-dir-too-long` readings (.andromeda/test-plan-detail/e2e-test-strategy.md:9) are the measured side of a session error the schema states; the statement's limit matches both.
