# tests · cascade read · group 1 of 4

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:8) The rewritten `docs/capabilities.md` and README.md must keep the capability statement's pin green: every verb and cause name, the five counts, and README's MCP tool count held to the driver schema's tables, with the `ci-scripts` leg's count of tests unmoved since the chunk adds none.
  > the four of `CapabilityStatementTest`, which pin `docs/capabilities.md`'s verb and cause names and its five counts, and README.md's count of the MCP tools, to the driver schema's tables — `docs/capabilities.md`, a document with no runtime surface, in test scope through this pin alone (.github/scripts/test_capability_statement.py:75-104)
  (.andromeda/test-plan-detail/test-scope-summary.md:8)
  > unit-tested with `python3 -m unittest discover -s .github/scripts` as the fast `ci-scripts` leg — 108 tests in eight test files
  (.andromeda/test-plan.md:11)
  > - **CI Python scripts:** unit-tested with `python3 -m unittest discover -s .github/scripts` as the fast `ci-scripts` leg (§4), among them the contract tests of the CI install script `.github/scripts/apt-install.sh`, of the coverage floor script and of the blitz-tests manifest, the capability statement's pin over `docs/capabilities.md` and README.md, and the workflow invariant that every package install of ci.yml is bounded
- (.andromeda/test-plan-detail/test-scope-summary.md:60) Every count the statement and the README state in words must be the schema's own, as its unit tests pin them: twelve verbs, thirteen node keys, eight causes, four exit statuses and fifteen session errors; the chunk moves none.
  > describe.rs 2 (the document holds `help`'s seven fields in order; every text in it is its tables'; its count row reads `[12, 13, 12, 3, 8, 4, 15]`)
  (.andromeda/test-plan-detail/test-scope-summary.md:57)
  > the verbs are named in order, each at its level — eight instance-level (`screenshot` the seventh, after `scroll`, and `lint` the eighth, after `screenshot`), then `start` · `status` · `stop`, then, since 2026-10-12-self-description, `help` at the tool level, twelve;
  (.andromeda/test-plan-detail/test-scope-summary.md:57)
  > refusal.rs 3 (the causes are named in order — eight (`the_eight_causes_are_named_in_order`, ratified by the founder, 2026-10-18), `fonts-unavailable` last;
  (.andromeda/test-plan-detail/test-scope-summary.md:58)
  > since that chunk the thirteen node keys are named in order, each with its help, and the four exit statuses are stated in status order)
  (.andromeda/test-plan-detail/test-scope-summary.md:56)
  > every `SessionError` message — all fifteen variants (thirteen until 2026-10-19-session-recording)
- (.andromeda/test-plan-detail/test-scope-summary.md:16) The README's MCP form and its "the twelve driver commands as tools" must state what `mcp_server` proves of the `mcp` role — the twelve verbs listed in order — and its command-line flow what `cli_flow`'s two shell flows run.
  > `tests/mcp_server.rs` (6 tests: the binary's `mcp` role completes the Counter flow by tool calls alone — `initialize`, `tools/list` offering the verbs in order, twelve, `start`, `snapshot`, `click` — answers a `lint` tool call with its line of findings
  (.andromeda/test-plan-detail/test-scope-summary.md:16)
  > `tests/cli_flow.rs` (2, unix only: the two shell flows of `tests/flows/` under plain `sh`)
  (.andromeda/test-plan.md:15)
  > `examples/seven_guis` holds integration-test targets over its `escher-session` binary (host, log, command line, shell flows, timer host, MCP, recording)
- (.andromeda/test-plan-detail/test-scope-summary.md:15) The README's "Try it" must show only the `escher-session` command-line and host forms that `cli_commands` and `host_binary` run as a process.
  > `tests/cli_commands.rs` (13 tests, unix only: every driver command run as a process of the binary — §6 → Drivers per surface;
  (.andromeda/test-plan-detail/test-scope-summary.md:15)
  > the `escher-session` binary in its host role, `serve <task> --session <dir> [--record <dir>]`, serves a counter session
- (.andromeda/test-plan-detail/test-scope-summary.md:18) The README must name only the six tasks the binary serves, and its sentence on a session recorded as a replayable check must hold to what `recording` proves of the two committed checks.
  > `cli_commands` starts, reads and stops all six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`)
  (.andromeda/test-plan-detail/test-scope-summary.md:18)
  > the committed checks what the binary records, `record_checks.sh` run through the binary and both files compared byte for byte with the committed copies (examples/seven_guis/tests/recording.rs:205-262); off unix, `--record` refused `Unsupported`
  (.andromeda/test-plan-detail/test-scope-summary.md:17)
  > `tests/recording.rs` (4 tests, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay) — the command line and the `mcp` role keep the same check byte for byte, the state directory gone after `stop`;
- (.andromeda/test-plan.md:188) The README's cold-agent result and the statement's restated paragraph must read every figure from this recorded proof of the 2026-10-25 live run — twelve tools, six apps, 66 calls, 0 wrong against the 5 % bar, one lint, one screenshot — and claim nothing it does not hold.
  > - **Proof:** the contract tests `test_cold_agent.py` (§4, 37) and the live run on the operator host, never part of CI (no CI secret), fired once: `passed`, reasons none, `isolated` (the twelve `mcp__escher__*` tools, one server `escher`), `counts_agree` (66 · 66), `tasks_met` all six, `lint_calls` 1 · `screenshot_calls` 1 · `advance_calls` 1 · `part_reads` 2, `recorded_sessions` 6 with `recording_kept`, `check_passed` and `check_can_fail` (59 steps, 16 expects), 66 tool calls (advance 1 · click 14 · lint 1 · press 13 · screenshot 1 · scroll 1 · snapshot 13 · start 6 · stop 6 · type 10), 0 wrong of 66 against the 5 % bar, 0.8244 USD, `claude` 2.1.308 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json and evidence/live-run.md
  (.andromeda/test-plan.md:183)
  > the agent talks to the driver's own MCP surface, `escher-session mcp` (the twelve verbs)
- (.andromeda/test-plan.md:180) The README and the statement must name the run's six apps as the pipe's one task drives them, in one client session.
  > the one task `stand` — six apps, counter, flight booker, timer, CRUD, cells and the load page, in one client session
- (.andromeda/test-plan.md:185) A "0 wrong" the README or the statement states must mean the verdict's `wrong_calls` as defined, judged against the 5 % bar.
  > `wrong_calls` = refused + error + foreign `tool_use` blocks + `permission_denials`, deciding the outcome against `wrong_call_bar_pct` (5);
- (.andromeda/test-plan.md:187) The README and the statement must cite the run by verdict counts, booleans and closed words only, quoting, copying or committing nothing of the pipe's state.
  > `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json, record/}` only — `record/` (`mkdir -m 700`) the live server child's kept checks, never printed, copied or committed —
  (.andromeda/test-plan.md:186)
  > No event or verdict field is content-named, and no transcript text, tool argument, tool result, reply or check text reaches one

## Bindings
- **test-scope-summary's pin ↔ the driver schema's counts**: the pin that holds `docs/capabilities.md` and README.md (.andromeda/test-plan-detail/test-scope-summary.md:8) reads the same tables whose counts the escher-driver unit tests pin (.andromeda/test-plan-detail/test-scope-summary.md:60), so every count the rewritten documents state in words must be those tables' values.
- **test-scope-summary's seven_guis process checks ↔ §3 Cold-agent run pipe**: the twelve tools `mcp_server` lists in order (.andromeda/test-plan-detail/test-scope-summary.md:16) are the twelve `mcp__escher__*` tools the live run's proof records (.andromeda/test-plan.md:188), and the six tasks `cli_commands` serves (.andromeda/test-plan-detail/test-scope-summary.md:18) are the six apps the run drives (.andromeda/test-plan.md:180); the README's tool count, "Try it" and cold-agent result must agree with both sides.
- **§3 Cold-agent run pipe's Proof ↔ its State**: the figures the documents state come from the recorded verdict (.andromeda/test-plan.md:188), while the pipe's state it was read from is never printed, copied or committed (.andromeda/test-plan.md:187).
