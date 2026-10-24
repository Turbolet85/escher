# design · picked accounting

## Picked
color-palette#1.1 · carried · Stylo 0.23 moves the colour conversion's source type, so the conversion must hold through the take
color-palette#1.2 · not in scope · no upstream commit in the delta reaches the colour scheme or `prefers-color-scheme`
color-palette#1.6 · not in scope · dioxus-native's `base_color` frame clear is untouched by the delta and unread by any escher proof
color-palette#1.7 · not in scope · the canvas fallback in blitz-paint is untouched by the delta
color-palette#2.3 · carried · the stand accent's measured contrast is the `stand_contrast` reading the take must keep
color-palette#2.4 · not in scope · the default link colour in `default.css` is untouched by the delta
color-palette#3.5 · carried · the stand page background is one side of the contrast pairs `stand_contrast` re-reads
color-palette#3.7 · not in scope · root examples' backgrounds are no escher proof's reading
color-palette#4.4 · carried · the stand text colour is the other side of those contrast pairs
color-palette#5.1 · not in scope · the flight booker's invalid colours sit in task CSS the delta does not edit
color-palette#5.2 · not in scope · the success colours sit in task CSS the delta does not edit
color-palette#7.3 · not in scope · the default input focus outline is untouched by the delta
typography#1.6 · not in scope · the default form-control font in `default.css` is untouched by the delta
typography#1.7 · not in scope · default heading sizes are untouched by the delta
typography#1.8 · not in scope · the default code font is untouched by the delta
typography#1.9 · not in scope · the stand's type values sit in app CSS the delta does not edit
typography#1.12 · not in scope · the text-input editor size is untouched by the delta
typography#1.13 · not in scope · generic base sizes in `font_metrics.rs` are untouched by the delta
typography#1.14 · carried · the bullet font is set in `construct.rs`, one of the trial merge's four conflicts
typography#2.2 · carried · the stand's bundled font is what every stand check the take must keep green reads text with
typography#2.3 · not in scope · the WASM font-context rule is not reached by the take
typography#2.4 · not in scope · the font feature set is not moved by the delta
typography#2.5 · not in scope · the bullet font's registration in `document.rs` is not named by the delta
typography#2.6 · not in scope · the `fonts-unavailable` refusal is escher-driver's and the delta does not reach it
typography#2.8 · carried · upstream's new test targets run under blitz-tests' `system-fonts` rule
typography#3.1 · not in scope · the generic-family mapping is not named by the delta
typography#3.2 · not in scope · weight and width conversion is not named by the delta
typography#3.3 · not in scope · font-variant mapping is not named by the delta
typography#3.4 · not in scope · line-height mapping is not named by the delta
typography#3.5 · not in scope · spacing resolution is not named by the delta
typography#3.6 · not in scope · the locale read is not named by the delta
typography#3.7 · carried · `3954ecb85` adds `word-break: break-word`, which lands in the word-break mapping
typography#3.8 · not in scope · `ch` and `ic` metrics are not named by the delta
typography#3.9 · not in scope · rem seeding is not named by the delta
typography#3.11 · not in scope · emboldening is not named by the delta
spacing#2.4 · not in scope · stand card padding sits in task CSS the delta does not edit
spacing#2.5 · not in scope · the default body margin is untouched by the delta
spacing#3.1 · not in scope · `hspace`/`vspace` mapping is not named by the delta
spacing#3.2 · not in scope · body margin attributes are not named by the delta
spacing#3.3 · not in scope · scrollbar thumb geometry is not named by the delta
spacing#3.4 · not in scope · single-line input centring is not named by the delta
depth-strategy#1.3 · not in scope · stand card shadows sit in task CSS the delta does not edit
depth-strategy#1.6 · not in scope · the default dialog backdrop is untouched by the delta
depth-strategy#2.1 · not in scope · paint-child building is not named by the delta
depth-strategy#2.2 · not in scope · the stacking-context rule is not named by the delta
depth-strategy#3.1 · carried · `f102663b8` edits hit testing (`hit_inner`), the ground of escher's clip stop
depth-strategy#4.1 · not in scope · `z-index: auto` paint order is not named by the delta
depth-strategy#4.2 · not in scope · outset shadows are not named by the delta
border-radius#1.3 · not in scope · stand radii sit in task CSS the delta does not edit
border-radius#1.5 · not in scope · the default button radius is untouched by the delta
border-radius#1.7 · not in scope · the scrollbar thumb radius is not named by the delta
border-radius#2.1 · not in scope · corner-radius resolution is not named by the delta
motion#1.2 · carried · smooth scrolling lives in `scrolling.rs`, a predicted conflict and part of CARRY 4's clock surface
motion#1.3 · carried · `286874cc1` syncs dioxus-native-dom's `events.rs`, where programmatic scroll behaviour is read
motion#1.4 · carried · `scroll_into_view` in `scrolling.rs` is the driver's `scroll`, which must come through the conflict unchanged
motion#1.5 · not in scope · the fling's clock read is covered by the clock binding carried from motion#3.1
motion#1.6 · not in scope · the scrollbar fade's clock read is covered by the clock binding carried from motion#3.1
motion#2.1 · carried · `1c5265a8b` changes animation-set lookups in the animation run that style resolution drives
motion#2.2 · carried · the document's animating set, which settle reads, is what `1c5265a8b` touches
motion#3.1 · carried · the virtual-clock reading is CARRY 4's invariant, re-read on the take's tree
motion#4.1 · not in scope · the load wait is escher-driver's, and the delta does not reach that crate
motion#4.3 · carried · `286874cc1` replaces `events.rs`, where animation event data is `unimplemented!()`
iconography#1.1 · not in scope · the cursor choice's functions are not named by the delta's `document.rs` hunks
iconography#1.2 · not in scope · the cursor keyword mapping is not named by the delta
surface#1.1 · carried · `fb297a38c` edits `config.rs`, where `DocumentConfig::default()` adds the UA sheet
surface#1.2 · carried · the select's `default.css` rules are part of CARRY 5's select model
surface#1.3 · not in scope · the window's initial theme is not named by the delta
surface#1.4 · not in scope · the default viewport is not named by the delta
surface#6.3 · not in scope · the compositing alpha mode is not named by the delta
surface#7.2 · not in scope · the `scrollbars` feature is not named by the delta
surface#15.1 · carried · the `escher-session` output contract is an escher proof that must still pass after the take
