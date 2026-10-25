# Classified — tests · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · no · it places unit-test modules in blitz-paint, dioxus-native-dom, blitz-dom and escher-driver, none reached by the task, and the chunk adds no unit test
unit-test-strategy#2.1 · yes · settle.rs's load counter (`is_noop` the inner provider's, a request in flight after a quiet pass reads `Loads`) is what the chunk's held load in flight and its load wait rest on
unit-test-strategy#2.2 · yes · the load wait over a scripted settle, the `command` span's field names and the recording and replay pins are what the chunk's act, load-wait, host-stderr, replay and cold-agent span checks rest on; no driver file is edited
unit-test-strategy#3.1 · yes · the stand-repeat cases (three rounds, stop at the first red) and the a11y leg's target words stand_keyboard, stand_contrast and stand_act_select bind the chunk's stand-repeat and re-pinned checks; the other CI invariants are not reached
unit-test-strategy#4.1 · no · the manifest pin reads Cargo.toml's one `[[test]]` table, which the task does not change
unit-test-strategy#5.1 · yes · test_cold_agent.py's 33 cases are the cold-agent contract the chunk moves (`tasks_met` over the new goals, the scrub-set and planted-marker checks) and run in the fast ci-scripts leg
unit-test-strategy#6.1 · no · the capability statement's pin reads docs/capabilities.md and the schema's counts, and the task adds no verb, cause, session error, node key or exit
unit-test-strategy#7.1 · yes · node.rs's DISABLED and ENABLED button reads, which the enabled-state Tab orders rest on, and select.rs's selectedness and typeahead rules, which the new flight-booker select and `type` on a select re-assert, bind
unit-test-strategy#8.1 · no · the WPT runner's unit tests; the task touches no WPT or CSS conformance path
unit-test-strategy#9.1 · no · slice s03 is named only by its number, no reached path identifies it, and the chunk adds no unit test
unit-test-strategy#9.2 · no · slice s07 is named only by its number, no reached path identifies it, and the chunk adds no unit test
unit-test-strategy#9.3 · no · blitz-vibey-script is not reached, and its tests are not needed
unit-test-strategy#9.4 · yes · tests/blitz-tests is reached in its stand_*.rs and fixtures files whole and in common/mod.rs by unit, so the chunk must add no in-file unit module there
