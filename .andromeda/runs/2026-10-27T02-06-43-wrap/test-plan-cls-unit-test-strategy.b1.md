# Classified — test-plan · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · no · mcp::tests still sits out of line in mcp_tests.rs, declared at mcp.rs:291-293 (mcp.rs not reached), and the eight out-of-line modules are unchanged; the change adds helpers inside mcp_tests.rs and moves no module declaration
unit-test-strategy#2.1 · no · blitz-test-harness is unedited (no harness file changed), so its settle, screenshot and contrast pins stand
unit-test-strategy#2.2 · yes · add-to · F3: the MCP listing test's row checks moved into three private #[track_caller] helpers, which the mcp_tests record beside `help` must now carry
unit-test-strategy#3.1 · no · ci.yml, ci-leg.sh and the 108 CI-scripts tests are unmoved, and the entity credits no stand-repeat membership to the sweep test
unit-test-strategy#4.1 · no · the blitz-tests manifest is unchanged (no Cargo.toml edit) and its 146 targets read unmoved, so the one-table `all` pin stands
unit-test-strategy#5.1 · no · the cold-agent pipe and scripts/cold-agent.sh are unchanged, and the twelve-verb table it serves is unmoved
unit-test-strategy#6.1 · no · no verb, refusal cause, count or capability line changed, so the statement's pins still hold
unit-test-strategy#7.1 · no · nothing under packages/blitz-dom/ changed, so the node-level tests and their citations stand
unit-test-strategy#8.1 · no · the WPT runner is not reached and no file under wpt/runner changed
unit-test-strategy#9.1 · no · the change adds no #[test] or #[cfg(test)] attribute (its helpers are #[track_caller]; mcp_tests.rs still holds 16 #[test]), so the search over slice s03 still reads none
unit-test-strategy#9.2 · no · the change adds no #[test] attribute, so the search over the other seven files of slice s07 still reads none
unit-test-strategy#9.3 · no · blitz-vibey-script is not reached (no file under it changed), so the search over its source files still reads no unit test
unit-test-strategy#9.4 · no · the sweep test's new helpers, predicate and constants carry no cfg attribute, so the #[cfg search over tests/blitz-tests still reads no in-file cfg(test) module
