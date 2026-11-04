@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: the landmark roles inventory is a not-needed item, the probe names no role
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: the header and section landmark roles are role derivation the probe does not name
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role_from_name's role-attribute mapping is role derivation the probe does not name, it reads roles off the tree only
@answer n=4 entity=aria-patterns-roles#2.2 v=yes
why: aria-selected maps to selected, and the CRUD listbox's option rows carry aria-selected, which the probe reads as handler-to-state state
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button and submit to Button is role derivation the probe does not name, it addresses nodes by author_id
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: a link needs href, and the probe reaches no anchor element
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: a multiple select's ListBox is a select-list reading the probe does not read
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: a single select's ComboBox and expanded state are the select list's reading, which the probe does not read
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: set_multiselectable on a multiple select is a select-list reading the probe does not read
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup to Group is a select-list reading the probe does not read
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: the group header row of an open select list is a select-list reading the probe does not read
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list and table element roles are role derivation the probe does not name
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: the input type to role mapping is role derivation the probe does not name
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: the div, h2 and p roles are role derivation the probe does not name
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden-content exclusion from the tree is not read by the probe
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser app's clickable div controls are outside the seven_guis stand the probe runs on
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: the browser app's image alt text is outside the seven_guis stand the probe runs on
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the readme app's stylesheet focus states are outside the seven_guis stand the probe runs on
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the captured GitHub fixture markup is not read by the probe on seven_guis
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture's contentinfo role is not read by the probe on seven_guis
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture's query combobox is not read by the probe on seven_guis
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture's listbox and menu roles are not read by the probe on seven_guis
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture's landmark regions are not read by the probe on seven_guis
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new fixture's script-flipped aria-expanded is outside the seven_guis stand the probe runs on
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new fixture's aria-label on nav and sections is outside the seven_guis stand the probe runs on
