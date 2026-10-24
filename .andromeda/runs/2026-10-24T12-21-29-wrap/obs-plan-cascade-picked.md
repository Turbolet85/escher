# obs-plan · picked accounting

## Picked
telemetry-strategy#3.2 · not touched · the new example `multi_doc.rs` adds no tracing, logging or metrics framework, only a `println!`, so the examples-slice absence stands
telemetry-strategy#3.3 · not touched · the new target `hit_test_dropped_nodes` reads 0 env · 0 print · 0 subscriber, so the test-crate absence stands
span-trace-coverage#1.1 · not touched · `multi_doc.rs` opens no span and instruments nothing
span-trace-coverage#1.5 · not touched · the layout module's split into `inline/{build,layout,mod}.rs` adds no span (the ten commits' census reads only one commented `dbg!`), and the absence is dated to its reading's slice
span-trace-coverage#1.10 · not touched · `hit_test_dropped_nodes` adds no tracing, span or subscriber
metric-coverage#1.2 · not touched · the resolve phases are unchanged, since `resolve.rs` only changes its import (+2 −2), and the digits belong to the citation sweep
metric-coverage#1.4 · not touched · the FPS overlay is unchanged, and `toolbar.rs` gains only an import and the `Parallel` threading line
metric-coverage#1.6 · not touched · the `paint_bench` row is unchanged, and the new example's timing row lands after the `screenshot` row
metric-coverage#1.7 · proposed · `multi_doc.rs` is a new root example that times resolves with `Instant::now()` and prints them, a row this table lacks
metric-coverage#1.10 · not touched · the per-test duration is unchanged, `main.rs` gains only the mimalloc allocator lines 1-3, and the citation sweep owns the digits
metric-coverage#1.11 · not touched · the per-run figures are unchanged, and the citation sweep owns the `main.rs` digits
metric-coverage#1.12 · not touched · the report timings are unchanged, and the citation sweep owns the `main.rs` digits
log-coverage#3.1 · not touched · the browser's log sites are unchanged, because `document_loader.rs` and `toolbar.rs` gain only an import and the `Parallel` threading line
log-coverage#4.1 · proposed · the new example's one `println!` (multi_doc.rs:111-115) joins the examples' `println!` output citations
log-coverage#6.1 · not touched · the take adds and removes no blitz-dom log site, and the citation sweep owns `mutator.rs`'s re-pointed digits
log-coverage#6.3 · not touched · the take changes no layout log site, and the citation sweep owns the re-points into `construct.rs` and `layout/mod.rs`
log-coverage#6.4 · not touched · the take changes no node log or print site, and the citation sweep owns the re-points into `element.rs` and `node.rs`
log-coverage#6.11 · not touched · the take adds no WPT runner log call (its `main.rs` gains the allocator and `attr_test.rs` gains reads), and the citation sweep owns the digits
log-coverage#7.2 · not touched · `multi_doc.rs` adds no structured logging, so the examples-slice absence stands
error-capture-reporting#1.2 · not touched · `multi_doc.rs`'s `panic!` on an unknown mode is no error-capture service
pii-scrubbing-compliance#5.4 · not touched · the take adds no log site and no redaction to the layout slice
