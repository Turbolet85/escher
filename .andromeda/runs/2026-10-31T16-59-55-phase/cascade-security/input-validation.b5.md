### input-validation#1.93 · values · input · ## Input Validation · lines 321–323
- **Boundary:** Dioxus mutations
  - **What to validate:** `dangerous_inner_html`
  - **How (stack-specific):** Applied with `set_inner_html` (packages/dioxus-native-dom/src/mutation_writer.rs:430-431)
### input-validation#1.94 · values · input · ## Input Validation · lines 324–326
- **Boundary:** Dioxus mutations
  - **What to validate:** Attribute value types
  - **How (stack-specific):** Values of unsupported `AttributeValue` types are ignored (packages/dioxus-native-dom/src/mutation_writer.rs:306-308)
### input-validation#1.95 · values · input · ## Input Validation · lines 327–329
- **Boundary:** Dioxus mutations
  - **What to validate:** Boolean attributes with a falsy value
  - **How (stack-specific):** Removed, not written: a falsy value (`None`, Text `"false"`, Bool false, Int 0, Float 0.0) of one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set (`checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them; the 27 before `inert` the bridge's own `BOOLEAN_ATTRIBUTES` list until 2026-10-13-upstream-sync-polish-and-ship, which retired it for upstream's predicate, #1188, and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`) clears the attribute, on the dynamic and the static template attribute path alike; a truthy value of a listed name, and an attribute outside the list holding `false`, are still written. It is an in-process mutation path: no new input class and no crossing (packages/dioxus-native-dom/src/mutation_writer.rs:439-475; tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:1-9) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md and, for the 28-name predicate, escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md
### input-validation#1.96 · values · input · ## Input Validation · lines 330–332
- **Boundary:** JS API arguments
  - **What to validate:** `Response` status
  - **How (stack-specific):** Throws RangeError for a status outside 200–599 (packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:38-39)
### input-validation#1.97 · values · input · ## Input Validation · lines 333–335
- **Boundary:** JS API arguments
  - **What to validate:** `fetch()` method
  - **How (stack-specific):** Rejects with TypeError for any method other than GET or HEAD (packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:81-83)
### input-validation#1.98 · values · input · ## Input Validation · lines 336–338
- **Boundary:** JS API arguments
  - **What to validate:** Selectors
  - **How (stack-specific):** An unparseable selector raises a `SyntaxError` DOMException (packages/blitz-vibey-script/src/dom/document.rs:357-376; packages/blitz-vibey-script/src/dom/element/selectors.rs:9-17)
### input-validation#1.99 · values · input · ## Input Validation · lines 339–341
- **Boundary:** JS API arguments
  - **What to validate:** Scroll options
  - **How (stack-specific):** `ScrollBehavior` and `ScrollLogicalPosition` values outside their enumerations throw TypeError (packages/blitz-vibey-script/src/dom/element/scrolling.rs:35-52; packages/blitz-vibey-script/src/dom/element/scrolling.rs:206-228); a single non-object `scrollTo` argument throws TypeError (packages/blitz-vibey-script/src/dom/element/scrolling.rs:90-95)
### input-validation#1.100 · values · input · ## Input Validation · lines 342–344
- **Boundary:** JS API arguments
  - **What to validate:** `width`/`height` reflection
  - **How (stack-specific):** Parses non-negative integers and caps at 2147483647, falling back to the default (packages/blitz-vibey-script/src/dom/element/attributes.rs:289-305; packages/blitz-vibey-script/src/dom/element/attributes.rs:343-354)
### input-validation#1.101 · values · input · ## Input Validation · lines 345–347
- **Boundary:** JS API arguments
  - **What to validate:** Timers
  - **How (stack-specific):** Delays that are non-finite or not positive become 0; clear functions ignore non-finite or negative ids (packages/blitz-vibey-script/src/runtime/timers.rs:51-59; packages/blitz-vibey-script/src/runtime/timers.rs:135-137)
### input-validation#1.102 · values · input · ## Input Validation · lines 348–350
- **Boundary:** JS API arguments
  - **What to validate:** `CSS.registerProperty`
  - **How (stack-specific):** Requires a dictionary with `name` and `inherits`, and maps registration failures to SyntaxError or Error (packages/blitz-vibey-script/src/runtime/css.rs:81-131)
### input-validation#1.103 · values · input · ## Input Validation · lines 351–353
- **Boundary:** JS API arguments
  - **What to validate:** Style `setProperty` / `cssText`
  - **How (stack-specific):** `setProperty` ignores invalid declarations; `cssText` assignment re-serializes and drops invalid declarations (packages/blitz-vibey-script/src/dom/style.rs:120-123; packages/blitz-vibey-script/src/dom/style.rs:142-150)
### input-validation#1.104 · values · input · ## Input Validation · lines 354–356
- **Boundary:** JS API arguments
  - **What to validate:** CSS property accessors on `CSSStyleDeclaration`
  - **How (stack-specific):** Script reads and writes CSS properties through named accessors that `installStyleProperties` defines on `CSSStyleDeclaration` (packages/blitz-vibey-script/src/runtime/bootstrap/style.js:26-48), one per name `css_property_names` takes from Stylo's property list (packages/blitz-vibey-script/src/runtime/css.rs:8-18) — a realization on the registered script-to-DOM crossing, not a boundary widening; witnessed by `style_property_accessors` (packages/blitz-vibey-script/tests/dom.rs:495-527) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md
### input-validation#1.105 · values · input · ## Input Validation · lines 357–359
- **Boundary:** JS API arguments
  - **What to validate:** Node arguments
  - **How (stack-specific):** Native methods throw TypeError when `this` or an argument is not a DOM node (packages/blitz-vibey-script/src/dom/mod.rs:53-60; packages/blitz-vibey-script/src/dom/node.rs:284-290); `replaceChild` checks for `HierarchyRequestError` and `NotFoundError` (packages/blitz-vibey-script/src/dom/node.rs:362-382)
### input-validation#1.106 · values · input · ## Input Validation · lines 360–362
- **Boundary:** JS API arguments
  - **What to validate:** geometry.js conversions
  - **How (stack-specific):** Rejects BigInt in number conversion and non-object dictionaries with TypeError (packages/blitz-vibey-script/src/geometry.js:27-46)
### input-validation#1.107 · values · input · ## Input Validation · lines 363–365
- **Boundary:** JS API arguments
  - **What to validate:** Selection (`document.getSelection`)
  - **How (stack-specific):** `setBaseAndExtent` checks its argument count and its offsets and throws a JS error on a bad call, leaving the selection unchanged; the API writes the document's text selection, and the `innerText` / `outerText` getters that arrived beside it only read; upstream's, and escher uses none of them (packages/blitz-vibey-script/src/dom/selection.rs:69-119; packages/blitz-vibey-script/tests/selection.rs:1-266) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
### input-validation#1.108 · values · input · ## Input Validation · lines 366–368
- **Boundary:** Persistence (sqlite)
  - **What to validate:** SQL values
  - **How (stack-specific):** Statements bind values through `params!` placeholders (apps/browser/persistence/src/lib.rs:165; apps/browser/persistence/src/lib.rs:231-243; apps/browser/persistence/src/lib.rs:249-256; apps/browser/persistence/src/lib.rs:268-271)
### input-validation#1.109 · values · input · ## Input Validation · lines 369–371
- **Boundary:** Persistence (sqlite)
  - **What to validate:** Stored URLs
  - **How (stack-specific):** Re-parsed on load; unparsable rows are skipped (apps/browser/persistence/src/lib.rs:184-188)
### input-validation#1.110 · values · input · ## Input Validation · lines 372–374
- **Boundary:** Serialization output
  - **What to validate:** Escaping
  - **How (stack-specific):** Text escaped with `encode_text_to_string` and attribute values with `encode_quoted_attribute_to_string`, but text inside `style`, `script`, `xmp`, `iframe`, `noembed`, `noframes` and `plaintext` is written unescaped (packages/blitz-dom/src/node/serialize.rs:138-159; packages/blitz-dom/src/node/serialize.rs:182-196)
### input-validation#1.111 · values · input · ## Input Validation · lines 375–377
- **Boundary:** Serialization output
  - **What to validate:** Inline SVG
  - **How (stack-specific):** Must be well-formed XML: text with `&amp;`/`&lt;` and `xlink:href` must still parse (tests/blitz-tests/tests/inline_svg_serialize.rs:1-2; tests/blitz-tests/tests/inline_svg_serialize.rs:40-58)
### input-validation#1.112 · values · input · ## Input Validation · lines 378–380
- **Boundary:** Test harness queries
  - **What to validate:** Selectors
  - **How (stack-specific):** Selector parsing returns a `Result` (`try_parse_selector_list`, `query_selector(...).unwrap()`) (tests/blitz-tests/tests/scoped_query_selector.rs:32-35; tests/blitz-tests/tests/scoped_query_selector.rs:165); the harness query helper panics with "invalid selector" on a selector that fails to parse (packages/blitz-test-harness/src/inspect.rs:27-34); scoped queries with a text or comment node as scope return no matches (tests/blitz-tests/tests/scoped_query_selector.rs:126-149)
### input-validation#1.113 · values · input · ## Input Validation · lines 381–383
- **Boundary:** WPT runner
  - **What to validate:** Timeout-quarantine entries
  - **How (stack-specific):** Must contain a path and reason and must not duplicate (panics otherwise) (wpt/runner/src/test_runners/mod.rs:33-41); a unit test checks paths are non-empty without whitespace and reasons are in `KNOWN_REASONS` (wpt/runner/src/test_runners/mod.rs:406-441)
### input-validation#1.114 · values · input · ## Input Validation · lines 384–386
- **Boundary:** WPT runner
  - **What to validate:** Fuzzy ranges
  - **How (stack-specific):** Must parse as integers with `min <= max`, else the spec is dropped (wpt/runner/src/test_runners/fuzzy.rs:25-35; wpt/runner/src/test_runners/fuzzy.rs:90-96)
### input-validation#1.115 · values · input · ## Input Validation · lines 387–389
- **Boundary:** WPT runner
  - **What to validate:** checkLayout
  - **How (stack-specific):** Attribute values that do not parse as f32 produce a subtest error (wpt/runner/src/test_runners/attr_test.rs:248-253); an unparseable selector panics (wpt/runner/src/test_runners/attr_test.rs:79-81)
### input-validation#1.116 · values · input · ## Input Validation · lines 390–392
- **Boundary:** WPT runner
  - **What to validate:** Wrapper HTML
  - **How (stack-specific):** Escapes META `title` as text and `script` as a double-quoted attribute (wpt/runner/src/test_runners/js_wrapper.rs:63-74)
### input-validation#1.117 · values · input · ## Input Validation · lines 393–395
- **Boundary:** WPT runner
  - **What to validate:** Script type
  - **How (stack-specific):** Only `<script>` types "", text/javascript, application/javascript and module count as JavaScript (wpt/runner/src/test_runners/mod.rs:53-63)
