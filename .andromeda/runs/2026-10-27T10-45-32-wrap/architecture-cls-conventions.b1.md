# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · yes · stale · construct.rs gained lines at 313-317, so its cited :552 moved to :557, and apps/browser/src/tab.rs is a merge surface reached whole, so its cited :46-49, :105-107 and :117-119 can have moved
conventions#2.1 · no · every manifest it cites is byte-identical to the base (Untouched) and its cited crate roots are not in the reached files
conventions#3.1 · yes · stale · stylo.rs gained a line at 510, so its cited spec-URL lines :993, :1023-1026 and :1105 moved down by one
conventions#4.1 · yes · stale · stylo.rs gained a line at 510 (cited :1216 moves to :1217), construct.rs gained lines at 313-317 (cited :417 and :602 move to :422 and :607), and runtime.rs keeps 151 lines so its cited :2056 delimiter and dom/element.rs:224 moved with the module split
conventions#5.1 · yes · stale · runtime.rs keeps 151 lines so its cited :1386-1408 natives moved into the runtime modules, and document.rs gained a line at 16 so its builder citations :105-108, :121-124 and :139-142 move by one
conventions#6.1 · yes · stale · mutator.rs gained lines at 368-371 and 520-523 (cited :1337-1338 moves by eight), construct.rs gained lines at 313-317 (cited :461-469 moves by five) and blitz-shell lib.rs gained lines at 90, 93-98 and 104-106 (cited :162-173 moves by ten)
conventions#7.1 · yes · stale · stylo.rs gained a line at 510, so its cited unsafe blocks :751-758, :1326 and :1398-1403 moved by one, and runtime.rs keeps 151 lines so its let-chain citation :1498-1500 moved into the new modules
conventions#8.1 · yes · stale · dioxus_document.rs gained a test at 725-759, so its cited bridge-test declaration :818-820 moved down by 35, and mutator.rs gains at 368-371 and 520-523 move its cited :1528-1529 by eight
conventions#9.1 · no · its cited escher-driver files (cli.rs, recording.rs) are untouched (empty escher-driver diff), so the three-of-eight placement holds
conventions#9.2 · no · fixtures/mod.rs is not in the reached files and the report names no shared fixture the new target reads
conventions#9.3 · no · its cited escher-driver mcp_tests.rs is untouched (empty diff) and stand_requirement_sweep.rs is not in the reached files
conventions#10.1 · yes · stale · apps/browser/src/tab.rs, a merge surface the change edited, can have moved its cited #[derive(Store)] lines :27 and :39
conventions#11.1 · no · the examples/assets fixtures it cites are not in the reached files and no example file is reached
