@answer n=1 entity=test-scope-summary#1.1 v=yes
why: CapabilityStatementTest pins docs/capabilities.md's verb and counts and README's tool count, which a new verb moves (task item 9).
@answer n=2 entity=test-scope-summary#2.1 v=yes
why: the examples/seven_guis host_log, mcp_log, mcp_server and cli_commands pins of verb count and the graph and actions needles at trace bind the new verb (task items 9 and 10).
@answer n=3 entity=test-scope-summary#2.2 v=no
why: apps/browser tab test; the chunk does not reach apps/browser (not_needed item 2).
@answer n=4 entity=test-scope-summary#2.3 v=no
why: blitz-dom document, mutator, net and form unit tests; the chunk leaves blitz-dom unchanged and names none of them.
@answer n=5 entity=test-scope-summary#2.4 v=no
why: blitz-dom behaviours (changed set, select, font, text_extent, hit testing) that the coverage read neither names nor calls.
@answer n=6 entity=test-scope-summary#3.1 v=no
why: the paint skip of an inline box holding only block children; the chunk changes no paint and names no such box.
@answer n=7 entity=test-scope-summary#4.1 v=no
why: upstream blitz-dom and blitz-paint commits witnessed by incremental_oracle; no reached file or named behaviour.
@answer n=8 entity=test-scope-summary#5.1 v=no
why: blitz-dom hit testing under a non-invertible transform; the chunk names no hit test through it.
@answer n=9 entity=test-scope-summary#6.1 v=no
why: blitz-dom multiple-select and group rows in select_interaction; the chunk names no select behaviour.
@answer n=10 entity=test-scope-summary#7.1 v=no
why: the default.css chosen-row contrast pair; the chunk paints no row and changes no stylesheet.
@answer n=11 entity=test-scope-summary#8.1 v=no
why: the press residual for non-adjacent multiple rows; no check of this chunk reaches it.
@answer n=12 entity=test-scope-summary#9.1 v=no
why: a key press past a dropped anonymous block in select_popup; the chunk presses no key and touches no layout.
@answer n=13 entity=test-scope-summary#10.1 v=no
why: the key_press_after_container_lost_its_box target; the chunk names no key press or dropped block.
@answer n=14 entity=test-scope-summary#11.1 v=yes
why: dioxus_runtime_probe measures link 2, each node's handlers, and pins the bridge's buffered render, which lives in dioxus_document.rs, a file the chunk reaches whole.
@answer n=15 entity=test-scope-summary#12.1 v=yes
why: the buffered render's focus limit lives in dioxus_document.rs, reached whole, so the chunk's bridge record must keep it as stated.
@answer n=16 entity=test-scope-summary#13.1 v=yes
why: DioxusDocument::graph's unconfirmed reading and GRAPH_DRIVES in tests/blitz-tests/tests/common/mod.rs are the graph read the chunk names (task items 1 and 2).
@answer n=17 entity=test-scope-summary#14.1 v=no
why: the windowed select list's unmeasured limits; the chunk does not touch the popup or the list.
@answer n=18 entity=test-scope-summary#15.1 v=no
why: blitz-dom list marker, text-transform, writing-mode and scrolled_box_client_rect rules; none is reached or named.
@answer n=19 entity=test-scope-summary#15.2 v=no
why: pseudo_element_flow_classification target; the chunk changes no flow classification.
@answer n=20 entity=test-scope-summary#15.3 v=no
why: node-level in-file tests of scrollbar, range and select rules; no reached file or named behaviour.
@answer n=21 entity=test-scope-summary#15.4 v=no
why: the :open state of details and dialog, pinned by open_state; the chunk names no :open behaviour.
@answer n=22 entity=test-scope-summary#16.1 v=no
why: node/select_list.rs list state and its readers; the chunk names no popup reader.
@answer n=23 entity=test-scope-summary#17.1 v=no
why: the open_state check of :open; the chunk does not touch it.
@answer n=24 entity=test-scope-summary#18.1 v=no
why: blitz-dom virtual clock units; the chunk names no clock read or Timer advance.
@answer n=25 entity=test-scope-summary#19.1 v=no
why: blitz-net inline unit tests over test-only listeners; no caller or reached file.
