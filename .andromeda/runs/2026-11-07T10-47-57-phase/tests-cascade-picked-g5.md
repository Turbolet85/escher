@account n=1 entity=test-data-fixtures#2.1 v=carried
why: the committed checks record the stand's ids, snapshot text and diffs, which the buffered render must leave byte-for-byte replayable through stand_replay
@account n=2 entity=test-data-fixtures#3.3 v=not_in_scope
why: a shared fixture page the chunk neither adds nor edits; its readers' re-run is a re-check every change owes
@account n=3 entity=test-data-fixtures#3.4 v=not_in_scope
why: select list pages over a select model the chunk does not reach
@account n=4 entity=test-data-fixtures#3.5 v=carried
why: the hazard pages live in dioxus_runtime_probe.rs, the file whose two pins the chunk inverts
@account n=5 entity=test-data-fixtures#3.7 v=not_in_scope
why: the Cells screen's measured size is not re-measured by the chunk; stand_cells_served's re-run is a re-check every change owes
@account n=6 entity=test-data-fixtures#3.20 v=not_in_scope
why: the font payload is untouched; no paint or font path changes
@account n=7 entity=test-data-fixtures#3.28 v=not_in_scope
why: the load page's network answer is unchanged; the chunk changes no load
@account n=8 entity=mocking-stubbing-discipline#1.7 v=not_in_scope
why: HeldNet is not added to or changed; the chunk adds no stand-in
@account n=9 entity=mocking-stubbing-discipline#1.8 v=not_in_scope
why: LateNet is not added to or changed; the chunk adds no stand-in
@account n=10 entity=mocking-stubbing-discipline#1.9 v=not_in_scope
why: LoadNet is not added to or changed; the chunk adds no stand-in
@account n=11 entity=mocking-stubbing-discipline#1.15 v=not_in_scope
why: the no-op net, navigation and shell providers are only used through DocumentConfig::default and keep their behaviour by their own code
@account n=12 entity=mocking-stubbing-discipline#1.21 v=not_in_scope
why: the TimerTicks time stand-in is untouched; the chunk's tests drive time through the vdom and harness only
@account n=13 entity=ci-integration#1.1 v=carried
why: the stand-repeat leg runs every stand check three times with no retry over the new buffered render path
@account n=14 entity=ci-integration#2.1 v=not_in_scope
why: the embolden leg is part of the fast gate every change passes, unchanged by this chunk
@account n=15 entity=ci-integration#5.1 v=carried
why: the record of dioxus_runtime_probe's legs states its two should_panic hazard pins, which the chunk inverts
@account n=16 entity=ci-integration#10.1 v=not_in_scope
why: cargo stopping at the first red target is a property of every red job, not of this chunk's work
@account n=17 entity=ci-integration#15.1 v=carried
why: the ported unit test and inverted pins run under both the workspace and the per-package build, whose engine features differ
@account n=18 entity=ci-integration#16.1 v=carried
why: the ported unit test adds a passed test to each test job's tally, which the per-OS tally record must re-read
@account n=19 entity=ci-integration#17.1 v=carried
why: the local baseline's newest re-count moves by the ported unit test and must be re-counted
@account n=20 entity=ci-integration#18.1 v=not_in_scope standing=S6
why: a new source module counts toward its crate's line coverage floor in the coverage leg
