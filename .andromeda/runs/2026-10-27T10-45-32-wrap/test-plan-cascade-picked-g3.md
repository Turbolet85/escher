# test-plan · picked accounting · group 3 of 4

## Picked
test-strategy#8.1 · proposed · its blitz-tests count reads 147 files and 146 targets; the report's `bt-files` moves it to 148 · 147 at `test-strategy.md:54`
unit-test-strategy#1.1 · not touched · the added tests sit inline (`dioxus_document.rs` 725-759, `tab.rs`'s own `mod tests`), so the eight out-of-line modules stand as stated; only cited lines shifted, the citation sweep's
unit-test-strategy#8.1 · not touched · `node/element.rs` gained the open-state lines only, the five text-input scroll tests and the node tests unchanged; the shifted lines are the citation sweep's
unit-test-strategy#10.3 · proposed · the absence holds, but its 32-file search set predates the split that moved the crate's runtime and element bindings into 13 new Rust modules, none holding a test module
unit-test-strategy#10.4 · not touched · the one new blitz-tests file holds three `#[test]` fns, two helpers and a const by the report's New text, no module, so the absence holds; the 61-file count is that search's reading, as earlier additions left it
integration-test-strategy#1.1 · proposed · the Script ↔ DOM row carries F4 (the `file:`-load tests) and F5 (the CSS property accessors test)
integration-test-strategy#3.1 · not touched · the browser's new test sits inside `src/tab.rs`'s own `mod tests`, so every app hit still sits in `src`
e2e-test-strategy#2.2 · not touched · the report classes the browser tab test a unit test and adds no test that launches a window; the native boot smoke is a gate, not a test
test-data-fixtures#3.1 · proposed · the seed table carries F6, files written under the temp dir at test time
test-data-fixtures#3.4 · proposed · the script tests' inline HTML is no longer per test alone: two `file:`-load tests share one `FILE_LOAD_TEST_HTML` and `FILE_LOAD_TEST_FILES`
test-data-fixtures#3.5 · not touched · the text-input literals in `node/element.rs` are unchanged, only lines above them added; citation shifts are the sweep's
test-data-fixtures#3.11 · not touched · the mutator's lines added at 368-371 and 520-523 leave its test viewport as stated; citation shifts are the sweep's
test-data-fixtures#3.19 · not touched · the fetch seed's base URL and JSON body are unchanged, its lines shifted by the 455-488 insert, the sweep's
test-data-fixtures#4.1 · not touched · the chunk adds no fixture file or loader; its new seeds are in-code constants a test helper writes out
mocking-stubbing-discipline#1.1 · not touched · the mutator's redraw-counting fakes are untouched by its two added ranges; the shell's redraw route changed in blitz-shell, not in these fakes
mocking-stubbing-discipline#1.10 · not touched · `MapFetcher` and its NotFound answer stand; the `file:`-load tests are new rows of F4 and F6, and its lines shifted, the sweep's
mocking-stubbing-discipline#3.1 · not touched · `Cargo.toml` is byte-identical and no package entered; `publish-browser.yml`'s 15 added lines are upstream's publish steps
mocking-stubbing-discipline#3.2 · not touched · no package entered, left or moved, so no mocking library reaches the apps; the tab test is a plain unit test
ci-integration#1.1 · proposed · the Legs record carries F3: the new target is in the `test` leg and `run all` only
ci-integration#4.1 · not touched · every manifest is byte-identical, so feature unification stands; no new log site, and `host_log` and `mcp_log` are green under both builds
ci-integration#5.1 · proposed · the per-OS rows move to CI#40487650912 at `ci-integration.md:43` (the report's `ws-tally`)
ci-integration#6.1 · proposed · the local tally moves 208 · 1117 → 209 · 1125 at `ci-integration.md:64` (the report's `ws-tally`)
