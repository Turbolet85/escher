### input-validation#1.77 · values · input · ## Input Validation · lines 273–275
- **Boundary:** Layout values
  - **What to validate:** Aspect ratios
  - **How (stack-specific):** Degenerate aspect ratios (zero, infinite, NaN) are discarded before use (packages/blitz-dom/src/layout/replaced.rs:132-145), the natural height a usable `aspect-ratio` coerces included, clamped at 0 (packages/blitz-dom/src/layout/replaced.rs:201-211)
### input-validation#1.78 · values · input · ## Input Validation · lines 276–278
- **Boundary:** Layout values
  - **What to validate:** Inline-box heights
  - **How (stack-specific):** Kept finite (`min(f32::MAX)`) so huge author lengths cannot stall the line breaker (packages/blitz-dom/src/layout/inline/layout.rs:470-478)
### input-validation#1.79 · values · input · ## Input Validation · lines 279–281
- **Boundary:** Layout values
  - **What to validate:** Text extent (`BaseDocument::text_extent`)
  - **How (stack-specific):** Since 2026-10-19-visual-lint a read-only reader on every document, remote pages included: an id that does not resolve, a node that is no inline root or one with no inline layout data reads `None` through `get_node`, never an index; it mutates nothing and has no log site, print, env read or panic path; it returns geometry only, and its one non-test caller is `Session::lint`, in process and through the `lint` verb's arm (packages/blitz-dom/src/text_extent.rs:18-54)
### input-validation#1.80 · values · input · ## Input Validation · lines 282–284
- **Boundary:** Layout values
  - **What to validate:** a clipping box's visible region (`BaseDocument::visible_region`)
  - **How (stack-specific):** On every document, remote pages included: `visible_region` reads each clipping box through its transforms, by the private `padding_box_in_view` over `transformed_bounds`; with no transform on the chain it reads the padding box in the client rect's frame, and a mapped box with a non-finite edge answers `None`; exact under a translate and a scale, under a rotation it reads the rotated padding box's bounding box, so it holds corners the box does not show; its callers are `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region`; the added lines hold no log site, print, env read or panic path; pinned by `visible_region_transformed` 6, `stand_act_transformed` 3 and `transformed_client_rect` — an engine change for every document, ratified (packages/blitz-dom/src/scrolling.rs:790-812; packages/blitz-dom/src/document.rs:2574-2578) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
### input-validation#1.81 · values · input · ## Input Validation · lines 285–287
- **Boundary:** Paint values
  - **What to validate:** Geometry
  - **How (stack-specific):** Geometry whose transformed bounds are non-finite or beyond f32::MAX is culled before painting (packages/blitz-paint/src/render.rs:404-419)
### input-validation#1.82 · values · input · ## Input Validation · lines 288–290
- **Boundary:** Paint values
  - **What to validate:** a non-invertible transform under a hit test
  - **How (stack-specific):** On every document, remote pages included: `Node::hit_inner` answers `None` for the whole subtree of a transform whose determinant is 0 or not finite, so it takes no hit; the added lines hold no log site, print, env read or panic path; pinned by `hit_non_invertible_transform` 2 — an engine change for every document, ratified (packages/blitz-dom/src/node/node.rs:1472-1477) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
### input-validation#1.83 · values · input · ## Input Validation · lines 291–293
- **Boundary:** Paint values
  - **What to validate:** Gradient lengths
  - **How (stack-specific):** Percentage overflow clamped to the f32 range, with unit tests for both extremes (packages/blitz-paint/src/gradient.rs:497-504; packages/blitz-paint/src/gradient.rs:521-535)
### input-validation#1.84 · values · input · ## Input Validation · lines 294–296
- **Boundary:** Paint values
  - **What to validate:** Border radii
  - **How (stack-specific):** Oversized radii are scaled down uniformly so adjacent radii do not overlap (packages/blitz-paint/src/kurbo_css/css_box.rs:57-75)
### input-validation#1.85 · values · input · ## Input Validation · lines 297–299
- **Boundary:** Paint values
  - **What to validate:** Hidden inputs
  - **How (stack-specific):** Inputs with type=hidden are not painted (packages/blitz-paint/src/render.rs:306-310)
### input-validation#1.86 · values · input · ## Input Validation · lines 300–302
- **Boundary:** Scrolling
  - **What to validate:** Targets
  - **How (stack-specific):** Clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:164-167); programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:566-568)
### input-validation#1.90 · values · input · ## Input Validation · lines 312–314
- **Boundary:** Forms
  - **What to validate:** File inputs
  - **How (stack-specific):** The `accept` attribute is not applied as a filter (packages/blitz-dom/src/events/pointer.rs:955-957)
### input-validation#1.95 · values · input · ## Input Validation · lines 327–329
- **Boundary:** Dioxus mutations
  - **What to validate:** Boolean attributes with a falsy value
  - **How (stack-specific):** Removed, not written: a falsy value (`None`, Text `"false"`, Bool false, Int 0, Float 0.0) of one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set (`checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them; the 27 before `inert` the bridge's own `BOOLEAN_ATTRIBUTES` list until 2026-10-13-upstream-sync-polish-and-ship, which retired it for upstream's predicate, #1188, and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`) clears the attribute, on the dynamic and the static template attribute path alike; a truthy value of a listed name, and an attribute outside the list holding `false`, are still written. It is an in-process mutation path: no new input class and no crossing (packages/dioxus-native-dom/src/mutation_writer.rs:439-475; tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:1-9) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md and, for the 28-name predicate, escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md
### input-validation#1.122 · values · input · ## Input Validation · lines 408–410
- **Boundary:** Example apps
  - **What to validate:** 7GUIs inputs
  - **How (stack-specific):** Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59); cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:29-31; examples/seven_guis/src/tasks/cells.rs:147; examples/seven_guis/src/tasks/cells.rs:230-233); circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:111-114); the flight booker's mode `select#flight-type` writes its `flight_type` signal by a closed match on the `input` value — `return` a return flight, any other value one-way — over its two options (`one-way`, `return`) (examples/seven_guis/src/tasks/flight_booker.rs:66-89)
### input-validation#1.123 · values · input · ## Input Validation · lines 411–413
- **Boundary:** Example apps
  - **What to validate:** 7GUIs Cells, the `cells-grid` keys
  - **How (stack-specific):** `cells-grid` takes `tabindex` 0 and an `onkeydown` with a closed key match and no role: arrows move `current`, clamped, Enter opens the editor on `current`, focused, and the editor's Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early (examples/seven_guis/src/tasks/cells.rs:425-433), ratified as to the grid — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
