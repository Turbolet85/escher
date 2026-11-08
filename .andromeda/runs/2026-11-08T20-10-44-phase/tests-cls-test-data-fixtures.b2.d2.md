@answer n=1 entity=test-data-fixtures#3.24 v=no
why: PAINT_TREE_BENCH_HTML feeds paint_tree_bench, an ignored benchmark the chunk does not touch (not_needed item 8).
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: browser entry helpers in apps/browser, outside the chunk's files and not_needed item 2.
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: `.test` URLs are apps/browser test data, not_needed item 2.
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the favicon PNG belongs to apps/browser favicon tests, not_needed item 2.
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the fetch body belongs to blitz-vibey-script DOM tests, not_needed item 3.
@answer n=6 entity=test-data-fixtures#3.29 v=yes
why: the stand_actions check (task item 5) puts the load page's fetch in flight and reads load-fetch's stated cause, so LoadNet's 200 ms answer binds.
@answer n=7 entity=test-data-fixtures#3.30 v=yes
why: GRAPH_DRIVES and Drive are named in the chunk's needs (item 1), and stand_graph reads the drives that decide which handlers the coverage count sees.
@answer n=8 entity=test-data-fixtures#3.31 v=yes
why: ACTIONABLE_AT_BOOT is read by the new stand_actions check, which the actions verb need (item 5) builds.
@answer n=9 entity=test-data-fixtures#3.32 v=no
why: render_blocking_stylesheet's three cases are a separate check the chunk does not reach.
@answer n=10 entity=test-data-fixtures#3.33 v=no
why: the blitz-net loopback unit tests are outside the chunk's files and needs.
@answer n=11 entity=test-data-fixtures#4.1 v=yes
why: the owed replay test and the coverage check read kept checks through Check::from_bytes, so a fixture or loader added in slice s08 would break the absence this entry states.
