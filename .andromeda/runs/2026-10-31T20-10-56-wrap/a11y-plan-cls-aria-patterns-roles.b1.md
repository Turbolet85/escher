# Classified — a11y-plan · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · landmark pins sit in untouched accessibility_roles.rs, and the session's accessibility.rs edits concern only a single select's expanded and select bounds
aria-patterns-roles#1.2 · no · header and section pins sit in untouched accessibility_roles.rs, and the session's accessibility.rs edits concern only a single select's expanded and select bounds
aria-patterns-roles#2.1 · yes · stale · the accessibility.rs insert at 238-242 moves the role_from_name citation 357-420
aria-patterns-roles#2.2 · yes · stale · the accessibility.rs insert moves the citation 422-442, and a single select's aria-expanded is no longer read, so "read by value on any element" needs that exception
aria-patterns-roles#2.3 · no · button and submit pins sit in untouched accessibility_roles.rs and cite no accessibility.rs line
aria-patterns-roles#2.4 · yes · stale · the accessibility.rs insert at 238-242 moves the citation 502-519
aria-patterns-roles#2.5 · yes · add-to · F13: a single select is ComboBox with expanded written from the list's open state and its chosen option selected; the accessibility.rs citation 502-519 also moves
aria-patterns-roles#2.6 · no · the session's accessibility.rs edits write expanded for a single select only, so a multiple select's ListBox and set_multiselectable write stand
aria-patterns-roles#2.7 · yes · contradict · a group's label is now painted as a header on an open list, so "unpainted" no longer holds (F14 adds the header row and the no-label box)
aria-patterns-roles#2.8 · yes · stale · the accessibility.rs insert at 238-242 moves the citation 502-519
aria-patterns-roles#2.9 · yes · stale · the accessibility.rs insert at 238-242 moves the citation 537-555
aria-patterns-roles#2.10 · no · div, h2 and p pins sit in untouched accessibility_roles.rs and cite no accessibility.rs line
aria-patterns-roles#2.11 · yes · stale · the accessibility.rs insert at 238-242 moves the citations 284-285 and 287-300
aria-patterns-roles#3.1 · no · the apps/browser icon controls are not in the reached files
aria-patterns-roles#3.2 · no · the apps/browser images are not in the reached files
aria-patterns-roles#3.3 · no · the apps/readme stylesheet is not in the reached files
aria-patterns-roles#4.1 · no · the GitHub fixtures under examples/assets were not edited
aria-patterns-roles#4.2 · no · the Google fixture under examples/assets was not edited
aria-patterns-roles#4.3 · no · the Google fixture under examples/assets was not edited
aria-patterns-roles#4.4 · no · the Google fixture under examples/assets was not edited
aria-patterns-roles#4.5 · no · the Google fixture under examples/assets was not edited
aria-patterns-roles#4.6 · no · the servo-new fixture under examples/assets was not edited
aria-patterns-roles#4.7 · no · the servo-new fixture under examples/assets was not edited
aria-patterns-roles#4.8 · no · the graphite fixture under examples/assets was not edited
aria-patterns-roles#4.9 · no · the servo fixture under examples/assets was not edited
