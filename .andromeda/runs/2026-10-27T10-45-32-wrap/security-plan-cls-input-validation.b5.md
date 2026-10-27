# Classified — security-plan · input-validation.b5

## Answers
input-validation#1.90 · yes · stale · packages/blitz-vibey-script/src/runtime.rs, a reached merge surface, is 151 lines after its code moved into the runtime/ modules, so the cited span 2263-2313 lies past its end
input-validation#1.91 · yes · add-to · F4, the named CSSStyleDeclaration accessors (runtime/css.rs css_property_names, bootstrap/style.js installStyleProperties), is a new write path beside setProperty and cssText, whose dom/style.rs stands unreached
input-validation#1.92 · no · packages/blitz-vibey-script/src/dom/mod.rs and dom/node.rs are not in the reached files, and the change adds no node-method argument crossing
input-validation#1.93 · no · packages/blitz-vibey-script/src/geometry.js is not in the reached files (dom/element/geometry.rs is a different file), and its BigInt and dictionary conversions are unchanged
input-validation#1.94 · no · packages/blitz-vibey-script/src/dom/selection.rs and tests/selection.rs are not in the reached files, and no touch names the selection API or innerText
input-validation#1.95 · no · apps/browser/persistence/src/lib.rs is not in the reached files, and tab.rs's history skip removes a write without adding a bound value
input-validation#1.96 · no · apps/browser/persistence/src/lib.rs is not in the reached files, and the re-parse of stored URLs on load is unchanged
input-validation#1.97 · no · packages/blitz-dom/src/node/serialize.rs is not in the reached files (node/element.rs is a merge surface, but this entity cites no line of it), and no touch names a serialization change
input-validation#1.98 · no · tests/blitz-tests/tests/inline_svg_serialize.rs is not in the reached files, and no touch names inline SVG or serialization
input-validation#1.99 · no · tests/blitz-tests/tests/scoped_query_selector.rs and packages/blitz-test-harness/src/inspect.rs are not in the reached files, and :open matching changes neither the parse Result nor the panic path cited
input-validation#1.100 · no · wpt/runner/src/test_runners/mod.rs is not in the reached files (only wpt/WPT_COMMIT is), and the entry checks it states do not depend on the pinned commit
input-validation#1.101 · no · wpt/runner/src/test_runners/fuzzy.rs is not in the reached files, and its integer range check does not depend on the pinned commit
input-validation#1.102 · no · wpt/runner/src/test_runners/attr_test.rs is not in the reached files, and its f32 parse and selector panic are unchanged
input-validation#1.103 · no · wpt/runner/src/test_runners/js_wrapper.rs is not in the reached files, and its wrapper escaping is unchanged
input-validation#1.104 · no · wpt/runner/src/test_runners/mod.rs is not in the reached files, and the script types it counts as JavaScript are unchanged
input-validation#1.105 · no · wpt/runner/src/net_provider.rs is not in the reached files (the task's Untouched list holds the WPT runner unreached), and its data: decoding is unchanged
input-validation#1.106 · no · wpt/runner/src/test_runners/harness_test.rs is not in the reached files, and its page-message JSON check is unchanged
input-validation#1.107 · no · .github/workflows/wpt-post-results.yml is not in the reached files (publish-browser.yml is the only workflow reached), and its PR lookup is unchanged
input-validation#1.108 · no · .github/scripts/wpt_diff_to_pr.py is not in the reached files, and its comment splice is unchanged
input-validation#1.109 · no · examples/seven_guis/src/tasks files are not in the reached files, and no touch names the 7GUIs validators or the flight booker mode select
input-validation#1.110 · no · examples/custom_widget.rs and examples/preact/index.html are not in the reached files, and no touch names parse_color or the TodoMVC input trim
input-validation#1.111 · no · examples/assets/graphite.html and google.html are not in the reached files, and no touch names the fixture form attributes
