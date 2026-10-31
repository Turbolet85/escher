# Classified — a11y · keyboard-navigation.b3

## Answers
keyboard-navigation#4.9 · no · an autofocus feature forward in the Cargo manifests; the chunk's focus contract is a P4 fork and reads no autofocus
keyboard-navigation#4.10 · no · IME event handling; the list adds no composition surface, and IME is not needed
keyboard-navigation#4.11 · yes · a falsy `disabled` is removed, so the disabled gate reads presence and the disabled refusal holds on `enabled` reading Some(false); a falsy `selected` on options and `multiple` carry the same removal
keyboard-navigation#5.1 · no · browser urlbar arrow keys; outside the select and the chunk's reached files
keyboard-navigation#5.2 · no · browser tab shortcuts; not reached
keyboard-navigation#5.3 · no · browser new-tab search input; not reached
keyboard-navigation#5.4 · no · rdme shortcuts; not reached
keyboard-navigation#5.5 · no · `use_back_button` in dioxus-native hooks; not reached
keyboard-navigation#5.6 · no · todomvc focus and keys; not reached
keyboard-navigation#5.7 · no · Cells commit on Enter or blur; not a select and not reached
keyboard-navigation#5.8 · no · Cells grid arrows; the Cells rows of stand_keyboard's tables are not edited, only select rows are added
keyboard-navigation#5.9 · no · Cells editor; not a select, and its stand_keyboard rows are not edited
keyboard-navigation#5.10 · yes · the tree's focus is the list and never the option, and the builder reads no `aria-activedescendant`, pinned by `accessibility_active_descendant_unread`, which bounds any option focus the fork gives
keyboard-navigation#5.11 · no · TodoMVC Preact autofocus; not reached
keyboard-navigation#5.12 · no · Android activity focus; not reached
keyboard-navigation#6.1 · no · docs.rs fixture tabindex on toolbar containers; the chunk edits no fixture markup
keyboard-navigation#6.2 · no · google fixture role and tabindex; the chunk edits no fixture markup
keyboard-navigation#6.3 · no · google fixture jsaction keydown bindings; fixture markup not edited
keyboard-navigation#6.4 · no · google fixture `:focus-visible` outline; a fixture stylesheet, not `default.css`, and not edited
keyboard-navigation#6.5 · no · graphite fixture newsletter focus border; fixture markup and CSS not edited
keyboard-navigation#6.6 · no · servo.css focus ring; a fixture stylesheet the chunk edits none of
keyboard-navigation#6.7 · no · servo headings tabindex -1; fixture markup not edited
keyboard-navigation#6.8 · no · label `for`/`id` and nesting for inputs; not the optgroup label the chunk paints
keyboard-navigation#7.1 · yes · the engine focus, the snapshot's `focused` and the tree's focus agree at each step, which the chunk's focus with a list open must keep
keyboard-navigation#7.2 · yes · stand_keyboard's `tab_order` and `ENABLED_TAB_ORDER` hold the select's Tab stop and the ArrowDown step on `select#flight-type`, which move with any new row or key step
