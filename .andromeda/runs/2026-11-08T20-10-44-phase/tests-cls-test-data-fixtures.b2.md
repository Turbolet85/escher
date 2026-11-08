@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the paint_tree_bench benchmark page is paint-benchmark data outside the chunk's reach (not-needed item 8)
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: browser entry helpers belong to apps/browser, which the chunk does not reach (not-needed item 2)
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the `.test` TLD test URLs are apps/browser test data, which the chunk does not reach (not-needed item 2)
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the in-test favicon PNG belongs to apps/browser, which the chunk does not reach (not-needed item 2)
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the fetch body is blitz-vibey-script test data, which the chunk does not reach (not-needed item 3)
@answer n=6 entity=test-data-fixtures#3.29 v=yes
why: stand_actions puts the LoadNet fetch in flight to read load-fetch's stated cause, so its answer depends on this fixed network answer
@answer n=7 entity=test-data-fixtures#3.30 v=yes
why: GRAPH_DRIVES is a named needs table (item 1) whose stand writes are what the coverage read counts as exercised handlers
@answer n=8 entity=test-data-fixtures#3.31 v=yes
why: stand_actions re-measures these actionable counts at boot (item 5), the answer the replay test keeps and compares
@answer n=9 entity=test-data-fixtures#3.32 v=no
why: render_blocking_stylesheet's stylesheet answers are outside the actions, graph and coverage reads the chunk names
@answer n=10 entity=test-data-fixtures#3.33 v=no
why: the blitz-net loopback listeners are unit tests of a crate the chunk does not reach
@answer n=11 entity=test-data-fixtures#4.1 v=no
why: the absence is a search of slice s08 alone, and its record names no path the chunk reaches or adds
