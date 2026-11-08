@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the paint_tree_bench benchmark page is the paint-bench tier, which the chunk does not reach
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: browser entry helpers belong to apps/browser tests, which the chunk does not reach
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the .test TLD URLs are browser-test data, which the chunk does not reach
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the favicon PNG bytes are browser-test data, which the chunk does not reach
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the fetch body is blitz-vibey-script test data, which the chunk does not reach
@answer n=6 entity=test-data-fixtures#3.29 v=yes
why: stand_actions, which the chunk adds and changes, puts the load page's fetch in flight through the harness to read load-fetch's stated cause, so the LoadNet answer it reads binds
@answer n=7 entity=test-data-fixtures#3.30 v=yes
why: GRAPH_DRIVES and its Drive rows are named by task item 1 and are what stand_graph reads for the graph verb
@answer n=8 entity=test-data-fixtures#3.31 v=yes
why: ACTIONABLE_AT_BOOT holds the actionable node counts that the chunk's stand_actions check reads
@answer n=9 entity=test-data-fixtures#3.32 v=no
why: the render_blocking_stylesheet answers are stated in that check, which the chunk does not touch
@answer n=10 entity=test-data-fixtures#3.33 v=no
why: the blitz-net loopback listeners are unit-test sockets in blitz-net, which the chunk does not reach
@answer n=11 entity=test-data-fixtures#4.1 v=no
why: the absence over slice s08 is a search of files the task does not name, and the chunk's kept checks are carried by standing n=8 and task item 3
