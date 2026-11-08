@carried n=1 label=architecture/standard-contracts fact=fct-jelmr2gx
why: the new read is a new verb in `VERBS`, so `docs/capabilities.md` must name it as a code span and restate its verb count in words, and README.md's «the {word} driver commands as tools» must move with it, for the re-read pin to stay green.
quote: the capability-statement pin — `docs/capabilities.md` and README.md held to the driver schema's tables: `CapabilityStatementTest`, four cases in stdlib `unittest`, reads the verb names from `VERBS` (`schema.rs`), the cause names from `Cause::name`'s match arms (`refusal.rs`) and the lengths of `SESSION_ERRORS` (`error.rs`), `NODE_FIELDS` and `EXIT_STATUSES` (`schema.rs`), and asserts every verb and every cause a code span of the statement, each count stated in words — `{word} verbs` · `{word} refusal causes` · `{word} session errors` · `{word} node keys` · `{word} exit statuses`, the markdown's whitespace folded so a phrase may wrap a line — and README.md holding «the {word} driver commands as tools»; it reads by path from the repository root, writes nothing and reads no env var
@carried n=1 run=2 label=architecture fact=fct-ksgwc2yv
quote: the command and refusal schema stated once as `'static` data — thirteen verbs, eight closed causes, no refusal holding what a call supplied;
@carried n=2 label=architecture/standard-contracts fact=fct-jelmr2gx
why: a new verb on the MCP listing changes the escher server's tool count, so the cold-agent pipe's isolation check, which asserts thirteen `mcp__escher__{verb}` tools, must count the new tool or the live run fails isolation.
quote: isolated — the thirteen `mcp__escher__{verb}` tools, one server `escher` connected —,
@carried n=2 run=2 label=architecture/standard-contracts fact=fct-jelmr2gx
quote: only the escher server's thirteen tools
@carried n=3 label=architecture/standard-contracts fact=fct-jelmr2gx
why: the read is proven on the stand and through the command line, MCP and a kept check, whose seven_guis targets and `stand_*.rs` checks run under the stand-repeat leg's zero-failure budget, so the chunk's new checks must pass three rounds with no retry.
quote: then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and `recording`; the first red command ends the leg with its status — a zero-failure budget, no retry, no `--ignored`, no env var — and three green rounds end `stand-repeat: 3 of 3 rounds green`
@carried n=4 label=architecture/standard-contracts fact=fct-jelmr2gx
why: the chunk adds code to escher-driver and may add a reader to dioxus-native-dom, so its tests must keep those crates at or above their coverage floors.
quote: the `coverage` leg calls it with `packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`
@binding n=1
**standard-contracts (capability-statement pin) ↔ the body's driver session contract**: the new verb raises the verb count the schema states once as static data (architecture fct-ksgwc2yv), and `docs/capabilities.md`'s `{word} verbs` and README.md's «the {word} driver commands as tools» must state the same new count (architecture/standard-contracts fct-jelmr2gx).
@binding n=2
**standard-contracts (cold-agent isolation count) ↔ the plan's MCP listing**: a new tool in `mcp/listing.rs` must be matched by the cold-agent pipe's tool count, which today asserts thirteen `mcp__escher__{verb}` tools (architecture/standard-contracts fct-jelmr2gx).
@binding n=3
**standard-contracts (capability-statement pin) ↔ the body's front page**: README.md's command count is read by the pin, and the front page's own rule names that one test (architecture fct-3eyld63q), so the count must change in README.md and in `docs/capabilities.md` together.
