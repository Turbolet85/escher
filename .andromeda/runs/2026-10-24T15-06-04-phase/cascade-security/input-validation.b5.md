### input-validation#1.90 · values · input · ## Input Validation · lines 312–314
- **Boundary:** JS API arguments
  - **What to validate:** `CSS.registerProperty`
  - **How (stack-specific):** Requires a dictionary with `name` and `inherits`, and maps registration failures to SyntaxError or Error (packages/blitz-vibey-script/src/runtime.rs:2263-2313)
### input-validation#1.91 · values · input · ## Input Validation · lines 315–317
- **Boundary:** JS API arguments
  - **What to validate:** Style `setProperty` / `cssText`
  - **How (stack-specific):** `setProperty` ignores invalid declarations; `cssText` assignment re-serializes and drops invalid declarations (packages/blitz-vibey-script/src/dom/style.rs:120-123; packages/blitz-vibey-script/src/dom/style.rs:142-150)
### input-validation#1.92 · values · input · ## Input Validation · lines 318–320
- **Boundary:** JS API arguments
  - **What to validate:** Node arguments
  - **How (stack-specific):** Native methods throw TypeError when `this` or an argument is not a DOM node (packages/blitz-vibey-script/src/dom/mod.rs:53-60; packages/blitz-vibey-script/src/dom/node.rs:284-290); `replaceChild` checks for `HierarchyRequestError` and `NotFoundError` (packages/blitz-vibey-script/src/dom/node.rs:362-382)
### input-validation#1.93 · values · input · ## Input Validation · lines 321–323
- **Boundary:** JS API arguments
  - **What to validate:** geometry.js conversions
  - **How (stack-specific):** Rejects BigInt in number conversion and non-object dictionaries with TypeError (packages/blitz-vibey-script/src/geometry.js:27-46)
### input-validation#1.94 · values · input · ## Input Validation · lines 324–326
- **Boundary:** JS API arguments
  - **What to validate:** Selection (`document.getSelection`)
  - **How (stack-specific):** `setBaseAndExtent` checks its argument count and its offsets and throws a JS error on a bad call, leaving the selection unchanged; the API writes the document's text selection, and the `innerText` / `outerText` getters that arrived beside it only read; upstream's, and escher uses none of them (packages/blitz-vibey-script/src/dom/selection.rs:69-119; packages/blitz-vibey-script/tests/selection.rs:1-266) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
### input-validation#1.95 · values · input · ## Input Validation · lines 327–329
- **Boundary:** Persistence (sqlite)
  - **What to validate:** SQL values
  - **How (stack-specific):** Statements bind values through `params!` placeholders (apps/browser/persistence/src/lib.rs:165; apps/browser/persistence/src/lib.rs:231-243; apps/browser/persistence/src/lib.rs:249-256; apps/browser/persistence/src/lib.rs:268-271)
### input-validation#1.96 · values · input · ## Input Validation · lines 330–332
- **Boundary:** Persistence (sqlite)
  - **What to validate:** Stored URLs
  - **How (stack-specific):** Re-parsed on load; unparsable rows are skipped (apps/browser/persistence/src/lib.rs:184-188)
### input-validation#1.97 · values · input · ## Input Validation · lines 333–335
- **Boundary:** Serialization output
  - **What to validate:** Escaping
  - **How (stack-specific):** Text escaped with `encode_text_to_string` and attribute values with `encode_quoted_attribute_to_string`, but text inside `style`, `script`, `xmp`, `iframe`, `noembed`, `noframes` and `plaintext` is written unescaped (packages/blitz-dom/src/node/serialize.rs:138-159; packages/blitz-dom/src/node/serialize.rs:182-196)
### input-validation#1.98 · values · input · ## Input Validation · lines 336–338
- **Boundary:** Serialization output
  - **What to validate:** Inline SVG
  - **How (stack-specific):** Must be well-formed XML: text with `&amp;`/`&lt;` and `xlink:href` must still parse (tests/blitz-tests/tests/inline_svg_serialize.rs:1-2; tests/blitz-tests/tests/inline_svg_serialize.rs:40-58)
### input-validation#1.99 · values · input · ## Input Validation · lines 339–341
- **Boundary:** Test harness queries
  - **What to validate:** Selectors
  - **How (stack-specific):** Selector parsing returns a `Result` (`try_parse_selector_list`, `query_selector(...).unwrap()`) (tests/blitz-tests/tests/scoped_query_selector.rs:32-35; tests/blitz-tests/tests/scoped_query_selector.rs:165); the harness query helper panics with "invalid selector" on a selector that fails to parse (packages/blitz-test-harness/src/inspect.rs:27-34); scoped queries with a text or comment node as scope return no matches (tests/blitz-tests/tests/scoped_query_selector.rs:126-149)
### input-validation#1.100 · values · input · ## Input Validation · lines 342–344
- **Boundary:** WPT runner
  - **What to validate:** Timeout-quarantine entries
  - **How (stack-specific):** Must contain a path and reason and must not duplicate (panics otherwise) (wpt/runner/src/test_runners/mod.rs:33-41); a unit test checks paths are non-empty without whitespace and reasons are in `KNOWN_REASONS` (wpt/runner/src/test_runners/mod.rs:406-441)
### input-validation#1.101 · values · input · ## Input Validation · lines 345–347
- **Boundary:** WPT runner
  - **What to validate:** Fuzzy ranges
  - **How (stack-specific):** Must parse as integers with `min <= max`, else the spec is dropped (wpt/runner/src/test_runners/fuzzy.rs:25-35; wpt/runner/src/test_runners/fuzzy.rs:90-96)
### input-validation#1.102 · values · input · ## Input Validation · lines 348–350
- **Boundary:** WPT runner
  - **What to validate:** checkLayout
  - **How (stack-specific):** Attribute values that do not parse as f32 produce a subtest error (wpt/runner/src/test_runners/attr_test.rs:248-253); an unparseable selector panics (wpt/runner/src/test_runners/attr_test.rs:79-81)
### input-validation#1.103 · values · input · ## Input Validation · lines 351–353
- **Boundary:** WPT runner
  - **What to validate:** Wrapper HTML
  - **How (stack-specific):** Escapes META `title` as text and `script` as a double-quoted attribute (wpt/runner/src/test_runners/js_wrapper.rs:63-74)
### input-validation#1.104 · values · input · ## Input Validation · lines 354–356
- **Boundary:** WPT runner
  - **What to validate:** Script type
  - **How (stack-specific):** Only `<script>` types "", text/javascript, application/javascript and module count as JavaScript (wpt/runner/src/test_runners/mod.rs:53-63)
### input-validation#1.105 · values · input · ## Input Validation · lines 357–359
- **Boundary:** WPT runner
  - **What to validate:** `data:` URLs
  - **How (stack-specific):** Processing and base64 decoding errors record a request failure (wpt/runner/src/net_provider.rs:60-65)
### input-validation#1.106 · values · input · ## Input Validation · lines 360–362
- **Boundary:** WPT runner
  - **What to validate:** Page messages
  - **How (stack-specific):** Parsed as JSON and checked for `type` before use (wpt/runner/src/test_runners/harness_test.rs:231-250)
### input-validation#1.107 · values · input · ## Input Validation · lines 363–365
- **Boundary:** CI scripts
  - **What to validate:** PR lookup
  - **How (stack-specific):** Selects only open PRs whose head sha equals the workflow run's head sha, and skips when none is found (.github/workflows/wpt-post-results.yml:36-42)
### input-validation#1.108 · values · input · ## Input Validation · lines 366–368
- **Boundary:** CI scripts
  - **What to validate:** Comment splice
  - **How (stack-specific):** `splice` replaces only when both markers exist with end after start (.github/scripts/wpt_diff_to_pr.py:252-260)
### input-validation#1.109 · values · input · ## Input Validation · lines 369–371
- **Boundary:** Example apps
  - **What to validate:** 7GUIs inputs
  - **How (stack-specific):** Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59); cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:28-30; examples/seven_guis/src/tasks/cells.rs:146; examples/seven_guis/src/tasks/cells.rs:229-232); circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:111-114)
### input-validation#1.110 · values · input · ## Input Validation · lines 372–374
- **Boundary:** Example apps
  - **What to validate:** Widget and TodoMVC
  - **How (stack-specific):** The widget's `color` attribute is parsed with `parse_color`, falling back to black on failure (examples/custom_widget.rs:109-115); the TodoMVC page trims input and ignores empty text (examples/preact/index.html:60-63)
### input-validation#1.111 · values · input · ## Input Validation · lines 375–377
- **Boundary:** Example fixtures
  - **What to validate:** Form inputs
  - **How (stack-specific):** The graphite fixture's name and email inputs are required and the email input is type=email (examples/assets/graphite.html:2052; examples/assets/graphite.html:2054); its phone input has autocomplete=off and tabindex=-1, and its column is display:none (examples/assets/graphite.html:2053; examples/assets/graphite.html:293-295); the google fixture's query textarea is limited to maxlength 2048 (examples/assets/google.html:3181)
