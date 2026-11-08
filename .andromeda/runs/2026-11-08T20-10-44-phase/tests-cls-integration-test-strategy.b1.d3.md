@answer n=1 entity=integration-test-strategy#1.1 v=no
why: pseudo-element flow tests of layout, which the chunk's dioxus-native-dom handler record does not reach
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script DOM tests in another crate, outside the chunk's reach
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: a radio change event in blitz-vibey-script, outside the chunk's reach
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: URLSearchParams tests in blitz-vibey-script, outside the chunk's reach
@answer n=5 entity=integration-test-strategy#5.1 v=yes
why: the coverage check's calls run through Session::run returning after settle with one escher_driver line per call, in both layout modes
@answer n=6 entity=integration-test-strategy#6.1 v=no
why: the load-wait behaviour of a driver click, which the task leaves not needed and the coverage read neither adds nor changes
@answer n=7 entity=integration-test-strategy#7.1 v=yes
why: stand_graph checks the DioxusDocument::graph reader, GraphNode.handlers and the graph verb that the coverage read is built on
@answer n=8 entity=integration-test-strategy#8.1 v=yes
why: stand_actions holds the actions verb's listed nodes and read-only shape, which the chunk must keep and its replay test compares
@answer n=9 entity=integration-test-strategy#9.1 v=yes
why: the owed replay compares an actions answer whose size bound applies to whichever screen the scope leaves open
@answer n=10 entity=integration-test-strategy#10.1 v=yes
why: this is the owed kept actions-step replay test, CARRY cry-sxrivdk3, which the chunk writes
@answer n=11 entity=integration-test-strategy#11.1 v=yes
why: a coverage verb reached over MCP must parent a command span recording its verb alone, as the graph and actions tool calls do
@answer n=12 entity=integration-test-strategy#12.1 v=yes
why: the coverage read must hold no value, typed text or NodeId, as the graph verb answer holds none under task item 7
@answer n=13 entity=integration-test-strategy#13.1 v=yes
why: the coverage read is checked as the actions verb is, read-only with no value and no NodeId written
@answer n=14 entity=integration-test-strategy#14.1 v=no
why: session socket lifecycle, socket modes and idle expiry, which the chunk neither changes nor calls
@answer n=15 entity=integration-test-strategy#14.2 v=yes
why: a new verb's trace logs must read 0 id, name and graph needles in host_log and mcp_log with one command span per call
@answer n=16 entity=integration-test-strategy#15.1 v=no standing=S6
why: the select list no-option-logged test is the convention that a check logs nothing of the screen, and the coverage read does not touch a select list
@answer n=17 entity=integration-test-strategy#16.1 v=no
why: a hosted select list click through cc-popup, a select interaction the coverage read does not reach
@answer n=18 entity=integration-test-strategy#17.1 v=no
why: a multiple select driven through the driver, a control the chunk does not touch
@answer n=19 entity=integration-test-strategy#18.1 v=no
why: the select popup list through a held session, outside the chunk's coverage reach
@answer n=20 entity=integration-test-strategy#19.1 v=no
why: select popup harness input and key events, which the coverage read does not drive
@answer n=21 entity=integration-test-strategy#20.1 v=no
why: select popup painted frames, outside the chunk's reach
@answer n=22 entity=integration-test-strategy#21.1 v=no
why: render-blocking stylesheet loading, outside the chunk's reach
@answer n=23 entity=integration-test-strategy#22.1 v=yes
why: a removed listener's attribute stands on a node, so the handler reader and the one-per-node-and-event count must read listeners exactly
@answer n=24 entity=integration-test-strategy#23.1 v=yes
why: the signal registry and graph reader the chunk edits take each task's signal subscribers, read by graph::tests and stand_graph
@answer n=25 entity=integration-test-strategy#24.1 v=no standing=S5
why: the apps' #[test] functions sit in src cfg(test) mod tests, the unit-test placement convention the chunk's new unit tests keep
