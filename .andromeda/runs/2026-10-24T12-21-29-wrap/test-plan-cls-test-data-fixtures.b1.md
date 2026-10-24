# Classified — test-plan · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · no · the examples/assets pages stay untouched, and the one new example, multi_doc.rs, holds no asset
test-data-fixtures#2.1 · no · the two committed Counter checks stay unmoved, and the replay and recording tests that embed them are not touched
test-data-fixtures#3.1 · no · the inline HTML strategy and its two cited files stay unchanged, and the new hit-test target's direct HtmlDocument construction is an instance of the same strategy
test-data-fixtures#3.2 · no · blitz-vibey-script is not among the session's touched files, so its inline script-test HTML stays unchanged
test-data-fixtures#3.3 · yes · stale · element.rs gains lines above the cited 1082 and 1115-1118 (at 45-48, 56-61, 313, 422-426 and 431), so both citations shift
test-data-fixtures#3.4 · no · util.rs, which holds these byte-string literals, is not among the session's touched files
test-data-fixtures#3.5 · no · fuzzy.rs is not among the WPT runner files the session shifts (main.rs and attr_test.rs), so its inline inputs stay unchanged
test-data-fixtures#3.6 · no · .github/ stays byte-unchanged, so the ENTRIES list stands
test-data-fixtures#3.7 · no · document.rs is not among the session's touched files, so its user-agent stylesheets stand
test-data-fixtures#3.8 · yes · stale · mutator.rs gains lines at 375-378 and 499-502, above the cited 1848, so that citation shifts
test-data-fixtures#3.9 · no · document.rs and examples/seven_guis/src/lib.rs are not among the session's touched files, so the bundled font and its registration stand
test-data-fixtures#3.10 · no · the background-layer injections stay unchanged, and the aspect-ratio change reaches replaced elements, not background layers
test-data-fixtures#3.11 · no · paint_tree_bench.rs and its generated pages stay unchanged, and the new multi_doc example is a manual timing run, not a test benchmark page
test-data-fixtures#3.12 · no · PAINT_TREE_BENCH_HTML still supplies the external page, and the MULTI_DOC_* variables configure the example's own run and supply no page
test-data-fixtures#3.13 · no · the browser helper files are not among the session's touched files, and the mimalloc default changes none of them
test-data-fixtures#3.14 · no · the .test URLs in browser_history.rs and persistence/src/lib.rs stay unchanged, as neither file is touched
test-data-fixtures#3.15 · no · apps/browser/src/favicon.rs is not among the session's touched files, so its in-test PNG stands
test-data-fixtures#3.16 · no · blitz-vibey-script's tests/dom.rs is not among the session's touched files, so its fetch base URL and JSON body stand
test-data-fixtures#4.1 · no · the session adds no fixture file or loader, as the new hit-test target builds its documents directly with no shared fixtures module
