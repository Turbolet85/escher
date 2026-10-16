# security extract

## Relevance
partial: the chunk adds no surface, input class, dependency or env read, but it moves two files that hold the driver's validated external-input boundary (`packages/escher-driver/src/mcp.rs`, `packages/escher-driver/src/schema.rs`), so every bound and ban on them has to come through the split unchanged.

## Constraints
- The MCP server's boundary has to stay byte-for-byte in force across whatever submodules `mcp.rs` becomes. That covers the request-line bound, the `id` echo forms, the fixed protocol-error codes and messages with no `data`, notifications running nothing, the result bound through `ToolResult::bounded`, and replies written by the crate's hand-written JSON writer only, with `serde_json` reading and writing nothing (per security-plan §Input Validation, MCP server row).
- The same row requires no print, `unwrap`, `expect` or `panic!` outside the tests, and states that requirement as a probe over the module that counts 0. After the split, "the module" means every file the split produces, so the probe has to cover all of them (per security-plan §Input Validation, MCP server row).
- The command schema has to stay a closed `'static` table, and `validate` / `validate_session` must not take a `Session` in their signature. Beyond that, the refusal order must stay fixed, and nothing a call supplies may reach an `unwrap`, `expect`, `panic!`, `unreachable!` or index. Nothing public may run a `Command` that did not come from `validate`. Splitting the verb table away from its help texts must not widen any visibility in a way that breaks this (per security-plan §Input Validation, Driver command schema row).
- The two spans have to keep exactly their fields: `command` under `escher_driver` with its eight fields of fixed words, bools and counts, and `tool_call` with `tool` and `ending` only. No `Call`, `ArgValue`, `Outcome`, `Refusal`, argument, screen text or shot data may be fielded or logged (per security-plan §Input Validation, Driver command schema row).
- Escher's sink rules for `escher_*` targets mean a moved call site must keep its target and its field names. A renamed target or field changes what the allowlist prints or redacts (per security-plan §Logging & Monitoring, Log format and backends).
- The split must add no dependency and must not move `Cargo.lock`. Builds keep `--locked` (per security-plan §Dependency Security, Pinning; per security-plan §Supply chain integrity, Lockfile verification).
- Moving tests out of `packages/blitz-dom/src/document.rs`, or out of the two driver files, must add no env read and no test-only listener. That holds even in test code, beyond the reads already registered (per security-plan §Secret Management, Environment values read).

## Patterns to follow
- Keep the help texts, `VERBS` and `NODE_FIELDS` as fixed `'static` table text after the split. Under the ratified carrier, `help`, the tool descriptions and the remedies are allowed to carry only the program's own fixed table text (per security-plan §Input Validation, MCP server row).
- Keep a single bounding choke point for MCP results (`ToolResult::bounded`) and a single reply writer. Do not create one copy per submodule (per security-plan §Input Validation, MCP server row).
- Keep `Refusal`, `Cause` and `Fault` with no `String` field, so a refusal cannot hold what a call supplied. A seam that moves cause or remedy tables keeps them as `&'static str` (per security-plan §Error Handling, Error format).
- Validation stays in front of execution: an MCP `tools/call` passes the JSON-kind check, then `validate` / `validate_session`, then `Session::run`, wherever the dispatch code ends up (per security-plan §Input Validation, Driver command schema row).

## Anti-patterns to avoid
- Do not raise a crate-private constructor of `Command`, or a validation internal, to `pub` (or `pub(crate)` across the new modules) so that a sibling file can reach it. Nothing public may run a `Command` that did not come from `validate` (per security-plan §Input Validation, Driver command schema row).
- Do not let `serde_json` serialize, add a `Debug` print, or add a log line while moving reply-writing code (per security-plan §Input Validation, MCP server row).

## Contract bindings
- security ↔ obs: the unchanged `command` and `tool_call` spans are the obs plan's span contracts as well. The checks that pin them are `stand_act_spans`, `host_log` and `mcp_log`, and the no-needle sink measurement in `CLAUDE.md`'s critical warnings depends on their fields.
- security ↔ tests: the MCP boundary is pinned by `examples/seven_guis/tests/mcp_server.rs` and the inline unit tests in `mcp.rs`, and the schema by the inline unit tests in `schema.rs`. Those tests move with the split, and their row count must not drop. If `document.rs` tests become a new blitz-tests target, `.github/scripts/test_blitz_tests_targets.py` pins it.
- security ↔ wrap reconcile: the plan's `file:line` citations into `packages/escher-driver/src/mcp.rs` (MCP server row, Driver command schema row, §Threat Model Summary, §Authentication & Authorization stdio row) and `schema.rs` (Driver session socket row) go stale when the code moves, so they have to be re-pointed at the wrap. Whether the current code already passes the no-panic and no-print probe at count 0 is research's question.

## Acceptance criteria contributions
- (security) The no-print / `unwrap` / `expect` / `panic!` probe, outside `#[cfg(test)]`, reads 0 over every file that replaces `mcp.rs`, and the same holds for `unwrap` / `expect` / `panic!` / `unreachable!` in the split `schema.rs` validation path. The MCP check `mcp_server` stays green (per security-plan §Input Validation, MCP server row).
- (security) The span field sets are unchanged: `command` has `verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`, `tool_call` has `tool` · `ending`, both under target `escher_driver`, and `stand_act_spans`, `host_log` and `mcp_log` stay green (per security-plan §Logging & Monitoring, Log format and backends).
- (security) The chunk's diff leaves `Cargo.lock` and every `Cargo.toml` dependency list unchanged, and `bash .github/scripts/ci-leg.sh audit` passes (per security-plan §Dependency Security, Pinning).
- (security) A search for `std::env|env::var|env!` over the files the chunk adds or changes finds no read beyond those already registered (per security-plan §Secret Management, Environment values read).
