proposals:
  - detector: D-new-detail
    severity: warning
    fact: F3
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 88
    section: '## 1. Test Scope Summary'
    change: '- **blitz-net:** 3 inline unit tests in its `#[cfg(test)] mod tests`, `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, over two test-only `TcpListener::bind` calls on `127.0.0.1:0`, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:513-520; packages/blitz-net/src/lib.rs:522-530; packages/blitz-net/src/lib.rs:532-545) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'test-scope-summary gains a blitz-net record: its 3 inline unit tests over two test-only loopback binds.'
    rationale: 'report.md:28 states the two test-only binds at blitz-net/src/lib.rs:526 and :534 in the crate test module, served by no binary and reached by no caller; report.md:103 counts unit blitz-net 3; report.md:283-289 lists the module and the three tests. The file holds no blitz-net record, so this is new detail.'
    basis: packages/blitz-net/src/lib.rs:465-546
note: the file held no blitz-net record to extend; the body's standing blitz-net absence line, now false, is a standing entity left to the strong readers.
