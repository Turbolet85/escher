# a11y-plan · picked accounting · group 2 of 2

## Picked
keyboard-navigation#1.1 · not touched · the report re-read `is_disabled` and `flush_is_focussable` unchanged in the reached `node/element.rs`, only their line numbers shift (the citation sweep's), so the focusability rule holds
keyboard-navigation#1.3 · not touched · the take's `open` arm in the conflict surface `mutator.rs` stands as a second independent `if` after the range arm, so the `tabindex` / `href` / `disabled` recompute holds; its second range shifts +4 (the sweep's)
keyboard-navigation#1.4 · not touched · the `DISABLED`/`ENABLED` toggling is unchanged (`is_disabled` re-read); the only additions to `node/element.rs` are the `flush_open_state` call at 488-489 and the method at 543-549
keyboard-navigation#1.5 · not touched · disabled-ness and focusability re-read unmoved by the report; `pointer.rs` and `keyboard.rs` not reached, `node/element.rs` only shifted
keyboard-navigation#1.6 · not touched · the take's `mutator.rs` hunks are the `open` arms at 368-371 and 520-523 alone (exact-take diff green), so the file input's inner-button `tabindex="-1"` holds; its range shifts +8 (the sweep's)
keyboard-navigation#1.7 · not touched · the autofocus flush in `mutator.rs` is outside the take's two `open` hunks; the claim holds, its ranges shift +8 (the sweep's)
keyboard-navigation#1.9 · not touched · the `:focus` / `:focus-within` / `:focus-visible` arms at `stylo.rs:488-492` stand before the one added line 510 (`NonTSPseudoClass::Open`); `:open` is F1, placed beside #2.5
keyboard-navigation#2.1 · not touched · removal's blur and IME disable cite unreached files and hold; the IME enable at an editor's construction is F3, screen-reader-support's, placed beside #12.1
keyboard-navigation#2.5 · proposed · F1: the `details` the summary toggles now holds `ElementState::OPEN`, so `:open` matches it and flips with the toggle
keyboard-navigation#3.4 · not touched · Enter / Space activation (`keyboard.rs`, unreached) is unchanged by the report; what the summary's click now moves (`:open`) is F1 beside #2.5
keyboard-navigation#3.11 · not touched · in the reached `window.rs` only the redraw route moved (+2 at 193-194); key dispatch holds, its range shifts (the sweep's)
keyboard-navigation#3.12 · not touched · `application.rs` not reached and `window.rs` only shifted; the macOS binding forwarding holds
keyboard-navigation#3.13 · not touched · the report names this IME event plumbing (`keyboard-navigation.md:48`) re-read, no-change
keyboard-navigation#4.1 · not touched · `focus()` / `blur()` moved whole into `dom/element/form_controls.rs` (286-291, 293-298), nothing removed; the claim holds and only its citations move (the sweep's, by hand at P2)
keyboard-navigation#4.2 · proposed · F4 belongs beside it: the script exposure group gains the CSS property accessors on `CSSStyleDeclaration`; its own claim holds, `set_autofocus` moved whole to `form_controls.rs`
keyboard-navigation#4.4 · not touched · `dioxus_document.rs` gains only one test past 724; the keyboard-event forwarding at 371-375 holds
keyboard-navigation#4.5 · not touched · `dioxus_document.rs` gains only one test past 724; the focus-event forwarding at 366-369 holds
keyboard-navigation#4.9 · not touched · `mutation_writer.rs` and the boolean-attribute tests are not reached and the 28-name predicate holds; what an `open` write now sets is F1 beside #2.5
keyboard-navigation#7.2 · not touched · `stand_keyboard`, the Tab tables and `run stand` (205 · 0 · 7) unmoved; the new target runs outside the a11y leg and `run stand`
visual-design-verification#6.1 · proposed · F2 belongs beside it: state styling gains author styles keyed on `:open` for an open `details` or `dialog`; the browser.css row itself is not reached
visual-design-verification#7.4 · not touched · in `window.rs` only the redraw route moved; the zoom keys hold, their range shifts +2 (the sweep's)
visual-design-verification#8.1 · not touched · the exclusion is by computed style and its files are not reached, so it holds; the script route to `style.display` / `visibility` is F4, placed beside keyboard-navigation#4.2
screen-reader-support#12.1 · proposed · F3: a text input already focused when its editor is constructed enables IME at construction, beside focus's enable and blur's disable
