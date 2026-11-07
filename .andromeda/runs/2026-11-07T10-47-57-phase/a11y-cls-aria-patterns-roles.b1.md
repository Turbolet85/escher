@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: the landmark tag to role mapping is builder derivation the chunk does not touch, and no chunk code or check names nav or main
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: the header and section role mapping is builder derivation the chunk does not touch, and nothing it re-asserts names those tags
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: the role attribute mapping is builder derivation the chunk does not touch, and the buffer carries the attribute value unchanged
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: the ARIA state attribute mapping is builder derivation the task excludes, and the replay keeps those attribute values as the direct write gave them
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: the button and submit role mapping is builder derivation the chunk does not touch, and no need names a button role
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: the link role on an href anchor is builder derivation the chunk does not touch, and no need names an anchor
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: the select to ComboBox and ListBox mapping is builder derivation, and the select control's keys and handling are excluded by the task
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: the single select's expanded bool is engine state the chunk does not touch, and its list opening is excluded by the task
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: the multiple select's set_multiselectable output is builder behaviour the chunk does not touch, and select handling is excluded
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: the optgroup to Group mapping is builder derivation, and the task excludes optgroup handling and names no group role
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: the option group header row's choosing and box naming are select popup behaviour the chunk does not touch
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: the list, table and cell role mapping is builder derivation the chunk does not touch, and no need names these elements
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: the input type to role mapping is builder derivation, and the focus witness reads the focused node's identity rather than its role
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: the div, h2 and p role mapping is builder derivation the chunk does not touch, and no need names these elements
@answer n=15 entity=aria-patterns-roles#2.13 v=yes
why: hidden and inert exclude nodes from the tree, and the replay must remove those attributes on a falsy value (task need 8) so the exclusion matches the direct write
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser app's clickable div controls are outside the chunk's Dioxus stand and bridge paths
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: the browser app's image alt text is app content the chunk does not touch, and no chunk check reads it
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: rdme's stylesheet focus styles are paint the chunk does not change, as the task excludes all style changes
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the captured GitHub fixtures are example assets the chunk's stand and witnesses do not load
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture's contentinfo role is a fixture value outside the chunk's checks
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture's combobox textarea is a fixture outside the chunk's checks, and no need names it
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture's listbox, option and menu roles are fixture values outside the chunk's checks
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture's landmark roles are fixture values outside the chunk's checks
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new hamburger's aria-expanded flip is a script behaviour in a fixture outside the chunk's paths
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new nav and section labels are fixture values outside the chunk's checks
