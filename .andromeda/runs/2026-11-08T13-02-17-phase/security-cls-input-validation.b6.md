@answer n=1 entity=input-validation#1.126 v=no
why: geometry.js argument conversion in blitz-vibey-script, a JS API the read never calls or touches
@answer n=2 entity=input-validation#1.127 v=no
why: URLSearchParams bootstrap in the engine's script runtime; the read uses no such API and changes no engine code
@answer n=3 entity=input-validation#1.128 v=no
why: document.getSelection argument checks in blitz-vibey-script; the read writes no selection and reaches no such call
@answer n=4 entity=input-validation#1.129 v=no
why: sqlite placeholder binding in the browser app persistence, outside the driver read's code and tests
@answer n=5 entity=input-validation#1.130 v=no
why: stored-URL re-parse in the browser app persistence, outside the driver read's code and tests
@answer n=6 entity=input-validation#1.131 v=no
why: text and attribute escaping in blitz-dom DOM serialization, which the read never runs; the driver's JSON written form is a different path
@answer n=7 entity=input-validation#1.132 v=no
why: inline SVG well-formedness in blitz-dom serialization tests, which the chunk does not reach
@answer n=8 entity=input-validation#1.133 v=no
why: the blitz-test-harness selector panic is that harness's own helper behaviour; the scope names no selector query and the read is proven through the driver, not by selectors
@answer n=9 entity=input-validation#1.134 v=no
why: WPT runner timeout-quarantine entries, the WPT runner being a not-needed area of this task
@answer n=10 entity=input-validation#1.135 v=no
why: WPT runner fuzzy-range parsing, a not-needed area outside the read's code
@answer n=11 entity=input-validation#1.136 v=no
why: WPT runner checkLayout attribute parsing, a not-needed area outside the read's code
@answer n=12 entity=input-validation#1.137 v=no
why: WPT runner wrapper HTML escaping, a not-needed area outside the read's code
@answer n=13 entity=input-validation#1.138 v=no
why: WPT runner script-type classification, a not-needed area outside the read's code
@answer n=14 entity=input-validation#1.139 v=no
why: WPT net provider data: URL decoding, a not-needed area and the read loads nothing
@answer n=15 entity=input-validation#1.140 v=no
why: WPT harness page-message parsing, a not-needed area outside the read's code
@answer n=16 entity=input-validation#1.141 v=no
why: wpt-post-results workflow PR lookup, a CI area the task lists as not needed
@answer n=17 entity=input-validation#1.142 v=no
why: comment splice in the WPT diff-to-PR CI script, which the chunk does not change or run
@answer n=18 entity=input-validation#1.143 v=no
why: 7GUIs example-app input validation, outside the driver read and the dioxus-native-dom readers it touches
@answer n=19 entity=input-validation#1.144 v=no
why: 7GUIs cells-grid key handling in an example app, which the chunk does not use or change
@answer n=20 entity=input-validation#1.145 v=no
why: widget color parsing and TodoMVC input trimming in the examples, which the chunk does not reach
@answer n=21 entity=input-validation#1.146 v=no
why: graphite and google example fixture form attributes, none of which the scope names; the chunk's minimal fixtures are new and not these assets
