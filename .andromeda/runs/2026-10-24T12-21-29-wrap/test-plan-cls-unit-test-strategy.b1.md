# Classified — test-plan · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · no · the session adds no unit module, and the cited files (blitz-paint, dioxus-native-dom, escher-driver, blitz-dom document.rs) are not named as changed
unit-test-strategy#2.1 · no · blitz-test-harness is unchanged, and its settle, screenshot and contrast tests read none of the engine changes (hit testing, aspect-ratio sizing, style threading)
unit-test-strategy#2.2 · no · escher-driver is unchanged, its verb, key, refusal and test counts stand, and none of its cited unit tests reads hit testing or dropped nodes
unit-test-strategy#3.1 · no · the .github files and the CI Python tests are byte-unchanged, so test_ci_workflows.py and the 100-test count stand
unit-test-strategy#4.1 · no · tests/blitz-tests/Cargo.toml is unchanged and BlitzTestsTargetsTest stays green on its one [[test]] table, and the new line in the unbuilt all.rs is outside what it reads
unit-test-strategy#5.1 · no · scripts/cold-agent.sh, its contract tests and the twelve-verb fake are unchanged, so the 33-test pipe and the 100-test count stand
unit-test-strategy#6.1 · yes · stale · the lines the session added in node.rs and element.rs sit above the cited test ranges node.rs 2060-2139 and element.rs 1072-1187 and move them down
unit-test-strategy#7.1 · yes · stale · the mimalloc global allocator at lines 1-3 of wpt/runner/src/main.rs moves the cited main.rs test lines 854-914 down by three
unit-test-strategy#8.1 · no · the session adds no unit test, and its one new test is a blitz-tests integration file
unit-test-strategy#8.2 · no · the session adds no test to the other seven files of slice s07, and its one new test sits in tests/blitz-tests
unit-test-strategy#8.3 · no · blitz-vibey-script is not among the session's changes and gains no test
unit-test-strategy#8.4 · yes · stale · the new blitz-tests file hit_test_dropped_nodes.rs moves the blitz-tests file count the 61-file search runs over by one
