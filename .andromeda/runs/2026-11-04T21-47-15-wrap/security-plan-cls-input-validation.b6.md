@answer n=1 entity=input-validation#1.126 v=no
why: its rule lives in inline_svg_serialize.rs, which the session does not reach, and the Cargo.lock change moves only pulldown-cmark, reached only through rdme
@answer n=2 entity=input-validation#1.127 v=no
why: its rule lives in scoped_query_selector.rs and the blitz-test-harness inspect.rs, neither reached by the session, and the lock change does not move them
@answer n=3 entity=input-validation#1.128 v=no
why: its rule lives in the WPT runner's test_runners/mod.rs, which the session does not reach, and the lock change moves no WPT runner dependency
@answer n=4 entity=input-validation#1.129 v=no
why: its rule lives in the WPT runner's fuzzy.rs, which the session does not reach, and the lock change moves no WPT runner dependency
@answer n=5 entity=input-validation#1.130 v=no
why: its rule lives in the WPT runner's attr_test.rs, which the session does not reach, and the lock change moves no WPT runner dependency
@answer n=6 entity=input-validation#1.131 v=no
why: its rule lives in the WPT runner's js_wrapper.rs, which the session does not reach, and the lock change moves no WPT runner dependency
@answer n=7 entity=input-validation#1.132 v=no
why: its rule lives in the WPT runner's test_runners/mod.rs script-type check, which the session does not reach, and the lock change moves no WPT runner dependency
@answer n=8 entity=input-validation#1.133 v=no
why: its rule lives in the WPT runner's net_provider.rs, which the session does not reach, and the lock change moves no WPT runner dependency
@answer n=9 entity=input-validation#1.134 v=no
why: its rule lives in the WPT runner's harness_test.rs, which the session does not reach, and the lock change moves no WPT runner dependency
@answer n=10 entity=input-validation#1.135 v=no
why: its rule lives in .github/workflows/wpt-post-results.yml, which the session does not reach, and the CI audit change touches no workflow step here
@answer n=11 entity=input-validation#1.136 v=no
why: its rule lives in .github/scripts/wpt_diff_to_pr.py, which the session does not reach
@answer n=12 entity=input-validation#1.137 v=no
why: its rule lives in the examples/seven_guis task files, which the session does not reach, and the lock change moves only pulldown-cmark, reached only through rdme
@answer n=13 entity=input-validation#1.138 v=no
why: its rule lives in examples/seven_guis/src/tasks/cells.rs, which the session does not reach
@answer n=14 entity=input-validation#1.139 v=no
why: its rule lives in examples/custom_widget.rs and examples/preact/index.html, which the session does not reach
@answer n=15 entity=input-validation#1.140 v=no
why: its rule lives in the examples/assets graphite.html and google.html fixtures, which the session does not reach
