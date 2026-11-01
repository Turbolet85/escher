# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · the CLI URL row cites examples/screenshot.rs and examples/paint_bench.rs, neither reached by the session
input-validation#1.2 · no · the numeric CLI argument row cites examples/paint_bench.rs, examples/screenshot.rs and examples/multi_doc.rs, none reached
input-validation#1.3 · no · the output filename row cites examples/screenshot.rs:175-183, not reached
input-validation#1.4 · no · the preact_script path row cites examples/preact_script.rs, not reached
input-validation#1.5 · no · the agent-run.sh row cites scripts/agent-run.sh, which the report names as not reached
input-validation#1.6 · no · the cold-agent.sh row cites scripts/cold-agent.sh, which the report names as not reached
input-validation#1.7 · yes · stale · the cited test_ci_workflows.py:450-484 and :504-520 sit past the lines added at :110-116 in a reached file, so they moved
input-validation#1.8 · no · the coverage_floor.py row is named not reached, and its cites in coverage_floor.py and test_coverage_floor.py are unchanged
input-validation#1.9 · no · the cold-agent check row cites scripts/cold-agent.sh, not reached
input-validation#1.10 · no · the driver session socket row cites escher-driver files only, which hold no diff
input-validation#1.11 · no · the highlighted-row claim lives in escher-driver's carrier and lint, no reached file
input-validation#1.12 · no · the state directory row cites escher-driver files only, which hold no diff
input-validation#1.13 · no · the recording directory row cites escher-driver files only, which hold no diff
input-validation#1.14 · no · the Check::from_bytes row cites escher-driver files only, which hold no diff, and tests/blitz-tests/checks/ is not reached
input-validation#1.15 · no · the session label row cites escher-driver files only, which hold no diff
input-validation#1.16 · no · the command schema row cites escher-driver files only, which hold no diff
input-validation#1.17 · no · the escher-session argv row cites escher-driver and examples/seven_guis files, none reached
input-validation#1.18 · no · the MCP server row cites escher-driver files only, which hold no diff
input-validation#1.19 · no · the browser urlbar row cites apps/browser/src/nav.rs, not reached
input-validation#1.20 · no · the navigation row cites apps/browser/src/nav.rs and packages/dioxus-native/src/link_handler.rs, neither reached
input-validation#1.21 · no · the Link href row cites blitz-dom events/pointer.rs and url.rs, neither reached
input-validation#1.22 · no · the Document URL row cites packages/blitz-dom/src/url.rs, not reached
input-validation#1.23 · no · the URL fragment row cites packages/blitz-dom/src/scrolling.rs, not reached
input-validation#1.24 · yes · stale · packages/blitz-net/src/lib.rs is a merge surface reached whole, and its error arm and waker move at :313-316, so the file-URL read cited at :159-170 is touched
input-validation#1.25 · yes · stale · packages/blitz-net/src/lib.rs is a merge surface reached whole, and the cited data: URL lines at :396-406 lie past the error arm's added lines at :313-316
