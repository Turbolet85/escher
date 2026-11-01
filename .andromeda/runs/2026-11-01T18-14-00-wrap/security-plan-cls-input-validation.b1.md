# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · examples/screenshot.rs is not reached, and the session touches no CLI example
input-validation#1.2 · no · examples/paint_bench.rs, examples/screenshot.rs and examples/multi_doc.rs are not reached
input-validation#1.3 · no · examples/screenshot.rs is not reached
input-validation#1.4 · no · examples/preact_script.rs is not reached
input-validation#1.5 · no · scripts/agent-run.sh is not reached, as the task's Untouched list says
input-validation#1.6 · no · scripts/cold-agent.sh is not reached, as the task's Untouched list says
input-validation#1.7 · yes · stale · packages are not the issue here: .github/scripts/test_ci_workflows.py is reached and its SHA-pin assertion adds lines at :110-116, so the InstallScriptTest citations at :450-484 and :504-520 moved
input-validation#1.8 · no · .github/scripts/coverage_floor.py and test_coverage_floor.py are not reached, and the session moves no coverage floor
input-validation#1.9 · no · scripts/cold-agent.sh, where the reply-check rule lives, is not reached
input-validation#1.10 · no · packages/escher-driver holds no diff, so the session socket's grammar and bounds stand
input-validation#1.11 · no · packages/escher-driver holds no diff, and the highlighted row still crosses no carrier
input-validation#1.12 · no · packages/escher-driver holds no diff, so the state directory rules stand
input-validation#1.13 · no · packages/escher-driver holds no diff, so the recording directory rules stand
input-validation#1.14 · no · packages/escher-driver holds no diff, and the recorded-check reader and tests/blitz-tests/checks are not reached
input-validation#1.15 · no · packages/escher-driver holds no diff, so the session-label rule stands
input-validation#1.16 · no · packages/escher-driver holds no diff, so the command schema stands
input-validation#1.17 · no · the escher-session CLI in packages/escher-driver and examples/seven_guis are not reached
input-validation#1.18 · no · packages/escher-driver holds no diff, so the MCP server's rules stand
input-validation#1.19 · no · apps/browser is not reached, and its nav.rs is not in the reached set
input-validation#1.20 · no · apps/browser/src/nav.rs and packages/dioxus-native/src/link_handler.rs are not reached
input-validation#1.21 · no · packages/blitz-dom/src/events/pointer.rs and url.rs are not reached, and the link href path is unchanged
input-validation#1.22 · no · packages/blitz-dom/src/url.rs is not reached, and DocumentUrl parsing is unchanged
input-validation#1.23 · no · packages/blitz-dom/src/scrolling.rs is not reached
input-validation#1.24 · yes · stale · packages/blitz-net/src/lib.rs is a merge surface, so the file: read at :159-170 is touched whole, and the fetch error arm now calls handler.error (:313) with the waker moved after it (:316)
input-validation#1.25 · yes · stale · packages/blitz-net/src/lib.rs is a merge surface, and the lines added at :304-316 move the data: URL citations at :154-157 and :396-406
