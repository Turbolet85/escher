@row_add n=1 row=decision/per-crate-coverage-floors area=ci-integration rec=1
name: Per-crate coverage floors
rule: The coverage leg fails unless line coverage summed under each crate directory holds its floor: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90.
guard: coverage_floor.py chained in the coverage leg
@row_add n=2 row=contract/cli-exit-codes/3 area=e2e-test-strategy rec=3,4
boundary: escher-session CLI exit codes
item: 3
promise: A session error: a command that finds no session, a state directory too long, or a group-open recording directory.
@row_add n=3 row=contract/session-errors/dead area=integration-test-strategy rec=4,8
boundary: Session errors
item: Dead
promise: A socket file that nothing answers on, as after the session host is killed.
@row_add n=4 row=contract/session-errors/unsupported area=integration-test-strategy rec=4,8
boundary: Session errors
item: Unsupported
promise: Returned by the session arms on a non-unix target.
@row_add n=5 row=contract/session-run/graph area=integration-test-strategy rec=5,7
boundary: escher_driver Session
item: graph
promise: The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
