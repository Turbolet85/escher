# Classified — security · input-validation.b5

## Answers
input-validation#1.90 · yes · its rule lives in runtime.rs, a conflict surface the e6f4615ec split moves, and the JS API argument need names these registration checks
input-validation#1.91 · yes · the scope names cssText assignment never throws and efdc5472a adds CSSStyleDeclaration property accessors beside it, so the named behaviour binds though style.rs is unreached
input-validation#1.92 · yes · the JS API argument need names the native APIs' argument checks, which these TypeError and replaceChild checks are
input-validation#1.93 · no · geometry.js is a JS shim the chunk does not reach, and the need names the native APIs' checks rather than its dictionary conversions
input-validation#1.94 · yes · the need names the native APIs' argument checks, and setBaseAndExtent's count and offset checks are one of them though selection.rs is unreached
input-validation#1.95 · no · apps/browser/persistence is not reached and the scope names no SQL or history change
input-validation#1.96 · no · the stored-URL reload sits in apps/browser/persistence, which the chunk does not reach, and no scope item names it
input-validation#1.97 · no · blitz-dom node/serialize.rs is not reached and the scope names no serialization change
input-validation#1.98 · no · inline_svg_serialize.rs is not reached and the scope names no SVG change
input-validation#1.99 · no · inspect.rs and scoped_query_selector.rs are not reached, and the chunk's new upstream tests use the harness as every test does
input-validation#1.100 · no · the WPT runner is not reached (WPT_COMMIT moves only the pin) and the scope names no runner change
input-validation#1.101 · no · fuzzy.rs is not reached and the WPT pin does not change how ranges parse
input-validation#1.102 · no · attr_test.rs is not reached and the scope names no runner change
input-validation#1.103 · no · js_wrapper.rs is not reached and the scope names no runner change
input-validation#1.104 · no · the runner's script-type filter in test_runners/mod.rs is not reached
input-validation#1.105 · no · the runner's net_provider.rs is not reached, and the ScriptFetcher narrowing is a different component
input-validation#1.106 · no · harness_test.rs is not reached and the scope names no runner change
input-validation#1.107 · no · wpt-post-results.yml is not reached, and the chunk's CI edits are the coverage-leg lines and the blitz-tests target pin
input-validation#1.108 · no · wpt_diff_to_pr.py is not reached and no CI edit touches the comment splice
input-validation#1.109 · yes · the stand checks the chunk re-asserts read the flight booker's date validation and its select#flight-type match in both layout modes, and the select model is a named merge surface
input-validation#1.110 · no · custom_widget.rs and the Preact TodoMVC page are not reached and sit outside the stand
input-validation#1.111 · no · the graphite and google fixtures are not reached and the stand does not serve them
