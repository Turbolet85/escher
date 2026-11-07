@answer n=1 entity=unit-test-strategy#1.1 v=no standing=S1
why: the convention of where a unit test sits (a cfg(test) mod tests block or a #[path] child); the chunk's ported test follows it inline in dioxus_document.rs and adds no out-of-line module the convention's list must record
@answer n=2 entity=unit-test-strategy#2.1 v=no
why: blitz-paint's layers.rs is not a reached file and its tests check paint layers the chunk does not touch
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: blitz-shell's event test is outside the reached files and the scope names no shell redraw behaviour
@answer n=4 entity=unit-test-strategy#4.1 v=yes
why: the settle loop's answers (one-pass idle, k+1 for k polls, Render at the pass limit, Loads, quiet-pass rule) are the poll answer the scope names as the buffered poll's return value that must not move, so the settle clauses bind; the screenshot and contrast readers in the same entity are a re-check
@answer n=5 entity=unit-test-strategy#4.2 v=no
why: escher-driver's wire, session and error tests are outside the reached files and the scope excludes escher-driver
@answer n=6 entity=unit-test-strategy#5.1 v=no
why: escher-driver's help and covered tests are outside the reached files and the scope excludes escher-driver
@answer n=7 entity=unit-test-strategy#6.1 v=no
why: escher-driver's MCP listing test is outside the reached files and the scope excludes escher-driver
@answer n=8 entity=unit-test-strategy#7.1 v=no
why: escher-driver's type help test is outside the reached files and the scope excludes escher-driver
@answer n=9 entity=unit-test-strategy#8.1 v=no
why: the CI workflow and leg-script tests guard workflow shape, install bounds, leg dispatch and the seeded-lcov coverage row; the chunk edits no workflow, leg script, install step or target list, so these are re-checks every change owes through the gate
@answer n=10 entity=unit-test-strategy#9.1 v=no
why: the blitz-tests manifest's single [[test]] table and autotests setting stay as they are, since the chunk adds no target and edits no manifest and dioxus_runtime_probe stays picked up by run all
@answer n=11 entity=unit-test-strategy#10.1 v=no
why: the cold-agent pipe is outside the reached files and the scope excludes it
@answer n=12 entity=unit-test-strategy#11.1 v=no
why: the capability statement test reads docs and schema counts that the chunk does not change
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: blitz-dom node tests for scrollbar, text input and element state test the engine's own document, which the chunk leaves unchanged
@answer n=14 entity=unit-test-strategy#13.1 v=no
why: blitz-dom select step, span and group tests test the engine's own document, which the chunk leaves unchanged
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: blitz-dom select_list tests test the engine's own document, which the chunk leaves unchanged
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: blitz-dom form submission tests for select entries test the engine's own document, which the chunk leaves unchanged
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: blitz-dom mutator restyle tests compare restyle against a fresh document, a rule over the engine's restyle the scope does not name; the chunk's replay uses the mutator as a component it leaves unchanged, so its guarantee holds by its own code
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: blitz-dom document picture and inline-flag tests test the engine's own document, which the chunk leaves unchanged
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: blitz-net handler tests are outside the reached files and the scope names no network behaviour
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: the wpt runner's unit tests run on upstream only and are outside the Dioxus bridge
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: the absence is over slice s03's files, which name none of the chunk's reached files, and the chunk adds its test to dioxus-native-dom and to none of those files
@answer n=22 entity=unit-test-strategy#20.2 v=no
why: the absence over slice s07's other files names none of the chunk's reached files, and the chunk adds no test there
@answer n=23 entity=unit-test-strategy#20.3 v=no
why: blitz-vibey-script holds no reached file and ScriptDocument is not a Dioxus document
@answer n=24 entity=unit-test-strategy#20.4 v=no
why: the chunk edits the existing pins and module text of dioxus_runtime_probe.rs and adds no in-file cfg(test) module under tests/blitz-tests, and the re-read stale_node_mapping and incremental_oracle carry no such rule
