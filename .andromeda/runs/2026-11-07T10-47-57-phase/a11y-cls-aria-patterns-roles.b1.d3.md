@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: landmark tag-to-role mapping is builder derivation the chunk does not touch, and no landmark element is named by the scope.
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: header and section tag-to-role mapping is builder derivation the chunk does not touch.
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: the role attribute's mapping is builder derivation, and the scope names no role attribute value.
@answer n=4 entity=aria-patterns-roles#2.2 v=yes
why: a native checkbox's checked and an option's selected are boolean attributes whose presence the replay carries through the falsy-removal path, a fork over the 28 list; aria-* derivation is not touched.
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button-to-Button is builder derivation over a tag the buffer passes unchanged, and no button-specific code or check is named.
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: Link-by-href is builder derivation, and href is a string attribute outside the falsy boolean path.
@answer n=7 entity=aria-patterns-roles#2.5 v=yes
why: the select's multiple is a boolean attribute carried through the falsy-removal path (a fork over the 28 list), and the ListBox or ComboBox role follows it.
@answer n=8 entity=aria-patterns-roles#2.6 v=yes
why: the chosen option's selected is a boolean attribute carried through the falsy-removal path (a fork over the 28 list); the list-open state is not touched.
@answer n=9 entity=aria-patterns-roles#2.7 v=yes
why: the multiple attribute is a boolean carried through the falsy-removal path (a fork over the 28 list), and set_multiselectable follows it.
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup-to-Group and its label name read are builder derivation and name reads the scope excludes, with no boolean attribute involved.
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: a header row's choosing and an unlabelled group's name read are select-list behaviour and builder name reads, both excluded by the scope.
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list, table and cell tag mappings are builder derivation over tags the buffer passes unchanged.
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: input type-to-role mapping is builder derivation, and type is a string attribute outside the falsy boolean path.
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: div, h2 and p tag-to-role mapping is builder derivation the chunk does not touch.
@answer n=15 entity=aria-patterns-roles#2.13 v=yes
why: hidden and inert are boolean attributes the scope names on the falsy-removal path, and their exclusion of a node from the tree is what the replay must carry.
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: browser app controls are not a DioxusDocument render, and no part of the chunk reaches the browser app.
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: alt text is a name read the scope excludes, on browser app images the chunk does not reach.
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the stylesheet's focus styles are paint the chunk does not change, in an app the chunk does not reach.
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: captured GitHub fixture markup is HTML-path input the scope excludes from the Dioxus render.
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture's contentinfo role is builder derivation on a fixture the chunk does not touch.
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google textarea's combobox reading is builder derivation with aria-* attributes outside the scope.
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture's listbox, option and menu roles are builder derivation with aria-* attributes outside the scope.
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture's landmark roles are builder derivation the chunk does not touch.
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo fixture's aria-expanded, flipped by its own script, is an aria-* state the scope excludes on a non-Dioxus fixture.
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo fixture's aria-label names are name reads the scope excludes.
