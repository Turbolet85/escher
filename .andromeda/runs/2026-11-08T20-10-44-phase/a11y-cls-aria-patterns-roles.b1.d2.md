@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: landmark role mapping (nav, main, aside and the rest) is the tree builder's HTML-to-AccessKit mapping, which the task leaves out, and no landmark is an actionable node
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: header and section role mapping is the tree builder's HTML-to-AccessKit mapping, which the task leaves out, and neither is actionable
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role_from_name's ARIA role-attribute derivation is tree-builder role derivation named not needed (task not_needed 4); the chunk reads the role and maps none
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: aria-selected, aria-pressed, aria-invalid and aria-checked state derivation is named not needed (task not_needed 4); the chunk reads the states and derives none
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button and submit mapping to Button is HTML-to-AccessKit role mapping (task not_needed 4); the chunk changes no role
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: a link is Link only with href, which is HTML-to-AccessKit role mapping (task not_needed 4); the actionable rule stays unchanged and this entity does not state it
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: select multiple mapping to ListBox, else ComboBox, is HTML-to-AccessKit role mapping (task not_needed 4)
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: a single select's ComboBox with expanded written from its list's open state is tree-builder state derivation (task not_needed 4); the chunk changes no select behaviour
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: a multiple select's ListBox writing set_multiselectable is tree-builder state derivation (task not_needed 4)
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup mapping to Group and its label name are tree-builder role and naming (task not_needed 4 and 5); the chunk names nothing
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: a group's header row choosing nothing in an open list is select-popup activation behaviour (task not_needed 2); the chunk adds no activation or key behaviour
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list, table and cell role mapping is HTML-to-AccessKit role mapping (task not_needed 4); the chunk builds no table read
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: input type-to-role mapping is HTML-to-AccessKit role mapping (task not_needed 4); the redaction rule rules over the masked value, which this mapping does not state
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: div, h2 and p role mapping is HTML-to-AccessKit role mapping (task not_needed 4)
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden-content exclusion is named not needed (task not_needed 7); the chunk changes no exclusion and the coverage read only reads the graph nodes the snapshot already holds
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: clickable divs as browser controls describe apps/browser markup, which the chunk does not touch
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: browser image alt text describes apps/browser markup, which the chunk does not touch
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: rdme stylesheet focus states belong to apps/readme; the chunk paints nothing and changes no focus (task not_needed 3 and 10)
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the GitHub fixtures' aria attributes and roles describe captured example assets that neither the chunk's code nor its replay test names
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture's contentinfo role describes an example asset that the chunk does not name
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google query combobox's attribute set describes an example fixture the chunk does not read
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google suggestion listbox and settings menu roles describe fixture markup the chunk does not name
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google landmark regions describe fixture markup the chunk does not name
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new hamburger's aria-expanded flip describes fixture script the chunk does not drive
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new nav and section aria-label values describe fixture markup the chunk does not name
