# security extract

## Relevance
partial — the chunk only adds tests, with no new surface. It does touch the MCP request framing, a boundary the plan bounds (`read_request` in `packages/escher-driver/src/mcp/framing.rs`). It must also add no dependency, env read or listener. The `create_text_node` and Timer sites lie outside any security boundary.

## Constraints
- The MCP framing bounds must stay as they are (per security-plan §Input Validation, MCP server row): a request line of at most 65,536 bytes (`MAX_MCP_REQUEST_BYTES`); a longer line read to its terminator, dropped and answered `-32700`; an empty line skipped; non-UTF-8 or non-JSON answered `-32700`. A fast-failing test for the `chunk.len() + 1` mutants must pin this behaviour as it stands, without moving any bound. Whether the current code already consumes the terminator correctly on every path (an empty line, an oversize line, a line at the bound) is research's question.
- If a mutant can be caught only by changing shipped code (P4's fork), the framing module must still hold no print, `unwrap`, `expect` or `panic!` outside the tests (per security-plan §Input Validation, MCP server row: "a probe over the module, count 0").
- An MCP protocol error must stay a fixed code and a fixed message with no `data`. The admitted request `id` must stay the one client-sent value written back (per security-plan §Error Handling).
- The chunk must add no dependency (per security-plan §Dependency Security). `Cargo.lock` stays committed, builds pass `--locked`, and `deny.toml` stays unchanged. `cargo-mutants` is a host instrument and never a workspace or dev dependency.
- The chunk must add no env read beyond the enumerated list, in tests included (per security-plan §Secret Management, Environment values read). That covers a test watchdog, a timeout knob or a scratch-dir lookup. A test-only read is registered in that list before it lands, as `TMPDIR` and `CARGO_TARGET_TMPDIR` are.
- The chunk must add no listener, socket or port (per security-plan §API Security). The session socket stays the one served listener. A test-only bind would need registration as test-only, as the `client::imp::tests` listener was.
- The chunk must add no span field and no log site that holds what a call supplies or what a screen reads (per security-plan §Logging & Monitoring). The `command` and `tool_call` spans keep their fields as they are.

## Patterns to follow
- Drive `serve_mcp` in process over an in-memory reader and writer it is handed, not over stdio or a socket. The framing's bounds and fixed errors are the contract under test (per security-plan §Input Validation, MCP server row; §Threat Model Summary, local stdio vector).
- Keep any watchdog or time bound inside the test. The harness settle reads no clock and bounds its pass loop at 64. Time on a held instance moves only by `advance`, 1 to 60000 ms a call (per security-plan §API Security, Settle and loads in flight row). The Timer mutants' loop must be bounded on the test's side, not by adding a clock or a wait to shipped code.
- Follow the registration-first pattern for anything test-only that resembles a surface: the `eab-{pid}` listener and `TMPDIR` read in `client::imp::tests` were registered before landing (per security-plan §Secret Management, Environment values read).

## Anti-patterns to avoid
- Don't widen or relax a framing bound or a protocol error's form to make a mutant fail. For example, don't raise `MAX_MCP_REQUEST_BYTES`, don't add `data` to an error, and don't echo more of the request (per security-plan §Input Validation, MCP server row; §Error Handling).
- Don't add a dependency, even a dev-only one, to bound a hang (e.g. a timeout crate) (per security-plan §Dependency Security).
- Don't add an unregistered env read or listener in a test helper (per security-plan §Secret Management, Environment values read; §API Security).

## Contract bindings
- security ↔ tests: the MCP framing bounds (65,536-byte line, `-32700` on oversize or non-JSON, empty line skipped) are pinned by escher-driver's `mcp_tests.rs` and the `mcp_server` integration check. A new fast-failing framing test joins that pin, and the test-plan's per-file counts move with it (per security-plan §Input Validation, MCP server row).
- security ↔ CI: the `audit` leg (`bash .github/scripts/ci-leg.sh audit`, advisories and licences over the whole workspace) runs on every push. A chunk that adds no dependency must leave it reading `advisories ok` and `licenses ok` (per security-plan §Dependency Security, CI integration).

## Acceptance criteria contributions
- The MCP framing's behaviour is unchanged, checked two ways. The existing `mcp_tests.rs` and `mcp_server` checks pass unmodified. The new framing test passes on unmutated code and asserts the bound, the `-32700` answer to an oversize line and the skipped empty line as they read today (per security-plan §Input Validation, MCP server row).
- `bash .github/scripts/ci-leg.sh audit` reads `advisories ok` and `licenses ok`, and the chunk's diff touches neither `Cargo.lock`, `deny.toml` nor any `Cargo.toml` dependency table (per security-plan §Dependency Security).
- A census over the chunk's added lines finds 0 new env reads (`std::env|env!\(|getenv`) outside the registered list, and 0 new binds (`UnixListener|TcpListener|UdpSocket|bind\(`) (per security-plan §Secret Management, Environment values read; §API Security).
- If P4's fork changes shipped `mcp/framing.rs`, a probe over that module's non-test code counts 0 `println!|eprintln!|unwrap\(|expect\(|panic!` (per security-plan §Input Validation, MCP server row).
