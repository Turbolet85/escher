# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · yes · nav, main, aside, footer, article, blockquote and figure landmark roles, witnessed by accessibility_roles.rs:74-93, which the chunk re-reads under accessibility_*
aria-patterns-roles#1.2 · yes · header and section landmark roles, witnessed by accessibility_roles.rs:157-180, re-read under accessibility_*
aria-patterns-roles#2.1 · yes · the role attribute's mapping to AccessKit roles is the tree's role identity that the re-read accessibility_roles checks cover; accessibility.rs is not reached, so it binds through that check
aria-patterns-roles#2.2 · yes · aria-selected, aria-pressed, aria-invalid, aria-checked and role=switch state values, pinned by accessibility_states.rs, re-read under accessibility_*
aria-patterns-roles#2.3 · yes · button and submit map to Button, witnessed by accessibility_roles.rs:128-180, re-read under accessibility_*
aria-patterns-roles#2.4 · yes · a link is a Link only with href, witnessed by accessibility_roles.rs:117-126, re-read under accessibility_*
aria-patterns-roles#2.5 · yes · select multiple is ListBox and a plain select is ComboBox, the select model's role rule witnessed by accessibility_roles.rs:128-155
aria-patterns-roles#2.6 · yes · a single select's ComboBox writes expanded from its list's open state, the select model's tree rule that select_popup and stand_act_popup assert, through the mutator and node.rs conflicts
aria-patterns-roles#2.7 · yes · a multiple select's ListBox writes set_multiselectable, a row of the engine-debts merge surfaces the chunk re-reads row by row
aria-patterns-roles#2.8 · yes · optgroup as Group with its label as name, and the header painted on an open list, which is the select list's rows and names the chunk re-reads
aria-patterns-roles#2.9 · yes · an option group's header row chooses nothing and a group without a label reads the whole list's box, the select list's keys and clicks through the mutator and node.rs conflicts, witnessed by select_popup.rs:805-855
aria-patterns-roles#2.10 · yes · ul, ol, li, table, thead, tr, th and td roles, witnessed by accessibility_roles.rs:95-115, re-read under accessibility_*
aria-patterns-roles#2.11 · yes · input roles by type, including PasswordInput for the masked password input, witnessed by accessibility_roles.rs:128-180
aria-patterns-roles#2.12 · yes · div, h2 and p roles, witnessed by accessibility_roles.rs:157-180, re-read under accessibility_*
aria-patterns-roles#2.13 · yes · the hidden-content rules (aria-hidden, hidden, display none, visibility hidden) and the select option exception, witnessed by accessibility_hidden.rs and the single select's hidden options the select model drives
aria-patterns-roles#3.1 · no · clickable divs in apps/browser sources; apps/browser is not a reached file and no named check reads it
aria-patterns-roles#3.2 · no · image alt text in apps/browser sources; apps/browser is not a reached file and no named check reads it
aria-patterns-roles#3.3 · no · apps/readme stylesheet focus states for [role=button] and [role=tabpanel]; apps/readme is not reached and no named check reads it
aria-patterns-roles#4.1 · no · captured GitHub fixture markup under examples/assets; fixtures are outside the reached files and the plan's Not needed excludes them
aria-patterns-roles#4.2 · no · Google fixture contentinfo markup under examples/assets; fixtures are outside the reached files and the plan's Not needed excludes them
aria-patterns-roles#4.3 · no · Google fixture combobox textarea markup under examples/assets; fixtures are outside the reached files and the plan's Not needed excludes them
aria-patterns-roles#4.4 · no · Google fixture listbox, option and menu markup under examples/assets; fixtures are outside the reached files and the plan's Not needed excludes them
aria-patterns-roles#4.5 · no · Google fixture landmark markup under examples/assets; fixtures are outside the reached files and the plan's Not needed excludes them
aria-patterns-roles#4.6 · no · servo-new fixture hamburger aria-expanded markup under examples/assets; fixtures are outside the reached files and the plan's Not needed excludes them
aria-patterns-roles#4.7 · no · servo-new fixture aria-label markup under examples/assets; fixtures are outside the reached files and the plan's Not needed excludes them
