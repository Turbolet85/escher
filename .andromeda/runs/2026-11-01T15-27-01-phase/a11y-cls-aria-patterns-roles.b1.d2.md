# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · yes · landmark roles are asserted by accessibility_roles.rs:74-93, a file the chunk re-reads under accessibility_*
aria-patterns-roles#1.2 · yes · header and section roles are asserted by accessibility_roles.rs:157-180, re-read under accessibility_*
aria-patterns-roles#2.1 · no · role_from_name in accessibility.rs names no check and no reached file, and the chunk asserts no ARIA role attribute
aria-patterns-roles#2.2 · yes · aria-selected, aria-pressed, aria-invalid, aria-checked and role=switch are pinned by accessibility_states.rs, re-read under accessibility_*; the option's selected state is the select model
aria-patterns-roles#2.3 · yes · button and submit roles are asserted by accessibility_roles.rs:128-180, re-read under accessibility_*
aria-patterns-roles#2.4 · yes · a link only with href is asserted by accessibility_roles.rs:117-126, re-read under accessibility_*
aria-patterns-roles#2.5 · yes · select as ListBox or ComboBox by multiple is asserted by accessibility_roles.rs:128-155, and the select model's ComboBox is named in the Needs
aria-patterns-roles#2.6 · yes · single select ComboBox with expanded from the list's open state is the select model in the tree, asserted by select_popup.rs:565-614 and stand_act_popup
aria-patterns-roles#2.7 · yes · the multiple select's ListBox set_multiselectable is an engine-debts record, and the engine debts' merge surfaces are re-read row by row
aria-patterns-roles#2.8 · yes · optgroup as Group with its label, painted as the open list's header, sits in the select list's rows and the optgroup header weight the Needs name
aria-patterns-roles#2.9 · yes · the group header row chooses nothing and an unlabelled group reads the list's box, asserted by select_popup.rs:805-855 within the select list's rows re-read
aria-patterns-roles#2.10 · yes · list, table and cell roles are asserted by accessibility_roles.rs:95-115, re-read under accessibility_*
aria-patterns-roles#2.11 · yes · input roles by type, including password, are asserted by accessibility_roles.rs:128-180, re-read; the password masking the Needs name reads the same input
aria-patterns-roles#2.12 · yes · div, h2 and p roles are asserted by accessibility_roles.rs:157-180, re-read under accessibility_*
aria-patterns-roles#2.13 · yes · aria-hidden, hidden, display none and select-option exclusion are asserted by accessibility_hidden.rs:28-137, re-read; the default sheet's hidden-content values are named in the Needs
aria-patterns-roles#3.1 · no · clickable divs in the browser app; apps/browser is not reached and no chunk check covers it
aria-patterns-roles#3.2 · no · image alt text in the browser app; apps/browser is not reached and no chunk check covers it
aria-patterns-roles#3.3 · no · focus states in the readme app's stylesheet; apps/readme is not reached and no chunk check covers it
aria-patterns-roles#4.1 · no · captured GitHub fixture markup; the chunk names no such fixture and fixture records lie outside its files
aria-patterns-roles#4.2 · no · captured Google fixture markup; the chunk names no such fixture and fixture records lie outside its files
aria-patterns-roles#4.3 · no · captured Google fixture markup; the chunk names no such fixture and fixture records lie outside its files
aria-patterns-roles#4.4 · no · captured Google fixture markup; the chunk names no such fixture and fixture records lie outside its files
aria-patterns-roles#4.5 · no · captured Google fixture markup; the chunk names no such fixture and fixture records lie outside its files
aria-patterns-roles#4.6 · no · captured servo-new fixture markup; the chunk names no such fixture and fixture records lie outside its files
aria-patterns-roles#4.7 · no · captured servo-new fixture markup; the chunk names no such fixture and fixture records lie outside its files
