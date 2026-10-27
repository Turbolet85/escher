# security-plan · picked accounting · group 2 of 3

## Picked
input-validation#1.26 · proposed · F1's `file:` refusal by document (`document_may_load`, a new `FetchError` variant) joins the Script fetch boundary beside `FetchError::InvalidData`, which still holds
input-validation#1.27 · not touched · document.rs's added lines are the `<script src>` `file:` check (163-169, 210-211, 222) and line 16, the executed type set not among them; its citation's shift is the sweep's
input-validation#1.39 · not touched · construct.rs's added lines are the IME enable (313-317) and the pseudo flow vote (860, 867-872), the inline SVG parse path not among them; its citation's shift is the sweep's
input-validation#1.41 · not touched · element.rs's added lines are `flush_open_state` (488-489, 543-550), `set_style_property` / `remove_style_property`'s refusal unchanged; F4's accessors are placed beside the JS API style row 1.91
input-validation#1.45 · not touched · stylo.rs's one added line is `NonTSPseudoClass::Open` (510), the `bgcolor` parse unchanged; its citation's shift is the sweep's
input-validation#1.46 · not touched · stylo.rs gains only line 510 (`:open`), the dimension parser unchanged; its citation's shift is the sweep's
input-validation#1.47 · not touched · stylo.rs gains only line 510 (`:open`), the SVG `width` / `height` rejection unchanged; its citation's shift is the sweep's
input-validation#1.48 · not touched · stylo.rs gains only line 510 (`:open`), the `border` and body-margin parses unchanged; its citation's shift is the sweep's
input-validation#1.51 · not touched · construct.rs's added lines are the IME enable and the pseudo vote, the `<ol start>` parse unchanged; its citation's shift is the sweep's
input-validation#1.52 · not touched · element.rs's added lines are `flush_open_state` and its call in `ElementData::new`, `attr_parsed` and the `disabled` presence read unchanged; its citation's shift is the sweep's
input-validation#1.53 · proposed · F3, the `open` presence read (`flush_open_state` into `ElementState::OPEN`, so `:open` matches), joins beside this `disabled` presence row; `is_disabled` and `flush_is_focussable` re-read unchanged
input-validation#1.54 · not touched · mutator.rs's added lines are the `open` arms of `set_attribute` / `remove_attribute` (368-371, 520-523), the autofocus read unchanged; its citation's shift is the sweep's
input-validation#1.56 · not touched · mutator.rs's added lines are the `open` arms alone, the canvas `src` parse unchanged; its citation's shift is the sweep's
input-validation#1.57 · not touched · window.rs's added lines 193-194 are the redraw route, not the poll's changed-set drain and tree rebuild; the snapshot readers, element ids and escher-driver are unreached; its citations' shifts are the sweep's
input-validation#1.63 · not touched · the conflict resolution keeps escher's range re-sanitize arm and puts the `open` arm after it as a second independent `if`, `RangeInputData`'s seeding re-read unchanged; its citations' shifts are the sweep's
input-validation#1.65 · not touched · the report re-reads the select arms unchanged, `dioxus_document.rs` removes 0 lines (added at 725-760) and escher's select-by-id comment is carried into `runtime/events.rs`
input-validation#1.80 · not touched · `dioxus_document.rs`'s diff removes 0 lines and adds only the test at 725-760, the `data-dioxus-id` parse unchanged
input-validation#1.84 · not touched · the `Response` status check moved whole with the runtime split (a move, none removed), the reading unchanged; its citation's re-point is the sweep's
input-validation#1.85 · not touched · the GET/HEAD rule moved whole into the `runtime/` split; the `file:` check `fetch()` gains is F1, carried beside Script fetch 1.26; its citation's re-point is the sweep's
input-validation#1.86 · not touched · the element selector bindings moved whole into `dom/element/selectors.rs` (`invalid_selector_error` 9-17), the `SyntaxError` reading unchanged; its citation's re-point is the sweep's
input-validation#1.87 · not touched · the scroll option parsers moved whole into `dom/element/scrolling.rs` (35-52, 64-113, 206-228), the TypeError readings unchanged; its citation's re-point is the sweep's
input-validation#1.88 · not touched · the dimension reflection moved whole into `dom/element/attributes.rs` (289-305, 320-357), the reading unchanged; its citation's re-point is the sweep's
input-validation#1.89 · not touched · the timer coercions moved whole into `runtime/timers.rs`, whose new text shows the same finite-and-positive delay (55-59) and finite non-negative id (135-137) checks; its citation's re-point is the sweep's
input-validation#1.90 · not touched · `CSS.registerProperty` moved whole into `runtime/css.rs` `css_register_property` (71-132), the reading unchanged; its citation's re-point is the sweep's
input-validation#1.91 · proposed · F4's CSS property accessors on `CSSStyleDeclaration` join the script-side style row; `setProperty` / `cssText` themselves (`dom/style.rs`) are unreached
