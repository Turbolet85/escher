@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: landmark tag-to-role mapping is the builder's derivation, which the chunk does not touch (scope not_needed, role derivation).
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: header and section role derivation is the builder's, untouched by the buffered render.
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role_from_name is the builder's ARIA role mapping, untouched; the replay carries role attribute values verbatim and adds no rule over them.
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: ARIA state attribute derivation is the builder's and is named not_needed; the snapshot's state reads stay witnessed by the existing checks.
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button and submit role derivation is the builder's, untouched by the chunk.
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: the href gate for Link is the builder's rule; the replay passes href as any attribute and adds no rule over it.
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: select ComboBox and ListBox role derivation is the builder's; no select control or its mapping changes in the chunk.
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: single select expanded state and its pattern belong to the select popup, whose keys and list opening the chunk does not change.
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: multiselectable on a multiple select's ListBox is the builder's write, untouched by the chunk.
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup to Group role and its label name are the builder's derivation and the select popup, not the buffered render.
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: option group header row choice is select popup behaviour, which the chunk does not change.
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list, table and cell role derivation is the builder's, untouched by the chunk.
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: input role mapping by type is the builder's; the focus witness checks node identity, not role, so no rule here is acted on.
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: div, h2 and p role derivation is the builder's, untouched by the chunk.
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden, inert and display exclusion rules are the builder's and named not_needed (no visibility rule changes); the replay's attribute path is a bridge rule, not this one.
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser app's clickable div controls are outside the chunk, which edits no app.
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: browser app image alt text is outside the chunk, which edits no app.
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: rdme stylesheet focus states are paint, and the chunk changes no style or paint.
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: GitHub fixture markup is an input the chunk neither edits nor carries a rule over; fixtures are named not_needed.
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: Google fixture contentinfo role is fixture content the chunk neither edits nor carries a rule over.
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: Google fixture combobox attributes are fixture content the chunk neither edits nor carries a rule over.
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: Google fixture listbox, option and menu roles are fixture content the chunk neither edits nor carries a rule over.
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: Google fixture landmark roles are fixture content the chunk neither edits nor carries a rule over.
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: servo-new aria-expanded flip runs in the script of that fixture on the blitz path, not in the Dioxus render the chunk changes.
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: servo-new aria-label on nav and sections is fixture content the chunk neither edits nor carries a rule over.
