@answer n=1 entity=input-validation#1.126 v=no
why: geometry.js conversions are blitz-vibey-script JS API argument checks, a boundary the read never crosses
@answer n=2 entity=input-validation#1.127 v=no
why: URLSearchParams is a page-script bootstrap API in blitz-vibey-script, and the read runs no script
@answer n=3 entity=input-validation#1.128 v=no
why: document selection is a page-script DOM API in blitz-vibey-script, and the read changes no selection
@answer n=4 entity=input-validation#1.129 v=no
why: sqlite statement binding belongs to the browser app persistence, which the chunk does not touch
@answer n=5 entity=input-validation#1.130 v=no
why: re-parsing stored URLs on load is browser persistence, and the read keeps no stored history
@answer n=6 entity=input-validation#1.131 v=no
why: text and attribute escaping in blitz-dom serialization is engine markup output, and the read's answer is escher-driver JSON of ids, verbs and cause words
@answer n=7 entity=input-validation#1.132 v=no
why: inline SVG serialization is engine markup output, and the read writes no markup
@answer n=8 entity=input-validation#1.133 v=no
why: selector parsing and harness panics in blitz-test-harness are not on the read's path, which addresses nodes by stable id and not by selector
@answer n=9 entity=input-validation#1.134 v=no
why: WPT timeout-quarantine entries belong to the WPT runner, which the chunk does not reach
@answer n=10 entity=input-validation#1.135 v=no
why: WPT fuzzy-range parsing belongs to the WPT runner, which the chunk does not reach
@answer n=11 entity=input-validation#1.136 v=no
why: WPT checkLayout attribute parsing belongs to the WPT runner, which the chunk does not reach
@answer n=12 entity=input-validation#1.137 v=no
why: WPT wrapper HTML escaping belongs to the WPT runner, which the chunk does not reach
@answer n=13 entity=input-validation#1.138 v=no
why: WPT script-type classification belongs to the WPT runner, which the chunk does not reach
@answer n=14 entity=input-validation#1.139 v=no
why: WPT data: URL decoding is in the WPT net provider, and the read loads nothing
@answer n=15 entity=input-validation#1.140 v=no
why: WPT page-message parsing belongs to the WPT runner, which the chunk does not reach
@answer n=16 entity=input-validation#1.141 v=no
why: the wpt-post-results PR lookup is a CI workflow the chunk does not touch
@answer n=17 entity=input-validation#1.142 v=no
why: the comment splice in wpt_diff_to_pr.py is a CI script outside the chunk's file list
@answer n=18 entity=input-validation#1.143 v=no
why: the 7GUIs flight booker and cells validation are example apps the chunk does not reach
@answer n=19 entity=input-validation#1.144 v=no
why: the 7GUIs cells-grid key handling is an example app's handler, which the read never runs
@answer n=20 entity=input-validation#1.145 v=no
why: the widget color parse and TodoMVC input trimming are example apps the chunk does not reach
@answer n=21 entity=input-validation#1.146 v=no
why: the graphite and google fixture form attributes are existing example assets, and the chunk's own minimal fixtures carry no control value
