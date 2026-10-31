# Classified — a11y-plan · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · the landmark pins in untouched accessibility_roles.rs still hold and the change moves no nav, main or aside role
aria-patterns-roles#1.2 · no · the header and section pins in untouched accessibility_roles.rs still hold and no role mapping moved
aria-patterns-roles#2.1 · yes · stale · the citation accessibility.rs:357-420 sits past the insert at 238-242 and moves
aria-patterns-roles#2.2 · yes · stale · the citation accessibility.rs:422-442 sits past the insert at 238-242 and moves; the aria-selected, aria-pressed and aria-invalid reads are unchanged
aria-patterns-roles#2.3 · no · button and submit map through the untouched accessibility_roles.rs pins, and the button Enter and Space row is unmoved
aria-patterns-roles#2.4 · yes · stale · the citation accessibility.rs:502-519 sits past the insert at 238-242 and moves
aria-patterns-roles#2.5 · yes · add-to · F13: a single select is a ComboBox with expanded written from the open list and the chosen option selected; its citation accessibility.rs:502-519 also moves
aria-patterns-roles#2.6 · no · the multiple select's ListBox and set_multiselectable write are unchanged, F11 keeps its other row semantics, and size is read only as a height hint
aria-patterns-roles#2.7 · yes · stale · a group's label is now painted as a header on an open list, so the unpainted reading is stale, and F14 adds the header row beside it
aria-patterns-roles#2.8 · yes · stale · the citation accessibility.rs:502-519 sits past the insert at 238-242 and moves
aria-patterns-roles#2.9 · yes · stale · the citation accessibility.rs:537-555 sits past the insert at 238-242 and moves
aria-patterns-roles#2.10 · no · div, h2 and p map through the untouched accessibility_roles.rs pins, which the change does not reach
aria-patterns-roles#2.11 · yes · stale · the citations accessibility.rs:284-285 and 287-300 sit past the insert at 238-242 and move, while 178-181 above it holds
aria-patterns-roles#3.1 · no · the apps/browser icon, tab and toolbar files are not in the reached files
aria-patterns-roles#3.2 · no · the apps/browser image alt files are not in the reached files
aria-patterns-roles#3.3 · no · apps/readme github-markdown.css is not reached, and default.css is a different file
aria-patterns-roles#4.1 · no · the GitHub fixture under examples/assets is unedited
aria-patterns-roles#4.2 · no · the google fixture under examples/assets is unedited
aria-patterns-roles#4.3 · no · the google fixture under examples/assets is unedited
aria-patterns-roles#4.4 · no · the google fixture under examples/assets is unedited
aria-patterns-roles#4.5 · no · the google fixture under examples/assets is unedited
aria-patterns-roles#4.6 · no · the servo-new hamburger is a button, not a select, and examples/assets is unedited
aria-patterns-roles#4.7 · no · the servo-new fixture under examples/assets is unedited
aria-patterns-roles#4.8 · no · the graphite fixture under examples/assets is unedited
aria-patterns-roles#4.9 · no · servo.html under examples/assets is unedited, and its burger is a link, not a select
