@answer n=1 entity=input-validation#1.126 v=no
why: blitz-vibey-script CSS.registerProperty argument checks; the chunk runs no document script and edits no script runtime
@answer n=2 entity=input-validation#1.127 v=no
why: style setProperty and cssText parsing in blitz-vibey-script; the chunk parses no CSS
@answer n=3 entity=input-validation#1.128 v=no
why: named CSSStyleDeclaration accessors in blitz-vibey-script; the chunk's driver and Dioxus code never reach them
@answer n=4 entity=input-validation#1.129 v=no
why: DOM node argument checks in blitz-vibey-script; the chunk calls no script DOM method
@answer n=5 entity=input-validation#1.130 v=no
why: geometry.js conversions in blitz-vibey-script; the chunk reads and writes no script geometry
@answer n=6 entity=input-validation#1.131 v=no
why: URLSearchParams bootstrap in blitz-vibey-script; the chunk neither edits nor reaches the page-script crossing
@answer n=7 entity=input-validation#1.132 v=no
why: document selection API in blitz-vibey-script; the chunk touches no selection code
@answer n=8 entity=input-validation#1.133 v=no
why: sqlite persistence statements; the chunk stores nothing in sqlite
@answer n=9 entity=input-validation#1.134 v=no
why: sqlite-stored URL reload; the chunk persists no URLs
@answer n=10 entity=input-validation#1.135 v=no
why: HTML serialization escaping in blitz-dom; the chunk serializes no markup and its coverage record is not HTML
@answer n=11 entity=input-validation#1.136 v=no
why: inline SVG well-formedness in blitz-dom serialization; the chunk emits no SVG
@answer n=12 entity=input-validation#1.137 v=no
why: blitz-test-harness selector queries; the chunk's owed tests live in escher-driver and use no selector
@answer n=13 entity=input-validation#1.138 v=no
why: WPT runner timeout-quarantine entries; the chunk does not reach the WPT runner
@answer n=14 entity=input-validation#1.139 v=no
why: WPT runner fuzzy-range parsing; the chunk does not reach the WPT runner
@answer n=15 entity=input-validation#1.140 v=no
why: WPT runner checkLayout attribute parsing; the chunk does not reach the WPT runner
@answer n=16 entity=input-validation#1.141 v=no
why: WPT runner wrapper HTML escaping; the chunk does not reach the WPT runner
@answer n=17 entity=input-validation#1.142 v=no
why: WPT runner script-type classification; the chunk runs no document script and does not reach the WPT runner
@answer n=18 entity=input-validation#1.143 v=no
why: WPT runner net-provider data: URL handling; the chunk adds no fetch and does not reach the WPT runner
@answer n=19 entity=input-validation#1.144 v=no
why: WPT harness page-message JSON parsing; the chunk does not reach the WPT runner
@answer n=20 entity=input-validation#1.145 v=no
why: wpt-post-results.yml PR lookup; the chunk touches no publish or WPT workflow
@answer n=21 entity=input-validation#1.146 v=no
why: wpt_diff_to_pr.py comment splice; the chunk edits test_capability_statement.py, not this script
@answer n=22 entity=input-validation#1.147 v=no
why: seven_guis example input validation; the chunk changes no example app
@answer n=23 entity=input-validation#1.148 v=no
why: seven_guis cells-grid key handling; the chunk changes no example app
@answer n=24 entity=input-validation#1.149 v=no
why: custom widget color parsing and TodoMVC input trimming; the chunk changes no example
@answer n=25 entity=input-validation#1.150 v=no
why: example fixture form inputs; the chunk touches no fixture
