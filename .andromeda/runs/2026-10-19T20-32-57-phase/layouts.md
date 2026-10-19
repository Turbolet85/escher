# layouts extract

## Relevance
partial — the chunk moves test code behind the `escher-session` command line (cli surface) and its host/`mcp` log checks; it creates, moves or restyles no screen, region, component or focus stop, so the plan binds only as an unchanged-surface guard.

## Constraints
- The `escher-session` command line's four endings must read the same after the split: accepted (one uncoloured JSON line on stdout, exit 0), refused (refusal JSON on stdout, exit 1), usage (one fixed line on stderr, stdout empty, exit 2), session error (error object with its fixed `remedy` on stdout, fixed message on stderr, exit 3). The tests moving out of `cli.rs` are what pin these, so they move with their assertions unchanged (per layout-templates §Surface: cli → Primary screens).
- No command ending may write back anything the caller typed, the `--record` path included. Assertions guarding that must survive the move intact (per layout-templates §Surface: cli → Primary screens).
- The `mcp` role writes protocol lines only on stdout, uncoloured with no ESC byte, and its end writes nothing on stdout. The host role's stderr log lines carry `service.name=seven_guis`. The shared log-check fragment that `host_log.rs` and `mcp_log.rs` will call must keep asserting each of these for its own role, without weakening either role's check (per layout-templates §Surface: cli → Primary screens).
- The `escher-session` host role and its `mcp` role boot one lean task only through `seven_guis::stand`, adding no markup, id, class, style, wrapper or order, and a held instance reads as a fresh boot. A shared test fragment must not introduce a second boot path (per layout-templates §Surface: desktop-native → Primary screens).
- §Surface: cli cites `packages/escher-driver/src/cli.rs` line ranges, all at or before 474: 40-42, 46-101, 137-203, 160-165, 178-183, 276-291, 302-336, 339-366, 454-474. The test module begins at 476, so the split should leave lines 1-475 in place for those citations to keep resolving. Whether the `#[path]` seam shifts any cited line is research's question (per layout-templates §Surface: cli → Primary screens).

## Patterns to follow
- A private sibling-module split that leaves the parent's surface untouched: the `mcp` role's listing lives in the private child `packages/escher-driver/src/mcp/listing.rs` while the role's behaviour reads unchanged (per layout-templates §Surface: cli → Primary screens).
- The host role and the `mcp` role share one boot that `main` hands to the command line. Share the log-check fragment the same way: state it once and have both roles' checks call it (per layout-templates §Surface: cli → Primary screens).

## Anti-patterns to avoid
- Do not change, in a test-only chunk, any usage line, ending text, exit status, verb order (`help` last) or stdout/stderr split of `escher-session`, including through a test edited "to match" (per layout-templates §Surface: cli → Primary screens).
- Do not add any markup, id, class, style, wrapper or order on the stand to make a moved or merged test pass (per layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- §Surface: cli → Primary screens defers the session contract `escher-session` serves to test-plan §3 → Session lifecycle. The moved `cli.rs` and `recording.rs` tests and the shared host/`mcp` log fragment sit on that contract, so the tests extractor owns their naming and count invariants.
- §Surface: cli → Primary screens cites `cli.rs` line ranges, which binds to the wrap's citation cascade (drift-base). If a cited range moves, the plan must be re-pointed.

## Acceptance criteria contributions
- (layouts) After the split, each `cli.rs:` line range cited in §Surface: cli still holds the code it cited: the usage, `help`, `mcp` role and `--record` readers. Check this by reading those ranges against the pre-split file (per layout-templates §Surface: cli → Primary screens).
- (layouts) `escher-session` still answers the same four endings with the same exit statuses (0/1/2/3), stdout/stderr split and usage line. The moved tests pass under their old names with unchanged assertions (per layout-templates §Surface: cli → Primary screens).
- (layouts) The shared log-check fragment in `examples/seven_guis/tests/common/mod.rs`, called by both `host_log.rs` and `mcp_log.rs`, still asserts the host role's `service.name=seven_guis` stderr lines and the `mcp` role's protocol-only, ESC-free stdout (per layout-templates §Surface: cli → Primary screens).
