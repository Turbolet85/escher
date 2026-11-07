@answer n=1 entity=unit-test-strategy#1.1 v=no standing=S1
why: a new unit test sits in a #[cfg(test)] mod tests block in its source file or a #[path] child file, the convention S1 lists; the ported test goes into dioxus_document.rs's mod tests, the default the list states
@answer n=2 entity=unit-test-strategy#2.1 v=no
why: blitz-paint's layers.rs tests, a file the chunk does not reach
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: blitz-shell's event.rs redraw test, a crate the chunk does not reach
@answer n=4 entity=unit-test-strategy#4.1 v=yes
why: the settle loop's quiet-pass rule and the Render, Loads and Layout readings it reads from poll are named by the scope (task item 8), so the buffered poll must return the same answer
@answer n=5 entity=unit-test-strategy#4.2 v=no
why: escher-driver's wire, session, error, refusal, schema and recording rules are over its own code, a crate the chunk does not edit
@answer n=6 entity=unit-test-strategy#5.1 v=no
why: escher-driver's help texts, VERBS count and cause count, a crate the chunk does not edit
@answer n=7 entity=unit-test-strategy#6.1 v=no
why: escher-driver's MCP listing test helpers, a file the chunk does not reach
@answer n=8 entity=unit-test-strategy#7.1 v=no
why: escher-driver's type help in schema_tests.rs, a file the chunk does not reach
@answer n=9 entity=unit-test-strategy#8.1 v=yes
why: the stand-repeat leg's three rounds over the seventeen named targets and the fast leg's ci-leg.sh gate are the scope's own obligations (task items 12 and 16), which the chunk's code must pass
@answer n=10 entity=unit-test-strategy#9.1 v=no
why: the blitz-tests manifest's single all target and its test = false shape, which the chunk does not touch (task not_needed item 8)
@answer n=11 entity=unit-test-strategy#10.1 v=no
why: the cold-agent pipe test_cold_agent.py, scripts the chunk does not touch and no live run is a check (task not_needed item 2)
@answer n=12 entity=unit-test-strategy#11.1 v=no
why: the capability statement test over docs and schema counts, which the chunk does not change (task not_needed item 8)
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: blitz-dom node tests for scrollbar fade, text-input scroll and element state, engine behaviour the chunk only uses and the scope does not name
@answer n=14 entity=unit-test-strategy#13.1 v=no
why: blitz-dom node/select.rs step, span and group tests, engine behaviour the chunk only uses and the scope does not name
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: blitz-dom select_list.rs tests, engine behaviour the chunk only uses and the scope does not name
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: blitz-dom form.rs select submission tests, engine behaviour the chunk does not call and the scope does not name
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: blitz-dom mutator.rs incremental restyle identity is the engine's own guarantee, which the chunk only uses through the existing writer; the scope names layout identity over incremental_oracle, not restyle
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: blitz-dom document.rs picture and inline-flag tests, engine behaviour the chunk only uses and the scope does not name
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: blitz-net's inline NetHandler tests, a crate the chunk does not reach
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: wpt runner tests, which run on upstream only and over HtmlDocument, not the Dioxus bridge (task not_needed item 1)
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: a search reading of slice s03 holding no unit tests, a slice the chunk's dioxus-native-dom files are not in
@answer n=22 entity=unit-test-strategy#20.2 v=no
why: a search reading of the other seven files of slice s07, which the chunk adds no test to
@answer n=23 entity=unit-test-strategy#20.3 v=no
why: blitz-vibey-script source files, a crate the chunk does not reach
@answer n=24 entity=unit-test-strategy#20.4 v=no
why: a search reading of in-file cfg(test) modules in tests/blitz-tests, where the chunk adds none; its inverted pins are integration tests of the probe file
