@row_add n=1 row=contract/driver-session/reach area=standard-contracts fact=F1 detector=D-arch-resources
boundary: escher-driver session
item: Session::reach
promise: Session::reach runs each acting step (click, type, press, advance) of a named fixture state as a validated call through the one run path, in process, and stops at the first refused or unsettled step with the earlier steps standing, not rolled back.
basis: fct-urtem5yt, fct-m266tyny
because: The chunk lands a new public driver API that reaches a named fixture state through the validated run path, a contract no row registers.
@row_add n=2 row=contract/driver-session/fixture-named area=standard-contracts fact=F2 detector=D-arch-resources
boundary: escher-driver session
item: Fixture::named
promise: A fixture name is compared whole against the app's closed fixture table; any other name is refused UnknownState, and every ReachError text is fixed, holding nothing a caller supplied.
basis: fct-urtem5yt, fct-llv2u4hm
because: The chunk lands the closed-table name lookup and its fixed-text refusal type, a promise at the driver session boundary no row states.
@row_add n=3 row=absence/fixture-state-on-carriers area=standard-contracts fact=F3 detector=none
capability: A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
basis: fct-ggyakcis, fct-m266tyny, fct-kczcj7hv
because: The founder ratified a start in a state as in process only, so the absence of any carrier for a fixture state is a whole capability the project does not have.
@row_add n=4 row=contract/headless-stand/start-in area=standard-contracts fact=F4 detector=D-arch-resources
boundary: seven_guis headless stand
item: start_in
promise: A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
basis: fct-urtem5yt, fct-k57px24b
because: The stand gains per-task fixture tables and a public start in a state that checks the name before the boot, a stand contract no row states.
@row_set n=1 row=contract/driver-recording/kept-check cell=promise seen=1 fact=F5 detector=none
value: The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
basis: fct-bv2xw5ml
because: A recorded session started in a state keeps its reach as ordinary calls and replays from boot, which the kept-check promise says less.
@row_set n=2 row=decision/fixtures-test-module cell=rule seen=1 fact=F6 detector=none
value: A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot, the hold and the hold in a named state, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
basis: fct-urtem5yt, fct-m266tyny
because: The shared fixtures module now states the hold in a named state beside its boot and hold, which the convention must name.
@row_set n=3 row=decision/shared-fixtures-stated-once cell=rule seen=1 detector=none
value: The blitz-tests fixture module states the fixture boot, the hold, the hold in a named state, the shared fixture pages and the held network stand-in once.
basis: fct-urtem5yt, fct-m266tyny
because: The shared fixtures module now states the hold in a named state once, so the row listing what it states once is incomplete without it.
@row_add n=5 row=decision/std-result-under-dioxus-prelude area=conventions fact=F7 detector=none
name: std Result spelled under the Dioxus prelude glob
rule: A file that glob-imports the dioxus_native prelude spells a two-parameter result as std::result::Result.
why: On Android the glob brings manganis's one-parameter Result alias, a failure only the CI Android job sees.
basis: fct-ez7afaim, fct-n6mllq3p
because: A red Android CI run showed the prelude glob's Result alias breaks a bare two-parameter Result, and the remedy is a convention every prelude-globbing file must follow.
