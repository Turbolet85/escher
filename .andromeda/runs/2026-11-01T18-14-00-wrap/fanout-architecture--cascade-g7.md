proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 225
    section: "## Standard Contracts"
    change: "After line 225, a new line of the visual lint's entity, one leading space: ` Since 2026-11-01-upstream-sync-ui-graph `Harness::text_contrast` reads `None` past an element on the walk with a `mix-blend-mode` other than `normal`, through `paints_more_than_colour` (packages/blitz-test-harness/src/contrast.rs:154-156), so `Session::lint`'s `contrast` reads such a node as not measured, the same reading with no second formula, and no stand row moves.`"
    sidecar: "standard-contracts: the visual lint's `contrast` reading gains `text_contrast`'s `None` past a blend, no second formula, no stand row moved (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:24 states `paints_more_than_colour` at `contrast.rs:154-156` and that `Session::lint`'s `contrast` reads the same with no second formula and no stand row moving; line 225 states the lint's `None` as not measured and names no blend; line 225 stands at 1891 chars, so this is a new line, not an in-place clause."
    basis: packages/blitz-test-harness/src/contrast.rs:154-156
    fact: F19
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 227
    section: "## Standard Contracts"
    change: "In line 227, `until the load is answered or `LOAD_WAIT` has passed` becomes `until the load is answered, a failed load answering it, or `LOAD_WAIT` has passed`; the rest of the line stands."
    sidecar: "standard-contracts: the load wait's end now counts a failed load as answered (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:49 marks `standard-contracts.md:227` changed, a failed load ends the wait; report.md:25 states `settle_waiting` is unedited and returns where it would otherwise have run to `LOAD_WAIT`, and that the driver contract owes that sentence; the clause keeps line 227 under 2 000 chars."
    basis: packages/blitz-test-harness/src/settle.rs:121-125
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 227
    section: "## Standard Contracts"
    change: "After line 227, a new line of the load wait's entity, one leading space: ` Since 2026-11-01-upstream-sync-ui-graph blitz-test-harness's `CountedHandler` forwards `NetHandler::error` (packages/blitz-test-harness/src/settle.rs:121-125), so a failed load reaches the document on a harness or driver session and ends the wait, `settle_waiting` unedited returning where it would otherwise have run to `LOAD_WAIT`, rather than reading `NotSettled(Loads)` for good.`"
    sidecar: "standard-contracts: the load wait records `CountedHandler`'s forward of `NetHandler::error`, through which a failed load ends the wait on a harness or driver session (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:25 states the forward at `settle.rs:121-125`, that without it a failed stylesheet read `NotSettled(Loads)` for good on a harness document and on the driver, and that a failed load now ends the wait; line 227 stands at 1902 chars, so the fact takes its own line."
    basis: packages/blitz-test-harness/src/settle.rs:121-125
    fact: F15
note: "F15 is placed here beside the load wait, so a harness-contract group proposing it too holds the duplicate; standard-contracts#20.1 stands at 56 KB against the category's 8 KB entity shape, a standing fault and not this session's."
