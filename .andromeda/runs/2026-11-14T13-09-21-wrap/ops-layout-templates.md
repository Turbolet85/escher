@row_add n=1 row=decision/stand-fixture-states-closed-table area=surface-desktop-native fact=F1 detector=none
name: Stand fixture states as a closed table per task
rule: Each stand task holds a closed table of named fixture states, each a label-form name named once and reached from boot by a fixed list of acting calls on keyed controls; a task with no state holds an empty table.
guard: every_fixture_state_name_is_of_the_label_form_and_named_once
basis: fct-urtem5yt
because: The chunk adds the stand's per-task tables of named fixture states, a convention every later state must follow, and no row of the stand says it.
@row_add n=2 row=contract/stand-fixture-states/fourteen-people area=surface-desktop-native detector=none
boundary: Stand named fixture states
item: fourteen-people
promise: A CRUD task state reached from boot by Create clicks on the keyed create control alone.
basis: fct-urtem5yt
because: The CRUD table now occupies this state name, an item of a closed list callers start in.
@row_add n=3 row=contract/stand-fixture-states/person-selected area=surface-desktop-native detector=none
boundary: Stand named fixture states
item: person-selected
promise: A CRUD task state reached from boot by one click on the first person's keyed row.
basis: fct-urtem5yt
because: The CRUD table now occupies this state name, an item of a closed list callers start in.
@row_add n=4 row=contract/stand-fixture-states/return-flight area=surface-desktop-native detector=none
boundary: Stand named fixture states
item: return-flight
promise: A flight booker state reached from boot by choosing the return mode in the keyed flight-type select.
basis: fct-urtem5yt
because: The flight booker table now occupies this state name, an item of a closed list callers start in.
@row_add n=5 row=decision/started-state-reads-as-by-hand-path area=surface-desktop-native fact=F2 detector=none
name: Started state reads as its by-hand path
rule: A session started in a named fixture state, or a stand page held in one, reads in both layout modes as that state reached by hand from boot and takes the next call as the by-hand session does.
guard: stand_start_in_state
basis: fct-bv2xw5ml
because: The chunk promises that a start in a state is indistinguishable from its by-hand path, a rule later states and checks must keep.
@row_add n=6 row=decision/fixture-name-checked-whole-before-boot area=surface-cli fact=F3 detector=none
name: Fixture state name checked whole before boot
rule: A fixture state name is compared whole against the app's closed table before anything boots; any other name is refused as an unknown state with nothing started, its text fixed and holding nothing the caller supplied.
guard: a_name_no_state_holds_is_refused_and_the_session_reads_as_boot
basis: fct-k57px24b
because: The chunk adds a caller-supplied state name validated against a closed table, a validation rule no row states.
@row_add n=7 row=decision/reach-through-validated-run-path area=surface-cli fact=F4 detector=none
name: Reach runs every step through the validated run path
rule: A reach runs each step of a fixture state through the session's validated run path in order and stops at the first refused or unsettled step, naming the step's index and its cause; the steps before it stand and are not rolled back.
guard: a_reach_stops_at_the_first_refused_step_naming_its_index_and_cause
basis: fct-urtem5yt
because: The chunk adds the reach of a state, which admits no step outside validation and promises no rollback, a contract later carriers must respect.
@row_add n=8 row=absence/fixture-state-on-carriers area=surface-cli fact=F5 detector=none
capability: Starting in a fixture state over the command line, the MCP server, the session socket or a kept recording
basis: fct-ggyakcis
because: The founder ratified that a fixture state is reached in process only, the absence of a whole carrier capability.
@row_none n=1 fact=F6 type=4
relied: A recorded session that reached a named state replays from boot as its steps, the check form unchanged; the recording row stays true as it stands.
