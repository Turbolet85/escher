@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: the HTML-to-AccessKit landmark mapping is the engine's, dropped as not needed, and the read takes roles from the model
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: header and section roles are the same engine mapping, which the read only consumes through the model
@answer n=3 entity=aria-patterns-roles#2.1 v=yes
why: the CRUD list's rows are options of a listbox by role, the scope's named carrier, and the role attribute mapping here yields that role
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: ARIA state attribute mapping is dropped as not needed, since the read lists verbs and refusal causes and not states
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button to Button is an engine role mapping the read takes from the model, adding no mapping of its own
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: the link role of an anchor with href is the engine's HTML role mapping, which the scope does not name
@answer n=7 entity=aria-patterns-roles#2.5 v=yes
why: the scope's multiple select rows read their own client rects, and the ListBox role of a multiple select is what separates its rows from a single select's
@answer n=8 entity=aria-patterns-roles#2.6 v=yes
why: the scope's open list and closed single select option rules turn on the list's open state, which this entry says the ComboBox expanded state is written from
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: set_multiselectable is an engine flag the scope does not name, and the read's rows take their bounds and no multiselectable reading
@answer n=10 entity=aria-patterns-roles#2.8 v=yes
why: the scope names an optgroup reading Role::Group, which this entry states
@answer n=11 entity=aria-patterns-roles#2.9 v=yes
why: the scope names a group header row choosing nothing, which this entry states
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list and table element roles are the engine's HTML role mapping, dropped as not needed, and the CRUD rows are options by role
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: the input type to role mapping is the engine's, and the read carries no node value so the password masking needs no role
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: div, heading and paragraph roles are the engine's HTML role mapping, which the read takes from the model
@answer n=15 entity=aria-patterns-roles#2.13 v=yes
why: the scope names a select's option excepted from display none and the display none, visibility hidden and inert exclusions that bound which nodes the read can list
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: the browser app's clickable divs are outside the Dioxus document the read covers
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: browser app image alt text is not reached by a read of actionable nodes
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: the rdme stylesheet's focus states paint only, and the read paints nothing
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: the GitHub fixture attributes are neither the stand nor the read's minimal fixture
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: the Google fixture's contentinfo role is outside the stand and the read's fixture
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: the Google fixture's combobox textarea is outside the stand and the read's minimal fixture
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: the Google fixture's listbox and option roles are outside the stand, and the CRUD list rows are the scope's listbox
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: the Google fixture's landmark regions are neither the stand nor the read's minimal fixture
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: the servo-new aria-expanded is a merged state the task drops as not needed
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: the servo-new nav and section aria-label names are name computation the task drops as not needed
