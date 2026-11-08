@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the external benchmark page is the paint_tree_bench input, which the task drops as not needed
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: browser entries are apps/browser fixtures, and the chunk does not reach apps/browser
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the .test TLD is a browser test URL convention, and the chunk does not reach apps/browser
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the favicon bytes belong to the browser favicon test, which the chunk does not reach
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the fetch body is the blitz-vibey-script DOM test fixture, and no script engine is touched
@answer n=6 entity=test-data-fixtures#3.29 v=yes
why: stand::boot_load is named in scope as a boot the read is proven on, and its LoadNet answer is the behaviour that call carries
@answer n=7 entity=test-data-fixtures#3.30 v=yes
why: GRAPH_DRIVES are the inputs of stand_graph, which the scope names alongside the UI graph reader the read draws on
@answer n=8 entity=test-data-fixtures#3.31 v=no
why: render_blocking_stylesheet is its own check over stylesheet cases the chunk neither changes nor reads
@answer n=9 entity=test-data-fixtures#3.32 v=no
why: the blitz-net loopback listeners are unit tests of a crate the chunk does not reach
@answer n=10 entity=test-data-fixtures#4.1 v=no
why: the absence is a search of slice s08's 16 files, and the task's file and scope records do not name that slice
