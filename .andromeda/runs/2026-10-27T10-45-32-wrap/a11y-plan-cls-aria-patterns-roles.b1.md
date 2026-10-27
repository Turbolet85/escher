# Classified — a11y-plan · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · the landmark role mapping in blitz-dom's accessibility builder is unchanged, and accessibility_roles.rs is not a reached file
aria-patterns-roles#1.2 · no · the header and section landmark mapping is the same unchanged builder code, and accessibility_roles.rs is not a reached file
aria-patterns-roles#2.1 · no · role_from_name in accessibility.rs is not a reached file, and the task keeps the role derivation unchanged
aria-patterns-roles#2.2 · no · the ARIA state reads aria-selected, aria-pressed, aria-checked and aria-invalid are listed untouched, and accessibility.rs is not a reached file
aria-patterns-roles#2.3 · no · the button role mapping is unchanged builder code, and neither accessibility.rs nor accessibility_roles.rs is a reached file
aria-patterns-roles#2.4 · no · the href gate on the link role is unchanged builder code outside the reached files
aria-patterns-roles#2.5 · no · the select role split at accessibility.rs:475-492 is unchanged, and the re-read select model moves no role
aria-patterns-roles#2.6 · no · the list and table role mapping is unchanged builder code outside the reached files
aria-patterns-roles#2.7 · no · the input type role mapping at accessibility.rs:509-527 is unchanged, and the IME-on-construct change moves no role
aria-patterns-roles#2.8 · no · the generic, heading and paragraph role mapping is unchanged builder code outside the reached files
aria-patterns-roles#2.9 · yes · add-to · the CSS accessors make a script's element.style.display or visibility write land as inline style, which reaches this hidden-content exclusion (the task's indirect touch on §4)
aria-patterns-roles#3.1 · no · its cited icons.rs, tab_strip.rs and toolbar.rs are not reached files, and this entity cites neither main.rs nor tab.rs
aria-patterns-roles#3.2 · yes · stale · apps/browser/src/tab.rs, cited at :289 for the empty favicon alt, is a merge surface reached whole, so that line can have moved and no report line shows it still holds
aria-patterns-roles#3.3 · no · the rdme stylesheet is not a reached file, and its [role=button] and [role=tabpanel] focus rules key on role attributes the session does not change
aria-patterns-roles#4.1 · no · the GitHub fixtures under examples/assets are not reached, and the session changes none of the markup or ARIA reads they show
aria-patterns-roles#4.2 · no · examples/assets/bottom_only.html is not a reached file, and its contentinfo role is unchanged
aria-patterns-roles#4.3 · no · the google.html combobox markup is not a reached file, and no ARIA read it relies on changed
aria-patterns-roles#4.4 · no · the google.html listbox, option and menu roles are not a reached file, and the role mapping they rely on is unchanged
aria-patterns-roles#4.5 · no · the google.html navigation, search and contentinfo roles are not a reached file, and the landmark mapping is unchanged
aria-patterns-roles#4.6 · no · servo-new.html is not a reached file, its aria-expanded reading is unchanged, and its script is cited inline at :232-239 so no file: script load reaches it
aria-patterns-roles#4.7 · no · the servo-new.html nav and section aria-label markup is not a reached file, and the label reads are unchanged
aria-patterns-roles#4.8 · no · the graphite.html carousel aria-label text is not a reached file, and the name reads are unchanged
aria-patterns-roles#4.9 · no · servo.html is not a reached file, and the role, aria-label, aria-expanded and aria-hidden markup it carries is unchanged by the session
aria-patterns-roles#4.10 · no · the servo.html download links with role="button" are not a reached file, and the button role mapping is unchanged
aria-patterns-roles#4.11 · no · svg.html is not a reached file, and the CSS accessor route reaches display and visibility, not the aria-hidden read these icons carry
