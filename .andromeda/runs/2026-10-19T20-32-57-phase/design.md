# design extract

## Relevance
partial — no rendered UI, token, typography, motion or colour is touched; the one design surface in reach is the `escher-session` agent-read output contract (per design-system §Surface: cli / Component Patterns), which the moved `cli.rs` tests and the edited seven_guis shared test helpers pin, and which the chunk must leave unchanged.

## Constraints
- design-system §Surface: cli / Component Patterns requires `escher-session` to print exactly one line of JSON on stdout per command (answer, refusal or session error object, keyed by the schema's own words) and nothing else there; the tests leaving `packages/escher-driver/src/cli.rs` for a sibling file keep every assertion of that shape, bodies unchanged (per design-system §Surface: cli / Component Patterns).
- design-system §Surface: cli / Component Patterns requires a usage error to print one fixed line on stderr with stdout empty, and a session error its fixed message on stderr with a fixed `remedy` in its object; any moved test pinning those readings keeps its name and assertion (per design-system §Surface: cli / Component Patterns).
- design-system §Surface: cli / Component Patterns requires the output to carry no colour, no colour dependency, no ESC byte on any answer line and no terminal test, so it reads the same piped or not; the plan names the seven_guis checks' shared reader and the shared `Speaker` (both in `examples/seven_guis/tests/common/mod.rs`) as the assertions of the no-ESC reading — the shared log-check fragment added to that file must not weaken or reroute them (per design-system §Surface: cli / Component Patterns).
- design-system §Surface: cli / Component Patterns requires the exit status to carry the class — `0` accepted · `1` refused · `2` usage · `3` session error, read from the schema's `EXIT_STATUSES` table; tests pinning those statuses move unchanged (per design-system §Surface: cli / Component Patterns).
- design-system §Surface: cli / Component Patterns requires the `mcp` role's stdout to hold newline-delimited JSON-RPC 2.0 protocol lines only, each reply one uncoloured line, and an `Err` ending's fixed message on stderr with status `3`; `mcp_log.rs`'s checks folded into the shared fragment keep reading that split (per design-system §Surface: cli / Component Patterns).

## Patterns to follow
- The `escher-session` output contract as stated in design-system §Surface: cli / Component Patterns — agent-read, one uncoloured JSON line on stdout, fixed stderr lines, class by exit status — is the behaviour every moved or deduplicated test continues to assert; whether `cli.rs`'s inline tests and `host_log.rs`/`mcp_log.rs` currently assert any of these readings is research's question.
- One shared reader in `examples/seven_guis/tests/common/mod.rs` already carries the no-ESC assertion for every answer line and every MCP reply (per design-system §Surface: cli / Component Patterns); the new shared log-check fragment sits beside those helpers in the same file rather than in a new module.

## Anti-patterns to avoid
- No colour, ESC byte or terminal test introduced anywhere on the `escher-session` stdout path, a test helper included — the surface is defined as uncoloured and terminal-agnostic (per design-system §Surface: cli / Component Patterns).
- No change to what any `escher-session` or `mcp` role line says on stdout or stderr as a side effect of the split — the chunk is behaviour-unchanged and the output contract is a design surface (per design-system §Surface: cli / Component Patterns).

## Contract bindings
- design ↔ tests: design-system §Surface: cli / Component Patterns names the seven_guis shared reader and `Speaker` in `examples/seven_guis/tests/common/mod.rs` as the witnesses of the no-ESC, one-line output readings; the chunk edits that file, so those witnesses must survive intact (test-plan's harness owns the file's contract).
- design ↔ obs: the stderr lines design-system §Surface: cli / Component Patterns fixes (usage line, session error message, `mcp` `Err` ending) share stderr with the telemetry sink that `host_log.rs`/`mcp_log.rs` check; the shared fragment must keep distinguishing the fixed CLI lines from log records exactly as both checks do today (obs-plan owns the sink rules).

## Acceptance criteria contributions
- (design) After the split, every test that asserts an `escher-session` answer line is one JSON line with no ESC byte still runs under the same name with the same assertion (per design-system §Surface: cli / Component Patterns).
- (design) No source line under `packages/escher-driver/src/` or `examples/seven_guis/` gains a colour dependency, an ESC byte literal on an output path or a terminal test (per design-system §Surface: cli / Component Patterns).
- (design) The `mcp` role's replies, as read by the shared `Speaker` after the fragment move, are still asserted one uncoloured line each with no ESC byte (per design-system §Surface: cli / Component Patterns).
