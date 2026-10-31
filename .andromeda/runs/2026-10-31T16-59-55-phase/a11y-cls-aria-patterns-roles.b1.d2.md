# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · landmark mapping of nav, main, aside, footer, article, blockquote and figure; the chunk adds no region (Not needed), so it is a re-check every change owes
aria-patterns-roles#1.2 · no · landmark mapping of header and section; the chunk adds no region (Not needed), so it is a re-check every change owes
aria-patterns-roles#2.1 · no · role_from_name's ARIA role attribute mapping; the chunk writes no role attribute path and the option and select roles come from tag mapping, so it is a re-check every change owes
aria-patterns-roles#2.2 · yes · an option of a select is excepted from aria-selected reading and its selected is its chosen state, which the chunk's options must read (Needs: hidden-content carve-out and option selected)
aria-patterns-roles#2.3 · no · button and submit role mapping; the chunk adds no button
aria-patterns-roles#2.4 · no · link role mapping; the chunk adds no link
aria-patterns-roles#2.5 · yes · a single select is ComboBox and a multiple select ListBox; the scope names the closed single select's open state (expanded) and the multiple select's set_multiselectable, which must stay as built
aria-patterns-roles#2.6 · yes · a multiple select's ListBox writes set_multiselectable; the scope names it and requires it to stay as built
aria-patterns-roles#2.7 · yes · optgroup reads Role::Group with its non-empty label as name, named in the tree and unpainted; the scope paints or reads a group's label
aria-patterns-roles#2.8 · no · list, table and cell role mapping; the chunk adds no list or table and an option is not an li
aria-patterns-roles#2.9 · no · input role mapping by type; the chunk adds no input and the stand rows it moves are select#flight-type's, whose role is #2.5's rule
aria-patterns-roles#2.10 · no · div, h2 and p role mapping; the chunk adds no heading and no generic node the scope names
aria-patterns-roles#2.11 · yes · an option of a select is excepted from display: none, while hidden, visibility: hidden and inert still exclude it; the scope's hidden-content carve-out and the rule that the list must not change which options the tree holds
aria-patterns-roles#3.1 · no · browser app clickable divs in apps/browser; the chunk reaches no apps/browser file
aria-patterns-roles#3.2 · no · browser app image alt text in apps/browser; text alternatives in fixtures and the browser are Not needed, and the chunk adds no image
aria-patterns-roles#3.3 · no · rdme stylesheet focus states for role=button and role=tabpanel in apps/readme; the chunk reaches no apps/readme file
aria-patterns-roles#4.1 · no · captured GitHub fixture markup under examples/assets; the chunk edits no fixture markup (Not needed)
aria-patterns-roles#4.2 · no · Google fixture markup under examples/assets; the chunk edits no fixture markup (Not needed)
aria-patterns-roles#4.3 · no · Google fixture query textarea markup under examples/assets; the chunk edits no fixture markup (Not needed)
aria-patterns-roles#4.4 · no · Google fixture listbox and option markup under examples/assets; the chunk's options are native option elements and it edits no fixture markup (Not needed)
aria-patterns-roles#4.5 · no · Google fixture landmark markup under examples/assets; the chunk edits no fixture markup and adds no region (Not needed)
aria-patterns-roles#4.6 · no · servo-new fixture hamburger markup under examples/assets; the chunk edits no fixture markup (Not needed)
aria-patterns-roles#4.7 · no · servo-new fixture nav and section labels under examples/assets; the chunk edits no fixture markup (Not needed)
aria-patterns-roles#4.8 · no · graphite fixture carousel button labels under examples/assets; the chunk edits no fixture markup (Not needed)
aria-patterns-roles#4.9 · no · servo.html fixture nav and burger markup under examples/assets; the chunk edits no fixture markup (Not needed)
