@answer n=1 entity=unit-test-strategy#1.1 v=yes mode=stale
why: the ported task_polled_during_render_can_access_document at dioxus_document.rs 846-885 and the earlier insertions moved the bridge_tests declaration cited at dioxus_document.rs 888-890
@answer n=2 entity=unit-test-strategy#2.1 v=no
why: blitz-paint layers.rs is not a file the session reaches
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: blitz-shell event.rs is not a file the session reaches
@answer n=4 entity=unit-test-strategy#4.1 v=no
why: the blitz-test-harness settle, screenshot and contrast files are not reached and the initial_build and poll signatures callers use are unmoved
@answer n=5 entity=unit-test-strategy#4.2 v=no
why: the escher-driver wire, session, error, refusal, command, execute and json files are not reached
@answer n=6 entity=unit-test-strategy#5.1 v=no
why: escher-driver schema_tests, refusal and json are not reached and the verb and cause counts are unmoved
@answer n=7 entity=unit-test-strategy#6.1 v=no
why: escher-driver mcp_tests.rs is not reached
@answer n=8 entity=unit-test-strategy#7.1 v=no
why: escher-driver schema_tests.rs and schema.rs are not reached
@answer n=9 entity=unit-test-strategy#8.1 v=no
why: the CI workflows and ci-leg.sh are unchanged, with no leg, step or selection moved
@answer n=10 entity=unit-test-strategy#9.1 v=no
why: tests/blitz-tests/Cargo.toml is unchanged and the manifest check reads only that file
@answer n=11 entity=unit-test-strategy#10.1 v=no
why: the cold-agent pipe and its test_cold_agent.py are not reached
@answer n=12 entity=unit-test-strategy#11.1 v=no
why: the capability statement files and their counts are not reached
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: blitz-dom node scrollbar, element and node.rs are not reached
@answer n=14 entity=unit-test-strategy#13.1 v=no
why: blitz-dom node/select.rs is not reached
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: blitz-dom node/select_list.rs is not reached
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: blitz-dom form.rs is not reached
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: blitz-dom mutator.rs is not reached and the mutation writer's file is byte-identical
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: blitz-dom document.rs is a different file from the reached dioxus_document.rs
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: blitz-net lib.rs is not reached
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: the wpt runner files are not reached
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: the reached files that carry tests cannot sit in this test-free slice and the new mutation_buffer.rs holds no test
@answer n=22 entity=unit-test-strategy#20.2 v=no
why: the reached files that carry tests cannot sit in the seven test-free files of slice s07 and the new mutation_buffer.rs holds no test
@answer n=23 entity=unit-test-strategy#20.3 v=no
why: the blitz-vibey-script sources are not reached
@answer n=24 entity=unit-test-strategy#20.4 v=yes mode=stale
why: the probe file in tests/blitz-tests is reached whole by its renamed and inverted pins and new helper, and the no-cfg claim over that slice is not shown to hold on it
