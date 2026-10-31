# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · landmark role mapping in accessibility.rs, a unit the chunk must only keep holding; re-check every change owes
aria-patterns-roles#1.2 · no · header and section landmark mapping, unnamed by the scope; re-check every change owes
aria-patterns-roles#2.1 · no · role attribute mapping in role_from_name, which the chunk neither reads nor changes; re-check every change owes
aria-patterns-roles#2.2 · yes · the option's chosen state read as `selected` and the `expanded` state from aria-expanded, both named by the scope, at accessibility.rs:184-194 and the option exception
aria-patterns-roles#2.3 · no · button role mapping, unnamed by the scope; re-check every change owes
aria-patterns-roles#2.4 · no · link role mapping on href, unnamed by the scope; re-check every change owes
aria-patterns-roles#2.5 · yes · the closed single select's ComboBox role, which the scope's closed single select and the stand tree's select#flight-type read
aria-patterns-roles#2.6 · yes · a multiple select's set_multiselectable, which the scope names as staying as built
aria-patterns-roles#2.7 · yes · optgroup as Role::Group with its non-empty label as name, which the scope names as this chunk's to paint or read
aria-patterns-roles#2.8 · no · list and table role mapping, unnamed by the scope; re-check every change owes
aria-patterns-roles#2.9 · no · input type role mapping, unnamed by the scope and the chunk adds no input; re-check every change owes
aria-patterns-roles#2.10 · no · generic and text role mapping, unnamed by the scope; re-check every change owes
aria-patterns-roles#2.11 · yes · the option-under-select carve-out from display none, with hidden, inert and visibility still excluding, which the scope names
aria-patterns-roles#3.1 · no · browser app's clickable div controls, outside the reached files
aria-patterns-roles#3.2 · no · browser app images and alt text, outside the reached files
aria-patterns-roles#3.3 · no · readme app stylesheet focus states, outside the reached files
aria-patterns-roles#4.1 · no · GitHub fixture markup under examples/assets, which the chunk edits none of
aria-patterns-roles#4.2 · no · Google fixture markup under examples/assets, which the chunk edits none of
aria-patterns-roles#4.3 · no · Google fixture markup under examples/assets, which the chunk edits none of
aria-patterns-roles#4.4 · no · Google fixture markup under examples/assets, which the chunk edits none of
aria-patterns-roles#4.5 · no · Google fixture markup under examples/assets, which the chunk edits none of
aria-patterns-roles#4.6 · no · servo-new fixture markup under examples/assets, which the chunk edits none of
aria-patterns-roles#4.7 · no · servo-new fixture markup under examples/assets, which the chunk edits none of
aria-patterns-roles#4.8 · no · graphite fixture markup under examples/assets, which the chunk edits none of
aria-patterns-roles#4.9 · no · servo fixture markup under examples/assets, which the chunk edits none of
