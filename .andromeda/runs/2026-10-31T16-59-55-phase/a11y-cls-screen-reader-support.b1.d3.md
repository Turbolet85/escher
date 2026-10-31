# Classified — a11y · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · yes · option names read the text runs of the open list's options and of the flight booker's option texts; accessibility.rs is reached whole
screen-reader-support#1.2 · yes · the flight booker's `select#flight-type` is named `Flight type` by `aria-label`, a name the stand tables re-read with any new row
screen-reader-support#1.3 · no · `<label>` binding names no select the chunk touches; the flight booker's select is named by `aria-label`, and the fx-pick page is not a unit the chunk edits
screen-reader-support#1.4 · yes · the option readings the chunk changes: a closed select's options read the select's client rect as `bounds`, the open list's options read as `ListBoxOption` with `selected`, and the flight-type readings
screen-reader-support#1.5 · yes · a `multiple` select's rows keep their own client rects as `bounds`, which the closed-option `bounds` change must not move
screen-reader-support#1.6 · yes · a `multiple` select's row click and its `input` and `change` values stay as built, `value` staying `select_value`
screen-reader-support#2.1 · yes · the open list's options become element nodes that must carry their element id as `author_id`, and the tree-equality and stand control checks read the `ComboBox` and `ListBoxOption` roles and names
screen-reader-support#3.1 · yes · the snapshot reads an option's role, name and focus from the tree and `enabled` and `value` from the element, with no second mapping; the chunk edits the options' `bounds` in snapshot.rs and the `disabled` refusal holds on `enabled`
screen-reader-support#3.2 · no · native checkbox and radio `checked` and `toggled` reading; the chunk adds no checkable control
screen-reader-support#4.1 · yes · options made actionable through the driver fall under the actionable-key rule on `select#flight-type`, where the check must still return nothing
screen-reader-support#4.2 · no · `cells-grid` names no element the chunk touches
screen-reader-support#4.3 · no · the load page holds no select or option, and no row the chunk re-pins or re-measures reads it
screen-reader-support#5.1 · yes · the flight booker's mode is `select#flight-type`, whose options state the choice by `selected`, and the `aria-activedescendant` unread pin bounds any tree focus an option is given
screen-reader-support#6.1 · yes · the builder's `option` carve-out under `select` and its role reading key on the element's HTML tag name
screen-reader-support#6.2 · yes · tree focus names only built nodes, else the window; an option the fork gives focus must be a built node and agree with the engine's focus and the snapshot's `focused`
screen-reader-support#6.3 · no · a purpose statement of the role-mapping test, which the chunk neither edits nor reads
screen-reader-support#7.1 · no · `accesskit_xplat` event routing; the adapters are listed as not needed and the chunk builds none
screen-reader-support#8.1 · no · the AccessKit adapter's creation from the window handle; listed as not needed
screen-reader-support#8.2 · no · the `accessibility` feature gate of screen-reader integration; the chunk adds no feature and the windowed list is an engine overlay, not an assistive-technology surface
screen-reader-support#9.1 · no · `lang` and `xml:lang` inheritance; language and direction are listed as not needed, the list adds no document language
screen-reader-support#9.2 · no · `dir` bidi selectors in default.css; language and direction are listed as not needed, and the overlay's styles do not touch `dir`
screen-reader-support#9.3 · no · fixture `lang` attributes under examples/assets; the chunk edits no fixture markup
screen-reader-support#9.4 · no · `lang="en"` on example pages; the chunk edits no page markup
screen-reader-support#10.1 · no · absence of an html lang attribute in the s02 fixtures; the chunk edits no fixture markup
screen-reader-support#11.1 · no · an `sr-only` tooltip in the GitHub fixture; text alternatives and hidden decoration in fixtures are listed as not needed
</content>
