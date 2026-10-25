# Classified — test-plan · unit-test-strategy.b1

## Answers

unit-test-strategy#1.1 · no · the Rust unit-test module locations it lists sit in source files the change does not reach, and its new files are a document and a Python CI-script pin, not a module inside a Rust source
unit-test-strategy#2.1 · no · the blitz-test-harness unit tests it names sit in files the change does not reach, and no harness behaviour changed
unit-test-strategy#2.2 · no · the escher-driver unit tests it names sit in Rust files the change does not reach, and the schema, wire and refusal they pin are unchanged, the change adding no Rust symbol, verb, cause, session error, exit or carrier
unit-test-strategy#3.1 · yes · add-to · the CI-scripts leg's per-file list gains test_capability_statement.py, CapabilityStatementTest with 4 tests (F1), and this record gains the pin's by-path reads, no writes or env reads and its evidence/pin-controls.md controls (F2); the leg's current reading moves from 100 tests over seven files to 104 over eight, the dated 2026-10-15 reading standing as written
unit-test-strategy#4.1 · no · test_blitz_tests_targets.py and the tests/blitz-tests/Cargo.toml it reads are not reached by the change, so its one test's claim stands
unit-test-strategy#5.1 · no · test_cold_agent.py and scripts/cold-agent.sh are unchanged and not reached, so its 33 tests, its fake's twelve tools and its isolation rows stand
unit-test-strategy#6.1 · no · the blitz-dom node tests it names sit in files the change does not reach, and no engine behaviour changed
unit-test-strategy#7.1 · no · the wpt/runner unit tests it names sit in files the change does not reach
unit-test-strategy#8.1 · no · the absence holds, since the change reaches no Rust file and adds no `#[test]` or `#[cfg(test` attribute, its new files being a document and a Python script
unit-test-strategy#8.2 · no · the absence holds, since the change reaches no Rust file and adds no `#[test]` to the other seven files of slice s07
unit-test-strategy#8.3 · no · the absence holds, since the change reaches no blitz-vibey-script file and adds no `#[cfg(test)]` or `mod tests`
unit-test-strategy#8.4 · no · the absence holds, since the session added no blitz-tests file or test and the workspace Rust tally did not move (206 result lines, 1105 passed, 0 failed, 12 ignored)
