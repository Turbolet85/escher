@account n=1 entity=test-strategy#9.1 v=not_touched
why: the chunk adds no test-target file and no out-of-line test module (the report lists fct-7m7lxsmy no-change under `probe`), so the directory pattern and its counts still hold
@account n=2 entity=unit-test-strategy#1.1 v=not_touched
why: the new mutation_buffer.rs holds no test and the ported test sits in dioxus_document's in-file module, so the eight out-of-line modules stand as stated
@account n=3 entity=unit-test-strategy#20.4 v=not_touched
why: the probe file's change re-says helpers and pins and keeps 10 `#[test]`, adding no in-file `#[cfg(test)]` module to tests/blitz-tests
@account n=4 entity=integration-test-strategy#13.1 v=not_touched
why: mutation_writer.rs is byte-identical and the buffer replays the same listener calls through it; the report lists fct-xs6og5lj no-change
@account n=5 entity=integration-test-strategy#14.1 v=not_touched
why: the signal-readers link and its test are unchanged by the buffered render; the report lists fct-3vcgyjbv no-change
@account n=6 entity=e2e-test-strategy#2.1 v=proposed
why: it states the RefCell already borrowed panic as standing with its fix routed, and the report retires the panic as fixed by the buffered render
@account n=7 entity=e2e-test-strategy#3.1 v=proposed
why: it names the two should_panic pins the report retires, renamed and inverted to the two spawned_acts tests that are now the fix's witness on 0.7
@account n=8 entity=test-data-fixtures#3.5 v=not_touched
why: the hazard(spawned) page is unchanged; the report carries fct-ay5qvau6 as citation-only, the citation sweep's
@account n=9 entity=ci-integration#5.1 v=proposed
why: it says the probe pins the MountedData hazard as two should_panic tests, which the report retires
@account n=10 entity=ci-integration#16.1 v=proposed
why: the per-OS tallies are re-read on CI#42371051720 with the probe chunk's and this chunk's moves, and F3 extends its ANSI-strip rule
@account n=11 entity=ci-integration#17.1 v=not_touched
why: the report lists fct-kjlei6et as a dated reading true as dated and gives no new local fast tally; the a11y leg 14 · 114 and run stand 237 · 0 · 7 are unmoved
@account n=12 entity=quality-gates-coverage-targets#2.1 v=proposed
why: line 17's dioxus-native-dom reading moves to 2937/3387 = 86.71 % at CI#42371051720, its res-6ssktw6x owner clause is re-said absorbed, and F4 carries the new file's coverage
@account n=13 entity=quality-gates-coverage-targets#5.1 v=not_touched
why: the complexity reading is dated with no later reading claimed and the report states none for the new module, so the claim still holds as dated
