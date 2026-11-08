@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the session changed none of paint_tree_bench.rs or its external benchmark page, the only sources this entity names
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the session changes no apps/browser helper, so the browser entry builders it cites are not reached
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the session changes no browser test URL, so the .test TLD entries it cites are not reached
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the session changes no favicon helper, so the in-test 1x1 PNG it cites is not reached
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the session changes no blitz-vibey-script test, so the fixed fetch base URL and JSON body are not reached
@answer n=6 entity=test-data-fixtures#3.29 v=yes mode=add-to
why: stand_actions boots the stand load page and puts a fetch in flight on load-fetch, a reader this entity's list of readers does not name
@answer n=7 entity=test-data-fixtures#3.30 v=yes mode=add-to
why: F3, the ACTIONABLE_AT_BOOT table, joins GRAPH_DRIVES as a record in the stand checks' shared common module, which the session reaches
@answer n=8 entity=test-data-fixtures#3.31 v=no
why: the session changes no render_blocking_stylesheet check, so its three stated stylesheet cases are not reached
@answer n=9 entity=test-data-fixtures#3.32 v=no
why: blitz-net is untouched and the session adds no loopback listener, so the two TcpListener tests it cites are not reached
@answer n=10 entity=test-data-fixtures#4.1 v=no
why: the session adds no fixture file or loader in slice s08, and its new ACTIONABLE_AT_BOOT constant and fixtures declarer are neither
