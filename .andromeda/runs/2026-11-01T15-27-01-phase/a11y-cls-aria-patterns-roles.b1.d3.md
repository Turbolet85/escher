# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · yes · landmark role mapping (nav, main, aside, footer, article, blockquote, figure) is witnessed by accessibility_roles.rs:74-93, one of the re-read `accessibility_*` checks
aria-patterns-roles#1.2 · yes · header and section role mapping is witnessed by accessibility_roles.rs:157-180, a re-read `accessibility_*` check
aria-patterns-roles#2.1 · no · the `role=` attribute mapping lives in packages/blitz-dom/src/accessibility.rs, which the chunk does not reach, and no re-read check it names witnesses it
aria-patterns-roles#2.2 · yes · aria-selected, aria-pressed, aria-invalid and aria-checked states and role=switch are pinned by accessibility_states.rs, a re-read `accessibility_*` check, and the select option's `selected` state is in the select model the scope names
aria-patterns-roles#2.3 · yes · button and submit mapping to Button is witnessed by accessibility_roles.rs:128-155 and :157-180, re-read checks
aria-patterns-roles#2.4 · yes · `<a>` is a Link only with href, witnessed by accessibility_roles.rs:117-126, a re-read check
aria-patterns-roles#2.5 · yes · `<select multiple>` ListBox versus ComboBox is witnessed by accessibility_roles.rs:128-155, and the select's ComboBox side is named in the scope
aria-patterns-roles#2.6 · yes · a single select's ComboBox with `expanded` from the list's open state is named in the scope (ComboBox and expanded in the tree, asserted by select_popup and stand_act_popup) and goes through the mutator.rs and node.rs conflicts
aria-patterns-roles#2.7 · no · the `multiple` select's `set_multiselectable` write lives in accessibility.rs, which the chunk does not reach, and the scope names only the single-choice ComboBox side of the select model
aria-patterns-roles#2.8 · yes · optgroup as Group, with its label named in the tree and painted as a header on the open list, is in the select list's merge surface and the text.rs painted-header scope
aria-patterns-roles#2.9 · yes · a header row choosing nothing on an open list is a rule over the list's keys and clicks, which the scope names, and the select list's merge surface is re-read row by row
aria-patterns-roles#2.10 · yes · list, table and row-group role mapping is witnessed by accessibility_roles.rs:95-115, a re-read check
aria-patterns-roles#2.11 · yes · form input role mapping by `type`, including PasswordInput under the masking scope, is witnessed by accessibility_roles.rs:128-155 and :157-180, re-read checks
aria-patterns-roles#2.12 · yes · div, h2 and p role mapping is witnessed by accessibility_roles.rs:157-180, a re-read check
aria-patterns-roles#2.13 · yes · hidden content (aria-hidden, hidden, display none, visibility hidden) and the select option exception are the hidden-content and inert scope, witnessed by accessibility_hidden.rs:28-137, a re-read check
aria-patterns-roles#3.1 · no · describes clickable divs in apps/browser, which the chunk does not reach or test
aria-patterns-roles#3.2 · no · describes image alt text in apps/browser, which the chunk does not reach or test
aria-patterns-roles#3.3 · no · describes focus styling in apps/readme's stylesheet, which the chunk does not reach or test
aria-patterns-roles#4.1 · no · records attributes in the GitHub fixture, a file the chunk does not name or re-read
aria-patterns-roles#4.2 · no · records a role in the Google fixture, a file the chunk does not name or re-read
aria-patterns-roles#4.3 · no · records a combobox in the Google fixture, a file the chunk does not name or re-read
aria-patterns-roles#4.4 · no · records listbox and menu roles in the Google fixture, a file the chunk does not name or re-read
aria-patterns-roles#4.5 · no · records landmark roles in the Google fixture, a file the chunk does not name or re-read
aria-patterns-roles#4.6 · no · records aria-expanded in the servo-new fixture, a file the chunk does not name or re-read
aria-patterns-roles#4.7 · no · records aria-label on nav and sections in the servo-new fixture, a file the chunk does not name or re-read
