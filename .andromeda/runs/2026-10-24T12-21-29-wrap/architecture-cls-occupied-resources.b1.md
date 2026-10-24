# Classified — architecture · occupied-resources.b1

## Answers
occupied-resources#1.1 · no · the take adds no listener, port or socket, and the session socket and the mcp stdio role stay unchanged, escher-driver being byte-unchanged
occupied-resources#2.1 · yes · stale · wpt/runner/src/main.rs gains three lines for the mimalloc global allocator, so its dummy.local citation at main.rs:578-579 moves to 581-582, and no outbound host is added
occupied-resources#3.1 · yes · stale · wpt/runner/src/main.rs gains three lines for the mimalloc global allocator, so its WPT_DIR and output citations at main.rs:469 and :491-496 move to :472 and :494-499, and no filesystem area is added
occupied-resources#4.1 · no · the take changes no recording path, since escher-driver is byte-unchanged and the session adds no file write
occupied-resources#4.2 · no · the two committed Counter checks and their stand output are unmoved, since the stand stays Sequential, its tallies hold, and the check files and record_checks.sh are untouched
occupied-resources#5.1 · yes · add-to · the new process-wide static PARALLEL_TRAVERSAL_LOCK (a Mutex taken around every Parallel traversal) and mimalloc as the global allocator of apps/browser and the WPT runner must be carried, and the config.rs:21-24 and stylo.rs:149-152 citations move
occupied-resources#6.1 · no · the seven_guis test module and its stderr needle reads are unchanged, since the stand, the driver and the sink allowlist do not move
occupied-resources#7.1 · yes · add-to · the three new env reads by examples/multi_doc.rs, MULTI_DOC_N, MULTI_DOC_SEQ and MULTI_DOC_FLAT, must be registered beside RUST_LOG, WPT_DIR, PAINT_TREE_BENCH_HTML and TMPDIR, and the WPT_DIR citation at main.rs:469-475 moves to 472-478
occupied-resources#8.1 · no · the take adds no workspace crate or binary name, since root Cargo.toml is byte-identical and the new multi_doc example and hit_test_dropped_nodes target are neither crates nor binaries
occupied-resources#9.1 · no · the .github workflows and scripts are unchanged, so the ci.yml citations and cache rules stand, and the Cargo.lock change is one more instance of the re-key mechanism the entity already states
