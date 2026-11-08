@answer n=1 entity=input-validation#1.126 v=no
why: CSS.registerProperty in blitz-vibey-script is a script API the chunk neither changes nor uses
@answer n=2 entity=input-validation#1.127 v=no
why: style setProperty and cssText live in blitz-vibey-script, and the chunk runs no document script
@answer n=3 entity=input-validation#1.128 v=no
why: CSS property accessors on CSSStyleDeclaration are a script-to-DOM crossing the chunk does not touch
@answer n=4 entity=input-validation#1.129 v=no
why: DOM node argument checks in blitz-vibey-script native methods are not reached by the chunk
@answer n=5 entity=input-validation#1.130 v=no
why: geometry.js conversions are script-side code the chunk does not reach
@answer n=6 entity=input-validation#1.131 v=no
why: URLSearchParams and URL bootstrap scripts are a document script surface the chunk does not reach, adding no script and no navigation
@answer n=7 entity=input-validation#1.132 v=no
why: document selection in blitz-vibey-script is not touched and not reached by the chunk
@answer n=8 entity=input-validation#1.133 v=no
why: SQL value binding in the sqlite persistence crate is not reached, the chunk stores nothing in sqlite
@answer n=9 entity=input-validation#1.134 v=no
why: re-parsing stored URLs on load in persistence is not reached, the chunk stores no URLs
@answer n=10 entity=input-validation#1.135 v=no
why: HTML text and attribute escaping in blitz-dom serialization is not reached, the coverage reply is driver JSON
@answer n=11 entity=input-validation#1.136 v=no
why: inline SVG well-formedness in serialization and blitz-tests is not reached by the chunk
@answer n=12 entity=input-validation#1.137 v=no
why: selector parsing in the blitz-test-harness and blitz-tests suites is not reached, the chunk's tests are escher-driver replay tests
@answer n=13 entity=input-validation#1.138 v=no
why: WPT timeout-quarantine entry checks are in the WPT runner, which the chunk does not reach
@answer n=14 entity=input-validation#1.139 v=no
why: WPT fuzzy range parsing is in the WPT runner, which the chunk does not reach
@answer n=15 entity=input-validation#1.140 v=no
why: WPT checkLayout attribute parsing is in the WPT runner, which the chunk does not reach
@answer n=16 entity=input-validation#1.141 v=no
why: WPT wrapper HTML escaping is in the WPT runner, which the chunk does not reach
@answer n=17 entity=input-validation#1.142 v=no
why: WPT script type classification is in the WPT runner, which the chunk does not reach
@answer n=18 entity=input-validation#1.143 v=no
why: WPT data: URL decoding in the net provider is not reached, the chunk adds no fetch and does not reach the WPT runner
@answer n=19 entity=input-validation#1.144 v=no
why: WPT harness page-message JSON checks are not reached by the chunk
@answer n=20 entity=input-validation#1.145 v=no
why: PR lookup in wpt-post-results.yml is a WPT workflow the chunk does not touch, and it states no cargo or uses-pinning convention
@answer n=21 entity=input-validation#1.146 v=no
why: the comment splice rule belongs to wpt_diff_to_pr.py, a script the chunk does not change
@answer n=22 entity=input-validation#1.147 v=no
why: 7GUIs flight booker, cells and circle drawer validation is in an example app the chunk does not reach
@answer n=23 entity=input-validation#1.148 v=no
why: cells-grid key handling is in the 7GUIs example app, which the chunk does not reach
@answer n=24 entity=input-validation#1.149 v=no
why: widget color parsing and TodoMVC input trimming are in example code the chunk does not reach
@answer n=25 entity=input-validation#1.150 v=no
why: graphite and google fixture form inputs are fixtures the chunk does not touch and forms it does not submit
