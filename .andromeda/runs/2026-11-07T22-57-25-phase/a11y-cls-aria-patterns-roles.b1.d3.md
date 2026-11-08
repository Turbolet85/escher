@answer n=1 entity=aria-patterns-roles#1.1 v=yes
why: landmark tag-to-role mapping is role derivation the chunk must leave untouched, and the graph reads the same role with no second mapping
@answer n=2 entity=aria-patterns-roles#1.2 v=yes
why: header and section tag-to-role mapping is role derivation the chunk must leave untouched
@answer n=3 entity=aria-patterns-roles#2.1 v=yes
why: the role attribute to AccessKit role mapping decides whether a click-reacting element states a role, so the GenericContainer rule depends on it
@answer n=4 entity=aria-patterns-roles#2.2 v=yes
why: role="switch" to Switch is role derivation the chunk leaves untouched; the aria-state mapping in the same entity is not_needed item 4 and does not bind
@answer n=5 entity=aria-patterns-roles#2.3 v=yes
why: button and submit to Button is role derivation the chunk must leave untouched
@answer n=6 entity=aria-patterns-roles#2.4 v=yes
why: an a element without href reads GenericContainer, the click-reacting roleless case the chunk's role item names
@answer n=7 entity=aria-patterns-roles#2.5 v=yes
why: select to ComboBox or ListBox is role derivation that the stand's select nodes read through the snapshot model
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: its ComboBox role repeats #2.5, and its expanded and selected readings belong to the select popup, which the chunk does not change
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: the multiselectable write is an ARIA property on the ListBox, not role derivation, and the chunk changes no select-list state
@answer n=10 entity=aria-patterns-roles#2.8 v=yes
why: optgroup to Group and its label as name are role and name derivation the graph must read without a second mapping
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: the option group header row's choosing behaviour is select popup behaviour, which the chunk does not change
@answer n=12 entity=aria-patterns-roles#2.10 v=yes
why: the ul, ol, li, table, tr, th and td role mapping is role derivation the chunk must leave untouched
@answer n=13 entity=aria-patterns-roles#2.11 v=yes
why: the input type mapping includes password to PasswordInput, which the masking rule needs to identify the masked node, and the rest is role derivation
@answer n=14 entity=aria-patterns-roles#2.12 v=yes
why: div to GenericContainer is the roleless fallback the chunk's role item names for a click-reacting element
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden-content exclusion is not_needed item 6, and the graph inherits the tree's exclusion without the chunk naming or changing it
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: browser app clickable divs are blitz app controls outside the Dioxus document graph the chunk reads
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: browser app image alt text is a browser fixture concern, which not_needed item 8 leaves alone
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the readme app stylesheet's focus states on role=button and tabpanel do not reach the chunk's Dioxus stand code
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the GitHub fixture's ARIA attributes are fixture content the chunk's stand tests do not load
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture's contentinfo role is fixture content outside the chunk's tests
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture's combobox textarea is fixture content outside the chunk's tests
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture's listbox and menu roles are fixture content outside the chunk's tests
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture's landmark regions are fixture content outside the chunk's tests
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new fixture's aria-expanded hamburger is fixture content outside the chunk's tests
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new fixture's aria-label nav and sections are fixture content outside the chunk's tests
