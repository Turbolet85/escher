@answer n=1 entity=keyboard-navigation#5.8 v=no
why: Cells arrows and scroll into view are key behaviour the chunk leaves unchanged, and the entity states no role, name or Tab order
@answer n=2 entity=keyboard-navigation#5.9 v=no
why: Cells editor Enter, commit and click return are key behaviour the chunk leaves unchanged
@answer n=3 entity=keyboard-navigation#5.10 v=yes
why: the CRUD list's stated listbox role, option rows, People name and third Tab stop are roles, names and Tab orders of a seven_guis stand task that the chunk must keep
@answer n=4 entity=keyboard-navigation#5.11 v=no
why: the Preact TodoMVC example autofocus and Enter add sit outside the chunk's code and tests
@answer n=5 entity=keyboard-navigation#5.12 v=no
why: Android Activity focus setup sits outside the chunk's code and tests
@answer n=6 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture tabindex on toolbar containers is a fixture the chunk neither reads nor re-measures
@answer n=7 entity=keyboard-navigation#6.2 v=no
why: the google fixture's explicit role button and tabindex are explicit roles that the role derivation leaves untouched, and no named scope reads this fixture
@answer n=8 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction keydown handlers are JS handlers outside the Dioxus vdom walk the graph reads
@answer n=9 entity=keyboard-navigation#6.4 v=no
why: the google fixture focus-visible outline is a focus style rule the chunk does not change
@answer n=10 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture outline and focus border rules are style rules the chunk does not change
@answer n=11 entity=keyboard-navigation#6.6 v=no
why: the servo.css focus outline and box-shadow are style rules the chunk does not change
@answer n=12 entity=keyboard-navigation#6.7 v=no
why: the servo.html heading tabindex -1 is a fixture the chunk neither reads nor re-measures
@answer n=13 entity=keyboard-navigation#6.8 v=no
why: label association by for, id and nesting is name derivation that the chunk keeps through the same snapshot mapping, and examples form.rs is not named by the scope
@answer n=14 entity=keyboard-navigation#7.1 v=yes
why: the CRUD crud-name to crud-surname Tab step and the snapshot focused node read on the stand are a seven_guis Tab order and focused reading that the chunk's snapshot changes must keep
@answer n=15 entity=keyboard-navigation#7.2 v=yes
why: the stand_keyboard Tab walk of tab_order with the snapshot focused node agreeing at every step binds because the scope keeps the shared Tab orders and the chunk moves seven_guis signals
@answer n=16 entity=keyboard-navigation#7.3 v=yes
why: LOAD_TAB_ORDER and CELLS_TAB_ORDER are the shared Tab orders that the scope says must stay as they are
@answer n=17 entity=keyboard-navigation#7.4 v=yes
why: stand_act_range states the Timer slider's role Slider, its name and the forward focus sequence, which are seven_guis roles, names and Tab orders the chunk keeps, while the range key handling itself is not reached
@answer n=18 entity=keyboard-navigation#7.5 v=no
why: text and range input and change event firing is event behaviour the chunk does not change
@answer n=19 entity=keyboard-navigation#7.6 v=yes
why: stand_act_select reads a multiple select's rows each with its own bounds and refuses covered options, which the node bounds of an open list's rows must keep
@answer n=20 entity=keyboard-navigation#7.7 v=yes
why: the select popup checks read open list row bounds in both layout modes, which the chunk's node bounds for an open list must keep
@answer n=21 entity=keyboard-navigation#7.8 v=yes
why: the flight booker select's name Flight type and its second Tab stop are a seven_guis stand task's stated name and Tab order that the chunk must keep
@answer n=22 entity=keyboard-navigation#7.9 v=no
why: the key press past a dropped anonymous block is a key press and layout check that reads no graph bounds, role or Tab order, and the chunk changes none of them
@answer n=23 entity=keyboard-navigation#8.1 v=no
why: an observed absence of tabindex handling in the older s06 files is not a rule the chunk's code or tests must honour
@answer n=24 entity=keyboard-navigation#8.2 v=no
why: an observed absence of tabindex handling in the older s10 files is not a rule the chunk's code or tests must honour
@answer n=25 entity=keyboard-navigation#8.3 v=no
why: an observed absence of focus or keyboard handling in the older s07 files is not a rule the chunk's code or tests must honour
