# Classified — test-plan · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · stale · dioxus_document.rs, a conflict surface, gained lines at 725-759 above its bridge_tests declaration cited at :818-820, so that citation moves
unit-test-strategy#2.1 · no · blitz-test-harness is not reached, its API and unit tests are unchanged, and the workspace run is green
unit-test-strategy#2.2 · no · escher-driver is byte-unchanged and not reached, so its unit tests and cited lines hold
unit-test-strategy#3.1 · no · escher-driver is byte-unchanged and not reached, so mcp_tests.rs's listing test and its helpers hold
unit-test-strategy#4.1 · no · publish-browser.yml gained 15 lines, but the CI-scripts leg reads 108 tests OK over the changed tree, so UpstreamGuardTest's guard pin holds
unit-test-strategy#5.1 · no · the manifest is not reached and all.rs keeps test = false, so BlitzTestsTargetsTest still holds and reads green in fast with the new target
unit-test-strategy#6.1 · no · scripts/cold-agent.sh and test_cold_agent.py are byte-unchanged and no reached file is in the pipe
unit-test-strategy#7.1 · no · docs/capabilities.md, README.md and the escher-driver schema they count are not reached, so the five counts hold
unit-test-strategy#8.1 · yes · stale · node/element.rs, a merge surface, gained 10 lines at 488-489 and 543-550 above its five text-input scroll tests cited at :1099-1214, so that range moves
unit-test-strategy#9.1 · no · wpt/runner is not reached, and only the wpt/WPT_COMMIT pin moved, which no listed runner test names
unit-test-strategy#10.1 · no · the session adds no #[test] and no Rust file to slice s03, its new tests sitting in tests/blitz-tests, blitz-vibey-script, dioxus-native-dom and apps/browser
unit-test-strategy#10.2 · no · the session adds no #[test] and no Rust file to slice s07, its new tests sitting in tests/blitz-tests, blitz-vibey-script, dioxus-native-dom and apps/browser
unit-test-strategy#10.3 · yes · stale · the runtime/ and dom/element/ split adds 13 Rust modules to the 32-file slice, so its count is stale, while no inline test module was added so the absence holds
unit-test-strategy#10.4 · yes · stale · the merge-surface tests/all.rs and the new target pseudo_element_flow_classification.rs sit in tests/blitz-tests, so the 61-file cfg reading is stale until re-measured
