# Classified — architecture · occupied-resources.b1

## Answers
occupied-resources#1.1 · no · no socket, port or listener is added in lint.rs or the MCP tests, and the lint findings it names are re-read unchanged (stand_lint 7 green)
occupied-resources#2.1 · no · the chunk adds no URL, host or fetch, its changes being private factoring and test helpers
occupied-resources#3.1 · no · the lint findings the live run carries are re-read unchanged (stand_lint 7 green), and no carriage is added
occupied-resources#3.2 · no · the load page's `LOAD_URL` stand-in lives in examples/seven_guis/src/stand.rs, which is not among the reached files
occupied-resources#4.1 · no · the MCP tests' `env!("CARGO_MANIFEST_DIR")` moved only in line (:633 → :650) and remains, and the cited CARGO_MANIFEST_DIR readers are unmoved, so the claim holds
occupied-resources#5.1 · no · scripts/cold-agent.sh and its `record/` directory are not among the reached files, and no kept check is added
occupied-resources#5.2 · no · the committed 0.2.0 live-run copies are not among the reached files, and the chunk's own evidence sits under escher-0.3.0/chunks/2026-10-26-codebase-health
occupied-resources#6.1 · no · recording.rs and session.rs are not among the reached files, and no recording directory, check file or write is added
occupied-resources#6.2 · no · the two committed Counter checks are byte-unchanged, so their path, mode and content hold
occupied-resources#7.1 · no · no span, field or event is added (`info_span!` count 2 unchanged, the `command` and `tool_call` span fields unchanged)
occupied-resources#8.1 · no · the `LoadNet` stand-in in examples/seven_guis/src/stand.rs is not among the reached files
occupied-resources#9.1 · no · the shared module examples/seven_guis/tests/common/mod.rs is not among the reached files
occupied-resources#9.2 · no · nothing under packages/blitz-dom changed, so `PARALLEL_TRAVERSAL_LOCK` and its callers are untouched
occupied-resources#9.3 · no · no manifest, `Cargo.lock` or allocator change, so the mimalloc binaries are untouched
occupied-resources#10.1 · no · no env var is added or changed, and the compile-time `env!` reads in the MCP tests (count 2) are not runtime reads this entry lists
occupied-resources#11.1 · no · no crate, binary, script, feature or served server name changes, and the new helpers are private items rather than registered names
occupied-resources#12.1 · no · no CI workflow, step, leg or lockfile changed, so no cache key re-keys, and CI run 40434241601 is green on 3c04042ed
