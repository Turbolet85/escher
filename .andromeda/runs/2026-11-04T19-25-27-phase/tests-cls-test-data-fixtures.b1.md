@answer n=1 entity=test-data-fixtures#1.1 v=no
why: examples/assets CSS and real-world pages; the probe runs on seven_guis components and reads no such page
@answer n=2 entity=test-data-fixtures#2.1 v=no
why: the committed counter-cli and counter-mcp recordings; the probe records no session
@answer n=3 entity=test-data-fixtures#3.1 v=no
why: inline HTML constants of the incremental_oracle and dir_attribute checks; the probe builds from rsx components, not inline markup
@answer n=4 entity=test-data-fixtures#3.2 v=no
why: hostile select markup of select_popup; the probe reads no select
@answer n=5 entity=test-data-fixtures#3.3 v=no standing=S3
why: shared fixture page home, tests/fixtures/mod.rs; the probe adds no shared page and reads none of these
@answer n=6 entity=test-data-fixtures#3.4 v=no standing=S3
why: single-reader select list pages stay in their check's own file; the probe adds no page
@answer n=7 entity=test-data-fixtures#3.5 v=no standing=S3
why: single-reader lost-box pages stay in their check's own file; the probe adds no page
@answer n=8 entity=test-data-fixtures#3.6 v=yes
why: the probe boots the served Cells screen through boot_cells, and its size against the snapshot budget bounds any read of it
@answer n=9 entity=test-data-fixtures#3.7 v=no
why: option_needles reads a select's options from a snapshot; the probe reads no select
@answer n=10 entity=test-data-fixtures#3.8 v=no
why: inline HTML of the blitz-vibey-script tests; the script engine is not reached
@answer n=11 entity=test-data-fixtures#3.9 v=no
why: temp-dir sibling files of the script file-load tests; the probe writes no file
@answer n=12 entity=test-data-fixtures#3.10 v=no
why: URLSearchParams script pages of blitz-vibey-script; the script engine is not reached
@answer n=13 entity=test-data-fixtures#3.11 v=no
why: inline literals in blitz-html and blitz-dom sources; no parsed-markup read is made here
@answer n=14 entity=test-data-fixtures#3.12 v=no
why: SVG byte-string inputs of blitz-dom util; no SVG is read
@answer n=15 entity=test-data-fixtures#3.13 v=no
why: fuzzy runner inputs of the WPT runner, which the chunk does not reach
@answer n=16 entity=test-data-fixtures#3.14 v=no
why: Python CI script entries; no script changes
@answer n=17 entity=test-data-fixtures#3.15 v=no
why: CI script pins read repository text; the chunk adds no verb, capability or docs change
@answer n=18 entity=test-data-fixtures#3.16 v=no
why: user-agent stylesheets of blitz-dom unit tests; engine unit tests are not this chunk's
@answer n=19 entity=test-data-fixtures#3.17 v=no
why: restyle trees of the blitz-dom mutator unit tests; no incremental snapshot is read here
@answer n=20 entity=test-data-fixtures#3.18 v=no
why: the 800x600 and 400x300 sizes are blitz-dom unit-test viewports; the scope names no viewport for the stand
@answer n=21 entity=test-data-fixtures#3.19 v=yes
why: scope item 10 names the bundled DejaVu Sans that the headless stand registers, the font every probe measurement runs under
@answer n=22 entity=test-data-fixtures#3.20 v=no
why: background-image injections of the background_size tests; no image load is read here
@answer n=23 entity=test-data-fixtures#3.21 v=no
why: generated benchmark pages of paint_tree_bench, which is not_needed; no runtime exposure is read
@answer n=24 entity=test-data-fixtures#3.22 v=no
why: external paint-tree benchmark page; benchmarks are outside the chunk
@answer n=25 entity=test-data-fixtures#3.23 v=no
why: browser entry helpers; the chunk reaches no browser crate
