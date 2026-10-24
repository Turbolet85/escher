# Classified — tests · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · no · the human-read example pages under examples/assets, which the task excludes from the sweep
test-data-fixtures#2.1 · yes · the two committed Counter checks that the 0.2.0 session-recording witness replays and keeps fresh byte for byte, with no screenshot step
test-data-fixtures#3.1 · yes · the new lint and text-extent fixture pages are inline HTML string constants or format!-built pages, the form the task's nothing-seeded rule requires
test-data-fixtures#3.2 · no · the inline script tests of blitz-vibey-script, which the task excludes as not needed
test-data-fixtures#3.3 · yes · the sweep's fixtures are inline string literals in code, the form the task's Needs require for test data
test-data-fixtures#3.4 · no · SVG parsing byte-string literals in blitz-dom's util.rs unit tests, which the task excludes and the chunk does not reach
test-data-fixtures#3.5 · no · the WPT runner's fuzzy inputs, which the task excludes
test-data-fixtures#3.6 · no · the inline ENTRIES of the WPT diff script's tests, which the task excludes
test-data-fixtures#3.7 · no · user-agent stylesheets injected in document.rs unit tests, which the chunk does not inject and document.rs is not reached
test-data-fixtures#3.8 · yes · the sweep runs on the stand's pinned 800x600 viewport at scale 1.0 (seven_guis::stand::options), the viewport the task names
test-data-fixtures#3.9 · yes · the bundled DejaVu Sans that the headless stand registers for every generic, which the task says binds every new fixture check that reads text
test-data-fixtures#3.10 · no · background-size's images injected into background layers, a test path the chunk does not reach; its loads run through HeldNet and inject no image
test-data-fixtures#3.11 · no · generated benchmark pages of paint_tree_bench, which the task excludes
test-data-fixtures#3.12 · no · the external benchmark page behind PAINT_TREE_BENCH_HTML, which the task excludes
test-data-fixtures#3.13 · no · browser entry helpers of apps/browser's tests, which the task excludes
test-data-fixtures#3.14 · no · the .test URLs of apps/browser's tests, which the task excludes; the task names no URL for the chunk's offline fixtures
test-data-fixtures#3.15 · no · the in-test favicon PNG of apps/browser's tests, which the task excludes
test-data-fixtures#3.16 · no · the fetch base URL and JSON body of blitz-vibey-script's tests, which the task excludes
test-data-fixtures#4.1 · no · an absence of fixture files and loaders in slice s08, a slice no reached file names; the chunk's fixture loader fixtures::boot is bound by the task's Needs
