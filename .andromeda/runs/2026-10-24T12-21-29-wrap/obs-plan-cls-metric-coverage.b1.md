# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the root manifest is byte-identical and dioxus-native changed only its src/lib.rs, so the Cargo.toml citations and the log features hold
metric-coverage#1.2 · yes · stale · resolve.rs gains two lines above the cited block, at 29 and 32, so the citation 77-169 reads 79-171
metric-coverage#1.3 · no · packages/debug_timer is not among the changed files and the new multi_doc printer sits outside it, so its citations hold
metric-coverage#1.4 · yes · stale · toolbar.rs gains lines at 3 and 203, so the Toggle FPS menu citation 436-439 reads 438-441
metric-coverage#1.5 · no · blitz-paint is not among the changed files, so layers.rs keeps its counters and citations
metric-coverage#1.6 · yes · add-to · examples/multi_doc.rs is a new in-process timing printer outside debug_timer that this group must now carry beside the bench
metric-coverage#1.7 · yes · add-to · examples/multi_doc.rs is a new in-process timing printer outside debug_timer that this group must now carry beside the screenshot example
metric-coverage#1.8 · no · paint_tree_bench.rs is not among the changed files, and the dropped-node skip in hit testing changes what the bench times but not what it prints
metric-coverage#1.9 · no · paint_tree_bench.rs is not among the changed files and the session names no change to node or stacking-context counts on the external page
metric-coverage#1.10 · yes · stale · wpt/runner/src/main.rs gains three lines at its top for the mimalloc allocator, so the citations 375, 624 and 694 move to 378, 627 and 697
metric-coverage#1.11 · yes · stale · wpt/runner/src/main.rs gains three lines at its top, so the citations 498-518 and 783-829 move to 501-521 and 786-832
metric-coverage#1.12 · yes · stale · wpt/runner/src/main.rs gains three lines at its top, so the citation 837-851 moves to 840-854
metric-coverage#1.13 · no · no CI or .github file changed, so the citations wpt.yml:26 and :64-67 hold
metric-coverage#2.1 · no · the apps additions, mimalloc and the Parallel builds, match none of the searched tokens and add no file under apps/
metric-coverage#2.2 · no · no added line carries a counter!, histogram! or gauge! macro, so the absence stands over the searched files
metric-coverage#2.3 · no · no added line carries a counter!, histogram! or gauge! macro, so the absence stands over the listed files
metric-coverage#2.4 · no · the inline layout moved as a whole and no added line carries a metric token, so the text-layout metrics() matches stand
metric-coverage#2.5 · no · no added line carries a metric, counter, histogram or gauge token, so the absence stands over the listed files
metric-coverage#2.6 · no · the touches name no file of that slice and no added line carries a metric token
metric-coverage#2.7 · no · no added line carries a metric, counter!, histogram, OpenTelemetry, Sentry or Prometheus token, and mimalloc matches none
