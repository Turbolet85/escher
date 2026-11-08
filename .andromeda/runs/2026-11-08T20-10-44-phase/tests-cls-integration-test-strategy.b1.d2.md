@answer n=1 entity=integration-test-strategy#1.1 v=no
why: upstream's pseudo-element flow-classification layout tests, which the chunk does not reach
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script's script DOM tests, which the chunk does not reach
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: a radio's change event in the script DOM, which the chunk does not reach
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: the URLSearchParams tests in blitz-vibey-script, which the chunk does not reach
@answer n=5 entity=integration-test-strategy#5.1 v=yes
why: the coverage check holds a session and runs its calls through Session::run, which must return after settle in both layout modes as this entity states
@answer n=6 entity=integration-test-strategy#6.1 v=no
why: a harness.click on a load in flight in stand_actions is a call the chunk neither makes nor changes, and its runs go through Session::run, covered by integration-test-strategy#5.1
@answer n=7 entity=integration-test-strategy#7.1 v=yes
why: the UI graph reader DioxusDocument::graph, the graph verb and stand_graph with GRAPH_DRIVES are the graph surface the coverage read lists its handlers from
@answer n=8 entity=integration-test-strategy#8.1 v=yes
why: stand_actions is the chunk's needed actions check, whose listed causes, both-mode reads and no-value read the replay test keeps and compares
@answer n=9 entity=integration-test-strategy#9.1 v=yes
why: stand_actions, the chunk's needed actions check, asserts the Cells actions answer's size bound, a value that check keeps holding
@answer n=10 entity=integration-test-strategy#10.1 v=yes
why: the owed replay test of a kept actions step is the chunk's own named work, CARRY cry-sxrivdk3
@answer n=11 entity=integration-test-strategy#11.1 v=yes
why: a new coverage verb, if it is one, is an MCP tool whose tool_call span parents a command span recording its verb alone, as the graph and actions tool calls do
@answer n=12 entity=integration-test-strategy#12.1 v=yes
why: the coverage read must carry no typed text, as the graph verb's no-value rule and malformed-argument refusal state, and the scope names that read as the graph and actions verbs are
@answer n=13 entity=integration-test-strategy#13.1 v=yes
why: the coverage read must carry no control value, mask or NodeId, as the actions verb's no-value rule and malformed-argument refusal state, and the scope names that read as the actions verb is
@answer n=14 entity=integration-test-strategy#14.1 v=no
why: the session socket lifecycle edges of the escher-session host, which a new verb does not reach, its verb count being the planner's item nine
@answer n=15 entity=integration-test-strategy#14.2 v=yes
why: the scope names the driver's log discipline for any new verb, host_log and mcp_log reading 0 id, name and graph needles at trace with one command span per call
@answer n=16 entity=integration-test-strategy#15.1 v=no
why: a select list's trace rule in host_log and mcp_log, which the coverage read does not touch
@answer n=17 entity=integration-test-strategy#16.1 v=no
why: a hosted popup list opened and chosen through cli_commands, a select-list behaviour the coverage read does not touch
@answer n=18 entity=integration-test-strategy#17.1 v=no
why: a multiple select driven through stand_act_select, a select behaviour the coverage read does not touch
@answer n=19 entity=integration-test-strategy#18.1 v=no
why: the select list driven through stand_act_popup, a popup behaviour the coverage read does not touch
@answer n=20 entity=integration-test-strategy#19.1 v=no
why: the harness-level select_popup list behaviour, which the coverage read does not touch
@answer n=21 entity=integration-test-strategy#20.1 v=no
why: select_popup_painted's frames of the list, a paint rule the coverage read does not touch
@answer n=22 entity=integration-test-strategy#21.1 v=no
why: render_blocking_stylesheet's resource-loading rule, which the chunk does not reach
@answer n=23 entity=integration-test-strategy#22.1 v=yes
why: the graph reader must read each element's current listeners exactly, mounted included, through the vdom dynamic-attribute walk, which the graph reader and the exercised record rely on
@answer n=24 entity=integration-test-strategy#23.1 v=yes
why: the signal registry's subscriber reach and the graph reader's signal reading, which the chunk's signal_registry.rs and graph.rs reach through SignalRegistry
@answer n=25 entity=integration-test-strategy#24.1 v=no
why: a search record of integration tests in the apps, where the chunk adds no apps test and its own placement follows the tests/blitz-tests and src rules
