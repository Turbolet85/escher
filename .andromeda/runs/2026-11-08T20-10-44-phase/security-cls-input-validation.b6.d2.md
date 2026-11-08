@answer n=1 entity=input-validation#1.126 v=no
why: CSS.registerProperty is blitz-vibey-script surface that the chunk neither edits nor drives, and its not_needed item says it runs no document script
@answer n=2 entity=input-validation#1.127 v=no
why: setProperty and cssText handling is blitz-vibey-script style code the chunk does not reach
@answer n=3 entity=input-validation#1.128 v=no
why: CSS property accessors on CSSStyleDeclaration are script-side and unreached by the chunk's code or tests
@answer n=4 entity=input-validation#1.129 v=no
why: native DOM node argument checks in blitz-vibey-script are not reached, the chunk runs no script
@answer n=5 entity=input-validation#1.130 v=no
why: geometry.js conversions are script-side code outside the chunk's files and its no-script scope
@answer n=6 entity=input-validation#1.131 v=no
why: URLSearchParams is a page-script surface the chunk does not touch and adds no navigation or fetch
@answer n=7 entity=input-validation#1.132 v=no
why: document selection handling in blitz-vibey-script is not reached by the chunk
@answer n=8 entity=input-validation#1.133 v=no
why: sqlite parameter binding in apps/browser persistence is not reached, the chunk stores nothing in sqlite
@answer n=9 entity=input-validation#1.134 v=no
why: re-parsing of stored URLs in browser persistence is not reached by the chunk
@answer n=10 entity=input-validation#1.135 v=no
why: text and attribute escaping in blitz-dom serialization is not edited or read by the chunk
@answer n=11 entity=input-validation#1.136 v=no
why: inline SVG well-formedness is an engine serialization rule outside the chunk's reach
@answer n=12 entity=input-validation#1.137 v=no
why: selector parsing in blitz-test-harness and blitz-tests is not reached by the chunk's tests
@answer n=13 entity=input-validation#1.138 v=no
why: WPT runner quarantine entries are outside the chunk, which does not reach the WPT runner
@answer n=14 entity=input-validation#1.139 v=no
why: WPT fuzzy range parsing belongs to the WPT runner, which the chunk does not reach
@answer n=15 entity=input-validation#1.140 v=no
why: WPT checkLayout attribute parsing belongs to the WPT runner, which the chunk does not reach
@answer n=16 entity=input-validation#1.141 v=no
why: WPT wrapper HTML escaping belongs to the WPT runner, which the chunk does not reach
@answer n=17 entity=input-validation#1.142 v=no
why: WPT script-type classification belongs to the WPT runner, which the chunk does not reach
@answer n=18 entity=input-validation#1.143 v=no
why: WPT net provider data: URL error handling is the runner's, unreached, and the chunk adds no data: URL handling
@answer n=19 entity=input-validation#1.144 v=no
why: WPT page-message JSON parsing belongs to the WPT runner, which the chunk does not reach
@answer n=20 entity=input-validation#1.145 v=no
why: the wpt-post-results.yml PR lookup is a WPT workflow the chunk does not touch
@answer n=21 entity=input-validation#1.146 v=no
why: the comment splice in wpt_diff_to_pr.py is a WPT publish script the chunk does not name or reach
@answer n=22 entity=input-validation#1.147 v=no
why: 7GUIs example validation is in example code the chunk neither changes nor drives
@answer n=23 entity=input-validation#1.148 v=no
why: cells-grid key handling is an example app handler the chunk does not drive or change
@answer n=24 entity=input-validation#1.149 v=no
why: widget color parsing and TodoMVC input trimming are example code the chunk does not reach
@answer n=25 entity=input-validation#1.150 v=no
why: graphite and google fixture form inputs are fixture pages the chunk does not touch, and no password or file input masking is stated
