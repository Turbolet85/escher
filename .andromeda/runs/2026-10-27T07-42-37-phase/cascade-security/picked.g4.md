### input-validation#1.67 · values · input · ## Input Validation · lines 243–245
- **Boundary:** Layout values
  - **What to validate:** Aspect ratios
  - **How (stack-specific):** Degenerate aspect ratios (zero, infinite, NaN) are discarded before use (packages/blitz-dom/src/layout/replaced.rs:132-145), the natural height a usable `aspect-ratio` coerces included, clamped at 0 (packages/blitz-dom/src/layout/replaced.rs:201-211)
### input-validation#1.68 · values · input · ## Input Validation · lines 246–248
- **Boundary:** Layout values
  - **What to validate:** Inline-box heights
  - **How (stack-specific):** Kept finite (`min(f32::MAX)`) so huge author lengths cannot stall the line breaker (packages/blitz-dom/src/layout/inline/layout.rs:470-478)
### input-validation#1.69 · values · input · ## Input Validation · lines 249–251
- **Boundary:** Layout values
  - **What to validate:** Text extent (`BaseDocument::text_extent`)
  - **How (stack-specific):** Since 2026-10-19-visual-lint a read-only reader on every document, remote pages included: an id that does not resolve, a node that is no inline root or one with no inline layout data reads `None` through `get_node`, never an index; it mutates nothing and has no log site, print, env read or panic path; it returns geometry only, and its one non-test caller is `Session::lint`, in process and through the `lint` verb's arm (packages/blitz-dom/src/text_extent.rs:18-54)
### input-validation#1.74 · values · input · ## Input Validation · lines 264–266
- **Boundary:** Scrolling
  - **What to validate:** Targets
  - **How (stack-specific):** Clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:164-167); programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:566-568)
### input-validation#1.76 · values · input · ## Input Validation · lines 270–272
- **Boundary:** Forms
  - **What to validate:** Submitted controls
  - **How (stack-specific):** Excludes controls with a datalist ancestor, disabled controls, non-submitter buttons and unchecked checkboxes/radios (packages/blitz-dom/src/form.rs:224-240)
### input-validation#1.78 · values · input · ## Input Validation · lines 276–278
- **Boundary:** Forms
  - **What to validate:** File inputs
  - **How (stack-specific):** The `accept` attribute is not applied as a filter (packages/blitz-dom/src/events/pointer.rs:926-928)
### input-validation#1.80 · values · input · ## Input Validation · lines 282–284
- **Boundary:** Dioxus mutations
  - **What to validate:** `data-dioxus-id`
  - **How (stack-specific):** Parsed as `usize`; a parse failure yields no element id (packages/dioxus-native-dom/src/dioxus_document.rs:32-39)
### input-validation#1.83 · values · input · ## Input Validation · lines 291–293
- **Boundary:** Dioxus mutations
  - **What to validate:** Boolean attributes with a falsy value
  - **How (stack-specific):** Removed, not written: a falsy value (`None`, Text `"false"`, Bool false, Int 0, Float 0.0) of one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set (`checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them; the 27 before `inert` the bridge's own `BOOLEAN_ATTRIBUTES` list until 2026-10-13-upstream-sync-polish-and-ship, which retired it for upstream's predicate, #1188, and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`) clears the attribute, on the dynamic and the static template attribute path alike; a truthy value of a listed name, and an attribute outside the list holding `false`, are still written. It is an in-process mutation path: no new input class and no crossing (packages/dioxus-native-dom/src/mutation_writer.rs:439-475; tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:1-9) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md and, for the 28-name predicate, escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md
### input-validation#1.84 · values · input · ## Input Validation · lines 294–296
- **Boundary:** JS API arguments
  - **What to validate:** `Response` status
  - **How (stack-specific):** Throws RangeError for a status outside 200–599 (packages/blitz-vibey-script/src/runtime.rs:120-123)
### input-validation#1.85 · values · input · ## Input Validation · lines 297–299
- **Boundary:** JS API arguments
  - **What to validate:** `fetch()` method
  - **How (stack-specific):** Rejects with TypeError for any method other than GET or HEAD (packages/blitz-vibey-script/src/runtime.rs:164-168)
### input-validation#1.86 · values · input · ## Input Validation · lines 300–302
- **Boundary:** JS API arguments
  - **What to validate:** Selectors
  - **How (stack-specific):** An unparseable selector raises a `SyntaxError` DOMException (packages/blitz-vibey-script/src/dom/document.rs:357-376; packages/blitz-vibey-script/src/dom/element.rs:1450-1515)
### input-validation#1.87 · values · input · ## Input Validation · lines 303–305
- **Boundary:** JS API arguments
  - **What to validate:** Scroll options
  - **How (stack-specific):** `ScrollBehavior` and `ScrollLogicalPosition` values outside their enumerations throw TypeError (packages/blitz-vibey-script/src/dom/element.rs:1181-1198; packages/blitz-vibey-script/src/dom/element.rs:1328-1350); a single non-object `scrollTo` argument throws TypeError (packages/blitz-vibey-script/src/dom/element.rs:1236-1241)
### input-validation#1.88 · values · input · ## Input Validation · lines 306–308
- **Boundary:** JS API arguments
  - **What to validate:** `width`/`height` reflection
  - **How (stack-specific):** Parses non-negative integers and caps at 2147483647, falling back to the default (packages/blitz-vibey-script/src/dom/element.rs:683-699; packages/blitz-vibey-script/src/dom/element.rs:737-748)
### input-validation#1.89 · values · input · ## Input Validation · lines 309–311
- **Boundary:** JS API arguments
  - **What to validate:** Timers
  - **How (stack-specific):** Delays that are non-finite or not positive become 0; clear functions ignore non-finite or negative ids (packages/blitz-vibey-script/src/runtime.rs:2069-2077; packages/blitz-vibey-script/src/runtime.rs:2396-2398)
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
### input-validation#1.94 · values · input · ## Input Validation · lines 324–326
- **Boundary:** JS API arguments
  - **What to validate:** Selection (`document.getSelection`)
  - **How (stack-specific):** `setBaseAndExtent` checks its argument count and its offsets and throws a JS error on a bad call, leaving the selection unchanged; the API writes the document's text selection, and the `innerText` / `outerText` getters that arrived beside it only read; upstream's, and escher uses none of them (packages/blitz-vibey-script/src/dom/selection.rs:69-119; packages/blitz-vibey-script/tests/selection.rs:1-266) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
### input-validation#1.109 · values · input · ## Input Validation · lines 369–371
- **Boundary:** Example apps
  - **What to validate:** 7GUIs inputs
  - **How (stack-specific):** Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59); cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:28-30; examples/seven_guis/src/tasks/cells.rs:146; examples/seven_guis/src/tasks/cells.rs:229-232); circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:111-114); the flight booker's mode `select#flight-type` writes its `flight_type` signal by a closed match on the `input` value — `return` a return flight, any other value one-way — over its two options (`one-way`, `return`) (examples/seven_guis/src/tasks/flight_booker.rs:66-89)
