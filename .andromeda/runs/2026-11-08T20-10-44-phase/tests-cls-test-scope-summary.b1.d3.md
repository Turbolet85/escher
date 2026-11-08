@answer n=1 entity=test-scope-summary#1.1 v=yes
why: CapabilityStatementTest pins docs/capabilities.md's verb names, counts and README's tool count, which a new driver verb moves (task item 9 names test_capability_statement.py)
@answer n=2 entity=test-scope-summary#2.1 v=yes
why: host_log and mcp_log's graph and actions needles at trace, the cli and mcp verb tests, the fourteen-verb tools/list pin and recording.rs's byte-for-byte check of the committed checks are all named by the scope (items 3, 9, 10)
@answer n=3 entity=test-scope-summary#2.2 v=no
why: the apps/browser tests are named not_needed (task item 2) and the chunk does not touch apps/browser/src/tab.rs
@answer n=4 entity=test-scope-summary#2.3 v=no
why: blitz-dom document, mutator, net and form tests pin engine behaviour the chunk neither edits nor reads by any rule the scope names
@answer n=5 entity=test-scope-summary#2.4 v=yes
why: the actions verb's listing of actionable nodes (unkeyed_actionable) and its listed covered cause are what the scope's actions verb answers and the owed replay compares (items 5, 4)
@answer n=6 entity=test-scope-summary#3.1 v=no
why: the inline box holding only block children is a paint skip owned by a CARRY the chunk does not reach
@answer n=7 entity=test-scope-summary#4.1 v=no
why: the upstream table-damage, Stylo prefs, sub-pixel size and culling commits are blitz-dom and blitz-paint witnesses the chunk does not change or read
@answer n=8 entity=test-scope-summary#5.1 v=no
why: a non-invertible transform taking no hit is blitz-dom hit-testing, which the scope does not name and the coverage read and actions answer do not call
@answer n=9 entity=test-scope-summary#6.1 v=no
why: the multiple-select row and group behaviour of select_interaction is not named by the scope and is not read by the coverage read or the actions answer
@answer n=10 entity=test-scope-summary#7.1 v=no
why: the chosen-row contrast pair is a default.css value the chunk neither edits nor reads
@answer n=11 entity=test-scope-summary#8.1 v=no
why: the driver's inability to build non-adjacent multiple rows and a Ctrl press is a press residual outside the chunk's verbs
@answer n=12 entity=test-scope-summary#9.1 v=no
why: the key press past a dropped block is a select_popup rule and the chunk presses no key
@answer n=13 entity=test-scope-summary#10.1 v=no
why: key_press_after_container_lost_its_box tests a key press on a dropped anonymous block, which the chunk does not press or touch
@answer n=14 entity=test-scope-summary#11.1 v=yes
why: dioxus_runtime_probe measures link 2 (each node's handlers) and link 4 (a held signal's readers), the graph reader and signal registry the chunk edits (items 1, 2)
@answer n=15 entity=test-scope-summary#12.1 v=no
why: the buffered render's limit is about set_focus_to on nodes created in the same render, and the chunk's dispatch record and graph reader do not focus
@answer n=16 entity=test-scope-summary#13.1 v=yes
why: the graph's unconfirmed flag limit over conditional writes is the confirmed flag the chunk reads, and the GRAPH_DRIVES table the scope names drives those writes (items 1, 2)
@answer n=17 entity=test-scope-summary#14.1 v=no
why: the windowed select list's unmeasured limits are a select_popup matter the chunk does not reach
@answer n=18 entity=test-scope-summary#15.1 v=no
why: the list marker, text-transform, writing-mode and scrolled-box bounds tests are blitz-dom layout the chunk does not reach
@answer n=19 entity=test-scope-summary#15.2 v=no
why: pseudo-element flow classification is a blitz-dom layout test the chunk does not reach
@answer n=20 entity=test-scope-summary#15.3 v=no
why: the in-file node tests (scrollbar, element, range, select, out-of-flow and classes) are blitz-dom node rules the chunk does not reach
@answer n=21 entity=test-scope-summary#15.4 v=no
why: the :open state of details and dialog is a blitz-dom node rule the chunk does not reach
@answer n=22 entity=test-scope-summary#16.1 v=no
why: the select_list in-file tests and popup checks are blitz-dom select-list rules the chunk does not reach
@answer n=23 entity=test-scope-summary#17.1 v=no
why: the open_state check of :open is a blitz-dom engine check the chunk does not reach
@answer n=24 entity=test-scope-summary#18.1 v=no
why: the virtual clock and DocumentConfig.clock tests are blitz-dom time rules and the chunk reads no clock
@answer n=25 entity=test-scope-summary#19.1 v=no
why: the blitz-net inline TCP unit tests are served by no binary and reached by no caller the chunk touches
