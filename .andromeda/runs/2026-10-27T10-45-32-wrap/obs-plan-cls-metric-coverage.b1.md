# Classified — obs-plan · metric-coverage.b1

## Answers

metric-coverage#1.1 · no · its cited manifests (Cargo.toml:265-268; dioxus-native Cargo.toml:60-68) stay byte-identical, so the log-feature gates they hold are unchanged
metric-coverage#1.2 · no · cites blitz-dom resolve.rs, which is not among the reached files and no touch names it
metric-coverage#1.3 · no · cites debug_timer lib.rs, which is not among the reached files and no touch names it
metric-coverage#1.4 · no · cites apps/browser fps_overlay.rs and toolbar.rs, neither among the reached files, so the overlay's ring, 250 ms poll and Toggle FPS item are unchanged
metric-coverage#1.5 · no · cites blitz-paint layers.rs, which is not among the reached files and no touch names it
metric-coverage#1.6 · no · cites examples/paint_bench.rs, which is not among the reached files
metric-coverage#1.7 · no · cites examples/screenshot.rs, which is not among the reached files
metric-coverage#1.8 · no · cites examples/multi_doc.rs, which is not among the reached files
metric-coverage#1.9 · no · cites tests/blitz-tests/tests/paint_tree_bench.rs, which is not among the reached files
metric-coverage#1.10 · no · cites the same paint_tree_bench.rs external-page lines, which are not among the reached files
metric-coverage#1.11 · no · cites wpt/runner/src/main.rs, which is not among the reached files, and the merged WPT_COMMIT pin is cited by no line of it
metric-coverage#1.12 · no · cites wpt/runner/src/main.rs, which is not among the reached files
metric-coverage#1.13 · no · cites wpt/runner/src/main.rs, which is not among the reached files
metric-coverage#1.14 · no · cites .github/workflows/wpt.yml, which is not among the reached files, and the merged publish-browser.yml is a different workflow
metric-coverage#2.1 · yes · stale · apps/browser/src/main.rs (2 lines added) and tab.rs (63 added), both merge surfaces, lie inside the 86-file apps slice this search reads, and the report re-reads absence searches only for the blitz-vibey-script / blitz slice
metric-coverage#2.2 · no · its 15 s05 files are not named as reached and the row cites no reached file
metric-coverage#2.3 · no · its 17 listed s06 files are not named as reached and the row cites no reached file
metric-coverage#2.4 · no · its 8 slice files are not named as reached, the row cites no reached file, and its matches are text-layout metrics() calls and comments
metric-coverage#2.5 · no · its 16 listed files are not named as reached, the row cites no reached file, and its search matches nothing
metric-coverage#2.6 · yes · stale · the blitz-vibey-script / blitz slice grew by 13 Rust modules and 9 JS bootstrap files, so its 32-file count is stale (its absence re-read holds)
metric-coverage#2.7 · no · its 21 listed s11 files are not named as reached and the row cites no reached file
