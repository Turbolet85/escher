@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: the landmark role mapping in blitz-dom and accessibility_roles.rs is not among the session's files and the session leaves role derivation as it was
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: the header and section roles cite accessibility_roles.rs, which the session does not reach, and its roles read as before
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role_from_name in blitz-dom accessibility.rs is not among the session's files and the ARIA role attribute mapping is unchanged
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: the ARIA state mapping in blitz-dom accessibility.rs is not reached, and the session adds a snapshot field rather than changing any accessibility state
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: the button and submit role mapping cited in accessibility_roles.rs is not reached by the session and reads as before
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: the link role mapping in blitz-dom accessibility.rs and accessibility_roles.rs is not reached and the session changes no link path
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: the select ListBox and ComboBox mapping in blitz-dom accessibility.rs is not reached and the session changes no select role
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: the single select ComboBox pattern and its expanded write in accessibility.rs and select_popup.rs are not reached by the session
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: the multiple select set_multiselectable write in blitz-dom accessibility.rs is not reached and the session changes no select path
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: the optgroup Group role and its label in blitz-dom accessibility.rs are not reached and the session leaves them as they were
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: the option group header row in document.rs and select_popup.rs is not reached and the session changes no select popup behaviour
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: the list and table role mapping in accessibility.rs and accessibility_roles.rs is not reached by the session
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: the form input role mapping by type in accessibility.rs and accessibility_roles.rs is not reached and reads as before
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: the generic container, heading and paragraph roles cited from accessibility_roles.rs are not reached by the session
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden content and inert exclusion stay unchanged in accessibility.rs, and the session does not reach that file or the hit test
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser clickable divs are markup in apps/browser sources that the session does not reach, and the new clickable field does not change that markup
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: the alt text of the browser's logo, favicons and icon buttons in apps/browser is not reached by the session
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the focus-state rules in the apps/readme github-markdown stylesheet are not reached by the session
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the captured GitHub fixture attributes in examples/assets are not reached and the session changes no fixture
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture's contentinfo role in examples/assets is not reached and the session changes no fixture
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture's query textarea combobox attributes in examples/assets are not reached and the session changes no fixture
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture's listbox, option and menu roles in examples/assets are not reached and the session changes no fixture
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture's landmark regions in examples/assets are not reached and the session changes no fixture
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new fixture's hamburger button attributes and script in examples/assets are not reached and the session changes no fixture
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new fixture's aria-label usage in examples/assets is not reached and the session changes no fixture
