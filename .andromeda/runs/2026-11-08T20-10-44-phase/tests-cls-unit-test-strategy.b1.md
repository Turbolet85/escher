@answer n=1 entity=unit-test-strategy#1.1 v=yes
why: the scope names the bridge_tests child module and its cfg(test) shim, and this entry records its out-of-line location and path.
@answer n=2 entity=unit-test-strategy#2.1 v=no standing=S5
why: the entry records the test-module location of blitz-paint's layers.rs, a file the chunk does not reach, under the unit-test location convention.
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: blitz-shell's event.rs is not reached by the chunk and its one test is not one the chunk must honour.
@answer n=4 entity=unit-test-strategy#4.1 v=yes
why: the scope names Session::run returning after settle, and the settle-loop rules carry that call's behaviour, which the coverage check relies on; the screenshot and contrast rules are not reached.
@answer n=5 entity=unit-test-strategy#4.2 v=yes
why: the verb table, command validation, JSON forms, command line, MCP, recording and replay rules are what a new verb and the replay test reach (task items 3, 4, 9, 11).
@answer n=6 entity=unit-test-strategy#5.1 v=yes
why: the VERBS count at 14 and describe's count row are the pins a new verb moves to fifteen (task item 9).
@answer n=7 entity=unit-test-strategy#6.1 v=yes
why: the MCP listing test asserts the fourteen verbs and is among the pins a new verb moves (task item 9, stand_mcp_tools).
@answer n=8 entity=unit-test-strategy#7.1 v=no
why: the help of type is a unit of schema_tests.rs that a new verb does not edit, so it is a re-check every change owes.
@answer n=9 entity=unit-test-strategy#8.1 v=no
why: the actions.rs unit tests over call validation and the id bound are a re-check, since the chunk keeps the actions verb's shape and edits neither actions.rs nor its calls.
@answer n=10 entity=unit-test-strategy#9.1 v=yes
why: the field-kind table keyed per verb for graph and actions nodes is what a new verb's result field reaches (task item 11).
@answer n=11 entity=unit-test-strategy#10.1 v=yes
why: the coverage-floor cases over a seeded lcov, the stand-repeat cases naming the seventeen targets, and the capability-statement and coverage-floor test files bind (task items 9, 12, 13); the other CI invariants are a re-check.
@answer n=12 entity=unit-test-strategy#11.1 v=no standing=S1
why: the manifest holds exactly one [[test]] table, the convention the standing forbids a new one of, and the chunk adds no table.
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: the cold-agent pipe's verb list is named to Cold-agent bar and not counted here (task item 9), and the rest of its pipe is not reached.
@answer n=14 entity=unit-test-strategy#13.1 v=yes
why: test_capability_statement.py pins VERBS against docs/capabilities.md and README's tool count, which a new verb moves (task item 9).
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: blitz-dom's node scrollbar, text-input and disabled-state tests are not reached by the chunk and are not needed (task item 5 not_needed).
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: blitz-dom's select node steps, spans and groups are not reached by the chunk and are not needed (task item 5 not_needed).
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: blitz-dom's select_list.rs is not reached by the chunk and is not needed (task item 5 not_needed).
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: blitz-dom's form.rs submitted-entry tests are not reached by the chunk and are not needed (task item 5 not_needed).
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: blitz-dom's mutator.rs restyle tests are not reached by the chunk and are not needed (task item 5 not_needed).
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: blitz-dom's document.rs picture and inline-flag tests are not reached by the chunk and are not needed (task item 5 not_needed).
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: blitz-net's inline tests are not reached by the chunk and are not needed.
@answer n=22 entity=unit-test-strategy#21.1 v=yes
why: graph.rs is reached whole and graph::tests pins the UI graph reader the coverage read is built on.
@answer n=23 entity=unit-test-strategy#22.1 v=yes
why: signal_registry.rs is reached whole and signal_registry::tests pins the registry the dispatch record is added to.
@answer n=24 entity=unit-test-strategy#23.1 v=yes
why: the scope names the bridge_tests child module, and this test drives the dispatch record the new exercised record must hold on an app that registers no state.
@answer n=25 entity=unit-test-strategy#24.1 v=yes
why: the read-only check re-measures the whole snapshot before and after (task item 7), and this entry states a value that snapshot holds (NodeState.clickable).
