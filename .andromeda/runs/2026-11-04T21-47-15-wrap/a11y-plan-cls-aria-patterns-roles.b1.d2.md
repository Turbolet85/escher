@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: the session reaches only dioxus_runtime_probe.rs, Cargo.lock and deny.toml, and this landmark mapping lives in accessibility_roles.rs and blitz-dom, both unchanged
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: the header and section mapping cited at accessibility_roles.rs:157-180 sits in files the session did not touch, and the HTML-to-AccessKit mapping is untouched
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: the role_from_name mapping in blitz-dom accessibility.rs is untouched, and the session changes no engine code under packages/
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: the ARIA state mapping in blitz-dom accessibility.rs and accessibility_states.rs is untouched, and the session changes neither
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: the button mapping cited at accessibility_roles.rs lies outside the session's three reached files, and its mapping is unchanged
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: the link href rule in blitz-dom accessibility.rs is untouched, and the session reaches neither that file nor accessibility_roles.rs
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: the select ListBox and ComboBox mapping in blitz-dom accessibility.rs is untouched, and the session reaches neither that file nor its test
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: the single-select ComboBox pattern and its select_popup.rs test are unchanged, and the session's reached files do not include them
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: the multiple-select set_multiselectable write in blitz-dom accessibility.rs is untouched, and the session changes no engine code
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: the optgroup Role::Group mapping in blitz-dom accessibility.rs is untouched, and the session reaches no file that holds it
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: the option group header row rule at document.rs:1006-1023 and select_popup.rs is unchanged, and the session reaches neither file
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: the list and table role mapping cited at accessibility_roles.rs:95-115 and accessibility.rs:502-519 is untouched by the session
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: the input type to role mapping in blitz-dom accessibility.rs is untouched, and the session changes no engine or builder code
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: the generic, heading and paragraph mapping cited at accessibility_roles.rs:157-180 is untouched, and the session reaches no file that holds it
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: the hidden-content rules in blitz-dom accessibility.rs and accessibility_hidden.rs are untouched, and the session reaches neither file
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser clickable-div controls under apps/browser are outside the session, which changes no file in apps/ and reaches none of them
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: the browser image alt attributes under apps/browser are outside the session, which changes no file in apps/
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the readme stylesheet focus states under apps/readme are outside the session, which changes no file in apps/
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the captured GitHub fixture attributes under examples/assets are unchanged, since the session makes no change under examples/
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google contentinfo role in examples/assets/bottom_only.html is unchanged, since the session makes no change under examples/
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the google.html combobox textarea attributes are unchanged, since the session makes no change under examples/
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the google.html listbox, option and menu roles are unchanged, since the session makes no change under examples/
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the google.html landmark roles are unchanged, since the session makes no change under examples/
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new.html hamburger button attributes are unchanged, since the session makes no change under examples/
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new.html aria-label uses are unchanged, since the session makes no change under examples/
