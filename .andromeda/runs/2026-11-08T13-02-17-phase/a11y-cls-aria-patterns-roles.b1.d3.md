@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: landmark role names (nav, main, aside, footer and the rest) name no actionable node, and the landmark inventory is not-needed item 7.
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: the header and section landmark roles name no actionable node, and the landmark inventory is not-needed item 7.
@answer n=3 entity=aria-patterns-roles#2.1 v=yes
why: the ARIA role attribute mapping (button, tab, option, listbox) decides which elements carry an interactive role and the CRUD list's person rows are options by role, so the read's listing rides on it.
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: the aria-selected, aria-pressed, aria-checked and aria-invalid state mapping is not read, since the read lists verbs and refusal causes, not states (not-needed item 2), and disabled keys on the disabled attribute only.
@answer n=5 entity=aria-patterns-roles#2.3 v=yes
why: button and submit mapped to Button is the element kind item 6 keys disabled on, and a button's click verb and disabled cause are what the read lists.
@answer n=6 entity=aria-patterns-roles#2.4 v=yes
why: an anchor with href reads Link and is natively focusable, so whether it is listed as an interactive focusable node depends on this mapping, and the scope leaves open which roles are interactive.
@answer n=7 entity=aria-patterns-roles#2.5 v=yes
why: single versus multiple select is the split item 9 draws between a closed single select's options taking no click and a multiple select's rows reading their own client rects.
@answer n=8 entity=aria-patterns-roles#2.6 v=yes
why: a single select's ComboBox with expanded from the open list state is the owner pattern item 9 relies on, closed options taking no click and an open list's row click choosing it.
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: the multiselectable state write is a state the read does not list, and a multiple select's row handling is already bound through the ListBox and client-rect rule of 2.5.
@answer n=10 entity=aria-patterns-roles#2.8 v=yes
why: item 9 names the optgroup reading Role::Group, so the group is a non-owner node the read must treat by that role, and its label is the name the group carries.
@answer n=11 entity=aria-patterns-roles#2.9 v=yes
why: a group header row choosing nothing and an unlabelled group reading the whole list's box are the owner-served rules item 9 names.
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: the list and table roles (ul, ol, li, table, tr, th, td) name no node the read lists, the CRUD rows are listbox options under 2.1 and 2.5, and cells grid semantics and the role inventory are not-needed items 6 and 7.
@answer n=13 entity=aria-patterns-roles#2.11 v=yes
why: the input role by type (password as PasswordInput, text, number, checkbox, radio, range) and textarea as MultilineTextInput are the element kinds item 6 keys disabled on and item 11 masks by password role.
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: h2 as Heading and p as Paragraph name no actionable node, and a clickable div is listed through its listener under item 1, not through its GenericContainer role.
@answer n=15 entity=aria-patterns-roles#2.13 v=yes
why: the hidden-content rules (hidden, display none with the select option excepted, visibility hidden, and children of a hidden element excluded) bound which nodes the read can list, as item 8 states.
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser app's clickable div controls sit in apps/browser, outside the read's files and the Dioxus stand, so the read reaches none of them.
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: the browser's image alt text sits in apps/browser, and the read lists ids not names (not-needed item 3).
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the rdme stylesheet's focus-state paint for role=button and role=tabpanel is painting, and the read paints nothing (not-needed item 10).
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the captured GitHub fixture attributes (aria-hidden, tooltip, img with aria-label, aria-labelledby) are not the stand or the minimal fixture the read proves on.
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture's contentinfo role is not the stand or the minimal fixture the read proves on.
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture's combobox textarea and its autocomplete and popup attributes are not the stand or the minimal fixture the read proves on.
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture's listbox, option and menu roles are not the stand, and the stand's CRUD list is the listbox evidence item 10 names.
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture's landmark regions fall under the not-needed landmark inventory, and the fixture is not one the read proves on.
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new fixture's aria-expanded on its hamburger is a merged state the read does not list (not-needed item 2), and the fixture is not one the read proves on.
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new fixture's aria-label names on nav and sections are names the read does not list (not-needed item 3), and the fixture is not one the read proves on.
