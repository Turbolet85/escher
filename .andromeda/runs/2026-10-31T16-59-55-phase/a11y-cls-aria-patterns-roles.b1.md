# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · landmark roles nav, main and the rest sit outside the list's reach; accessibility_roles.rs is not reached
aria-patterns-roles#1.2 · no · header and section roles sit outside the list's reach; accessibility_roles.rs is not reached
aria-patterns-roles#2.1 · no · the chunk adds no `role` attribute path, and the options read their roles from the tree, not a role mapping
aria-patterns-roles#2.2 · yes · the option reads its chosen state as `selected` (accessibility.rs:184-194), and the tree reads `expanded` from `aria-expanded` alone, both named in Needs
aria-patterns-roles#2.3 · no · button roles sit outside the list's reach
aria-patterns-roles#2.4 · no · link roles sit outside the list's reach
aria-patterns-roles#2.5 · yes · a closed single select reads ComboBox, the role the open list and the stand's `select#flight-type` readings hold; the `multiple` ListBox branch stays as built
aria-patterns-roles#2.6 · yes · Needs names a `multiple` select's `set_multiselectable` as built, which the chunk keeps
aria-patterns-roles#2.7 · yes · Needs names the optgroup's Role::Group and its label as name, and the chunk paints or reads that label
aria-patterns-roles#2.8 · no · list and table roles sit outside the list's reach
aria-patterns-roles#2.9 · no · form input roles, date inputs included, sit outside the list's reach; the chunk changes no input
aria-patterns-roles#2.10 · no · generic and text roles sit outside the list's reach
aria-patterns-roles#2.11 · yes · Needs names the option carve-out from display none, with visibility hidden and inert still excluding it, and the list must not change which options the tree holds
aria-patterns-roles#3.1 · no · browser controls under apps/browser, which the chunk does not reach
aria-patterns-roles#3.2 · no · browser image alt text under apps/browser, which the chunk does not reach
aria-patterns-roles#3.3 · no · readme stylesheet focus states under apps/readme, which the chunk does not reach
aria-patterns-roles#4.1 · no · captured GitHub fixture markup, which the chunk edits none of
aria-patterns-roles#4.2 · no · Google fixture markup, which the chunk edits none of
aria-patterns-roles#4.3 · no · Google fixture markup, which the chunk edits none of
aria-patterns-roles#4.4 · no · Google fixture markup, which the chunk edits none of
aria-patterns-roles#4.5 · no · Google fixture markup, which the chunk edits none of
aria-patterns-roles#4.6 · no · servo-new fixture markup, which the chunk edits none of
aria-patterns-roles#4.7 · no · servo-new fixture markup, which the chunk edits none of
aria-patterns-roles#4.8 · no · graphite fixture markup, which the chunk edits none of
aria-patterns-roles#4.9 · no · servo fixture markup, which the chunk edits none of
