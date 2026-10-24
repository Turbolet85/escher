### input-validation#1.79 · values · input · ## Input Validation · lines 279–281
- **Boundary:** Forms (dioxus-native)
  - **What to validate:** Form data validity
  - **How (stack-specific):** `NativeFormData::valid` always returns true, with the comment "todo: actually implement validation here" (packages/dioxus-native-dom/src/events.rs:327-330)
### input-validation#1.80 · values · input · ## Input Validation · lines 282–284
- **Boundary:** Dioxus mutations
  - **What to validate:** `data-dioxus-id`
  - **How (stack-specific):** Parsed as `usize`; a parse failure yields no element id (packages/dioxus-native-dom/src/dioxus_document.rs:32-39)
### input-validation#1.81 · values · input · ## Input Validation · lines 285–287
- **Boundary:** Dioxus mutations
  - **What to validate:** `dangerous_inner_html`
  - **How (stack-specific):** Applied with `set_inner_html` (packages/dioxus-native-dom/src/mutation_writer.rs:430-431)
### input-validation#1.82 · values · input · ## Input Validation · lines 288–290
- **Boundary:** Dioxus mutations
  - **What to validate:** Attribute value types
  - **How (stack-specific):** Values of unsupported `AttributeValue` types are ignored (packages/dioxus-native-dom/src/mutation_writer.rs:306-308)
### input-validation#1.83 · values · input · ## Input Validation · lines 291–293
- **Boundary:** Dioxus mutations
  - **What to validate:** Boolean attributes with a falsy value
  - **How (stack-specific):** Removed, not written: a falsy value (`None`, Text `"false"`, Bool false, Int 0, Float 0.0) of one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set (`checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them; the 27 before `inert` the bridge's own `BOOLEAN_ATTRIBUTES` list until 2026-10-13-upstream-sync-polish-and-ship, which retired it for upstream's predicate, #1188, and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`) clears the attribute, on the dynamic and the static template attribute path alike; a truthy value of a listed name, and an attribute outside the list holding `false`, are still written. It is an in-process mutation path: no new input class and no crossing (packages/dioxus-native-dom/src/mutation_writer.rs:439-475; tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:1-9) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md and, for the 28-name predicate, escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md
### input-validation#1.101 · values · input · ## Input Validation · lines 345–347
- **Boundary:** WPT runner
  - **What to validate:** Fuzzy ranges
  - **How (stack-specific):** Must parse as integers with `min <= max`, else the spec is dropped (wpt/runner/src/test_runners/fuzzy.rs:25-35; wpt/runner/src/test_runners/fuzzy.rs:90-96)
### input-validation#1.102 · values · input · ## Input Validation · lines 348–350
- **Boundary:** WPT runner
  - **What to validate:** checkLayout
  - **How (stack-specific):** Attribute values that do not parse as f32 produce a subtest error (wpt/runner/src/test_runners/attr_test.rs:241-246); an unparseable selector panics (wpt/runner/src/test_runners/attr_test.rs:79-81)
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
### input-validation#1.109 · values · input · ## Input Validation · lines 369–371
- **Boundary:** Example apps
  - **What to validate:** 7GUIs inputs
  - **How (stack-specific):** Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59); cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:28-30; examples/seven_guis/src/tasks/cells.rs:146; examples/seven_guis/src/tasks/cells.rs:229-232); circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:111-114)
