@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: its landmark mapping cites accessibility_roles.rs and blitz-dom, neither reached by the session, which adds only actionable_nodes and driver verbs
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: its header and section mapping cites accessibility_roles.rs, a file the session did not reach
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role_from_name lives in blitz-dom accessibility.rs, which the session did not change
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: ARIA state reads live in blitz-dom and the cited state tests, none reached, and the session's actionable rule reads roles and listeners, not states
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: the button mapping cites accessibility_roles.rs, a file the session did not reach
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: the link mapping lives in blitz-dom accessibility.rs, which the session did not change
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: the select mapping lives in blitz-dom accessibility.rs, which the session did not change
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: the single-select ComboBox rule cites blitz-dom and select_popup.rs, neither reached by the session
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: the multiple-select set_multiselectable write lives in blitz-dom, which the session did not change
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: the optgroup Group mapping lives in blitz-dom, which the session did not change
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: the option group header row cites blitz-dom document.rs and select_popup.rs, neither reached by the session
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: the list and table mapping cites accessibility_roles.rs and blitz-dom, neither reached by the session
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: the form input mapping cites accessibility_roles.rs and blitz-dom, neither reached by the session
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: the generic and text mapping cites accessibility_roles.rs, a file the session did not reach
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden-content exclusion lives in blitz-dom, which the session left unchanged, and its element-level rule is separate from actionable_nodes
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the clickable div controls live in apps/browser, a file the session did not reach
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: the browser image alt text lives in apps/browser files, which the session did not reach
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the focus-state styling lives in apps/readme assets, which the session did not reach
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the captured GitHub fixture attributes live in examples/assets, which the session did not reach
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture contentinfo role lives in examples/assets, which the session did not reach
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture combobox attributes live in examples/assets, which the session did not reach
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture listbox and menu roles live in examples/assets, which the session did not reach
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture landmark roles live in examples/assets, which the session did not reach
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new hamburger attributes live in examples/assets, which the session did not reach
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new nav and section labels live in examples/assets, which the session did not reach
