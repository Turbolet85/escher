@answer n=1 entity=unit-test-strategy#1.1 v=no standing=S1
why: the convention itself, where a new unit test sits; the ported test lands in dioxus_document.rs's inline mod tests and adds no out-of-line module
@answer n=2 entity=unit-test-strategy#2.1 v=no
why: blitz-paint's layers.rs is not reached by the change
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: blitz-shell's event.rs redraw test is not reached by the change
@answer n=4 entity=unit-test-strategy#4.1 v=yes
why: the settle loop's quiet-pass rule and poll's answer (Render, Loads, Layout readings) are named by the scope, and the buffered poll must return the same answer
@answer n=5 entity=unit-test-strategy#4.2 v=no
why: escher-driver's wire, session, error, refusal, schema, command, execute and json tests are not reached; the driver is not edited
@answer n=6 entity=unit-test-strategy#5.1 v=no
why: escher-driver's help and covered tests are not reached; the driver is not edited
@answer n=7 entity=unit-test-strategy#6.1 v=no
why: escher-driver's MCP listing test helpers are not reached; the driver is not edited
@answer n=8 entity=unit-test-strategy#7.1 v=no
why: escher-driver's type help test is not reached; the driver is not edited
@answer n=9 entity=unit-test-strategy#8.1 v=yes
why: the stand-repeat case pins three rounds over the seventeen named stand targets, the leg the scope names for the new render path; the rest of the CI shape (caches, pins, uploads, install bounds) is untouched
@answer n=10 entity=unit-test-strategy#9.1 v=no
why: the blitz-tests manifest shape is not edited and the chunk adds no test target; dioxus_runtime_probe is already reached by run all
@answer n=11 entity=unit-test-strategy#10.1 v=no
why: the cold-agent pipe is not reached; its verdict and scripts do not change
@answer n=12 entity=unit-test-strategy#11.1 v=no
why: the capability statement and its counts are not reached; no capability text changes
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: blitz-dom node tests (scrollbar, element, node, range, select) are not reached; the engine's document is unchanged
@answer n=14 entity=unit-test-strategy#13.1 v=no
why: blitz-dom select steps, spans and groups are not reached; the engine is unchanged
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: blitz-dom select_list tests are not reached; the engine is unchanged
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: blitz-dom form.rs select submission tests are not reached; the engine is unchanged
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: blitz-dom mutator.rs restyle tests are the engine's own; the change edits the Dioxus bridge and not mutator.rs
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: blitz-dom document.rs picture and inline-flag tests are not reached; the engine is unchanged
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: blitz-net's inline tests are not reached by the change
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: the wpt runner is not reached; the chunk runs on the Dioxus bridge, not the WPT runner
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: a measured absence over slice s03 that the change does not add a test to; it states no rule the chunk must honour
@answer n=22 entity=unit-test-strategy#20.2 v=no
why: a measured absence over slice s07 that the change does not add a test to; it states no rule the chunk must honour
@answer n=23 entity=unit-test-strategy#20.3 v=no
why: blitz-vibey-script is not reached; ScriptDocument is not a Dioxus document
@answer n=24 entity=unit-test-strategy#20.4 v=no
why: a measured absence of in-file cfg(test) modules in tests/blitz-tests; the chunk adds no such module, its probe edit being integration tests
