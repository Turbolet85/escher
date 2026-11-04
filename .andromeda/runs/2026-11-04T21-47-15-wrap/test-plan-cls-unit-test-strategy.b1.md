@answer n=1 entity=unit-test-strategy#1.1 v=no
why: the session adds an integration target under tests/blitz-tests and touches no packages/ file, so the unit-test locations and out-of-line modules it names still hold
@answer n=2 entity=unit-test-strategy#2.1 v=no
why: layers.rs in blitz-paint is outside the session's reached files and its three unit tests are unchanged
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: blitz-shell's event.rs is outside the session's reached files and its one unit test is unchanged
@answer n=4 entity=unit-test-strategy#4.1 v=no
why: the session calls Harness::from_vdom as an existing API and changes no blitz-test-harness file, so settle, screenshot and contrast tests still hold
@answer n=5 entity=unit-test-strategy#4.2 v=no
why: escher-driver is untouched by the session, which changes no driver, verb, wire, schema or telemetry code
@answer n=6 entity=unit-test-strategy#5.1 v=no
why: escher-driver's help texts, VERBS count and describe count row are untouched by the session, which changes no driver code
@answer n=7 entity=unit-test-strategy#6.1 v=no
why: mcp_tests.rs is an escher-driver file the session does not reach, and its listing test is unchanged
@answer n=8 entity=unit-test-strategy#7.1 v=no
why: schema_tests.rs and schema.rs are escher-driver files the session does not reach, so type's help is unchanged
@answer n=9 entity=unit-test-strategy#8.1 v=no
why: the workflows and test_ci_workflows.py are unchanged, the audit row still asserts the audit leg's advisories and licenses, and the session adds no CI-scripts test
@answer n=10 entity=unit-test-strategy#9.1 v=no
why: tests/blitz-tests/Cargo.toml is unchanged, so autotests stays on and the single all [[test]] table still holds, the new target being found by autodiscovery and declared nowhere
@answer n=11 entity=unit-test-strategy#10.1 v=no
why: the cold-agent pipe and its test_cold_agent.py are outside the session's reached files and unchanged
@answer n=12 entity=unit-test-strategy#11.1 v=no
why: test_capability_statement.py and docs/capabilities.md are outside the session's reached files, and the session changes no verb, cause, session error or node key
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: blitz-dom node tests are outside the session's reached files, and the session changes nothing under packages/
@answer n=14 entity=unit-test-strategy#13.1 v=no
why: blitz-dom's node/select.rs is outside the session's reached files and its tests are unchanged
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: blitz-dom's node/select_list.rs is outside the session's reached files and its six inline tests are unchanged
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: blitz-dom's form.rs is outside the session's reached files and its two select tests are unchanged
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: blitz-dom's mutator.rs is outside the session's reached files and its restyle tests are unchanged
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: blitz-dom's document.rs picture and inline-flag tests are outside the session's reached files and unchanged
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: blitz-net's inline tests are outside the session's reached files and the session adds no network code
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: the wpt runner's tests are outside the session's reached files, and wpt is not touched
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: slice s03 is a set of source files the session does not reach, so its search count and absence claim stand
@answer n=22 entity=unit-test-strategy#20.2 v=no
why: slice s07 is an 8-file set the session does not reach, so its #[test] search stands
@answer n=23 entity=unit-test-strategy#20.3 v=no
why: blitz-vibey-script source files are outside the session's reached files and unchanged
@answer n=24 entity=unit-test-strategy#20.4 v=yes mode=stale
why: the session adds tests/blitz-tests/tests/dioxus_runtime_probe.rs to the tree this 61-file in-file cfg search covers, so the searched count and the absence reading are not re-read on the new file
