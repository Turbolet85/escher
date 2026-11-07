@answer n=1 entity=input-validation#1.126 v=no
why: inline SVG serialization is blitz-dom output the buffered render never touches; the chunk adds no serialization path
@answer n=2 entity=input-validation#1.127 v=no
why: test-harness selector parsing in scoped_query_selector and inspect.rs is not edited, re-run or re-measured by the chunk
@answer n=3 entity=input-validation#1.128 v=no
why: WPT timeout-quarantine input belongs to the WPT runner, which the chunk does not reach
@answer n=4 entity=input-validation#1.129 v=no
why: WPT fuzzy-range parsing belongs to the WPT runner, which the chunk does not reach
@answer n=5 entity=input-validation#1.130 v=no
why: WPT checkLayout attribute parsing belongs to the WPT runner, which the chunk does not reach
@answer n=6 entity=input-validation#1.131 v=no
why: WPT wrapper HTML escaping belongs to the WPT runner, which the chunk does not reach
@answer n=7 entity=input-validation#1.132 v=no
why: WPT script-type classification is not on the Dioxus bridge, which runs no document scripts
@answer n=8 entity=input-validation#1.133 v=no
why: data: URL decoding is the WPT net provider, and the buffered render fetches nothing
@answer n=9 entity=input-validation#1.134 v=no
why: WPT page-message parsing in harness_test.rs is not reached by the chunk
@answer n=10 entity=input-validation#1.135 v=no
why: the PR lookup in wpt-post-results.yml is CI workflow input, and no workflow is edited
@answer n=11 entity=input-validation#1.136 v=no
why: the comment splice in wpt_diff_to_pr.py is a CI script the chunk does not touch
@answer n=12 entity=input-validation#1.137 v=no
why: 7GUIs validation is example app code the chunk does not edit, and its handlers run outside the render path the port changes
@answer n=13 entity=input-validation#1.138 v=no
why: the cells-grid key and focus rules are app event code; set_focus outside render keeps its current behaviour and the chunk edits no example
@answer n=14 entity=input-validation#1.139 v=no
why: the widget color parse and TodoMVC input trimming are example code the chunk does not reach
@answer n=15 entity=input-validation#1.140 v=no
why: the graphite and google fixture form attributes are HTML fixtures the chunk does not reach, and NativeFormData validity is unchanged
