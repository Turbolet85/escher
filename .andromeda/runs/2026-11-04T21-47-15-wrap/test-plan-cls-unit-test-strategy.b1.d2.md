@answer n=1 entity=unit-test-strategy#1.1 v=no
why: the session changed nothing under packages/ and this entity states only where packages/ unit tests sit
@answer n=2 entity=unit-test-strategy#2.1 v=no
why: layers.rs under packages/blitz-paint is outside the session's reach and its three tests are unchanged
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: the blitz-shell event.rs unit test under packages/ is untouched
@answer n=4 entity=unit-test-strategy#4.1 v=no
why: the blitz-test-harness API, settle, screenshot and contrast are unchanged and the new target only calls Harness::from_vdom and existing readers
@answer n=5 entity=unit-test-strategy#4.2 v=no
why: the escher-driver wire, session, error, refusal, schema, command, execute and json tests are not reached as no driver file or verb changed
@answer n=6 entity=unit-test-strategy#5.1 v=no
why: the escher-driver list help and covered tests are not reached as no driver file or verb changed
@answer n=7 entity=unit-test-strategy#6.1 v=no
why: the escher-driver MCP listing test is not reached as no driver file or verb changed
@answer n=8 entity=unit-test-strategy#7.1 v=no
why: the escher-driver schema test for type's help is not reached as the schema is unchanged
@answer n=9 entity=unit-test-strategy#8.1 v=no
why: no CI workflow, ci-leg.sh or CI script changed and the Cargo.lock and deny.toml moves are not what these tests read
@answer n=10 entity=unit-test-strategy#9.1 v=no
why: tests/blitz-tests/Cargo.toml did not move and the test still holds, the manifest keeping autotests unset and exactly one [[test]] table
@answer n=11 entity=unit-test-strategy#10.1 v=no
why: the cold-agent pipe, its script and its tests are stated as not touched
@answer n=12 entity=unit-test-strategy#11.1 v=no
why: the capability statement tests read docs, schema and README and the session changed none of them
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: the blitz-dom node tests under packages/ are untouched
@answer n=14 entity=unit-test-strategy#13.1 v=no
why: the blitz-dom node select step and span tests under packages/ are untouched
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: the blitz-dom select_list tests under packages/ are untouched
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: the blitz-dom form.rs select tests under packages/ are untouched
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: the blitz-dom mutator.rs restyle tests under packages/ are untouched
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: the blitz-dom document.rs picture and inline tests under packages/ are untouched
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: the blitz-net lib.rs tests under packages/ are untouched
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: the wpt runner sources and their tests are not touched
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: the reached probe file lies in tests/blitz-tests, slice s12 of entity 20.4, not in slice s03 whose files this absence claim searches
@answer n=22 entity=unit-test-strategy#20.2 v=no
why: the reached probe file lies in tests/blitz-tests, slice s12 of entity 20.4, not in slice s07 whose files this absence claim searches
@answer n=23 entity=unit-test-strategy#20.3 v=no
why: the blitz-vibey-script sources are not reached as the session touched no such file
@answer n=24 entity=unit-test-strategy#20.4 v=yes mode=stale
why: the new tests/blitz-tests/tests/dioxus_runtime_probe.rs joins the searched tests/blitz-tests set, so the 61-file count and the no-in-file-cfg(test) reading are re-read and not shown to hold on it
