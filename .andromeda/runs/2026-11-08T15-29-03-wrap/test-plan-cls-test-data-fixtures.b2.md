@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the paint-tree bench page is supplied by paint_tree_bench.rs, which the session does not reach
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the browser entry helpers live in apps/browser, which the session leaves unchanged
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the .test TLD URLs are set in apps/browser files, which the session does not reach
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the favicon PNG is encoded in apps/browser/src/favicon.rs, which the session does not reach
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the fetch body is set in blitz-vibey-script tests, which the session leaves unchanged
@answer n=6 entity=test-data-fixtures#3.29 v=yes mode=stale
why: stand_actions boots the load page and puts a fetch in flight on load-fetch, so the list of boots of the load page no longer names every one
@answer n=7 entity=test-data-fixtures#3.30 v=yes mode=add-to
why: F3, the ACTIONABLE_AT_BOOT table, joins the shared tables beside GRAPH_DRIVES in the reached tests/blitz-tests/tests/common/mod.rs
@answer n=8 entity=test-data-fixtures#3.31 v=no
why: render_blocking_stylesheet.rs is not reached, and the blitz-net loader and harness its three cases run on are unchanged
@answer n=9 entity=test-data-fixtures#3.32 v=no
why: the blitz-net loopback unit tests are in packages/blitz-net, which the session leaves unchanged
@answer n=10 entity=test-data-fixtures#4.1 v=yes mode=stale
why: the reached test files name the fixtures module (stand_actions reads fixtures::password_labelled), so the no-match search for fixture over slice s08 is not shown to still hold
