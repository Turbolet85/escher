@row_set n=1 row=decision/shared-fixtures-in-one-module cell=rule seen=1 fact=F6 detector=none
value: The fixture boot and hold, the hold in a named fixture state, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net.
basis: fct-m266tyny
because: The shared fixtures module now states a hold in a named state beside its boot and hold, so the row naming what the module states once must name it too.
@row_set n=2 row=decision/inline-cfg-test-modules cell=except seen=1 detector=D-tests-framework
value: cfg(test) modules held in files of their own through a path attribute at their parent.
basis: fct-u3fajhmw
because: The driver gains another cfg(test) module held in a file of its own through a path attribute, so the exception's stated count of such modules no longer holds and the exception must stand without it.
@row_add n=1 row=contract/driver-session/reach area=session-lifecycle fact=F1 detector=none
boundary: Driver session
item: reach
promise: A session reaches a named fixture state in process by running each step (click, type, press, advance) through the ordinary validated run path in order; the reach stops at the first refused or unsettled step, naming its index and its cause or what is busy, and the steps before it stand.
basis: fct-urtem5yt
because: The driver gains a reach of a named fixture state through the validated run path, a promise at the session boundary no row of the area states.
@row_add n=2 row=contract/driver-session/fixture-state-name area=session-lifecycle fact=F2 detector=none
boundary: Driver session
item: fixture state name
promise: A fixture state name is compared whole against the app's closed table; any other name is refused UnknownState with a fixed text holding nothing supplied, and a refused name starts no session.
basis: fct-urtem5yt
because: The driver gains a whole-name lookup of fixture states refusing any other name with a fixed text, a validation contract no row of the area states.
@row_add n=3 row=absence/fixture-state-on-carriers area=session-lifecycle fact=F3 detector=none
capability: A start in a fixture state over the command line, MCP, the session socket or a kept check
basis: fct-ggyakcis
because: The founder ratified a start in a state as in process only, so no carrier gains a fixture state and the area must record that whole absence.
@row_add n=4 row=contract/session-recording/reached-state area=session-lifecycle fact=F4 detector=none
boundary: Session recording
item: session started in a state
promise: A recorded session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot through the ordinary call path with the check form unchanged.
basis: fct-bv2xw5ml
because: A start in a state is recorded and replayed with no new check form, a promise about the kept check other readers rely on that no row of the area states.
@row_add n=5 row=contract/stand-boot/start-in area=3-test-harness-contract fact=F5 detector=none
boundary: seven_guis headless stand
item: stand::start_in
promise: stand::start_in checks a fixture state name against the task's fixture states before it boots, then starts the session and reaches the state; a refused name boots nothing and returns no session.
basis: fct-k57px24b
because: The stand gains a start in a named state that checks the name before it boots, a stand entry point beside boot that the harness contract's stand rows do not state.
@row_add n=6 row=decision/start-in-state-reads-as-by-hand-path area=integration-test-strategy fact=F7 detector=D-tests-coverage
name: Start in a state held to its by-hand path
rule: A session started in a named fixture state reads, in both layout modes, as that state reached by hand from boot and takes the next call as the by-hand session does; an unknown name is refused with the session reading as boot.
guard: stand_start_in_state
basis: fct-bv2xw5ml
because: The start in a state is checked at the integration tier against its by-hand path in both layout modes, a standing equivalence future work on fixture states must keep.
@row_add n=7 row=decision/std-result-under-native-prelude area=ci-integration fact=F8 detector=none
name: std Result spelled under the native prelude glob
rule: In a file that globs the dioxus_native prelude, a two-parameter result type is spelled std::result::Result.
why: On Android the glob brings a one-parameter Result alias, and only CI's Android test job compiles it, the dev host having no Android target.
basis: fct-ez7afaim
because: A bare two-parameter Result under the prelude glob turned CI's Android test job red while every other host compiled it, so the convention that avoids it must stand.
