# Classified — a11y-plan · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · landmark role mapping lives in accessibility.rs and accessibility_roles.rs, neither reached and both unedited
aria-patterns-roles#1.2 · no · header and section role mapping lives in accessibility.rs and accessibility_roles.rs, neither reached and both unedited
aria-patterns-roles#2.1 · no · role_from_name is in accessibility.rs, which holds no change
aria-patterns-roles#2.2 · no · ARIA state reads are in accessibility.rs and accessibility_states.rs, both unedited, and the reads still hold
aria-patterns-roles#2.3 · no · button role mapping is in accessibility.rs and accessibility_roles.rs, neither reached
aria-patterns-roles#2.4 · no · link role mapping is in accessibility.rs, which holds no change
aria-patterns-roles#2.5 · no · select role mapping is in accessibility.rs, which holds no change
aria-patterns-roles#2.6 · yes · stale · select_popup.rs lost the row-3 skip and its comment, so the cited span 561-610 moved past line 513
aria-patterns-roles#2.7 · no · set_multiselectable is written in accessibility.rs, which holds no change
aria-patterns-roles#2.8 · no · optgroup mapping is in accessibility.rs, which holds no change, and the open list's painting reads as recorded on both builds
aria-patterns-roles#2.9 · yes · stale · document.rs gained 290 lines in 13 ranges from line 45, and select_popup.rs lost lines above 801, so both cited spans moved
aria-patterns-roles#2.10 · no · list and table role mapping is in accessibility.rs and accessibility_roles.rs, neither reached
aria-patterns-roles#2.11 · no · form input role mapping is in accessibility.rs, which holds no change
aria-patterns-roles#2.12 · no · generic and text role mapping is in accessibility.rs, which holds no change
aria-patterns-roles#2.13 · no · accessibility.rs and accessibility_hidden.rs hold no edit, and the a11y leg holds green under the restyle narrowing, so the hidden exclusion still holds
aria-patterns-roles#3.1 · no · browser icon controls are in apps/browser, which is not reached
aria-patterns-roles#3.2 · no · browser image alt text is in apps/browser, which is not reached
aria-patterns-roles#3.3 · no · rdme focus styles are in apps/readme/assets, which is not reached
aria-patterns-roles#4.1 · no · GitHub fixture markup is in examples/assets, which is not reached
aria-patterns-roles#4.2 · no · Google fixture contentinfo is in examples/assets, which is not reached
aria-patterns-roles#4.3 · no · Google fixture combobox markup is in examples/assets, which is not reached
aria-patterns-roles#4.4 · no · Google fixture listbox and menu markup is in examples/assets, which is not reached
aria-patterns-roles#4.5 · no · Google fixture landmark markup is in examples/assets, which is not reached
aria-patterns-roles#4.6 · no · servo-new fixture and its script are in examples/assets, which is not reached
aria-patterns-roles#4.7 · no · servo-new fixture aria-label markup is in examples/assets, which is not reached
