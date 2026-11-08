@answer n=1 entity=unit-test-strategy#1.1 v=yes mode=stale
why: the session's dispatch test and cfg(test) shim in dioxus_document.rs and the graph and clickable rows in mcp.rs and schema.rs move the cited lines of reached files
@answer n=2 entity=unit-test-strategy#2.1 v=no
why: blitz-paint is untouched and its layers.rs tests are not reached by the session
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: blitz-shell is untouched and its event.rs test is not reached by the session
@answer n=4 entity=unit-test-strategy#4.1 v=no
why: blitz-test-harness is untouched, so its settle, screenshot and contrast tests and their line citations hold
@answer n=5 entity=unit-test-strategy#4.2 v=yes mode=stale
why: the schema's verb table moves from twelve to thirteen with graph after lint, the instance verbs from eight to nine, the node keys from thirteen to fourteen with clickable, and the field kinds from ten to twelve
@answer n=6 entity=unit-test-strategy#5.1 v=yes mode=stale
why: VERBS moves from 12 to 13 and the describe count row from [12, 13, 12, 3, 8, 4, 15] to [13, 14, 12, 3, 8, 4, 15]
@answer n=7 entity=unit-test-strategy#6.1 v=yes mode=stale
why: the MCP listing now holds thirteen tools with graph after lint, so the twelve-verb listing test's name and its row walk move in reached mcp_tests.rs
@answer n=8 entity=unit-test-strategy#7.1 v=yes mode=stale
why: schema.rs and schema_tests.rs are reached whole and the graph verb row can move the cited lines 247-249 and 100-102
@answer n=9 entity=unit-test-strategy#8.1 v=no
why: the CI workflow, ci-leg.sh and test_ci_workflows.py are not reached, the leg's target words are unchanged, and the Python counts are dated readings
@answer n=10 entity=unit-test-strategy#9.1 v=no
why: the blitz-tests Cargo.toml this test reads is not reached and its one [[test]] table and autotests setting are unchanged
@answer n=11 entity=unit-test-strategy#10.1 v=yes mode=stale
why: the FAKE server speaks the twelve tools and must now answer graph, and the allowed tool set now counts thirteen
@answer n=12 entity=unit-test-strategy#11.1 v=no
why: the pin's test code is unchanged and docs/capabilities.md and README.md still hold every verb and the counts in words, all green
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: blitz-dom is byte-unchanged, so its node tests for scrollbar, text-input scroll and disabled state are not reached
@answer n=14 entity=unit-test-strategy#13.1 v=no
why: blitz-dom is byte-unchanged, so its node select step, span and group tests are not reached
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: blitz-dom is byte-unchanged, so its select_list.rs tests are not reached
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: blitz-dom is byte-unchanged, so the form.rs select submission tests are not reached
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: blitz-dom is byte-unchanged, so mutator.rs's restyle tests are not reached
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: blitz-dom is byte-unchanged, so the picture and inline-flag tests in document.rs are not reached
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: blitz-net is untouched and its three inline tests are not reached
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: the WPT runner is unreached and its test files are outside the change
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: the session adds no #[test] or cfg(test) item to the untouched slice s03
@answer n=22 entity=unit-test-strategy#20.2 v=no
why: the session adds no test to the other seven files of slice s07, which it does not reach
@answer n=23 entity=unit-test-strategy#20.3 v=no
why: blitz-vibey-script is unreached and its source files hold no test module, which the session does not add
@answer n=24 entity=unit-test-strategy#20.4 v=yes mode=stale
why: the new stand_graph.rs under tests/blitz-tests/tests joins the 61 files searched for in-file cfg modules, so the searched count moves
