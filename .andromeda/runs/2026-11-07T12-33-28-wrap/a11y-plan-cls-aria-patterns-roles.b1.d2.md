@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: the session changes only the Dioxus mutation buffer and the probe, not role derivation or the landmark mapping that accessibility_roles.rs pins
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: the session changes only the Dioxus mutation buffer and the probe, not role derivation or the header and section mapping that accessibility_roles.rs pins
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role_from_name is role derivation, which the session leaves unchanged in the AccessKit builder
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: ARIA state reads are untouched by the session, which only moves the render-time document borrow
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button role derivation is unchanged by the session and its builder is not modified
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: the link role rule in accessibility.rs is role derivation, which the session does not change
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: the select role rule in accessibility.rs is role derivation, which the session does not change
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: the single-select ComboBox pattern and its popup test are not in the session's files, and the builder is unchanged
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: the multiple-select ListBox mapping is role derivation in accessibility.rs, which the session does not change
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup to Group role mapping is role derivation in accessibility.rs, which the session does not change
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: the option group header row lives in blitz-dom document.rs and select_popup.rs, neither of which the session reaches
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list and table role mappings are role derivation, which the session leaves unchanged
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: form input role mapping by type is role derivation, which the session does not change
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: generic and text role mapping is role derivation, which the session leaves unchanged
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden-content rules of the AccessKit builder are unchanged by the session, which does not alter the builder
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser app's clickable-div controls are in apps/browser, which the session does not reach
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: the browser app's image alt attributes are in apps/browser, which the session does not reach
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: rdme's stylesheet focus rules are in apps/readme CSS, which the session does not reach
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the GitHub fixtures under examples/assets are input files the session does not change
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture under examples/assets is an input file the session does not change
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture's combobox markup under examples/assets is an input file the session does not change
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture's listbox and menu markup under examples/assets is an input file the session does not change
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture's landmark markup under examples/assets is an input file the session does not change
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new fixture under examples/assets is an input file the session does not change
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new fixture under examples/assets is an input file the session does not change
