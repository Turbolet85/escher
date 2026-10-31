# Classified — security-plan · input-validation.b5

## Answers
input-validation#1.93 · no · the Dioxus `dangerous_inner_html` rule lives in packages/dioxus-native-dom/src/mutation_writer.rs, which the change does not reach
input-validation#1.94 · no · the attribute-value-type rule lives in packages/dioxus-native-dom/src/mutation_writer.rs:306-308, which the change does not reach
input-validation#1.95 · no · the falsy boolean-attribute clearing lives in packages/dioxus-native-dom/src/mutation_writer.rs:439-475 and its test file, neither reached; the session's author-`open` fact concerns select steering, not this write path, so it is not placed here
input-validation#1.96 · no · the Response status check lives in packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js, outside every reached file
input-validation#1.97 · no · the fetch method check lives in packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js, outside every reached file
input-validation#1.98 · no · the selector SyntaxError lives in packages/blitz-vibey-script/src/dom/document.rs and selectors.rs, neither reached
input-validation#1.99 · no · the scroll-option TypeErrors live in packages/blitz-vibey-script/src/dom/element/scrolling.rs, not reached
input-validation#1.100 · no · the width/height reflection lives in packages/blitz-vibey-script/src/dom/element/attributes.rs, not reached; the session's `size` parse is a Rust Markup read this JS row does not hold
input-validation#1.101 · no · the timer delay handling lives in packages/blitz-vibey-script/src/runtime/timers.rs, not reached
input-validation#1.102 · no · the CSS.registerProperty validation lives in packages/blitz-vibey-script/src/runtime/css.rs, not reached
input-validation#1.103 · no · the setProperty and cssText validation live in packages/blitz-vibey-script/src/dom/style.rs, not reached
input-validation#1.104 · no · the CSS property accessors live in packages/blitz-vibey-script/src/runtime/css.rs and runtime/bootstrap/style.js, not reached
input-validation#1.105 · no · the node-argument TypeErrors live in packages/blitz-vibey-script/src/dom/mod.rs and dom/node.rs, not reached
input-validation#1.106 · no · the geometry.js conversions live in packages/blitz-vibey-script/src/geometry.js, not reached
input-validation#1.107 · no · the selection API lives in packages/blitz-vibey-script/src/dom/selection.rs, not reached; the session writes no text selection
input-validation#1.108 · no · the sqlite params! binding lives in apps/browser/persistence/src/lib.rs, not reached
input-validation#1.109 · no · the stored-URL re-parse lives in apps/browser/persistence/src/lib.rs, not reached
input-validation#1.110 · no · the escaping rule lives in packages/blitz-dom/src/node/serialize.rs, not reached; node/mod.rs is reached but does not hold this rule
input-validation#1.111 · no · the inline SVG rule and its test live in serialize paths and tests/blitz-tests/tests/inline_svg_serialize.rs, neither reached
input-validation#1.112 · no · the selector parse results live in tests/blitz-tests/tests/scoped_query_selector.rs and packages/blitz-test-harness/src/inspect.rs, neither reached
input-validation#1.113 · no · the WPT timeout-quarantine rule lives in wpt/runner, which the session does not touch
input-validation#1.114 · no · the WPT fuzzy-range rule lives in wpt/runner, which the session does not touch
input-validation#1.115 · no · the checkLayout rule lives in wpt/runner/src/test_runners/attr_test.rs, which the session does not touch
input-validation#1.116 · no · the WPT wrapper escaping lives in wpt/runner/src/test_runners/js_wrapper.rs, which the session does not touch
input-validation#1.117 · no · the WPT script-type rule lives in wpt/runner/src/test_runners/mod.rs, which the session does not touch
