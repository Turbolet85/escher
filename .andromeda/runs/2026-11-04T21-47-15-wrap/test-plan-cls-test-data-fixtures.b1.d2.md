@answer n=1 entity=test-data-fixtures#1.1 v=no
why: the session reached only dioxus_runtime_probe.rs, Cargo.lock and deny.toml, and this row cites examples/assets pages it did not touch
@answer n=2 entity=test-data-fixtures#2.1 v=no
why: this row cites the committed checks counter-cli.json and counter-mcp.json, which the session did not change
@answer n=3 entity=test-data-fixtures#3.1 v=no
why: this row cites the inline HTML in incremental_oracle.rs and dir_attribute.rs, which the session did not reach
@answer n=4 entity=test-data-fixtures#3.2 v=no
why: this row cites select_popup.rs hostile_html, a file the session did not reach
@answer n=5 entity=test-data-fixtures#3.3 v=no
why: this row cites fixtures/mod.rs many_items, a file the session did not reach
@answer n=6 entity=test-data-fixtures#3.4 v=yes mode=add-to
why: the session keeps dioxus_runtime_probe's one-reader pages toggled_listener, shared_state and hazard each in its own file, the same per-file pattern this row lists (F5)
@answer n=7 entity=test-data-fixtures#3.5 v=no
why: this row cites key_press_after_container_lost_its_box.rs, a file the session did not reach
@answer n=8 entity=test-data-fixtures#3.6 v=no
why: this row cites stand_cells_served.rs and the seven_guis stand, which the session did not reach
@answer n=9 entity=test-data-fixtures#3.7 v=no
why: this row cites the seven_guis shared test module common/mod.rs, which the session did not reach
@answer n=10 entity=test-data-fixtures#3.8 v=no
why: this row cites blitz-vibey-script dom.rs, which the session did not reach
@answer n=11 entity=test-data-fixtures#3.9 v=no
why: this row cites temp_dir_with_files in blitz-vibey-script dom.rs, which the session did not reach
@answer n=12 entity=test-data-fixtures#3.10 v=no
why: this row cites blitz-vibey-script url_search_params.rs, which the session did not reach
@answer n=13 entity=test-data-fixtures#3.11 v=no
why: this row cites html_sink.rs and element.rs inline literals, which the session did not reach
@answer n=14 entity=test-data-fixtures#3.12 v=no
why: this row cites blitz-dom util.rs SVG byte literals, which the session did not reach
@answer n=15 entity=test-data-fixtures#3.13 v=no
why: this row cites the WPT fuzzy runner inputs in fuzzy.rs, which the session did not reach
@answer n=16 entity=test-data-fixtures#3.14 v=no
why: this row cites .github/scripts/test_wpt_diff_to_pr.py, which the session did not change
@answer n=17 entity=test-data-fixtures#3.15 v=no
why: this row cites test_capability_statement.py and the repository files it pins, none of which the session changed
@answer n=18 entity=test-data-fixtures#3.16 v=no
why: this row cites blitz-dom document.rs user-agent stylesheets, which the session did not reach
@answer n=19 entity=test-data-fixtures#3.17 v=no
why: this row cites blitz-dom mutator.rs restyle trees, which the session did not reach
@answer n=20 entity=test-data-fixtures#3.18 v=no
why: this row cites the fixed viewports in mutator.rs and document.rs, which the session did not reach
@answer n=21 entity=test-data-fixtures#3.19 v=no
why: this row cites the blitz-dom document.rs font and seven_guis lib.rs DejaVu payload, which the session did not reach
@answer n=22 entity=test-data-fixtures#3.20 v=no
why: this row cites background_size.rs and svg_background_size.rs image loads, which the session did not reach
@answer n=23 entity=test-data-fixtures#3.21 v=no
why: this row cites paint_tree_bench.rs generated pages, which the session did not reach
@answer n=24 entity=test-data-fixtures#3.22 v=no
why: this row cites the external benchmark page in paint_tree_bench.rs, which the session did not reach
@answer n=25 entity=test-data-fixtures#3.23 v=no
why: this row cites apps/browser test helpers, which the session did not reach
