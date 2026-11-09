@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: the landmark role mapping lives in accessibility_roles.rs and blitz-dom accessibility.rs, neither reached by the session
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: the header and section role mapping lives in accessibility_roles.rs and blitz-dom accessibility.rs, neither reached by the session
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role_from_name lives in packages/blitz-dom/src/accessibility.rs, which the session does not reach and no ARIA role mapping moved
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: the ARIA state reads live in blitz-dom accessibility.rs and accessibility_states.rs, neither reached, and the session moved no builder read
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: the button and submit mapping lives in blitz-dom accessibility.rs and accessibility_roles.rs, neither reached by the session
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: the link mapping lives in blitz-dom accessibility.rs and accessibility_roles.rs, neither reached by the session
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: the select role mapping lives in blitz-dom accessibility.rs and accessibility_roles.rs, neither reached by the session
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: the single-select ComboBox and expanded reads live in blitz-dom accessibility.rs and select_popup.rs, neither reached by the session
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: the multiselectable write lives in blitz-dom accessibility.rs, which the session does not reach
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: the optgroup Group mapping lives in blitz-dom accessibility.rs, which the session does not reach
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: the option group header row rule lives in blitz-dom document.rs and select_popup.rs, neither reached by the session
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: the list and table mapping lives in accessibility_roles.rs and blitz-dom accessibility.rs, neither reached by the session
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: the form input type mapping lives in blitz-dom accessibility.rs and accessibility_roles.rs, neither reached by the session
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: the generic and text mapping lives in accessibility_roles.rs, which the session does not reach
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: the hidden-content exclusion lives in blitz-dom accessibility.rs and accessibility_hidden.rs, neither reached, and the task marks it unchanged
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser clickable-div controls live in apps/browser sources, which the session does not reach
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: the browser image alt text lives in apps/browser sources, which the session does not reach
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the rdme focus-state styling lives in apps/readme/assets/github-markdown.css, which the session does not reach
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the GitHub fixture attributes live in examples/assets/github_profile_reduced2.html, which the session does not reach
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture contentinfo role lives in examples/assets/bottom_only.html, which the session does not reach
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture combobox attributes live in examples/assets/google.html, which the session does not reach
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture listbox and menu roles live in examples/assets/google.html, which the session does not reach
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture landmark roles live in examples/assets/google.html, which the session does not reach
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new fixture hamburger attributes live in examples/assets/servo-new.html, which the session does not reach
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new fixture aria-labels live in examples/assets/servo-new.html, which the session does not reach
