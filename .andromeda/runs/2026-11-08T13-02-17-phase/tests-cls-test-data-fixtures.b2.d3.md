@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the external benchmark page feeds paint_tree_bench, which is not_needed (benchmarks run ignored or by hand)
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: browser entries belong to apps/browser, which the chunk does not reach
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the .test URLs are browser test data, outside the chunk's reach
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: favicon bytes are browser test data, outside the chunk's reach
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the fetch body belongs to blitz-vibey-script, which no script engine change touches
@answer n=6 entity=test-data-fixtures#3.29 v=yes
why: stand::boot_load is a boot the read is proven on in both layout modes, so its LoadNet answer and the other boots keeping net_provider None bind
@answer n=7 entity=test-data-fixtures#3.30 v=yes
why: stand_graph is in scope and reads the GRAPH_DRIVES writes over the UI graph reader that graph.rs reaches whole
@answer n=8 entity=test-data-fixtures#3.31 v=no
why: the render-blocking stylesheet check is not in the scope and the chunk does not reach its stylesheet answers
@answer n=9 entity=test-data-fixtures#3.32 v=no
why: the blitz-net loopback listeners are served by no binary the chunk reaches
@answer n=10 entity=test-data-fixtures#4.1 v=no
why: the record is a search result of absent fixtures in slice s08, a statement of absence the chunk does not need to honour
