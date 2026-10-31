proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 11
    section: "## 8. Mocking & Stubbing Discipline"
    change: add after line 11 the table row | Shell (event loop) | `CountedWakes`, a test-local `EventLoopProxyProvider` in `blitz-shell`'s first and one unit test since 2026-10-31-engine-debts, stands in for the event loop's proxy and counts its wakes — `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` | (packages/blitz-shell/src/event.rs:138-142; packages/blitz-shell/src/event.rs:144-171) |
    sidecar: 2026-10-31-engine-debts — §8 gains the `CountedWakes` row, the test-local `EventLoopProxyProvider` stand-in of `blitz-shell`'s first unit test.
    rationale: the report's Changes add `blitz-shell/src/event.rs`'s first `#[cfg(test)]` module (Crates / modules; unit tests blitz-shell 0 → 1) and New text, by line lists `impl EventLoopProxyProvider for CountedWakes` in it, a hand-written stand-in this file's table of stand-ins does not yet hold.
    basis: packages/blitz-shell/src/event.rs:138-142
