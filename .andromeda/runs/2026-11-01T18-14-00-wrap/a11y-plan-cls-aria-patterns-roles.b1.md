# Classified — a11y-plan · aria-patterns-roles.b1

## Answers

aria-patterns-roles#1.1 · no · the landmark mapping sits in accessibility.rs and accessibility_roles.rs, neither reached, and the session changes no role derivation
aria-patterns-roles#1.2 · no · the header and section mapping sits in the same untouched builder and roles test, no hunk reaches it
aria-patterns-roles#2.1 · no · role_from_name in accessibility.rs is untouched by the session and the claim still holds
aria-patterns-roles#2.2 · no · the aria state reads sit in accessibility.rs and accessibility_states.rs, both unedited, and the builder reads attributes by value as before
aria-patterns-roles#2.3 · no · the button mapping sits in accessibility.rs and accessibility_roles.rs, neither reached
aria-patterns-roles#2.4 · no · the link rule sits in accessibility.rs, untouched by the session
aria-patterns-roles#2.5 · no · the select role mapping sits in accessibility.rs and accessibility_roles.rs, neither reached
aria-patterns-roles#2.6 · yes · stale · select_popup.rs lost its row-3 skip so line 513 is one harness.press, and the citation select_popup.rs:561-610 sits past it and moves
aria-patterns-roles#2.7 · no · the multiple-select ListBox write sits in accessibility.rs, untouched by the session
aria-patterns-roles#2.8 · no · the optgroup Group mapping sits in accessibility.rs, untouched by the session
aria-patterns-roles#2.9 · yes · stale · document.rs (a merge surface, 290 lines added from line 45 on) and select_popup.rs (row-3 skip gone) move the citations document.rs:1006-1023 and select_popup.rs:801-851
aria-patterns-roles#2.10 · no · the list and table role mapping sits in accessibility.rs and accessibility_roles.rs, neither reached, and table damage is layout only
aria-patterns-roles#2.11 · no · the input type mapping sits in accessibility.rs and accessibility_roles.rs, both untouched
aria-patterns-roles#2.12 · no · the generic and text mapping sits in the untouched builder and roles test
aria-patterns-roles#2.13 · no · the hidden exclusion still holds with accessibility.rs and accessibility_hidden.rs unedited, and the restyle narrowing changes no exclusion rule this entity states
aria-patterns-roles#3.1 · no · the browser app's clickable div controls sit in apps/browser, which no reached file touches
aria-patterns-roles#3.2 · no · the browser image alt text sits in apps/browser, not reached
aria-patterns-roles#3.3 · no · the focus-state rules sit in apps/readme's github-markdown.css, not reached; blitz-dom's default.css change is a different sheet
aria-patterns-roles#4.1 · no · the GitHub fixture attributes sit in examples/assets, which the session does not touch
aria-patterns-roles#4.2 · no · the Google contentinfo role sits in examples/assets, not reached
aria-patterns-roles#4.3 · no · the Google textarea combobox attributes sit in examples/assets, not reached
aria-patterns-roles#4.4 · no · the Google listbox and menu roles sit in examples/assets, not reached
aria-patterns-roles#4.5 · no · the Google landmark roles sit in examples/assets, not reached
aria-patterns-roles#4.6 · no · the servo-new hamburger aria-expanded sits in examples/assets, not reached
aria-patterns-roles#4.7 · no · the servo-new aria-label fixture sits in examples/assets, not reached
