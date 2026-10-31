# Classified — security-plan · input-validation.b5

## Answers
input-validation#1.93 · no · the rule lives in packages/dioxus-native-dom/src/mutation_writer.rs:430-431, which is not in the reached list, and the session changes no Dioxus mutation path
input-validation#1.94 · no · the rule lives in packages/dioxus-native-dom/src/mutation_writer.rs:306-308, which is not reached, and the session changes no mutation path
input-validation#1.95 · no · the rule lives in mutation_writer.rs:439-475 and tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs, both unreached; the 28-name list is unmoved and the new `expanded` and `size` reads are not among its names
input-validation#1.96 · no · the rule lives in packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js, which is not reached, and the session touches no script runtime
input-validation#1.97 · no · the rule lives in packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js, which is not reached, and the session touches no script runtime
input-validation#1.98 · no · the rule lives in packages/blitz-vibey-script/src/dom/document.rs and dom/element/selectors.rs, which are not reached
input-validation#1.99 · no · the rule lives in packages/blitz-vibey-script/src/dom/element/scrolling.rs, which is not reached; the session's scroll_into_view change is in Rust BaseDocument, not the JS scroll surface
input-validation#1.100 · no · the rule lives in packages/blitz-vibey-script/src/dom/element/attributes.rs, which is not reached
input-validation#1.101 · no · the rule lives in packages/blitz-vibey-script/src/runtime/timers.rs, which is not reached
input-validation#1.102 · no · the rule lives in packages/blitz-vibey-script/src/runtime/css.rs, which is not reached
input-validation#1.103 · no · the rule lives in packages/blitz-vibey-script/src/dom/style.rs, which is not reached
input-validation#1.104 · no · the rule lives in packages/blitz-vibey-script/src/runtime/bootstrap/style.js and runtime/css.rs, which are not reached
input-validation#1.105 · no · the rule lives in packages/blitz-vibey-script/src/dom/mod.rs and dom/node.rs, which are not reached
input-validation#1.106 · no · the rule lives in packages/blitz-vibey-script/src/geometry.js, which is not reached
input-validation#1.107 · no · the rule lives in packages/blitz-vibey-script/src/dom/selection.rs, which is not reached
input-validation#1.108 · no · the rule lives in apps/browser/persistence/src/lib.rs, which is not reached; the session adds no persistence path
input-validation#1.109 · no · the rule lives in apps/browser/persistence/src/lib.rs, which is not reached; the session adds no persistence path
input-validation#1.110 · no · the rule lives in packages/blitz-dom/src/node/serialize.rs, which is not reached; the list is not in the DOM and no node key is added
input-validation#1.111 · no · the rule lives in tests/blitz-tests/tests/inline_svg_serialize.rs, which is not reached, and no SVG path changed
input-validation#1.112 · no · the rule lives in packages/blitz-test-harness/src/inspect.rs and tests/blitz-tests/tests/scoped_query_selector.rs, which are not reached
input-validation#1.113 · no · the rule lives in wpt/runner/src/test_runners/mod.rs, which is not reached; the WPT runner is untouched
input-validation#1.114 · no · the rule lives in wpt/runner/src/test_runners/fuzzy.rs, which is not reached; the WPT runner is untouched
input-validation#1.115 · no · the rule lives in wpt/runner/src/test_runners/attr_test.rs, which is not reached; the WPT runner is untouched
input-validation#1.116 · no · the rule lives in wpt/runner/src/test_runners/js_wrapper.rs, which is not reached; the WPT runner is untouched
input-validation#1.117 · no · the rule lives in wpt/runner/src/test_runners/mod.rs, which is not reached; the WPT runner is untouched
