# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · the landmark role mapping is what the Not-needed list excludes (the chunk adds no role and changes no mapping), and no Needs item reads a landmark
aria-patterns-roles#1.2 · no · header→Header and section→Section are the landmark mapping the Not-needed list excludes, and no Needs item reads a header or section
aria-patterns-roles#2.1 · yes · role_from_name's role=switch→Switch and its checkbox, radio and menu-item roles are what the Needs' role-switch mapping and the five-checkable-roles aria-checked witness rest on
aria-patterns-roles#2.2 · yes · the Needs name these state reads as v020-03 witnesses: aria-checked toggled on the five checkable roles alone, aria-pressed elsewhere, a native checkbox or radio reading neither, aria-selected and aria-invalid
aria-patterns-roles#2.3 · no · button→Button is the HTML-to-AccessKit role mapping the Not-needed list excludes, and the Needs read a button's focus, disabled focusability and aria-pressed, none resting on the Button role
aria-patterns-roles#2.4 · no · `<a>` as Link is the HTML-to-AccessKit role mapping the Not-needed list excludes, and no Needs item names a link
aria-patterns-roles#2.5 · no · select→ComboBox is the HTML-to-AccessKit role mapping the Not-needed list excludes, and the Needs read the select's keyboard model, its options' chosen state and its snapshot value, none resting on the role
aria-patterns-roles#2.6 · no · list, table and row mapping is the HTML-to-AccessKit role mapping the Not-needed list excludes, and the CRUD rows the Needs name are read by aria-selected, which 2.2 reads on any element
aria-patterns-roles#2.7 · yes · the Needs' native-checkbox-or-radio reading-neither witness rests on checkbox→CheckBox and radio→RadioButton, two of the five checkable roles that otherwise read aria-checked
aria-patterns-roles#2.8 · no · div→GenericContainer, h2→Heading and p→Paragraph are the HTML-to-AccessKit role mapping the Not-needed list excludes, and the Needs read ids, states and contrast on these elements, not their roles
aria-patterns-roles#2.9 · yes · the Needs name display none, visibility hidden and inert excluding nodes from the tree and the snapshot, and a select's option kept under display none, the rule this entity states
aria-patterns-roles#3.1 · no · apps/browser is not reached, and the chunk's focus witnesses sit on stand and fixture controls, not on the browser app's clickable divs
aria-patterns-roles#3.2 · no · text alternatives are a Not-needed area (no requirement in reach and no check this chunk adds reads them), and apps/browser is not reached
aria-patterns-roles#3.3 · no · focus rings are a Not-needed area and apps/readme is not reached, so rdme's focus-state styles bind nothing this chunk proves
aria-patterns-roles#4.1 · no · a captured GitHub page the chunk does not reach, and its aria-labelledby is an upstream name read the Not-needed list excludes
aria-patterns-roles#4.2 · no · a captured Google page the chunk does not reach, and its contentinfo is a landmark mapping the Not-needed list excludes
aria-patterns-roles#4.3 · no · a captured Google page's combobox markup (aria-autocomplete, aria-controls, aria-owns, aria-haspopup) that the chunk does not reach and no Needs item names
aria-patterns-roles#4.4 · no · a captured Google page's listbox, option and menu roles, which the chunk does not reach and no Needs item names
aria-patterns-roles#4.5 · no · a captured Google page's landmark regions, a landmark mapping the Not-needed list excludes, on a page the chunk does not reach
aria-patterns-roles#4.6 · no · a captured servo-new page the chunk does not reach, and its aria-expanded is an upstream read the Not-needed list excludes
aria-patterns-roles#4.7 · no · a captured servo-new page the chunk does not reach, and its aria-label names are text alternatives the Not-needed list excludes
aria-patterns-roles#4.8 · no · a captured graphite page the chunk does not reach, and its aria-label carousel names are text alternatives the Not-needed list excludes
aria-patterns-roles#4.9 · no · a captured servo.html page the chunk does not reach, and its aria-expanded is an upstream read the Not-needed list excludes
aria-patterns-roles#4.10 · no · a captured servo.html page's role=button download links, which the chunk does not reach and no Needs item names
aria-patterns-roles#4.11 · no · a captured svg.html page the chunk does not reach, and focusable=false on its SVG icons is named by no Needs item
