### input-validation#1.96 · values · input · ## Input Validation · lines 330–332
- **Boundary:** Scrolling
  - **What to validate:** Targets
  - **How (stack-specific):** Clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:164-167); programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:566-568)
### input-validation#1.97 · values · input · ## Input Validation · lines 333–335
- **Boundary:** Scrolling
  - **What to validate:** an open select list's own scroll
  - **How (stack-specific):** `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, moving no chain or viewport; a wheel over the list scrolls it, and a wheel elsewhere closes it and then scrolls; its offset stays within its rows (`Geometry::clamped`); pinned by `a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement` and `a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel` (packages/blitz-dom/src/scrolling.rs:669-672; packages/blitz-dom/src/node/select_list.rs:296-304; packages/blitz-dom/src/node/select_list.rs:496-519; tests/blitz-tests/tests/select_popup.rs:870-911) — an engine change for every document and a boundary widening, ratified — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md
### input-validation#1.98 · values · input · ## Input Validation · lines 336–338
- **Boundary:** Forms
  - **What to validate:** `method` / `enctype`
  - **How (stack-specific):** `method` parsed case-insensitively and defaults to GET; an unknown `enctype` defaults to `application/x-www-form-urlencoded` (packages/blitz-dom/src/form.rs:80-111; packages/blitz-dom/src/form.rs:373-382)
### input-validation#1.99 · values · input · ## Input Validation · lines 339–341
- **Boundary:** Forms
  - **What to validate:** Submitted controls
  - **How (stack-specific):** Excludes controls with a datalist ancestor, disabled controls, non-submitter buttons and unchecked checkboxes/radios (packages/blitz-dom/src/form.rs:224-240)
### input-validation#1.100 · values · input · ## Input Validation · lines 342–344
- **Boundary:** Forms
  - **What to validate:** Names and values
  - **How (stack-specific):** Line endings normalized to CRLF before submission (packages/blitz-dom/src/form.rs:427-464)
### input-validation#1.101 · values · input · ## Input Validation · lines 345–347
- **Boundary:** Forms
  - **What to validate:** File inputs
  - **How (stack-specific):** The `accept` attribute is not applied as a filter (packages/blitz-dom/src/events/pointer.rs:1050-1052)
### input-validation#1.102 · values · input · ## Input Validation · lines 348–350
- **Boundary:** Forms (dioxus-native)
  - **What to validate:** Form data validity
  - **How (stack-specific):** `NativeFormData::valid` always returns true, with the comment "todo: actually implement validation here" (packages/dioxus-native-dom/src/events.rs:327-330)
### input-validation#1.103 · values · input · ## Input Validation · lines 351–353
- **Boundary:** Dioxus mutations
  - **What to validate:** `data-dioxus-id`
  - **How (stack-specific):** Parsed as `usize`; a parse failure yields no element id (packages/dioxus-native-dom/src/dioxus_document.rs:48-55)
### input-validation#1.104 · values · input · ## Input Validation · lines 354–356
- **Boundary:** Dioxus mutations
  - **What to validate:** `dangerous_inner_html`
  - **How (stack-specific):** Applied with `set_inner_html` (packages/dioxus-native-dom/src/mutation_writer.rs:430-431)
### input-validation#1.105 · values · input · ## Input Validation · lines 357–359
- **Boundary:** Dioxus mutations
  - **What to validate:** Attribute value types
  - **How (stack-specific):** Values of unsupported `AttributeValue` types are ignored (packages/dioxus-native-dom/src/mutation_writer.rs:306-308)
### input-validation#1.106 · values · input · ## Input Validation · lines 360–362
- **Boundary:** Dioxus mutations
  - **What to validate:** Boolean attributes with a falsy value
  - **How (stack-specific):** Removed, not written: a falsy value (`None`, Text `"false"`, Bool false, Int 0, Float 0.0) of one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set (`checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them; the 27 before `inert` the bridge's own `BOOLEAN_ATTRIBUTES` list until 2026-10-13-upstream-sync-polish-and-ship, which retired it for upstream's predicate, #1188, and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`) clears the attribute, on the dynamic and the static template attribute path alike; a truthy value of a listed name, and an attribute outside the list holding `false`, are still written. It is an in-process mutation path: no new input class and no crossing (packages/dioxus-native-dom/src/mutation_writer.rs:439-475; tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:1-9) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md and, for the 28-name predicate, escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md
### input-validation#1.107 · values · input · ## Input Validation · lines 363–365
- **Boundary:** JS API arguments
  - **What to validate:** `Response` status
  - **How (stack-specific):** Throws RangeError for a status outside 200–599 (packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:38-39)
### input-validation#1.108 · values · input · ## Input Validation · lines 366–368
- **Boundary:** JS API arguments
  - **What to validate:** `fetch()` method
  - **How (stack-specific):** Rejects with TypeError for any method other than GET or HEAD (packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:81-83)
### input-validation#1.109 · values · input · ## Input Validation · lines 369–371
- **Boundary:** JS API arguments
  - **What to validate:** Selectors
  - **How (stack-specific):** An unparseable selector raises a `SyntaxError` DOMException (packages/blitz-vibey-script/src/dom/document.rs:357-376; packages/blitz-vibey-script/src/dom/element/selectors.rs:9-17)
### input-validation#1.110 · values · input · ## Input Validation · lines 372–374
- **Boundary:** JS API arguments
  - **What to validate:** Scroll options
  - **How (stack-specific):** `ScrollBehavior` and `ScrollLogicalPosition` values outside their enumerations throw TypeError (packages/blitz-vibey-script/src/dom/element/scrolling.rs:35-52; packages/blitz-vibey-script/src/dom/element/scrolling.rs:206-228); a single non-object `scrollTo` argument throws TypeError (packages/blitz-vibey-script/src/dom/element/scrolling.rs:90-95)
### input-validation#1.111 · values · input · ## Input Validation · lines 375–377
- **Boundary:** JS API arguments
  - **What to validate:** `width`/`height` reflection
  - **How (stack-specific):** Parses non-negative integers and caps at 2147483647, falling back to the default (packages/blitz-vibey-script/src/dom/element/attributes.rs:289-305; packages/blitz-vibey-script/src/dom/element/attributes.rs:343-354)
### input-validation#1.112 · values · input · ## Input Validation · lines 378–380
- **Boundary:** JS API arguments
  - **What to validate:** Timers
  - **How (stack-specific):** Delays that are non-finite or not positive become 0; clear functions ignore non-finite or negative ids (packages/blitz-vibey-script/src/runtime/timers.rs:51-59; packages/blitz-vibey-script/src/runtime/timers.rs:135-137)
### input-validation#1.113 · values · input · ## Input Validation · lines 381–383
- **Boundary:** JS API arguments
  - **What to validate:** `CSS.registerProperty`
  - **How (stack-specific):** Requires a dictionary with `name` and `inherits`, and maps registration failures to SyntaxError or Error (packages/blitz-vibey-script/src/runtime/css.rs:81-131)
### input-validation#1.114 · values · input · ## Input Validation · lines 384–386
- **Boundary:** JS API arguments
  - **What to validate:** Style `setProperty` / `cssText`
  - **How (stack-specific):** `setProperty` ignores invalid declarations; `cssText` assignment re-serializes and drops invalid declarations (packages/blitz-vibey-script/src/dom/style.rs:120-123; packages/blitz-vibey-script/src/dom/style.rs:142-150)
### input-validation#1.115 · values · input · ## Input Validation · lines 387–389
- **Boundary:** JS API arguments
  - **What to validate:** CSS property accessors on `CSSStyleDeclaration`
  - **How (stack-specific):** Script reads and writes CSS properties through named accessors that `installStyleProperties` defines on `CSSStyleDeclaration` (packages/blitz-vibey-script/src/runtime/bootstrap/style.js:26-48), one per name `css_property_names` takes from Stylo's property list (packages/blitz-vibey-script/src/runtime/css.rs:8-18) — a realization on the registered script-to-DOM crossing, not a boundary widening; witnessed by `style_property_accessors` (packages/blitz-vibey-script/tests/dom.rs:495-527) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md
### input-validation#1.116 · values · input · ## Input Validation · lines 390–392
- **Boundary:** JS API arguments
  - **What to validate:** Node arguments
  - **How (stack-specific):** Native methods throw TypeError when `this` or an argument is not a DOM node (packages/blitz-vibey-script/src/dom/mod.rs:53-60; packages/blitz-vibey-script/src/dom/node.rs:284-290); `replaceChild` checks for `HierarchyRequestError` and `NotFoundError` (packages/blitz-vibey-script/src/dom/node.rs:362-382)
### input-validation#1.117 · values · input · ## Input Validation · lines 393–395
- **Boundary:** JS API arguments
  - **What to validate:** geometry.js conversions
  - **How (stack-specific):** Rejects BigInt in number conversion and non-object dictionaries with TypeError (packages/blitz-vibey-script/src/geometry.js:27-46)
### input-validation#1.118 · values · input · ## Input Validation · lines 396–398
- **Boundary:** JS API arguments
  - **What to validate:** Selection (`document.getSelection`)
  - **How (stack-specific):** `setBaseAndExtent` checks its argument count and its offsets and throws a JS error on a bad call, leaving the selection unchanged; the API writes the document's text selection, and the `innerText` / `outerText` getters that arrived beside it only read; upstream's, and escher uses none of them (packages/blitz-vibey-script/src/dom/selection.rs:69-119; packages/blitz-vibey-script/tests/selection.rs:1-266) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
### input-validation#1.119 · values · input · ## Input Validation · lines 399–401
- **Boundary:** Persistence (sqlite)
  - **What to validate:** SQL values
  - **How (stack-specific):** Statements bind values through `params!` placeholders (apps/browser/persistence/src/lib.rs:165; apps/browser/persistence/src/lib.rs:231-243; apps/browser/persistence/src/lib.rs:249-256; apps/browser/persistence/src/lib.rs:268-271)
### input-validation#1.120 · values · input · ## Input Validation · lines 402–404
- **Boundary:** Persistence (sqlite)
  - **What to validate:** Stored URLs
  - **How (stack-specific):** Re-parsed on load; unparsable rows are skipped (apps/browser/persistence/src/lib.rs:184-188)
