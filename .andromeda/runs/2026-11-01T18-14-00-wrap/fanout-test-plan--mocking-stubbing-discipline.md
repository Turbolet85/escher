proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 19
    section: "## 8. Mocking & Stubbing Discipline"
    change: Add this table row after the settle unit tests' row, | Net handler (blitz-net unit tests) | since 2026-11-01-upstream-sync-ui-graph a test-local `Handler` implements `NetHandler` in `blitz-net`'s `#[cfg(test)] mod tests`, beside the module's `fetch` helper returning `Result<Bytes, String>`, over three tests (`delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors`, `http_error_body_is_not_delivered_as_css`); the network itself is no stand-in in the two that bind, each a `std::net::TcpListener` on `127.0.0.1:0`, served by no binary and reached by no caller | (packages/blitz-net/src/lib.rs:471-486; packages/blitz-net/src/lib.rs:488-511; packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534) |
    sidecar: test-plan §8 gains a row for blitz-net's test-local `Handler`, the `NetHandler` stand-in of its three new unit tests, two of which bind real loopback listeners.
    rationale: The report adds `struct Handler`, `impl NetHandler for Handler` and `fetch` in blitz-net's new test module (report.md:283-289) and names its two test-only binds (report.md:28), and §8's table lists each hand-written stand-in by concern yet holds no row for this one; report.md:48 keeps rows 15-16 unchanged, so the row is an addition and edits neither.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:283
note: Outside this file and absent from the report's Counts, .andromeda/test-plan.md:30 still reads blitz-net as no test observed absent where report.md:283-289 adds its test module, and .andromeda/test-plan.md:32 names blitz-paint's inline unit tests in gradient.rs and css_box.rs alone where report.md:295-298 adds three in layers.rs.
