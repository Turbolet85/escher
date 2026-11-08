@answer n=1 entity=standard-contracts#6.6 v=no
why: style-data filtering in borrow_data and the scoped resolve context is engine style behaviour the scope does not name
@answer n=2 entity=standard-contracts#7.1 v=no
why: the Taffy layout traits and the paint and table APIs are layout internals the chunk changes nothing in and the scope does not name
@answer n=3 entity=standard-contracts#8.1 v=no
why: mix-blend-mode in the stacking-context arm is style and paint behaviour the read does not reach
@answer n=4 entity=standard-contracts#9.1 v=yes
why: the read's node kinds (text input, checkbox, range input) are SpecialElementData variants the reached graph and actionable code would match, and the entity states the enum is not non_exhaustive
@answer n=5 entity=standard-contracts#10.1 v=yes
why: is_disabled and its can_be_disabled set are the disabled reading the shared refusal path must keep
@answer n=6 entity=standard-contracts#10.2 v=yes
why: row_disabled is the open list's reading of a disabled row, which the answered-first refusal must agree with option_disabled
@answer n=7 entity=standard-contracts#10.3 v=yes
why: the select module's readers options, is_select_option and is_single_select carry the owner-served select case the read lists
@answer n=8 entity=standard-contracts#10.4 v=yes
why: owner_select and option_disabled are the owner reading and the disabled option reading the read's refusal uses
@answer n=9 entity=standard-contracts#10.5 v=yes
why: SelectListView and SelectListRow are the public read types of an open list's rows that the answered-first refusal reads through; the colour constants do not bind
@answer n=10 entity=standard-contracts#10.6 v=no
why: a just-opened list surviving a focus restyle concerns a click opening a list, which the read never dispatches
@answer n=11 entity=standard-contracts#10.7 v=no
why: the list's row limit and placement are its own geometry, which the read sees only through boxes the scope does not name
@answer n=12 entity=standard-contracts#10.8 v=no
why: the recorded out-of-flow flag is layout state the visible_region reading does not name
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-block-children flag only changes what paints a box's own shadow, background and border
@answer n=14 entity=standard-contracts#10.10 v=no
why: the classes atoms are the class attribute, while the read's stable id is an author key and no class reaches the read
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state is the details and dialog open state, not the open select list the read answers first
@answer n=16 entity=standard-contracts#11.1 v=no
why: provider traits govern fetch and load registration, which the read never makes and the scope names no such behaviour
@answer n=17 entity=standard-contracts#12.1 v=no
why: HTML parsing and blitz-net fetching are neither reached by the read nor named by the scope
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint_scene, the shell re-exports and the password mask paint through renderers the read never runs and takes no shot of
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list lays out and draws rows, and the read paints nothing
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint are text painting the read does not touch
@answer n=21 entity=standard-contracts#13.4 v=no
why: mix-blend-mode and isolation paint layers the read does not paint
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend's missing frame witness is a paint test gap with its own CARRY owner, not a rule the read's code honours
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider redraw requests through its event loop are windowed shell behaviour the headless read never triggers
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the harness's settle, pump, input helpers and load counting are the steps and load waits the read must not run, and the stand proof runs through this harness
@answer n=25 entity=standard-contracts#16.1 v=yes
why: the scope says the read moves no virtual clock time, and this entity names the calls that move the harness clock (tick, advance_time, pump) and the ones that never do (settle, screenshot)
