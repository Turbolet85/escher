@account n=1 entity=standard-contracts#1.1 v=carried
why: the graph is keyed on the agent-visible node the document's `accessibility_tree`, enriched by `DioxusDocument` with stable ids, hands out
@account n=2 entity=standard-contracts#2.2 v=not_in_scope
why: `select_value` feeds the snapshot's existing `value`, which the chunk leaves as it reads; the graph's values are masked through `is_masked_input`, not through it
@account n=3 entity=standard-contracts#2.6 v=not_in_scope
why: the select readers and tests the driver reads are used unchanged and the scope names none of them
@account n=4 entity=standard-contracts#3.1 v=carried
why: `build_accessibility_tree` builds the agent-visible nodes and their bounds the graph is keyed on (task item 4)
@account n=5 entity=standard-contracts#3.2 v=not_in_scope
why: a single select's `expanded` write is engine tree state the chunk neither changes nor re-reads
@account n=6 entity=standard-contracts#3.3 v=not_in_scope
why: the tree's focus fallback is engine behaviour the graph read does not reach
@account n=7 entity=standard-contracts#3.4 v=not_in_scope
why: a native control's `toggled` write is engine tree state the chunk neither changes nor re-reads
@account n=8 entity=standard-contracts#4.1 v=carried
why: the «reacts to a click» reading reads which elements a Click reaches with an app handler, keyboard activation's queued Click included
@account n=9 entity=standard-contracts#5.1 v=carried
why: the graph reader maps vdom nodes and readers' scopes onto document nodes, and a stale id indexed into `NodeTree` panics
@account n=10 entity=standard-contracts#6.1 v=not_in_scope
why: the client rect under transforms feeds the snapshot's existing `bounds`, used unchanged by the graph
@account n=11 entity=standard-contracts#6.6 v=not_in_scope
why: style data and the resolve context are engine internals the graph read does not reach
@account n=12 entity=standard-contracts#8.1 v=not_in_scope
why: a stacking-context arm of layout and paint the graph read does not reach
@account n=13 entity=standard-contracts#9.1 v=not_in_scope
why: node data, widgets and serialization are used unchanged and the scope names none of them
@account n=14 entity=standard-contracts#10.1 v=carried
why: `is_disabled` gates the pointer and activation keys, so the «reacts to a click» reading must decide how a disabled control with a handler reads
@account n=15 entity=standard-contracts#10.9 v=not_in_scope
why: an inline box's paint flag the graph read does not reach
@account n=16 entity=standard-contracts#11.1 v=not_in_scope
why: provider traits are untouched; the graph makes no fetch
@account n=17 entity=standard-contracts#13.5 v=not_in_scope
why: the blend's missing frame witness is paint, which the graph does not reach
@account n=18 entity=standard-contracts#15.1 v=not_in_scope standing=S1
why: the harness is the stand checks' home and helper, owed equally by every new stand check
@account n=19 entity=standard-contracts#16.1 v=not_in_scope
why: the harness's virtual clock is used unchanged; the graph read moves no time
@account n=20 entity=standard-contracts#17.1 v=carried
why: the stand's boots and root contexts are edited as its signals move onto the registry, and it must still install no telemetry and read no env var
@account n=21 entity=standard-contracts#18.1 v=not_in_scope
why: the stand's font is no input of a boolean «reacts to a click» reading or of the graph
@account n=22 entity=standard-contracts#19.1 v=carried
why: Cells is one screen the starting «reacts to a click» reading is re-pinned on and whose signals move onto the registry
@account n=23 entity=standard-contracts#19.2 v=carried
why: the load page is a served stand screen whose click-driven state moves onto the registry with the stand's other signals
@account n=24 entity=standard-contracts#19.3 v=not_in_scope
why: `LoadNet` is a network stand-in the graph neither reads nor changes
@account n=25 entity=standard-contracts#20.1 v=carried
why: a span field on an escher target prints unless it is in `CONTENT_FIELDS`, so the graph read's span may carry no node, name, handler, state or value
