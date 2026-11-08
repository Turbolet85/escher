@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: landmark role names from the HTML-to-AccessKit mapping, which the task's not-needed role-derivation item drops; the chunk's graph and coverage read no landmark role
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: header and section role names from the HTML-to-AccessKit mapping, dropped by the same not-needed role-derivation item
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: ARIA role-attribute derivation in the tree builder, dropped by the not-needed role-derivation item; the chunk only reads the model's role
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: aria-selected, aria-pressed, aria-checked and aria-invalid state derivation, named verbatim in the not-needed state-derivation item
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button and submit to Button is HTML role mapping, not-needed; the replay compares kept and replayed answers, which holds under any deterministic mapping
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: a link being an anchor with href is HTML role mapping, not-needed, and the replay's equality does not depend on it
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: select to ListBox or ComboBox is HTML role mapping, not-needed, and the replay's equality does not depend on it
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: a single select's expanded state written from engine popup state is state derivation in the tree builder, not-needed
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: a multiple select's set_multiselectable write is state derivation in the tree builder, not-needed
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup to Group and its label as name are role mapping and label naming, both not-needed
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: a select popup's group header row choosing nothing is popup behaviour the scope does not name, and a group header is not an actionable node
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list and table role mapping, not-needed, and the chunk's test reads no table or list
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: form input type-to-role mapping, not-needed, and no input value or redaction rule is stated here
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: div, h2 and p role mapping, not-needed, and the chunk reads no generic or text role
@answer n=15 entity=aria-patterns-roles#2.13 v=yes
why: the coverage counts only agent-visible nodes, and this entity states which nodes aria-hidden, hidden, display none and visibility hidden remove from the tree, so the count boundary depends on it
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: browser app controls that are clickable divs sit in apps/browser, outside the chunk's stand and task files
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: browser app image alt text sits in apps/browser, outside the chunk's stand and task files
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: focus-state styling of role=button in rdme's stylesheet is paint, and the chunk paints nothing
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: captured GitHub fixture attributes under examples/assets, which the chunk's stand and replay do not read
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: Google fixture contentinfo role under examples/assets, which the chunk does not read
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: Google fixture combobox attributes under examples/assets, which the chunk does not read
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: Google fixture listbox, menu and option roles under examples/assets, which the chunk does not read
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: Google fixture landmark roles under examples/assets, which the chunk does not read
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: servo-new fixture aria-expanded toggling under examples/assets, which the chunk does not read, and aria-expanded derivation is not-needed
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: servo-new fixture aria-label naming under examples/assets, which the chunk does not read, and label naming is not-needed
