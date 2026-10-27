### input-validation#1.65 · values · input · ## Input Validation · lines 231–235
- **Boundary:** Markup attributes
  - **What to validate:** `select` (single-choice) — its choice, keys and typeahead, click, the `selected` attribute, an option's `label` · `value`, the `change` event
  - **How (stack-specific):** In every document, remote pages included: a `select` without `multiple` holds its choice as its options' `CHECKED` state, seeded from a `selected` attribute at creation, a `selected` write exclusive and a cleared one unchosen, every recorded select settled at the mutator's flush and on entering the document by HTML's selectedness setting algorithm — the last chosen, else the first option not disabled (packages/blitz-dom/src/node/element.rs:472-478; packages/blitz-dom/src/mutator.rs:1033-1036; packages/blitz-dom/src/mutator.rs:1113-1117; packages/blitz-dom/src/mutator.rs:1236-1257; packages/blitz-dom/src/node/select.rs:155-164); its rules, cited to the WHATWG `select` element, read the `option` children in tree order (an `optgroup`'s not listed), an option's label (its non-empty `label` attribute, else its stripped and collapsed text) and its value (its `value` attribute, else the text) (packages/blitz-dom/src/node/select.rs:17-193); on a focused single-choice select that `ElementData::is_disabled` does not read disabled, a pressed `ArrowDown` / `ArrowUp` takes the next / previous enabled option, `Home` / `End` the first / last, a `Key::Character` with no Ctrl, Alt or Meta is typeahead and every other key does nothing (Enter and Space open nothing) — a closed key match (packages/blitz-dom/src/events/keyboard.rs:108-113; packages/blitz-dom/src/events/keyboard.rs:165-200); typeahead keeps the typed text in the document's private `select_typeahead` buffer, starting afresh for another select or after 1 s of the document's clock, and chooses the first enabled option whose label starts with it, compared in lower case, an empty text matching nothing — compared, never kept past 1 s and never carried out (packages/blitz-dom/src/events/keyboard.rs:248-269; packages/blitz-dom/src/node/select.rs:183-193);
 a click on a select, past `handle_click`'s `is_disabled` walk gate, clears the typeahead and the text selection and focuses it (packages/blitz-dom/src/events/pointer.rs:857-870); every choice goes through one path that marks the changed set and dispatches `input` then the engine-emitted `change` (not cancelable, bubbling), each carrying `BaseDocument::select_value`, a choice of the chosen option firing nothing (packages/blitz-dom/src/events/keyboard.rs:271-297; packages/blitz-traits/src/events.rs:150; packages/dioxus-native-dom/src/dioxus_document.rs:377-382); the choice leaves only as form submission's entries (every chosen option not disabled, packages/blitz-dom/src/form.rs:276-290), the `input` and `change` events' value, the snapshot's `value` (the chosen option's value, no mask applying) and the options' tree nodes — an option under a select kept in the accessibility tree though it computes `display: none`, its `selected` written from its state, its `label` attribute its name (packages/blitz-dom/src/accessibility.rs:157-162; packages/blitz-dom/src/accessibility.rs:171-181; packages/blitz-dom/src/accessibility.rs:257-258; packages/dioxus-native-dom/src/snapshot.rs:241-259) — inside the ratified name and control-value crossings; the driver's `type` on a select is the same typeahead, its help one fixed sentence more (packages/escher-driver/src/schema.rs:238-239); no print, log site, env read or panic path in the rules module, and no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added; stated limits: a `multiple` select and an `optgroup`'s options are not modelled, no popup is painted and an option is not clicked (refused `covered`); pinned by the rules module's 4 unit tests, `select_interaction` 7 and `stand_act_select` 5, both layout modes (tests/blitz-tests/tests/select_interaction.rs:113-343;
 tests/blitz-tests/tests/stand_act_select.rs:79-256) — an engine change for every document and a boundary widening, ratified — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md
### input-validation#1.66 · values · input · ## Input Validation · lines 236–242
- **Boundary:** Markup attributes
  - **What to validate:** password and file `input` value (the snapshot's `value`)
  - **How (stack-specific):** Masked by the in-process reader: under dioxus-native-dom's `accessibility` feature `DioxusDocument::snapshot` reads `NodeState::value` for an `input` whose `type` is `password` or `file` (each compared ASCII-case-insensitively) as the public constant `MASKED_VALUE` — eight U+2022 characters, one fixed marker whatever was typed or chosen — where the control holds a non-empty value (a password's editor text, else its `value` attribute; a file input's `value` attribute — it holds no text editor), as `""` where that value is empty and as `None` where a file input has no `value` attribute; never the text, never its length and never a file's host path. The typed text and the path occur in no id, name or value of any snapshot node, and the typed text in no label or value of any accessibility-tree node.
 Password- and file-only, and — but for the password's paint, below — snapshot-only: every other element's `value` reads as before, and the DOM is untouched — a file input's `value` attribute still holds the chosen file's host path and form submission still carries it (§Data Protection); the mask adds no input class and no crossing — the snapshot's one text form, `Snapshot::to_text`, reads the model, so its `value=` field holds the quoted `MASKED_VALUE` and neither literal occurs anywhere in the text, which reaches no log or event and a file only as a kept check's kept answer (since 2026-10-19-recorded-check-replay), where the mask holds — a value in it reads `MASKED_VALUE`, as on every carrier, beside the `type` stand-in `MASKED_STAND_IN` (ratified by the founder, 2026-10-07 and 2026-10-19, as the `id` row records); a diff of two snapshots (`Snapshot::diff`) takes its values from the snapshots only, so a password's or a file input's value in a diff entry is `MASKED_VALUE` — measured for a password input: the typed text occurs in no id, parent, name or value of any diff entry, the first character's entry reads `MASKED_VALUE` and further characters change no entry; a file input's by the same construction, not separately run — and the diff adds no input class and no crossing (ratified by the founder, 2026-10-07, as the `id` row records); since 2026-10-07-act-by-id the driver returns that text and that diff to the caller of `Session::run`, and the mask holds there — measured: a password typed through the driver's `type` occurs 0 times in what the call returns, written out whole, and the entry's value reads `MASKED_VALUE`, in both layout modes;
 since 2026-10-10-driver-cli control values cross the session socket and reach the client's stdout in a call's answer (ratified by the founder, 2026-10-10, as the `id` row records), and since 2026-10-10-mcp-surface MCP tool results on the `mcp` role's stdout — the mask holding there by construction, a result's text being `Session::run`'s own output (a `snapshot` block measured equal, byte for byte, to `Session::run`'s screen text; since 2026-10-12-self-description a password typed through an MCP `type` measured too — the typed text 0 times in either tool result, the input reading `MASKED_VALUE` with role `PasswordInput` and a non-empty name, in both layout modes, `stand_mcp_masked` (tests/blitz-tests/tests/stand_mcp_masked.rs:67-143); a file input's over MCP by the same construction, not separately run) — a password's or a file input's as `MASKED_VALUE` and every other control's as it reads — measured for a password: the typed text occurs 0 times in the answer's written form and the value reads the mask, in both layout modes (`stand_act_filled`, and a unit test of the node writer); a file input's written form by the same construction, not separately run (packages/dioxus-native-dom/src/snapshot.rs:11-13; packages/dioxus-native-dom/src/snapshot.rs:256-283; packages/dioxus-native-dom/src/snapshot_text.rs:53; tests/blitz-tests/tests/stand_snapshot_state.rs:647-696; tests/blitz-tests/tests/stand_snapshot_text.rs:1-7; packages/dioxus-native-dom/src/snapshot_diff.rs:108; tests/blitz-tests/tests/stand_diff.rs:1-12; tests/blitz-tests/tests/stand_act_diff.rs:1-13)
 — the password mask as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md, the file mask and the text at escher-0.1.0/chunks/2026-10-06-compact-snapshot-serialization/report.md, the diff at escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/report.md, the driver-typed password at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md; since 2026-10-12-headless-screenshot the mask also holds on the screen's pixels: blitz-paint paints an `input` whose `type` reads `password` masked, a bullet per character laid out in the document's own fonts by blitz-dom's `BaseDocument::layout_as_text_input`, in every document and through every renderer that paints with blitz-paint, windowed included (`draw_masked_input_text`, packages/blitz-paint/src/render.rs:975-1049) — an engine change for every document, ratified by the founder (2026-10-12, «Движок маскирует», by question dialog in the overseer session, relayed verbatim by the overseer); measured: two values of one length paint byte-identical frames, focused or not, in both layout modes (`password_painted_masked`), and a password typed through the driver never leaves as pixels in a `screenshot` (`stand_act_screenshot`); unlike `MASKED_VALUE` the frame depends on the value's length, and a click inside a field holding a value places the caret by the bullets
 — `ClickTarget::TextInput`'s `password` flag (an `input` whose `type` reads `password`, ASCII case-insensitive) sends it to the private `click_password_input`, which lays out one U+2022 per character through `BaseDocument::layout_as_text_input`, scrolls that layout as paint scrolls it, finds the bullet under the point with parley's `Cursor::from_point` and moves the real editor to that character's byte offset (a click `move_to_byte`, a shift-click `extend_selection_to_byte`, two or more clicks `select_all`), so no width of the real characters reaches the caret, an overflowing value included, and where a click-placed caret paints in a shot says nothing of them; every other text input keeps `move_to_point`; the snapshot, the diff and the frame's bullets stay masked as above, and nothing new crosses (ratified by the founder, 2026-10-18) (packages/blitz-dom/src/events/pointer.rs:582-648; tests/blitz-tests/tests/password_caret_click.rs:83-121); a file input's paint is unchanged — as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md; since 2026-10-19-session-recording a recorded `type` whose element resolves to a masked control — `dioxus_native_dom::is_masked_input`, now public, the test the snapshot's `value` reader calls, no reading changed — or to none (refused `stale` or `not-found`) is kept in the check as `MASKED_STAND_IN` (`"masked"`) with `masked:true`, never the text, its length or a digest of it, while any other `type`'s text, one refused by `validate` included, is kept as it crossed (ratified by the founder, 2026-10-19: F1 A · F2 B); measured: 0 sentinel occurrences, and a secret of another length leaves byte-identical files (packages/dioxus-native-dom/src/snapshot.rs:232-239; packages/escher-driver/src/execute.rs:389-393; tests/blitz-tests/tests/stand_session_recording.rs:213-266)
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
### input-validation#1.70 · values · input · ## Input Validation · lines 252–254
- **Boundary:** Paint values
  - **What to validate:** Geometry
  - **How (stack-specific):** Geometry whose transformed bounds are non-finite or beyond f32::MAX is culled before painting (packages/blitz-paint/src/render.rs:404-419)
### input-validation#1.71 · values · input · ## Input Validation · lines 255–257
- **Boundary:** Paint values
  - **What to validate:** Gradient lengths
  - **How (stack-specific):** Percentage overflow clamped to the f32 range, with unit tests for both extremes (packages/blitz-paint/src/gradient.rs:497-504; packages/blitz-paint/src/gradient.rs:521-535)
### input-validation#1.72 · values · input · ## Input Validation · lines 258–260
- **Boundary:** Paint values
  - **What to validate:** Border radii
  - **How (stack-specific):** Oversized radii are scaled down uniformly so adjacent radii do not overlap (packages/blitz-paint/src/kurbo_css/css_box.rs:57-75)
### input-validation#1.73 · values · input · ## Input Validation · lines 261–263
- **Boundary:** Paint values
  - **What to validate:** Hidden inputs
  - **How (stack-specific):** Inputs with type=hidden are not painted (packages/blitz-paint/src/render.rs:306-310)
### input-validation#1.74 · values · input · ## Input Validation · lines 264–266
- **Boundary:** Scrolling
  - **What to validate:** Targets
  - **How (stack-specific):** Clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:164-167); programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:566-568)
### input-validation#1.75 · values · input · ## Input Validation · lines 267–269
- **Boundary:** Forms
  - **What to validate:** `method` / `enctype`
  - **How (stack-specific):** `method` parsed case-insensitively and defaults to GET; an unknown `enctype` defaults to `application/x-www-form-urlencoded` (packages/blitz-dom/src/form.rs:80-111; packages/blitz-dom/src/form.rs:377-386)
### input-validation#1.76 · values · input · ## Input Validation · lines 270–272
- **Boundary:** Forms
  - **What to validate:** Submitted controls
  - **How (stack-specific):** Excludes controls with a datalist ancestor, disabled controls, non-submitter buttons and unchecked checkboxes/radios (packages/blitz-dom/src/form.rs:224-240)
### input-validation#1.77 · values · input · ## Input Validation · lines 273–275
- **Boundary:** Forms
  - **What to validate:** Names and values
  - **How (stack-specific):** Line endings normalized to CRLF before submission (packages/blitz-dom/src/form.rs:431-468)
### input-validation#1.78 · values · input · ## Input Validation · lines 276–278
- **Boundary:** Forms
  - **What to validate:** File inputs
  - **How (stack-specific):** The `accept` attribute is not applied as a filter (packages/blitz-dom/src/events/pointer.rs:926-928)
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
