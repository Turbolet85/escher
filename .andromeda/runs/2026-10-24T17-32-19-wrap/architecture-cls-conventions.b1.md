# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · no · the fmt and clippy legs at .github/scripts/ci-leg.sh:20-25 and the cite at :37 sit above the five lines added at 54-58, so the claim still holds
conventions#2.1 · no · the nine changed files are the script and test sources and none is a manifest, so the Cargo.toml claims stand
conventions#3.1 · no · the report describes no module header, public item or doc change in the nine changed files, and the cited doc sites sit outside them
conventions#4.1 · no · the report adds no TODO or FIXME marker and no engine source changed, so the cited sites stand
conventions#5.1 · no · the report adds no library item, export or visibility change, and the moved fixture items are test-only helpers
conventions#6.1 · no · the report adds no feature, cfg gate or tracing site, escher-telemetry and escher-driver are untouched, and no span field is added
conventions#7.1 · no · the report describes no unsafe block, borrow workaround or let-chain in the nine changed files, and the cited sites sit outside them
conventions#8.1 · no · the 48 stand files, the target count and the fifteen stand_act_* checks are unmoved, stand_requirement_sweep and stand_act_spans keep their Session use, and each changed check extends its one behavior
conventions#9.1 · no · the report moves no unit-test module, escher-driver is untouched, and the nine changed files are integration tests and a script
conventions#9.2 · yes · add-to · many_items and its ITEMS constant join the shared fixture pages, moved from stand_snapshot_parts and now read by it and stand_act_spans, and the cited line ranges may have shifted
conventions#10.1 · no · the seven_guis stand source and the escher-session binary are untouched, and host_log.rs is a test, which the claim excludes
conventions#11.1 · no · the claim covers the HTML files under examples/assets, and none of the nine changed files is one
